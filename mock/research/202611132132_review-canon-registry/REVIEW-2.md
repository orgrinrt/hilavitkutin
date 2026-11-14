# Second review: pack `work/provisional-stamp-lint` (6eddac7), pack `work/canon-registry` (33a9585), hilavitkutin `work/canon-registry` (71f74dbf)

This review covers the three branches after the repairs `REVIEW.md` and `SECOND-READ.md` asked for, and after the coordinator's answer of 13 November at 21:47. It was done by a fresh `pr-reviewer-senior` sub-agent on opus, dispatched by hv-canon-registry-2 on 13 November 2026 with a neutral brief that ranked op's words above the rules, the rules above the coordinator, and the branches and the author's report last. Its report is copied below with only its bold removed and its sections kept.

## Gates

Canon gate: the main reading passes. I checked it against op's words (`.data/op-responses/202611022057_overnight-autonomy-and-the-provisional-stamp.md:25`, `OP-ANSWERS-18.md:5`, which I confirmed verbatim in `.data/op-responses/202611131156_op-on-aight-tell-me-where-the-work-is-at.md`) and against `.shared/rules/a-provisional-stamp-unblocks-work-while-op-is-away.md.tmpl:12-14`.

- A stamped provisional row now binds as provisional does.
- `held` binds nothing.
- The 390 rows say `maintainer`.

The `maintainer` value, read on my own before I read the coordinator's call: op chose the stamp for these rows himself ("The rest can be on provisional stamps to begin with"). Rule line 12 lists the routes open to the coordinators and does not forbid one op directs. `SECOND-READ.md:76` offered the same value as option (a). That makes two reads that ground it in quoted text. One field is misaligned, and finding 1 covers it.

Test gate: all three suites are green with `CARGO_INCREMENTAL=0 cargo test -q --workspace`.

| Tree | Result |
|---|---|
| 6eddac7 | exit 0, 1592 passed, 10 ignored |
| 33a9585 | exit 0, 1587 passed |
| merge a8ccb3f | exit 0, 1599 passed |

The tautology the first review found is gone. I found no new tautology. One sampled law and one control that controls nothing are left (finding 5). They block, together with the code findings.

## Verdicts

- Pack `work/provisional-stamp-lint`: land after fixes (findings 3, 4, 5).
- Pack `work/canon-registry`: land after fixes (findings 2, 3, 5, 6, 7).
- hilavitkutin `work/canon-registry`: land after fixes (findings 1, 8, 9). It lands after both pack branches, because it pins the pack at `dev`.

## Findings

1. hilavitkutin `mockspace.toml:614-617` and 9 rows in `mock/registry/question/*.toml` [canon drift]. This is a second stamp mechanism, and it hides from its own lint.
   - Field text: "On an open question the maintainer was not asked: the stamp it stays open on".
   - Rule `a-provisional-stamp-unblocks-work-while-op-is-away.md.tmpl:13` says: "Every stamp goes to op as a question ticket".
   - The 9 stamps name no ticket and have no `stamped_by`. None of the 9 is in `ROWS-FOR-OP.md`.
   - The lint never sees them, because `mockspace.toml:326-327` scopes it to `namespaces = "tenet"`.
   - Reproduction: in a scratch clone pinned to a8ccb3f, set `namespaces = "tenet, question"` and run `cargo mock --lint-only`. It exits 1 with 9 findings `[a-provisional-stamp-is-not-a-ratification/half] ... question::what_the_accessor_shape_... carries stamp and no stamped_by`.
   - The field came in at 2e57b81d. It is the very "second spelling" the lint's own doc (`a_provisional_stamp_is_not_a_ratification.rs:36-37`) says is a second mechanism.
   - Fix: these rows get `stamped_by = "maintainer"` and ticket `86aa80598714` (or a ticket of their own), the question kind declares `stamped_by`, and the lint reads `tenet, question`. If no work leans on these rows, the field goes instead.
2. `crates/tools/canon/src/verbs.rs:255-270` [correctness]: the coordinator's decision 1 is not met. Decision 1 says "`check` notes it with its ticket and exits 0". Measured on rv-hv pinned to a8ccb3f, `cargo mock canon check loimu_keeps_its_seven_access_modes_...` prints "is provisional, stamped by maintainer, not ratified: a stamp is as good as ..." and names no ticket. Fix: read the ticket out of `stamp` with the same parse the lint uses (`names_a_ticket`, lint `:117-124`), extracted once into `registry-lints/src/standing.rs` rather than copied. Assert the id in `check_passes_a_stamped_row_and_notes_its_stamp`.
3. Lint `:184-195`, `registry/namespaces.toml:84` and hilavitkutin `mockspace.toml:475` [spec, one shape]. A stamp on a row that is not provisional is refused only at `ratified`.
   - hilavitkutin says held is "carrying no stamp" and calls `stamped_by` "On a provisional row".
   - The pack encodes "held and stamped" as a legal state in its tests (`retired_tests.rs:456`, `a_retired_word_stays_out_tests.rs:307`).
   - A stamp approves the work built on it (rule `:12`). A held row binds nothing. One row cannot be both.
   - Reproduction: in a scratch clone, put a full `maintainer` stamp on held `the_arity_twelve_cap_...` (units-2.toml:335). `cargo mock --lint-only` reports "schema check passed", "all lints passed", exit 0.
   - The data is clean today, but nothing keeps it clean. One of the 390 is already stamped and superseded (`plan-2.toml:379`), so the report's "390 ... bind as provisional does" is 389.
   - Fix: refuse a stamp on any standing but the provisional one, and on a superseded row. Test each standing as its own case. Agree the shape with arvo-registry.
