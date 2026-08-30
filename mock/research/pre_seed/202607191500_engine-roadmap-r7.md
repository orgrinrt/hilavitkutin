# Engine roadmap r7: ordered from the three-way ledger

**Date:** 2026-07-19
**Status:** chart-the-path take 2, phase 5 draft. Pending the canonical-mirror and granularity
passes.
**Verification note.** Three independent passes (fact-check, adversarial, second canonical mirror) ran
over this document on 2026-07-19. Eleven fact-check findings and two mirror findings were applied. One
adversarial finding was **rejected on evidence**: it reported the `202607191200_a4-fibercons-nest-wireability`
sketch as nonexistent and possibly fabricated, having globbed `mock/research/*.md`. The sketch is
directory-shaped, present at `mock/research/sketches/202607191200_a4-fibercons-nest-wireability/`, and
committed as `860c5cac`. A reviewer's search method can be as wrong as an author's, so their findings were
verified against source too, not adopted on authority.

**On the "grounded exclusively on the audit" claim.** That names the source, not a warranty. The audit
carries at least one error of its own (the `strategy/` three-way return, corrected in W6 and now also in
the audit). Verify against source, not against either document.

**Note on "canon N" references:** the numbering is the audit's own index over the consolidation
spec and its amendments, not a numbered artefact in the repository. Each reference is only as good as
the spec text it points at, so verify the text, not the number.
**Grounded on:** `202607191400_engine-three-way-audit.md` exclusively. No status in this document
is inherited from a prior roadmap; every one traces to canon plus a call-site trace.
**Supersedes:** `202607181300_engine-roadmap-r6.md` entirely, including its phase order.

## Why the order changed

r6 and its predecessors ordered by feature phase: per-fiber morsels, then parallel completion, then
plan analysis, then dispatch order, then adapt, then consumer surfaces. That order assumed the
remaining work was construction. The ledger says otherwise. Twelve subsystems are finished and
unreached, so a large fraction of the remaining canon coverage is **wiring**, which is far cheaper
than the phase order implied and lands in a different sequence.

The ordering principle here is different: correctness first, then remove what actively contradicts
canon, then wire what already exists and matches canon, then build what is genuinely absent, then
surfaces, then measurement. Within each band, dependency order.

Two things changed position sharply. Phase-overlap moved from "build it" to "wire it", because
`ProgressCounter`, the arena accessors and the release fence are all complete and correct. And a
defect band appeared ahead of everything, because the roadmaps were citing broken code as evidence
that mechanisms worked.

Head+tail was placed the same way in the first draft and that was **wrong**; the granularity pass
caught it. Its thread-side and shape substrate is real, but its eligibility predicate does not
exist, so it is part wire and part build. The corrected W1 below says so, and the closing note
records how the error happened, because it is the same one that poisoned every prior roadmap.

## Band 0: defects

Not roadmap features. Bugs in reachable code whose bodies contradict their own contracts. They come
first because other work cites them as evidence, and because two of them make the engine
unusable on a supported configuration.

**D1. `replace_resource` and `replace_value` install nothing.** `scheduler/mod.rs:1187`, `:1207`.
Both take `_new: T`, call `mark_dirty`, and drop the value. This is canon 54's plan-recompute
trigger and domain 22's resource swap. `202606111700` cited the shipping `plan_dirty` array as
evidence the mechanism works. Either install the value or change the signature so it cannot lie;
the choice is a contract decision, so it needs a topic, not a patch.

Sequencing, per the mirror: sketch the install arm **before** the topic, so the contract decision is
taken knowing whether installing is cheap. And note what depends on it, which the draft left
dangling at the bottom of the document: A1-3 (`202606111400:83-85`) ties `replace_resource` writing
the real value to the adapt completion arc, so W5's `compute_execution_plan` work and band 5's adapt
benches both assume it. D1 is not free-standing; it gates measurement.

**D2. `StdThreadPool::spawn` is a no-op, so `platform-std` deadlocks.** `platform/std_tier.rs:119`.
`run_parallel` publishes a frame and waits for workers that were never spawned. The working
implementation is `spawn_fn` at `:104`, unreached. Fix is to route `spawn` through it or port the
`transmute_copy` marshalling the os tier uses.

**D3. `StdMemoryProvider::deallocate` rebuilds the layout with word alignment.**
`std_tier.rs:68`. UB for any allocation with alignment above `align_of::<usize>()`, which canon 2's
64-byte column alignment produces. Carry the original alignment.

**D4. Doc comments that describe behaviour their bodies do not implement.** Twelve found, listed in
the audit. Each is a trap for the next reader, and this roadmap's first draft fell into one of them
(see the closing note). Split, because several are disguised design decisions rather than doc fixes:

**D4a, mechanical corrections** for items with no downstream roadmap item. Say what the body does.

