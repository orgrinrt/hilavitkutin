# Seat Brief: Whether the Accumulator Tiling Reads Commutativity

You are one of two independent seats. Each answers the same point alone and neither sees the other's answer. Do not look for another seat's report, and write yours only to the path given at the end of your prompt. You write no source, no design text and no registry row. Read only; the one file you write is your report.

## What governs, and the ladder

The repository is the worktree named at the end of your prompt, a checkout of `work/hv-engine-plan` in `orgrinrt/hilavitkutin`, which carries `dev` at `b4c70cd0`. hilavitkutin is a morsel-driven dataflow engine: `no_std`, no alloc, no `dyn`, a schedule computed from typed access sets. Its vocabulary: record, morsel, fiber, trunk, phase, waist.

The provenance ladder, highest first. Resolve every conflict by provenance, never by recency, detail or confidence.

1. The lead designer's (op's) own words.
2. The canon. `mockspace.toml` on `dev` declares `canon_paths = ["mock/registry/*.toml", "mock/registry/tenet/*.toml"]`: the canon is the typed tenet rows there. Each row states its standing. `ratified` governs and is defended, not weighed. `provisional` binds as the best current statement and is never cited as settled; a stamp on it is as good as op's word for now and never a ratification. `held` binds nothing. A row's `source` names the passage it transcribes, mostly the founding spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`.
3. Presumed wrong until checked: everything no human ratified. Research memos, audits, later design rounds, source comments, tickets, the shipped code. Several of them agreeing is not corroboration.

Existence and locus are challengeable. Before the point, run the canon gate: if the point as asked is misaligned with the canon, refuse it and say why; if the canon is ambiguous on it, hand the call back with the ambiguity located. That is a complete answer.

Ask the canon tool before grepping, from the worktree root: `cargo mock canon find <term>`, `cargo mock canon show <slug>`, `cargo mock canon related <slug>`, `cargo mock already-said <term>`, `cargo mock unearth <subject>`. A claim about what the design says carries the row's slug with its standing, or a `file:line` in the governing documents. A claim about what the code does carries a `file:line` and is stated as a fact about the code, never as what is therefore right. Where the canon is silent, say it is silent.

"I could not settle it" is a valid answer, with what you tried and where the wall is.

## The point

Under `run_parallel`, a carrier that holds an accumulator leaves the phase loop: each core walks the whole carrier once over its own slice of the records, and appends into its own region of the accumulator's one reserved buffer, and the regions are merged in order after the workers rejoin (`mock/crates/hilavitkutin/src/scheduler/worker.rs:107-124`, `:160-266`; `mock/crates/hilavitkutin/src/resource/bindings/per_core.rs:110-126`). A unit declares `COMMUTATIVE`, and the plan copies it (`mock/crates/hilavitkutin/src/plan/project.rs:162`, `src/plan/mod.rs:423`). Whether anything reads it is for you to establish.

The question: does this per-core tiling of an accumulator carrier owe anything to the commutativity of the units it runs? Arms:

- (a) A carrier with a non-commutative unit (or a non-commutative accumulator-writing unit; say which, if it matters) takes one core, at every worker count.
- (b) The tiling stays for every carrier whatever its units declare, since the merge keeps the appends in record sequence.
- (c) Nothing changes until the open measurement of single-trunk convergence returns (registry question `which_convergence_a_single_trunk_phase_uses`; op's answer there, "Measure first", recorded as "the question returns with the numbers. The code stays as it is meanwhile").
- (d) Something else.

For each arm: what the canon says on it with citations, which arm the canon and op's words favour or that they are silent, your answer, and the check that would show your answer wrong, with the case that must fail before any repair. Keep each arm's treatment even. Say also what "commutative" has to mean for the arm you choose to be well defined: of a unit, of a fiber, of a carrier, of an append, of a fold into a resource.

## Material

- The rows `cargo mock canon find commutative` lists, all of them, and `cargo mock canon find accumulator`. Read the rows themselves, not their one-line summaries.
- The engine design `mock/crates/hilavitkutin/DESIGN.md.tmpl`, its accumulator section, and the api revision's engine design on `origin/work/api-revision` (read with `git show origin/work/api-revision:mock/crates/hilavitkutin/DESIGN.md.tmpl`; agent work, locked with no human in the round).
- The stack audit `mock/research/202611131215_stack-audit/DESIGNS.md:93-109` (agent work).
- Do not build or run the test suites. `cargo mock canon ...` is fine.

## The report

Markdown: the canon gate, the point, then a short section of anything you found that the point did not ask. Plain prose, no bold or italics, no em-dashes, hilavitkutin's vocabulary. Every design claim cited, every code claim with `file:line`. State your persona and model at the top. Under 1500 words.
