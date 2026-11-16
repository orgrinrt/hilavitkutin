# What was done about the review of the first cells

The review of `work/bench-build-and-link` at `34cedee4` raised twelve findings. Each is listed below with what was done about it and the commit that did it. Three commits carry the fixes: `9316a8ec` for the fixture's tests, `3114349f` for the call support crate and its tests, and `a32256f3` for `NOTES.md`. No recorded result directory was edited, and no bench was run.

1. The tautological digest test, which built one tree twice and compared it to itself, is deleted (`9316a8ec`).

2. A replay runs no cargo, so its record line now carries `-` for the compiled and expected counts, and `check` refuses a replay line that carries a count (`3114349f`, test `record.rs`). The sentence saying each replay compiled nothing is gone from NOTES (`a32256f3`).

3. The default arm is now held to `Linker: LLD <version>`, where the version is the toolchain's LLVM version, read at run time from `rustc -vV` and the host target spec. A mark that only starts the same way, another linker's mark, a missing mark, and an arm naming a linker with no known mark are each refused, with negative tests in `marks.rs`. `linker_mark` no longer special-cases Wild, so it and `check` agree (`3114349f`).

4. Every cargo call now starts from a cleared environment plus `PATH`, uses a fresh `CARGO_HOME` with no configuration, and runs from the filesystem root after refusing a configuration found there. `environment.rs` sets a hostile linker variable, `RUSTC`, `CARGO_BUILD_JOBS`, a profile variable, a cargo home configuration and a configuration above the fixture. It shows that the honest relink passes, and that each of those, once let through to the same command, breaks it (`3114349f`). The NOTES line about the environment is corrected (`a32256f3`).

5. NOTES no longer claims anything about incremental state on a clean build. It states that the smallest two-sided sign-test p with three pairs is 0.25, and both predicates now say they are medians of the warm mode only (`a32256f3`).

6. The run that the restart cut is over, and NOTES now says so. It cites `a_bench_run_cut_by_a_container_restart_loses_every_unfinishe` and `the_bench_harness_takes_a_build_kind_of_arm_with_one_mode_an`, and each other blocker has its own ticket: `record_the_97_crate_point_of_build_static`, `build_and_record_the_169_crate_point_of_the_build_and_link_b`, `record_the_time_passes_and_timings_columns_of_the_build_and`, `the_build_and_link_bench_assumes_an_elf_unix_host_and_fails` and `the_bench_harness_has_no_hook_before_a_cell_so_a_refused_cel` (`a32256f3`).

7. `lib.rs` is split along its seam: the vocabulary and the cell directories stay in `lib.rs`, and the call, toolchain, linker, measured-row and disk-space parts move to their own modules (`3114349f`).

8. The `Command::new("true")` placeholder is gone, replaced by an optional command. The ELF64 reader, the facade name test, the `df` reading, the `PATH`-only allow-list and a host whose default linker is not the toolchain's own lld carry `FIXME(the_build_and_link_bench_assumes_an_elf_unix_host_and_fails)`. They are marked rather than gated, since a gate per target would be a per-target table, which file 58 forbids (`3114349f`).

9. Each call refuses before building anything when its disk, counting what its own target directory already holds, cannot take the measured budget for its point plus a 1 GiB margin, with a test in `space.rs`. The refusal sits in the call's setup, not before each group, because the harness has no hook before a cell; that gap carries `FIXME(the_bench_harness_has_no_hook_before_a_cell_so_a_refused_cel)`. Every sweep declared in `build-static` and `link-static` now has its bound from a measured row, held by `bounds.rs` (`3114349f`).

10. The reach tests loop over every point in `POINTS` (`9316a8ec`). The bounds test checks laws instead of one value, the replay `KeepOutput` control asserts its reason, and `lock.rs` holds cargo's lock files and shows that a call which waited is recorded as `waited` and refused (`3114349f`).

11. The explanation in finding 4 of NOTES, which nothing measured, is dropped (`a32256f3`).

12. The pushed subjects of `b2bbf4ba` and `a13db568` lack backticks around their identifiers. Pushed history is not rewritten, so this note is the whole of what was done.

The incremental setting is left as it was, and NOTES hands it to the design round. For each recorded run, NOTES now says which guards it ran under.

## The second round

The second review, of `fd995314`, raised eight findings. Six are fixed on this branch, and two are carried by tickets, each cited by a FIXME at its sites. The support crates' tests pass: `build-call` 35, `build-fixture` 9.

1. The test that compared the default mark with the format string that builds it is deleted (`4b0bb1b3`). The real check stays: a real link's mark is compared with the worked-out mark in `controls.rs`.

2. Both benches now declare six passes, which lets the sign test reach p = 0.031. `tests/passes.rs` checks both `bench.toml` files and shows that three passes cannot reach significance (`b3421768`). In NOTES, every delta is now called a three-pass median delta that cannot reach significance. NOTES also says that `containers_build_with_incremental_off_and_line_tables` is a workspace ticket from kaski's registry, not a canon row.

3. The default arm's mark is now `Linker: ` followed by the version line that the toolchain's own `rust-lld` (under `<sysroot>/lib/rustlib/<host>/bin/`) prints for `--version`. The LLVM commit is part of that line. The mark is compared exactly, so the following are refused:
   - a system lld of the same version;
   - the same version with no build note;
   - a second linker's string joined after the right one.

   `marks.rs` holds each of these (`4b0bb1b3`).

4. A replay keeps its link line in one file per profile. It refuses a line whose `-o` resolves outside `target/<profile dir>` or climbs with `..`. `linkline.rs` covers both the failing and the passing cases (`704cea1f`).

5. Left, ticket `build_call_s_target_spec_reading_linker_mark_and_cargo_flags`. The hand-read target spec, the default linker's mark and the cargo flags are build configuration, and they move into hilavitkutin-build with its pragma round (job ticket `hilavitkutin_build_carries_linker_backend_edge_form_c` on kaski `dev`). The FIXMEs are in `toolchain.rs` and `cargo.rs`.

6. Left, ticket `a_link_static_arm_carries_its_linker_and_quantity_in_a_type`. The arm's linker and quantity are still parsed back from its name. The FIXMEs are at `linker::named` and `record::is_replay`.

7. The `.comment` reader now checks every offset and size against the file and returns an error instead of panicking. `elf.rs` cuts a small ELF file at every length and points each offset it trusts outside the file (`ce53f3e4`).

8. Every test that builds removes its bench's state, both at its start and when it ends. The removal also runs when the test panics, which `cleanup.rs` shows. The 430 MB of earlier `test-*` directories are removed (`7e7cfe46`).

NOTES now says near its top that the fixture's `dev` profile is not hilavitkutin's `dev`, as the row `there_are_five_build_profiles_dev_dev_opt_release_profiling_and_ci` defines it. So the numbers measure the toolchain's default route, not a profile hilavitkutin ships.
