# Review of the consumer-setup round, pass by pass

Round `202611151157` on `work/hilavitkutin-build-sets-a-consumer-up-by-itself-and-an`. Each pass is one fresh `pr-reviewer-senior` over the branch against `dev`, read-only. What a pass found is written here as the reviewer gave it, and what was done about each finding follows the pass.

The reviewer's own reproductions ran in its scratch directory and are not committed. They count as leads. Where a finding is fixed, the test that holds the fix is the evidence.

## Pass 1, tip `5e94a769`

Verdict: merge after fixes.

The canon gate passed. The reviewer read the crate against `the_build_crate_is_a_shared_build_dependency_and_not_an_xtask_and_it_optimises_how_code_is_compiled`, `every_crate_is_no_std_with_no_allocation` and the provisional `the_build_crate_is_no_std_takes_the_profile_and_target_strings_as_parameters_and_writes_its_text_into_a_formatting_writer`. It reads a macro whose expansion is std code in the consumer's script as keeping to the provisional row, since the crate itself still compiles `no_std`, and says a second independent read of that point is owed.

### Findings

1. A stale wrapper is never written again in a wired workspace. The script's only rerun trigger is `rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER`, and where the config file names the wrapper that variable is not in cargo's own environment, so the trigger never moves. The design's account of why the second build reruns the script is wrong: it reruns because the wrapper changes the metadata hash. A deleted wrapper, or one without its executable bit, stops cargo before any script runs.
2. A path dependency that is a member of another workspace has that workspace written into, and the notice says the next build takes the flags, which is false for the build that was run.
3. The root `README.md`, lines 147 to 163, documents a `configure().run()` builder and a generated include that do not exist.
4. `DESIGN.md.tmpl` says `script!` expands to `configure` into a `String`, and in three places that every crate calls `configure`. The expansion calls `Prototype::configure` straight to standard output.
5. `tests/wiring.rs`: the expected `wrapper` and `config` fields are computed with the expressions the implementation uses, so for those two the table is not written by hand.
6. `tests/script.rs` never asserts that the second build's rustc takes the flags. Missing as well: a stale wrapper rewritten end to end, the `CARGO_HOME` branch, the no-root branch, the write-failure branch, the foreign path dependency.
7. `tests/prototype.rs` samples the release profile alone.
8. `ConfigFile::read` takes the key under any table, and reads the inline form `build = { rustc-workspace-wrapper = ".." }` as naming none.
9. The two `lint:allow(forbidden-imports)` lines are an exemption added so the work passes.
10. The locked doc changelist says fourteen build scripts become the one line; thirteen do.

### Sound

The suite ran green, 66 tests. A first build in an unwired workspace writes the wrapper with mode 0755 and the config file, and the next build's rustc gets the flags. `plan` is a `const fn`, a fetched tree is written nothing, `Cargo.toml.orig` is declined. The thirteen scripts are the one line and the frozen one is untouched. Commit subjects hold the style.

### Follow-ups the reviewer does not block on

`Plan`'s public `bool` fields and `plan`'s `flags: bool` sit under the crate's blanket allow.

## What was done after pass 1

The round was reopened through `cargo mock unlock`, twice, and the numbers below are the findings above.

1. Fixed. In its own tree the script prints `cargo::rerun-if-changed` for the wrapper and the config file (`write_watch`), and the design says what reruns it and why the second build runs the scripts again. A deleted wrapper is stated as what the script cannot help. Held by `a_wrapper_that_holds_another_text_is_written_again` in `tests/script.rs`.
2. Not fixable in a script, and stated. The first fix told the case from `RUSTC_WORKSPACE_WRAPPER`, and its test failed: cargo names no workspace wrapper to the script of a crate outside the workspace it was run in, wired or not. The script therefore sees what the dependency's own first build would show it. The design now says a path dependency's own workspace is written into and that the dependency takes no flags, and `a_path_dependency_in_another_workspace_has_that_workspace_wired` holds it. The `Wired` notice no longer promises the flags to the build that was run. The `Foreign` tree that came out of this is real for another case, a workspace wired from a config file above it, and has its own test.
3. Fixed. The root `README.md` section says what ships.
4. Fixed in `DESIGN.md.tmpl` and the generated `docs/BUILD.md`.
5. Fixed. The writes are two literal tables beside the thirty rows of what is said.
6. Fixed for the flags (a rustc that writes its arguments down shows the second build's line ending with them), the stale wrapper, the write failure and the path dependency. Not held by a test, and named in the source changelist: the `CARGO_HOME` branch and the branch where cargo gives no root.
7. Fixed. Both profile names cargo passes and an unknown one, and `tune` over every `Profile`.
8. Fixed. The key counts under `[build]`, dotted at the top, or in an inline `build` table, and under no other table.
9. Kept, and handed to stack-coord. The macro's body is std code by design, and the lint has no way to know a macro body is compiled elsewhere. The two `use` lines name the ticket `forbidden_imports_reads_the_body_of_an_exported_macro_as_the`, as the crate's `no-bare-static-str` allows name theirs. Fixing the lint is in mockspace, outside this job.
10. Fixed. Thirteen, and the frozen fourteenth named.

Two things the reopening met in the tooling, both in the ticket `a_second_cargo_mock_unlock_in_one_round_overwrites_the_depre`: a second `cargo mock unlock` in one round overwrites the first deprecated changelist, which the `changelist-immutability` lint then refuses, and `unlock` with uncommitted source leaves a tree no commit accepts.
