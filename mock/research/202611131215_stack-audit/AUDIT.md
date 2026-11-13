# Stack Audit: hilavitkutin In The notko, arvo, hilavitkutin Stack

Date: 13 November 2026. Branch `work/stack-audit`, cut from `dev` at `ada51bb`. Nothing outside this directory and `mock/research/sketches/202611131400_chain-shapes-adhoc/` was changed.

This is one reading of the whole record: the designs, the research notes, the design rounds, the seed, the benches, the sketches, the unlanded branches and the source, judged first as intent and only then as code. It was asked for as a quality review of the stack, so it asks of every part whether the idea is right, whether the design derives from the idea, and whether the code could ever reach the design, in that order. The code is read as evidence that a design is buildable, never as the thing being judged.

The sibling files carry the parts that would make this one unreadable. `DESIGNS.md` holds the verdict and the evidence for each major design. `BENCH-RECORD.md` goes through every bench and spike on record, says why the numbers are what they are, and names the calls that were drawn from them. `ESTIMATES.md` reasons about how the designs would perform against hand-written code, against std and no_std Rust, and against the frameworks a reader would compare them to. `DOWNSTREAM.md` is the fit to what kaski, generic applications and server daemons need. `LEDGER.md` says what was read, to what depth, and what was not. `BENCH-CATALOGUE.md` is the list of benches still worth writing, posted separately on 13 November.

## The one page

hilavitkutin is a good idea, and the record contains an unusual amount of real evidence that the hard parts of it are buildable. The concept is a morsel-driven dataflow engine over columns where the schedule is a compile-time fact: work units declare typed read and write sets, a plan chain turns those into phases, trunks and fibers, and dispatch becomes a monomorphised walk with no indirect call. The pieces that sound impossible have been probed and work in isolation: const grouping from access-set types, a const-gated trunk walk that leaves no `blr` in the disassembly, register-resident fusion of a unit pair, a type-level fiber partition. The contracts-first instinct behind it is right, and the stack below it (notko, arvo) is the right place for the primitives.

The trouble is not in any one design. It is that the project has no settled canon, and has been running for seven months as though it had one. `mockspace.toml` declares no `canon_paths`, so by the workspace's own rule the designs stand in for canon and nothing in the repository is ratified. Meanwhile four documents each behave as the top tier: the 2480-line consolidation spec (itself marked `Status: IN PROGRESS` at line 4), a file that declares itself `Status: CANON` on op's 6 June ruling, a twelve-file seed that calls itself "the effective engine canon as of 2026-07-19" with `frozen = false`, and 797 rows on an unlanded branch, every one of them `provisional`. They contradict each other on points that decide the engine's shape, and the contradictions have been resolved repeatedly by agents, recorded as settled, and then overturned by op when he happened to read them. That is the finding this audit would put first: the governance, not the engineering.

The engineering, judged part by part, comes out better than the summaries in the repository suggest. The plan chain, the const-gated dispatch, the value-function direction of the api revision, the trunk model and the contracts layer are all sound in intent and mostly sound in design. Four things are going wrong. Three run entries exist where a ratified amendment says there is one engine path. Fusion is explicit opt-in on a linear chain, which cannot reach the headline it is supposed to carry. Parallelism caps at two walkers by design (head and tail) while op's stated region is millions of records, and what ships instead is an N-way split that one round deleted as drift. And a raw-pointer aliasing hole in the worker is known, confirmed and catalogued as an ignored test rather than fixed, with a note saying that blessing the shipped shape would bless undefined behaviour.

One thing is back to the drawing board, and it is not a design: it is the instruments. The suite cannot fail in the places that matter (twelve tests discard the build outcome, the incremental path asserts counts rather than values, there is no compile-fail fixture for the headline guarantee), the bench manifest holds 29 families of which none exercises the engine, the engine's own cost evidence is a timing test outside the harness, and the asm gate that five design decisions rest on has no committed output anywhere in the tree. Nothing downstream of an instrument can be trusted more than the instrument, and here that means the plan chain's nine steps are unfalsifiable: there is no oracle that says what the right plan for a given graph is, so no step can be shown wrong.

On performance, op's own instruction governs: the engine-versus-hand numbers on record are a baseline that predates fusing and adaptive crunching, and no call about the engine may rest on them. This audit draws none. What it does say is that the measured gap (2.2x to 4.6x of a fused std loop in June, single core) is the gap you would predict from the shape the engine actually runs, which materialises every intermediate through memory, and that the shape the design calls for would close most of it. An ad-hoc spike written for this audit puts an order of magnitude on that: storing intermediates as columns costs 2.2x to 3.0x of a fused loop once the data leaves cache, and the same morsel-wise structure with the intermediates in a reused scratch buffer costs 0.64x to 1.27x. The design's own answer, co-located per-fiber arenas, is that scratch buffer. So the design is pointed at the right thing; it is the building that has not happened.

