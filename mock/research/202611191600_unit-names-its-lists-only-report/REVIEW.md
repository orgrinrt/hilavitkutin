# Review Of The Branch `work/sketch-unit-names-its-lists-only`

The branch was reviewed twice before it was handed over, each time by a fresh `pr-reviewer-senior` pass dispatched by the session that wrote it, one at a time. Each pass read the branch against `origin/dev` and returned one report. This file says what each pass found and what was done about it. The reviewer writes nothing to the tree, so the words here are the session's account of its reports.

No pass has read the branch as it stands after the second pass's fixes. The maintainer's limit is three passes and a wrap-up, and the session stopped at two: the fixes after the second are all in the written record, and what it left open is listed at the end as follow-ups.

## Pass 1, over `a4e34134`

Verdict: do not merge as it stands, on two blockers in the written record, and a merge with follow-ups once they are fixed. The reviewer could not run the tests: the saturation guard refused both tries. Its figures came from the recorded files, and it read the body of every test.

The two blockers, and what was done.

1. `REPORT.md` cited a `REVIEW.md` that was not in the tree, and said a pass had been run before it had. This file is that record, and the report now points at it without saying what it holds.
2. The account of the canon behind Q1 named four rows a value-body unit sets aside, and three more ratified rows were missing: `a_unit_registers_its_ops_through_each_batch_and_reduce_on_the_context`, `the_op_author_controls_the_inner_loop_and_the_framework_controls_the_morsel_boundaries` and `process_batch_is_the_single_entry_point`. The session opened all three, found the reviewer right, and added them.

The findings that did not block, and what was done.

3. "No call and no jump" was wrong of the emitted code, which holds the two conditional branches of each loop. The wording now says so, and records what the reviewer saw beyond it: the value body and the positional body of `Drift` are the same instructions.
4. Attempt f1 was stated too widely. A function's lists can be carried as a marker parameter of the registering trait. That was built as attempt f2 and compiles. `FINDINGS.md` states f1 for a trait with no parameter and lists a builder that takes a function under what is not established.
5. Three recorded outputs were condensed by hand. The schedule attempt, the dependency tree, the run, the one-impl run and the mutant are now the raw output of their commands, each in a file of its own.
6. Fact 9 quoted the second of two errors in its recording. It now says which, and what the first is.
7. The title claimed the form for every unit. It now names the unit whose body is its value function.
8. `Unit` had no `COMMUTATIVE`. It has one now, defaulted as on `WorkUnit` and passed through by the blanket, and Q2 cites the provisional row on the declaration blanket across schedules.
9. `EDITS.md` did not show the manifests as diffs and did not list what the providers manifest leaves out. Each manifest now has a diff, and the providers manifest has its own paragraph.
10. The problems and the two questions are prose and not tickets. That stands, for the reason the report gives, and the coordinator files them.
11. The commit type `research:` is not in the repository's written list of types and is in use on `dev`. Left as it is.

Between the two passes the sketch also changed on the coordinator's word: the read product was moved onto notko's product carrier.

## Pass 2, over `7968def7`

Verdict: do not merge as it stands, on four blockers in the written record, and a merge with follow-ups once they are fixed. This pass ran `cargo test -q -p check` itself: run 1 passed, fixtures 2 passed. It regenerated the seven diffs and found them identical to the recorded ones, and found what is said of the notko design to match that design.

The four blockers, and what was done.

1. This file said pass 2 had read the carrier port before pass 2 had run. That sentence is gone, and this section is written after the pass returned.
2. Two more ratified rows are set aside for a value-body unit and were not named: `a_unit_over_columns_receives_a_morsel_window_and_a_unit_over_resources_only_runs_once_per_record` and `execute_returns_nothing_and_a_failing_unit_aborts_because_abort_gives_better_llvm_codegen`. The session opened both and added them, so `FINDINGS.md` and Q1 name nine. Four further rows the reviewer asked to have weighed are weighed in `FINDINGS.md`, and one of them, `the_morsel_dispatch_loop_is_always_inlined`, is said not to be met as written.
3. Attempt s1 printed a second error, E0207 on the schedule parameter, that the findings did not report, and Q2 leaned on the attempt. Both errors are now stated, and Q2 says that no other route to a one-impl unit on another schedule was tried.
4. Q1 was to reach the maintainer without the conditions the form was run under. They are in the question's text now.

The findings that did not block, and what was done.

5. `EDITS.md` called the formatter's changes layout alone. It now says what else the formatter did: the position of `const` in six impls of the model, and trailing lint comments moved in the engine copy. The comment in `carrier-model/Cargo.toml` no longer says verbatim.
6. Attempt c1's source was in no commit. It is a gated module of the api copy now, and its output was made again from it.
7. The two earlier sketches are not on `dev`. `FINDINGS.md` says which branch holds them.
8. Fact 9 said such a unit does not run, which was wider than its evidence. It says the build is refused with the witness inferred, and that naming the witness by hand was not tried.
9. The consumer-spelling row was listed as being for the form without saying that its own route is a computed context. That is said now.
10. The carrier port did not cite the maintainer's condition on it. It does now, with the two things that bear on it.
11. Whether `ReadVals<'_, Self::Read>` counts as naming nothing but the lists is not settled by the canon. It is part of Q1 now, with an option of its own.
12. The headings are in title case, the count of attempts in the report is right, and the outcome line carries its scope.

## Follow-ups Left Open

Each is a ticket the coordinator owes, since the session could file none.

1. Land the two earlier sketches from `work/sketch-derived-context-block-form`, which this sketch cites.
2. A fixture for whether a downstream crate can give `Read<'f>` a form for a member that is not `Readable`, now that `ReadVals` is bounded on the product and not on `ReadSet`.
3. The marker route of attempt f2 against the schedule parameter of attempt s1, before the one-impl spelling is taken for `Always` alone.
4. A unit that offers both bodies, run with its witness named by hand.
5. A third review pass over the branch as it stands, if the coordinator wants one before it lands.
