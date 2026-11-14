# Review round 2: `work/hv-bench-instruments` at 12e690c0, base `origin/dev`

Canon gate: passed. hilavitkutin's `mockspace.toml` declares no `canon_paths`, and `mock/benches/` sits outside the phase gate (`.claude/rules/mock-workspace.md`), so the governing documents are the job brief and the repo and workspace rules. Scope holds: nothing under `mock/crates`, nor `mock/benches/src/main.rs` or `mock/benches/bench.toml`, is touched. No ratchet, exemption or suppression added. I am a second reader of this branch; round 1 was the first.

## Verdict: MERGE AFTER FIXES

The four round-1 blockers are fixed in substance. The FNV literals are real and I checked them independently. The walker count is now observed. The crew no longer hangs on a panic. The committed run is clean, on the declared baseline, stripped of stock lines, and FINDINGS.md matches `--verdict` cell for cell. Two things in the new work are still not true as written, and both are cheap to fix.

## Findings

1. `mock/benches/engine_vs_std/results/*/*.meta.json` and `FINDINGS.md:170-173` [correctness/evidence], fix before landing. The two records in each cell disagree. All 20 `.build.txt` files say `commit: c894a779…, dirty: no`. 19 of the 20 `meta.json` files say `"git_commit":"23953983"`, and only `accumulator_n4096.meta.json` says `c894a779`. The cell timestamps run from 01:33:21Z to 02:00:05Z, and 23953983 was committed at 01:34:39Z into the same clone the run was reading HEAD from. So a commit, with whatever pre-commit hook that clone runs, happened in the middle of the timing. That contradicts `FINDINGS.md:171-172`, "with no other build on the machine while it timed". The binaries are still c894a779's, because the stage was checksummed before and after the run and 23953983 only touches a research note. But the committed artifact now names two commits, and the prose makes a claim the timestamps refute. I could not read the clone's hooks (permission refused), so whether that commit ran a build is unverified. Fix: say in FINDINGS that the harness's `git_commit` is the clone's HEAD at each cell, that a docs-only commit landed at 01:34, and that the stage record is the authority. Drop the "no other build" sentence, or replace it with what is known. The commit message of 55d33165 ("Every cell carries the build record of c894a779") is true of `.build.txt` only.

2. `mock/benches/engine_vs_std/support/src/tests.rs:547-564` [tests], fix before landing. `a_worker_that_panics_fails_the_frame_instead_of_hanging_it` cannot fail. Take out the `catch_unwind` in `crew.rs:237`, and the test spins in `frame_await_done` forever. Nothing in the test, in `checks.sh` or in `run-checks.sh` has a timeout (grep for `timeout|watchdog|recv_timeout`: no hits). So the regression shows up the way round 1 described the original defect: `run-checks.sh` and `stage.sh` hang instead of failing. The commit message of c894a779 says it "fails by timeout without it", and that is false. Fix: run the frame on a helper thread and `recv_timeout` its result on an `mpsc` channel (a few seconds), then fail on the timeout.

3. `mock/benches/engine_vs_std/src/verdict.rs` (`judge`) [drift], follow-up ticket. The verdict compares single-run medians with a hard ceiling and no noise band. `startup_wide_parallel` reads 1.091 against 1.10 and is called green. Between round 1's run and this one, five cells went from red to green: startup_accumulator 1.30 to 0.94, startup_wide_parallel 1.14 to 1.09, accumulator par at 4096 from red to 0.86, wide_parallel par at 4096, and branching at 65536. The code changed in between, so not all of that is noise. Still, nobody has measured how much is. FINDINGS says honestly that this is unmeasured (`:172-173`). The harness already computes per-cell CIs, and the verdict throws them away. A cell whose CI straddles the ceiling should read undecided.

