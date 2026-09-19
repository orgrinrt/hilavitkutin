# Execution: Threading, Strategy, Adaptation

One engine at every core count ([[identity]]). This chapter states how the
plan's structure executes: the parallelism model, the thread pool, strategy
selection, runtime adaptation, and the performance gate.

## The parallelism model

Parallelism is isolated column-disjoint trunks, one per core, with zero
synchronisation between them. Because sibling trunks share no write columns,
nothing coordinates during a phase; the disjointness is the license.
Cross-trunk synchronisation happens at exactly two explicit places: the
waist (the phase barrier between phases) and the bridge (a fan-in fiber that
runs after its parent trunks reach the required record range). Nothing else
crosses trunks.

A single fiber's records split across cores in exactly one situation: 2-way
head+tail convergence inside a single-trunk commutative phase, two threads
processing from opposite ends and converging in the middle. The split point
is a record boundary (the `mid_record` split, morsel-aligned by
construction, uniform with the full-range case), never a byte or slot
offset. Never an N-way
record or morsel partition. This was re-derived from the spec after
intermediate roadmaps drifted toward record-range distribution (the GATE-2
rechart); the drift is corrected and the trunk-per-core statement is the
governing model. It is proven end to end: the trunk nest devirtualises, two
disjoint trunks on two threads run with zero sync at bit-identical output,
and three compute-bound trunks measured 2.84x on three cores against an
ideal 3.00x.

Pipelined phases overlap through per-fiber progress counters: phase N+1
starts when phase N has produced one morsel, so total time approaches the
maximum phase rather than the sum, with all cores active on different
record windows. Core-pinned trunks stay on their assigned pool thread across
all their morsels for warm L1; leftover threads pick up work in priority
order: convergence tails first, then long branches, then bridges.

## The thread pool

