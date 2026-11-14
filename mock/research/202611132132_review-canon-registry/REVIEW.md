# Collected review: pack `work/provisional-stamp-lint` (cbca5f0), pack `work/canon-registry` (acc10c2), hilavitkutin `work/canon-registry` (b37925cf)

The review covers two repositories: `mockspace-extra-lints` (branches 1 and 2) and `hilavitkutin` (branch 3). It is kept here because the pack has no research directory of its own. The reviewer was a `pr-reviewer-senior` sub-agent on opus, dispatched by hv-canon-registry-2 on 13 November 2026 at the coordinator's ask. Its report is copied below with only its bold and its code fences removed. The second, independent read on finding 4 that the review calls for is in `SECOND-READ.md` beside this file.

## Verdict

I refused this review at both standing gates. None of the three branches is fit to land.

- Test gate, branch 1: the pack suite is red, and the lint's own tests include a tautological one and a vacuous one.
- Canon gate, branches 2 and 3: both build on a reading of "stamped" that op's own words forbid. Branch 3's "fix" for finding 3 is that same reading.

I stopped there, as the persona requires. What follows is the evidence for the refusal and the defects I had already established on the way to it. The checks I did not run are listed at the end.

This canon call rests on my read alone. A second, independent read is owed. If the two reads disagree, op decides; a third expert does not.

## Gate 1: the test gate fails on branch 1

1. The pack suite is red at `cbca5f0`, and red again on the merge `a562eb1`. The helper's "its 12 tests green" counts only the lint's own module.
   - Command: `CARGO_TARGET_DIR=/home/user/target cargo test -q -p mockspace-extra-lints`, run in `rv-pack-stamp`. It exits 101: `no_key_a_lint_reads_from_params_goes_unchecked_or_is_the_engines --- FAILED`, at `tests/no_lint_declares_a_key_the_engine_reserves.rs:202:5`, "`params.get` arguments the scan cannot resolve to a literal (file, argument): [("a_provisional_stamp_is_not_a_ratification.rs", "key")]".
   - Cause: `src/lints/a_provisional_stamp_is_not_a_ratification.rs:97` reads `params.get(key)` with a loop variable. The pack's guard test exists to refuse exactly that.
   - Control: the same test is green on a tree at the same base `18ae1e9` (`rv-pack-canon`, 5 passed). So this branch brings the red.
   - The same failure is in `rv-pack-merge`. The helper's report calls the merge of both pack branches "measured" and green. It is not.
   - Fix: four literal `params.get(TIERED_KEY)` calls, or one helper the scan can resolve. Do not touch the guard test.
2. `tests.rs:166-168`, `it_answers_to_its_name`, is a tautology and should be deleted, not improved. It asserts `name() == NAME`, and `name()` is literally `NAME` (lint `:75-77`). It cannot fail. All it does is add one to the green count that went into the report.
3. `tests.rs:156-163`, `every_finding_blocks`, passes on no findings at all. `f.iter().all(...)` over an empty `Vec` is true, so if the lint stopped finding anything this test would stay green. Fix: assert `!f.is_empty()` first.

Smaller gaps in branch 1, owed alongside:

- The contract fixes `stamped_by` to `coordinators | experts`. The lint never checks the value and leaves it to the schema `values`. No test plants a bad value against either.
- Every finding is located at `"registry"`, line `0` (`:125`), so it names no file or row line.

## Gate 2: the canon gate fails on branches 2 and 3