Finally, the scope. The founding consumers were a 200-record tool, a 10K to 100K batch, and a 100K to 10M pipeline. Op has since said the target is millions of records crunching constantly, and that kaski is a general game engine in the sense that Unreal, Unity and Godot are. Several load-bearing decisions were made for the old scope and have not been revisited: one record count for every column in a schedule, columns reserved once and never resized, a memory provider that cannot grow, abort as the only failure policy, and a worker panic that hangs the frame. None of these is wrong for a batch pipeline. All of them are wrong for a game or a daemon, and all of them are fixable in place if they are decided now rather than discovered later.

What I would do first, in order: ratify a canon and put the four competing documents under it; build the oracle and the law suite so the plan chain becomes falsifiable; fix the aliasing hole; decide the record-count question (op has already answered it in words, "contiguous per prototype and indices to process needed windows"); and only then go back to the performance work, with a bench that runs the engine in the region it is for.

## How to read this

Every part of the stack this audit looked at carries one of four marks, as asked.

Good as it is. The intent is right, the design derives from it, and nothing in the way it is built works against it.

Easy to get wrong but can be right. The direction holds, and there is a specific thing that must be true for it to come out right. The section says what that thing is.

Going wrong. Something concrete is wrong now: a design that does not follow from its intent, a deviation from a ratified decision with no record, or a mechanism that cannot do what it is asked to do. The section says where.

Back to the drawing board. The part cannot be repaired by fixing what is there, and the section says why.

A claim about what a design says carries a path and a line, and a claim about what the code does says so as a fact about the code. The repository declares no canon (`mockspace.toml`, no `canon_paths` key), so by the workspace rule the designs stand in for one and are read as less absolutely binding than a canon would be; where a document calls itself canon, this audit says that it does so rather than treating it as settled. Numbers produced for this audit are ad-hoc spike numbers and are called that every time they appear. Numbers from the repository name the committed result file where one exists and say so where none does.

## The concept and the direction

Mark: good as it is.

The concept is that a program is a set of work units over columnar stores, each declaring what it reads and what it writes, and that everything else (the order, the grouping into independent trunks, the window size, the dispatch) is derived from those declarations before the program runs. That is a real position, it is not an ECS (`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:99` says so in those words), and it is close to what the vectorised-execution literature settled on for analytical engines, with one difference that matters: those engines interpret a plan at runtime and this one monomorphises it.

Three things make the direction right rather than merely defensible. The access sets are the single source of truth, so the schedule, the aliasing facts and the parallel decomposition all come from one declaration instead of three. The engine owns no data model, so columns stay the consumer's shape. And the whole thing is `no_std` with no allocator, which forces every structure to be sized before the program runs, which is the same discipline the compile-time schedule needs anyway.

The direction also carries three intrinsic risks, none of which is a reason to change it, all of which need naming.

It rests on unstable compiler features that may not arrive. `generic_const_exprs` and `marker_trait_attr` are both load-bearing (engine `crates/hilavitkutin/src/lib.rs:24,:29`; api `crates/hilavitkutin-api/src/lib.rs:13-16`), and the api design says of the second that it is "unstable with no stabilisation track" and that the fallback is sketched and not built (`mock/crates/hilavitkutin-api/DESIGN.md.tmpl:896-910`). The api revision round notes that the engine enables a feature "which the workspace forbids" (`mock/design_rounds/202611130617-h6mb_topic.revise-the-api-in-place.md:105` on `work/api-revision`). This is a real dependency on something nobody in the project controls.

Monomorphisation is the dispatch, so the compile-time cost and the error quality are part of the product. What is measured is 25 work units over 13 stores at depth 5 compiling in 0.53 seconds (`mock/crates/hilavitkutin-api/DESIGN.md.tmpl:539-542`). A game engine's schedule is one or two orders of magnitude larger than that, and the shipped grouping already pins `GATE2_MAX_UNITS` at 256 as a fixed constant because a capacity-sized bound "overflows the trait solver" (`crates/hilavitkutin/src/plan/grouping.rs:38-48`).

Static composition is in tension with what the downstream consumers do. The engine forbids runtime registration, correctly; but a game spawns and despawns, a daemon takes work it did not know about at build time, and the record-count and growth decisions below follow from that tension rather than from anything about columns.

## Governance: there is no canon, and four documents behave as one

Mark: back to the drawing board.

This is the one place where the answer is not to repair what is there. The repository declares no canon, and four separate documents are written and cited as though they were the top tier.

