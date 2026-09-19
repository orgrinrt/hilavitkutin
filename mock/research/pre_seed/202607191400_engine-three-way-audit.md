# Engine arc: three-way audit of canon, source, and roadmap claims

**Date:** 2026-07-19
**Status:** chart-the-path take 2, phases 2 to 4. Replaces the comprehension basis of
`202607181200_engine-arc-recomprehension.md` and invalidates several conclusions in
`202607181300_engine-roadmap-r6.md`.
**Method:** three independent exhaustive extractions, one per axis, reconciled here.
Canon: 94 mechanisms from the consolidation spec plus every later addendum. Source: call-site
tracing from the public entry points across `mock/crates/hilavitkutin*/src/`. Roadmap: every
status claim in `mock/research/*.md`, quoted verbatim with dates.

## Why the earlier charts were wrong

The r6 chart was built by spot-checking. Each question got its own grep, and each grep produced
a locally-plausible answer that contradicted the last one. Head+tail went unbuilt, then
shipped-but-drifted, then superseded. A4 went mechanical, then needs-inversion, then wrong-shape,
then right-shape. That thrash was not carelessness about any one check. It was the absence of a
baseline: nothing in the process ever established, systematically, what canon requires and what
source actually does.

Two methods were tried during this audit and discarded, which is worth recording because both
look reasonable:

A name-based reachability script reported `EngineCtx`, `AccessSet` and `ColumnReaderApi` as
unreachable. That is plainly false. The engine dispatches through traits and monomorphisation, and
neither is visible to name matching. **Static name analysis cannot audit this codebase.**

rustc's own dead-code analysis reports almost nothing, because it does not flag `pub` items in a
library crate. Worse, most genuinely-dead items here carry no `#[allow(dead_code)]` at all: the
`pub use` re-export is what silences the lint. **The compiler's silence is not evidence of life.**

What works is call-site tracing per mechanism, against a list of mechanisms derived from canon
rather than from the code. That is what this document is.

## The three-state model, and why binary status caused every contradiction

The roadmap audit found seven mechanisms whose status two documents state incompatibly. Every one
is the same confusion: **"the code exists" treated as interchangeable with "the behaviour
happens."**

- r5 (2026-06-08) said head+tail "ships", citing a line. The line existed. The behaviour was not
  canon's.
- r4 marked G-a through G-e "UNBUILT"; r5 one day later said "all ship" and called the markers
  "expected-stale".
- r5 said the E4 meta pipeline was "absent. `VirtualFirerApi::fire` is a literal no-op"; the
  completion-arc roadmap three days later said it "ships end to end", citing nothing.

A binary DONE/UNBUILT ledger cannot describe a trait-and-generic engine, because such an engine is
full of code that compiles, has tests, and is never reached. Three states are needed:

**WIRED.** Substrate present and reached from an entry point, and its body does what canon
requires. Genuinely done.

**SUBSTRATE-ONLY.** Code exists, compiles, often has tests, and **nothing reaches it**. The
behaviour is absent. Wiring is the entire remaining work, which is a much smaller job than the
roadmaps that call these "unbuilt" imply, and a much larger one than the roadmaps that call them
"shipped" imply.

**ABSENT.** No implementation.

A fourth category turned out to be necessary and is the most dangerous: **HOLLOW**, where a
reachable function's body does not do what its own doc comment says. These are worse than absent,
because every reader above them assumes the behaviour.

## Live defects

Found by body-reading, not inferred. Ordered by severity.

**`replace_resource` and `replace_value` discard their argument.**
`scheduler/mod.rs:1187` and `:1207`. The signature takes `_new: T`; the body calls `mark_dirty`
and drops it. The doc says "Replace the existing `Resource<T>` instance in the data plane with
`_new`". No install occurs, in any build. A consumer calling this observes the dirty flag and
concludes the swap happened. This is the mechanism domain 22 calls the plan-recompute trigger, and
`202606111700` cites the `plan_dirty` array shipping as evidence the mechanism works.

**`StdThreadPool::spawn` spawns nothing.**
`platform/std_tier.rs:119`, body `let _ = _f;`, with the comment "generic-closure support arrives
in 5a4". `run_parallel` then publishes a frame and waits on `frame_await_done` for workers that
were never created. On the `platform-std` feature the engine deadlocks. The tier's own module doc
calls it a "`std::thread`-backed thread pool", and the real implementation exists twelve lines
above at `:104` as `spawn_fn`, which nothing calls.

**`StdMemoryProvider::deallocate` rebuilds the layout with the wrong alignment.**
`std_tier.rs:68` uses word alignment rather than the original. Undefined behaviour for any
allocation whose alignment exceeds `align_of::<usize>()`, which the 64-byte column alignment canon
requires (mechanism 2) would trigger.

**`classify_cores` returns all-P and probes nothing.**
`thread/class.rs:55` and its four `detect_into` arms, each `let _ = classes;`. The module doc
describes sysfs, sysctl and `GetSystemCpuSetInformation` probing, and states "The engine queries
`classify_cores()` once at pool construction". Nothing calls it.

