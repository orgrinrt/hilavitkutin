# Review 2 of work/api-revision at 21e11daf: merge after fixes

Reviewer: pr-reviewer-senior, fresh, on a worktree at `21e11dafe4ae`, over everything since review 1's head `eeb7f555`. The text below is the reviewer's report as it came back. Nothing in it is this seat's.

## Verdict

Merge after fixes.

The branch answers review 1's five findings, and every fix I mutated was caught by its new test. The blocker is in the instruments round. The `frozen-ledger` lint does not enforce the two properties its design names. Two further calls rest on readings op has not ratified, and they go back to you.

## The two gates

Design gate: passed, with two calls handed back (last section). The three rounds build what the designs and op's words ask for:

- Finding 2 has an explicit-value tier for every type and a `start` that does not exist for a type with neither constructor nor default. Its compile-fail doctest names the error code, E0599.
- The null-bit tier is not built, which the coordinator's reading allows. I checked notko at `/home/user/notko` and both cargo checkouts of it: it has no trait giving a type a null start. `NicheFilled` is sealed and means the reverse, and `MaybeNull`, `Slot` and `Maybe` reach their null through `Default` (`maybe.rs:400,654`, `slot.rs:142`). arvo has only `IsZero`-style predicates. Ticket df2e4da2d376 is cited.
- The instruments (a ledger of the edits forced on unit authors) serve op's "Mostly thinking about unit authors".

Suite audit: passed, with gaps.

| Suite | Result |
|---|---|
| `hilavitkutin-api` | green, doctests included |
| `hilavitkutin` | green (exit 0); 7 tests ignored |
| `hilavitkutin-conformance` | 13 + 14 + 9 + 3 green |
| `hilavitkutin-frozen-authors` | 18 green, 9 ignored (the catalogued disagreements) |
| `cargo mock --dir <worktree> --lint-only` | all lints pass |

- Lint unit tests (60/60) and cross-target tool tests (8/8, wasm under node included): green. They ran in `/home/user/hilavitkutin` at `d2711d87`, not in the worktree (see the process note). `git diff d2711d87 21e11daf -- mock/lints mock/tools mock/crates/hilavitkutin-frozen-authors mock/crates/hilavitkutin-conformance` is empty, so the result holds for this tree.
- Mutation 1, finding 3: at `map.rs:196` I wrote the live side only where the value changed. Both modes of `a_value_equal_to_the_frozen_one_still_lands_on_the_live_side` fail ("record 1 holds 57005 where 1 is due").
- Mutation 2, finding 2: at `column.rs:412` I wrote only the frozen side. 8 of 9 `buffered_start` tests fail, which matches the changelist's claim.
- Mutation 3, finding 1: at `worker.rs:218` I forced `if false && cut.0`. Three `reach_across_cores` tests fail, e.g. "neighbours read before their writer landed: [(5000, 15002, 15003), ...]".
- Every mutation was undone; `git status --porcelain` in the worktree is empty.
- The gap is the missing fundamental in finding 1 below: the `frozen_ledger` suite has no test of an edit whose hash was rewritten without a break.

## Review 1's findings

1. Answered. `plan/reach_cut.rs` and the segmented walk at `worker.rs:204-256`. Mutation 3 proves the new tests can fail, and the phase-loop control stays green.
2. Answered for `Buffered`, not for the class. The drain fills both sides through `MaybeUninit`, and its SAFETY comments hold: `ColumnValue: Copy`, the reservations are fresh and distinct, and a zero-length slice is sound. The 0xA5 poison provider makes the tests real. Still open:
   - A plain `Column<T>` is still read uninitialised (ticket 9b64b95e42d1).
   - The maintenance-rank compare at `map.rs:198-200` reads a plain column before its first write.
   - `run_fused` leaves its intermediate columns unwritten (catalogue row).
   - These are the same class. I did not reproduce them under Miri.
3. Answered. Mutation 1.
4. Answered. `ReachRule` takes `D`, and `buildable.rs:53` passes `DefaultPlanDims`. The builder only ever produces `DefaultPlanDims` schedulers (`builder.rs:178,285`), so the rule and the plan agree.
5. Answered. The comment at `column.rs:267-277` states the publish, await and boundary sequence, and the code matches it: `bring_in` is the one writer, and `dup_side` copies inside the pass.

## New findings

1. `mock/lints/frozen_ledger.rs:286-343`, correctness and tests (high).
   - The design says the lint fails "when a hash changed without a break recorded for it, and when a locked file changed at all" (`hilavitkutin-frozen-authors/DESIGN.md.tmpl:114-117`).
   - The lint only checks one snapshot for self-consistency. Edit a frozen file, rewrite its `hash` in `LEDGER.toml`, add no break: clean. The same edit of a `locked = true` file with `locked_breaks` equal to `breaks`: also clean.
   - Reproduced with probe A below, which passes, so the hole is real.
   - Holds for every frozen file. No line is locked today (`grep -c "locked = true"` gives 0), so the locked case is latent.
   - Fix: compare against the trunk through `view.git`, and check that the last `BREAKS.toml` `hash` (which the lint does not read today) equals the ledger hash. Add both cases as tests.
