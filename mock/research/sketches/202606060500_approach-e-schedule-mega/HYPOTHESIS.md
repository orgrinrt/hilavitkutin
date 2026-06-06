# Hypothesis: Approach-E schedule-mega single-core body (Phase D / #340)

The body probe (`202606052130_rcm-order-into-static-body`) settled that the
column-capable inline `RunFiberCol` walk devirtualizes for a branching DAG with
a multi-column join, and is order-agnostic: any statically-known cons-list
order, including an RCM permutation of registration order, compiles into one
straight-line body with no stored fn pointer. It ran one whole-range pass
(unit-outer, the morsel was the entire record range).

The single-core direction chosen in that sketch's FINDINGS is the consolidation
spec's dispatch Approach E (schedule-mega, "all trunks in one fn", spec
L1551-1615, preferred for >10K records). For single core, Approach E removes the
type-level fiber partition entirely: one monomorphised body walks the schedule,
and fiber/phase boundaries plus morsel sizes are in-body runtime control flow
and compile-time constants, which is the spec's "compiled per-core dispatch"
(L1596-1613). The body probe did not exercise that shape. Two structural
questions remain open:

1. Morsel-outer dispatch. The schedule-mega body wraps the inline walk in a
   morsel loop, calling the same walk per morsel sub-range (`EngineCtx::project`
   already takes the morsel; `each()` iterates exactly it). Does the walk still
   devirtualize when it is inside a runtime morsel loop, rather than run once
   over the whole range?

2. Multi-phase body. A phase boundary on single core is a sequence point: all
   records of phase P complete before phase P+1 begins (because P+1 reads, with
   a cross-record dependency, a column P wrote). Expressed as in-body control
   flow this is two sequenced per-phase morsel loops inside one fn. Does a body
   with two phases, each its own morsel loop wrapping its own inline walk,
   devirtualize end to end with no indirect dispatch at any phase or unit
   boundary?

The probe builds a two-phase schedule: phase 0 is the diamond (BranchX, BranchY,
JoinZ) producing Zv; phase 1 is NormW reading Zv and writing Wv. It runs both
phases morsel-outer inside one `#[inline(never)]` `run_schedule_mega` fn whose
only bounds are the two `RunFiberCol` bounds (the witnesses stay free generic
params, inferred at the call site). If it type-checks, runs correctly, and the
release disassembly of `run_schedule_mega` shows no surviving dispatch symbol
and zero indirect calls, then the multi-phase morsel-outer schedule-mega body is
feasible and the single-core assembly problem reduces to "build the flat
cons-list(s) in the plan's RCM-reordered topo order", with the within-level RCM
order among independent equal-depth units (BranchX vs BranchY first) a benched
perf question rather than a structural one.

Phase 1 here is element-wise on Zv; step-8 grouping would fuse an element-wise
dependent into phase 0. It is placed in a separate phase to exercise the
multi-phase body structure (two sequenced morsel loops), which is the codegen
shape step-8 emits for a genuine barrier (a reduction or a cross-record scan).
The devirt result is independent of whether this particular phase 1 needs the
barrier; the barrier's codegen is the same either way. A genuine cross-record
phase 1 needs either a cross-morsel read (awkward through the morsel-relative
`each()`/reader API) or an accumulator (the non-nil `AccumProject` tie, the
deferred residual MEMORY LATEST-55 flags for SRC-time confirmation), so it is
out of scope for this body-structure probe.

The bench question: at scale (>10K records, morsel-chunked), does the
within-level RCM order (phase 0 built BranchX-first vs BranchY-first, both
topo-valid) move single-core runtime measurably? The two branches touch disjoint
columns (Xv, Yv), so the order changes only which disjoint column is written
first within a morsel. The answer steers whether the SRC slice realises the
plan's RCM within-level reorder or accepts validated registration order on
single core (deferring the RCM within-level freedom to GATE-2 parallel, where it
maps to core assignment).