**D4b, deferred into their owning item.** `classify_cores` goes with W4, `AdaptArena` with the
adapt build, `dispatch::order` and the phase-barrier docs with R2. Correcting these now would
pre-judge the outcome of the item that owns them, and would be written twice.

## Band 0 outcome (2026-07-19, round `202607192000`)

**D2, D3 and D4a are done.** D1 is not started, per op's sketch-first direction.

Three things the band surfaced that the audit had not:

**`platform-std` had never been compiled.** Adding the spawn observation test revealed that
`tests/platform_std.rs` did not build at all: `now_ns()` gained `to_raw()` at some point, the
os-tier copy was updated and the std-tier copy was not, and nothing noticed because nothing
built it. So the configuration described as supported, with its own smoke tests and its own
validation gate, was not being exercised by anything. This strengthens D2 rather than changing
it: the pool that spawns nothing sat behind a gate that never ran.

**D3 is reachable, through the providers crate rather than the engine.** An engine-scoped grep
for `.deallocate(` returns nothing, which is why the audit could not name a caller.
`ArenaColumnStorage` (`hilavitkutin-providers/src/storage.rs`) reserves at `CACHE_LINE_ALIGN`
and frees at `:78` and `:159`, the latter from `Drop`. The audit's "live defect" classification
was right; its reachability trace was incomplete because it stopped at the crate boundary. Worth
carrying forward: a per-crate call-site trace misses cross-crate reachability, and the engine is
not the only consumer of its own api crate.

**The suite was avoiding the failing path deliberately.**
`hilavitkutin-providers/tests/column_storage.rs` said in its own module doc that
`StdMemoryProvider` was not used there "because its `deallocate` reconstructs the `Layout` with
word alignment, which mismatches a 64-byte-aligned block". The defect was known, documented, and
routed around. That is a sharper version of the all-green problem than the audit anticipated:
not a weak test, but an accurate comment explaining why the strong test was not written.

**One near-miss worth recording.** A first pass through D4a read only the struct doc on
`AdaptArena`, found it honest about its unbuilt state, and began writing a correction into both
this roadmap and the audit saying the HOLLOW entry was an overcall. The module doc above it, which
is what the audit was describing, does state the layout in present tense. The audit was right and
the correction would have been wrong, produced exactly the way the two errors the audit documents
were produced. It was caught before it landed. The lesson is not new, which is the point: a
partial read produces a confident wrong answer at the same rate no matter how many times the
failure has already been named.

## Band 1: remove drift

**R1. Delete the N-way ceil-slice from `run_core_phase`. OP-GATED WHEN DRAFTED, NOW RESOLVED.**
`scheduler/mod.rs:2107-2132`. Canon forbids it in terms that name the shape: "never as an N-way
record or morsel partition" (`202606111800:447-450`). It also applies with no commutativity gate,
while canon requires one. Removing it leaves single-trunk phases running serially until W1 lands,
which is the correct intermediate state: canon's parallelism comes from trunks, and a single-trunk
phase legitimately has none until head+tail is wired.

**Resolved by op, 2026-07-19, and the resolution removes the gate.** The draft flagged this as
op-gated because the forbidding language lives in `202606111800`, whose supremacy was contested
against `202606061000:3`. Op's ruling on that ambiguity: **neither document blanket-wins; treat both
as amendments to the consolidation spec and resolve each conflict on its own merits, with the
consolidation spec as the tiebreak.**

Applied here, that settles R1 rather than blocking it, because the tiebreak document states the
requirement independently. Consolidation spec `:770-771`: "Single-trunk phases: head+tail convergence
(2 threads, opposite ends, ~2x parallelism). Skip for non-commutative resource accumulation."
Spec `:1840-1841`: "Within any commutative fiber, records are independent. Two threads process from
opposite ends, converging in the middle."

**Correction, and it matters more than the wording.** An earlier revision of this paragraph rendered
that second quotation as "Two threads process same commutative fiber from opposite ends" and
attributed it to the spec. That string is not in the spec. It is
`hilavitkutin/src/thread/convergence.rs:3`, a **source doc comment**. The substance survives intact,
since the spec independently says two threads from opposite ends and gates on commutativity, so R1's
basis holds. But a source comment was quoted as canon inside the argument authorising a deletion.
That is the third occurrence of this exact error in this arc, after the 2026-06-08 document and this
roadmap's own W1 draft. See the closing note. So the 2-way
requirement never depended on the contested document; `202606111800`'s "never as an N-way record or
morsel partition" is a clarifying amendment consistent with the spec, not a new rule imposed by a
document of disputed authority.

R1 therefore proceeds on the consolidation spec's own text. The commutativity gate is required
under every reading and lands with it.

Note the interaction: this is the branch A4 deliberately left on the scalar window. Once it is
gone, that carve-out goes with it.

### R1, 2026-07-19: it is a deletion after all, and the detour is worth reading

