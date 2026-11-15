# The Slices Handed To Readers

Sibling of `SLICE-BRIEF.md`. Each numbered section is what one slice reader was asked. The three slices dispatched before this file existed (the canon in two halves, and the re-test of the 13 November audit) are described in `LEDGER.md`.

## Slice 3: op's intents

Limit: 60 tool calls, 2800 words. Stay in your working directory; one simple command per Bash call.

Op is the owner; his words are the governing tier. Sources:

(a) Captures under `/home/user/.data/op-responses/`. List with exactly `git -C /home/user ls-files .data/op-responses` (about 1330 files; the output is saved to a file you can grep for `2026111`). Read whole, with the Read tool, every capture from 202611121741 to 202611140724 whose name mentions hilavitkutin, arvo, notko, engine, canon, bench, convergence, buffering, pillars, extension, contracts, lattice, general-engine, simulation, width, split, crates, zig, research or upstream (about 45 short files), plus `202611131716_op-on-okay-so-while-we-wait-any-commentary-or.md`, `202611081138_the-engine-sees-numbers-and-folds-them.md` and `202611021120_op-arvo-is-kaski-s-only-numeric-substrate.md`.

(b) `msg kaski doc briefs/fixup/op-answers-N.md` for N from 1 to 30 (a missing one prints a fatal line). Read whole those touching hilavitkutin, arvo or notko.

(c) `mock/research/202611130850_the-record-read-whole/OP-ANSWERS*.md`, whole.

If (a) or (b) is refused, do the rest and say so.

Report, each item with op's verbatim short quote, its date and path:line:

1. His goals for hilavitkutin: what it is for, the scale region (record counts, how constantly), the performance ambition and against what, what kaski is and needs, other consumers he assumes, what must never be done; what he says of benches (baseline, measure first), fusing, convergence and head and tail, N-way splits, double buffering and swap, the pillars, the extension model and C ABI, failure and abort, the canon's standing and how rows are ratified or provisional, the api revision, load-time units, contracts and typestate, and the place of arvo and notko.
2. Rulings of his that overturned what agents had recorded.
3. Statements of his that pull against each other or against his own goals, both quotes, unsmoothed.
4. Questions put to him that the record shows unanswered.

## Slice 4: the root documents and the crate designs

Limit: 55 tool calls, 2800 words.

Read `mock/PRINCIPLES.md.tmpl`, `mock/DESIGN.md.tmpl`, `mock/WORKFLOW.md.tmpl`, `README.md`, `MIRI.md`, `mock/catalogue.md`, `docs/HILAVITKUTIN.md` (`docs/` is generated from the templates; confirm by comparing heads and read the templates as the source), and every `mock/crates/*/DESIGN.md.tmpl` (15 crates). Read whole the engine design `mock/crates/hilavitkutin/DESIGN.md.tmpl` and the api design `mock/crates/hilavitkutin-api/DESIGN.md.tmpl`; the others whole where under 400 lines, else the first 250 lines and the headings. Use at most ten canon queries to test the most important design statements against ratified rows.

Report:

1. Per document: what it sets out in five lines or fewer, its status line and date if any.
2. For the engine and api designs, fuller: the execution model (phases, trunks, fibers, morsels, fusion, convergence, parallelism, pool), the plan chain steps, dispatch, the run entries and what the design holds them to, storage (columns, resources, virtuals, accumulators, buffered columns, record counts, growth, memory provider), change tracking and incremental skip, adaptation, failure and panic policy, the unit and access-set contracts and how an undeclared access is refused, the unstable features relied on, and every stated performance number with where it is said to come from.
3. Contradictions inside a document and between documents, with both locations. The 13 November audit claimed: decomposed scalar columns in PRINCIPLES against a resource blob in the engine design; an inventory pattern recommended in PRINCIPLES against a ban on linker-magic registration; "there is no std tier" against three tiers; the build crate's design describing an API and scripts not in the tree. Say for each whether it holds at this commit, with lines.
4. Design statements that disagree with a ratified canon row, with the row id.
5. Parts plainly stale against the source tree (name the file showing it) and parts marked as not built.
6. Per crate, one paragraph: what the design says the crate is for and whether it reads as a real design, a sketch or a stub.

