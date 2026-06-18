# Hilavitkutin: A Canonical Design Specification

**Version:** standalone-1, 2026-06-11
**Status:** the consolidated, self-contained design of record
**Scope:** the whole hilavitkutin pipeline execution engine, its consumer
contract, its compile-time and runtime machinery, the substrate it rests on,
and the constraints it is built under

## About this document

This is a self-contained specification. A reader with no prior knowledge of
the project, the repository, or any earlier design artefact can read it top
to bottom and understand what hilavitkutin is, how it works, and why it is
shaped the way it is. Every external concept it depends on (the foundation
primitives in `notko`, the numeric and analysis primitives in `arvo`) is
included inline as an appendix; nothing here requires reading another file.

It supersedes, for the purpose of "what is the design," the scattered set of
prior documents: a founding consolidation spec, a chain of design-round
amendments, and a recent reconciliation addendum. Where this document and any
older one disagree, this document is the design. It does not narrate the
history of how the design was reached; it states the design.

The document has three parts. Part I is the design proper: what the engine
is and how every subsystem works. Part II records the constraints the design
is built under and the deliberate deviations those constraints force. Part
III is the substrate appendices (notko, arvo) plus a vocabulary glossary.
Read Part I for the design; consult Part II and III as the design references
them.

---

# Part I: The design

## 1. What hilavitkutin is

Hilavitkutin is a pipeline execution engine. A program built on it is a graph
of small typed units of work; the engine analyses that graph once, compiles
a dispatch program for it, and then runs that program over columns of data,
frame after frame, on a pre-allocated pool of threads.

The thesis that shapes everything else: **monomorphisation is the dispatch.**
There is no interpreter loop walking a list of boxed work items, no virtual
call per unit, no runtime type inspection. Each unit of work is a generic
type; the engine composes those types into a single monomorphised program at
compile time, and the compiled function *is* the schedule. The cost of
abstraction is paid by the compiler, once, and the running engine is as
direct as hand-written code that called each unit's body inline.

This makes hilavitkutin different from a job scheduler, an actor system, or
an ECS dispatcher that resolves work at runtime. Those resolve "what runs
next" while running. Hilavitkutin resolves it while compiling. The runtime's
only decisions are parameter-shaped (how big a window of records to process,
which precomputed configuration to use, whether a unit's inputs changed since
last frame) and never structural (which units exist, how they are grouped,
what order they dispatch in). Structure is fixed at build; parameters adapt
between frames.

The engine targets workloads that run the same graph many times over changing
data: simulation frames, compiler passes over a syntax tree, a lint engine
over a file set, a physics step, a render tick. The schedule is computed once
and reused across every frame; the per-frame cost is the work itself plus a
near-zero dispatch overhead.

Every crate in the engine is `#![no_std]`: no operating-system runtime, no
heap allocator pulled in by default, no dynamic dispatch, no runtime type
identity. Memory comes from a consumer-supplied provider; threads come from a
consumer-supplied pool; the engine owns the analysis, the dispatch codegen,
and the data plane's lifetime, and nothing else.

## 2. The execution hierarchy

The engine's vocabulary is a strict hierarchy from coarsest to finest. Every
term has one meaning; the words are load-bearing and used consistently
throughout.

```
pipeline -> core -> phase <-> waist -> trunk -> fiber <-> branch <-> bridge
  -> morsel -> micro-morsel -> record
```

- **pipeline**: the whole scheduled execution graph for one application. One
  `Scheduler` instance is one pipeline.
- **core**: a thread-pool worker. The engine pins work to cores; one
  physical core runs one worker.
- **phase**: a synchronization band. All work in a phase completes before
  the next phase begins. Phases are separated by waists.
- **waist**: a phase boundary. It is the point in the dependency graph where
  the number of concurrently-live execution paths reaches a local minimum; it
  is the natural place to put a barrier because few paths cross it. The waist
  is detected, not declared.
- **trunk**: a group of fibers that run together within one phase over
  disjoint columns. A trunk is the unit of core assignment: one trunk runs on
  one core with zero synchronization against its sibling trunks, because they
  touch disjoint data. Trunks are how the engine parallelises.
- **fiber**: a sequential dispatch unit: a chain of work units that run one
  after another, threaded so that each unit's output feeds the next while the
  intermediate data stays cache-resident.
- **branch / bridge**: alternative dispatch shapes between trunks. A *branch*
  is a chaser within a trunk's morsel scope; a *bridge* is a cross-trunk
  fan-in, where a unit consumes the outputs of several parent trunks once
  those parents have produced the records it needs.
- **morsel**: a window into a contiguous range of records. The engine
  processes a fiber morsel by morsel so that the working set of a morsel fits
  in cache. The morsel is the cache-friendly per-fiber unit.
- **micro-morsel**: a sub-morsel boundary for inner synchronization, used at
  very large scale when even a morsel's working set exceeds the innermost
  cache level.
- **record**: one data point in one column. Never called a "row," an
  "entity," or an "item"; the engine has no tables and no joins, only
  independent typed columns of records.
- **column**: an independent, typed store of records. Columns do not join;
  a work unit that needs data from two columns declares access to both, and
  the engine guarantees the records it sees are consistent.

The dead terms `chain`, `chain_group`, `partition`, `archetype`, `entity`,
`row`, and `order` are not used for any engine concept. (Their historical
mappings, where a reader of older material needs them: chain becomes fiber,
chain_group becomes trunk, partition becomes phase, archetype becomes fiber,
entity/row becomes record, order becomes scheduling hints.)

## 3. The consumer contract