**Final position: R1 stands as originally written.** Delete the N-way ceil-slice from
`run_core_phase`. It strands nothing, and single-trunk phases run serially until W1 builds head+tail.
R1a does not exist as a task; the R1a/R1b split introduced here earlier is withdrawn.

Getting there took four passes, three of which were wrong, and the shape of the error is more useful
than the conclusion.

**The claim that derailed it.** `ceil(total/ncores)` appears at three sites. `:2119` is the dispatch
split R1 targets. `:1923` is the accumulator region layout, canonical per `202606111800:452-456`.
`:960` computes the same slice and feeds `run_this = lo < hi`, a guard that returns before
dispatching. From that, three successive conclusions: participation is decided by record slice while
dispatch ownership is by trunk rank, so they disagree; deleting the branch strands trunks owned by
cores that opted out; and the accumulator region sizing has to change with it.

**Why all three were wrong.** `worker_accum_unit_outer` begins at `:933`. Every line the argument
rested on, `per`, `lo`, `hi`, `run_this`, `rebase_accums`, is inside that function, which handles
accumulator-bearing carriers only. At `:873` such a carrier runs unit-outer and `continue`s, never
reaching `run_core_phase`. The non-accumulator path calls `run_core_phase` unconditionally for every
core on every phase and states the invariant at `:913`: "every worker participates in each waist,
even one that owned no trunk this phase".

So the two ownership models are mutually exclusive paths, not competing gates on one path. They
cannot disagree, there is no hazard for the whole-phase mask to be covering, and accumulator sizing
is unaffected by anything R1 does.

**What was actually established, and stands.** Two tests, both green, both worth keeping:
`gate2_run_parallel.rs::every_producer_runs_when_cores_outnumber_records` pins that every producer
runs when cores outnumber records, and
`gate2_accumulator.rs::accumulator_in_a_two_trunk_phase_matches_single_core` pins that an
accumulator in a multi-trunk phase matches the single-core reference. Neither found a defect. Both
pin properties that were being asserted from argument rather than evidence, which is why they exist.

The doc corrections from round `202607192100` also stand independently: the design prose called the
N-way split "head+tail convergence" and described the accumulator offset as "the same head+tail
record-slice start", welding two mechanisms that share only a formula.

**The failure mode, stated plainly, because it recurred five times today.** Each wrong pass read code
near the target and never asked which function contained it. The `run_this` guard was read four
separate times across a sketch, a topic, two changelists and a roadmap restructure before anyone ran
`awk` to find its enclosing `fn`. Every one of those reads produced a confident, internally
consistent, wrong conclusion that the next artefact then built on.

The check that would have caught it costs one command. Before believing any claim about what gates
what: find the enclosing function, and confirm the caller reaches it.

**R2. Reconcile the nine duplicate mechanisms.** The draft said "keep the canonical one, delete the
other". **That instruction is unsafe as written, and it is the same shape as the r6 mistake this
roadmap exists to correct**: several dead halves are a *later item's* substrate.
`RecordRange::{Head,Tail}` is W1's; the `dispatch::order` const fold is the codegen keystone. A
blanket delete would destroy them. Three sub-items:

**R2a, collapse the barrier protocols.** Memory-ordering work, not cleanup. **Two corrections to
this entry, from tracing it on 2026-07-19 before starting.**

*There are three implementations, not two.*

1. `thread::barrier::waist_barrier` (`barrier.rs:104`) is **live**, called from
   `scheduler/mod.rs:917`. Sense-reversing: reads `barrier_sense`, `fetch_add(AcqRel)` on
   `phase_arrived`, and the last arriver stores the count back to zero, flips the sense, and wakes
   all. Carries futex parking and the per-core idle accounting.
2. `thread::barrier::phase_barrier_arrive` (`:45`) is **unreached**. `fetch_add(Release)` on the
   same `phase_arrived` word, and it does not self-reset: it depends on a separate
   `phase_barrier_reset` doing an `Acquire` load. A different protocol on shared state.
3. `dispatch::phase_run::waist_barrier` (`phase_run.rs:105`) is **unreached** and is a third
   implementation: spin-only, no parking, over a plain `AtomicUsize` passed in as an argument. It
   shares no state with the other two, so it is benign duplication rather than a hazard.

*The hazard is latent, not live.* The draft said "the hazard is live while both exist". It is not:
`phase_barrier_arrive` has zero call sites (`thread/mod.rs:29` re-exports it, which is what keeps
the dead-code lint quiet, and `phase_run.rs` mentions it only in comments). Nothing today executes
the second protocol, so nothing races. The hazard is that the two protocols are incompatible on
shared state and a future caller wiring the second one would corrupt the first, silently, because
the reset assumptions differ rather than the atomics being obviously wrong.

