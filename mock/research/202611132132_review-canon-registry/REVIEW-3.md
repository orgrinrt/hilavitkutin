# Third review: pack `work/canon-registry` (89c1dde) and hilavitkutin `work/canon-registry` (42bf1918)

This is the third and last review round over the registry branches, taken as one collection: the pack branch with arvo-registry's `work/provisional-stamp-lint` merged in at `08eb89b`, and the hilavitkutin branch that pins the pack at `dev` and lands after it. It also reviews, as published text, the twelve replacement commit messages in the author's report `/home/user/report-hv-canon-registry-2.md` and the replay script the coordinator is to run with them. It was done by a fresh `pr-reviewer-senior` sub-agent on opus with a brief ranking op's words first, the workspace rules second, the coordinator's calls third and the branches, the earlier reviews and the report last.

Everything below was measured in detached worktrees at the two shas, both removed afterwards; neither clone's branch or working tree was moved, and both read clean at `89c1dde` and `42bf1918` when I left. Plants in the worktrees were restored and checked with `git status --porcelain` before removal.

The short form: both branches are close. The pack needs its README brought up to the lint it ships and two small fail-open holes in the stamp lint closed. hilavitkutin carries nine stamped rows that its own stamp text says a later word of op's changes, which op's words forbid, and the replay as written would rewrite a commit that is already on `dev` and strip every signature. Each of these is named below with its reproduction and its fix.

## Gates

Canon gate: passes for the work as dispatched. The coordinator's three calls of 21:47 trace to op's words: a stamp "still approves the work depending on it" and is "as good as mine" (`.data/op-responses/202611022057_overnight-autonomy-and-the-provisional-stamp.md:25`), so a stamped provisional row binds (call 1); "The rest can be on provisional stamps to begin with" (`mock/research/202611130850_the-record-read-whole/OP-ANSWERS-18.md:5`) is op directing the stamp himself (call 2); call 3 decides nothing op's words speak to. The replay is licensed by "I allow the rewrite or replay of the bad commit messages as long as content integrity remains" (`.data/op-responses/202611132122_op-on-i-allow-the-rewrite-or-replay-of-the.md:22`). One part of the branch content is misaligned with op's words and is finding 1; it is a defect in what lands, not in the question I was asked, so the review proceeds. I am the first expert on finding 1's reading, and a second independent read is owed before it is settled either way.

Test gate, pack at `89c1dde`: `CARGO_INCREMENTAL=0 cargo test -q --workspace` exits 0, 1615 passed, 0 failed, 10 ignored (88 result lines summed). The branch adds no `#[ignore]`. I read every test body the branch adds or changes: the stamp lint's `tests.rs`, `crates/stamp/src/tests.rs`, `standing_tests.rs`, `retired_tests.rs`, `heirs_tests.rs` and the new case in `a_retired_word_stays_out_tests.rs`. No tautology; every control named THE CONTROL can fail on the property it controls. The two holes in findings 3 and 4 are implementation gaps, each with the red test that shows it; the tests present assert what they claim.

