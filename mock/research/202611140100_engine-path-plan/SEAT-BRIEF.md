# Seat Brief: Five Open Points on Hilavitkutin's Engine Path

You are one of two independent seats. Each answers the same five points alone and neither sees the other's answer. Do not look for another seat's report, and write yours only to the path given at the end of your prompt. You write no source, no design text and no registry row. Read only; the one file you write is your report.

## What governs, and the ladder

The repository is `/home/user/hilavitkutin` (branch `work/hv-engine-plan`, the trunk `dev` at `e99af156`). hilavitkutin is a morsel-driven dataflow engine: `no_std`, no alloc, no `dyn`, a schedule computed from typed access sets. Its vocabulary: record, morsel, fiber, trunk, phase, waist.

The provenance ladder, highest first. Resolve every conflict by provenance, never by recency, detail or confidence.

1. The lead designer's (op's) own words. Those that bear on these points are quoted below, verbatim.
2. The governing canon. `dev` declares no `canon_paths`, so there the designs stand in for canon: the founding spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, the amendment `mock/research/202606061000_canon-amendment-unified-engine.md` (its own header: canon on op's ruling of 2026-06-06), and the engine design `mock/crates/hilavitkutin/DESIGN.md.tmpl`. A canon registry exists on the unlanded branch `origin/work/canon-registry`; a checkout of it is at `/home/user/.worktrees/hv-canon`, where `cargo mock canon find <term>`, `cargo mock canon show <slug>`, `cargo mock canon related <slug>` and `cargo mock already-said <term>` work. Each row states its standing: `ratified` (by default from the written history, on op's word) or `provisional` (an agent's stamp op has not blessed). A ratified row governs and is defended, not weighed. A provisional row is a claim.
3. Presumed wrong until checked: everything no human ratified. Research memos, audits, later design rounds, source comments, tickets, shipped code. Several of them agreeing is not corroboration.

Existence and locus are challengeable: you may find that a thing these points take for granted should not exist, or belongs somewhere else, and say so. The shipped tree is claims to test, not context. Before the points, run the canon gate: if a point as asked is misaligned with the governing canon, refuse it and say why; if the canon is ambiguous on it, hand the call back with the ambiguity located. That is a complete answer.

Ask the canon tool before grepping, and cite: a claim about what the design says carries the row's slug with its standing, or a `file:line` in the governing documents; a claim about what the code does carries a `file:line` and is stated as a fact about the code, never as what is therefore right. Where the canon is silent, say it is silent.

"I could not settle it" is a valid answer for any point, with what you tried and where the wall is.

## Material

- The founding spec, the amendment and the engine design named above.
- The stack audit: `mock/research/202611131215_stack-audit/AUDIT.md`, `DESIGNS.md`, `DOWNSTREAM.md` (agent work).
- The engine source: `mock/crates/hilavitkutin/src/scheduler/` (`run.rs`, `run_parallel.rs`, `run_fused.rs`, `worker.rs`, `dirty.rs`), `src/thread/frame.rs`, `src/plan/`, `src/dispatch/`; the executor contract in `mock/crates/hilavitkutin-api/src/platform.rs`.
- The api revision, which lands before the build these points feed: branch `origin/work/api-revision` (tip `d45093ea`). Read with `git show origin/work/api-revision:<path>`; its engine design is `mock/crates/hilavitkutin/DESIGN.md.tmpl` there, and its locked changelist `mock/design_rounds/202611130617-h6mb/202611130617-h6mb_changelist.doc.lock.md`. Agent work, locked with no human in the round.
- The swap spec `mock/research/202607200200_replaceable-swap-semantics-spec.md` (draft, landed provisional), the deviation ledger `mock/research/202606072100_gate2-canonical-deviation-ledger.md`, the deviation 5 audit `mock/research/202607201600_gate2-deviation-5-audit.md`.
- Benches: `mock/benches/single-trunk-convergence/FINDINGS.md` and the `frame_wide` bench findings under `mock/benches/`.
- Do not build or run the test suites: the disk has about 3 GB free. `cargo mock canon ...` in the registry worktree is fine.

## Op's words that bear on the points, verbatim

- On the failure policy, asked whether to redesign it [1: redesign it now | 2: after the api revision | 3: abort stays, fix hang only]: "Abort stays, fix hang only".
- On which convergence a single-trunk phase uses [head-and-tail, finish the deletion | N-way, retire head-and-tail | both, chosen at plan time | measure first]: "Measure first", recorded as: a bench under the harness with both arms on single-trunk workloads; the question returns with the numbers; the code stays as it is meanwhile.
- "hilavitkutin is intended for the cases that have millions of simulation and crunching work data going constantly. So 1-4 morsels is almost never a realistic use case for hilavitkutin in the first place"
- On the engine's benches so far: "no calls should be made based on those yet, they are a baseline to compare against then."
- On the resource swap (spec S1 to S7): "Land as provisional".
- On two writers of one store: "Hints, dependents, registration".

## The five points

For each: state the arms that exist (add one if the listed arms miss it), what the governing canon and op's words say on it with citations, which arm they favour or that they are silent, your answer, and the check that would show your answer wrong. Keep each arm's treatment even.

1. The shape of the one path in source. Today `Scheduler::run`, `Scheduler::run_parallel` and `Scheduler::run_fused` are three entry functions with three walks. Arms: (a) one frame body, which every entry reaches, with the calling thread running the worker body where there is one core; (b) the three walks stay separate functions and are held to one meaning by a differential against a reference runner; (c) something else. Say also what becomes of `run_fused` under your answer: an entry of its own, a strategy the one path selects, or neither.

2. A panic in a unit's body on a `run_parallel` worker leaves `frame_await_done` waiting for an arrival that never comes (`src/thread/frame.rs:88-111`). Under op's "Abort stays, fix hang only", which mechanism ends the wait. Arms, from the open question ticket: (a) the executor contract requires the consumer's spawn to catch an unwind and report the arrival with a poisoned flag the engine turns into its failure; (b) units are required to be panic-free and the stall stays documented as a contract violation; (c) the engine adds a timeout or liveness check on the done barrier through the `Clock` provider; (d) something else, for instance a guard value on the worker's own frame whose drop reports the arrival.

3. Workers hold a shared reference to the scheduler across every park (`src/scheduler/worker.rs:77-84`) while `replace_resource` and `replace_value` take `&mut self` between frames (`src/scheduler/dirty.rs:48-114`, its FIXMEs). Arms: (a) the scheduler-plane relocation the deviation ledger and the deviation 5 audit name (what workers share moves to a plane of its own that the between-frame methods do not hold exclusively); (b) a narrower repair, where a worker keeps only a raw pointer across a park and forms its reference per frame after `frame_await` returns; (c) something else. Say what Miri (or another tool) would have to run over to show the hole closed, and what case must fail before the repair.

4. Whether a terminal fan-in closes a phase. A twelve-unit frame whose occupied depths carry 5, 3, 3 and 1 units (the last a join) is planned as one phase, because waist detection (`arvo_graph::waist_detect`, called from the engine's plan) admits interior local minima only, and the bench measured the parallel entry at about 1.08 cores of four on it. Arms, from the open question ticket: (a) leave the interior-only criterion and accept one phase for such frames; (b) admit the last occupied depth as a waist candidate in arvo when it is strictly narrower than its predecessor, for every consumer; (c) keep arvo's criterion and close a phase before a terminal convergence in hilavitkutin only.

5. The incremental skip and the plan band across the entries. On `dev`, `run` and `run_fused` ask `dirty_units` and `run_parallel` does not (`src/scheduler/worker.rs:307-388`); `run` and `run_parallel` consume the plan-dirty bits and open on the plan band and `run_fused` does neither (`src/scheduler/run_fused.rs:77-86`). Say what the one path owes on each: whether every walk gates on the dirty mask, what an accumulator fiber does under it, and what the fused walk does when a plan-affecting resource was replaced.

## The report

Markdown, one section per point in the order above, then a short section of anything you found that the points did not ask. Plain prose, no bold or italics, no em-dashes, hilavitkutin's vocabulary. Every design claim cited, every code claim with `file:line`. State your persona and model at the top. Under 1500 words per point.
