# Review 3: `work/hilavitkutin-stranded-and-placement` (`1fe01b4d`) against `origin/dev`

Canon gate: aligned except one leftover of the same class (finding 1). Checked against: ratified `execution-3.toml` `work_stealing_is_not_the_default_and_the_consumer_may_provide_a_stealing_executor` (read with `cargo mock canon show`), the four ratified `identity-3.toml` / persistence rows this branch adds, and `OP-CALL.md:7-16,68`. On the executor I wrote down my own reading from the row's quote ("the consumer can provide a stealing Executor via the Executor trait") before I read round 202611191546. My reading is that the ratified row governs over the 202605101036 seal and the trait opens. That agrees with REVIEW-2 finding 2, so two reads now hold it. Test gate: `cargo mock test -p hilavitkutin --full --no-cache` gave 340 passed, 21 skipped, and that run includes both heavy ui tests. `-p hilavitkutin-api --full` gave 88 passed. I could not run the whole-workspace `--cheap`: the saturation guard refused it at load 6.7 on 4 cores. I read every test body the diff adds or moves.

## Verdict

MERGE AFTER FIXES. One blocker, and it needs no round.

## REVIEW-2 findings, against the tip

1. Fixed. The four ratified rows each carry one option, and their `what` and `mechanism` stay inside its text. The api-surface restatement is held and agent-written. Residue is in finding 3.
2. Fixed, by the ladder. `Executor` is open (`hybrid.rs:30`), with a pass fixture. The question ticket `may_a_consumer_write_a_hilavitkutin_executor_or_does_the_eng` is now answered by the canon and should be closed or reworded. It must not stay open against the ratified row.
3. Fixed. Three branches are on hold, with the record owed. I re-ran `record-blobs.sh` on the retired branches I checked: none of hv-suite-reads, canon-registry, hv-leaf, loop-core or overflow-is-a-value has a record blob absent from `dev`. As a control, hv-suite-words shows its one blob.
4. Fixed. `engine_seal_is_private_fail.rs` fails with E0603 "module `sealed` is private", which is the reason it names, and it would build with `pub mod sealed` because `EngineOnly` is `pub` (`lib.rs:58-64`).
5. Fixed. No `KeyValueStore` remains outside the superseded row and the record.
6. Fixed. No em-dash added anywhere in the diff.
7. Half fixed. Every id walks its own `ADDRESSABLE`. The out-of-range `from_index(ADDRESSABLE)` is still not catalogued, and no changelist says why (follow-up 5).
8. Fixed. `ceiling_div_by_zero_fail.rs` fails with E0080, divide by zero, at `RESULT`, which is the reason it names. The doc now says `N.div_ceil(D)`.
9. Half. The `**Phase:**` line is restored. `plan-3.toml:212-213` still cites the copy in closed round 202611191347. That is defensible, because the flat file moves at every `close`, but no changelist says so (follow-up 4).

## Findings

1. `mock/agent/rules/cargo.md.tmpl:34-36`, `mock/agent/MAIN.md.tmpl:26,76`, `mock/DESIGN.md.tmpl:34` [spec, blocker]: these still say persistence is "standalone" and does "not depend on ... `hilavitkutin-api`, or on each other". The branch ratifies `op_ruled_that_the_memory_provider_contract_stays_in_the_api_and_persistence_builds_on_str_and_api_as_a_dependency`, and `OP-CALL.md:68` names "standalone" as the word his ruling overturned. The crate's own `Cargo.toml:10-11` depends on both. Every agent session loads the generated rule. This is the third pass of the same failure: the ruling was applied where it closed a gap and not where it falsifies a standing claim. Fix: in all three files, say persistence builds on `hilavitkutin-str` and `hilavitkutin-api` and never on the engine or `hilavitkutin-ctx`. These are root and agent templates, so no round is needed.
2. Commit `923fe3cd` [rules, hand on]: the subject "keep op's rulings to his words" puts workspace process into public history, the same class as the `3f0145c8` body. Only 1 of the last 200 `dev` subjects does this. Add it to the coordinator's reword list.
3. `identity-3.toml` ratified rows [spec, follow-up]: two ids carry agent glosses that the quote does not hold. They are "a consumer takes notko's sink" and "the runtime data plane". `rejects = ["a second sink surface beside notko's"]` (`:61`) is also inference. Move the `rejects` to the held row and ticket the id renames. Separately, the provisional, stamped `identity-2.toml:196` row is superseded whole by a held row. That demotes the sentences op did not touch from stamp to held, which inverts the ladder. Put this to the maintainer as a question.

## Follow-ups (do not block)

1. `hybrid.rs:78-80` says "its sealed impls" and `BACKLOG.md.tmpl:559` says "sealed `Executor` trait". Both are stale after the unsealing. Fold them into the existing `Executor: Sealed` ticket for `DESIGN.md.tmpl:78`.
2. `DESIGN.md.tmpl:734` says `Scheduler<'frame, Cfg, E: Executor = HybridExecutor>`, but `scheduler/mod.rs:148` has no `E`, and nothing in `src` takes an `Executor`. So a consumer's executor can be written but cannot take the library's place, and the doc at `executor_outside_the_engine_pass.rs:1-2` overclaims. Its `takes_any_executor` is a local function, so the fixture proves the trait can be implemented and nothing more. The gap predates this branch, and the branch restates it: ticket it with a `FIXME(<id>)` at the scheduler.
3. `tests/ceiling_div.rs` does not test `N = usize::MAX`, though the rustdoc (`ceiling_div.rs:18-19`) claims no overflow there. Add the case.
4. `cargo mock close` sweeps the unrelated flat cap-lifting topic into every round. Twelve copies exist under `design_rounds/`. Ticket it on mockspace v1.
5. `engine_id_index.rs` lacks a case at index `ADDRESSABLE`. `from_index` is unchecked (`codegen.rs:68,118,169,222`), so the case either pins the wrap or catalogues the hole.
6. `pool_frame.rs` and `hybrid.rs:56` keep "substrate" in moved text, which the vocabulary rule bans. Fix it on the next edit there.

## Verified

- All six fixtures are registered in `ui.rs:244-253`, and each fails or passes for the reason it names. The two regenerated stderr files differ only in line numbers.
- The `api` README, the crate table and the `Cargo.toml` comment match the tree. The commit subjects meet the format.
- The retire, hold and wait counts in `DECISIONS.md` (13, 3, 1) add up to its table.
- Every ratified row's `what` sits inside its `quote`.
