# Build and link bench, the first cells

This note reports steps 1 to 6 of the builder's steps in the build and link bench plan (`.shared/research/202611150830_stack-path/59-the-build-and-link-bench-plan.md` in the workspace repository, section 10), as built on the branch `work/bench-build-and-link` of hilavitkutin. The plan is an agent's work and was read as a claim to test; the lead designer's words in files 58 and 60 of the same directory, and hilavitkutin's registry rows, were read as the governing record.

What exists now is a generated, deterministic fixture of twelve domains at 25, 97 and 169 crates, a support crate that turns one bench call into one cargo build or one link and records what that call did, and two benches on the toolchain's own defaults: `build-static`, with incremental compilation on and off over four scenarios, and `link-static`, with the relink through cargo (quantity A) and the replay of the recorded link line (quantity B). Every timing below comes from the bench harness and is quoted beside its run directory under `mock/benches/results/`. Crate counts and byte sizes come from the call record the hooks library writes during the same harness run (`calls.tsv` in each run directory).

Nothing here configures a linker, a backend, an edge form, a C compiler or an LTO setting. Those arms wait on the hilavitkutin-build design round, as the plan's section 6 and the brief say, and the benches carry no flag table of their own.

The mid point of `build-static` (step 5) is not recorded. Its run was started and cut, as the blockers below say, and the rerun is ticket `record_the_97_crate_point_of_build_static`.

The support crate was reworked after a review (`REVIEW.md` beside this note). The runs recorded below ran under the guards as they were before that, and the section "The guards each recorded run ran under" says which.

## Step 1: a new bench is outside a design round

The write guard gates only `mock/crates/` and the mock workspace's root `Cargo.toml` (a claim about code: `.claude/hooks/mockspace-write-guard.sh`, lines 324 and 382). `mock/benches/` is outside both, and `frame_wide` was added in `44f7de28` with no round. So the bench and its support crates were added without a round.

## The canon gate

Rows asked of the canon (`cargo mock canon find`):

| row | state |
|---|---|
| `build_configuration_has_four_parts_and_no_custom_cargo_features` | ratified |
| `there_are_five_build_profiles_dev_dev_opt_release_profiling_and_ci` | ratified |
| `the_build_crate_is_a_build_dependency_of_every_crate_and_has_no_runtime_dependencies` | ratified |
| `build_behaviour_is_set_through_pragmas_enabled_per_profile_by_a_builder` | ratified |
| `a_fork_over_which_implementation_performs_better_is_registered_open_with_a_named_bench_oracle_and_ruled_from_the_findings` | provisional, stamped |

`cargo mock canon find linker` returns only rows of the linking crate, so the canon is silent on the linker axis. Steps 1 to 6 contradict none of these rows nor the lead designer's words in files 58 and 60, so nothing was refused. Four calls are handed back:

1. The LTO thin and fat sweeps of `link-static` are in neither the plan's list of what may be done before the round nor its list of what waits (section 6). They were not run.
2. `link-dylib` and `build-dylib` have no default cell inside steps 1 to 6, because the dylib form itself waits on the round. They were not added.
3. Incremental is switched by cargo's `CARGO_INCREMENTAL` in the call's environment, not by a profile key in the fixture's manifest, so no per-target configuration is written anywhere. Whether hilavitkutin-build should own that switch is the round's call.
4. The fixture's `dev` and `release` are cargo profiles of the generated workspace (`dev`: opt-level 1, no debug information; `release`: opt-level 3, one codegen unit). They are not hilavitkutin's five profiles, and the canon gives hilavitkutin's `dev` the cranelift backend, which this bench does not use. A reader must not read the `dev` cells as hilavitkutin's `dev`.

## What was built and where

All paths are under `mock/benches/`.

