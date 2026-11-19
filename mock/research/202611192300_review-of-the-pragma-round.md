# Review of the pragma round, pass by pass

Round `202611192054-xugn` on `work/hilavitkutin-build-sets-a-consumer-up-by-itself-and-an`. Each pass is one fresh `pr-reviewer-senior` over the branch against `dev`, read-only. What a pass found is written here as the reviewer gave it, and what was done about each finding follows the pass.

The reviewer's own reproductions ran in its scratch directory and are not committed. They count as leads. Where a finding is fixed, the test that holds the fix is the evidence.

## Pass 1, tip `af3cd109`

Verdict: do not merge.

The reviewer read the round against three ratified rows, `a_pragma_declares_the_pragmas_it_requires_through_the_pragma_trait`, `build_behaviour_is_set_through_pragmas_enabled_per_profile_by_a_builder` and `a_pragma_takes_effect_by_generated_config_by_the_compiler_wrapper_or_by_a_post_build_hook`. The canon gate failed at one point, finding 1, and passed elsewhere. It is the first read of that point and says a second independent one is owed. The test gate failed on missing fundamentals and on setup that helps, findings 2 and 3. The suite ran green, 110 tests in 395 s, and the commit lints passed.

### Findings

1. The link-argument route is a fourth mechanism, and the row names three: generated configuration, the compiler wrapper, a post-build hook. `Mold` and `Bfd` print `cargo::rustc-link-arg` from the script. The topic quotes the row and then calls the canon silent on linkers without squaring the new route with it. `Wild` already goes through the wrapper, so the other two can. The cfg route of `FastMath`, which the row puts under the wrapper, is older than this round, and this round writes it into the trait as a route.
2. The local build is held by no test: the lock, the wait, `cargo install` and the removal of other versions. The source changelist says so. A local directory registry with a stub crate runs all of it offline. Two defects on reading. A build interrupted during the minutes of the first install leaves the lock directory, and every later build on the machine waits 45 minutes and fails. The removal takes every `wild-*` beside the new directory, newer versions and other pins' locks among them, so two clones that pin different versions delete each other's tool.
3. Two crates that both claim the workspace are not refused in a parallel first build: both find no file and both move theirs into place. The test hides it with `-j 1`. The config file is placed by a hard link, which refuses where a file is there, and the generated file is not.
4. A pragma the script refuses still has its keys written to the generated file, after the `cargo::error`. `Cranelift` without the component leaves `profile.*.codegen-backend` in a file cargo reads before any script runs. The reviewer expects every later build to fail before a script runs and did not reproduce it. The cranelift test runs one build, never reads the file, and asserts one branch per machine.
5. The `linker` key of a link driver follows the last target built: builds for two targets in turn write the file again each time. Neither the design nor a test says so.
6. The root `README.md` tells readers to call `configure_with`, says `ExpandedLto` writes nothing yet, and names neither `, workspace` nor the include line.
7. The crate's rustdoc says `Wild` writes a link argument. Both of its flags go through the wrapper.
8. Two tests cannot catch what they claim. `tests/refused.rs` says the kept case holds a driver in every profile and it holds none, so a `claimed` that refused every driver would pass. `tests/generated.rs` asserts text over two prototypes `claimed` refuses, states that cannot occur.
9. Drift. `Workspace`'s `Ci` set holds `ExpandedLto` and no key is written for `ci`. `mock/Cargo.toml` keeps `lto` and `codegen-units` beside the generated file, the same setting twice. The source added 92 `lint:allow` lines citing `#72`, which the reviewer did not open. Four tests take 50 s or more, the slowest 215 s.

### Sound

`checked`, `crate_reach` and `claimed` are `const` assertions, and `tests/refused.rs` builds scratch crates for the broken and the kept sets and asserts `E0080`. The order of a verdict and the handling of `Free` are right. A shared key is written once. `claim` reads a first line that is not this crate's as somebody else's. The writes of the wrapper and of `config.toml` cannot interleave, and `config.toml` is never written over. The source changelist matches the source. The commit messages carry no byline, and the crate has no em-dash.

### Not checked

Whether clang's `--ld-path` takes the bare `wild` a tool on the path is handed as, how `cargo install` behaves under `--offline` or a jobserver, ticket `#72`, and the findings of the sketches other than `202611192125`.

### What was done

The doc changelist was reopened for findings 1 to 5. The rest of this section is written when the fixes are in.