The consolidation spec (`design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, 2480 lines) is the founding document and is cited as the authority in rounds through July. Its own header says `Status: IN PROGRESS` (`:4`).

A canon amendment (`mock/research/202606061000_canon-amendment-unified-engine.md`) says `Status: CANON` at `:3`, records op's 6 June ruling that there is no separate single-core path and that the same code paths run at one, two or seven cores (`:7,:9`), forbids the cons-list walk as a separate single-core dispatch shape (`:25`), and says it cannot be folded into the locked spec and so stands as canon until it can (`:34`). Today `dev` ships three run entries, `run`, `run_fused` and `run_parallel`, and they differ in behaviour: `run` applies the incremental skip (`crates/hilavitkutin/src/scheduler/run.rs:250-254`) and `run_parallel` does not, with a comment calling itself the "pure-RAW path (no incremental skip)" (`crates/hilavitkutin/src/scheduler/worker.rs:320-322`). No deviation ledger entry records this. It is a deviation from the most explicitly ratified decision in the record.

The seed (`mock/research/seed/`, twelve files) calls itself "the flat, self-contained expression of hilavitkutin's effective engine canon as of 2026-07-19" (`governance.md:3-9`) while its manifest carries `frozen = false` and says the freeze awaits op's rulings (`MANIFEST.toml:4,:10-11`).

The registry branch (`work/canon-registry`) holds 797 tenet rows and 58 question rows. Every tenet row is `provisional` and none is ratified. It is 26 commits ahead of `dev` and unlanded.

These four disagree on things that decide the shape. One engine path against three. Decomposed columns ("every aggregate struct-of-arrays-decomposes to scalar columns", `mock/PRINCIPLES.md.tmpl:119-126`) against the resource blob that a bench chose and that the engine design records as never decomposed (`mock/crates/hilavitkutin/DESIGN.md.tmpl:509-520`). An inventory pattern recommended (`mock/PRINCIPLES.md.tmpl:78-79,:140-145`) against linker-magic registration banned in every crate (`.claude/CLAUDE.md`). "There is no std tier" (`mock/PRINCIPLES.md.tmpl:198-202`) against three tiers including std (`consolidation:275-283`). The contradictions are catalogued in `DESIGNS.md`.

The second half of this is that agent-written resolutions have repeatedly been recorded as settled and then overturned. A round was abandoned as drift after it had produced a topic and two changelists (`mock/research/202606071400_gate2-phase-axis-course-correction.md:7-9`). A 3.5x parallel win stood in the record until op noticed the baseline was single-threaded std and it became parity (`mock/research/202606080900_gate2-n-perf-finding.md:7`, `202606081100:30-36`). A wake-policy default was set from a bench whose margins op then called "too convenient", and the re-examination retired those margins as point estimates while the default stayed (`mock/research/202607210100_wake-policy-reexamination.md:3-4,:62-64`). An N-way record split was declared drift against the consolidation and deleted, with the round recording an authority ruling that "neither standalone document blanket-supersedes the other" (`mock/design_rounds/202607192110/202607192100_topic.n-way-record-split-is-not-convergence.md:35-39`), and the N-way split is what `dev` runs today (`crates/hilavitkutin/src/scheduler/worker.rs:323-348`).

None of that is a failure of any individual round. It is what happens when there is no ratified tier and the documents below it are allowed to supersede each other by argument. The repair is to ratify a canon, which the registry branch is already built to hold, and then to apply it where it hurts: enumerate what it forbids, find the standing claims that assert those forbidden things, and fix the class.

## The marks, gathered

| Part | Mark |
| --- | --- |
| Concept and direction | good as it is |
| Governance and the canon tier | back to the drawing board |
| Contracts layer, the api as a surface | easy to get wrong but can be right |
| The api revision, direction | good as it is |
| The api revision, two-writer refusal | going wrong |
| Plan chain, steps 1 to 13 | easy to get wrong but can be right |
| Write-write ordering rule in the DAG | going wrong |
| Const-gated dispatch | easy to get wrong but can be right |
| Three run entries against the unified-engine amendment | going wrong |
| Fusion as explicit opt-in on a linear chain | going wrong |
| Trunk model (column-disjoint, one per core) | good as it is |
| Head-and-tail as the only record-range parallelism | going wrong |
| The N-way split that ships instead | going wrong |
| Worker raw-pointer aliasing | going wrong |
| Failure policy (abort, worker panic hangs) | back to the drawing board |
| Storage: one record count, no growth | going wrong |
| Accumulators | easy to get wrong but can be right |
| Adaptation | easy to get wrong but can be right |
| Incremental skip | easy to get wrong but can be right |
| Resource model (blob, erased addressing) | good as it is |
| hilavitkutin-sym, -str, -ctx | good as it is |
| hilavitkutin-linking | going wrong |
| hilavitkutin-persistence | easy to get wrong but can be right |
| hilavitkutin-providers | easy to get wrong but can be right |
| hilavitkutin-build | back to the drawing board |
| Test suite as an instrument | back to the drawing board |
| Bench harness as an instrument | easy to get wrong but can be right |
| The bench record as evidence | going wrong |
| Fit to kaski | going wrong |
| Fit to generic applications and daemons | going wrong |

The reasoning for each is in `DESIGNS.md`, `BENCH-RECORD.md` and `DOWNSTREAM.md`.

## Tickets filed

Every problem this audit met in its own work, and every workflow idea, was filed or bumped rather than written as prose. The ids are listed in `LEDGER.md` under "Tickets", with the repository each is about.

## What each pass changed

Pass 1, 13 November. The first complete reading: all seven files written from the digests, the first-hand reads and the ad-hoc spike. Two claims from slice readers were opened before being relayed, and one of them (`RTLD_LOCAL = 4` in the unix loader) was confirmed against the glibc header in this container and became a finding of its own.
