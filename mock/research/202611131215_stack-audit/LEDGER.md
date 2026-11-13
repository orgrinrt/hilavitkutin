# Ledger: What Was Read, To What Depth, And What Was Not

Sibling of `AUDIT.md`. The brief asked for an honest account of the reading behind the verdicts, so that a later reader can tell a judgement made from a whole document from one made from a digest, and can see what nobody opened.

The audit ran on one branch, `work/stack-audit`, cut from `dev` at `ada51bb` on 13 November 2026. The clone was read only apart from this directory and the one spike directory.

## How the reading was organised

Three kinds of reading went into this.

First-hand reads by the auditor, listed below by file. These are the documents the verdicts rest on most heavily, and every claim drawn from one was drawn from the text rather than from a summary of it.

Slice readings by dispatched sub-agents, each given a part of the tree too large to read directly, each told to write a digest naming its own depth and its own disagreements. Six digests came back: the bench record, the engine source, the branches, op's words across the channel, the design rounds with the research notes and sketches, and the ecosystem crates. Every digest states what it read whole and what it listed by title only; those statements are carried into the sections below rather than smoothed over.

Spot checks of claims before they were relayed. Where a digest made a claim that would become a finding, the underlying file was opened. Two of those checks are worth naming because they changed what this audit says. The `RTLD_LOCAL = 4` claim in `hilavitkutin-linking` was checked against both the source and the glibc header in this container and became a finding of its own. The `dep_graph` hybrid contradiction was checked against the committed findings file and holds.

## Read whole by the auditor

Configuration and the top-level map: `mockspace.toml` (332 lines), `mock/PRINCIPLES.md.tmpl` (287), `mock/DESIGN.md.tmpl` (160), `README.md` (211).

The designs: `mock/crates/hilavitkutin/DESIGN.md.tmpl` (1442), `mock/crates/hilavitkutin-api/DESIGN.md.tmpl` (1377).

The founding spec: `design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` (2480), and the execution-model topic of the same round through its slice reader.

The seed: all twelve files of `mock/research/seed/`.

Research notes: `202606052000_single-core-engine-ideal-vs-actual-audit.md` (144), `202607181200_engine-arc-recomprehension.md` (183), `202607181300_engine-roadmap-r6.md` (541), `202607200300_pre-freeze-open-question-dossier.md` (130), `202607210300_seed-verification-redo.md` (90), `202606072100_gate2-canonical-deviation-ledger.md`, `202606061000_canon-amendment-unified-engine.md` (39), `202606070700_engine-roadmap-r4-gate2-mechanism.md` (79).

Benches: `mock/benches/engine_vs_std/FINDINGS.md` (230).

Source: `scheduler/run.rs`, `scheduler/worker.rs`, `scheduler/dirty.rs`, `plan/step1_dag.rs`, `plan/grouping.rs` (first 120 lines), `dispatch/trunk_dispatch.rs` (first 110), `dispatch/fusion.rs`, `hilavitkutin-api/src/context.rs`, `hilavitkutin-linking/src/backend/unix.rs` (first 30), the ignore catalogue and `mock/benches/bench.toml` by structure.

Unlanded branches: the api-revision topic (`work/api-revision`) whole; `single-trunk-convergence/FINDINGS.md` (`work/bench-convergence`) whole; the branch table of ahead and behind counts across sixteen branches.

Channel documents: op's answer files 1, 2, 3, 7, 11, 12, 13, 14, 15, 16, 20, 21, 24; `what-hilavitkutin-is.md` (184); `map-loop-core.md` (256); `review-api-doc-1.md` (91); `panel-consolidation-1.md` (600); parts of `roadmap.md`.

Partly read: `cr-answers.md` from `work/canon-registry` (1250 lines, the frame-start and record-count answers read, the other six headings located but not read).

## The slice digests, and what each says of its own depth

The bench record. All 82 variant sources read whole, the manifests, the hand-written synthesis, the five-check implementation, the resource-storage round, the samples of the committed findings. About 330 csv, meta and findings files listed but not opened; the reader says so, and says which conclusions therefore rest on the headline bullet of one file per family.

The engine source. The modules reachable from the run entries read whole; `adapt/`, `intrinsics/` and `strategy/` found unreachable from them. 207 engine tests across 72 files counted, 59 api tests, 14 ignores, 1024 lint-allow markers. Eight internal disagreements recorded.