A consumer builds a pipeline by declaring work units and the stores they
touch, then registering them on a builder. This section is the whole surface
a consumer programs against.

### 3.1 WorkUnit

A work unit is a type that implements the `WorkUnit` trait. The trait
declares, at the type level, exactly which stores the unit reads and which it
writes, plus a scheduling discipline and an execution body:

```rust
trait WorkUnit<Sched = Always> {
    type Read: AccessSet;    // the stores this unit reads
    type Write: AccessSet;   // the stores this unit writes
    type Hint;               // scheduling hints (priority, atomicity, ...)
    type Ctx<'frame>;        // the per-frame context handed to execute
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>);
}
```

The `Read` and `Write` access sets are the heart of the contract. They are
type-level sets of store markers. The engine reads them to build the
dependency graph: if unit A writes a store that unit B reads, B depends on A,
and the engine orders them accordingly. The access sets are how the engine
knows, at compile time and with no runtime inspection, what every unit
touches.

`execute` receives a `Context` (see 3.4) and nothing else. It does not
receive handles to global storage, it does not reach into a registry, it
holds no long-lived references. Everything it can touch is exactly what its
access set declared, projected into the context for the duration of one call.

### 3.2 Stores

State lives in one of four store shapes. There is nothing else; a pipeline's
entire mutable and immutable state reduces to these.

- **`Resource<T>`**: a singleton store: one value of `T`, with
  scheduler-managed lifetime. Providers (a clock, a configuration, an
  interner) are Resources.
- **`Column<T>`**: a collection store: N records of `T` in columnar layout,
  accessed through cache-friendly morsel windows.
- **`Accum<T>`**: an accumulator: an append-only output store a unit pushes
  records into, with a reserved capacity fixed at build.
- **`Virtual<V>`**: a fired/event marker carrying no data, used for
  cross-unit signalling at plan-time-determined boundaries (see 3.3 and 8).

The disqualifying test for any new state shape a consumer might invent: *is
this a way to refer to scheduler-owned data from outside a work unit's
declared access set?* If yes, it is forbidden; it is reinventing the heap in
newtype clothing. State is a Resource or a Column; access to it is a work
unit's declared `Read`/`Write`; there are no `Ref`/`Handle`/`Key` types that
let arbitrary code dereference scheduler storage.

### 3.3 Schedules

A work unit carries a schedule, the `Sched` parameter, that says when it runs:

- **`Always`** (the default): every frame.
- **`On<V>`**: only in a pass where the virtual `V` was fired by some
  producer unit this pass. This is the event-gated discipline: a producer
  fires `V`, and every `On<V>` consumer runs that pass and only that pass.
- **`OnMeta<V>`**: gated on one of the engine's own lifecycle virtuals (see
  8). Consumer code uses `OnMeta<ScheduleEnd>` to run a hook at the end of
  each frame.

Schedules are a closed vocabulary; the gating is resolved at compile time
into a const predicate (always-true for `Always`, an epoch comparison for
`On<V>`, a lifecycle-rank match for `OnMeta<V>`) so a skipped unit costs a
single predictable branch, not a dispatch decision.

### 3.4 Context and the computed Ctx type

Inside `execute`, a unit reads and writes its stores through a `Context`. The
context is a typed bundle of accessors, one per store in the unit's access
set: a column reader for each `Column` read, a column writer for each
written, a resource accessor for each `Resource`, an accumulator writer for
each `Accum`, a virtual firer for each `Virtual` written. The accessors
window into the current morsel: a column read at index `i` reads record
`morsel.start + i`, so a unit's body is written against morsel-relative
indices and the engine places the window.

The full context type is mechanical to derive from the unit's `Read`,
`Write`, and `Sched`: each store kind in the access sets maps to one accessor
shape. A consumer therefore does not hand-write the context type. The engine
provides a type function:

```rust
type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, Self::Sched>;
```

`CtxFor` computes the entire context type from the three inputs. A consumer
declares `Read`, `Write`, `Hint`, and the schedule; the context follows
automatically and provably matches what the dispatch machinery projects (the
two are derived from the same fold, so they cannot disagree). The default
schedule is `Always`, so an ordinary unit writes
`type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>`.

### 3.5 The builder

A pipeline is assembled on a builder, one registration at a time:

```rust
let scheduler = Scheduler::builder()
    .with(MyResource::new(initial))   // register a Resource value
    .with(Column::<Record>::new())    // register a Column
    .with(Accum::<Output>::new())     // register an Accum
    .with(ProducerUnit)               // register a WorkUnit
    .with(ConsumerUnit)
    .clock(my_clock)                  // optional: supply a clock provider
    .build(storage, record_count)?;   // finalise: compute the plan, reserve columns
```

`.with(...)` accepts any registrable input (a unit, a store value, a platform
provider, a configuration) and routes it by type to the right accumulator on
the builder's type-state. `.build(...)` runs the whole plan analysis (section
4), reserves the columns through the supplied storage, and returns a
`Scheduler` ready to run. A unit that references an unregistered store is a
compile error naming the missing store.

Platform providers that carry a value (the clock) get a dedicated builder
slot (`.clock(...)`) because `.with(...)` tracks platform inputs by type and
drops their value; value-carrying providers must be retained.

### 3.6 Running

```rust
scheduler.run();                  // one frame, single-core
scheduler.run_parallel(&pool);    // one frame, parallel over the pool
scheduler.run_fused();            // one frame, fully fused linear chain
```