Test gate, hilavitkutin at `42bf1918` with the pack pinned by path: `CARGO_INCREMENTAL=0 cargo mock test` runs 497 tests in the main tree, 497 passed, 18 skipped. The benches tree fails 11 of 148 crates: `frame_wide` because its `checks_gate` test wants release arms built first, and `resource_storage/axis_ce` and the nine `single-trunk-convergence` crates on compile errors (E0308, E0061). The branch's only non-comment changes to Rust or bench files are one FIXME id in `mock/benches/frame_wide/bench.toml` and one message string in `mock/lints/vocabulary_discipline.rs` (`git diff origin/dev...42bf1918 -- '*.rs' 'mock/benches/*'`, comment lines excluded), so those reds are `dev`'s by construction. I did not re-run them on `dev` to measure it.

## Verdicts

- Pack `work/canon-registry` at `89c1dde`: land after fixes 2, 3 and 4.
- hilavitkutin `work/canon-registry`, as replayed: land after fixes 1, 5, 6 and 7, after the pack. `42bf1918` itself does not land, as the report says.

## Findings

1. hilavitkutin, nine rows [canon drift]: a stamp sits on rows that a later word of op's changes, and they bind against ratified rows. Each stamp says so in its own text: "its source is his founding text, and a later passage that is his changes the same point", then "stands on a stamp under the words of op 'The rest can be on provisional stamps'". Op's words exclude exactly this class. "Only those that are clearly not backed by me or actively against the canon otherwise, you bring to me. The rest can be on provisional stamps" (`OP-ANSWERS-18.md:5`): a row his later passage changes is not "the rest". And on 12 November: "stamping ratifications ornprovisionals that are clearly contradicting canon which is never allowed" (`.data/op-responses/202611121134_op-on-the-very-reason-we-have-the-mutations-and.md:24`). The rows, with the row holding his later word:
   - `memory.toml:21` `the_platform_has_three_tiers_no_os_os_and_std` against ratified `platform_tiers_are_os_and_no_os_only_and_the_std_tier_is_deferred_indefinitely`, whose source is op's "Consumer supplies it" (`OP-ANSWERS-0918.md:32`).
   - `memory.toml:36` `the_os_and_std_tiers_are_mutually_exclusive_...`, against the same ratified row.
   - `memory.toml:97` `each_platform_tier_ships_defaults_...`, against `the_os_tier_creates_no_threads_...`.
   - `memory.toml:201` `the_fiber_dispatcher_snapshots_resources_to_a_stack_local_array_..._and_writes_mutable_ones_back_after`, against ratified `a_seq_or_map_value_is_a_pointer_and_a_length_and_its_elements_are_live_streamed_from_their_column_and_never_copied`. This is the copying op's 12 November words were about.
   - `governance.toml:47` `the_consolidation_is_the_single_source_of_truth_for_the_current_design`, against ratified `the_unified_engine_amendment_is_canon_..._wins_over_intermediate_artifacts`.
   - `stores-2.toml:33`, `stores.toml:233`, `stores.toml:257`.
   - `execution-2.toml:481` `at_run_time_the_engine_spawns_threads_that_run_their_compiled_programs_...`, against `the_pool_is_the_consumer_s_executor_...` and this repository's own rule "No `thread::spawn`".
   - Reproduction: `cargo mock canon check the_platform_has_three_tiers_no_os_os_and_std platform_tiers_are_os_and_no_os_only_and_the_std_tier_is_deferred_indefinitely at_run_time_the_engine_spawns_threads_that_run_their_compiled_programs_with_no_scheduler_loop_or_work_queues` exits 0 and passes all three. A brief may build on the row that says the engine spawns threads.
   - The branch already did the right thing for seven such rows (`6637c2d0`, "mark seven listed rows superseded by the row holding his word"), and the author's own stamp text classes these nine the same way, then stamps them. `SECOND-READ.md` concluded "the 390 rows are not in the class held back from work" without opening these nine.
   - Fix: each of the nine either takes `superseded_by` naming the row that holds his word, as `6637c2d0` did, or goes to `held` and into `ROWS-FOR-OP.md` with the held ticket. The class is found by `grep -rn "a later passage that is his changes" mock/registry/tenet`, and the count goes in the commit. The two heirs that are themselves provisional (`stores.toml:233`, `:257` point at `the_round_level_amendments_registered_by_a1_are_five_dated_rounds`) contradict their own stamp text, which says the later passage is his; those two go to `held`.
   - Handed back, not decided: four stamped rows in `downstream.toml` (`ikiuni_s_design_applies_every_change_...` and three more) say of themselves "a reference from ikiuni and no part of the record of hilavitkutin". Whether a description of a downstream project belongs in this canon at all, and whether op's direction over "the entire research and design" of this repository reaches it, is not settled by his words. It is the coordinator's call, made from the canon.

2. Pack `README.md:32` [public-surface, blocker]: the README describes the stamp lint as of its first commit. It says it refuses "half a stamp, a stamp naming no question ticket, and a stamp left on a row that is ratified, with `namespaces`, `tiered`, `tier_field`, `ratified` and `ratifies_field`". The lint at `89c1dde` refuses seven kinds (`half`, `no-ticket`, `no-passage`, `on-a-ratified-row`, `not-stampable`, `on-a-superseded-row`, `ratified-still-stamped`) and reads eight keys (`stampable`, `supersedes_field`, `superseded_by_field` are missing). The README also says nothing of `held`, of the new crate, or of `canon check` refusing an unstated standing. Fix: rewrite that sentence for the lint as shipped and add the `held` standing where the README speaks of the canon tool.

3. Pack lint `a_provisional_stamp_is_not_a_ratification.rs:201-237` [correctness]: the lint never checks that `stamped_by` is one of `stamp::ROUTES`, and `no-passage` fires only on the exact string `maintainer` (`:228`). A route spelled any other way is a whole stamp that cites nothing. This is the fail-open the coordinator's call 3 closed for standings. Both consumers' schemas enforce the values today, so it is latent there, but the lint ships to any registry, and its defaults (`ruling`, `proposal`, `rung`) are arvo's. Reproduction, a test added in a scratch worktree:
   ```
   for by in ["Maintainer", "op", "maintainers", "the maintainer"] {
       let f = run(&[("proposal::p", &[("stamped_by", by), ("stamp", "Stamped, ticket `000000000000`.")])]);
       assert!(!f.is_empty(), "{by:?} passed with no finding");
   }
   ```
   fails with `"Maintainer" passed with no finding`. Fix: a finding kind `unknown-route` naming `ROUTES` when `by` is not among them, the test above kept as its case, and the kind added to `every_finding_blocks`.

4. Pack lint `:146-156` with hilavitkutin [correctness, REVIEW-2 finding 4 half met]: `cites_a_passage` checks the shape of a `<path>:<line>` and never that the file exists or holds that line. The maintainer route is the one that claims op's own word, and a fabricated citation passes every gate. Reproduction 1, in the pack: a maintainer stamp citing `` `no/such/FILE.md:99999` `` gives no finding (`rv3_plant_a_passage_in_no_file` failed: "a passage in a file that does not exist passed"). Reproduction 2, in hilavitkutin pinned by path to the pack: replacing the passage at `memory.toml:21` with `` `mock/research/no-such-dir/NO-SUCH-FILE.md:99999` `` gives `schema check passed`, `all lints passed`, exit 0. Fix: the lint is a `RepoLint` with the repository root in hand, so `no-passage` also fires when no cited path exists or the line is past its end, with a test for each. Checking the quoted words stays with ticket `check_the_words_a_maintainer_stamp_quotes_against_his_captur`.

5. The replay script in the report, lines 172 to 191 [correctness, content integrity]: as written it must not be run.
   - The range `4d31368e..42bf1918` holds 78 commits, not 77. The extra one is `e99af156`, the tip of `origin/dev`, a GitHub-signed commit. The script rewrites it to a new sha, and the last merge then merges that copy rather than `dev`.
   - `git commit-tree` does not read `commit.gpgSign`, so every rewritten commit loses op's SSH signature. All 77 branch commits carry one today.
   - Reproduction, in a `--shared` scratch clone, with no message changed: the script's tip is `69f53efa`, not `42bf1918`; `git merge-base --is-ancestor e99af156 69f53efa` says no; 78 commits differ; the first rewritten commit differs from `c1c6e9ab` only in its missing `gpgsig` header.
   - The check the report names, tree equality per pair, passes on that output, so it cannot catch either defect.
   - Fix, measured: pass `origin/dev` (`e99af156`) as the base and add `-S` to `git commit-tree`. With both changes and no message changed, the replay reproduces `42bf1918` exactly: the same sha, 77 commits, the `dev` tip an ancestor, signed. That identity run is the control to make before the real one. After the real one, check that `origin/dev` is an ancestor of the new tip and that `git rev-list --count origin/dev..<new>` is 77, beside the tree check.

6. hilavitkutin commit text [public-surface, blocker]: what the replay leaves.
   - `1f004d58` is not in the twelve and its subject is "docs: judge the ratified rows a review found against a later passage", the leakage shape the public-surface reference names. Suggested: "docs: judge the ratified rows against a later passage on their point".
   - `510088e1`, not in the twelve: "checked by filling every quote again from its source" narrates how the change was verified, the same shape the replacements for `dc8c0fcb`, `e2250ebe` and `fcef2c4f` took out. Suggested: drop that clause.
   - Replacement for `a038f18e`: "keep the tracked task op answered goes on 2026-09-02" does not parse. From `ROWS-FOR-OP.md:200`, the meaning is "keep the tracked task that op's answer of 2026-09-02 removes".
   - Replacement for `de923d6b`: "the four rows quoting lines a citation comment was added to pin the commit they were read at" has no verb for its subject. Suggested: "the four rows whose quoted lines later gained a citation comment pin the commit they were read at".
   - The other ten replacements read correctly, and I checked their claims against their trees: 33 tenets and 4 questions (`e2250ebe`), 49 and 2 (`dc8c0fcb`), 44 and 4 (`fcef2c4f`), four scripts (`c865900d`), 394 stamped and 102 held (`de923d6b`), one question row (`19306d5d`).
   - Every other kept message, read one at a time across all 77 commits and 14 pack commits: no byline, trailer, tool advert or `Co-Authored-By`, and author and committer are op on every commit.

7. Clause-dev `.shared/scripts/life/tickets-hilavitkutin.toml:1437-1450` [drift, outside both branches]: the two question tickets every stamp and held row name still describe a state that no longer exists. The stamp ticket's `what` reads "the 394 stamped tenets" and its note "(stamped_by experts) ... They stay provisional and bind nothing until op ratifies them". The rows say 389, `maintainer`, binding as provisional. The held ticket says 102; there are 106. The bump lines below correct the counts, but the body op reads first contradicts coordinator call 1. The rule asks that the ticket say "what was stamped, why" (`a-provisional-stamp-unblocks-work-while-op-is-away.md.tmpl:13`). Fix: rewrite `what` and `note` for the shape that lands. The keys stay as they are, since the rows cite them.

8. Pack `standing.rs:42` and `registry/namespaces.toml:77` [tests, drift]: the four standing words are spelled twice, in code and in the schema, and nothing holds them equal. `crates/stamp/src/tests.rs:31` does exactly this for the routes. Fix: the same test for `Standing::WORDS`. Not blocking.

9. `crates/stamp/src/lib.rs:40` [naming]: the doc says "an empty or spaced one is not", while `tests.rs:12` asserts that `` ` spaced ` `` reads as `spaced`. Say that padding is trimmed and inner space refuses. Not blocking.

10. hilavitkutin commit text [naming, not blocking]: "op" as a name stands in ten commits: `2e57b81d`, `63745c15`, `dcf06663`, `d15e306b`, `e28c3cf3`, `1747155a`, `b0e759ad`, `71f74dbf`, and `a038f18e` and `de923d6b` in their replacement text too. `1747155a` also names `.data/op-responses/`, a workspace path. The rest of the branch says "the maintainer" or "his". Since the replay rewrites messages anyway, folding these in costs nothing now and cannot be done later.

## REVIEW-2's findings at these shas

| # | Status | Evidence |
|---|---|---|
| 1 | resolved | 0 `stamp` or `stamped_by` lines under `mock/registry/question`, and no `stamp` field on the question kind |
| 2 | resolved | `canon check the_platform_has_three_tiers_no_os_os_and_std` prints "asked in ticket `make_formal_or_overturn_the_394_stamped_tenets_of_hilavitkut`", exit 0 |
| 3 | resolved | planted whole maintainer stamp on held `replace_value_is_the_supported_every_frame_swap_...` (`change-2.toml:18`): `not-stampable`, exit 1; superseded from either end covered by tests at lint `tests.rs:183-277` |
| 4 | half | shape checked (`no-passage`); a passage in no file passes (finding 4); quoted words ticketed |
| 5 | resolved | every route case loops `stamp::ROUTES`; the canon control (`waiting_as("held")`) fails if the stamp decides binding |
| 6 | resolved | `check_blocks_on_a_missing_or_misspelt_standing_...` asserts both refusals, and that the word lints still read both as binding |
| 7 | resolved | fields and routes in `crates/stamp`, read by the lint, `rows.rs` and `standing.rs`; the standing words remain twice (finding 8) |
| 8 | open | the replay carries it; findings 5 and 6 |
| 9 | resolved | module summaries are 76 and 79 characters of text |
| Line 0 | open, ticketed | lint `:159` still reports `registry:0`; ticket `registry_lints_locate_every_finding_at_registry_line_0` |

## The report's claims, checked

These match: 17 subjects, 846 tenets, 729 binding, 340 ratified (`canon subjects`); 389 provisional and stamped by `maintainer`, 106 held, 340 ratified, 11 provisional and superseded with no stamp (counted from the rows); `canon check` naming the ticket on a stamped row (exit 0) and refusing a held row (exit 1); `quote-check` over 846 tenets and 62 questions with nothing to report; `canon contradicts` clean; `--lint-only` exit 0 with the stamp lint listed `error, set here`; the held-row plant refused as `not-stampable`; the pack suite green; no ignore, allow, severity, ratchet or exemption added. Full `cargo mock` on the branch pinned by path leaves only the pin line changed, so the generated documents are current. Not accurate: the replay "keeping every tree" is true of trees and false of the dev ancestry and the signatures (finding 5). The round tag `round/202611132039-i9wf/end` exists locally and not on the remote, as the report says.

## Not run

- The `frame_wide` bench gate: it needs `cargo build --release --workspace` of the bench member first, about seven gigabytes of disk were free, and the branch changes two comment lines there.
- The bench reds on `origin/dev`: argued from the diff (two non-comment lines, neither in those crates), not measured.
- The replay with the twelve messages: only the identity replay, both as written and corrected, which is what decides findings 5's two defects.
- arvo-registry's own registry against finding 3: it is not cloned here, and its schema decides whether the hole is live there.
- `cargo fmt --check`: not gated, and red on `dev` already per `REVIEW-2.md`.
