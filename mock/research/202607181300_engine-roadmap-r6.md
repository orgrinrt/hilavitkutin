# Engine roadmap r6: corrected ledger and completed leaf set

**Date:** 2026-07-18
**Status:** chart-the-path phase 5 draft, pending the canonical-mirror pass and the granularity pass.
**Supersedes:** the status claims in `202606201700_arc-audit-and-updated-roadmap.md` and
`202606201900_granular-roadmap-and-sketch-plan.md`. The phase order in both is unchanged and
carried forward. Detail not restated here still lives in those two documents and in
`202606201400_per-fiber-morsel-blueprint.md`.
**Grounded on:** `202607181200_engine-arc-recomprehension.md` (the source-verified findings).

## What changed from r5

The phase order survives untouched: A, then G2C, then B, C, D, E, F, G, H, internals before
consumer surfaces per op's 2026-06-19 resolution. What changes is inside the phases.

G2C is substantially rewritten. Its headline item, head+tail convergence, was recorded as unbuilt
for a month while its actual workload was already being handled correctly under a different name a
few hundred lines away. The canonical 2-way mechanism turns out to be superseded by an N-way
generalisation rather than missing, so the phase loses a build it did not need and gains three real
gaps in the shipped form that nobody had looked for, plus the const-path devirtualisation work that
was the original point of G2C-1.

Five canonical leaves that sat in no phase are placed: the fused path's uniform windowing, the
accumulator path's unwindowed walk, the parallel path's absent incremental skip, the plan-recompute
trigger, and a standing lane for consumer-driven substrate widening. Two catalogued-red tests that
had no phase home get one. And a doc-level off-by-one between the spec's step numbering and the
source's is flagged before Phase B and C start reasoning by step number.

## Phase A: per-fiber morsel model (in progress)

A1 (the `FiberDispatch.morsel_size` field), A2a (the sketch), A2b (the loop inversion in `run`),
and A3 with A3b (the L1 window formula and the step reorder) are shipped. Two leaves remain, and
one is new.

**A4, parallel per-fiber sizing.** Thread the plan-baked per-fiber windows into
`run_core_phase` and `worker_main` through a unit-to-fiber-size lookup, which needs a `gate2_fiber`
mapping alongside the existing `gate2_phase` and `gate2_trunk` scratch (`scheduler/mod.rs:731`).
`Cfg::MORSEL_SIZE` stays the fallback for a fiber with a zero window. Blueprint slice 5.

A4 carries an ordering hazard against G2C-0, surfaced by the canonical-mirror pass. `run_core_phase`
takes a single scalar `msize` (`scheduler/mod.rs:2070-2079`), and the head+tail branch consumes that
same scalar while dispatching over a whole-phase mask rather than a single fiber
(`scheduler/mod.rs:2106-2131`). A per-fiber window does not map onto that branch, because the branch
is not walking one fiber. A4 must therefore explicitly leave the head+tail branch on the scalar and
say so in the source, rather than silently changing the windowing of a path whose shape is still
under review.

**A4 keeps its place ahead of G2C-0**, settled here rather than left open. An earlier draft offered
moving G2C-0 first as an alternative, which was a fork with no criterion attached. The criterion is
that G2C-0 is bookkeeping (a ledger entry, deletions, a doc fix, renames) and changes no dispatch
shape at all. The shape change is G2C-1, which sits much later. So reordering buys A4 nothing, and
the ordinary phase sequence stands.

**A5, the fused path (new).** `run_fused` (`scheduler/mod.rs:2227`) is the third record-dispatch
path and still windows by `Cfg::MORSEL_SIZE`. A fused linear chain is one fiber, so it should take
that fiber's plan-baked window. Small, mechanical, and it closes Phase A's coverage of the dispatch
paths rather than leaving one silently on the old model.

**A6, the accumulator unit-outer window decision (new).** `worker_accum_unit_outer` walks each unit
over the whole per-core region as a single `MorselRange` and never windows. Whether the domain-12 L1
window should apply inside that walk has two defensible answers, so it is not a decision to take by
default at implementation time. Windowing preserves the L1 residency the formula exists to buy;
not windowing keeps the accumulator append sequential over one contiguous region, which is what the
per-core region was carved out to be.

