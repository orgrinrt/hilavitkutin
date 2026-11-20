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

The doc changelist was reopened for findings 1 to 5, the design and both changelists were revised, and the source and tests follow them.

1. `Mold` and `Bfd` go through the wrapper as `-Clink-arg=-fuse-ld=..`, the way `Wild` already did, and the trait has three writers, one for each mechanism the row names. The cfg route of `FastMath` is not changed here: it is older than the round, and the question to the maintainer that lands with the round carries it.
2. `tests/script_build.rs` runs the local build whole over a directory registry and a stand-in crate, with a control whose registry is empty. The lock is the operating system's on a file that stays, so an interrupted build holds nobody up. Only versions below the pin are removed, by `older`, which `tests/tools.rs` holds; a newer version and another pin's lock stay.
3. The generated file is placed by a hard link where there was none, so the second of two claiming scripts is refused whichever order they run in. The test builds in parallel and checks that no scratch file is left.
4. The claiming script asks `write_uncarried` first and writes the file only where it says nothing. The cranelift test fixes what the toolchain has by the sysroot it answers with, runs both branches on every machine, and reads the file.
5. The design says the driver key follows the target built, and why.
6. The root `README.md` is rewritten for the crate as it is.
7. The rustdoc of the linkers says both flags go through the wrapper.
8. The kept case of `tests/refused.rs` holds a driver in all five sets. `tests/generated.rs` asserts over prototypes that `claimed` passes, each checked by a `const` item.
9. Kept on purpose: `Workspace`'s `Ci` set holds `ExpandedLto` and the design says no build selects the ci set yet; `mock/Cargo.toml` keeps its two keys because a run started outside `mock/` reads no `.cargo/`. The slow tests and the `#72` allows go to tickets.

## Pass 2, tip `b4800774`

A fresh reviewer, with the canon read on its own before it opened this record. Verdict: merge after fixes.

The canon gate passed. The reviewer read the rows on the three mechanisms, on the builder, on what a pragma requires, on the cfg arvo takes and on the crate being a build dependency, and found the round aligned with each. Its reading and pass 1's agree now that every linker goes through the wrapper. One point it handed back as older than the round and for the maintainer: the row on mechanisms lists `FastMath` under the wrapper and the row on arvo has it as a cfg of the build script, which is what the design and the code do.

Of pass 1's findings it found 3, 5, 6, 7 and 8 addressed, 4 addressed for cranelift alone, 1 and 9 short of the question and the tickets this record said were filed, and 2 short of two things, below.

### Findings

1. The source changelist says kaski's twenty-eight scripts move on a branch that lands straight after the round, and no such branch is on the remote. kaski pins the crate from `dev`, so its next update breaks them.
2. A pragma is asked about one tool where its text names more. `Wild` writes `-Clinker=clang` and waits on wild alone. `ClangCc` puts `clang++` and `llvm-ar` under `[env]` and is asked about clang alone, so the generated file can hold a key the machine cannot follow.
3. An older version of a tool is removed without its lock being asked for, so a clone that pins it and is building it loses it.
4. The mold test runs one branch per machine, and the wild test asserts nothing where clang is missing. The mold test reads the flag and not what linked.
5. The record says a question and two tickets were filed, and none was.
6. `Workspace` holds `ExpandedLto` with no bench cell cited, against the call on defaults.
7. `includes_generated` takes `sub/hilavitkutin-build.toml`, which is another file, and a test asserted that.
8. A local build that fails inside `script!` is followed by an error telling the author to use `script!`.
9. The count of pragmas in the tests is kept by hand beside a list kept by hand.

### Suite

115 tests passed, the slowest 6.5 s. No tautology and no test that asserts nothing. Missing: the wait on a held tool lock, the refusal of a missing package linker on a machine that has it, wild without clang, and the driver key at a second target.

### Not checked

Whether clang's `--ld-path` finds a bare `wild` on the path, whether `-Cprefer-dynamic` belongs on the dylib or on its dependents, hosts other than Linux, and that no design of the repository declares its canon.

### What was done

The doc changelist was reopened a third time, for finding 2.