Each call is one frame. The schedule is reused across calls; the per-frame
cost is the work plus near-zero dispatch overhead. `run` walks the compiled
program single-core in phase order; `run_parallel` dispatches trunks across
the pool's cores; `run_fused` collapses a linear pipeline into one fused walk
when the graph permits.

## 4. The plan stage

`build` runs a fixed analysis chain that turns the registered units and their
access sets into an execution plan. The steps, in order:

1. **Access matrix.** Collect every unit's `Read`/`Write` access set into a
   matrix of units against stores. This is the raw dependency information.
2. **Topological sort.** Order units so every writer precedes its readers; a
   dependency cycle is a build error. (See the registration-order note in
   Part II.)
3. **Waist detection.** Find the phase boundaries: the points in the
   dependency graph where the count of concurrently-live paths is locally
   minimal. Each waist starts a new phase. This is computed by `arvo`'s
   waist-detection primitive over the dependency adjacency.
4. **Reverse Cuthill-McKee (RCM) reordering.** RCM produces two orderings
   from the dependency structure. The **row reordering is the work-unit
   execution order**: among the topological orders that are all valid, RCM
   picks the one with the best cache locality, and this is the order the
   dispatch walk uses within a phase. The **column reordering is the arena
   byte layout**: it places columns in memory so that units that touch them
   together sit adjacently. (The execution-order role of RCM is canonical;
   see Part II for its mechanism constraint.)
5. **Block-diagonal / Dulmage-Mendelsohn decomposition.** Validate the phase
   structure and eliminate dead columns (columns no live unit reads).
6. **Trunk formation.** Within each phase, group fibers into trunks of
   disjoint column access. When a phase has many fibers, a spectral method
   (over the fiber-conflict graph) forms the trunks; for few fibers, a greedy
   or matrix-chain method does. Trunks are the parallelism unit: disjoint
   columns mean zero cross-trunk synchronization.
7. **Fiber grouping.** Within each trunk, chain units into fibers so an
   intermediate result stays cache-resident from the producer to the consumer
   that reads it.
8. **Morsel sizing.** Compute each fiber's morsel size from the cache model:
   roughly `L1_usable / sum_of_write_record_sizes`, clamped and aligned, so a
   morsel's write working set fits L1.
9. **Dirty propagation seed.** Compute, per unit, the predecessor mask (which
   units feed it) and the read mask (which stores it reads), so the runtime
   can skip units whose inputs did not change (section 7).

The plan is computed once at `build` and **stored back onto the consumer's
columnar storage** as flat data: the phase table, trunk table, fiber table,
per-unit metadata, per-fiber morsel sizes, and the RCM renumber all live as
columns the dispatch consumer reads. The engine owns the analysis and the
counters; the consumer owns the storage. A full recompute (microseconds) runs
only when the structure changes; there is no incremental plan maintenance.

## 5. Compile-time dispatch: monomorphisation as the schedule

This is the mechanism that makes "monomorphisation is the dispatch" real.

The registered units form a flat heterogeneous list at the type level (a
cons-list: `WuCons<HeadUnit, WuCons<NextUnit, ... WuNil>>`). The dispatch
machinery walks this list at compile time, and at each position it knows the
unit's type, its access set, and (from the plan's const grouping) which phase
and trunk it belongs to. The walk emits, for each trunk, a monomorphised
program containing exactly that trunk's units inlined in order, and dead-code
eliminates everything else. The shipped binary contains one direct,
inlined-body program per trunk, with **zero indirect calls** through the
dispatch path (verified by an assembly-level check that no `blr`/`call`
instruction appears in the dispatch monos).

Two compile-time techniques carry this, and both exist because of a hard
toolchain constraint (detailed in Part II): the engine **cannot** partition a
heterogeneous carrier into per-trunk sub-lists at the type level, because that
would require a form of specialization the language forbids as unsound. So:

- **Const-evaluated grouping.** Each unit's access set is folded, by const
  evaluation, into a bitmask over the store space. A const function computes
  the whole grouping (phases by waist, trunks by disjoint-column union-find,
  the rank renumber that orders lifecycle bands) from those mask arrays. The
  grouping lives in const data, never in the carrier's type.
- **Const-gated dead-code elimination.** The dispatch walk visits every
  carrier position but guards each unit's body with a compile-time predicate:
  "is this position a member of the trunk currently being emitted, in the
  phase currently dispatching?" Positions that fail the predicate fold away
  to nothing. The result is a member-only program per trunk, achieved without
  ever constructing a per-trunk type.

Type-keyed access (a unit reaching its specific store out of the registered
set) uses inferred **index witnesses** (`Here` for the head, `There<I>` for a
position `I` steps in) rather than type-equality matching, again to avoid the
forbidden specialization. The witness is computed once at the type level; the
projection is a direct field access at runtime.

The engine uses no `dyn Trait`, no `TypeId`, no `std::any`. A unit's identity
is its monomorphised function. This is why the dispatch is free: there is
nothing to resolve at runtime because the resolution is the type system's
output.

## 6. Single-core execution

`run` executes one frame single-core. It walks the compiled per-trunk
programs in phase order: phase 0's trunks, then the waist barrier (trivial
single-core), then phase 1's trunks, and so on. Within a trunk, each fiber
runs morsel by morsel; within a morsel, each unit's `execute` runs over the
morsel's record window with its context projected.

Three execution shapes share this path:

- **Morsel-outer** (the default for a fiber with cache-resident
  intermediates): the fiber processes one morsel fully (all its units, over
  that record window) before moving to the next morsel, so intermediates
  stay hot.
- **Unit-outer** (for a fiber that writes an accumulator): each unit runs
  over the whole record range before the next unit, because an
  append-ordering across records must be coherent.
