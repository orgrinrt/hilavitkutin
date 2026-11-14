# Review: `work/hv-bench-instruments` (tip 4de7bf70, base 271909e5)

Canon gate: passed. hilavitkutin declares no `canon_paths` (`.claude/rules/canon-design-code-chain.md`), and `mock/benches/` sits outside the phase gate (`.claude/rules/mock-workspace.md`). The governing documents are therefore the job brief (`/home/user/job-hv-bench-instruments.md`) and the repo and workspace rules. Scope holds: `git diff --name-only` touches nothing under `mock/crates`, nor `mock/benches/src/main.rs` or `mock/benches/bench.toml`. The one file outside `mock/benches/` is the research note `mock/research/202611132130_asm-gate-over-the-engine.md`. I found no ratchet, exemption or suppression. I am the first reader, and a second read is owed before anything here is treated as settled.

## Verdict: do not merge

Every check script is green, but the green proves less than it claims. Two of the new tests cannot fail on the defect their names advertise. The committed engine_vs_std results come from a dirty tree with no record of what that tree held, are normalised against a baseline nobody declared, and carry the stock harness verdict lines that ticket `frame_wide_follow_ups_stock_verdict_lines…` exists to remove. FINDINGS.md also miscounts its own reds. Each of these is small to fix. None can be absorbed as a follow-up, because each one is a claim the branch commits as true.

## Findings

1. **Blocks.** `mock/benches/bench-arm/src/image.rs:143-153` [tests]: `the_digest_is_fnv1a_over_the_bytes` claims to be "worked by hand from the offset basis and prime". It is not. It recomputes FNV-1a using the module's own `FNV_OFFSET` and `FNV_PRIME` and compares the result with `fnv1a_u32`, which uses the same two constants. If `FNV_PRIME` were wrong, both sides would be wrong together and the test would still pass. That makes it a tautology, and the comment calling it a hand calculation dresses the tautology up as a check. Delete it and assert a published literal instead, for example the FNV-1a-64 digest of `01 00 00 00`, written as a hex constant.

2. **Blocks.** `mock/benches/single-trunk-convergence/support/tests/correctness.rs:223-236` [tests]: `every_arm_runs_the_walkers_it_claims` asserts `HeadTail::walkers(w) == w.min(2)`, and the impl at `support/src/arms.rs:86-88` is literally `workers.min(2)`. The same holds for every other row. The test restates declarations. Nothing in it observes how many walkers a fold actually runs. The rustdoc at `arms.rs:24` says "Every fold is built with it, so a test holds the count to it", which is false: a fold built with `workers` instead of `Self::walkers(workers)` would still pass. `Nway` and `StdParallel` are also left out of the table. The test gate refuses "declarations nothing constrains". Replace it with a test that measures the count, for example the number of distinct `core` values that reach the walk closure in one `Op::Frame`, or `Crew::workers()` on the crew the fold built.

3. **Blocks.** `mock/benches/engine_vs_std/results/**` and `FINDINGS.md:178-202` [correctness/evidence]: four separate defects in the committed run.
   - Every `meta.json` records `"git_commit":"b14a6212-dirty"`. engine_vs_std writes no `build.txt` beside its CSVs, although frame_wide does, so nothing records which uncommitted code was timed. `stage.sh:17-18` states the rule this breaks: "a result from a dirty tree is only worth what the record of its dirt is".
   - `bench.toml:34,45,56…` declares `baseline = "evs_std"`, but every arm reports itself as `std`/`engine` (`variants/evs_std/src/lib.rs`, `#[bench_variant("std", …)]`). The harness validates the key against the path stem and then calls `with_baseline("evs_std")`, which silently matches nothing (`mockspace f42844c bench-harness/src/analysis.rs:323-327`). As a result, every committed findings file says `Baseline: **engine**` (for example `element_wise_n65536_findings.md:4`) and then tells the reader to re-baseline. Spell the baseline by the reported name, and file the harness defect against mockspace.
   - The 19 committed `_findings.md` files carry the harness's stock `_Why it matters:_` lines ("a concrete reason to switch", "a safe default pick", `accumulator_n1048576_findings.md:20,26`). Those are the class this branch's frame_wide ticket strips, at `frame_wide/src/lib.rs:165-177`. The fix was made in one member and the same defect was then committed in another. Move the stripper into a shared crate (`bench-arm`) and run it here too.
   - FINDINGS.md lists 14 red cells while saying "15 red". `engine_vs_std --verdict` over the committed results prints `element_wise n=65536: engine / std = 1.298, ceiling 1.10, RED`, and the prose omits it.
   - The "15.38x" overlapped-run figure (`FINDINGS.md:200`) has no committed artifact. Either delete the figure or commit the run.

4. **Fix before landing.** `mock/benches/engine_vs_std/support/src/crew.rs:200-216` and `:129-141` [correctness]: the `unsafe` is sound. The job is written only while every worker is parked, it is published by `seq` Release/Acquire (`mock/crates/hilavitkutin/src/thread/frame.rs:35,56`), and it is retired by `done` Release/Acquire (`:79,99`). The chunk ranges in `for_chunks` are disjoint and in bounds for `n = 0` as well, and `progress_slots` is never read by the frame functions. The liveness is not sound. If `f` panics on a worker, that thread unwinds past `frame_done_arrive`, `frame_await_done` spins and parks forever, and `Drop` then hangs in `await_exit` as well. The arms this replaced ran on `thread::scope`, which propagated the panic. Now an arm with an index bug hangs `run-checks.sh` and `stage.sh` in all three members instead of failing them. Wrap the job call in `catch_unwind`, always arrive at the barrier, and re-raise on the caller. Add a test where one worker panics and the frame returns `Err` or panics instead of hanging.

