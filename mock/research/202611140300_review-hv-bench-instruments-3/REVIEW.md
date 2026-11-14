# Review round 3: `work/hv-bench-instruments` at ddac3d51, base `origin/dev`

Canon gate: passed. `mockspace.toml` declares no `canon_paths`. `mock/benches/` sits outside the phase gate (`.claude/rules/mock-workspace.md`), so the job brief and the repo and workspace rules govern. Scope holds: every changed path is under `mock/benches/` or `mock/research/`, and neither `mock/benches/src/main.rs` nor `bench.toml` is touched. No ratchet, exemption or suppression was added. I read round 2's file only after forming my own reading of the three commits. This is a single read, so a second read is still owed on any canon call, though none is made here.

## Verdict: MERGE AFTER FIXES

The migration onto `StagedBuild` is faithful, the pin bump adds only staging, and round 2's finding 2 is now properly fixed: I took out `catch_unwind` in a copy and the test fails at 10 s instead of hanging. One regression came in with 560cd6b3. Deleting `bench-stage/src/lib.rs` deleted the only test of `stage.sh --verify`, and three frame_wide scripts still rely on that path. It is about 15 lines to fix. Because this is the last round, the coordinator may land the branch with finding 1 as a recorded follow-up. If so, it gets its own ticket and is not folded into fa4963b8d8fb.

## Findings

1. `mock/benches/stage.sh:37-45` together with `bench-stage/tests/stage_script.rs` [tests], a regression from 560cd6b3. Before this commit, `bench_stage::Stage::verify` shelled out to `stage.sh --verify`, and `a_file_replaced_after_staging_fails_verify` (old `bench-stage/src/lib.rs:184-193`) was the negative control for that path. The commit deleted both. The drivers now verify in Rust through the harness, but `frame_wide/occupancy.sh:68,159,180` and `frame_wide/vectorisation.sh:45,109` still call `"$stage" --verify`, and nothing tests it any more. `stage_script.rs` contains no `--verify` at all. If someone breaks the `sha256sum -c` line, those scripts will pass a swapped arm without anyone noticing. Fix: add a `stage_script.rs` test that stages the fixture, overwrites the arm, and asserts that `stage.sh --verify <out>` exits non-zero and names the file. Add the matching positive case too.

2. `stage.sh:37-45` against `mockspace_bench_harness::staging::StagedBuild::verify` [drift], follow-up ticket. One check now has two implementations, and they apply different rules. The harness refuses an empty record, a record that omits the running driver, and paths that escape the stage. `sha256sum -c` checks none of these. The brief's standing tenet is that "one concept has one owner". The harness now owns staging, so the shell callers should go through it, for example with a `--verify-stage` mode on the driver or a small harness bin. Until that happens, finding 1's test is what holds the script to its word.

3. `engine_vs_std/src/main.rs:47-71` and `single-trunk-convergence/src/main.rs:51-78` [correctness/docs], minor and unchanged by this branch. These two drivers verify only after the run. `frame_wide/src/main.rs:75,102-106` verifies before and after. Yet `stage.sh:10-11` says that a run "checks the checksums written at staging before it starts and again when it ends". Either add `stage.verify()` before `drive` in the two drivers (two lines each) or narrow the header. `FINDINGS.md:184` ("checksummed before the first cell and after the last") is still true, because the checksum was written at staging.

4. `engine_vs_std` and `single-trunk-convergence` [tests], minor. Only frame_wide tests the refusal of an unstaged driver (`frame_wide/tests/unstaged_driver.rs:54-57`). The other two drivers refuse in code (`main.rs:49-58` and `:54-63`), but if either lost its `of_current_exe` call it would go back to timing whatever sits in the shared target, and no test would notice. Copy the frame_wide test into both.

5. `engine_vs_std/results/*/*.build.txt:4` [evidence], note only. The 20 committed records still say `profile: release, as /home/user/hilavitkutin/mock/benches/engine_vs_std/Cargo.toml declares it`. Leaving them as written is correct, because hand-editing a run's record would be worse. The next staged run will carry the relative path. One clause in FINDINGS saying so would stop the next reader from flagging it again.

6. `stage.sh:48` [robustness], nit. `member=$(cd "$member" && pwd)` returns the logical path, while `git rev-parse --show-toplevel` returns the physical one. In a clone reached through a symlink, `${member#"$repo"/}` strips nothing and the absolute path comes back. The new test compares against a `canonicalize`d path, so it would not catch that. Use `pwd -P`. I checked: this worktree has no symlink in its path, so the test is valid here.

## Round 2 findings

