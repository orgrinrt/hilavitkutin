# Sketch: what decides worker participation once the N-way record split is gone

**Date:** 2026-07-19
**Hypothesis:** deleting the N-way ceil-slice from `run_core_phase` (roadmap R1) is a
self-contained removal of drift, affecting only that branch.
**Outcome:** **the hypothesis was correct.** Three passes argued otherwise and all three were wrong.
The record-slice guard they were built on does not gate the path R1 touches.

## Read this first: the sketch's own conclusion was wrong for most of its life

The body below is preserved in the order it was written, because the sequence is the useful part.
It ran: hypothesis, three escalating refutations, then a test that collapsed all three. The final
answer is at the bottom under "Fourth pass".

Summary of the error: `run_this = lo < hi` (`scheduler/mod.rs:966`) is inside
`worker_accum_unit_outer`, not the general worker loop. It gates the **accumulator** path only. The
non-accumulator path calls `run_core_phase` unconditionally for every core, every phase, and says so
at `:913`: "every worker participates in each waist, even one that owned no trunk this phase". The
two ownership models never coexist, so they cannot disagree.

Every pass below that reasons about them disagreeing is reasoning about a configuration that does
not exist.

## What the trace found

`ceil(total/ncores)` appears at three sites in `scheduler/mod.rs`, and they are not one mechanism.

`:2119`, inside `run_core_phase`'s single-trunk branch. This is the dispatch record split, the
drift R1 targets. Canon requires 2-way head+tail here, not N-way.

`:1923`, in `run_parallel`, feeding `merge_accums`. This is the accumulator region layout, and it
is canonical: `202606111800:452-456` specifies exactly this, one exclusive region per core, merged
after the workers rejoin.

`:960`, in the worker entry, and this is the one that breaks the hypothesis. It computes the same
slice and uses it twice:

```rust
let per = (total0 + ncores0 - 1) / ncores0;
let lo  = (core.0 * per).min(total0);
let hi  = (lo + per).min(total0);
let run_this = if total0 == 0 { core.0 == 0 } else { lo < hi };
if !run_this { return; }
let region = hi - lo;
let per_core = s.bindings.rebase_accums(USize(lo), USize(region));
```

`run_this` is the **participation guard**. A core whose record slice is empty returns before
dispatching anything.

## First conclusion, and why it was wrong

The reading above led to a prediction: since the two ownership models disagree about which cores
are idle, a core owning a trunk but holding an empty record slice would return early and strand its
work. A test was written to demonstrate it, five column-disjoint producers against one record on an
eight-core machine, so that only core 0 passes the guard.

**It passed.** The predicted failure does not happen, and the prediction is withdrawn.

The reason is the branch this round set out to delete. A single-trunk phase takes the N-way branch,
which dispatches `phase_mask`, the mask for the **whole phase**, not the mask for the trunks this
core owns. So every participating core runs every unit in the phase, and a core that opted out
strands nothing, because nothing depended on it specifically.

That is worth stating carefully, because it changes what the branch is. It is not only a record
split. It is also the reason participation and dispatch ownership are allowed to disagree: whole-
phase dispatch makes the disagreement harmless. Remove the branch and that cover goes with it.

The test is kept, passing, as a regression. It pins a property that holds today for a reason that
R1 removes.

## Why deleting `:2119` alone would still be a correctness bug

The two ownership models disagree about which cores are idle.

Under record-slice ownership, a core is idle when its slice is empty, which happens when
`ncores > total`. Under trunk-rank ownership, which is what the ordinary branch uses
(`rank % ncores == core`), a core is idle when no trunk's rank maps to it. These are different
predicates over different quantities.

Delete `:2119` and every phase falls to the trunk-rank branch, which dispatches only the trunks a
core owns. `:960` still gates participation on the record slice. Now the disagreement bites: a core
with an empty slice returns early while remaining the rank owner of one or more trunks, and with
whole-phase dispatch gone there is no other core running them. The units are silently dropped: no
panic, no assert, a frame that completes with missing output.

This is the shape the failed prediction was reaching for. It was wrong about the timing, not the
mechanism: the hazard is created by the deletion rather than already present. The regression test
added this round is exactly the case that flips from passing to failing at that point, which is why
it is worth keeping.

## What this means for R1

R1 is not a deletion. It is a change of ownership model in the worker entry, and it has three
parts that have to move together:

1. **Participation.** Under trunk-rank ownership a core participates if it owns any in-phase trunk.
   The rank walk already computes exactly that, so the guard becomes a property of the dispatch
   walk rather than a precondition checked before it. The cheap correct interim is to let every
   core enter and let the rank filter decide, accepting the wasted entry for a core that owns
   nothing, and revisit if it shows up in a bench.

2. **Accumulator regions.** These stay, and they stay N-way, because that is what canon asks for.
   But `lo_c` currently derives from the same slice that gated participation, so once participation
   is decided differently, the region layout needs its own derivation rather than inheriting one.
   It still tiles `[0, total)` one region per core; it just no longer means "the records this core
   will process".

3. **The dispatch branch.** Only now can it go, and single-trunk phases run serially until W1
   builds head+tail.

## Consequence for the roadmap