5. **Follow-up, brief job 7 unfinished.** Ticket b1767a1de1ea [tests]: `stage.sh` runs `run-checks.sh` before staging, but only for the three members that have one. The tests of `bench-arm` (5), `bench-stage` (5+5), `kernel_checks`, the `bc_*`/`ctz_*`/`dispatch_*`/`dep_graph_*` arms (17 crates), `src/disasm.rs` (8) and `asm_gate` (3) run in no script and no gate at all. Job 4's "negative control shown failing" therefore exists only as unit tests nothing runs. The brief says "Then fix it: a tool or the launcher runs them." Per `a-check-lives-in-lints-or-tools.md` that means a `mock/tools/` tool. The draft report must not call job 7 done.

6. **Follow-up.** Variants `dispatch_*` and `dep_graph_*` (for example `dispatch_trait/src/lib.rs:50-57`) [tests]: the fixes were removing `inline(never)` and taking the scratch out of the timed region. The only test added is `kernel_checks::varies`, which the old code also passes, so the tests establish nothing about the change. The commit message for b14a6212 ("with the old constant form as the control that fails") is true only of `ctz`. `varies` is also sampled at N = 256 while the arms run 256 through 16384.

7. **Follow-up.** Frame-macro compile-fail check [tests]: the draft says swapping two units in `frame!` "fails to compile at the `Locate` bounds (checked by hand, then restored)". A hand check becomes a committed `trybuild` compile-fail test (`a-hand-check-becomes-a-test-every-time.md`).

8. **Follow-up.** `frame_wide/heavy_n65536*`, `light_n65536*`, `occupancy.txt`, and `tests/engine_grouping.rs:35-36` [drift]: these describe `hand_par` and `rust_std_par` spawning per frame, and those arms no longer exist in that form. None of them is marked superseded. Add a one-line superseded note until the rerun.

9. **Follow-up.** Ticket 973449e7dc90 [drift]: `Crew` is a generic pool, yet it was moved into `evs-support`, which is engine_vs_std's workload crate. frame_wide and convergence now link all of engine_vs_std's workloads to get it. The branch had just created `bench-arm` as the shared home (`a-crate-holds-only-its-own-thing.md`).

10. **Follow-up.** [drift]: `mock/benches/.gitignore:7` ignores every `Cargo.lock`, and notko is followed by `branch = "dev"`. The claim that `asm_gate_report.md` "regenerates byte-identical from the same tree" therefore depends on notko dev not moving. `--check` resolved fresh here ("Locking 25 packages").

11. **Follow-up.** `support/src/tests.rs:147-184` [tests]: the `for_chunks` coverage law is tested at 3 workers only. Add 1, 2 and `MAX_CORES`.

12. **Follow-up.** `src/disasm_5check.rs` has no tests, and check 3 is known to say nothing (ticket 42eb704cf306).

13. **Nit.** `mock/research/202611132130_…md:3` uses bold metadata, which the repo allows only in `design_rounds/`. `NIGHT_FINDINGS_2026-05-11.md` keeps bold label bullets and is 902 lines. That file was over 500 lines before this branch.

## Checks run

| Check | Outcome |
|---|---|
| `engine_vs_std/run-checks.sh` | pass (40 s) |
| `frame_wide/run-checks.sh` | pass (3 min 30 s) |
| `single-trunk-convergence/run-checks.sh` | pass (40 s) |
| `cargo test` in `single-trunk-convergence/support` | 8 pass |
| `cargo test` in `bench-stage` | 5 + 5 pass |
| `cargo test` in `bench-arm` | 5 pass |
| `cargo test` in the 17 touched microkernel crates | all pass |
| `cargo run --release --bin asm_gate -- --check` | PASS: 9 bodies, control refused, report byte-identical |
| `cargo mock` at the root | exit 0, tree clean, existing warnings only in `mock/crates` |
| `engine_vs_std --verdict` over the committed results | 28 judged, 15 red, exit 1 |
| `results/summary.csv`, convergence | 88/448 warm rows `wide_spread`; columns 1-15 identical to base |
| NIGHT_FINDINGS dep_graph hybrid figures | +407.56% and +548.27% match the committed findings |
| Commit subjects | all under 72 characters, conventional; no trailers, bylines or em-dashes in any of the 5 messages or in added lines |

## Verified

- `stage.sh`: it refuses an existing directory, the checks gate the build, and `bench-stage/tests/stage_script.rs` has a failing control.
- The convergence `wide_spread` flag. Its summary regenerates unchanged apart from the new column.
- The `HeadTailMid` `Regions` impl is kept as the documented negative fixture for `head_and_tail_without_the_fallback_reverses_the_tail_half`, which asserts the reversal exactly. The `split.rs` doc now matches the code.
- The `ctz` and `bc` families have real reference checks with failing controls (`ctz_common` against `u64::trailing_zeros`, `bc_common::check`).
- The asm gate has a control that is shown failing, and the worker body is scanned.
- engine_vs_std `--verdict` exits 1 on any red cell, so the move to the harness keeps the old oracles strict.

## Notes

Fixing findings 1 to 4 is a few hours' work and needs no timing run: the committed results can be dropped and rerun later instead of being kept unprovenanced. Findings 5 and 7 are owed by the brief itself. The mockspace `with_baseline` silent no-op in finding 3 is upstream and wants its own ticket.
