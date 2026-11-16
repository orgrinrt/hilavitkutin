# Bench conformance: hilavitkutin ported to the composed harness

The benches of hilavitkutin are ported from the legacy sections form to the composed tree the mockspace harness runs today, so a run with no names builds and runs every cell. This note records what changed, what was fixed on the way, which gates were checked and what is still owed.

## Ported

All 37 root `[bench.*]` sections are composed members, each `<bench>/bench.toml` with its own `arms/`, `points` and `[timing]`. No `[[bench.*.sizes]]` table and no `target/release` path is left. `engine_vs_std`, `frame_wide`, `resource_storage` and `single-trunk-convergence` take `[sweep.*]`; their arm directories moved from `variants/` to `arms/<lib name>`, so result names keep their continuity. The eight shared crates moved to `support/`, and `dispatch_direct` serves both dispatch benches from one source. In all: 41 bench directories, 81 bench keys and 151 arm manifests. The results layout is unchanged.

## Fixed on the way

`atomic_acquire` produced a digest that never matched its siblings, so validation dropped the whole cell; it now folds `observed - prev`. No arm was deleted. Two arm bodies were rewritten by the port. Commit `dd0d26e3` claims to put them back and is empty; `e96bf949` is the commit that does: `rot_manual_shifts` had become `v.rotate_left(k)` and is again the hand-written `(v << k) | (v >> (64 - k))`, since the arm measures whether LLVM recognises the idiom, and `sat_checked` had become `saturating_add`, the same as its sibling, and is again the `checked_add` match falling back to `u64::MAX`. Both carry the clippy allow for the rewrite autofix would make, and the repo lint `a-bench-arm-keeps-the-idiom-it-measures` refuses either drifting back. Every arm was diffed against the merge base `2b137e9d` and these two were the only ones whose meaning changed. `ema_neon` runs its fallback on targets with no NEON under its own name, as op ruled for arvo's arms: a target with no vector path is a valid result.

Axis E of `resource_storage/axis_ce` timed itself with `Instant` inside a test and only printed its number, so no committed number backs the V3 decision. It is now the `resource_storage/false_sharing` sweep (`fs_packed`, `fs_padded`, threaded), with the kernel and three tests in `rsb_kernels::false_sharing`; `axis_ce` keeps axis C. The new sweep has no committed results yet, so the V3 decision is still unbacked; see Owed. The kernel had also lost the old floor of two threads, which is restored. No other clock is read in the bench tree. The polka-dots imports are verbatim research history and stay.

The `dispatch_static` hook gate now fails a cell only on check 1, devirtualisation. Checks 2 to 4 failed on every arm on the M1 and the Xeon alike, since they describe the engine's morsel loop and not `bench_entry`; check 1 is what `asm_gate_fixtures` already gates on. The disassembly checker lost a GOT slot held in `%r10` across `lock add`, `div` and panic calls, so a `memset` call read as a stored pointer, and `asm_gate_fixtures` failed hard on `dev` after notko's `dev` moved. That is fixed and the asm gate report regenerated. A call is treated as never returning only when its last name segment is a panic or abort entry point, so a returning function of the `panicking` module such as `is_zero_slow_path` forgets the register; keeping the register past a real panic call still rests on cold-block layout, and the checker says so. The gate selects the indirect-call check by its name rather than by its place in the list.

## Gates

Before the port the lints were green and `dispatch_static` red on every machine. After it `cargo mock --lint-only --commit` is green on every commit, and `cargo mock bench test` is green on every crate except `asm_gate_fixtures`, which failed in the full run (171 crates, one failed) before the checker fix and passes four of four alone after it.

A smoke run took every key at its smallest point in a scratch worktree, results not committed. On the port commit 80 of 81 cells ran clean, with `atomic_ordering_cost` failing validation and `dispatch_static` the old gate. After the fixes both pass. The whole tree was not run again after the checker fix.

## Branches carrying bench data

Each was checked against `dev` file by file and row by row. `salvage/bench-hilavitkutin-20261115T132518Z` holds 1709 files, all byte-identical on `dev`, and was deleted locally. `bench/macos-aarch64-apple-m1/20261114T1125Z` holds 591 ledger rows, all in `dev`'s `summary.tsv` files; its remote deletion is queued. `work/bench-recover-overwritten-runs` has a tree identical to `dev`, landed as `c5567576`, and is deleted. `recover/unlanded-rounds-and-benches` landed through `2f06ffe6` and `49ab3440` with later versions on `dev`; its remote deletion is queued.

## Owed

Runs are not yet reproducible from a hilavitkutin commit, since arm locks are ignored by git and notko and the harness follow `dev`: tickets `the_bench_driver_refuses_more_than_one_bench_framework_versi` and `bench_members_follow_notko_on_branch_dev_with_every_cargo_lo`. The microkernel arms still do not measure what their labels say: `four_microkernel_bench_families_do_not_measure_what_their_la`. A shared arm has no form in the composed tree: `an_arm_shared_by_two_benches_has_no_form_in_the_composed_tre`. A run cannot be held to one point of a sweep: `mock_bench_run_cannot_restrict_a_run_to_one_point_of_a_sweep`. Arms build into `mock/benches/target`: `cargo_mock_bench_run_builds_its_arms_into_mock_benches_targe`. No committed number backs the V3 decision until the `resource_storage/false_sharing` sweep is run and its results committed: `run_the_resource_storage_false_sharing_sweep_and_commit_resu`.
