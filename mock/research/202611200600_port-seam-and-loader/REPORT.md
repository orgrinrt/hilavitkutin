# Report of job port-seam-and-loader

This is the first job of the port of hilavitkutin onto current arvo (`mock/research/202611200345_arvo-port-plan/README.md`, "Job 1"). It ran on the branch `work/port-seam-and-loader`, which sits at `b56f6978de7e7a3041c37ecaa361cce8cd3f4e58` after one merge of `origin/dev`. The design round `202611200454-x98w` is closed and its three files are in `mock/design_rounds/202611200454-x98w/`. Nothing was landed: the coordinator lands from the `ready` status, since the repository takes no landing by the helper.

## What was done

The sketch `mock/research/sketches/202611200443_two-arvo-revisions-side-by-side/` has the outcome WORKS. Cargo resolves the old arvo at `1b2796e9` and arvo `dev` at `6def8831` side by side in one workspace under renamed dependency keys, on `nightly-2026-05-28`, and one `cargo test` builds and runs a crate on each. It needs no `[patch]` section, and it also builds with the notko patch that `mock/Cargo.toml` carries, so the plan's fallback of one long branch does not apply. The sketch's control, a changed expected value, turned it red.

The five dead manifest lines are gone: `arvo-comb` in `hilavitkutin`, `arvo-bitmask` in `hilavitkutin-api`, `arvo-bits` in `hilavitkutin-linking` and in `hilavitkutin-providers`, and `arvo` in `hilavitkutin-extensions`. Each was shown dead by `cargo check -p hilavitkutin -p hilavitkutin-api -p hilavitkutin-providers -p hilavitkutin-extensions -p hilavitkutin-linking --all-targets` with the lines removed, which passed, and no source of the five names any of them. The `arvo-comb` entry under `[workspace.dependencies]` went with its last user, and `mock/Cargo.lock` and the lock of the `cross-target` probe followed.

The tool `mock/tools/old-arvo-names`, asked as `cargo mock old-arvo-names`, prints for each crate and each bench directory the count of paths of the ten crates of the old revision, with the crates at zero included, and exits clean. It counts text over the tracked `.rs` files, as the plan's first count did. Its nine tests include one that plants an old path in a tracked file of a temporary repository and asserts the count of that crate moves and no other. Run with `RUSTFLAGS="--cfg old_arvo_names_mutant_blind"` that test and three more fail, and with `old_arvo_names_mutant_boundary` the test that a longer name is no old path fails. Both flags are declared in the tool's manifest.

`hilavitkutin-linking` carries its OS code as `OsCode`, an alias over `arvo_numeral::USize<Adapt<TowardZero, Saturate>>`, entered by `from_host` and read by `host`. The pair is stack-coord's stamp on the ticket `which_rounding_and_overflow_the_linking_crate_s_oscode_alias`, for this crate only, revisited by job 3. The sentence naming `arvo::ISize` is gone from `src/error.rs`. The fixture manifest of the lto smoke test named an arvo path in no source of the fixture, so it lost its `arvo` line and the notko patch that only that arvo needed, and `trampolines_survive_lto` passes without them.

## Counts

`cargo mock old-arvo-names` on the merged tree gives `hilavitkutin-linking` at 0 (it was 4), the engine `hilavitkutin` at 482 before and after, and 702 in all (706 before). `cargo mock canon contradicts mock/crates/hilavitkutin-linking` gives 0 lines under 0 tenets both on `origin/dev` and on the branch.

## Test runs

| Run | Result |
|---|---|
| sketch, `cargo test` on the manifest | 2 passed, 1 failed with the control, 2 passed again |
| `cargo check --all-targets` on the five crates without the dead lines | passed |
| `cargo test` of `old-arvo-names` | 9 passed; blind mutant 4 failed; boundary mutant 1 failed |
| `cargo mock test -p hilavitkutin-linking` | 13 passed, including the two new in `tests/os_code.rs` |
| `cargo mock test -p hilavitkutin-extensions -p hilavitkutin-extensions-macros` | 29 passed in 21s |
| `CARGO_INCREMENTAL=0 cargo mock test`, before the dev merge | members 800 tests, 799 passed, 1 failed; lints 122 passed; benches 182 crates, 167 tests passed, 0 failed; `old-arvo-names` 9 passed; `cross-target` 6 passed, 2 failed |
| `cargo mock`, after the dev merge | exit 0, one existing `no-bare-option` warning at `windows.rs:56` |
| `cargo mock test` of linking, extensions, extensions-macros, api, providers and the engine, after the dev merge | green |

