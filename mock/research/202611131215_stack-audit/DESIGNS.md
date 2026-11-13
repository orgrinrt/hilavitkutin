# The Designs, One At A Time

Sibling of `AUDIT.md`, which carries the summary, the marks and how to read them. This file holds the verdict and the evidence for each major design in hilavitkutin. Paths are relative to the repository root unless they name a branch.

The order is the order the engine runs in: the contracts a consumer writes against, the plan chain that reads them, the dispatch that executes the plan, fusion, parallelism, the stores, and the things that sit beside all of it (adaptation, failure, the ecosystem crates, the build crate).

## The contracts layer

Mark: easy to get wrong but can be right.

What right requires: the contract surface must stop naming the engine.

The idea is that a work unit declares an access set and an execute body, the engine derives everything else, and a consumer who writes a unit never learns how the engine is built. Op put the test himself: the plumbing may change and nobody who wrote a kernel or implemented a store edits a line (channel `roadmap.md:502-559`, the sufficient-shape checks). That is the right test and the surface fails it today in three places.

The author spells the engine's `Ctx` type in the unit's own signature (`crates/hilavitkutin-api/src/work_unit.rs:72-78`, `crates/hilavitkutin/src/plan/bundles.rs:132`). Record access is an unsafe read and write at a bare index (`crates/hilavitkutin-api/src/context.rs:117-137`). The morsel range reaches the author through `BatchApi` (`crates/hilavitkutin-api/src/context.rs:226-230`), which hands out exactly the plumbing fact the engine is supposed to own. Each of those is a line a kernel author would have to edit if the engine changed shape.