**`steal_fallback` is `todo!()`.** `thread/mod.rs:104`. Unreachable, so inert, but it is the
Executor extension point canon names in mechanism 69.

## Ledger

Grouped by state. Canon mechanism numbers refer to the extraction; source locations are cited.

### WIRED, and matching canon

The plan chain steps 1 through 4 (`build_dag`, `topo_sort`, `compute_waists`, upward rank),
`rcm_reorder` as a computation, `derive_phase_dispatch_order`, the `DrainStores` blob layout with
the scalar snapshot and live-streamed collections (canon 7, 65, as amended by the storage
addendum), the const-grouping carrier and its DCE walk (`RunTrunkDispatch`, `RunGatedTrunk`,
`RunFiber`, `IsRoot`, `PhaseAt`, `Member`), `EngineCtx` projection and the `Selector`/`Project`
witness families, `GateWith` with Always/On/OnMeta (canon 14 as amended), virtual firing and
epoch reset (canon 17, 20), the E4 meta lifecycle bands and `MetaBlock` bridge (canon 80 as
amended), incremental skip on the single-core and fused paths (canon 52, partially: see below),
per-fiber L1 morsel windows on `run` and `run_core_phase` (canon 22), the frame protocol and the
sense-reversing `waist_barrier`, futex parking, and the pass-duration and per-phase EMAs with
`select_adapt_config`'s decision half (canon 76).

**The accumulator unit-outer path is WIRED and canonical**, and this is the correction that
matters most. Canon states it explicitly: "each core gets an exclusive region of the accumulator
... the cores append into their regions in parallel, and the main thread merges the regions after
the workers rejoin, preserving append order" (`202606111800:452-456`).
`worker_accum_unit_outer` (`scheduler/mod.rs:933`) with `rebase_accums`, `collect_accum_live` and
`merge_accums` matches it.

### SUBSTRATE-ONLY: exists, unreached, behaviour absent

Twelve groups. Each is code that compiles and in most cases has tests.

The **codegen family**: `select_approach`, `codegen_fiber`, `codegen_core`, `FiberDispatch`,
`CoreDispatch`, `run_fiber`, and the api's `DispatchCodegen`, `StandardCodegen`, `LockFreeDispatch`,
`Scheduled`, `FiberShape`. `FiberShape` has **zero impls crate-wide**, so the family is
uninstantiable. This is canon 59, the compiled per-core program, recorded in the deviation ledger
as "not the shipped dispatch, op-blessed runtime mask instead, bench-gated escalation".

The **`dispatch::order` module** entire: `topo_order`, `CarrierMasks`, `carrier_order`,
`carrier_order_dyn`. Its own module doc calls the const order the devirtualisation keystone. The
scheduler has a runtime field of the same name, so the live path and the dead one collide by name.

The **nested carrier walks**: `RunTrunk`, `RunPhase`, `RunPipeline` and six impls. Nothing
constructs `FiberCons`/`TrunkCons`/`PhaseCons`. Established unwireable from flat registration by
`202606071200_gate2-carrier-mechanism-fork.md`: deriving the nested type is partition-by-key,
which walls on forbidden `specialization`.

**Progress and phase-overlap** (canon 33, 60): `ProgressCounter` with correct Release/Acquire,
`store_progress_arena`, `load_progress_arena`, `emit_progress_release_fence` with a real
`dmb ishst`. All unreached; `progress_slots` is `NonNull::dangling()`. The substrate matches
canon's requirement including the "plain store, not fetch_add" constraint.

**Head+tail convergence** (canon 32): `thread::Convergence` carrying `head_thread`, `tail_thread`
and a `meeting_record: ProgressCounter`; `plan::HeadTailConvergence` carrying head and tail accum
slots and a merge op; `RecordRange::{Head,Tail}` in the api. All unreached;
`core_program.rs:109` always emits `Full`.

**Per-core programs and core assignment** (canon 34, 59): `assign_cores` has a real round-robin
body but is called only from its own module's `cfg(test)`; `synthesise_core_programs` fills
`CoreProgram`s and has **test-only callers** (`tests/synthesise_core_programs.rs:60`, `:82`, `:114`),
none from an entry point; both `*_stub` variants have empty bodies.

**The parking tier API** (canon 78, predictive parking): `pick_tier`, `spin_budget_for`, `spin`,
`predicted_wait_ns_load`/`_store`, `ParkTier`, all with real bodies. `waist_barrier` calls
`atomic_wait` directly instead. `WakeStrategy` and `PoolFrame.predicted_wait_ns` both exist.

**The phase-barrier API**: `phase_barrier_arrive`, `_reset`, `_observe`, `BarrierArrival`. A second
barrier protocol on the same `phase_arrived` word as the live `waist_barrier`, using `Release`
where the live one uses `AcqRel`. Mixing them would be unsound.

**`thread::pool`** entire: `ThreadPool`, `ThreadPoolBuilder` and all methods, plus a `Drop` that
sets a flag nobody reads, duplicating what `frame::request_shutdown` does for real.