4. `mock/benches/frame_wide/heavy_n65536*`, `light_n65536*`, `occupancy.txt` [drift], round-1 #8 not addressed. These still describe `hand_par` and `rust_std_par` spawning per frame (`heavy_n65536_findings.md:51-99`), with no superseded marker in the tree. They also keep harness editorial that `plain` does not strip ("The strategy choice matters a lot for this workload", `:78`; "flatters all rivals", `:48`). The upstream ticket 617d143ed3f5 covers the second part. The first needs one line per file.

5. `mock/benches/README.md` [drift], follow-up. It still describes the scaffold workflow (`mock bench run` builds everything, `IdentityAdd` in `src/main.rs`). It says nothing of `stage.sh`, `run-checks.sh`, `bench-arm` or `bench-stage`, which every member on this branch now requires before a timing counts.

6. `engine_vs_std/results/*/*.build.txt:4` [nit]: `profile: release, as /home/user/hilavitkutin/mock/benches/engine_vs_std/Cargo.toml declares it` commits an absolute machine path. The record should name the member-relative path.

7. Commit 55d33165 subject [nit]: `engine_vs_std` is not backticked (`pr-text-is-published-writing-in-ops-voice.md`).

## Round 1 findings

1. FNV tautology (`bench-arm/src/image.rs:143-157`): addressed. The test asserts three literals. I recomputed them with an independent FNV-1a in bash arithmetic: offset basis `cbf29ce484222325`, "foobar" `85944171f73967e8` (the published value), `abcd` `fc179f83ee0724dd`, `01 00 00 00` `ad2aca7747985764`. All match.
2. Walker declarations test: addressed. `support/tests/walkers.rs` counts the distinct threads that apply a counting kernel in a frame, and the threads a fold spawns (from `/proc/self/task`), for all seven arms at W = 1 to 4, with `Nway` and `StdParallel` now included. A fold built with `workers` instead of `Self::walkers(workers)` fails on `spawned` at W = 3 and 4. I ran it 8 times in a row with no flake (about 7.5 s each). It is Linux-only through `#![cfg]`, and elsewhere it compiles to zero tests.
3. Committed run: addressed, with finding 1 above. (a) `.build.txt` beside every cell, `dirty: no`, but `meta.json` disagrees. (b) The arms report `evs_*` (`variants/evs_*/src/lib.rs:8`), and all 20 findings read `Baseline: evs_std` or `evs_std_startup`. (c) Zero `_Why it matters:_` lines and zero `**` in the 20 findings files. The stripper moved to `bench-arm/src/findings.rs`, and frame_wide uses it (`frame_wide/src/lib.rs:158`). (d) FINDINGS lists 10 red cells, and `--verdict` prints the same 10. Every ratio quoted at `FINDINGS.md:150-161` matches `INDEX.md` and the verdict. (e) The 15.38x figure is removed.
4. Crew panic liveness (`crew.rs:139-160,237-245`): fixed. The worker catches the panic, always arrives at the barrier, and the caller re-raises the first payload. Ordering is fine: the payload is written before `frame_done_arrive`, and the caller reads it after `frame_await_done`, which has Release/Acquire on `done`. `trampoline` uses the Rust ABI, so unwinding through it is defined. The test cannot fail, only hang: see finding 2.
5. Job 7 (members' tests in no gate): not addressed. Known, ticket fa4963b8d8fb.
6. `dispatch_*`/`dep_graph_*` tests: not addressed. These variants are untouched since 4de7bf70, and `kernel_checks::varies` is still the only test, still sampled at N = 256.
7. `frame!` swap compile-fail as `trybuild`: not addressed. There is no `trybuild` in any bench `Cargo.toml`. The author's draft report still says "checked by hand".
8. frame_wide stale results: not addressed (finding 4).
9. `Crew` in `evs-support`: not addressed. It stays on ticket 973449e7dc90.
10. Ignored `Cargo.lock`s and notko on `branch = "dev"` (`mock/benches/.gitignore:7`, `asm_gate_fixtures/Cargo.toml:26,37`, `engine_vs_std/Cargo.toml:36`): not addressed. "Regenerates byte-identical" still depends on notko dev not moving.
11. `for_chunks` at 3 workers only: addressed (`support/src/tests.rs:149-174`). It now runs at 1, 2, 3 and `MAX_CORES` (256), and asserts `workers()`.
12. `disasm_5check.rs` without tests: not addressed. It has 0 `#[test]`, and it stays on ticket 42eb704cf306.
13. Bold metadata in the asm-gate note: addressed (23953983, now prose at `:3`). `NIGHT_FINDINGS_2026-05-11.md` is unchanged: 902 lines, 102 bold label bullets.

## Tests added or changed in this round, and whether each can fail

- `bench_arm::image::the_digest_is_fnv1a_over_the_bytes`: can fail. The literals come from outside the crate.
- `bench_arm::findings::the_stock_lines_and_the_bold_go_and_the_rest_stays`: can fail. It is an exact literal comparison, and it covers a non-stock italic line that must survive.
- `engine_vs_std::tests::only_this_runs_cells_are_settled`: can fail. Settling every cell, settling none, or writing no record each breaks an assertion. It depends on mtime resolution finer than 20 ms, which holds on the Linux filesystems in use.
- `verdict` tests: renamed to the `evs_*` keys only. They can still fail at the ceiling boundary (`ceiling * 1.01`).
- `the_crew_covers_every_record_once_in_its_workers_ranges`: can fail. It now runs at 4 worker counts.
- `a_worker_that_panics_fails_the_frame_instead_of_hanging_it`: hangs rather than fails on regression (finding 2). The payload and reuse assertions are real.
- `walkers.rs::every_arm_runs_the_walkers_it_claims`: can fail on both the declaration and the built fold.
- `correctness.rs`: the declaration-restating test is deleted, which is correct, and a comment reference is updated. `the_dynamic_meeting_point_covers_every_record_once` exists at `:155`.

## Commands run

| Command | Result |
|---|---|
| `engine_vs_std/run-checks.sh` | exit 0, 45 s |
| `single-trunk-convergence/run-checks.sh` | exit 0, 65 s |
| `frame_wide/run-checks.sh` | exit 0, 219 s |
| `cargo test` in `bench-arm` | 6 pass |
| `cargo test` in `bench-stage` | 5 + 5 pass |
| `cargo test` in `single-trunk-convergence/support` | 7 pass (correctness) + 1 pass (walkers) |
| `cargo test --test walkers`, repeated 8 times | 8/8 pass, 7.1 to 8.8 s |
| `cargo test` in `engine_vs_std/support` | 9 pass |
| `cargo run --release --bin engine_vs_std -- --verdict` | 28 judged, 10 red, exit 1, matching FINDINGS.md |
| `cargo run --release --bin asm_gate -- --check` (in `mock/benches`) | PASS, 9 dispatch bodies, control refused, exit 0 |
| `cargo mock` at the root | exit 0, tree clean, warnings only in `mock/crates` (existing) |
| FNV-1a in bash arithmetic | 4 digests match the test literals |
| `git diff origin/dev...HEAD` added lines, em-dash count | 0 |
| commit bodies, bylines or trailers | 0; all 9 subjects conventional, 47 to 71 characters |
| `meta.json` `git_commit` tally | 19 × `23953983`, 1 × `c894a779` |

## Notes

The author's draft report at `/home/user/report-hv-bench-instruments.md`, which is not on the branch, is stale against this round. It still says "a test holds head-and-tail to min(W, 2)" and "checked by hand", and it has `RESULTS_TESTS`/`RESULTS_RUN` placeholders. Findings 1 and 2 are each well under an hour and need no timing run. Findings 3 to 5 and the unaddressed round-1 items 6, 7, 10 and 12 want ticket ids, if they have none, before this lands as "follow-ups".
