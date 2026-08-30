# Plan: Phases, Trunks, Fibers, Morsels

The plan stage turns WorkUnit declarations into the execution structure. It
runs once at schedule construction and reruns fully on structural change
(full recompute is microseconds on the const-generic structures; there is no
incremental plan maintenance). Everything here is plan-time analysis with
zero runtime cost; the structural outputs land on the compile-time side of
the static/adaptive split ([[identity]]), the parameter outputs (morsel
sizes, ranges, configs) on the adaptive side.

## Phase decomposition

The access matrix (rows are WUs, columns are stores) is the unifying
representation; every plan operation is a matrix or graph operation from the
arvo crates. The decomposition, in order: build the DAG from Read/Write
overlap, detect phases via waist analysis, identify trunks per phase, assign
branches and bridges, form fibers within each trunk, size morsels per fiber,
select an execution strategy per phase.

A phase is a wide DAG section between waists and executes with pipeline
parallelism against its neighbours. A trunk is a column-disjoint maximal
path within a phase: trunks share no write columns, so sibling trunks need
zero synchronisation. Branches chase within their trunk's morsel scope;
bridges fan in across trunks and run after parent trunks reach the required
record range. The trunk skeleton is the critical path by upward rank: the
highest-ranked WU roots the trunk, the trunk follows the heaviest successor,
everything else becomes branch or bridge.

## The nine plan steps

**Step 1, access matrix** (arvo-bitmask). Each WU's Read/Write tuple types
lower to bitmasks at registration. Edges: `write_masks[i] & read_masks[j] != 0`
means i precedes j; one AND plus compare per pair, O(N^2) on words (about a
microsecond at 50 WUs). Write-write ordering resolves by hint priority per
[[data-model]].

**Step 2, topological sort and renumbering** (arvo-graph). Kahn's algorithm
on the bit-matrix DAG with a fixed-size queue, zero heap. WU IDs renumber to
reverse topological order so the rank traversal in step 3 scans
sequentially, the same prefetcher principle as column co-location.

**Step 3, upward rank** (arvo-graph). `rank(u) = cost(u) + max(rank(succ))`,
bottom-up. Weight per WU is the sum of accessed column sizes (more column
data means more memory ops means more time). The critical path names the
serial bottleneck that gets priority treatment: core pinning, convergence.

**Step 4, waist detection** (arvo-graph). Walk topo order counting live
independent paths; local minima are waists, which define phase boundaries.
Distinct from block-diagonal structure: waists are bottlenecks within a
connected DAG.

**Step 5, RCM reordering** (arvo-sparse, always run). RCM over the access
matrix produces two orderings. The column reordering is the arena memory
layout: co-accessed columns get adjacent offsets in the fiber arena, so the
prefetcher pulls them together; this is the physical meaning of co-location.
The row reordering is canonically the WU execution order, so that the fiber
grouping walk meets column-sharing WUs adjacently and wide fan-out DAGs get
a cache-optimal order among the valid topological orders. The two outputs
have different natures under the static/adaptive split: both derive from
static access structure, and the row order belongs to the locked side,
applied at the static boundary (never reordered by any runtime trigger).

*Row-order fork resolved by bench (A1-1, ruled 2026-07-19; evidence
expanded same day).* The expanded ordering-theory bench (record
`202607200800_a1-1-ordering-theory-bench-expansion.md`) ruled for the RCM
row order against rival theories, not merely against an arbitrary order:
on a grid sharing topology at 64 MiB columns (the DRAM-resident
large-entity regime the engine targets), RCM's bandwidth-minimising order
wins outright, the spectral total-distance order loses 1.17x, and naive
registration order loses 1.65x. Below that scale, dispatch order among
valid topological orders measures near-neutral (~5%), and the earlier
1.10x chain-topology claim is retracted as sampling artifact. The
mechanism is bounded MAXIMUM reuse distance under DRAM-resident working
sets, not generic consecutive-column adjacency. Step 5's wording stands;
the shipped waist-phase dispatch order is measured drift at scale, and
applying the row order (the proven guarded-walk mechanism) dissolves the
provisional registration constraint ([[contracts]]).

**Step 6, block diagonal and Dulmage-Mendelsohn** (arvo-sparse, always run).
Connected components on the WU-column bipartite graph validate the phases
from step 4. Dulmage-Mendelsohn refines: horizontal components are dead-end
WUs whose outputs nothing reads (eliminable unless final consumer output);
vertical components are pure input columns; square components are the
matched core. Dead column elimination shrinks fiber column sets, enlarging
morsels and reducing register pressure, and the codegen skips eliminated
stores entirely.