*The third implementation is not unreached either, and that correction is itself a correction.* An
earlier revision of this entry, written the same day, called `dispatch::phase_run::waist_barrier`
unreached on the strength of an engine-source grep. It is reached: `RunPipeline` drives it, and
`tests/phase_pipeline_dispatch.rs::runpipeline_two_phase_matches_flat_walk` passes today, asserting
output-equivalence between the pipeline walk and the flat walk. `RunPhase` and `RunPipeline` are also
named in `hilavitkutin/DESIGN.md.tmpl:222-232`. So it is test-reachable and design-documented: R2c
reserved, not R2b deletable.

*And `phase_barrier_arrive` is design-documented too.* `DESIGN.md.tmpl:231` names it explicitly and
says "wired in a following round". It is a designed-but-unwired mechanism, not drift.

**So R2a as written does not survive.** The item said "collapse the barrier protocols, keep the
canonical one". There is no canonical one to keep: the consolidation spec's only barrier language is
about fiber formation (natural fan-in barriers as mandatory break points, `:1139-1153`) and says
nothing about the runtime waist-barrier shape. Canon does not choose between a sense-reversing
barrier and an arrive-plus-separate-reset barrier, so this is an implementation decision, not a
drift correction, and there is nothing here to delete on canonical grounds.

What is left of the item is smaller and honest: two barrier designs coexist, one wired and one
documented as pending, on a shared word with incompatible reset assumptions. The useful action is to
say so at both sites, so the next person wiring phase barriers sees the conflict before writing the
call rather than after. That is a doc change, not a collapse.

**Reordered: R2a drops behind R1a.** It is neither urgent nor canon-decided, and R1a is both scoped
and already guarded by a passing regression test.

**R2b, pure deletions** with no downstream consumer: `thread::pool` entire, the duplicated
fiber-member-mask loop, the dead spawn-marshalling half on whichever tier keeps the live one.

*Verified for `thread::pool` on 2026-07-19, and it holds.* `ThreadPool` and `ThreadPoolBuilder` have
exactly one reference in the workspace, the `pub use` at `thread/mod.rs:43`, and neither appears in
any design template. So the deletion is genuinely pure, and it needs no doc CL beyond the round's
own.

*But the absence from the design docs is itself worth naming.* These are `pub` types on the crate's
public API surface that no design document mentions. The `design-doc-source-mismatch` lint checks
one direction only, that every backticked type in a template exists in source. A public type with no
design entry passes silently. That is the inverse gap, and `thread::pool` is an instance of it
rather than an exception: the type was shipped publicly, documented nowhere, and consumed by nothing,
and no gate noticed for as long as it has existed. Band 4 should decide whether the surface audit
runs in both directions.

**R2d, consolidate the accumulator slice arithmetic. NEW, and it carries a correctness stake.**
Traced 2026-07-19 while scoping R2.

`ceil(total/ncores)` is computed twice, and the two results must agree exactly or the merge reads
the wrong memory. `worker_accum_unit_outer:957` derives `per`, then `lo`, `hi` and `region`, and
rebases the accumulator to `[lo, lo+region)`. `run_parallel:1920` recomputes `per` and hands it to
`merge_accums`, which reads core c's region at offset `c * per`. Nothing ties the two computations
together except that they are spelled the same way.

**Correction to a first draft of this entry, checked before it hardened.** The draft claimed the two
sites read different inputs, the worker taking `total` as a parameter while the merge reads
`(*me).record_count`. They share a source: `:864` is `let total = s.record_count`, so the parameter
the worker receives is that same field. The divergence risk is smaller than the draft said.

What remains is real but narrower: the formula itself is written twice. Nothing structural prevents
one from being edited without the other, and if that happens the merge reads misaligned regions
silently, with no assert to catch it. That is worth removing on its own; it did not need the
overstatement, and the overstatement is exactly the unverified-claim habit this roadmap has been
correcting all day.

The fix is one helper returning the slice for a given core, called from both sites, so divergence
becomes impossible rather than merely unlikely. This is not a deletion and not cleanup: it is
removing a duplicated invariant from a path where breaking it is silent.