**Resolution: bench, folded into G2C-M**, which already measures `worker_accum_unit_outer`'s scaling
curve. Add a windowed-versus-unwindowed arm at the same record counts and fiber shapes. If windowing
wins on the bandwidth-heavy fiber it applies; if it is inside noise, the simpler unwindowed walk
stays and the answer is recorded rather than left silent. A6 is therefore a decision leaf whose
evidence is produced by a bench that has to run anyway, not a separate measurement.

**A6 is the one leaf that cannot close inside its own phase**, and that is deliberate rather than an
oversight. Its evidence comes from G2C-M, which sits in the next phase, so under the A-then-G2C
order A6 stays open across the boundary. The consequence to hold: the accumulator walk keeps its
current unwindowed shape through Phase A, recorded as provisional in the source rather than as a
settled default, and the decision lands when G2C-M runs. Pulling G2C-M forward into Phase A is the
alternative and is rejected, because the bench needs the parallel work G2C-1 has not done yet to be
measuring the shape the engine will actually ship.

Scope note from the granularity pass: that function also contains a core-participation ceil-slice
(`scheduler/mod.rs:959-966`, commented in-source as "mirrors `run_core_phase`'s split"). That split
is the same mechanism G2C-0 governs, at a second call site, and it belongs to G2C-0's scope rather
than A6's. A6 is only the windowing question inside the walk.