**Step 7, spectral trunk formation** (arvo-spectral, when a phase has more
than 5 fibers). Build a fiber graph with edge weights equal to the shared
column bytes between fiber pairs; the Fiedler vector bisects it to minimise
cut weight, producing trunks whose internal fibers share heavy column sets
(one core's L1) while the cut between trunks carries only light or read-only
data. At 5 or fewer fibers, one trunk. Trunks may share read-only columns
from previous phases, never write columns.

*Recorded deviation.* Shipped code forms trunks from block-diagonal
connected components per phase and uses spectral to form fibers within wide
blocks instead. This role change is registered under the evidence-then-bless
standard (A2-4). The spectral-versus-greedy fiber bench has run (record
`202607201200`, bench `fiber_theory`: greedy is linear and fiber-grained at
38 to 398 ns; spectral is two to three orders heavier and trunk-grained),
and the delivered evidence proposes restoring canon's Step 7/8 role split
via a plan-chain corrective round; canon's step 7 wording stands, awaiting
the op bless ([[governance]] item 3).

**Step 8, fiber grouping** (arvo-comb). Greedy at 10 or fewer ops (walk the
RCM-ordered topo order; group a WU into its predecessors' fiber if all
predecessors share one fiber and the holistic feasibility check passes,
otherwise open a new fiber with fence dependencies); matrix-chain DP above
10 ops, with cost `record_count * union column bytes` for an interval (the
cost is the memory bandwidth of one data walk) under the same feasibility
predicate, minimising total data walks.

**Step 9, dirty propagation** (arvo-bitmask, runtime). Before each pass,
generation counters on root inputs seed a dirty mask; propagation is an OR
sequence over predecessor masks, one AND per WU (about 50 nanoseconds for 50
WUs). A WU with an entirely clean predecessor set skips execution. This
turns the pipeline from a batch processor into an incremental one.

Deferred algorithms, registered since the founding spec: SpMV patterns,
fill-reducing ordering, common-subexpression detection over the matrix,
Morton/Hilbert dispatch ordering.

## Fiber formation detail

A fiber is the unit of column co-location and morsel windowing; whether its
WUs fuse into one loop body or run as separate passes over the same morsel
is the codegen's call ([[dispatch]]). All fiber columns share one arena
allocation; record index advances one cursor across all columns; a morsel
boundary is one base-pointer update.

The holistic feasibility check is one calculation over competing resources:

1. Register file: arena base pointer, column offset immediates, write
   resource pointers, loop counter, intermediates, and constants must fit
   the usable GPRs (about 14 on aarch64, about 10 on x86-64). The register
   file, not L1 capacity, is the binding constraint on fiber width.
2. L1 write budget: the sum of write column sizes plus write resource
   collection sizes must fit the L1 write budget; read-only data rides the
   L2 prefetcher.
3. L1 plus L2 total: all accessed data fits both levels combined.
4. No fan-in: all predecessors in exactly one fiber; multi-fiber
   predecessors force a new fiber (the mandatory barrier is a bridge).
5. No pipeline breaker: an op needing all input records before any output is
   a materialisation boundary.

Column classification is static, from the Read/Write declarations:
fiber-internal columns (written by one WU, read only by the next in the same
fiber) are register-to-register data paths that dead-store elimination keeps
out of memory entirely, making the fiber a pure function pipeline; fiber
inputs are read from memory at morsel start; fiber outputs are written at
morsel end; a column can be internal and output at once. The fiber's column
budget counts the unique column set across its WUs, not the sum.

Column temperature guides cache budgeting: write-only and read-write columns
are L1 during the morsel; read-only dense columns prefetch through L2;
read-only sparse tolerate L3; read-only shared amortise in shared cache.

Read-only columns shared between trunks have two canonical strategies
(domain 11): snapshot-to-local (copy the shared column per trunk at the
phase transition; about 1.6 percent overhead, simpler, no cross-core
contention) and aligned morsel sync (zero copy, both trunks read the same
data; the fit on unified-memory ARM, complex on NUMA x86). The choice is a
per-target plan parameter, defaulting to aligned morsel sync on
unified-memory hosts and snapshot-to-local where cross-socket traffic
would pay for the copy.

Spanning-tree decomposition within each phase: weight WUs by column
accesses, take the longest weighted path as the trunk, fan-outs follow the
heaviest child with the rest as branches, fan-ins record bridges, recurse
per branch. Natural barriers are mandatory fiber breaks; between them lie
the candidate segments the grouper decides over.

The grouper emits up to three configs per phase, selected later per phase
([[execution]]): MAX_FUSE (group everything that fits; best cache, fewest
fibers; wins where a phase has one clear mega-trunk), BALANCED (split at
compute balance points; about 10 percent extra data walk for about 30
percent parallelism), MAX_SPLIT (every barrier-to-barrier segment its own
fiber; maximum parallelism). The column-count heuristic: under half the
budget, fuse; up to the budget, consider splitting if balanced and cores and
records warrant; over the budget, must split.

## Morsel model

The sizing formula, per fiber:

```
morsel_size = (L1_usable / sum(write_sizes)).clamp(MIN_MORSEL, MAX_MORSEL) & !3
```

`L1_usable` is detected L1 times 0.75; `write_sizes` sums write column
strides plus write resource collection sizes; read-only data does not
consume budget. MIN_MORSEL is configurable, MAX_MORSEL is 8192, and sizes
align to a multiple of four so cache-line alignment holds at any stride.
Whether a WU is morsel-chunked or scalar is inferred from its AccessSet:
any Column in Read/Write means morsel-chunked, Resource/Virtual only means
scalar.

The morsel is a 3D window: records by columns by temporal (frames); the
inner cube tiles the same way (x micro-morsel range by y active columns by
z op index). The
schedule is constructed once and reused across frames; morsel-to-core
affinity pins across frames for warm caches; morsel boundaries stay stable
unless the record count changes. Micro-morsels are the inner tiling level
that activates when peak live data at any op slice exceeds L1 (single-level
is sufficient until ECS-scale column counts).

Change detection rides the morsel: a per-morsel generation counter bumps on
write, unchanged morsels skip, and skips propagate through the DAG so an
unchanged root skips all transitive dependents.

## Data loading

hilavitkutin does not load data, allocate storage, or know record counts
ahead of time. Consumers materialise external data into columns before
execution through loader WorkUnits (`type Read = ()`), which makes loaders
root nodes and otherwise completely ordinary WUs. The consumer manages
storage lifecycle through the provider and the evict/inject APIs; the engine
recomputes plan parameters when records or structure change. Progress
counters between phases are the implicit flow control (phase N+1 cannot
outrun phase N); there is no backpressure mechanism because data is
pre-materialised. Loader placement matters to cache state (the first WU
warms the cache for everything downstream); RCM column layout addresses it
by placing co-accessed columns adjacently.