4. Lint `:3-8`, `:157-183` [spec]. The lint's doc and the pack schema say a `maintainer` stamp has "his passage cited in `stamp`", but nothing checks it. The planted stamp `stamped, ticket `000000000000`` on a `maintainer` row passed every gate (same scratch run as finding 3). This route claims op's own words, so it is the one that most needs a mechanical check. Fix: a `maintainer` stamp must cite a passage, at least a path and line. Have `quote-check` verify any quoted words against the captures.
5. [tests, sampled law] The law "no route reads as ratified" is asserted over 6 of its 9 cells (three `stamped_by` values against the clean, `on-a-ratified-row` and `ratified-still-stamped` cases).
   - Lint `tests.rs:41-50`, `81-88`, `90-108`, `166-181`.
   - `experts` is never shown passing clean, `coordinators` is never shown refused on a ratified row, and `every_finding_blocks` (`:157-164`) samples one kind of four.
   - The lint reads `stamped_by` only for presence (`:152`). These tests exist to catch a future value-dependent branch, and a sample cannot do that job.
   - Canon `standing_tests.rs:324-326` is labelled "THE CONTROL", but `subjects` counts ratified from `standing` and never reads `stamped_by`, so it cannot fail on the route.
   - Fix: loop every value over every case, and replace that control with one that can fail.
6. `standing.rs:56`, `:74` with `verbs.rs:277-296` [correctness]. Failing closed for the lints means failing open for `check`.
   - In a scratch clone, `standing = "Held"` gives `canon check` "`deviations_one_and_six_...` is unstated, not ratified", exit 0. A misspelt held row passes the dispatch gate. The schema catches it only at commit (`"Held" is not one of [...]`).
   - Fix: `check` refuses `Unstated` with a finding, `binds()` stays true for the word lints, and a test covers both directions.
7. `rows.rs:141-142`, `standing.rs:18`, lint `:38-39` [drift]. The two field names are spelled three times. Fix: `rows.rs` uses `STAMP_FIELDS`. The lint is always on and `registry-lints` sits behind `workflow`, so either one moves or the constants go to the crate both reach.
8. hilavitkutin commit bodies narrate how the work was produced, on a public repository (`public-surface.md:30-34`).
   - `c865900d`: "They lived in a scratch directory only, and the review of 791c837 asks...".
   - `b0436c3c`: "One reviewer over the two pack branches...".
   - `fcef2c4f`, `dc8c0fcb`, `e2250ebe`: "drafted from the changes three readers listed ... checked ... by a second reader".
   - Similar shapes in `7697b52c` and `66094918`.
   - The pack commits are clean.
   - Fix: reword before landing (`git rebase -r` with reword on this unlanded branch), or let the coordinator decide that cost against the rule.
9. [naming] First doc lines over 80 characters: lint `:1` (99) and `standing.rs` module summary (95).

## Prior findings at these shas

| # | Status | Evidence |
|---|---|---|
| 1 | resolved | keys are read by constant (`:89-95`), suites green |
| 2 | resolved | the test is deleted |
| 3 | resolved | `:159` asserts non-empty |
| 4 | resolved | `check` exits 0 on the stamped row and 1 on the held row, measured. Its "with its ticket" is still open (finding 2). |
| 5 | resolved | 390 rows are `maintainer`, none `experts` |
| 6 | resolved for the lints | `check` remains (finding 6) |
| 7 | resolved | `grep -c "every provisional row" mockspace.toml` gives 0 |
| 8 | stated | the report states it |
| 9 | ticketed | ticket `8542eadef864` named, not verified |
| Line-0 locations | still open | lint `:126-130`, ticket `2f3df983fb86` |

## The report's claims, checked

These match: 17 subjects, 846 tenets, 729 binding, 340 ratified; the `check` outputs; `contradicts` clean; `quote-check` 846 and 62 clean; `--lint-only` exits 0 with the stamp lint `error, set here`; all 390 stamps cite `OP-ANSWERS-18.md:5` and `86aa80598714`. The 106 held rows are a subset of `ROWS-FOR-OP.md`, whose 141 entries are 106 held, 25 questions and 10 superseded. Full `cargo mock` on a fresh clone pinned to the merge leaves only the pin line changed, so the generated docs are current. No ignore, allow, severity or exemption was added. Every subject has the right shape except f254a3dd, which the coordinator ruled on. No trailers or bylines.

## Not run

- The tickets' existence: `ticket` has no read verb.
- `cargo mock` on the `dev` pin, since the pack has not landed.
- The author's `maintainers` plant. My `Held` plant shows the schema enforcing values.
- arvo's 44 rows, which are outside the set.
- `cargo fmt --check` drift: it already fails on `origin/dev` (4 diffs in one file), and the branches add more. It is not gated.

rv-hv's line 309 is restored and `git status --porcelain` is empty. The scratch clones are deleted.