A4 note, same pass: `gate2_phase` and `gate2_trunk` are flagged in-source as awaiting a capacity
lift onto `Units` (`scheduler/mod.rs:103`, tracked at #690). A4's `gate2_fiber` array either adds a
third fixed `[USize; GATE2_MAX_UNITS]` to the pre-lift shape or lands mid-lift.

**Settled here rather than left to implementation time: add the third fixed array to the pre-lift
shape.** #690 is an independent capacity refactor with its own blast radius, and coupling A4 to it
would block a small slice on unrelated work while enlarging both. The third array is mechanically
identical to the two beside it, so it moves with them when #690 lands, and the cost of having
guessed wrong is one line in that later refactor. If #690 happens to land first, A4 simply follows
the lifted shape instead; the decision only binds while both are pending.

## Phase G2C: reconcile and complete the parallel model

**G2C-0, record the head+tail supersession (new, and it gates the rest of the phase).** The
execution-shape pass settled the question this roadmap first raised as open, and the answer inverts
the framing. Canon's 2-way head+tail is not missing. It was superseded by a strictly more general
N-way form, and the supersession is defensible on the merits; what is missing is any record that it
happened.

The argument, verified against source. `carrier_unit_outer` (`scheduler/mod.rs:2056-2068`) returns
true when **any** fiber in the carrier is not `morsel_local`, meaning any fiber writes an
accumulator, and such a carrier diverts whole to `worker_accum_unit_outer` (`scheduler/mod.rs:873`
tests it, `:882` dispatches it) before the phase walk. So the canonical head+tail case, a commutative
fiber accumulating on a single-trunk phase, never reaches the `tphase == 1` branch at all. It is
handled by the deviation-9 accumulator path, which does the same ceil-slice partition
(`scheduler/mod.rs:960-962`, arithmetic identical to `:2114-2116`), gives each core its own
accumulator region (`rebase_accums`, `:971`), and merges at the frame's existing publish boundary
rather than at a mid-fiber meeting race.

The successor is **order-preserving, not merely associative**, and that is the property canon
actually needs. `merge_accums` forward-compacts in ascending core order, and since cores own
ascending record slices the merged sequence is the record order
(`resource/bindings.rs:660-664` states the invariant). Canon's one explicit carve-out is
"non-commutative resource accumulation gives skip convergence for that fiber" (spec `:1843`), and
that carve-out exists because a 2-way opposite-ends converge cannot preserve record order: the tail
walker accumulates backwards. The N-way forward partition has no such problem. So the successor does
not merely generalise canon's mechanism, it removes the constraint that motivated canon's
carve-out. Non-commutative accumulation is safe on it, where canon had to skip.

That is worth stating precisely because an earlier draft of this section argued the successor was an
"N-way associative fold", which would have been the weaker claim and would have inherited canon's
commutativity requirement. It also criticised the `tphase == 1` branch for lacking a commutativity
gate without checking whether the successor has one. It does not: `unit_meta.commutative` is written
at `plan/mod.rs:422` and read nowhere in dispatch or the scheduler. That is a second dead plan-side
field alongside `Fiber.head_tail`, and it should be recorded as such. The absence is correct given
order preservation, but it is correct by accident rather than by design, and the reason should be
written down where the next reader will find it.

What actually reaches the `tphase == 1` branch is therefore a carrier with no accumulator anywhere,
where every fiber is `morsel_local` and every record is independent by construction. There is
nothing to converge and nothing to merge. Applying canon's 2-way shape there would cap parallelism
at 2x to protect an invariant that case does not have.

So G2C-0 is a bookkeeping and hygiene slice, not a build. Record the supersession in the canonical
deviation ledger (`202606072100_gate2-canonical-deviation-ledger.md` is the established place),
stating that the N-way order-preserving partition subsumes the 2-way converge and why. That ledger
also needs its own correction: its deviation-9 entry is marked not-built and residual, while
deviation 9 is in fact the path this whole argument rests on.

Fix the design doc, which currently contradicts itself:
`mock/crates/hilavitkutin/DESIGN.md.tmpl:291` describes the shipped N-way form while `:616` still
states "two threads... from opposite ends". Note the path: this is the per-crate design doc, not the
159-line `mock/DESIGN.md.tmpl`.

Audit and delete the dead 2-way surface per `no-legacy-shims-pre-1.0`: `HeadTailConvergence` and its
`AccumSlot` and `MergeOp` companions (`plan/fiber.rs:96-182`), `thread::Convergence`
(`thread/convergence.rs`), and the now-known-dead `unit_meta.commutative` field. Audit rather than
delete blind: `thread::Convergence` is constructed in `tests/core_assignment.rs:58`, so the deletion
touches a test, and `ConvergenceBuffer` is a different type that the accumulator path and
`tests/resource_accumulator.rs` genuinely use. Confirm each symbol's reference set before removing
it.

And correct the name in the places it survives on the N-way mechanism (`scheduler/mod.rs:2107` and
`:959`), because keeping canon's term on a different mechanism is what made this look like a missing
feature for a month.

**G2C-0 covers three call sites, and getting that count wrong is a correctness hazard.** The
ceil-slice `per = (total + ncores - 1) / ncores` is computed independently at three places:
`scheduler/mod.rs:2114-2116` in the phase walk, `:960-962` inside `worker_accum_unit_outer` (where
the source comment admits it "mirrors `run_core_phase`'s split"), and `:1923` inside `run_parallel`,
which recomputes it to locate each core's region for `merge_accums`.

The third one is why this matters beyond tidiness. The merge uses `per` to find where each core's
accumulator region starts (`resource/bindings.rs:653-656`). Applying G2C-1a's morsel alignment to
the two partition sites but not to the merge would desynchronise the merge from the partition it is
merging, and silently corrupt accumulator output. Any change to the partition shape, the alignment,
or the record-count floor lands at all three or at none.

This roadmap got the count wrong twice before landing: an earlier draft named only the phase-walk
site, a revision named two, and the review found the third. Three is the verified count as of
2026-07-18; anything touching the partition should re-grep rather than trust it.

This supersession is the agent's call rather than op's: it is already shipped, the evidence is
mechanism-level rather than aesthetic, and the deviation ledger plus this roadmap are the audit
trail. Flag it for op's asynchronous review rather than blocking on it.

**G2C-1, N-way record ownership on the const path (reframed, and now the phase's real work).** Both
the `tphase == 1` branch and the accumulator path are stuck on the runtime-masked `run_gated` walk,
losing the const-monomorphised devirtualisation the rest of dispatch has, because `dispatch_core`'s
trunk-ownership rank logic assumes a disjoint trunk-to-core assignment. It already threads a
`MorselRange` at its other call sites (`scheduler/mod.rs:2136`, `:2157-2168`), so the missing piece
is letting N cores share ownership of one const trunk partitioned by record range instead of by
trunk identity. That recovers full compile-time devirtualisation for the single-trunk case while
keeping N-way scaling, which neither the shipped shape nor canon's 2-way achieves. This is what
G2C-1 was originally chartered to find, stated correctly now that the mechanism it applies to is
known. Sketch first, at some-shape leeway, with an asm-gate fixture proving zero indirect calls.

**G2C-1a, morsel-align the slice boundary (new).** `per = ceil(total / ncores)`
(`scheduler/mod.rs:2114`) is not rounded to the morsel size, so cross-core slice boundaries land
mid-morsel and produce false sharing on up to `ncores - 1` cache lines for narrow write columns.
Round `per` up to a multiple of the window. Small and independent of G2C-1.

**G2C-1b, minimum records per core (new).** The branch fires on `total > 0 && ncores > 1` alone.
Canon's own eligibility list includes a record-count threshold (`plan/fiber.rs:150`), and the
existing parity bench (`202606081100`) shows the wide-parallel arm flat to negative in exactly the
band where per-core work thins against fixed wake and park overhead. Add the floor, and let the
measurement below set it.

**G2C-2, wire the result into `worker_main`.** Unchanged in intent, now applying to whatever G2C-1
proves.

**G2C-M, the scaling measurement (new, gates G2C-1's cost justification).** Measure the core-scaling
curve of both `worker_accum_unit_outer` and the `tphase == 1` branch at 2, 4, and 8 cores across
1K, 64K, and 1M records, on a bandwidth-heavy fiber (large write-column footprint) and a
compute-heavy one. If bandwidth saturates by 4 cores on the heavy fiber, that is the ceiling
regardless of dispatch shape and G2C-1's const-path migration is not worth its engineering cost; if
it scales near-linearly to 8, it is. This is the bench-decides discipline: the fork is a
performance question, so it gets measured rather than argued.

**G2C-3 and G2C-4**, the phase-overlap progress-counter mechanism, are unchanged and confirmed
unbuilt. The sketch scope narrowed by the 2026-07-02 pass stands: prove that a downstream Acquire on
an upstream-published counter composes with the `waist_barrier` Release fence for happens-before
without a per-morsel full barrier, at exact leeway, then wire it into `run_core_phase`.

**G2C-5, parallel incremental skip (new).** `run_core_phase` dispatches all-ones dirty
(`scheduler/mod.rs:2102`), so the parallel path has no incremental skip while the single-core path
does. Canon's dirty-propagation step (`:1418-1428`) places the skip before each pass with no
parallel exemption. Bring the parallel path to parity. This belongs in G2C rather than Phase A
because the hazard is the parallel one: the dirty mask has to be stable across the publish and await
window. That is the same happens-before argument G2C-3 is making, so G2C-5 **depends on G2C-3's
proof landing**, not merely on sharing a phase with it. Sequence it after G2C-4.

**G2C-6, the plan-recompute trigger (new).** Domain 16 (`:1476`) makes the engine responsible for
recomputing morsel splits at plan stage on any significant pipeline change. The `plan_dirty` seed
and `plan_cache` exist and `replace_resource` sets them, but no phase in any roadmap revision owns
the structural-DAG-change trigger that consumes them. It splits in two, because the second half
carries a hazard the one-line framing hid.

**G2C-6a, the trigger conditions.** Confirm and complete the `plan_dirty` seed: what counts as a
significant pipeline change, and which of those conditions already set the bit. Small, and provable
by inspection.

**G2C-6b, the recompute call.** Wire the actual re-run of the plan chain. This needs its own sketch,
because a re-plan mutates the descriptors and the grouping that live per-core dispatch state is
built on, and nothing currently establishes whether a recompute can happen while that state is live
or whether it needs a quiesce point at a frame boundary. Prove the safe point before wiring the
call.

## Phase B: plan-analysis chain

Unchanged from r5 and confirmed absent in source. B1a is resolved (the DP cost is spec Step 8's
`record_count x sum of size_of::<T_k>()` over the candidate fiber's union columns, with feasibility
being the full domain-14 holistic check, greedy at or below 10 ops and DP above). B1b's substrate is
confirmed (`arvo-comb::matrix_chain_dp` and `greedy_group`). B2 still needs its sketch, proving the
spectral label to contiguous `Trunk` renumber keeps `FiberGrouping.assignment` consistent. B3a is
resolved (arvo-sparse ships the Dulmage-Mendelsohn surface), leaving B3b as integration plus
dead-column elimination.

Two deferred benches become runnable inside this phase and are named here so they are not
rediscovered as surprises. **#644, the definitive spectral-versus-real-greedy fiber bench**, was
parked gated on a real greedy implementation plus a runtime to measure against; B1b supplies the
real greedy, so #644 runs at
the end of B1b and its result informs whether B2's spectral consumption is worth its complexity.
**#635, arvo's deferred cross-variant decisions** (RCM bitmask versus CSR, spectral dense versus
`SparseLaplacian`), sit upstream in arvo and bear on B2 and C1. Check #635's state before starting
B2; if it is still open, it is a cross-repo prerequisite rather than a hilavitkutin leaf, and the
fix-the-stack-upstream rule applies.

## Phase C: RCM-row dispatch order

Unchanged. C1 sketch at exact leeway, then C2 relaxes `NonTopologicalRegistration`, then C3 retires
the arena-only framing.

## Phase D: adapt completion

Every prior revision carried Phase D's tail as a single sentence naming six axes. That is under
decomposed to the point of being unschedulable, and three of the six hide an open question with no
assigned answer. Decomposed here.

**D-act-1, the descriptor refresh path.** Confirmed real by inspection: `morsel_windows` is read
only at build, so the actuation refreshes the `FiberDispatch` descriptors rather than writing the
plan field. Mechanical, no open question.

**D-act-2, the re-chunk rule.** This is where the flat listing hid a fork. "Wire the re-chunk on
`adapt_reconfigure`" says when to re-chunk and says nothing about *by what rule*, and that rule has
many defensible answers: halve the window of the phase whose EMA is the bottleneck, rebalance all
windows in proportion to their phase EMAs, step toward the L1 formula's value from the measured
side, or re-derive the formula against a corrected effective-bandwidth estimate. Picking one at
implementation time is exactly the on-the-fly course change this roadmap exists to prevent.

Resolution: bench, not argument. The catalogued contract `morsel_rechunk_reduces_idle_ns` is the accept
gate (it asserts idle time falls without total time rising), and `ema_adaptation_improves_imbalanced_workload`
is the corroborating arm. Implement two or three candidate rules behind the same actuation seam,
measure them on a deliberately imbalanced fixture, and keep the one that turns the contracts green
with the best margin. Record the losing rules as the audit trail. This is the same bench-decides
discipline as G2C-M; the rules are cheap to write because they share the descriptor-refresh path
D-act-1 provides.

**D-ema, the remaining EMA taps.** `fiber_ema`, `active_units`, and the parallel-path `phase_ema`
are mechanical: each mirrors a shipped single-core EMA pattern into a new tap point. No open
question, no sketch needed. Three small slices.

**D-arena, AdaptArena.** Recorded since the original audit as "option-B perf storage, bench-gated",
with the bench never named. Name it: the question is whether moving the adapt metrics off the
engine-internal fixed-cap fields into a dedicated arena costs or saves anything at the frame
boundary where they are folded. Measure the fold cost both ways across the record-count range the
other benches use. If the difference is inside noise, keep the shipped fixed-cap fields and close
the item; option B is only worth its complexity if it measurably wins.

**D-gen, per-morsel generation counters (S-6).** Domain 12 (`:861`) specifies a per-morsel
generation counter bumped on write, propagating through the DAG so an unchanged root skips its
transitive dependents. The engine currently has coarse per-store dirty only. This needs a **design
sketch before implementation**, at some-shape leeway: the open question is where the counters live
given no-alloc and a per-fiber morsel count that varies with the window, and how they compose with
the existing `store_dirty` mask rather than duplicating it. Do not start this one by writing code.

**D-str, strategy reselect, is blocked on work that is in no phase.** The audit recorded it as
"domain 14, after strategy plan-shaping is wired", and strategy plan-shaping appears in no phase of
any roadmap revision. So the dependency is real and dangling. Two sub-leaves:

**D-str-0, strategy plan-shaping**, the missing prerequisite. Domain 14's strategy axis must actually
shape the plan before anything can reselect it. Scope it and place it here rather than leaving it
implied by a subordinate clause. Needs its own comprehension pass against domain 14 before it can be
decomposed; it is the least-charted work remaining in the internals.

**D-str-1, reselect**, which only becomes meaningful once D-str-0 exists.

One housekeeping item: `a1_fiber_morsel_size.rs:121` is catalogued red with the reason "lands with
A2", and A2 has landed. Re-check whether the case now passes and either un-ignore it or restate the
real blocker.

## Phase E: consumer surfaces, plus a standing lane

E1 through E5 are unchanged and confirmed absent (`PipelineResult`, `read_slice`, `write_slice`, and
`morsel_range` have no definitions; `hilavitkutin-persistence/src/cold_store.rs` is a skeleton).

**E0, the consumer-pull lane (new).** This is a standing policy, not a step, and it is listed apart
from the lettered leaves deliberately: it has no fixed shape, so no sketch can prove it and no gate
can hold it. `Plannable` and `HintExt` landed out of sequence because a downstream consumer needed a
contract split in the api crate, which is not the sugar the internals-first rationale assumed
consumer work would be. That was the right call under `use-the-stack-not-reinvent`. The lane exists
so the next one is planned rather than inserted: a consumer hitting a substrate limitation gets the
substrate fixed upstream, additively, whenever it surfaces, without waiting for Phase E and without
disturbing the internals sequence.

E1 through E5 are stated at phase granularity here and are not yet decomposed to leaves. They cover
four undefined symbols plus a skeleton crate, and each will need the per-symbol splitting the other
phases have already had before the phase is actionable. That decomposition is due when Phase D
closes, not now; recording it so the phase is not mistaken for ready.

## Phases F, G, H

Unchanged. Heterogeneous P/E core awareness and version stamps; the full bench pass and microkernels
once internals are real; ecosystem integration last.

## Red arms and the gates that own them

Ten tests carry a `#[ignore = "catalogue:` marker. They map as follows, so a future reader does not
mistake a later-gate arm for an early-gate gap.

Seven adapt performance contracts (`adapt_perf_contracts.rs`) are owned by Phase D, several of them
specifically by the tier-1 re-chunk actuation. The fiber-plan-index case
(`a1_fiber_morsel_size.rs:121`) is owned by Phase A and is now unblocked, since its stated blocker
was A2 and A2 has landed.

Two had no phase home before this revision, and both now have one as a named leaf. The morsel window
floor case (`r6_morsel_window_formula.rs:178`) becomes **A7**, and it is **not an open question**:
its own catalogue entry states the intended resolution, which is to align the floor before clamping
so the post-clamp `& !3` can never land the window below `MIN_MORSEL`. A mechanical fix to the
domain-12 formula, homed in Phase A next to A3's other formula work, where the implementer follows
the catalogue entry rather than deciding anything. The collection-member case
(`resource_snapshot.rs:130`) becomes **E6**: it needs Seq and Map members wired into resource
values, which is domain-19 resource-collection work, so it sits in Phase E alongside the other
consumer-facing data-plane surfaces. Nothing earlier depends on it.

The perf gate's branching and accumulator arms stay red by design until the phases that own them
land. They are not gaps to fill with a single-stage special case.

## Housekeeping leaf: the step numbering is off by one from canon

`plan/steps.rs` numbers its own chain 1 through 9 and that numbering does not match the spec's.
Source has RCM at Step 4 (`:331`), block detection at 5 (`:362`), spectral at 6 (`:438`), fiber
grouping at 7 (`:477`), upward rank and dirty at 8 (`:827`), and morsel windows at 9 (`:941`). Canon
has RCM at Step 5, block and Dulmage-Mendelsohn at 6, spectral at 7, fiber grouping at 8, and dirty
propagation at 9. One source comment already half-notices, writing "canonical Step 9" at `:901` to
disambiguate.

This is doc-comment-only and changes no behaviour, but it is a live cross-reference hazard on
exactly the steps this arc is about to touch. Canon Step 8 is fiber grouping, which is where the
matrix-chain DP cost function lives and which Phase B cites by number; source Step 8 is upward rank
and dirty. Canon Step 9 is dirty propagation, which G2C-5 cites; source Step 9 is morsel windows.
Given that a prior round already went wrong by misreading which RCM ordering the spec's Step 5 and
Step 8 referred to, an off-by-one between the two numbering schemes is worth closing before Phase B
and C reason by step number. Renumber the source doc comments to canonical numbering. A mapping table was the
alternative and is rejected: it is a second artefact to keep in sync, where renumbering removes the
discrepancy at its source.

## Evidence ledger: every leaf has an assigned resolution

The point of this roadmap is that implementation never stops to discover that a step has several
possible answers and no evidence for choosing between them. That guarantee is only real if it is
checkable, so every leaf is classified below by how its open questions get answered. **No leaf may
be started while it sits in an unassigned state.** If a future reader finds one, that is a defect in
this document, not a decision for them to make at the keyboard.

| Leaf | Resolution | State |
|---|---|---|
| A1, A2a, A2b, A3, A3b | shipped | done |
| A4 parallel per-fiber sizing | mechanical; all three forks settled in-line (A4 keeps its place ahead of G2C-0, the head+tail branch stays on the scalar, `gate2_fiber` adds a third pre-lift array) | ready |
| A5 fused path window | mechanical | ready |
| A6 accumulator-walk windowing | bench, folded into G2C-M as an extra arm; resolves in G2C, not in Phase A | assigned, cross-phase |
| A7 morsel window floor | mechanical; the catalogue entry states the fix (align the floor before clamping) | ready |
| G2C-0 supersession bookkeeping | mechanical, plus a named reference audit before each deletion | ready |
| G2C-1 const-path record ownership | sketch, some-shape leeway, plus asm-gate fixture | specced |
| G2C-1a morsel-align the slice | mechanical, at all three call sites | ready |
| G2C-1b minimum records per core | the floor's value comes from G2C-M | assigned |
| G2C-2 wire the result | mechanical after G2C-1 | ready |
| G2C-3 phase-overlap ordering | sketch, exact leeway | specced |
| G2C-4 wire phase-overlap | mechanical after G2C-3 | ready |
| G2C-5 parallel incremental skip | mechanical once G2C-3's happens-before proof exists | assigned |
| G2C-6a re-plan trigger conditions | inspection | ready |
| G2C-6b re-plan against live state | sketch, exact leeway | specced |
| G2C-M scaling measurement | bench, with the decision rule stated in advance | specced |
| B1a DP cost function | resolved from canon (spec Step 8) | done |
| B1b DP fiber grouping | mechanical on confirmed substrate; #644 runs at its end | ready |
| B2 spectral into fiber grouping | sketch, some-shape leeway; check #635 upstream first | specced |
| B3a Dulmage-Mendelsohn substrate | resolved (arvo-sparse ships it) | done |
| B3b DM integration + dead columns | mechanical on confirmed substrate | ready |
| C1 RCM-ordered dispatch | sketch, exact leeway | specced |
| C2, C3 | mechanical after C1 | ready |
| D-act-1 descriptor refresh | mechanical, path confirmed by inspection | ready |
| D-act-2 re-chunk rule | bench between candidate rules, catalogued contracts as the accept gate | assigned |
| D-ema remaining taps | mechanical, three slices mirroring shipped patterns | ready |
| D-arena AdaptArena | bench, now named: fold cost both ways, keep shipped shape if inside noise | assigned |
| D-gen per-morsel generation counters | design sketch, some-shape leeway, before any code | specced |
| D-str-0 strategy plan-shaping | needs its own comprehension pass against domain 14 | not decomposed |
| D-str-1 strategy reselect | blocked on D-str-0 | blocked |
| E0 consumer-pull lane | standing policy, not a step | n/a |
| E1 to E5 consumer surfaces | per-symbol decomposition due when Phase D closes | not decomposed |
| E6 Seq and Map collection members | domain-19 wiring; nothing earlier depends on it | not decomposed |
| F, G, H | phase summaries, decomposition not yet due | not decomposed |
| step renumbering | mechanical; renumber to canon, mapping-table alternative rejected in-line | ready |

Three entries are honestly incomplete, and naming them is the point. **D-str-0 is the least-charted
work remaining in the internals**: strategy plan-shaping was carried for months inside a subordinate
clause ("after strategy plan-shaping is wired") attached to a different leaf, which is how a
prerequisite goes missing. It needs a comprehension pass against domain 14 before it can be
decomposed, and that pass should happen during Phase C rather than when Phase D reaches it. **E1 to
E5** are deliberately left at phase granularity until Phase D closes, because their shape depends on
what the internals settle into. **F, G, H** are not due.

Everything else is either done, ready to implement with no open question, or specced with a named
sketch or bench that must land before its slice starts.

## Sketch plan: what is unproven, and what each sketch must pin

Most leaves in this revision are mechanical and need no sketch: A4, A5, A7, G2C-0, G2C-1a, G2C-1b,
G2C-5, G2C-6a, the step renumbering, and the B and C leaves the 2026-07-02 pass already resolved
from canon and source (B1a, B1b's substrate, B3a). What follows is the set whose premise is genuinely
unproven, each with the hypothesis it must establish and the leeway accepted in the shape it proves.
Sketches land in `mock/research/sketches/<ts>_<topic>/` per `cl-claim-sketch-discipline`, carrying
the hypothesis, real code against the real crates, and an outcome of WORKS, FAILS WITH the error, or
INCONCLUSIVE.

They are specified here and written per tick, each gating its own slice, which is the convention the
2026-06-19 pass set and this revision keeps. Writing all of them up front would date the later ones
against a codebase the earlier slices will have changed.

**G2C-1, N-way record ownership on the const path.** Prove that `dispatch_core`'s trunk-ownership
rank logic can be extended so several cores share ownership of one const-monomorphised trunk,
partitioned by record range rather than by trunk identity, with no per-record runtime branch and no
loss of dead-code elimination for non-owned trunks. Leeway: some-shape. Either the rank logic takes
a record-range parameter, or a const-selectable third mode wraps it; prove one compiles and stays
devirtualised. Must ship its asm-gate fixture asserting zero indirect calls, since the whole point
is recovering devirtualisation.

**G2C-3, phase-overlap memory ordering.** Prove that a downstream worker's Acquire load on an
upstream-published `ProgressCounter` composes with the `waist_barrier` Release fence to give
happens-before, without a full barrier per morsel. Leeway: exact. Memory ordering is a correctness
contract, and a shape that merely compiles proves nothing here.

**G2C-6b, re-plan against live dispatch state.** Prove there is a point at which the plan chain can
re-run without tearing state that live per-core dispatch is reading, and identify it. Leeway: exact.
This is a safety contract, not a shape preference. The sketch's honest outcome may be that a quiesce
point at a frame boundary is required, which is a finding, not a failure.

**B2, spectral labels into fiber grouping.** Prove the projection from `k_way_partition`'s arbitrary
per-fiber cluster labels to `Trunk`'s contiguous `fiber_offset` and `fiber_count` range keeps
`FiberGrouping.assignment` consistent under the same permutation. Leeway: some-shape. The failure
this guards is the silent one: renumbering the trunks without applying the permutation to the
per-unit assignment changes grouping semantics with nothing failing loudly.

**C1, RCM-ordered dispatch.** Prove that an RCM-ordered `topo_order` drives carrier-position
const-dispatch correctly, since the carrier walks by carrier position rather than by `topo_order`
index. Leeway: exact. The ordering contract is precise and a near-miss is a wrong-answer bug.

**D-gen, per-morsel generation counters.** Prove where a per-morsel generation counter can live
under no-alloc when the morsel count per fiber varies with the window, and how it composes with the
existing `store_dirty` mask instead of duplicating it. Leeway: some-shape. This is the one Phase D
leaf that must not be started by writing code; the storage question decides the design.

**Benches, not sketches.** G2C-M measures rather than proves a shape: its result gates whether
G2C-1's engineering cost is justified, not whether G2C-1 is possible, and a bandwidth ceiling at
four cores is an acceptable answer that closes the item. It also carries A6's windowed-versus-
unwindowed arm and sets G2C-1b's floor. D-act-2 benches candidate re-chunk rules against the
catalogued adapt contracts. D-arena benches the metrics fold both ways. Each states its decision
rule before it runs, so the measurement settles the question rather than starting an argument.

## How the head+tail question was settled, and a recorded disagreement

The question this roadmap opened, whether canon's 2-way head+tail or the shipped N-way slice is
correct, went to two independent passes with neutral briefs. They disagreed, which is worth
recording, because the disagreement is instructive about how the drift went unnoticed.

The canonical-mirror pass leaned toward canon and declined to settle it. Its reasoning was that the
2-way supporting types compile and sit unused, which it read as evidence that the canonical
mechanism is expressible on this toolchain today and was pre-provisioned rather than considered and
rejected. It concluded the N-way throughput edge was unmeasured and routed the decision to op.

The execution-shape pass settled it against canon, on a mechanism-level argument the mirror had not
found: the diversion at `carrier_unit_outer` means the canonical case never reaches the branch under
review, so the two shapes are not competing for the same workload at all. Canon's case is already
served, in generalised form, by the accumulator path.

The second argument wins because it is specific and checkable, and it checks out: both of its
load-bearing claims were verified against source before adoption
(`scheduler/mod.rs:2056-2068` for the diversion condition, `:960-971` for the accumulator path's
identical partition plus per-core regions plus merge). The first argument was an inference from the
existence of unused types, and unused types are equally consistent with a mechanism that was
superseded before it was wired. That inference is exactly the trap that let the drift persist: the
dead surface looked like an unfinished feature rather than a retired one.

The lesson generalises past this instance. When a canonical mechanism appears missing, check whether
a different shipped path already serves its case before scheduling the build. Here, a month of
roadmap carried "head+tail convergence: unbuilt" while its actual workload was being handled
correctly a few hundred lines away under a different name.