The design is also ahead of the code in a way that reads as settled when it is not. `LinkedBin<dyn TraitFamily>` appears in the api design at `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:193-214` and in the engine design's builder example as `LinkedBin::<dyn LinterApi>` (`mock/crates/hilavitkutin/DESIGN.md.tmpl:1239`), in a stack whose own instructions ban `dyn` in every crate. `ColumnValue` is a blanket over `T: Copy + 'static` with no size bound in the api design (`:218-237`) while the seed says at most 16 bytes, enforced at compile time (`mock/research/seed/data-model.md:13-17`). `PlanAffecting` is sealed in the api design (`:1119-1126`) and open by op's ruling in the seed (`mock/research/seed/storage.md:84-87`). The builder is `.add/.resource/.column/.memory/.threads/.clock/.build()` in the seed (`mock/research/seed/scheduler.md:10-20`) and a single `.with` in the api design (`:34-75`).

None of that is hard to fix. All of it is the same defect: several documents at the same tier, none of them ratified, each read as the contract by whoever opened it.

## The api revision

Mark: good as it is, with one part going wrong.

The revision (branch `work/api-revision`, round `mock/design_rounds/202611130617-h6mb_topic.revise-the-api-in-place.md`) is the right move and it is the most careful piece of design work in the record. It takes the surface in place on op's word rather than nuking it (`:37`), it sets out what it takes from the earlier map and what it leaves (`:43-47`), and it records that no human has sat on it and every decision is provisional (`:13`).

The value function is the best idea in it. A unit writes `ctx.each().map(|r| rec![...])` and returns values rather than reaching into a column at an index (`mock/crates/hilavitkutin-api/DESIGN.md.tmpl:291-318` on that branch), with `RecordOp<S>: WorkUnit<S>` so the fused and unfused forms are one trait family rather than two. That removes the unsafe index, removes the morsel range from the author's view, and makes fusion a property of the shape rather than an opt-in. Three of the surface's four leaks close with one change.

`Reach<C, const N: Cap>` with `Window` here, before and after is the right shape for a neighbour read, and putting the geometry in an opt-in crate keeps the engine out of the consumer's data model. There is no example and no sketch of it yet, which is the gap.

One part is going wrong. The round refuses two writers of a buffered column, in those words (`:43-45`, map row 21), and op's answer on exactly that question was that a schedule should exist for this specifically, and asked whether we are sure we know what hilavitkutin is (channel `op-answers-13`, item 2.27 of the op-words digest). A refusal is the opposite of what he asked for. The engine's existing rule is also not what three design texts say it is: the DAG builds a write-write edge from the registration index alone (`crates/hilavitkutin/src/plan/step1_dag.rs:45-51`), while the consolidation, the seed data-model and the api design all say hints first, then dependents, then registration as the fallback, and op re-affirmed that order on 13 November. No ledger entry records the deviation.

The round also reports that the branch carries a `registration_binding` bench whose own changelist says "No committed number shows the value function at parity" (`:406` of the changelist on that branch). That is honest and it is the number the direction needs.

## The plan chain

Mark: easy to get wrong but can be right.

What right requires: an oracle. Until one exists no step of the chain can be shown wrong, and several steps are already known to compute things nothing reads.

The chain is thirteen steps from the declared access sets to a per-core program (`mock/crates/hilavitkutin/DESIGN.md.tmpl:30-95`). The idea is sound and the decomposition is sensible: build the dependency graph, group into phases, cut trunks, cut fibers inside trunks, size the morsel from the write set, order for locality. Where the design is specific it is usually right, and where a step was probed it worked.

Four things in it are computed and read by nothing. `COMMUTATIVE` is declared on the unit and read nowhere (`crates/hilavitkutin-api/src/work_unit.rs:84`, `crates/hilavitkutin/src/plan/project.rs:162`). `plan.morsel_sizes` is read only at plan build and the dispatch uses a uniform `Cfg::MORSEL_SIZE` (`mock/research/202606201300_adapt-actuation-chart.md:6-30`). `phase.strategy` is read nowhere, same file. `Fiber.head_tail` is computed and unused (`mock/research/202607181200_engine-arc-recomprehension.md:24-45`). A plan step whose output nothing consumes is not a step; it is a cost paid at build time for a value that cannot be wrong because nothing can disagree with it.

The step-7 and step-8 roles are swapped against the design: the shipped chain uses greedy where the design puts spectral and the other way round, and the round that found it says restoring the design's roles is op's call (`mock/research/202607201200_spectral-role-bench-evidence.md:70-78`). The bench behind that is discussed in `BENCH-RECORD.md`; the short version is that it measures plan-build time on a bare DAG with no access matrix, which is not the thing the roles were chosen for.

The registration order must be topological or the build fails with `BuildError::NonTopologicalRegistration`, and the engine design calls this provisional (`mock/crates/hilavitkutin/DESIGN.md.tmpl:1143-1149,:1171-1183`); the seed says it must never be presented to consumers as permanent (`mock/research/seed/contracts.md:83-91`). Both are right, and it is a leak of the plan chain's internals into the order a consumer writes lines in.

## Const-gated dispatch

Mark: easy to get wrong but can be right.

What right requires: it has to hold at a game's unit count, and nothing has tried.

This is the cleverest part of the engine and it works. Access sets fold to const masks, a const function groups them into phase and trunk arrays, and a position-threaded walk uses `generic_const_exprs` to monomorphise per trunk so that a unit outside the trunk is dead code (`mock/research/202606070700_engine-roadmap-r4-gate2-mechanism.md:14-28`; `crates/hilavitkutin/src/plan/grouping.rs:1-23`, `crates/hilavitkutin/src/dispatch/trunk_dispatch.rs:5-21`). Six sketches established the pieces separately and the disassembly shows no indirect call. For what it does, this is good work.

Three things say it is at the edge of what the toolchain will carry. `GATE2_MAX_UNITS` is 256 and `GATE2_MAX_ACCUMS` is 16, both fixed constants rather than capacity-derived, because the capacity-sized bound "overflows the trait solver (a generic-constant well-formedness loop)" (`crates/hilavitkutin/src/plan/grouping.rs:38-48,:55`). An inline const block had to be replaced by associated consts because of a "generic-constant complexity limit" (`crates/hilavitkutin/src/dispatch/trunk_dispatch.rs:44-48`). And the roadmap that established the mechanism admits the N-wide array fold is unproven while claiming "no feasibility unknown remains", on a proof of three units (`mock/research/202606070700_engine-roadmap-r4-gate2-mechanism.md:32-35,:74`).

Those two constants are also against op's standing instruction that everything is a knob and there are no constants in engine code.

What actually runs is a hybrid, and the design should say so: compile-time trunk membership, a runtime phase compare in the caller's loop (`crates/hilavitkutin/src/dispatch/trunk_dispatch.rs:14-19`), a runtime dirty mask, and a fiber member mask rebuilt per phase and fiber at runtime with a FIXME on it (`crates/hilavitkutin/src/scheduler/run.rs:232-242`). The deviation ledger records the blessing for the runtime ownership mask (`mock/research/202606072100_gate2-canonical-deviation-ledger.md:13-19`), so this is a known and approved deviation, not drift. It is the gap between "compiled per-core programs" as the design says it and what the words mean in the tree.

The codegen half is declared and empty: `codegen_fiber` and `codegen_core` are typed-correct, body-empty skeletons (`crates/hilavitkutin/src/dispatch/mod.rs:114-128`; `mock/crates/hilavitkutin/DESIGN.md.tmpl:477-505`), and `CoreDispatch` resolves to `()` (`crates/hilavitkutin/src/dispatch/dispatch_codegen.rs:396-408`).

## Three run entries

Mark: going wrong.

The finding here is not the count of entry functions. Three functions that differ only in which strategy they select would be fine, and the amendment is about dispatch shape and code paths rather than about public names (`mock/research/202606061000_canon-amendment-unified-engine.md:7,:9,:25`).

The finding is that the three differ in behaviour. The incremental skip applies on one and not the other, and the source says so in a comment calling itself the pure-RAW path (`crates/hilavitkutin/src/scheduler/run.rs:250-254` against `crates/hilavitkutin/src/scheduler/worker.rs:320-322`), so the same program does a different amount of work depending on which entry the consumer called, and `run_fused` additionally loses accumulator appends that the other two keep (`mock/crates/hilavitkutin/DESIGN.md.tmpl:381-390`). An amendment that says the same code paths run at one, two and seven cores is not satisfied by paths that compute differently. No ledger entry records it, which is the part that matters: a blessed deviation is a decision and an unrecorded one is a surprise for the next reader.

## Fusion

Mark: going wrong.

The intent is that a chain of per-record operations becomes one loop with the intermediates in registers, which is where the bulk of the engine's promised advantage over a per-pass implementation comes from. The evidence that it works in isolation is good: a sketch shows a 2.09x gain with the unfused form at two stores three loads and the fused form at one and one (`mock/research/sketches/202606061600_d4-internal-column-fusion/src/main.rs:307-340`), and a second shows the `RecordOp` fold at 131 instructions down to 72 (`mock/research/sketches/202606091200_d4-recordop-chain-fusion/src/main.rs:441-472`).

What ships cannot carry the headline. The mark is on the combination rather than on any one choice, and the design is honest about the constraint behind the first of them: fusion is a separate entry point, `run_fused`, chosen by the consumer, because automatic detection is not expressible in the current type system (`crates/hilavitkutin/src/dispatch/fusion.rs:21-28`, and `mock/crates/hilavitkutin/DESIGN.md.tmpl:372-377` names `min_specialization` as the reason). The carrier must fold to one linear chain of one-in one-out operations (`:157-161`), so a diamond does not fuse, and the round that measured a diamond found that materialisation is the dominant branching gap and that fusing it recovers about 75 percent (`mock/research/202606072130_gate2-r4d-branching-bench-finding.md:53-58`). And the fused entry silently loses accumulator appends, which the engine design records as a known hole with a ticket (`mock/crates/hilavitkutin/DESIGN.md.tmpl:381-390`, ticket `0fb716c5a463`).

So the mechanism that is supposed to close the gap is opt-in, shape-restricted, and unsound in combination with another feature. The api revision's value function is the way out, because a unit that returns values rather than writing columns makes fusion a property of the declaration; this is the strongest argument for that revision and the round does not make it.

The intermediate question is not settled either. The design's answer to materialisation is a co-located arena per fiber (`consolidation:959-975`). The ad-hoc spike written for this audit (`mock/research/sketches/202611131400_chain-shapes-adhoc/`) puts numbers on why that matters: on one kernel, on one guest, storing intermediates as full columns costs 2.2x to 3.0x of a fused loop out of cache, and the same morsel-wise structure with intermediates in a reused scratch buffer costs 0.64x to 1.27x. Those are ad-hoc spike numbers for one kernel and they are not a measurement of hilavitkutin. They do say the design is pointed at the right mechanism.

## Trunks, fibers and phases

Mark: good as it is.

Column-disjoint trunks, one per core, with no synchronisation inside a phase and a waist as a mandatory boundary, is a good model. It gets parallelism from the declared access sets rather than from locks, it has no shared mutable state by construction, and a sketch measured 2.84x on three cores against an ideal of 3.00x (`mock/research/sketches/202606070500_gate2-b2-trunk-parallel-speedup/src/main.rs:315-338`, a sketch timer, not a harness bench).

The weakness is that it is bounded by the number of column-disjoint trunks a program happens to have. A simulation whose units all touch the same few columns has one trunk, and then the model gives nothing. That is what the next section is about.

## Record-range parallelism

Mark: going wrong.

The seed states the rule as "never an N-way record or morsel partition" (`mock/research/seed/execution.md:17-23`) and the answer for a single-trunk phase is head-and-tail convergence: two walkers claiming records from opposite ends. The amendment behind it is quoted in the round that deleted the alternative (`mock/design_rounds/202607192110/202607192100_topic.n-way-record-split-is-not-convergence.md:31-33`).

Two walkers is at most 2x. Op's stated region is millions of records crunching constantly (channel `op-answers-20`). A rule that caps the only available parallelism for same-column work at two walkers works directly against that goal, and this audit says so as a finding rather than defending the row: the rule is right about what it forbids (an unordered split of records with no commutativity gate) and wrong about what it permits (only two). The composition that satisfies both is an N-way split gated on the declared commutativity, which the engine already computes and reads nowhere (`crates/hilavitkutin-api/src/work_unit.rs:84`).

The design has an answer to this and it needs taking seriously, because it is the reason the cap looks acceptable. The consolidation states that deep pipelines never significantly benefit from threading, and supports it with a seven-fiber deep pipeline gaining 1.03x to 1.05x (`consolidation:1919-1920`). If that holds, a long chain over the same columns has nothing to gain from more walkers and two is as good as eight.

It does not hold, because the measurement and the conclusion are about different kinds of parallelism. A seven-fiber pipeline threaded across its stages is pipeline parallelism: each thread runs a different stage, the stages are coupled by the data flowing between them, and the speedup is bounded by the slowest stage and the handoff. That is the thing that gains 1.03x, and the number is believable. Splitting the records of one fused chain across threads is data parallelism: each thread runs the whole chain over its own slice, with no coupling at all. The ad-hoc spike of this audit measured the second on a four-stage chain on a four-core guest and got 2.6x to 3.7x over one thread at 2^24 and 2^26 records (`mock/research/sketches/202611131400_chain-shapes-adhoc/FINDINGS.md`, an ad-hoc spike on one kernel, not a measurement of hilavitkutin). The same kernel is memory-bound at those sizes, which is where one would most expect threading to give nothing, and it still gave most of the machine.

So the consolidation's sentence is right about what it measured and has been read as a reason for something else. A design decision that caps record-range parallelism at two rests on it, and the two things it joins together should be separated before that decision is defended again.

What ships is the ungated N-way split that the round called drift: exactly one trunk and more than one core gives per-core ceil windows over the record range (`crates/hilavitkutin/src/scheduler/worker.rs:323-348`), with a comment citing the spec line for head-and-tail. A unit that reads another record's index across a slice boundary races, and nothing constrains the index.

The measured picture, from the one harness bench that exists for this (`mock/benches/single-trunk-convergence/FINDINGS.md` on `work/bench-convergence`): the engine cannot run head-and-tail at all, because dispatch never emits head or tail ranges (`:11`), so the bench built a crew outside the engine; head-and-tail beats one walker by 1.58x to 1.84x (`:19-32`); at four and eight walkers the N-way form leads by 1.04x to 1.88x (`:42-56`); and append falls back to sequential under head-and-tail (`:38`). The bench says of itself that it is a baseline and no design call rests on it (`:3`), which is the right standing for it.

## Worker aliasing and failure

Mark: going wrong for the aliasing, back to the drawing board for the failure policy.

The worker holds a raw `*const Scheduler` across parks while the public surface takes `&mut self`. The deviation ledger carries it as item seven, sound by discipline only (`mock/research/202606072100_gate2-canonical-deviation-ledger.md:73-79`), and the audit of it says in its own words that blessing the shipped shape would bless undefined behaviour (`mock/research/202607201600_gate2-deviation-5-audit.md:43-44`). It is catalogued as an ignored test because it is not natively observable (`:49-52`). A known miscompilation-class hole carried as an ignored test is the one item in this audit that should be fixed before anything else in the engine is built on top of it.

Failure is the part that needs rethinking rather than repair. The policy is abort (`consolidation:2448-2451`), a worker panic can hang `frame_await_done` (`mock/crates/hilavitkutin/DESIGN.md.tmpl:607-610`), `PipelineResult` is designed and not shipped (`mock/research/seed/scheduler.md:117-120`), and zero workers clamps to one with undecided behaviour and can hang (`mock/crates/hilavitkutin/DESIGN.md.tmpl:1421-1432`). For a batch pipeline that is a defensible choice. For a daemon it is not a choice at all: a server that aborts the process because one unit saw bad input is not a server. `DOWNSTREAM.md` goes through what this costs.

## Storage, the record count, and growth

Mark: going wrong.

One scheduler has one record count for every column (`crates/hilavitkutin/src/scheduler/builder.rs:170`; `mock/crates/hilavitkutin/DESIGN.md.tmpl:1188-1196`). Columns are sized at plan time and never resized (`mock/research/202607200200_replaceable-swap-semantics-spec.md:62-63`), the storage holds one base pointer and re-reserving is undefined (`crates/hilavitkutin-api/src/storage.rs:46-47,:59`), and the memory provider cannot grow (`crates/hilavitkutin-api/src/platform.rs:28-53`). The seed meanwhile says the engine does not know record counts ahead of time (`mock/research/seed/plan.md:214-216`), which the builder signature contradicts.

Op has answered the shape of the fix in words: contiguous per prototype, with indices to process the needed windows (op-words digest item 2.29, channel `op-answers-13`). That is a per-domain record count, which is what the api revision adds as a new row (`mock/design_rounds/202611130617-h6mb_topic.revise-the-api-in-place.md:43-45`, row 4). The direction is settled; the engine has not moved.

The resource model itself is good. A contiguous blob with erased static-shape addressing was chosen by a bench against five alternatives, with the decomposed form losing 1.90x to 3.06x on a wide gather and the erased form within about 1.7 percent of native (`mock/research/202606210600_storage-bench-findings.md`), and op ruled on it (`mock/design_rounds/202607161000/...hybrid-decision:12-19`). It contradicts `mock/PRINCIPLES.md.tmpl:119-126`, which still says every aggregate decomposes, and that contradiction should be resolved in the principles rather than argued again.

## Accumulators

Mark: easy to get wrong but can be right.

What right requires: the admissible input must stop depending on the worker count.

An accumulator is a fourth store kind with append semantics, per-core regions merged after a phase. The capacity is the record count (`mock/crates/hilavitkutin/DESIGN.md.tmpl:1194-1205`), the per-core capacity is `ceil(N/cores)`, and the kept records under `run_parallel` therefore depend on how many cores the machine has (`:414-420,:597-603`; `crates/hilavitkutin-api/src/context.rs:66-88`). A program that keeps every record on a four-core machine and refuses some on a two-core machine is not deterministic, and determinism is the thing a simulation cannot trade.

The overflow semantics were three different things in three places at the time of the panel reading, and `dev` has since landed a sticky refusal (`c42d707`), which is the right direction. `AccumRefused` does not yet distinguish the worker-count case.

## Adaptation

Mark: easy to get wrong but can be right.

What right requires: one axis actuating end to end, measured, before the other eight are built.

The design is nine axes of runtime adaptation feeding the next frame's plan parameters. The types exist and the data does not: `select_adapt_config` is unimplemented, `AdaptWu::execute` is a stub, `ema_update` has no-op bodies, and eight of nine axes carry types with no data behind them (`mock/research/202606191600_p1-adapt-comprehension.md`; `mock/crates/hilavitkutin-providers/src/adapt_wu.rs:91-99`, `adapt_ema.rs:87-126`). Seven performance contracts for it are catalogue-red and ignored under a ticket, which is the right way to carry a design that is not built.

The reason it cannot actuate is upstream of the stubs: the two values adaptation would set, the morsel size and the phase strategy, are read at plan build and nowhere else (`mock/research/202606201300_adapt-actuation-chart.md:6-30`). Until the dispatch reads a per-fiber window, there is nothing for an EMA to move.

## Incremental skip

Mark: easy to get wrong but can be right.

What right requires: the same skip on every path, and a changed-bit that is not the consumer's to set by hand.

The mechanism is a dirty bit per store, propagated to the units that depend on it, and it is proven in a sketch and landed on the single-core path. It does not apply on the parallel path (`crates/hilavitkutin/src/scheduler/worker.rs:320-322`) nor on the unit-outer path (`mock/research/202606082300_run-parallel-meta-parity-findings.md:20-22`), so the saving disappears exactly where the work is largest. The bit is set by the consumer calling `mark_dirty` (`crates/hilavitkutin/src/scheduler/dirty.rs:87-94`), which makes correctness depend on the consumer remembering; op's answer, that the writing unit declares it (op-words digest 2.24) and that changed is declared per column (api-revision topic `:91`), is the better rule and is not built.

## The ecosystem crates

`hilavitkutin-sym` and `hilavitkutin-ctx`: good as it is. Small, self-contained, no unsafe, contracts that do what their designs say. The sym README says a 4-bit kind against 3 in the design and the source (`crates/hilavitkutin-sym/src/kind.rs:21`), which is a one-line correction.

`hilavitkutin-str`: good as it is, with one thing to be decided openly. It registers const strings through linker sections (`crates/hilavitkutin-str/src/macros.rs:16,:20`, `src/section.rs:23-60`) in a stack whose instructions ban linker-magic registration in every crate. The crate's design documents the exemption (`mock/crates/hilavitkutin-str/DESIGN.md.tmpl:92-147`), so this is a declared carve-out rather than a violation; the ban should name it, or the carve-out should go. The Windows walker is absent.

`hilavitkutin-linking`: going wrong, and this one is a defect rather than a disagreement. `RTLD_LOCAL` is defined as 4 (`crates/hilavitkutin-linking/src/backend/unix.rs:8-9`) and passed to `dlopen` as `RTLD_NOW | RTLD_LOCAL` (`:26`). Four is the macOS value; on the glibc in this container `RTLD_LOCAL` is 0 and 4 is `RTLD_NOLOAD` (`/usr/include/x86_64-linux-gnu/bits/dlfcn.h:27,:38`, read for this audit). So on Linux glibc the loader asks for the library not to be loaded, and `Library::load` returns null for any library not already resident. The ten tests in `tests/basic.rs` do not catch it. This is also exactly the failure the workspace rule about gating per target and per feature exists to prevent: one unix constant for two platforms that do not share it.

`hilavitkutin-providers`: easy to get wrong but can be right. The arena interner guards a sentinel before an unchecked read (`crates/hilavitkutin-providers/src/interner.rs:219`, `from_utf8_unchecked` at `:235`) while the handle masks to 28 bits, so a `u32::MAX` sentinel arrives as `0x0FFFFFFF` and may miss the guard. That is the slice reader's inference and this audit did not run it; it is a reachability question that one test would settle. Eleven `unimplemented!` stub bodies in the adapt provider are honest placeholders for a design that is not built.

`hilavitkutin-persistence`: easy to get wrong but can be right. The cold store opens without reading a manifest, flushes without writing and reports success, loads reporting missing, and snapshots nothing (`crates/hilavitkutin-persistence/src/cold_store.rs:32,:60,:69,:78`). The three ignored tests encode the manifest and backup promises, which is the right way to carry it. A flush that writes nothing and returns success is the one shape to change now, because a caller cannot tell it from a flush that worked.

## hilavitkutin-build

Mark: back to the drawing board.

The crate's design says every hilavitkutin crate and every consumer's `build.rs` calls `hilavitkutin_build::configure().run()` (`mock/crates/hilavitkutin-build/DESIGN.md.tmpl:3-6`). No such function exists: the public surface is an enum of thirteen pragmas, a profile enum, three axis enums and a requirements table (`crates/hilavitkutin-build/src/lib.rs:55-68`). The whole of what the crate emits is one cargo directive, `cargo::rustc-cfg=arvo_fast_math`, and only when the fast-math pragma is in the set (`crates/hilavitkutin-build/src/bootstrap.rs:286-288`), plus a config file of two comment lines and two values that the source itself calls a placeholder read by nothing (`:273-275,:296`). No crate in the tree depends on it.

The design meanwhile owns the flags: the LLVM plugin arguments, the fat-LTO settings, the PGO and BOLT workflows, the `target-cpu=native` inference, the profile-staleness warning (`:166-208,:216-268`). None of those strings exists in the source. The scripts the design calls existing implementation (`:281-303`, a 94-line pass plugin, a math peephole, a 407-line release container script) are not in the tree. The design's gain figures (3 to 5 percent static BOLT, 10 to 20 percent PGO) name no file.

This matters beyond the crate because `ExpandedLto` is load-bearing for the engine's whole devirtualisation story (`mock/research/seed/foundations.md:133-137`), and op has said more than once that he wants the vectorisation work here (op-words digest 2.52). What exists is a pragma vocabulary with nothing behind it. The repair is not to finish the design as written; it is to decide what the crate is for now that cargo and the toolchain have moved, and to write a small thing that works rather than a large thing that is declared.

## The contradictions, gathered

These are the places where two documents in the repository say different things about the same decision. Each is cheap to resolve and each has cost someone a reading.

One engine path (`mock/research/202606061000_canon-amendment-unified-engine.md:7`) against three run entries (`mock/crates/hilavitkutin/DESIGN.md.tmpl:261-294`).

Every aggregate decomposes (`mock/PRINCIPLES.md.tmpl:119-126`) against the resource blob never decomposed (`mock/crates/hilavitkutin/DESIGN.md.tmpl:509-520`).

Inventory pattern recommended (`mock/PRINCIPLES.md.tmpl:78-79,:140-145`) against linker-magic registration banned in every crate (`.claude/CLAUDE.md`).

No std tier (`mock/PRINCIPLES.md.tmpl:198-202`) against three tiers including std (`consolidation:275-283`).

Wake thresholds under 100 ns and 10 us (`mock/PRINCIPLES.md.tmpl:238-239`) against 2 us and 50 us (`mock/crates/hilavitkutin/DESIGN.md.tmpl:72`) against a measured default of park immediately with a spin budget of zero (`mock/research/202607202340_wake-policy-evidence.md:34-66`).

The engine holds no thread and the executor is the consumer's (`mock/crates/hilavitkutin/DESIGN.md.tmpl:1404-1414`) against a pool of `physical_core_count` threads joined on drop (`:614-616,:692-698`).

`PlanAffecting` sealed (`mock/crates/hilavitkutin-api/DESIGN.md.tmpl:1119-1126`) against open by op's ruling (`mock/research/seed/storage.md:84-87`).

`ColumnValue` any `Copy` (`mock/crates/hilavitkutin-api/DESIGN.md.tmpl:218-237`) against at most 16 bytes enforced at compile time (`mock/research/seed/data-model.md:13-17`).

Units buffer internally and write on return (`mock/research/seed/data-model.md:132-134`) against a straight pointer write into the column (`mock/research/202606052000_single-core-engine-ideal-vs-actual-audit.md:66`).

Lint severities never go down and a blocking gate is the design (`mock/research/seed/constraints.md:97-98`) against the warn and off severities in `mockspace.toml:207-238`, with a comment there naming 559 drift sites.