The branches. Sixteen branches compared against `dev`; the api-revision, loop-core, canon-registry and recover-landing-source branches read into. The recovery branches matter to this audit and are treated as part of the record, not as noise: `work/recover-landing-source` is 26 commits ahead of `dev` and carries the sources of the six bench families whose results are committed without them, which turns "results with no source" from a finding into a landing problem. It also holds the fiber-windowed parallel run and the deletion of the N-way split, with the swap of round 202607200900 deliberately held out under a ticket.

Op's words. 834 lines of verbatims with their paths, drawn from the channel and the research notes. The quotations in this audit come from that digest with the underlying path named; where a document paraphrases op rather than quoting him, this audit says so (the 7 June correction in `mock/research/202606072330...:50-60` is an agent's rewrite, not a quoted block, and is cited as such).

The design rounds, research notes and sketches. 281 round directories, about 800 files, 75 top-level research notes, 107 sketches. About 35 files read whole, about 25 in part or by grep, the remainder listed by title. The reader flags that its absence claims (no design for several record sets, no design for a double-buffered column store) come from regular-expression greps rather than from a read of every file, and this audit carries that caveat with those two claims.

The ecosystem crates. All eleven read whole, but only `hilavitkutin-build` after the reader's own context was compacted, so the line numbers for the other ten are less verified and the reader says which. Where this audit makes a finding from that digest it either opened the file (the linking constant) or marks the claim as the reader's inference (the providers sentinel).

## What nobody opened

The 951-line standalone spec `mock/research/202606111800`. It is one of the documents the N-way round treats as an authority, and no reader on this audit read it. Any conclusion about what that spec says is therefore second-hand in this report, and the one place it matters (the authority ruling that neither standalone document blanket-supersedes the other) is quoted from the round rather than from the spec.

Thirty-four of the thirty-six files of the founding round, including its twenty research notes, read by title and heading only.

About 370 round topic files, and every changelist in the repository.

The remaining eight headings of the canon-registry answers file, and the 797 registry rows themselves beyond their count and their status.

`mock/research/hilavitkutin-build-design-audit.md` beyond its first 70 lines, and the two history files.

The resource-storage variant wrappers, so the six storage arms are known through the shared kernels and not through the crates that call them.

Everything in kaski itself. The downstream judgement in `DOWNSTREAM.md` rests on the kaski facts in the panel digest (the tick budget, the fixed-point lane) and on op's words about what kaski is, not on a reading of that repository.

## Tickets

Filed by this audit, all on 13 November.

`6147afe44835`, hilavitkutin: the linking crate passes the macOS `RTLD_LOCAL` value on linux, where 4 is `RTLD_NOLOAD`.

`ddc5948e4ab2`, hilavitkutin: three run entries ship where the unified-engine amendment says there is one path, with no ledger entry.

`7b84b07366ca`, hilavitkutin: the ungated N-way record split ships where its own round deleted it as drift.

`0da7ae8cb2b4`, hilavitkutin: four microkernel bench families do not measure what their label says, and the night synthesis draws rules from them.

`d7b3d658aa07`, hilavitkutin: the five-check asm gate has no committed output while five design decisions rest on it.

`216466ad875f`, hilavitkutin: the build crate's design describes an API and scripts that are not in the tree.

`236905118acf`, clause-dev: a sonnet sub-agent dispatch fell back to opus with no report at the call.

`dd32b5b630cc`, clause-dev: the helper bootstrap takes about 35 minutes before any work starts.

Not filed, because the registry already carries them: the empty `CoreDispatch`, the no-op `HybridExecutor::run`, `run_fused` losing accumulator appends, the worker panic stalling the frame join, the write-write ordering by registration position, the engine enabling `generic_const_exprs`, `MemoryProviderApi` being a safe trait the unsafe code relies on, the parallel skip having no differential, the thirteen catalogued tests with no bodies, the perf oracles being timing assertions, the cold store needing real file access, and the fixtures that cannot fail. Each of those is a row in `.shared/scripts/life/tickets-hilavitkutin.toml` and the report names the defect rather than restating the ticket.

## The spike

`mock/research/sketches/202611131400_chain-shapes-adhoc/` was written for this audit, committed with its source and both outputs, and its `FINDINGS.md` says in its own first paragraph that it is an ad-hoc spike and not a bench. It links no hilavitkutin crate. Every number from it in this report is called an ad-hoc spike number where it appears.