1. `support/build-fixture/`: the generator. Twelve domains in three tiers (roots 0 to 3, mids 4 to 7, leaves 8 to 11), each a contract crate and `c` implementation crates, and one facade binary that composes every implementation. Mids read every root; leaves read every root and two mids; only the facade takes implementations. `c` is 1, 7 and 13 at the points 25, 97 and 169. Each implementation carries 2230 non-inlined functions (`FUNCTIONS`), and each contract four generic items (`GENERICS`). The seed is `0x6876_626c_6669_7831`. `write` refuses a tree whose digest is not the committed one (`COMMITTED`: 25 `0x426667db754199ed`, 97 `0x6053cab89e05fb7a`, 169 `0xc5bb68a50a2c7a46`) and writes a hand-made, sorted `Cargo.lock`, so the fixture builds `--offline --locked`. Tests: `tests/digest.rs` (the committed digest at every point, which is what holds the generator deterministic, and a changed seed refused) and `tests/reach.rs` (each edit site's reach, and the graph's shape, at every point).
2. `support/build-call/`: one call. Each arm and point gets `<root>/<bench>/<arm>/n<n>/` with the workspace, its own target directory, temporary directory and an empty cargo home; the arm's other points are pruned when it moves on. The pinned toolchain is resolved once through rustup, and its own `cargo` and `rustc` are then run by path. Their environment is cleared and only `PATH` is put back, so no inherited variable reaches the build: no flag or wrapper variable, no `CARGO_TARGET_<triple>_LINKER`, no `CARGO_PROFILE_*`, no `CARGO_BUILD_JOBS`, no `RUSTC`. Cargo's configuration files cannot reach it either: the cargo home is the cell's empty `home/`, and cargo runs from the filesystem root with the fixture named by `--manifest-path`, so no directory above the fixture is searched, and a configuration at the root itself refuses the call. On this machine that keeps out `/root/.cargo/config.toml` (`[build] incremental = true`, `[profile.dev] debug = 0`) and the workspace's `/home/user/.cargo/config.toml`. The command line adds `-Zchecksum-freshness`, `--offline --locked`, and the target and build directories of the cell. The untimed setup first holds the disk to the cell's budget (below), then does what the scenario needs (an empty target for `clean`; a catch-up build and a new edit slot content for the edit scenarios; a catch-up build and removal of the executable and the facade's own deps for the relink; the link line printed by `rustc --print link-args -C save-temps`, stored, and its output removed for the replay, which then runs with the same cleared environment and the variables rustc's line sets). The timed block is the one cargo build or the one link process, inside the harness's `timed!`. `record` appends one line per call to the call record, and `check` holds every call of a cell to: the scenario's label; for a cargo call, a compiled count equal to the generator's prediction, and for a replay, which runs no cargo, no count at all (`-` in the record); the setup having undone what it must (`absent_before`); the output present after; exit 0; cargo never waiting on a lock; for a link, the mark the arm's linker must leave, which for the `default_*` arms is `Linker: LLD <LLVM version>` worked out at run time from the toolchain's own description of the host target (a self-contained lld flavor) and its LLVM version, and an arm naming any other linker is refused until hilavitkutin-build gives that linker its mark; and the same check line printed by the built program on every call. Tests: `tests/controls.rs` (a clean call that skips the clean, an edit by file time only, a relink and a replay that keep the executable, each refused with its reason; the honest relink's mark equal to the one worked out), `tests/marks.rs` (a missing mark, another linker's mark, a version that only starts alike, and an arm named for mold carrying lld's mark, each refused), `tests/record.rs`, `tests/environment.rs` (a hostile linker, `RUSTC`, jobs and profile variable, a hostile cargo home and a hostile configuration above the fixture: the call builds and links, and each breaks the same command where it reaches it), `tests/lock.rs` (the cell's build locks held while a call starts: cargo prints its `Blocking` line with stderr piped, and the call is recorded as waiting and refused), `tests/space.rs`, `tests/bounds.rs`, `tests/linkline.rs`.
3. `build-static/` with arms `llvm_incremental` and `llvm_non_incremental`, sweeps `clean`, `leaf`, `mid`, `root`, points 25 and 97.
4. `link-static/` with arms `default_relink` (quantity A) and `default_replay` (quantity B), sweeps `dev` at 25 and 97 and `release` at 25.
5. `bench.toml`: a workload `build` that is the call alone. `src/lib.rs`: `routine_for`, `on_init` and `after_cell` route the two benches to `build_call::hooks`, which writes `calls.tsv` into each run directory and fails the cell when `check` refuses.