2. `mock/lints/frozen_no_feature_gate.rs:54-74` with `frozen/scan.rs:37-48`, correctness (low).
   - The check reads one line at a time. `#![` on one line and `feature(never_type)]` on the next passes, though rustc accepts it.
   - Reproduced with probe B. `frozen_no_recursion_limit` uses the same per-line scan; not probed.
3. `hilavitkutin/src/scheduler/worker.rs:210-228`, spec drift (low).
   - `reach_cuts`, `consumer_mask` and `group_n` are called at runtime on every pass on every core.
   - The design (`hilavitkutin/DESIGN.md.tmpl:1390-1391`) says the cuts fold "at compile time".
   - Fix: wrap them in `const { ... }`.
4. Public surface (blocks under the checklist).
   - `hilavitkutin-conformance` is publishable (no `publish = false`) but has no README or `readme` field of its own, and the root `README.md` never names it.
   - `README.md:199` lists `generic_const_exprs`, a feature the workspace forbids, and `hilavitkutin/src/lib.rs:22` enables it. That dates from `8adc7285`, before this branch, but it is the forbidden feature and is on the public page.
   - Generated `docs/HILAVITKUTIN.md:1647` ("kept as a failing test with its ticket") mentions the workflow in a public document.
5. Commit `57dc2c99`, style. "Merge branch 'tmp/frozen-lints' into work/api-revision" is not `type: lowercase` and leaks a branch name. The other 29 subjects are well formed, with no bylines or tool adverts. `0025165d`'s body opens "The first review found...", which is workflow leakage in a published commit.
6. `hilavitkutin-api/src/store.rs`, naming and const (low).
   - `StagedBuffered::into_inner` could be a `const fn`.
   - `StartsFromDefault` is open and carries a `type Staged` nothing needs; sealing it would close the surface.

## Verified sound

- The `Window::over` SAFETY claim in `conformance/oracle/walk.rs:47`.
- No new `unsafe` in `frozen-authors`.
- The law and mutant tests: each mutant fails at a named word, and each has a faithful control.
- The `reach_across_cores` tests: a sleep at the slice edge forces the sequence, and there are controls for zero records and for a core with no slice.
- No em-dashes in changed lines.
- The new `Cargo.toml` files' dependencies match `cargo.md`.

## Probes

Run in a scratch copy of the lint crate pointed at the worktree. Both pass, so both holes are real.

A, appended to `mock/lints/frozen_ledger/tests.rs`:

```rust
#[test] fn probe_review2_a_rewritten_hash_with_no_break_passes() { fn edited(p: &str) -> Option<Vec<u8>> { if p == "src/lib.rs" { Some(b"pub fn f() {}".to_vec()) } else { read(p) } } let ledger = ledger_of(&[("Cargo.toml", &hash_text(MANIFEST_OK.as_bytes()), 0), ("src/lib.rs", &hash_text(b"pub fn f() {}"), 0)]); assert_eq!(findings(&ledger, "", &files(), edited), vec![], "an edit with its hash rewritten and no break"); let locked = ledger.replace("locked = false", "locked = true").replace("form = ", &format!("locked_breaks = 0{}form = ", char::from(10u8))); assert_eq!(findings(&locked, "", &files(), edited), vec![], "the same edit of a locked file"); }
```

B, appended to `mock/lints/frozen_no_feature_gate/tests.rs`:

```rust
#[test] fn probe_review2_an_attribute_split_over_two_lines_passes() { let text = format!("#![{}feature(never_type)]", char::from(10u8)); assert!(findings(&text).is_empty(), "an inner feature attribute over two lines"); }
```

- Control for A: the existing `an_edited_file_is_refused` shows the same edit refused when the hash is left alone.
- The three source mutations are described in the suite audit. Both probes were removed again.

## Calls handed back

1. Ranking constructor over `Default`. Op's words rank the null bit, then `Default`, and add `HasTrivialCtor` as "another one to support too" without placing it. Putting the trivial constructor ahead of `Default` is the coordinator's reading and is not ratified; the design is silent.
2. `start_from` for types that have a start. `start_from` is available even for a type that has a start of its own. Whether that is a "wrong API that isn't applicable" under op's third remark is not settled by his words.
3. Generic callers. A caller that knows only `T: Default` gets the default even for a type that also has a trivial constructor (sketch `202611132225`). That breaks the sequence under generic code, and nothing ratified accepts it.
4. Two of the oracle's frame-meaning points are the round's own readings. It never skips an `Always` unit, and it refuses to run a firer listed behind its consumer. The doc changelist `202611132054` section 1 says so. Some of the nine ignored tests call the engine wrong against those readings.

## Process note

- My first `cargo mock --lint-only` and a background `cargo mock test` ran with the shell's working directory still at `/home/user/hilavitkutin`, the other seat's clone. Both only read and built into the shared `/home/user/target`, and `git status` there was empty before and after.
- I could not stop that `cargo mock test` (pid 9868, refused by the permission classifier). It ends at its 1500 s timeout.