Pool size equals the physical core count; threads spawn once at pipeline
construction and persist, parking between frames. The frame wait parks
immediately by default (bench-decided; deviation 3 below): the bounded spin
pre-roll ships as consumer-tunable machinery (`WAKE_SPIN_BUDGET` on
`RunCfg`, the run-configuration parameter of the scheduler's run entry)
with a default budget of zero, and parking uses the platform wait primitive
(futex on Linux, ulock on macOS). The pre-allocated
pool eliminates the small-N catastrophe: the parallel crossover sits near 2K
records instead of the roughly 50K of spawn-per-frame designs. Thread spawn
overhead itself is negligible; the pool exists for the crossover and the
warm caches.

Heterogeneous cores are detected once at startup (core classes, per-class
cache sizes). Critical-path trunks pin to P-cores, leaf and branch work to
E-cores, and E-cores get proportionally smaller morsel ranges rather than
equal shares (even distribution drags the whole frame to E-core speed;
proportional-to-speed splitting measured 1.81x lower makespan). Thread count
is `min(physical_cores, parallelisable width + 1)`, the plus-one being the
chaser/convergence thread; more threads than parallelisable width is a loss.

Work stealing is not the default. The default is deterministic morsel
assignment, pre-partitioned at plan time with no atomic fetch-add. If
variable per-record cost causes imbalance, the consumer provides a stealing
executor through the `Executor` extension point; the library ships the
deterministic one.

## Frame protocol and barriers

The waist barrier is worker-side and sense-reversing: one publish/await per
frame, workers stay hot across phases and synchronise at waists themselves.
(An earlier main-orchestrated shape, where the main thread serialised phases
with a park/wake round trip per waist, is superseded; the worker-side
barrier is the shipped realisation of the frame protocol.) The designed
predictive parking tiers at sync points (estimated wait under 100ns spins,
up to about 10us spin-loops with backoff, above that parks; a wrong
prediction costs one park/unpark cycle) remain the adapt-phase `pick_tier`
follow-up; the benched park-immediately default is the bar any tier
selection must beat (deviation 3 below).

The N-versus-1 oracle is the parallel acceptance discipline: for a
commutative pipeline, output at N cores must be bit-identical to one core,
partitioned by trunk. The accumulator path runs unit-outer (the outer loop
walks work units; the inner loop folds that unit's per-core accumulator
regions) with per-core regions merged in append order.

## Recorded deviations awaiting evidence-then-bless

Six shipped realisations diverge from canonical wording and are registered
(GATE-2 deviation ledger) under the A2-4 standard: canon shape built or
sketched, benched where a trigger is named, then op rules bless-or-rebuild.
(Numbering note: the set below is the working enumeration; the ledger's own
section numbers differ. Item 2 here is the already-op-blessed ownership
mask, whose bless predates the A2-4 batch; A2-4's own six-item list instead
carries the main-orchestrated waist barrier, which the ledger's 2026-06-08
correction header had already superseded, so this enumeration substitutes
the live item. The batch's evidence channels are
all delivered as of 2026-07-19; see [[governance]] item 5 for the per-item
status and records.)

1. `PoolFrame` inline in the Scheduler with a `Pin` receiver on the parallel
   entry, instead of arena placement (trigger: consumer Pin ergonomics; the
   arena route restores the canonical shape).
2. The runtime core-ownership mask over per-trunk monos instead of fully
   compile-time per-core programs (op-blessed, bench-gated; escalation is
   build-script codegen; see [[dispatch]]).
3. Park-immediately as the frame-wait default: RESOLVED BY BENCH
   (`202607202340`). The canonical bounded spin tier was built
   (consumer-tunable `RunCfg::WAKE_SPIN_BUDGET` on both frame waits) and
   the re-examined seven-arm bench ruled for park-immediately on
   reproducibility grounds (park never worse, no spin policy ever
   better, across three invocations; the first run's point margins were
   variance-inflated and are retired), so the default is 0 and the
   machinery ships tunable; the telemetry-driven `pick_tier` selection
   stays the adapt-phase follow-up with park-immediately as its bar
   (records `202607202340` and `202607210100`, the latter controlling).
4. Pointer-size spawn (the closure must fit a pointer, compile-checked) and
   exit-counter join instead of thread joins (sound; a real limit for
   arbitrary consumer pools).
5. Raw scheduler aliasing: workers hold a type-erased pointer to the whole
   scheduler. The 202607201600 audit REFUTED the claimed
   parked-between-frames soundness: the between-frames `&mut self` public
   surface aliases a parked worker's live `&Scheduler`
   (miscompilation-class, timing-independent), so the whole-plane arena
   relocation (deviations 1 and 6) is soundness-required, not optional; the
   hole is catalogued as an ignored test pending that round.
6. Inline GATE-2 scratch arrays on every scheduler (dead weight for
   single-core consumers; arena relocation is the reconciliation, tied to
   deviation 1).

## Strategy selection

No single strategy dominates. Record count gates first: under 10K records
sequential always wins (thread overhead dominates); 10K to 100K depends on
shape; above 100K threading is always considered. Pipeline shape gates
second: wide shapes (roots exceeding half the depth) suit adaptive or
pipe-chase execution; deep serial shapes stay sequential (threading a deep
pipeline measured as pure overhead; serial dependency depth is a fundamental
limit); mixed shapes with parallel trunks go phased. The design's target
envelope spans three consumer scales (hundreds of records with deep
sequential shapes; tens of thousands mixed; hundreds of thousands to ten
million wide and phased), and the scale gates above were calibrated across
that envelope.

Per-phase config selection picks among the grouper's MAX_FUSE, BALANCED,
and MAX_SPLIT configs ([[plan]]) independently per phase: a clear mega-trunk
phase takes MAX_FUSE plus convergence, wide independent branches take
MAX_SPLIT, moderate shapes take BALANCED.

Plan-time strategy selection estimates weight ratios from WU counts times
column accesses (rough light/medium/heavy classes; a wrong pick costs 10 to
20 percent, not correctness): under 10K records sequential; consumer weight
over half producer weight, chase-steal; light producers, adaptive; otherwise
pipe-chase. The LIGHT_THRESHOLD constant is named by canon and deliberately
unvalued: a bench sets it. Frame budget controls incrementality only, not
throughput, and matters only to time-budgeted loops.

The schedule is constructed once and reused across frames; morsel-to-core
affinity pins across frames; boundaries stay stable unless record count
changes.

## Runtime adaptation

Two analysis tiers. Static, at plan time: per-fiber cache pressure, data
flow volume between fibers, column lifetimes, peak memory watermark.
Runtime, between frames: per-morsel timing, change frequency (hot, warm,
cold morsels from generation counters), cache residency prediction, frame
time prediction, throughput trending (thermal throttling, memory pressure).

Metrics ride a `SchedulerMetrics` resource. Designed field set: pass
count, EMA pass duration, per-unit EMAs, active units, stolen count, idle
time; the shipped resource currently carries pass count, EMA pass duration,
last record count, change-seen count, and idle time, with the per-unit EMA,
active-unit, and steal fields landing with the adapt build. EMA decay is
one eighth per pass, batch-updated vectorised. The clock feeds it through the
builder slot ([[foundations]]).

The three reorganisation triggers evaluate between frames, never during
execution, as cheap mask comparisons against EMA thresholds: significant
fiber morsel-timing drift recomputes morsel sizes; phase balance shift
re-selects per-phase configs among the pre-monomorphised variants by runtime
index; record-count change (including a resource swap that marks the plan
dirty) triggers the parameter-side plan recompute. All three land on the
parameter side of the static/adaptive split; nothing adaptive ever reorders
dispatch or regroups fibers. Morsel temperature feeds core assignment (hot
morsels to P-cores, cold to E-cores or deferral). Adaptation hangs off the
meta pipeline's lifecycle points: the trigger evaluation is itself WU work
gated on `meta::ScheduleEnd`, between one frame's end and the next plan
stage ([[scheduler]]).

Cadence policy stays consumer-side: the engine measures and exposes the
metrics resource; the consumer reads it in an `On<meta::ScheduleEnd>` WU and
implements its own cadence.

## The performance gate

The gate is blocking and standing. The baseline is optimally threaded
standard-library code at equal core count with a persistent pool matching
the engine's spawn-once discipline. The persistent-pool baseline is A1-8's
ruling; the fairness correction's actually-run 2026-06-08 baseline was a
spawn-per-frame `thread::scope`, and the earlier single-threaded baseline
had overstated parallel results as a 3.5x win, a superseded claim. Honest
parity against parallel std is the required bar; the persistent-pool arm
runs as bands land. Bars are per-arm calibrated (tight where the
engine should win, parity tolerance mid-range), measured median-of-N; the
flat single-ratio bar is retired (A1-8). Red arms that a later roadmap band
resolves stay red as standing oracles per the strict-by-design rule; they
are the measurement, not the problem.