The bound on a call (`support/build-call/src/routine.rs`) is three times the slower arm's first timed call of the cell, rounded up to a second, read from the table in `support/build-call/src/measured.rs` and scaled by the crate count from the nearest measured point below for a point not yet measured. Every sweep of both benches has a row at its smallest point, so none runs under the harness's 300 s fallback; `tests/bounds.rs` reads both `bench.toml` files and fails on a sweep that would. The link rows (1 s at dev 25, dev 97 and release 25, so a bound of 3 s) are read from run `20261116T083526Z-b2bbf4ba`, which itself ran with no bound.

The disk budget of a cell (`support/build-call/src/space.rs`) is the largest target directory a cell of the same incremental setting left, from the same table: 562731093 bytes at 25 with incremental on, 82479237 at 25 and 364947567 at 97 with it off, scaled the same way. Before each call the setup reads the free space with `df -Pk` and refuses where the free bytes and those the cell's own target already holds are under the budget plus 1 GB, as the plan's section 8 asks. The harness runs no hook before a cell, so the refusal fails the cell instead of recording it absent (ticket `the_bench_harness_has_no_hook_before_a_cell_so_a_refused_cel`). The plan's step 4 asks for the first cell's directory by `du -sb`; the hooks library reads the same byte sum (`bytes_under`) into `target_bytes` on every call instead.

## Results

Machine class `linux-x86_64-c4-container-intel` (4 cores, Intel Xeon at 2.30 GHz, 15 GB memory, the build directories on ext4 under `/tmp`). Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, LLVM 22.1.6, `cargo 1.98.0-nightly (fbb61be30 2026-05-26)`, target `x86_64-unknown-linux-gnu`, cargo's default jobs (4), checksum freshness on. Linker: the toolchain's default, whose mark in `.comment` reads `Linker: LLD 22.1.6`, reached through the default driver. LLVM backend for the fixture crates and the shipped std, rlib edges, static std, no prefer-dynamic, panic unwind, no strip, LTO off. Harness: 3 passes, 1 run per pass, batch 1, one validation seed and one determinism seed. Every worker ran in both modes, warm and cold, and `samples.csv` holds both; the report reads the warm mode alone, so every median below is the harness's warm median of three samples.

With three paired samples per arm, the smallest p the sign test can give is 0.25 (all three pairs one way, two-sided). Every paired delta quoted below therefore has an adjusted p of 0.25 or more, and none reaches significance after adjustment; the reports mark the ones whose bootstrap CI excludes zero as "YES (adj: no)". A delta here says which way three pairs went and by how much, and decides nothing until a run with more passes holds it. The arms themselves build with the harness's `opt-level=3, lto="fat", codegen-units=1`; that is the arm cdylib's profile, not the fixture's.

### build-static at 25 crates, fixture profile dev