1. The kaski branch is written in a worktree, twenty-eight scripts to `configure_as::<Deterministic, _, _>`, and cannot build or be pushed green before this round is on `dev`, since kaski takes the crate from there by git. The source changelist says so. It lands by kaski's flow straight after, and the coordinator of the stack is told the contract before this round lands.
2. `Pragma::NEEDS` is a list of rows where `TOOL` was one. `Wild` needs wild and clang, `GccCc` gcc, g++ and ar, `ClangCc` clang, clang++ and llvm-ar, and the table has the four new rows. The script provisions every need, and `write_uncarried` asks about each.
3. Before an older version is removed the script tries its lock, and leaves the version where the lock is held. `tests/script_build.rs` holds one and finds its directory left. A clone that only links through an older version at that moment holds no lock; that is in the follow-up ticket.
4. The tests hand a build a path of their own. The refusal of mold, of clang under `Wild` and of llvm-ar under `ClangCc` each run on every machine, over a path with the program left out. Where the machine has mold the test reads mold's mark in the binary of the second build and its absence in the first.
5. Filed: the question `which_canon_row_says_where_fastmath_takes_effect_and_is_the`, which carries the builder's spelling too; the task `the_lint_allows_of_hilavitkutin_build_cite_a_tracker_number`; the task `follow_ups_of_the_pragma_round_s_second_review_pass_in_hilav`. No test is slow any more, so no ticket for one.
6. Filed as the question `does_the_workspace_prototype_keep_expandedlto_before_a_bench`. The prototype is unchanged until it is answered.
7. `includes_generated` takes the bare name in quotes alone, and the test line moved to the cases it does not take.
8. The line says that a `script!` with no path says why in its own error, and that no other script can name the pragma.
9. Not changed: the crate has no public list of every pragma to hold the count to, and adding one is a surface of its own. It is in the follow-up ticket, with the test of the wait on a held lock and of the driver key at a second target.

## Pass 3, tip `4d093273`

Verdict: MERGE AFTER FIXES. The third and last pass. The canon gate passed, read on its own before this record: the rows on a pragma's requirements, on pragmas per profile by a builder, on the three mechanisms and on the crate being `no_std` agree with the round, and the two open points are with the maintainer as question tickets. That is the second read of the canon that agrees.

### Of pass 2's findings

Addressed: 2, 3 in source and test, 5, 6, 7, 8. Partly: 4, since the two tests of mold and of wild linking still return early on a machine without the program. Not addressed: 1, the kaski branch, which has no commit. Left with a ticket: 9.

### Findings

1. The design said every older version of a tool is removed, and the source keeps one whose lock is held. Code ahead of its design.
2. llvm-ar is proved by `LLVM version` on the first line it prints. An LLVM built with no vendor name prints `LLVM (http://llvm.org/):` first and the version on the second, so there `ClangCc` is refused while llvm-ar is installed. The test's table held a first line no llvm-ar prints.
3. The root readme says nothing has to be installed by hand for `Wild`, which needs clang since pass 2.
4. The kaski migration is uncommitted and edits a design file of kaski outside a round.
5. Eighty-nine allows cite a tracker number, where the ticket now has an id.
6. A test calls the lock file of the version it removes another pin's, and lock files of removed versions are never removed, which the design did not say.
7. The array of refused tools has sixteen places by a comment, where a set can name forty-eight.
8. Two commit subjects are statements and not imperatives.

### Suite

119 tests passed in 20 s. No tautology. The defects are the ones above, and two edge cases ticketed where a test was owed: the wait on a held lock, and the driver key at a second target.

### Not checked

The bodies of the test binaries that did not change since pass 2, whether the rendered document matches its template, cargo's include on a stable toolchain, hosts other than Linux, and `Proof::mark`, which nothing in the crate reads.

### What was done

The doc changelist was reopened a fourth time, for findings 1, 2 and 6. This was the last pass, so what is not fixed below lands as follow-ups in the ticket `follow_ups_of_the_pragma_round_s_second_review_pass_in_hilav`.

1. The design says an older version whose lock cannot be taken at once stays, and that no lock file is removed and why.
2. The script reads the first two lines a tool prints, joined by a space, and the design says so. The upstream text was read at `llvm/lib/Support/CommandLine.cpp`, tag `llvmorg-19.1.0`, `VersionPrinter::print`: with no `PACKAGE_VENDOR` it prints `LLVM (http://llvm.org/):`, a newline and two spaces, then the name and version. `tests/tools.rs` holds that text joined and the first line alone, which is refused. `tests/script_pragmas.rs` builds under a stand-in llvm-ar that prints the two lines and finds the key written, beside one whose two lines hold no version, which is refused.
3. The readme says `Wild` links through clang and that a machine without it is told so.
4. Not changed before the landing. kaski's commit gate builds its lints against this crate from `dev`, so the branch cannot be committed there until this round has landed, and it lands straight after. Its design edit is checked against kaski's own round rules then.
5. Follow-up.
6. The test names the file for what it is and says why it stays.
7. The array has one place per row of `TOOLS`.
8. Left as pushed.

Also follow-ups: the two early returns of pass 2's finding 4, the two edge-case tests, and the wording of the error for a missing path in a script written by hand.