## Slice 5: the design rounds and the research since 12 November

Limit: 70 tool calls, 3000 words.

Half one. `ls mock/design_rounds` lists about 330 rounds by timestamp. Establish from `mock/WORKFLOW.md.tmpl` and a few round directories how an open round is told from a closed one, and count closed, abandoned and open. Then read the rounds from `202611122301` to `202611140752` (about 50): the topic file whole where under 200 lines, else its first 120 lines. For each: the topic in one line, what it decided or changed, whether it is closed, whether its changelists say they were applied. Group by theme.

Half two. Read whole: `mock/research/202611131930_triage/TRIAGE.md`; the SEAT files beside `mock/research/202611140100_engine-path-plan/PLAN.md` for where a seat disagreed (the auditor has read PLAN.md itself); the directories `202611131951_load-time-units`, `202611140512_sound-memory-provider`, `202611140543_panic-aborts-on-a-worker`, `202611140606_parked-worker-aliasing`, `202611140613_review-jobs-one-to-three`, `202611132308_review-api-revision-2`, `202611140036_review-api-revision-3`, `202611132042_review-hv-leaf`, `202611132132_review-canon-registry`; and the files `202611132004_review-api-revision-1.md`, `202611130728_data-bound-registration-findings.md`, `202611132020_buffered-pass1-reading-a.md`; also `202611130850_the-record-read-whole/RECOVERED.md`.

Report:

1. Round counts and the per-round table for 12 to 14 November, by theme.
2. Per research document: what it claims, what it recommends, its status (acted on, superseded, open). For the triage: each item and its disposition.
3. Per review document: the findings raised, which were fixed, which stand open.
4. What the night of 13 to 14 November landed and what each piece says of its own remaining holes: the api revision (did it land on dev, and what did it change: value-form units, buffered columns, the two-writer rule, reach, the conformance crate), the skip differential, the memory provider, unit abort, the asm gate.
5. Contradictions between these documents, and claims that look unsupported by what they cite.

## Slice 6: the microkernel benches

Limit: 80 tool calls, 3200 words. Run no bench.

Start with `mock/benches/README.md`, `mock/benches/bench.toml`, `mock/benches/NIGHT_FINDINGS_2026-05-11.md`, `docs/BENCHES.md`, `checks.sh`, `stage.sh`, `bench-arm`, `bench-stage`, `variant-path`, `codegen_dispatch_axis_a_assertions.md`. The families are those with `<family>_n<size>.csv`, `.meta.json` and `_findings.md` directly in `mock/benches` (35 of them, from `atomic_inc_strategy` to `shift_amount_origin`, including the `rsb_*` five, `registration_binding` and `dep_graph_csr_9`). Sources are under `mock/benches/variants`.

Also test the earlier audit's `mock/research/202611131215_stack-audit/BENCH-RECORD.md`: it says four microkernel families do not measure what their label says and that the night synthesis draws rules from them. Find which four and check each against source and numbers.

Per family, compactly:

1. What it says it measures, the arms, the sizes, the timing method (iterations, warmup, statistic, black_box), and the machine facts of one meta.json.
2. Headline numbers from the committed csv or findings for the smallest and largest size, with file names.
3. Whether the source measures what the label says. Look for: arms the optimiser makes identical, work hoisted or folded out of the timed loop, a missing black_box, arms differing in more than the named variable, sizes too small to leave cache or defeat the predictor, timer resolution against measured duration, noise larger than the claimed difference, single runs presented as findings.
4. The mechanism you believe explains the numbers (your reading), and where you cannot tell.
5. What the record drew from it: grep NIGHT_FINDINGS, `docs/BENCHES.md`, `mock/PRINCIPLES.md.tmpl` and the crate designs for the family name; report each rule or decision citing it and whether the numbers carry it.

Then: the machine or machines everything ran on (aarch64 Apple silicon against x86_64, cores), whether families are comparable, and families with results but no source or source but no results.

## Slice 7: the engine-level benches and the instruments

Limit: 85 tool calls, 3500 words. Run no bench.