4. A stamped row "binds nothing", and `canon check` exits 1 on it. Op's words say the opposite.

   The canon, with path and line:
   - `/home/user/.shared/rules/a-provisional-stamp-unblocks-work-while-op-is-away.md.tmpl:12`: "That stamp is as good as op's for now, and the work depending on it proceeds."
   - Op verbatim, same file `:22`: "a temporary stamp that ratifies things ... it's as good as mine ... still approves the work depending on it, so nothing gets blocked while I'm away."
   - `.data/op-responses/202611122006_stamps-by-separate-experts-then-start-upstream-work.md:20`: "Use those as the stamps for the latter rounds".
   - `OP-ANSWERS-18.md:5`: "The rest can be on provisional stamps to begin with". Op chose the stamp as the vehicle for exactly these 390 rows.

   What the branches do instead:
   - `crates/registry-lints/src/standing.rs:92-105` and `crates/tools/canon/src/rows.rs:53`: a stamp makes a row non-binding whatever its standing.
   - `crates/tools/canon/src/verbs.rs:286`: `check` blocks on a stamped row.
   - `crates/tools/canon/src/lib.rs:184-185`: "a job built on a row that is ... stamped ... is built on nothing, so the coordinator asks before it dispatches".
   - hilavitkutin `mockspace.toml:475`: "A provisional row that carries a stamp ... binds nothing".

   Op said a stamp approves the work built on it so that nothing blocks. This branch turns every stamp into a dispatch block. It reverses his rule, on 390 rows, in the tool every brief is checked against.

   The coordinator's own contract does not license it either:
   - The contract says "the row keeps its standing". In the pack, `provisional` binds as the best current statement, and `check` exits 0 with a note. Branch 2 lets the stamp override that standing.
   - Branch 1's module doc (`:3-9`) says "the stamp is as good as his for now". That puts the two pack branches on two different shapes of one contract, which is exactly what the contract forbids.

   The origin is the brief's call 1, "a row that is provisional, stamped or listed for op must not read as binding" (`job-hv-canon-registry-2.md:110`):
   - That is a coordinator's call with no human in its loop, and it contradicts a rule that is op's words.
   - The prior review's finding 3 asked only that the part-one rows not bind, and said "Rows that merely wait on him can stay provisional."
   - A whole seat was spent building the stricter reading into two repos. That cost was paid on nothing.

   What survives under op's words, and the second read should confirm it: `held` binding nothing for the rows brought to him. That matches "Only those that are clearly not backed by me ... you bring to me."

5. `stamped_by = "experts"` on 390 rows asserts a panel that never sat.
   - The rule (`:12`) defines a stamp by two routes: "the three coordinators converge on it, or two domain experts agree after a panel stretch".
   - These rows were put on stamps by op's 13 November instruction and filled in by reader agents. Their own stamp says so: 330 copies of "nothing read in the written history shows it to be his ... under the words of op".
   - hilavitkutin redefines the word to fit. `mockspace.toml:525`: "`experts` where readers of the record found nothing of his standing against it". The pack defines the same value as "two domain experts agreed" (`registry/namespaces.toml:84`). One value with two meanings is a second shape.
   - The contract has no value for "op directed it onto a stamp". This gap is ambiguous and the call goes back to the coordinator. Do not settle it by redefining `experts` per repo.

## Other defects found on the way, blocking once the gates clear

6. Branch 2 weakens a check, at `crates/tools/canon/src/standing_tests.rs:31`.
   - Base `18ae1e9:standing_tests.rs:24-49` caught a misspelt standing (`"Provisional"`) as `Unstated`, which binds. Branch 2 deletes that case.
   - Now any unknown word is `Declared` and binds nothing (`standing.rs:57-58`, `:76-78`). A typo fails open and leaves every check.
   - The only guard left is the schema enum. mockspace's own `mock/crates/mockspace-registry/src/model.rs:39-63` records this exact failure ("a lint keyed on the field silently ignored the value it did not recognise").
   - Fix: bind nothing only on words the project declares, not on any word outside the pack's set. Restore the misspelling control.
7. hilavitkutin `mockspace.toml:475` says "every provisional row of this registry carries one [a stamp]". The count says otherwise: 400 `provisional`, 390 `stamped_by` (`grep -h '^standing\|^stamped_by' mock/registry/tenet/*.toml | sort | uniq -c`).
8. The report says "This branch touches no file there" about `mock/crates/hilavitkutin-api/`. It does. `00c092f8` and the merge `b14757d9` rename the FIXME ids at `src/dispatch_codegen.rs:403` and `src/platform.rs:467`. That is the crate `hv-api-revision` overhauls, and the overlap is owed a name in the report.
9. `rules_out` drops `TypeId` because of a matcher defect. The reason given is that the identifier reading matches `type Id`. The report names no ticket for that defect.

## Not run, because of the gate stop

- The hilavitkutin checks on the merge pin (`--lint-only`, `canon subjects`, `canon check`, `contradicts`, both `quote-check` runs). The pin would point at a red pack (finding 1), and their meaning depends on finding 4.
- What I did run: `cargo mock` on the committed pin exits 0, and `git status --porcelain` is empty afterwards.
- Pack branch 2's own tests: 84 and 129 passed.
- Prior findings 1 and 4 to 12: not re-checked. Finding 3 is inverted into finding 4 above. Finding 13 stands as the coordinator decided: `f254a3dd` is the only failing subject in `git log 4d31368e..b37925cf`, and no commit carries a byline.

## Order of repair

1. Fix branch 1's red and delete its tautology.
2. Hand finding 4 back to the coordinator against the cited rule lines, then do the second read, with the contract amended if needed.
3. Rework the stamp reading in branch 2 and hilavitkutin together.
4. Then dispatch a fresh review.