**The `adapt/` module** entire: nine axis re-exports, `AdaptAxis`, `AdaptAxisDispatch`, `AdaptMode`,
and `AdaptArena`, whose doc describes per-fiber 64-byte-aligned hot lines and per-core park slots
while the struct is two `PhantomData`s. `select_adapt_config` computes its decision without any of
them.

**`strategy/`**: `StrategySelector`, `DefaultSelector`, `Strategy`. Canon 72 requires four-way
plan-time selection; the body only ever returns `Adaptive` and never constructs `PipeChase`.

**`resource::accumulator`**: `AccumulatorSlot`, `ConvergenceBuffer` and `combine`, an
arbitrary-combiner fold, distinct in data model from the live `merge_accums` concatenation.

**Core classification** (canon 68): covered under defects above.

### ABSENT

Matrix-chain DP fiber grouping (canon 51, the >10-op branch; greedy only ships). Dulmage-Mendelsohn
integration and dead-column elimination (canon 49; the arvo-sparse substrate exists upstream).
Spectral trunk formation consumed by the runner (canon 50). `spectral_partition`
(`plan/steps.rs:452`) is called at `plan/steps.rs:663`; whether that caller is itself reachable from
`compute_execution_plan` is untraced, so "not in the runner chain" is unproven as stated and needs
the trace before B3 is scoped. RCM row order as dispatch order (canon 48; computed and discarded).
Micro-morsel inner tiling (canon 24). The *tiling behaviour* is absent; an earlier draft said no
`micro_morsel` symbol exists anywhere, which is false: `MICRO_MORSEL_INTERVAL` is executable code at
`hilavitkutin-api/src/run_cfg.rs:104`, and `dispatch/morsel.rs:67` has its own unreached copy. Per-morsel
generation counters (canon 26; coarse per-store dirty only). Version stamps (canon 81; substituted
by `store_dirty`). Shared read columns between trunks (canon 35). `PipelineResult` and per-fiber
poisoning (canon 83). The morsel-absolute slice accessor. The persistence engine bridge. Sub-byte
bitpacking stride. Column classification consumed (canon 39; currently all-Internal).

### DRIFT: shipped, reachable, and contrary to canon

**The `tphase == 1` N-way ceil-slice** (`scheduler/mod.rs:2107-2132`). Canon 32 requires two
threads from opposite ends of a **commutative** fiber, and the standalone spec narrows it further
in terms that name this exact shape: "Single-fiber record splitting exists only as a constrained
two-way head-and-tail convergence in a single-trunk commutative phase, **never as an N-way record
or morsel partition**. Parallelism comes from trunks, not from slicing one fiber's records across
cores" (`202606111800:447-450`). The shipped branch is N-way, same-direction, and applies to any
single-trunk phase with no commutativity gate. `unit_meta.commutative` is written at
`plan/mod.rs:422` and read nowhere.

This correction reverses `202607181300_engine-roadmap-r6.md` and the sketch
`202607191200_a4-fibercons-nest-wireability`, both of which concluded the N-way form was a
legitimate supersession of canon's 2-way. Canon had already considered and forbidden that exact
generalisation. The sketch's argument was sound from mechanism and performance and still landed
wrong, because it reasoned from an incomplete baseline. The original arc-audit finding, that
head+tail is unbuilt, was right.

**`replace_resource` / `replace_value`**: see defects.

## Nine places where one mechanism has two implementations

Dispatch order (runtime flatten versus the dead const fold, colliding on the name `topo_order`).
The waist barrier, three ways (live sense-reversing, dead spin-only, dead two-call protocol on the
same word with weaker ordering). Per-core trunk ownership (dead precomputed `core_phase_mask`
versus the live inline `rank % ncores`; `core_mask.rs:3-7` calls the mask form "op's chosen
mechanism"). Accumulator merge (dead arbitrary-combiner fold versus live region compaction).
Head+tail (dead `RecordRange::{Head,Tail}` versus the inline arithmetic, itself duplicated across
two call sites). Virtual stamp (dead binding-side pair versus live `VirtualFire`). Spawn
marshalling (polarity inverts between the os and std tiers; on std the working one is dead and the
live one is the no-op). Thread-pool concept. Fiber member masks (the same loop written twice, both
carrying the #340 FIXME).

## What this changes

The engine is substantially less complete than the roadmaps claim and substantially more
*built* than they imply, at the same time, because most of the gap is wiring rather than
construction. Twelve subsystems sit finished-but-unreached.

Three of the four dispatch paths carry per-fiber windows; the codegen path that canon designates
as the real one does not exist in reachable form. The parallel path lacks incremental skip. The
adapt subsystem computes a decision nothing consumes. The plan chain runs steps 1 to 5 and
discards the output of 5, 6 and 7.

The immediate consequence for the roadmap: r6's G2C-0 said to record a supersession and delete the
2-way head+tail types. That is backwards. Those types are the canonical target, and the N-way
branch is the drift to remove. Deleting them would have destroyed the correct mechanism's
substrate.

The next document is the corrected roadmap, ordered from this ledger rather than from any prior
roadmap.