Read: `mock/benches/engine_vs_std` (FINDINGS.md whole, bench.toml, src, variants, results, run-checks.sh); `mock/benches/frame_wide` (bench.toml, src, variants, tests, both findings files, csv heads, occupancy.sh and .txt, vectorisation.sh and .txt); `mock/benches/resource_storage` with `mock/research/202606210600_storage-bench-findings.md` (note `aggregate.py` exists although the project bans python); `mock/benches/single-trunk-convergence`; `mock/benches/results` (INDEX.md and each of fiber_theory, rcm_order, rcm_rivals, swap_band, swap_column, swap_cost, wake_policy) with the matching sections of `docs/BENCHES.md`, and find each family's source or say it is missing; `mock/benches/runs`; `mock/benches/asm_gate_fixtures` (asm_gate_report.md whole), `mock/benches/src`, `mock/benches/support`; the notes `mock/research/202611131250_bench-harness-at-a81e195.md`, `202611131300_bench-matrix-for-the-intended-region.md`, `202611131400_frame-wide-first-run.md`, `202611131900_buffered-column-swap-cost.md`, `202611132130_asm-gate-over-the-engine.md`, the three `review-hv-bench-instruments` directories, and `202606080900_gate2-n-perf-finding.md`, `202606081100_gate2-parallel-bench-fairness.md`, `202607202340_wake-policy-evidence.md`, `202607210100_wake-policy-reexamination.md`, `202607200600_a1-1-rcm-order-bench-resolution.md`, `202607201100_swap-semantics-ratification-evidence.md`; and the earlier audit's `BENCH-RECORD.md`, `BENCH-CATALOGUE.md` and `ESTIMATES.md` as claims to test.

Per bench:

1. The scenario, what each arm really runs (which engine entry, which baseline, how many threads), the sizes, the timing method, the machine (architecture, cores).
2. Headline numbers with the committed file and line.
3. Whether it measures what it claims. Look for: a baseline not like for like, work the optimiser removed, arms differing in more than the named variable, sizes inside cache when the claim is about memory-bound work, thread spawn inside the timed region, single runs, a region the engine is not for (the intended region is millions of records crunched constantly).
4. The mechanism you believe explains each headline number (your reading): memory bandwidth, materialised intermediates, dispatch overhead, vectorisation, cache occupancy, wake latency, false sharing. Say where you cannot tell.
5. What decision the record drew from it and whether the numbers carry it.

For the instruments: what each checks, what the committed asm gate report shows for the engine, what the three instrument reviews found and whether each finding was fixed.

For the earlier audit: each bench BENCH-CATALOGUE.md proposed and whether it has since been written; each estimate in ESTIMATES.md with its assumptions; claims in BENCH-RECORD.md now wrong or outdated.

## Slice 8: the sketches

Limit: 110 tool calls, 3800 words. Build nothing.

`mock/research/sketches` holds about 114 entries. For every sketch, one to three lines: the question, what it built or ran, what it found (any number with the file and line it is committed at), whether output is committed or only narrated, and whether the finding supports what it is said to support. Open the source where the finding is a performance number, an assembly claim or a compiles-or-not claim.

More depth, with a read of the source, for: the gate2 family (2026060703 to 2026060803); the d1, d4, c5, c6, e4, e5, e6, e7, e8 and f2 families of 6 June; `202606202100_a2a-per-fiber-morsel-loop`, `202607190900_a4-parallel-per-fiber-windows`, `202607201300_arena-poolframe`; and everything from November 2026 (`20261112*`, `20261113*`, `20261114*`). For each of these add: the timing method or assembly check used, sizes, machine, the mechanism you believe explains the numbers (your reading), and whether the sketch links the real hilavitkutin crates or is a stand-alone model. A stand-alone model shows the idea can be expressed, not that the engine does it; say which each time.

Then two lists: sketches whose finding was later contradicted (grep `mock/research` for the sketch name where you suspect it), and design claims resting on a sketch with no committed output.

## Slice 9: the source and tests

Limit: 100 tool calls, 3800 words.

The code is known to be incomplete; report structure and mechanism as evidence of whether the design can be built, not style. Another reader has already established the state of the run entries, the skip, the aliasing hole, the panic boundary and the test counts; do not repeat that. Cover what it did not read:

