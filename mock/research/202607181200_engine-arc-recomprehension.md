# Engine-completion arc: re-comprehension against canon (chart-the-path phases 2-4)

**Date:** 2026-07-18
**Status:** synthesis of an independent comprehension pass and a source-verification pass, run because the recorded next step (A4) needed confirmation that the sequence around it is still true.
**Oracle:** consolidation spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` (22 domains, R1-R9).
**Prior roadmap under test:** `202606201700_arc-audit-and-updated-roadmap.md` (phase sequence) plus `202606201900_granular-roadmap-and-sketch-plan.md` (granular leaves), as amended by the 2026-07-02 canon-alignment pass.

## Verdict

The phase sequence is sound and does not need re-ordering. The arc is on the canonical path, and
the two large corrections it was built to make (the per-fiber morsel model in Phase A, the
plan-analysis chain in Phase B) are still the right corrections in the right order.

What the pass did find is that the roadmap's **status ledger** has gone stale in one load-bearing
place, and that four canonical leaves are absent from every phase. The stale entry matters more
than the missing leaves, because it would have sent the next slice into a collision: the roadmap
records head+tail convergence as unbuilt, and something occupying that name is in fact shipped, in
a shape that diverges from canon on every axis the spec constrains.

Everything below is verified against source. Where an intermediate artefact and the source
disagree about shipped state, the source is the fact, per `cl-claim-sketch-discipline`. Where the
source and the spec disagree about intent, the spec is the design, per `design-is-the-oracle`.

## The one that matters: head+tail convergence is shipped, drifted, and mis-named

The roadmap (`202606201700:130`) records head+tail convergence as PROVEN-but-UNBUILT and schedules
it as G2C-1 (a sketch proving it can be a const-selectable third dispatch mode) followed by G2C-2
(wire it into `worker_main`). Source contradicts the status: `scheduler/mod.rs:2107-2132` already
dispatches a record-range split, labelled "Head+tail convergence (spec :770)", whenever
`tphase.0 == 1 && ncores.0 > 1 && total > 0`.

The canonical statement is narrow and constrains four things. Spec `:770` reads "Single-trunk
phases: head+tail convergence (2 threads, opposite ends, ~2x parallelism). Skip for non-commutative
resource accumulation." Spec `:1838-1844` adds "Two threads process same commutative fiber from
opposite ends... Non-commutative resource accumulation gives skip convergence for that fiber
(domain 08)."

The shipped branch diverges on all four. It is an N-way ceil-slice across every core rather than
2-way. Every core walks in the same direction, so there is no opposite-ends convergence and no
meeting point. There is no commutativity gate: the runtime condition tests single-trunk only. And
the plan's own eligibility record is never consulted. `Fiber.head_tail: Maybe<HeadTailConvergence>`
(`plan/fiber.rs:203`) is computed at plan time, its doc (`plan/fiber.rs:147-154`) states eligibility
as "COMMUTATIVE, single-trunk-phase, record-count-threshold-met, accumulation-compatible" and
promises that "codegen lowers to a two-ended projection with a deterministic merge at the
convergence point", and no such codegen exists.

The canonical mechanism therefore has a full plan-side and thread-side surface that nothing
consumes. `thread::Convergence`, carrying exactly the canonical shape (`head_thread`, `tail_thread`,
`meeting_record: ProgressCounter`, `thread/convergence.rs:14-19`), is re-exported at
`thread/mod.rs:35` and reached by no engine code; its one construction is a test
(`tests/core_assignment.rs:58`), so a deletion touches that test.
`HeadTailConvergence`, `MergeOp`, and the head/tail `AccumSlot` pair are plan-only in the same way.
Note that `ConvergenceBuffer` is a different type and is genuinely live; do not sweep it in.

The drift is also written into the design doc, which now contradicts itself across 300 lines.
`mock/crates/hilavitkutin/DESIGN.md.tmpl:291` describes the shipping contract as "The head+tail convergence branch (a
single-trunk waist-bounded phase split by record range across cores)", which is the N-way shipped
form. `mock/crates/hilavitkutin/DESIGN.md.tmpl:616` restates canon as "Head+tail convergence: two threads process same
commutative fiber from opposite ends. Every commutative fiber gets ~2x parallelism." An intermediate
implementation round redefined a canonical term in place, which is precisely the failure mode
`canonical-design-outranks-intermediate-rounds` exists to catch.

This is very unlikely to be a live correctness bug today. Accumulator-bearing carriers divert to
`worker_accum_unit_outer` through `carrier_unit_outer()` (`scheduler/mod.rs:873`) before reaching
this branch, and a pure per-record column write is safe under any record partition. But that safety
is incidental rather than enforced: the eligibility the plan computes, which is what would enforce
it, is discarded.

Two things follow for the roadmap. G2C-1 and G2C-2 are not "build the unbuilt mechanism"; they are
"reconcile a shipped divergent mechanism against canon, then wire the plan's eligibility into
dispatch". And G2C-1's stated purpose, proving head+tail can be a const-selectable third mode
"without a per-dispatch runtime branch", is now aimed at replacing exactly the runtime branch that
shipped in its place.

One genuine design question falls out: whether N-way is strictly better than the spec's 2-way, since
it engages every core rather than two, or whether the 2-way opposite-ends shape exists for reasons
the spec did not spell out, such as the accumulator merge being a 2-way converge, cache and NUMA
locality, or deterministic merge order.

**Resolved, same day, and the resolution inverts the framing above.** The section as written assumes
the two shapes compete for the same workload. They do not. `carrier_unit_outer`
(`scheduler/mod.rs:2056-2068`) diverts any carrier containing a non-`morsel_local` fiber, meaning
any fiber that writes an accumulator, to `worker_accum_unit_outer` before the phase walk. Canon's
head+tail case is a commutative fiber accumulating on a single-trunk phase, so it never reaches the
branch analysed above. It is served by the accumulator path, which performs the same ceil-slice
partition (`:960-962`), gives each core its own accumulator region (`rebase_accums`, `:971`), and
merges at the frame's existing publish boundary. The merge is order-preserving, not merely
associative: it forward-compacts in ascending core order and cores own ascending record slices, so
the merged sequence is the record order (`resource/bindings.rs:660-664`). That removes the very
constraint behind canon's non-commutative carve-out, since a 2-way opposite-ends converge cannot
preserve record order but a forward N-way partition can.

So the canonical mechanism is superseded rather than missing, and the branch analysed above only
ever sees accumulator-free carriers where every record is independent and there is nothing to
converge. The findings that survive are the naming collision, the dead 2-way surface, the
self-contradicting design doc, and the unrecorded supersession. The finding that does not survive is
"the canonical mechanism is unbuilt". See `202607181300_engine-roadmap-r6.md` for the resolution and
the three real gaps the same pass found in the shipped N-way form.

## Canonical leaves absent from every phase

Four mechanisms are in neither the shipped source nor any roadmap phase. They are not in the
original audit's S-1 through S-8 skip list either, so nothing currently tracks them.

**The parallel path has no incremental skip.** `run_core_phase` states it outright at
`scheduler/mod.rs:2102`: "All-ones dirty: run_parallel dispatches the pure-RAW path (no incremental
skip), so every owned member runs." Canon Step 9 (`:1418-1428`) places incremental skip "at runtime
(not plan-time), before each pass" and describes it as what "transforms the pipeline from a batch
processor to an incremental processor". It states no parallel exemption, and domains 12 and 16 both
treat it as a pipeline property. So the parallel path silently loses a canonical mechanism the
single-core path has, and the loss is invisible to the roadmap.

**`run_fused` still windows uniformly.** There are three record-dispatch paths, and Phase A named
two. `run` was inverted to per-fiber windows by A2b, the worker path
(`worker_main:865` into `run_core_phase`) is A4's target, and `run_fused` (`scheduler/mod.rs:2227`)
still takes `Cfg::MORSEL_SIZE`. A fused linear chain is one fiber, so its window should be that
fiber's plan-baked L1 window rather than the const fallback. Phase A as written never mentions it.

**The accumulator unit-outer path bypasses per-fiber windowing entirely.**
`worker_accum_unit_outer` walks each unit over the whole per-core region. Whether the domain-12 L1
window applies inside that walk is unstated in both the blueprint and the roadmap. Silence here is
a decision waiting to be made by default.

**Consumer-driven substrate widening has no lane.** `Plannable` and `HintExt` landed on 2026-07-17
(`bad6ee64`, rounds `202607171807` and `202607171824`, sketch
`sketches/202607171733_plannable-declaration-split/`), lifting `WorkUnit`'s declaration half into a
trait a non-fiber runner can implement, driven by a downstream GPU renderer that needs to declare
and plan work without the column-record execution model. The work is additive, breaks no existing
implementor, and is correct per `use-the-stack-not-reinvent`. The point is not that it was wrong;
it is that the roadmap could not see it. Phase E sits last on the internals-first rationale that
"consumer paths are mostly sugar atop working internals", and this consumer pull was not sugar. It
was a contract split in the api crate. More of the same shape should be expected, and it deserves a
named recurring lane rather than arriving as an unplanned insert each time.

## What the roadmap gets right, confirmed against source

Phase-overlap is genuinely unbuilt, exactly as recorded. The substrate is all there:
`ProgressCounter` (`dispatch/progress.rs:32`), `emit_progress_release_fence`
(`dispatch/sync.rs:47`), `PoolFrame.progress_slots` (`hilavitkutin-api/src/platform.rs:167`), and
per-core slot assignment (`plan/core_program.rs:91`). Nothing in the dispatch loop consumes it; the
scheduler's standalone frame sets `progress_slots: NonNull::dangling()` (`scheduler/mod.rs:802`) and
the worker path uses strict `waist_barrier` only. G2C-3 and G2C-4 stand as written, including the
narrowed sketch scope the 2026-07-02 pass gave them.

Phase B is absent as recorded. `k_way_partition` is called (`plan/steps.rs:467`) but its output is
explicitly unconsumed, stated in the source at `plan/steps.rs:448`: "the runner does not consume
this output yet". Neither `matrix_chain_dp` nor `dulmage_mendelsohn` appears anywhere in engine
source, so fiber grouping is greedy-only. The 2026-07-02 pass already resolved B1a (the DP cost
function is spec Step 8's `record_count x sum of size_of::<T_k>()` over union columns) and B3a
(arvo-sparse ships the Dulmage-Mendelsohn surface, no cross-repo PR needed), and both resolutions
survive this check.

Phase C is as recorded. `rcm_reorder` runs (`plan/steps.rs:344`); dispatch consuming the row order
is the open correction, and the C1 sketch is still required at exact leeway.

Phase E's gaps are real and unchanged. `PipelineResult`, `read_slice`, `write_slice`, and
`morsel_range` have no definitions anywhere in the crates.
`hilavitkutin-persistence/src/cold_store.rs` remains a skeleton whose `flush` and `snapshot` are
no-ops and whose `load` returns `PersistenceError::Missing`.

## Current measured state

The engine suite is green: every test binary reports zero failures, with 10 tests catalogued red
via `#[ignore = "catalogue:`, each naming its gap and a tracking task (seven adapt performance
contracts plus one window-floor case and one fiber-plan-index case under #341, one collection-member
case under #344). That mapping is the catalogue discipline working as intended. One ignore reason
has gone stale: `a1_fiber_morsel_size.rs:121` says its case "lands with A2", and A2 has landed, so
the case is now actionable rather than blocked.

Four unused-import warnings are present in the engine build, appearing to date from the most recent
merge. Minor, but they are new drift.

The working tree carries one unstaged `.gitignore` addition written by the mockspace bootstrap (a
catch-all `target/` rule). Tool-generated rather than in-flight work, but it needs a branch and a
PR rather than a direct commit to trunk.

## What this changes

Nothing in the phase order. A4 remains the correct next slice, and the internals-first sequence
behind it holds. The changes are to the ledger and to the leaf set: G2C's framing flips from
construction to reconciliation, four leaves join the map, and one design question about N-way versus
2-way head+tail needs an answer before G2C starts, since it determines whether that phase replaces
the shipped branch or canonises it.
