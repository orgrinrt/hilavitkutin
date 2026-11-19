# hila-records-and-red: the two sketch records, the bench charts and the ui fixtures

This job had three parts: find what deletes the committed bench charts when the two sketch branches land, land those branches' records, and make dev green on `hilavitkutin::ui compile_fail_fixtures`. The charts turned out not to be deleted by any landing. Both records are merged on `work/hila-records-and-red`, and the ui fixtures were already green on dev.

## The bench charts

Ticket `land_tool_generate_only_step_deletes_the_bench_charts_under` (clause-dev commit `ceeeb00ab946`) says that landing the two sketch branches runs a generate-only step which removes the committed charts under `docs/`. It does not reproduce. Both branches were cut at `2b137e9d`, before dev's `07a9e9a3` (bench: add the build and link bench) added the 14 `BENCHES.build-static.*` and `BENCHES.link-static.*` charts. So a two-dot diff of dev against either branch lists those 14 charts as deleted. A merge does not delete them.

Measured on 19 Nov against hilavitkutin dev `739b02e6`:

- `cargo mock --generate-only` on dev, on a full worktree and on a seat built like the landing seat (sparse by `land.seat_sparse`, no LFS smudge, `data-refs-up` run) changed nothing.
- A no-commit merge of each branch, then generate-only, then the commit with its hook, deleted nothing.
- The real `land_local` from `land-core.sh`, run against a local bare copy of the repository, landed both branches, and `docs/` on the result is identical to `739b02e6`.

The latent case is real, though, and it belongs to mockspace. `bench_docs::generate` returns no paths when it finds no run under `mock/benches/results/`, and `sweep_retired` then removes every committed `BENCHES.*` chart. A tree that has the charts but lacks the results would lose them. That is ticket `idea_bench_docgen_with_no_runs_keeps_the_committed_charts_in`.

Guard arm: clause-dev `land-core_test.sh`, `a_generated_chart_the_trunk_gained_after_the_branch_was_cut_is_kept_by_its_landing`, on clause-dev branch `work/hila-records-and-red` at `2c221fc4`. It is green as written and red with `LAND_TEST_CHART_GENERATOR` set to a generator that removes the chart (the landing is refused and dev keeps its old tip).

## The records

`origin/work/sketch-accumulator-grouped-by-data` (`aa049495`) and `origin/work/sketch-derived-context-block-form` (`66a385d2`) are merged into the job branch as `b96724eb` and `cc9f1878`. Both are record-only (research only, per clause-dev `.shared/research/202611191130_stack-unmerged-branches/README.md`). After both merges, `git diff origin/dev HEAD -- docs` is empty.

## The ui fixtures

Ticket `hilavitkutin_dev_two_ui_fixtures_expect_stderr_three_lines_b`: `red-check hilavitkutin compile_fail_fixtures` on `739b02e6` prints green. The `.stderr` files expect lines 92 and 74, which matches the sources, and were last re-blessed by `252c5190`. Closing the ticket is the coordinator's step, with that sha.

## Test runs

- `land-core_test.sh`, new arm: 1 passed; with the mutation, 1 failed as it should.
- `red-check ... hilavitkutin compile_fail_fixtures`: green on the job branch while it still stood at dev's `739b02e6`, so no separate dev run was needed. The reviewer ran it again on the branch tip, green.

## Tickets filed or bumped

- `land_tool_generate_only_step_deletes_the_bench_charts_under`: bumped with the finding.
- `idea_bench_docgen_with_no_runs_keeps_the_committed_charts_in` (mockspace).
- `bootstrap_counts_the_cargo_mock_step_passed_though_its_binst` (clause-dev).
- `a_helper_s_git_push_to_a_local_bare_repository_is_refused_by` (clause-dev).
- `hilavitkutin_accum_per_gives_worker_count_dependent_results` (hilavitkutin), the shipped-engine failure the accumulator sketch records.
- `idea_sketches_md_and_cargo_mock_sketches_take_a_split_outcom` (mockspace).

## Review

One `pr-reviewer-senior` pass over the branch at `b2ef4af5`: merge after fixes. Its one blocker was a wrong sha for the charts (`dea557ae` touches nothing under `docs/`), fixed above together with its two minor points. Its follow-up is the two tickets just listed.

## Canon

This job changes no source and no design, so `canon contradicts` was not run. The two records came in as their authors wrote them.