1. The plan chain as built (`mock/crates/hilavitkutin/src/plan`, every step file): what each step computes, whether at compile time (const or type level), at build, or per frame, its fixed limits, and where it leans on arvo crates (`arvo-graph`, `arvo-sparse`, `arvo-spectral`, `arvo-comb`).
2. Dispatch as built (`src/dispatch`, `trunk_dispatch.rs`, `wu_fn.rs`, `engine_ctx`): how a unit body is reached, what is monomorphised, where an indirect call or a runtime mask test sits in the per-morsel and per-record path, how a column pointer reaches the unit body, and what the unit author writes (read the api's `work_unit.rs`, `context.rs` and one engine test that defines units).
3. Storage as built (`src/resource`, `src/column` or wherever columns live, `hilavitkutin-api/src/storage.rs`, `hilavitkutin-providers`): column layout, the arena, buffered columns and the swap, accumulators and per-core regions, alignment and false-sharing precautions.
4. `src/thread` (barrier, frame, pool frame): how workers park and wake, the atomics and their orderings.
5. The api crate's contract surface: the WorkUnit trait, access sets, how an undeclared access is refused, store kinds, how arvo and notko types appear in signatures, and the feature gates.
6. Each other crate briefly (`-api-macros`, `-build`, `-conformance`, `-ctx`, `-extensions`, `-extensions-macros`, `-frozen-authors`, `-kit`, `-linking`, `-persistence`, `-providers`, `-str`, `-sym`): what it implements, how complete against its design's headings, test count.
7. From the `mock` directory run `cargo test -p hilavitkutin` once (nightly is pinned; it may take several minutes; if a guard refuses the build, say so and move on) and report the pass, fail and ignored counts verbatim, or the first error verbatim.
8. Count `unsafe` blocks per crate by grep and say, for those in `src/scheduler`, `src/thread` and `src/resource`, whether each carries a safety argument.

## Slice 10: downstream, and the stack below

Limit: 90 tool calls, 3500 words. Build nothing. Read only, in every repository. Stay in your working directory; reach the other clones with the Read tool by absolute path (`/home/user/kaski`, `/home/user/arvo`, `/home/user/notko`) and with `git -C <clone> ls-files <dir>` where that is allowed.

Part one, kaski as a consumer. Find how kaski uses or plans to use hilavitkutin, arvo and notko: its Cargo manifests, design documents and principles, its canon (`cargo mock canon find hilavitkutin` inside `/home/user/kaski` if allowed, else grep its registry), `/home/user/.shared/state/kaski.md`, and source that calls the engine. Report what kaski is (scope, simulation model, tick budget, record counts, determinism and fixed point, platforms including wasm), which parts are meant to run on hilavitkutin and which do, what it says it lacks from the engine, and workarounds it carries. Quote the documents.

Part two, arvo and notko. For each: what it provides, its maturity in its own words, the unstable features it needs, what landed in the night of 13 to 14 November (round directories stamped 20261113 and 20261114, and the audit at `mock/research/202611131215_stack-audit` in arvo, in ten lines). Then from hilavitkutin's side: `mock/Cargo.toml` pins arvo at rev `1b2796e92ad2b8176c2177d72a9cffa161b7b92e` and notko at branch dev with a patch section; say whether the pin is behind arvo's dev if you can tell, count uses of `arvo::` and `notko::` per crate under `mock/crates/*/src` (`grep -rc`), read a few hot-path sites, and report workarounds for something arvo or notko lacks (grep FIXME near arvo or notko).

Part three, the fit, as evidence and not verdict. For each need below say what hilavitkutin's canon or designs answer and what is absent (at most fifteen canon queries from the audited tree; read `cargo mock canon subject downstream` whole): (a) a game engine: fixed tick with a hard frame budget, records created and destroyed, several populations of different counts, events, deterministic replay, wasm and consoles, fixed point in hot paths, save and load; (b) a generic application: small data, start-up cost, ease of use, compile time, error reporting; (c) a server daemon: long uptime, work unknown at build time, data growth, back-pressure, a panic that must not take the process, observability, graceful shutdown, async I/O at the edges.