1. meta.json and build.txt disagreement, plus "no other build" (`FINDINGS.md`): addressed at `FINDINGS.md:182-189,208-210`. The "no other build" claim is gone. `.build.txt` is now named as the authority, and the 23953983 move is explained. I checked the facts. 23953983 is `2026-11-14 01:34:39 +0000` and touches one research note (1+/1-). The tally is 19 × `23953983` and 1 × `c894a779`. The 55d33165 subject cannot be fixed without a rewrite, and that is acceptable.
2. Panic test cannot fail: addressed (`support/src/tests.rs:198-216`). The frames now run on their own thread, and the test calls `recv_timeout(10 s)`. A panic inside the frames drops the sender, so it fails at once with `Disconnected`. A hang fails on `Timeout`. I mutated `crew.rs:237` in a scratch copy so the job runs without `catch_unwind`. The test failed after 10.00 s with "the frame hung on a worker's panic ...: Timeout", and the suite did not hang.
3. Verdict noise band: not addressed, ticketed by the coordinator.
4. frame_wide superseded results: not addressed, ticketed.
5. Benches README: not addressed, ticketed.
6. Absolute path in build.txt: partly addressed. `stage.sh:103` is fixed, and `stage_script.rs:59-65` guards it. Before the fix, `$member` was absolute (`stage.sh:48`) and contained the repo root, so the test can fail. The committed records keep the old line (finding 5).
7. 55d33165 subject backticks: not fixable without a history rewrite. The three new subjects are backticked where needed and run 66, 66 and 60 characters.

## The three commits

- 560cd6b3, the migration. This keeps the old semantics. An unstaged driver is still refused (`StagedBuild::of_current_exe`, which now canonicalizes the path and requires `release/` plus `binaries.sha256`). Verification is unchanged: evs and stc check after the run, frame_wide before and after. The upstream `verify` is stricter than the old one, since it requires the running driver to be in the record, and `stage.sh:96` writes `release/<driver>`, which satisfies that. In evs and stc, `_link` is a named binding and lives until the end of `main`. Every return drops it, and its `Drop` removes `target` only if this call made the link and the link still points at this stage. Neither the harness's `src/` nor these members contain `process::exit`, `set_current_dir` or `panic = "abort"`, so nothing skips the drop or moves the relative `target`. The one behaviour change is deliberate: `target` is now removed at the end of the run, where before it was left in place. A concurrent run in the same member can still repoint `target` partway through. That hazard predates this branch and is not new here.
- Pin bump from f42844c to 5a775f3. `5a775f3` is on mockspace `origin/dev`. `git diff f42844c 5a775f3` touches the new `staging.rs`, `staging_tests.rs`, the `sha2` dependency, `pub mod staging`, a README section, one skill line, and `build.rs`. In `build.rs` the perf-counters branch keeps its logic and only gains the `stage_link` cfg. No other harness source changed. In these members, every `-`/`+` pair on a pin changes only the rev, apart from the `stage-link` feature in evs and stc and the removal of the `bench-stage` dependency.
- 42fb4614 and ddac3d51: covered above. REVIEW.md lands where the record rule wants it.

## Tests added or changed since round 2

- `a_worker_that_panics_fails_the_frame_instead_of_hanging_it`: can fail, and I showed it failing by mutation.
- `the_stage_holds_the_driver_and_the_arm_and_nothing_else`, new path assertion: can fail against the pre-fix script (finding 6 is its blind spot).
- `fixture-member/src/main.rs`: reads through `StagedBuild`. `an_arm_replaced_after_staging_is_seen_by_the_staged_driver` still fails with exit 2 on a swapped arm, which makes it a real negative control for the harness path.
- Deleted: the five `bench-stage` unit tests. Upstream `staging_tests.rs` covers what they covered and more (malformed lines, escaping paths, a missing driver, link keep, replace, remove and refuse). The exception is the `stage.sh --verify` control (finding 1).
- `engine_vs_std/src/main.rs` `stage_in`: now builds a `StagedBuild`. Its `binaries.sha256` line names `release/driver`, so the stricter verify still accepts it.

## Commands run

| Command | Result |
|---|---|
| `engine_vs_std/run-checks.sh` | exit 0, 97 s |
| `single-trunk-convergence/run-checks.sh` | exit 0, 76 s |
| `frame_wide/run-checks.sh` | exit 0, 257 s |
| `cargo test` in `bench-arm` | 6 pass |
| `cargo test` in `bench-stage` | 5 pass |
| `cargo test` in `engine_vs_std/support` | 9 pass |
| `cargo test` in `single-trunk-convergence/support` | 7 + 1 pass |
| `cargo run --release --bin engine_vs_std -- --verdict` | 28 judged, 10 red, exit 1, matches FINDINGS |
| `cargo mock` at root | exit 0, tree clean after, no finding under `mock/benches` (warnings only in `mock/crates`, existing) |
| `cargo test --release -p evs-support a_worker_that_panics` with `catch_unwind` removed (scratch copy) | FAILED on `Timeout` at 10.00 s, no hang |
| `git diff f42844c 5a775f3 --stat` in mockspace | staging module, tests, `sha2`, README, skill line, `build.rs` cfg |
| `git branch -r --contains 5a775f3` | `origin/dev` |
| `grep -- --verify` under `mock/benches` | 5 calls in frame_wide scripts, 0 in any test |
| `meta.json` `git_commit` tally | 19 × `23953983`, 1 × `c894a779` |
| em-dashes in added lines, bylines in new commit bodies | 0, 0 |