Run `20261116T063249Z-f04daa0a-dirty`, under `results/build-static/<sweep>_n25/linux-x86_64-c4-container-intel/`. The run is marked dirty because the per-sweep `points` keys were uncommitted at the time; the bench code was as in `f04daa0a`, and no bound on a call was set (the harness's 300 s fallback). Every cell passed the hooks library's check over 32 calls.

| scenario | compiled per call | incremental on | incremental off | difference |
|---|---|---|---|---|
| clean | 25 | 16864.64 ms | 15672.26 ms | no significant difference: the paired delta's CI is -1033.91 ms to +1256.21 ms, adjusted p 1.0 |
| leaf | 2 | 3379.88 ms | 5451.34 ms | on is 2071.46 ms faster |
| mid | 7 | 693.37 ms | 5582.15 ms | on is 4874.94 ms faster |
| root | 19 | 1119.72 ms | 14310.33 ms | on is 13088.60 ms faster |

Sizes after a clean build at 25 crates (call record of the clean cell): the executable 9622376 bytes with incremental on and 9621552 off; the target directory 356158701 bytes on, of which 273746764 are incremental state, and 82479173 off.

holds for: target x86_64-unknown-linux-gnu, machine class linux-x86_64-c4-container-intel, rustc 57d06900f, LLVM 22.1.6, checksum freshness, jobs 4, linker LLD 22.1.6 by the default driver, LLVM backend, rlib edges, static std, LTO off, fixture profile dev (opt-level 1, debug 0), incremental as named, fixture seed `0x6876_626c_6669_7831` at point 25 (D 12, c 1, g 4, k 2230, three tiers, not folded, digest `0x426667db754199ed`), edit kind the constant in the body of the edited crate's `pub(crate)` edit slot function `mark` (in a contract it is also called from the default method `twice` that dependants instantiate), timed quantity whole build, harness 3 passes by 1 run by batch 1, medians of the warm mode only, no bound, commit `f04daa0a` with uncommitted bench files.

### link-static, fixture profiles dev and release

Run `20261116T083526Z-b2bbf4ba`, committed bench files, under `results/link-static/<sweep>_n<point>/linux-x86_64-c4-container-intel/`. No bound on a call was set for these cells. Every cell passed the check over 32 calls: each relink compiled the facade alone; after the first call's setup built the workspace, every later setup compiled 0 crates (`setup_compiled`), which is the plan's "switching nothing recompiles nothing"; every executable carried the mark `Linker: LLD 22.1.6`; the replay wrote an executable of the same size as the relink, and both printed the same check line on every call.

| sweep and point | relink, A | replay, B | difference | executable |
|---|---|---|---|---|
| dev, 25 | 148.57 ms | 57.84 ms | 91.26 ms (harness Δ) | 9621552 bytes |
| dev, 97 | 524.41 ms | 319.64 ms | 221.94 ms (harness Δ) | 64622048 bytes |
| release, 25 | 137.90 ms | 62.37 ms | 75.42 ms (harness Δ) | 9617080 bytes |

The target directory at dev 97 is 364818109 bytes and at release 25 is 53286866 bytes, incremental off.

holds for: the same target, machine class, toolchain, linker, backend, edge form, std form and LTO as above; incremental off; fixture profile as named; fixture points as named (97: c 7, digest `0x6053cab89e05fb7a`); scenario link with every rlib fresh; timed quantity A or B as named; harness 3 passes by 1 run by batch 1, medians of the warm mode only, no bound; commit `b2bbf4ba` with every bench file committed.

## What the default cells already show

1. Isolation holds in the static form at 25 crates: an edit to one leaf implementation compiled the implementation and the facade alone (2 crates, every one of 32 calls), and the counts after mid and root edits are exactly the generator's reach, 7 and 19. This is the first row of the plan's section 9 table: the contract and implementation split already gives the isolation, so the edge form question narrows to the link. Measured in the harness run.
2. The default link is a small part of a rebuild at 25 crates: the relink is 148.57 ms against 5451.34 ms for a non-incremental leaf rebuild and 3379.88 ms for an incremental one, and the bare replay of the link line is 57.84 ms. At 97 crates the relink is 524.41 ms and the replay 319.64 ms. Measured in the harness runs; the leaf rebuild at 97 is not yet measured, so no ratio is claimed there. The difference between relink and replay at 25, 91.26 ms, is cargo and rustc on the facade's few lines.
3. Incremental off took longer on every edit scenario at 25 crates (by 2.07 s after a leaf edit, 4.87 s after a mid edit, 13.09 s after a root edit, each the paired median delta of three passes, all three pairs the same way, adjusted p 0.25). On a clean build the harness found no significant difference. Off keeps 273746764 bytes of incremental state off the disk after a clean build. Measured in the harness run. This bears on `containers_build_with_incremental_off_and_line_tables`: off is not cheap after an edit here.
4. The mid and root rebuilds with incremental on took less than the leaf rebuild (693.37 ms and 1119.72 ms against 3379.88 ms). The timings are measured; why is not, and no reading of the generated source is offered for it. An edit kind that changes a contract's interface is a separate scenario the plan does not name, and is handed back as a question for the round.

## The guards each recorded run ran under

Runs `20261116T063249Z-f04daa0a-dirty` (build-static at 25) and `20261116T083526Z-b2bbf4ba` (link-static), and the partial runs under `results/void/`, ran under the guards as they were before the review: cargo through the rustup proxy in the fixture's directory, with only the flag, wrapper and target directory variables taken out, so a linker or profile variable, `RUSTC`, `CARGO_BUILD_JOBS`, the cargo home's configuration and a configuration above the fixture could have reached them (on this machine `/root/.cargo/config.toml`, which sets `incremental = true` under `[build]` and `debug = 0` under `[profile.dev]`; the call's own `CARGO_INCREMENTAL` overrides the first, and the fixture's `dev` profile already sets debug 0); a link held to any `Linker:` mark rather than the default linker's own, though every recorded mark reads `Linker: LLD 22.1.6`, which is the one now required; a replay's crate count written as 0 and checked against 0; no disk check; no bound on the link cells. Their numbers stand as recorded; a rerun under the current guards is what would show whether any of these mattered.

## Blockers, and what was tried

1. No run of `build-static` at 97 crates finished. The first run with bounds set (`20261116T070018Z-b2bbf4ba`) was stopped by hand during the clean cell at 97, since the tool stops a background task after two hours; its staged output is committed at `results/void/1794812418-3267/`. The run started again detached from the tool (`20261116T084342Z-a13db568`) was cut when the container restarted at 10:27 UTC on 16 November; its partial output is committed at `results/void/1794818622-27840/`, with `clean_n25` complete and nothing at 97. The push of this branch ran its lint gate while that run's clean cell at 25 was being timed, so that cell carries the disturbance. Tickets: `a_bench_run_cut_by_a_container_restart_loses_every_unfinishe` (mockspace: a cut run loses every unfinished cell) and `record_the_97_crate_point_of_build_static` (the rerun, and the 97 rows of the bound and disk tables replaced by its readings).
2. A set bound costs builds, and every worker runs in both modes. Ticket `the_bench_harness_takes_a_build_kind_of_arm_with_one_mode_an` (mockspace).
3. `cargo mock bench test <crate> --test <name>` drops the separate value. Ticket `fix_mock_bench_test_dropping_the_value_of_a_separate_test_ar` (mockspace).
4. The point at 169 crates was not built; its digest is committed and tested. Ticket `build_and_record_the_169_crate_point_of_the_build_and_link_b`.
5. A refused disk fails a cell instead of recording it absent, since the harness has no hook before a cell. Ticket `the_bench_harness_has_no_hook_before_a_cell_so_a_refused_cel` (mockspace).
6. The support crate assumes an ELF host with POSIX tools, so on macOS, Windows or a 32-bit target its link cells fail instead of being left out by declaration. Each site carries ticket `the_build_and_link_bench_assumes_an_elf_unix_host_and_fails`.

## What remains of steps 1 to 6

1. Step 5: the mid point of `build-static`, ticket `record_the_97_crate_point_of_build_static`.
2. The plan's secondary columns, ticket `record_the_time_passes_and_timings_columns_of_the_build_and`.
3. The large point, the LTO sweeps, every non-default linker, driver, backend, edge form and C compiler arm, `link-dylib`, `build-dylib`, and the folded sweep of P42 are not here and wait on the hilavitkutin-build round (steps 7 to 9), except for the LTO sweeps, which are handed back above.