**Stale comment introduced by R1, fixed here.** `:956` reads "Head+tail record slice for this core
(mirrors run_core_phase's split)". `run_core_phase`'s split no longer exists; R1 deleted it. The
comment now points at nothing, and it also mislabels the slice as head+tail when it is the
accumulator's N-way regioning. That was my miss in the R1 round: I checked the design templates for
references to the deleted branch and did not check source comments elsewhere in the same file.
R2d's round owns those lines and corrects it.

**R2c, deferred pairs.** Anything whose dead half a later item consumes stays until that item
lands, and is recorded here as reserved rather than dead. Deleting on the "unused" signal alone is
what the audit found r6 nearly doing.

### R2 reclassified AGAIN, 2026-07-19: reference-count is the wrong test

**Op's correction: chart-the-path exists to remove redundancy, YAGNI rot and drift, not to catalogue
it.** The reclassification below preserved things on the signal "something still references it". That
signal keeps drift alive. A test exercising superseded machinery is YAGNI rot with a green
checkmark; a design paragraph describing a replaced mechanism is drift that got written down.

The right test is **supersession**, not reference count: has a later design decision replaced this?
If yes it goes, and the test and the doc paragraph go with it, because they are describing the thing
that lost.

Applying it:

**`phase_run` entire is superseded, not reserved.** A1 registry item 3 states it outright: "r4
(`202606070700`) replaces r3's G2-0c **nested-carrier type construction** with the const-eval
grouping + const-gated DCE mechanism". `RunPhase` / `RunPipeline` **are** that nested-carrier
construction. The engine reaches them only via `pub use` at `dispatch/mod.rs:41`; the sole exerciser
is `tests/phase_pipeline_dispatch.rs`, which asserts the superseded path matches the flat walk. That
test is not coverage, it is a tether keeping a replaced mechanism compiling. Delete the module, the
re-export, the test, and the `DESIGN.md.tmpl:222-232` paragraph describing it. `phase_run`'s local
`waist_barrier` goes with it, which removes the third barrier without needing a separate decision.

**The `phase_barrier_arrive` family goes too.** Two protocols on one word, and the live one is the
sense-reversing barrier. R2a's earlier conclusion, "no canonical barrier to keep, so keep both and
add a doc note", was the same preservation reflex. Canon not naming a runtime barrier shape does not
make two competing protocols on shared state acceptable; it makes the shipped one the answer and the
other one drift. Delete `phase_barrier_arrive`, `phase_barrier_reset`, `phase_barrier_observe`, and
the `DESIGN.md.tmpl:231` note promising to wire them.

**`core_phase_mask` is the exception and stays.** Not because a test references it, but because
`core_mask.rs:3-4` names the mask form op's chosen mechanism and `trunk_dispatch.rs:83` cites it as
the rule the live inline code implements. It is the specification of a shipped mechanism, not a
replaced one. Supersession does not apply.

### The earlier reclassification, kept for the record



The split above put three items in R2b as pure deletions. Two of those three are wrong, and the
pattern is consistent enough to state as a finding: **the audit's "nine duplicates" are mostly not
deletable, and the "dead" half is repeatedly the specified one.**

**Genuinely deletable (R2b).** `thread::pool` entire. `ThreadPool` and `ThreadPoolBuilder` have
exactly one reference in the workspace, the `pub use` at `thread/mod.rs:43`, no test touches them,
and no design template names them. The other workspace hits for the string "ThreadPool" are
diagnostic text about the `ThreadPoolApi` provider contract, a different thing.

**Already resolved.** The inline head+tail arithmetic was "duplicated across two call sites"; R1
removed one. The two remaining (`:957` and `:1920`) are both on the accumulator path, worker side and
merge side, and they must agree or the merge reads the wrong regions. Consolidating those two is a
real item, and it is not a deletion.

**Reserved, not deletable (R2c).**

*Per-core trunk ownership.* The audit called `core_phase_mask` the dead half against the live inline
`rank % ncores`. It is not dead in any sense that permits deletion: `core_mask.rs:3-4` names the mask
form "op's chosen mechanism (2026-06-07)", `tests/gate2_core_phase_masks.rs` exercises it across
eight assertions, and `dispatch/trunk_dispatch.rs:83` cites it as "the R4a rule" that the live inline
threading implements. It is the specification, with tests, and the live code is its implementation.
Deleting it removes the spec and its coverage while keeping the derived copy.

*Spawn marshalling.* D2 inverted this: the std tier's generic `spawn` is now real, so `spawn_fn` is
the dead half on both tiers. But `os.rs:114`'s `spawn_fn` owns a `trampoline` at `:135`, and the
generic `spawn` has its own separate monomorphic entry point, so which trampoline belongs to which
path needs tracing before either goes. Not a one-line deletion.

*The barriers.* See R2a above: dissolved, all three stay, one of them test-reachable and
design-documented.

**The generalisation, and it is the same one R1 produced.** An item marked dead in the audit is a
claim about reachability, and reachability claims in this arc have been wrong far more often than
right. Before any R2 deletion: find every reference across all crates including `tests/`, check
whether a design template names it, and check whether the live code cites it as its own rule. Three
of the five traced so far failed at least one of those.

## Band 2: wire existing substrate

Everything here has a complete implementation that nothing reaches. The work is the call site plus
whatever the call site needs, not the mechanism. This is the band the prior roadmaps most
misjudged.

**W1. Head+tail convergence** (canon 32). **The first draft of this item claimed the plan already
computes eligibility, citing `plan/fiber.rs:147-154`. That citation is a doc comment on the
`HeadTailConvergence` struct, not code.** `head_tail` is assigned exactly once, `Maybe::Isnt`
(`plan/fiber.rs:221`); `HeadTailConvergence` is constructed nowhere; `unit_meta.commutative` is
written at `plan/mod.rs:422` and read nowhere. The eligibility predicate is **absent**, not
computed. See the note at the end of this document: this is the same error the whole audit exists
to correct, committed inside the correction.

So W1 is not a band-2 wire. Its substrate is real (`thread::Convergence` with head and tail thread
handles and a `meeting_record: ProgressCounter`; the `HeadTailConvergence` shape;
`RecordRange::{Head,Tail}`), but two of the four pieces have to be built. Four sub-items:

**W1a, the eligibility predicate.** Consume `unit_meta.commutative`, single-trunk-phase, the
record-count threshold, and accumulation-compatibility; populate `Fiber.head_tail`. This is a
build, not a wire.

**W1b, decide what `mid_slot` means, then allocate whatever it needs.** A prior draft asserted that
`RecordRange::Head { mid_slot: USize }` (`hilavitkutin-api/src/dispatch_codegen.rs:338`) carries a
slot index into the progress arena rather than a record boundary, and made W1 depend on W2a on that
basis. **The fact-check could not support it, and the only written statement contradicts it**: the
variant's own doc (`:337`) reads "Head half: `0..mid` (head+tail convergence, head thread)", which is
a record boundary. The field *name* suggests an arena slot; the doc says a record index. Nothing
constructs `Head` or `Tail`, so no call site adjudicates.

Treat this as **undetermined and W1's to decide**, not as an established dependency. If it resolves
to a record boundary, W1 does not need the arena and does not depend on W2a. If it resolves to an
arena slot, it does. The meeting record is a `ProgressCounter` and `PoolFrame.progress_slots` is
`NonNull::dangling()` (`scheduler/mod.rs:802`), so the arena is unallocated either way.

**W1c, emit Head and Tail** from `core_program.rs:109`, which today always emits `Full` and sits
inside `synthesise_core_programs`. That function has **test-only callers**
(`tests/synthesise_core_programs.rs:60`, `:82`, `:114`), not zero callers as two prior drafts said;
nothing reaches it from an entry point. Needs W5b's honest accounting, not just W5a's reachability.

**W1d, two-walker dispatch and merge** at the meeting record, gated on W1a's predicate.

Dependencies are therefore R1 **plus W5b**, plus W2a **only if** W1b resolves `mid_slot` to an
arena slot. The draft said R1 alone, which was too few; the first correction said R1 plus W2a plus
W5b, which assumed the unverified arena reading.

**W2. Phase-overlap progress counters** (canon 33, 60). Substrate is genuinely complete:
`ProgressCounter` with correct Release/Acquire, `store_progress_arena`, `load_progress_arena`,
`emit_progress_release_fence` with a real `dmb ishst`. Canon's constraint that the producer store is
plain rather than `fetch_add` is already satisfied. Two sub-items, split because the risk is all in
the second:

**W2a, allocate the arena and publish.** Give `PoolFrame.progress_slots` real backing rather than
`NonNull::dangling()`, and publish at morsel completion. Mechanical. Shared with W1b, so whichever
lands first carries it.

**W2b, consumer-side acquire.** The downstream phase acquires on the counter instead of waiting on
the full barrier. This is the memory-ordering item, and **it is the one step in this roadmap a
sketch cannot fully prove**: the happens-before argument is provable on paper, but holding under
contention needs a stress harness. Plan for both.

**W3. Parking tiers** (canon 78). Substrate real, with a signature gap the draft missed:
`waist_barrier` takes `(pool, core, expected, now)` and carries **no phase index**, while
`predicted_wait_ns_load(pool, phase)` requires one. Threading a `PhaseId` through the barrier and
its call sites (`scheduler/mod.rs:917`, plus the second duplicate `waist_barrier` at
`dispatch/phase_run.rs:105`) is part of the work, so W3 sequences after R2a. Also
`spin_budget_for(class: CoreClass, ..)` needs a real `CoreClass`, so canon's E-core half depends on
W4. **Scope W3 to the uniform-core case**; the heterogeneous half lands with W4c. Canon's
thresholds are explicit: under 100ns spin, 100ns to 10us `spin_loop` with backoff, over 10us park.

**W4. Core classification and heterogeneous morsels** (canon 68). Mostly build, not wire. Three
sub-items:

**W4a, the platform probes** (sysfs, sysctl, `GetSystemCpuSetInformation`, and the fallback). Four
independent units, and **none is provable by sketch off its own platform**, which is a real
constraint on how this gets validated.

**W4b, pool construction consumes the classes.**

**W4c, heterogeneous morsel sizing and the thread-count formula.** This is the design content:
canon requires P-cores get **larger** morsels and E-cores proportionally smaller, not equal, and
thread count `min(physical_cores, parallelisable width + 1)`. Pairs naturally with W3's spin
budget.

**W5. Per-core programs and core assignment** (canon 34, 59). `assign_cores` has a real body called
only from its own tests; `synthesise_core_programs` fills `CoreProgram`s and has zero callers. But
that body is a **skeleton**, which the draft missed: `total_fibers` is a heuristic over non-zero
`morsel_windows` (`core_program.rs:72`), `trunk_count` is hardcoded `ZERO` (`:140`), and every core
is assumed to participate in every phase (`:118-133`). Making it reachable makes a placeholder
reachable. Two sub-items:

**W5a, make it reachable**, explicitly labelled skeleton in its own doc so the next reader is not
trapped the way this roadmap's first draft was.

**W5b, honest fiber and trunk accounting** from `FiberGrouping` and `assign_cores` output. W1c
depends on W5b, not W5a.

Neither makes canon 59's compiled per-core program live: that needs the codegen family, which is
`FiberShape`-gated with zero impls. The deviation ledger (`202606072100:19`) records it as
op-blessed away in favour of the runtime mask, and the draft presented that as a policy choice. The
mirror corrects the framing, and the fact-check corrects the mirror's citation: the wall for canon 59
is recorded at `202606072100:17` (full `specialization`, which is forbidden, or const-generic
recursion overflow). A1-2 (`202606111400:59-70`) does describe rustc "rejects field access on
generic constants under `generic_const_exprs`", verbatim, but its subject is consumer-tunable caps,
not the codegen family. That is GCE's live WATCH-tier rough edge per
`unstable-features.md`, so canon 59 is closer to **cannot be done as specified today** than to
declined. Record it as toolchain-forced, and revisit if the GCE situation moves. W5 is the
plan-side half only.

**W6. Strategy selection** (canon 72). **Also not a wire; the draft's premise was false.**
`DefaultSelector::select` (`strategy/mod.rs:30`) takes `(record_count, depth, fibers, roots)`.
Canon's rule needs producer and consumer weights, which are not parameters and have no producer
anywhere in the crate. The `Strategy` enum (`strategy/mod.rs:8-13`) has no `ChaseSteal` variant,
though canon's rule and this roadmap's own prose both name it. Precision the mirror added: the
shipped body is a **three-way** return, `Sequential` / `Adaptive` / `Phased`, and `Phased` is live.
The draft's "only ever returns `Adaptive`" was wrong; what is true is that `ChaseSteal` does not
exist and `PipeChase` is never constructed. Four sub-items, and the whole thing
is bench-gated rather than band-2:

**W6a**, add the `ChaseSteal` variant and change the selector's trait signature to take weights.
**W6b**, compute producer and consumer weights in the plan (canon defines weight as WU count times
column access counts). **W6c**, the LIGHT_THRESHOLD bench, since canon names the constant and never
values it. **W6d**, wire the selection into the plan and consume it at dispatch.

## Band 3: build the absent

**B1. Parallel incremental skip** (canon 52). `run_core_phase` dispatches all-ones dirty. Canon
places the skip before each pass with no parallel exemption. Depends on W2's happens-before
argument for the dirty mask's stability across publish and await.

**B2. `run_fused` per-fiber window.** The third dispatch path still takes the const fallback.
Mechanical.

**B3. Plan chain steps 6 and 7 consumed** (canon 49, 50). Dulmage-Mendelsohn integration plus
dead-column elimination, and spectral trunk formation consumed by the runner. Substrate is upstream
in arvo-sparse and arvo-spectral. Canon gates spectral at more than 5 fibers. #635 is a cross-repo
prerequisite to check first; #644 becomes runnable once real greedy exists.

**B4. Matrix-chain DP fiber grouping** (canon 51). Canon: greedy at 10 ops or fewer, DP above, cost
`record_count x sum of size_of::<T_k>()` over the union columns of ops i..j, both modes sharing the
holistic feasibility predicate. `arvo-comb::matrix_chain_dp` exists.

**B5. RCM row order as dispatch order** (canon 48). Computed today, and **not discarded**: it is
consumed by `RecommendedOrder::from_rcm_order` (`scheduler/mod.rs:374`) and persisted via
`store_column` (`:594`). What is absent is its use as the WU execution order, which is the actual
gap; two prior drafts overstated it as discarded. Canon is explicit
that RCM produces two orderings and the row one is the execution order fed to step 8. The standalone
spec constrains the mechanism: the zero-indirect form needs a pre-monomorphised RCM-ordered carrier
variant, and whether the cache win justifies it is a benchmarked decision. So this is a bench-gated
fork, not a straight build. Dissolves the `NonTopologicalRegistration` restriction.

**B6. Per-morsel generation counters** (canon 26). Coarse per-store dirty only today. Needs the
storage sketch: where counters live under no-alloc when morsel count varies with the window.

**B7. Column classification consumed** (canon 39). Classification computes but everything lands
Internal, so DSE cannot skip memory ops for genuinely internal columns.

**B8. Version stamps** (canon 81), **micro-morsel tiling** (canon 24, canon marks it ECS-scale so
it may stay deferred), **shared read columns between trunks** (canon 35, canon states a
recommendation rather than a requirement).

## Band 3b: canon-completion items no band covered

Surfaced by the canonical-mirror pass. All four are named in the locked A1-3 addendum
(`202606111400:79-91`) as chartered completion scope, and appear in no band of any roadmap
revision including this one's first draft. None is large; the finding is that they were invisible.

**Executor work-stealing extension point** (canon 69). `steal_fallback` (`thread/mod.rs:104`) is
`todo!()`. Canon requires the engine ship the extension point, not a stealer, so the item is a trait
surface plus the deterministic default staying default.

**Sub-byte bitpacking stride.** `ColumnValue::BIT_WIDTH` is declared; stride is still
`size_of::<T>()`. Canon 1 makes stride type-native, and canon 3 carves out sub-byte types via
specialisation.

**Intrinsics and day-one microkernels** (canon 93, domain 13, spec:878-932). Seven named:
cache-line zero, paired load/store, LSE atomics, non-temporal store, fences, worker parking,
trailing zeros. Note canon explicitly **removed** explicit prefetch (about 2x slower on Apple
Silicon) and **bans** `likely`/`unlikely` in the dispatch loop, so this item is partly deletion.

**Schedule introspection** (#183), and kits-and-providers polish from the same A1-3 line.

## Band 4: consumer surfaces

Morsel-absolute slice accessor. `PipelineResult` with per-fiber Completed/Failed/Poisoned and
dependent poisoning (canon 83). Persistence engine bridge, evict and inject (canon 84). Plugin-host
facade bridge. viola integration (#254). Plus the standing consumer-pull lane: a downstream
consumer hitting a substrate limit gets the substrate fixed upstream additively, whenever it
surfaces, which is how `Plannable` and `HintExt` landed.

## Band 5: measurement

The perf gate's red arms, the ASM contract fixtures per typestate slice, and the benches each
earlier band names: W6's LIGHT_THRESHOLD, B3's #644 and #635, B5's RCM cache-win fork, and the
per-fiber walk-overhead measurement r6 identified. Canon's parallel target is parity with an
optimally threaded standard-library baseline at equal core count.

## Open questions this roadmap does not settle

**Which document is canon: RESOLVED by op, 2026-07-19.** Neither blanket-wins. Both
`202606061000` and `202606111800` are amendments to the consolidation spec, not replacements for
it. Each conflict is resolved on its own merits, and the consolidation spec is the tiebreak.

Consequences to carry: a later document does not silently override a decision it never discusses,
which is what a blanket-supersession reading would have allowed. The storage addendum's
bench-decided layout survives on its own merits (it is a bench result, and the spec has no
contrary finding). And any future claim of the form "document X supersedes Y" is not
self-executing; it needs the specific conflict named and resolved.

**Steps 7 and 8 run out of numeric order** per the spec's own text ("After fiber grouping (step 8,
which runs first for the initial greedy pass)"). No memo resolves it. Affects B3 and B4 sequencing.

**LIGHT_THRESHOLD** is named in canon and never valued.

**D1's contract choice: RESOLVED by op, 2026-07-19.** Sketch the install arm first, then decide.
The sketch establishes whether writing into the blob data plane is cheap; the contract decision is
taken with that in hand rather than before it.

## Closing note: the error this document committed while correcting it

The audit that grounds this roadmap defines a category, HOLLOW, for reachable code whose body does
not do what its own doc comment says, and argues that such comments are traps because everything
above them assumes the behaviour. It names the failure that poisoned six weeks of roadmaps: a 2026-06-08
document claimed head+tail "ships", citing a line; the line existed; the behaviour was not canon's.

The first draft of W1 in this document then wrote that "plan-side eligibility already computes
COMMUTATIVE plus single-trunk plus record threshold plus accumulation-compatible", citing
`plan/fiber.rs:147-154`. Those lines are a **doc comment on the `HeadTailConvergence` struct**
describing what the mechanism would do when built. `head_tail` is assigned `Maybe::Isnt` once and
never otherwise; the struct is constructed nowhere. The same error, on the same mechanism, inside
the document written to correct it.

Two things follow, and both are load-bearing for whoever works this roadmap.

**The audit's method is sound but not self-enforcing.** Call-site tracing is what produced every
correct row. For this one item, doc-reading was substituted for tracing, and nothing in the process
caught it, because a ledger is only as good as its weakest row and it cannot check itself. The
granularity pass caught it only because it was briefed to stress-test the cheapness premise against
the substrate rather than to review the reasoning.

**"Substrate exists" must be established by construction sites, not by type declarations.** A type
being declared, having a `Default`, and being re-exported proves nothing about whether anything
builds one. For every remaining item in band 2, the check is: what constructs this, and what reads
it. Where the answer is "nothing", it is a build, whatever its doc comment says.

## What is deliberately not here

Any status inherited from a prior roadmap. Any claim that a mechanism ships because a document said
so. The FiberCons nested carrier, which the 2026-06-07 fork established is unwireable from flat
registration and which the mandate replaced.