- **Fused** (`run_fused`): a linear chain of units with no branching collapses
  into a single fused unit, dispatched as one morsel walk; the intermediate
  values never touch a column at all.

The accumulator append surface saturates at the reserved capacity (a
soundness guard): an `Accum<T>` reserves a fixed capacity at build, and an
append past it is dropped rather than overflowing. Capacity equals the record
count at build by default; a pipeline that appends more than one record per
input record sizes its accumulator with headroom.

## 7. Incremental dirty-skip

A pipeline that runs the same graph every frame usually has frames where most
inputs did not change. The engine turns the batch processor into an
incremental one: before each frame, a dirty seed (which stores changed since
last frame, set by the consumer marking a store dirty or by a resource
replacement) propagates forward over the per-unit predecessor masks. A unit
whose entire predecessor set is clean is skipped for that frame, leaving its
output columns untouched. The propagation is a bitmask sweep over the
dependency graph (tens of nanoseconds for tens of units); the skip is a
single predicate per unit. The first frame after build is all-dirty (cold
start), so everything runs once.

## 8. The self-hosting meta pipeline

The scheduler schedules itself. The same work-unit-and-virtual machinery that
runs consumer work also runs the engine's own lifecycle, through a small set
of **meta virtuals** fired in a fixed order each frame:

```
PlanStage -> ScheduleReady -> PassStart -> (consumer work) -> ScheduleEnd
```

These are lifecycle ranks: meta units gated on `PlanStage` run first (plan
maintenance), then `ScheduleReady`, then `PassStart` (per-pass setup before
consumer work), then the consumer band, then `ScheduleEnd` (the epilogue,
after all consumer work). The grouping makes the lifecycle rank the outer
phase key, so a meta unit lands in the phase band for its lifecycle point and
dispatches in order with consumer work, on the same carrier, through the same
const-gated walk.

The engine exposes its own observation state to meta units through a bridge.
Mutable per-frame engine state (a pass counter, frame-duration metrics) lives
in an engine-owned `MetaBlock` held as a scheduler field, **not** as a
consumer `Resource` (consumer resources are read-only `Copy` values and
cannot carry the interior-mutable cells the engine updates each pass). A meta
unit reads this state through a `MetaAccess`-gated accessor available only on
a context that carries a meta reference; a plain consumer context cannot name
the accessor, enforced at compile time. So a consumer hook
(`OnMeta<ScheduleEnd>`) can read the frame's metrics, but consumer work cannot
reach engine internals.

The meta pipeline runs identically single-core and parallel. In the parallel
case the meta bands dispatch on a designated core (the main thread) with the
frame publish/await protocol as the ordering barrier: leading bands before
the workers are released, trailing bands after they rejoin.

## 9. Parallel execution

`run_parallel` runs one frame across a pre-allocated thread pool. The model
is **core-pinned trunks joined only at waists and bridges.**

- The pool is spawned once, at the first parallel frame, and the workers park
  between frames; there is no per-frame thread creation.
- Each trunk is owned by one core (the within-phase trunk rank modulo the core
  count). A core runs its trunks over the full record range with zero
  synchronization against sibling trunks, because trunks in a phase touch
  disjoint columns by construction.
- A **waist** is a barrier: every core finishes its phase-N trunks before any
  core starts phase N+1, so a phase-N+1 reader sees every record a phase-N
  writer produced.
- A **bridge** is a cross-trunk fan-in within the dependency structure: a unit
  that consumes several parent trunks' outputs runs after those parents have
  produced the records it needs.
- The frame protocol is a publish/await pair: the main thread publishes the
  frame (releasing parked workers), the workers run their owned trunks, and
  the main thread awaits their completion. These two points are the only
  synchronization; everything between is lock-free by construction.

Single-fiber record splitting exists only as a constrained two-way
head-and-tail convergence in a single-trunk commutative phase, never as an
N-way record or morsel partition. Parallelism comes from trunks, not from
slicing one fiber's records across cores.