R1 splits. The deletion is the last step, not the only one, and it is gated on the participation
change landing first. The r7 entry describing R1 as "delete the N-way ceil-slice" and estimating it
at ~26 lines was wrong about both the shape and the risk: the lines are easy, and removing them
without the ownership change silently drops work.

Worth noting how the error was nearly made. The audit traced this branch by reading it, found it
self-contained, and classified it as drift with a local fix. It is self-contained as a *branch*.
The coupling is through a formula duplicated at a distance, in a different function, with a comment
("mirrors run_core_phase's split") that describes the duplication accurately and is easy to read
as incidental rather than load-bearing. Grepping the formula rather than reading the branch is what
surfaced it.

## Third pass: the accumulator is built on record-slice ownership, so R1a is not local either

Continuing into R1a turned up a constraint neither earlier pass saw.

`tests/gate2_accumulator.rs` states the shipped design in its own module doc: "each core takes its
head+tail record slice, appends into its own per-core region of the reserved buffer (offset to the
slice start, fresh live cell), and a post-frame forward compaction merges the per-core regions".
The region sizing depends on it. A core's region is `hi - lo`, and that is sound only because the
core appends at most once per record **in its own slice**, under the global at-most-one-append-per-
record bound.

Move participation to trunk rank and that stops holding. Under trunk-rank ownership a core walks
**every** record for the trunks it owns (`start = 0; while start < total` in the ordinary branch),
so a core owning an appending trunk can append up to `total` times. A region of `total/ncores` is
then undersized by a factor of `ncores`.

So the two ownership models are not interchangeable at the accumulator. Record-slice ownership is
what makes the region arithmetic work.

## The fork this opens

**Option A: regions sized for the worst case.** Every core gets a full `total`-capacity region.
Correct under any ownership model, and costs `ncores` times the accumulator memory. On an 8-core
machine an accumulator over a million records goes from one buffer to eight.

**Option B: keep record-slice ownership for accumulator-bearing phases.** The dispatch split becomes
2-way head+tail per canon, and the region layout stays keyed to the records a core actually walks.
Cheap, but it means dispatch ownership is not uniform: accumulator-bearing phases keep a
record-partitioned shape while others use trunk rank.

**Option C: the reading that dissolves it.** Canon's two-way rule is about *which threads walk the
records*; the accumulator's N-way regioning is about *where appends land*. If head+tail means only
two threads ever walk a commutative fiber, then only two cores ever append, and the other regions
are unused rather than undersized. Under this reading nothing needs resizing, R1a shrinks back to
the participation guard alone, and the N-way region layout is simply over-provisioned.

C is the most likely correct reading and it is not established. It depends on whether an
accumulator-bearing phase is always single-trunk commutative, which the plan decides, and I have not
traced that. Do not build against C without confirming it.

## Fourth pass: a test, and everything above collapses

The fork was settled by building it: an accumulator WU in a phase carrying a second, column-disjoint
trunk, run under `run_parallel` on eight cores against 256 records
(`gate2_accumulator.rs::accumulator_in_a_two_trunk_phase_matches_single_core`). It passes, matching
the single-core reference exactly.

Tracing why produced the real structure, at `scheduler/mod.rs:873`:

```rust
if s.carrier_unit_outer().0 {
    worker_accum_unit_outer::<...>(s, USize(core_id), ncores, total);
    frame_done_arrive(&s.pool, ncores);
    continue;
}
```

An accumulator-bearing carrier runs unit-outer and **`continue`s**. It never reaches
`run_core_phase`. And `worker_accum_unit_outer` begins at `:933`, which means every line this sketch
built its argument on, `per` at `:960`, `lo` and `hi` at `:961-962`, `run_this` at `:966`,
`rebase_accums` at `:971`, is inside that function.

So the record slice is the accumulator path's ownership model, entirely. The non-accumulator path
(`:900-920`) has no record-slice gate of any kind: it calls `run_core_phase` for every core on every
phase, and its own comment at `:913` states the invariant plainly.

**Therefore:**

- There is no participation-versus-dispatch disagreement. The first pass invented it by reading a
  guard from one path as though it applied to the other.
- The whole-phase-mask branch is not "cover" for a disagreement. The second pass invented that to
  explain why the first pass's predicted bug did not reproduce. The bug did not reproduce because it
  was never possible.
- The accumulator region sizing is sound by construction and unaffected by anything R1 does, since
  accumulator carriers never enter `run_core_phase`. The third pass's fork does not exist; its
  option C was right for a reason it did not identify.
- **R1 is what r7 originally said: a deletion.** Removing the branch leaves single-trunk phases
  serial until W1 builds head+tail, and strands nothing.

## Status

**Hypothesis CONFIRMED at the fourth attempt.** Three prior conclusions in this document are wrong
and are kept rather than deleted, because the failure mode is the finding: each pass reasoned from
code near the target instead of tracing which function the code was in, and each produced a
confident, internally coherent, wrong answer that the next pass then built on. The `run_this` guard
was read four separate times before anyone asked which function contained it.

The earlier framing of the fork, now void:

**INCONCLUSIVE on the fork.** The participation finding (first pass) and the whole-phase-mask cover
(second pass) both stand and are verified. The accumulator sizing question is open, and it is the
thing that decides how large R1a is: under C it is a small change, under A it is a memory-model
change to the accumulator, under B it is a permanent split in the dispatch ownership story.