## Reds, each measured on origin/dev

`hilavitkutin-build::refused a_requirement_of_each_form_is_refused_where_unmet` failed, and `red-check` printed `branch=fail dev=fail -> dev's` at `ad6da29eba45`. The container's toolchain lacks the cranelift component its fixture asks for. The ticket is `hilavitkutin_build_refused_a_requirement_of_each_form_is_ref`.

The `cross-target` tool's tests `the_host_and_wasm_leave_the_same_stream` and `a_wasm_build_that_computes_otherwise_is_told_apart` failed on the probe's build of `hilavitkutin-api` (`E0107` at `project_items!`). The same two failed with the same error on a detached worktree of `origin/dev`. The ticket is `the_cross_target_probe_does_not_build_hilavitkutin_api_for_t`.

## What was added, and why it is here

Each addition was asked which crate is for it. `OsCode` is in `hilavitkutin-linking/src/error.rs` because the crate owns its error type and the type of what that type carries, and arvo-numeral supplies everything else, so the crate writes no numeral of its own. `tests/os_code.rs` belongs to the same crate. The tool is in `mock/tools` as the plan names it. The two new pins `arvo-numeral` and `arvo-format` in `mock/Cargo.toml` are the revision step 3 needs and are the one place the brief's sentence on `mock/Cargo.toml` (only the removed lines) is read as meaning removed lines of step 2 plus the pins of step 3. The `arvo-format` dependency is there because arvo-numeral does not re-export `Adapt` or the two policies.

The two casts to `usize` in the backends carry `lint:allow(no-bare-numeric)` with `tracked: #206`, the marker the other FFI-boundary lines of this crate already use, since `from_host` takes the host `usize` and errno and `GetLastError` are a `c_int` and a `u32`.

## What is still wrong

The design text of four crates still names a dependency their manifests no longer have, and each belongs to that crate's own round, since changing a design nukes its code: `hilavitkutin-extensions/DESIGN.md.tmpl` line 139 (arvo), `hilavitkutin-api/DESIGN.md.tmpl` lines 244 and 1761 (arvo-bitmask), `hilavitkutin/DESIGN.md.tmpl` lines 85 and 1824 and `mock/DESIGN.md.tmpl` lines 32 and 58 (arvo-comb), and `mock/agent/MAIN.md.tmpl` line 81. `hilavitkutin-linking/BACKLOG.md.tmpl` says every public signature uses arvo primitives, which still holds.

The new `os_code` tests were not shown red on a mutant: the alias's adaptation never changes a bit copy, so the tests hold the doors, not the policy pair. The pair is invisible to the crate until arithmetic or a conversion meets it.

`cargo mock check` runs the whole test suite and took over five minutes, so it was not waited on after the first time. The first of its results, the red build test, is the one above.

## Tickets

Questions filed with `ask`: `which_rounding_and_overflow_the_linking_crate_s_oscode_alias` (stack-coord, answered a), `close_the_open_row_on_growing_the_arvo_families_in_hilavitku` and `arvo_s_public_name_at_the_hilavitkutin_port` (both op's).

Problems filed: `a_new_tool_directory_with_a_manifest_and_no_source_stops_eve` (the write guard builds the lint pack and refuses while a new tool has no source), `the_pre_push_cargo_deny_check_reports_a_dropped_fetch_as_a_v` (a dropped proxy connection read as a violation, and the push passed on retry), `the_mockup_workflow_skill_says_cargo_toml_is_not_gated_and_t` (the skill and the guard disagree on a crate's `Cargo.toml`), and the two reds above.