An accumulator-bearing parallel frame uses a unit-outer path: each core gets
an exclusive region of the accumulator (its base advanced past the prior
cores' regions), the cores append into their regions in parallel, and the
main thread merges the regions after the workers rejoin, preserving append
order.

The engine's parallel performance target is parity with an optimally
threaded standard-library baseline using the same core count: it wins at
fan-out extremes and runs at parity through the mid-range. (The performance
gate that enforces this is described in Part II.)

Work-stealing is not the default. Morsel assignment is deterministic and
pre-partitioned. A consumer that wants stealing supplies an `Executor`
implementation; the engine provides the extension point, not a built-in
stealer.

## 10. Runtime adaptation

The engine adapts between frames, but only parameters, never structure. This
is the governing principle and it is absolute: no runtime signal ever
regroups fibers, reorders dispatch, or changes which units exist. Those are
structural and happen only at build. What adapts is the plan's parameters:
morsel sizes, per-phase configuration selection, record-range windows, core
affinity.

The mechanism is exponential moving averages (EMAs) of execution metrics,
folded with a fixed weight (one eighth) each frame, read by the engine to
make parameter decisions. The canonical metric is frame duration: the engine
samples a clock at frame start and end and folds the duration into a
metric the meta pipeline carries. A consumer reads the prediction (the EMA of
recent frame durations) through the `OnMeta<ScheduleEnd>` bridge for its own
budgeting.

Three reorganisation triggers, all parameter-side:

- **Morsel-timing.** When a fiber's per-fiber duration EMA drifts, recompute
  that fiber's morsel size from the cache model and write the new size into
  the plan parameter the next frame reads. No regrouping.
- **Phase-balance.** When a phase's duration EMA shows imbalance, re-select
  among the pre-monomorphised per-phase configuration variants by a runtime
  index. The variants are all compiled; the selection is an index, not a
  recompile.
- **Record-count.** When the record count crosses the cache model's rebuild
  threshold, recompute the parameter outputs (morsel ranges, approach
  selection) over the unchanged static unit set. This is a parameter
  recompute, never a structural rebuild; a unit-set change is a separate,
  build-time recompile.

A consumer that replaces a resource between frames (a new configuration, a
swapped provider) marks the plan dirty and the next frame picks up the new
value and any parameter recompute it implies.

Predictive parking uses the per-phase EMA: a worker about to wait on a waist
barrier picks spin, spin-loop, or park by the predicted wait, rather than
always spinning or always parking.

## 11. Platform tiers

The engine is generic over its platform providers and ships two tiers:

- **os** (the default): a thin layer over the operating system's primitives
  (memory mapping, threads, a monotonic clock) for a hosted target.
- **no_os**: no platform implementation; the consumer supplies memory,
  threads, and clock providers by dependency injection, for a bare-metal or
  custom-runtime target.

There is no std tier; the engine never links the standard library. (A std
tier was specified historically and is deferred indefinitely.) The providers
are trait contracts (`MemoryProviderApi`, `ThreadPoolApi`, `ClockApi`); the
scheduler is generic over them, never `dyn`. The clock defaults to the os
monotonic clock under the default feature and to a null clock on no_os until
the consumer supplies one.

## 12. The plugin-host layer

The engine itself never loads code at runtime; its unit composition is static.
But a downstream host (a tool built on the engine that wants to load
third-party extensions at runtime) is supported by a separate layer of
crates: a cross-platform dynamic-library loader (pull-based, explicit-symbol
resolution over the OS's dynamic linking, still `no_std`, no allocator) and a
contract-bound orchestration layer above it (extension descriptors, a
per-extension lifecycle, capability dispatch over stable capability ids).

A loaded plugin integrates with the engine through the **facade pattern**: a
statically-registered work unit that declares only its own bridge stores (the
columns the plugin reads and writes on the host side) and, in its `execute`,
hops across a C ABI boundary once per morsel to call the plugin, handing it a
morsel-relative range. The plugin owns its own absolute cursor and its own
internal dispatch. The facade unit's bridge edges enter the engine's
dependency analysis normally (so the plugin is scheduled correctly relative
to host work), the host work units keep their zero-indirect dispatch, and the
per-morsel ABI hop keeps the indirect call off the per-record hot path. The
plugin's unknown internal access is not host data, so it needs no
over-approximating access set; the facade declares what it bridges and the
engine's registration check passes by construction.

Every extension loads, runs, and drops independently of its siblings at
arbitrary points; the framework never assumes an all-plugins-loaded gate.

---

# Part II: Constraints and deliberate deviations

The engine is built on a pinned nightly Rust toolchain and under a strict
no-heap, no-dynamic-dispatch discipline. Several design choices are forced by
hard limits of that toolchain, and a few are deliberate deviations from the
pure ideal that the limits make necessary. They are recorded here so a reader
never mistakes a constraint-driven shape for a free choice.

## 13. The hard constraints

- **No full specialization.** Lifetime-dependent specialization is unsound and
  the language forbids it. The engine therefore cannot partition a
  heterogeneous unit carrier into per-trunk sub-lists at the type level (that
  needs specialization), and cannot do type-equality-keyed projection. The
  const-evaluated grouping plus const-gated dead-code elimination (section 5)
  and the index-witness projection are the sound mechanisms that replace it.
  This is the single most shaping constraint in the engine.
- **Generic-const-expression limits.** The engine sizes fixed arrays by
  const-generic expressions over capacities. Two limits bite: the compiler
  rejects field access on a generic constant (so a cap cannot be read as
  `Cfg::SOME_CAP.0` inside an array length), and it caps the complexity of an
  inline const block in a trait bound. The engine works around the first by
  routing caps through an associated-type capacity pattern (an associated
  type whose generic-array projection carries the size, never a const field
  access), and the second by lifting compile-time predicates into
  associated-const carrier structs rather than inline const blocks.
- **Fixed capacity ceilings.** Where a cap cannot yet be lifted to be
  consumer-tunable under the above limits, it is a fixed default with a
  documented value and a tracked condition under which it lifts. Caps are
  defaults, never policy; the engine does not hardcode the one of many
  possible answers as the only answer.
- **Accumulator capacity is a soundness guard.** Appends saturate at the
  reserved capacity; this is deliberate, to keep the append surface sound
  without bounds-checking every push. Consumers size with headroom.

## 14. The deliberate deviations

- **RCM execution order is mechanism-constrained.** The design says RCM's row
  ordering is the work-unit execution order, applied by the engine itself.
  Under the toolchain, the only way to apply an arbitrary computed order while
  preserving zero-indirect dispatch is to precompile the RCM-ordered carrier
  as one of the pre-monomorphised configuration variants; a const-function-
  computed order hits the generic-const-expression wall, and a build-script or
  proc-macro cannot see the resolved access sets. So the RCM order is applied
  through the variant mechanism, and whether its cache win justifies the
  variant cost on a given workload is a benchmarked decision.
- **Producer-before-consumer registration is provisional.** The ideal is that
  the engine auto-orders units regardless of registration order. Until the
  RCM execution-order recovery fully lands, the engine requires the consumer
  to register producers before consumers and validates it (a non-topological
  registration is a build error). This is a provisional constraint, to be
  relaxed when the order-recovery mechanism or the toolchain matures.
- **Engine-owned meta state.** The self-hosting meta pipeline's mutable state
  lives in an engine-owned scheduler field rather than in the uniform
  Resource/Column data model, because consumer resources are read-only `Copy`
  values and cannot carry the engine's interior-mutable per-frame cells. The
  meta state is reached only through the `MetaAccess`-gated accessor; the
  deviation is contained and compile-time enforced.

## 15. The performance gate

The engine's parallel claim (parity with optimally-threaded standard-library
code at equal core count) is enforced by a benchmark gate that is a standing
part of the build, not a one-off measurement. The gate runs per-arm
calibrated bars (tight where the engine should win at fan-out extremes,
parity-tolerant through the mid-range), measures against a persistent-pool
standard-library baseline that matches the engine's spawn-once discipline,
and aggregates over repeated runs to remove variance. A red gate is a signal
to fix the code, never to weaken the bar.

## 16. Build state and the path to completion

This specification describes the complete canonical design. The implementation
has reached the following state, and the remaining work is charted as a
single completion arc (the engine's parallel core and self-hosting meta
pipeline are functionally complete; what remains is adaptation completion,
performance substance, the operations surface, and the ecosystem bridges):

- **Complete and verified:** the foundation layer (numeric and analysis
  substrate, capacity-typed array sizing, access-set machinery, witness
  projection, columnar storage with plan store-back); single-core
  zero-indirect execution (the fiber walk, fusion, morsel windowing,
  accumulator appends with per-frame reset, incremental dirty-skip);
  compile-time grouping (the access-mask fold, the const grouping, the
  const-gated trunk dispatch); parallel execution (the spawn-once pool, the
  frame publish/await protocol, core-pinned trunks with waist barriers, the
  unit-outer accumulator path); the self-hosting meta pipeline end to end; and
  the first runtime-adaptation metric (clock-sourced frame-duration EMA).
- **Charted, not yet built:** the rest of runtime adaptation (per-fiber and
  per-phase EMAs, the three reorganisation triggers, predictive parking);
  the performance-substance features (micro-morsel tiling, the branch dispatch
  shape, sub-byte bitpacking stride, shared-read-column strategy, intrinsic
  microkernels, the RCM-order decision); the operations surface (frame status
  with dependent poisoning, the work-stealing extension point, schedule
  introspection, live plan caching); and the ecosystem bridges (the facade
  plugin-host integration, declarative unit bundles, the hot/cold persistence
  spine).

Every remaining step rests on a mechanism already proven feasible; the build
from here is mechanical.

---

# Part III: Substrate appendices and glossary

The engine builds on two foundation crates, `notko` and `arvo`, developed
alongside it. A reader does not need their repositories: the features the
engine depends on are inventoried here.

## Appendix A: notko (foundation primitives)

`notko` is the zero-dependency, `#![no_std]`, no-alloc foundation crate that
both arvo and hilavitkutin build on. The engine uses it for fallibility,
boundedness, and the FFI layout types.

### A.1 The fallibility ladder

notko replaces `Option` and `Result` with three tiers that mirror the
precision/throughput tradeoff arvo uses for numerics. The tier names what the
failure path costs.

`Just<T>` (the hot tier) is an infallible value wrapper, `#[repr(transparent)]`
over `T`, zero-cost. It is the type for a position that could be fallible but
whose failure is proven unreachable; `?` on it compiles to nothing (its `Try`
residual is an uninhabited enum, so the branch is always Continue).

```rust
#[repr(transparent)]
pub struct Just<T>(T);
```

`Maybe<T>` (the warm tier) replaces `Option<T>`: a two-variant enum
`Is(T) | Isnt`. When `T` carries a niche (a pointer, a reference, a
`NonZero`), the compiler niche-fills, so `Maybe<T>` is the same size as `T`
with `Isnt` as the null pattern. It carries the full Option-equivalent method
set and converts both ways with `Option`.

```rust
pub enum Maybe<T> { Is(T), Isnt }
```

`Outcome<T, E>` (the cold tier) replaces `Result<T, E>`: `Ok(T) | Err(E)`,
platform-standard repr (FFI-critical results wrap a payload in a dedicated
`#[repr(C)]` struct rather than forcing the enum's layout).

```rust
pub enum Outcome<T, E> { Ok(T), Err(E) }
```

A `ConstTry` / `ConstFromResidual` pair gives const-callable parallels of
`core::ops::Try` so an explicit `match x.branch() { ... }` works in a
`const fn` (the `?` syntax itself stays non-const). The engine uses `Maybe`
and `Outcome` throughout its fallible surface and `Just` on proven-infallible
hot paths.

### A.2 Boundedness

`Boundable` marks a type carrying a value in a compile-time `[MIN, MAX]` range,
rejected out of range at construction; `BoundError<I>` names which bound was
crossed and carries both the offending value and the bound. `NonZeroable`
marks a type with a zero sentinel and a guaranteed-nonzero form (`try_new`
returns `Maybe::Isnt` for zero). arvo implements both on its fixed-point
types; a consumer takes `T: Boundable` or `T: NonZeroable` and programs
against the guarantee.

```rust
pub trait Boundable: Sized {
    type Inner: Clone;
    const MIN: Self::Inner;
    const MAX: Self::Inner;
    fn try_new(value: Self::Inner) -> Outcome<Self, BoundError<Self::Inner>>;
    fn value(self) -> Self::Inner;
}
```

### A.3 Layout and FFI boundary types

`NicheFilled` is a sealed marker for types where the compiler's
bit-pattern-zero niche actually realises (references, `NonNull`, the `NonZero`
family, function pointers at arities 0 through 8). `MaybeNull<T: NicheFilled>`
is a `#[repr(transparent)]` wrapper over `Maybe<T>` that pins the layout with
a per-instantiation compile-time assertion that its size equals `T`'s, for the
FFI position where a pointer-or-integer-sized nullable representation is the
point. `Slot<T>` is the same idea for `T: NonZeroable + NicheFilled`. The
engine uses these where it marshals data across the plugin-host C ABI.

### A.4 The profile macro

`#[profile(Hot | Warm | Cold)]` is notko's load-bearing enabler: an attribute
that rewrites a function body to a strategy at expansion time. The tiers are
zero-sized markers; `Warm` passes the source `Result`-shaped function through
unchanged, `Hot` emits a release copy whose return type becomes `Just<T>` with
the body rewritten (`Ok(x)` to `Just::new(x)`, `Err(e)` to a panic, the error
match collapsed to an unwrap), and `Cold` keeps the `Outcome` wrap. The
primitives are usable without the macro; it is an optional accelerator that
lets a consumer pick the cost tier per call site rather than per type.

### A.5 Core-shim traits and posture

`IteratorExt` (`next_maybe`) and `PartialOrdExt` (`partial_cmp_maybe`) are
blanket adapters bridging std trait-method signatures that must name `Option`
to `Maybe` at the call site. notko is `#![no_std]`, zero deps, no alloc; the
std primitives survive only in std-trait-method signatures. The runtime crate
is no_std; only the proc-macro crate uses std (and `syn`/`quote`) at compile
time, and what it emits is no_std-clean.

## Appendix B: arvo (numeric and analysis substrate)

`arvo` is the numeric and analysis substrate, `#![no_std]`, depending only on
notko. The engine uses it for numeric primitives, for the const-generic array
sizing that makes its fixed-size storage work, and for the graph and analysis
primitives its plan stage runs.

### B.1 Fixed-point primitives

arvo's arithmetic carriers are `UFixed` (unsigned) and `IFixed` (signed), both
`#[repr(transparent)]` over a strategy-dispatched storage word. They lower to
a bare machine primitive at codegen; the wrapper carries the width and
strategy at the type level only.

```rust
#[repr(transparent)]
pub struct UFixed<const I: IBits, const F: FBits, S: Strategy = Warm>(
    Bits<{ ufixed_bits(I, F) }, S>,
);
```

`I` is integer bits, `F` is fractional bits, `S` is the strategy marker. A
consumer names an exact width through aliases rather than spelling `I`/`F`:
`Uint<3>` is a 3-bit unsigned, `Uint<47, Hot>` a 47-bit unsigned with the
`Hot` strategy, `Int<8>` a signed 8-bit. Non-power-of-two widths are
first-class; the engine's `UnitId` is a `Uint<16>`-class width. The
`repr(transparent)` contract makes the unwrap to the backing primitive free,
and the container choice is the strategy's, not the consumer's.

### B.2 Strategy markers

Four zero-sized markers implement a sealed `Strategy` trait, each projecting
onto overflow policy, container width, and packing layout:

- **Hot**: wrapping overflow, minimum byte-aligned container, dense layout;
  optimises for L1 density and op throughput.
- **Warm** (the default): wrapping, doubled container (so a single op in
  logical range cannot overflow), dense; the development-friendly default,
  bounded to `I + F <= 32`.
- **Cold**: wrapping, minimum container, bitpacked for storage density;
  widens before arithmetic, narrows on store.
- **Precise**: saturating overflow, doubled container, dense.

Cross-strategy operations resolve to the more conservative side by a rank
(`Precise > Cold > Warm > Hot`). The marker attaches as `S` on any numeric
type with a precision/throughput tradeoff.

### B.3 Sizes and caps

`USize` (a `#[repr(transparent)]` wrapper over the platform pointer width),
`Bool` (the control-flow boolean predicates return, distinct from the 1-bit
data type), and `Cap` (a `#[repr(transparent)]` wrapper over `USize` used in
const-generic position to name a fixed capacity) exist over bare primitives so
values carry semantic identity, route through the strategy pipeline, and nest
inside other const-generic newtypes.

### B.4 Floats

`FastFloat<F>` enables fast-math semantics (reassociation, reciprocal
approximation); `StrictFloat<F>` holds bit-exact IEEE 754 for
reproducibility-sensitive paths. Width `F` is sealed to `f32`/`f64`.

### B.5 Bit primitives

`Bits<N, S, Sign>` is the opaque storage word underlying every fixed-point
type. The mask family (`arvo-bitmask`) is what the plan stage uses for DAG
adjacency and grouping, generic over a bit-bearing word `W` (not fixed at 64
bits):

```rust
#[repr(transparent)]
pub struct Mask<W> { pub word: W }

pub struct BitMatrix<W, C: Capacity> {
    pub rows: C::Array<Mask<W>>,  // rows[i] is node i's successor mask
}
```

`Mask<W>` and `BitMatrix<W, C>` carry two independent axes: the per-row bit
width `W` (node fan-out) and the row-count capacity `C`. Parameterising `W` is
exactly how the engine lifted its former 64-node cap.

### B.6 Capacity and Dim (the const-array foundation)

This is the load-bearing surface for the engine's const-generic array sizing.
`Capacity` makes a fixed storage size a *type* rather than a const-generic
value, so the backing array's literal length sits in a generic associated type
and `generic_const_exprs` never runs over capacity arithmetic in consumer
code:

```rust
pub trait Capacity {
    type Array<T>: AsRef<[T]> + AsMut<[T]>;   // [T; N] for Dim<N>
    const CAP: Cap;
    fn filled<T>(v: T) -> Self::Array<T>;
    fn from_fn<T>(f: impl FnMut(USize) -> T) -> Self::Array<T>;
}

pub struct Dim<const N: usize>;
impl<const N: usize> Capacity for Dim<N> {
    type Array<T> = [T; N];
    const CAP: Cap = cap(N);
}
```

`Dim<N>` is the concrete capacity, generic over `const N: usize`, but the
`Capacity` trait itself is non-generic, so a consumer binds `C: Capacity` with
no const parameter and keeps type-dispatch. `[T; N]` is plain
min-const-generics; the cap arithmetic appears only in the associated-const
value position. `ConstCapacity` is a sibling `pub const trait` carrying the
const-callable surface (a `Copy`-bound array GAT plus `get`/`set`) that
compile-time DAG analysis needs, since `Capacity::from_fn` is not
const-callable. `Dim<N>` implements both. The engine's `PlanDims` threads
`Capacity` associated types for every plan dimension (units, stores, fibers,
phases), and the consumer-tunable-caps work routes the remaining hardcoded
caps through this same pattern.

### B.7 Graph and analysis primitives

The plan stage consumes these from arvo's algorithm crates. Each is generic
over a `Capacity` (node/row count) and a bit-container word (the row-width
axis that removed the 64-node cap), and never imports the numeric types
directly.

- `topo_sort`: Kahn topological sort; returns `(valid_count, order)`, with
  `valid_count < N` signalling a cycle.
- `waist_detect`: sets a bit per node whose depth-level width is a strict
  local minimum (the phase/waist boundaries the scheduler barriers on).
- `components`: assigns a component id per node.
- `upward_rank` / `downward_rank`: per-node rank as weight plus max
  successor rank, walked in reverse topo order.
- `rcm_reorder`: reverse Cuthill-McKee permutation (min-degree start,
  ascending-degree BFS, final reverse), through the mask set surface so any
  row word works.
- `Csr`: compressed-sparse-row backing with live-row/live-nnz slack tails.
- `fiedler_vector` + `spectral_bisection`: the spectral pair the engine uses
  for fiber/trunk grouping, backed by a Laplacian and power iteration.
- `greedy_group` + `bin_pack`: greedy fiber grouping into bounded windows.

### B.8 Const-generics posture

arvo uses `generic_const_exprs` (a nightly feature on watch in the stack's
soundness sweep) with `adt_const_params` and `const_trait_impl`. Caps are
const-generic because array lengths must be known at compile time, so storage
is fixed-size with no heap and the type system verifies a graph algorithm's
scratch arrays match the node count. The bridge from a typed `Cap` to an array
length is a `const fn cap_size(c: Cap) -> usize`, the canonical use-site
because the language requires a `usize` array length and rejects the inline
field-access form in const-generic position. The field-access wall bites when
a generic consumer threads a capacity through its own generic code; that is
precisely why `Capacity`/`Dim<N>` exist as the GCE-free replacement (the
literal `[T; N]` lives behind the GAT, the cap appears only as an
associated-const value, and no const expression runs in the consumer's type
position). The known GCE unsoundness is unreachable because the stack never
reflects type identity into subtyping; migration to the successor feature is
tracked.

## Appendix C: vocabulary glossary

**Execution hierarchy** (coarsest to finest): pipeline (the whole scheduled
graph for one app), core (a thread-pool worker), phase (a synchronization
band), waist (a phase boundary at a local minimum of live paths), trunk (a
group of fibers over disjoint columns, the core-assignment unit), fiber (a
sequential chain of units with cache-resident intermediates), branch (a
chaser within a trunk's morsel scope), bridge (a cross-trunk fan-in), morsel
(a cache-friendly window of records), micro-morsel (a sub-morsel inner sync
boundary), record (one data point in a column), column (an independent typed
record store, no joins).

**Type-level:** WorkUnit (the unit of execution; declares `Read`/`Write`
access sets, a `Hint`, a `Ctx`, an `execute`), AccessSet (a type-level set of
store markers), Resource (a singleton store), Column (a collection store),
Accum (an append-only output store), Virtual (a fired event marker), Context
(the per-frame accessor bundle a unit's `execute` receives), CtxFor (the type
function that computes the context type from `Read`/`Write`/`Sched`), schedule
(Always / On<V> / OnMeta<V>), witness (Here / There<I>, the inferred index
that projects a store out of the registered set without specialization),
MetaBlock (the engine-owned mutable meta state), MetaAccess (the gate that
restricts meta state to meta units).

**Plan and runtime:** plan (the analysis output: phases, trunks, fibers,
morsel sizes, the dependency masks), waist detection (finding phase
boundaries), RCM (the reordering producing the execution order and the arena
layout), trunk formation (grouping fibers into disjoint-column trunks), dirty
skip (the incremental skip of units with unchanged inputs), morsel sizing (the
cache-model window computation), the meta pipeline (the engine scheduling
itself through lifecycle virtuals), the EMA (the runtime metric that drives
parameter adaptation).

**Discipline:** no_std / no-alloc / no-dyn / no-TypeId (the constraint
envelope), monomorphisation-is-dispatch (the central thesis), parameters-not-
structure (adaptation changes parameters, never the graph), schedule-once-
reuse (the plan is computed once and reused across frames).
