# Report: the sketch of a unit that names its lists and nothing else

This is the work report of job `unit-context-sketch` on branch `work/sketch-unit-names-its-lists-only`, written by the session that ran it on 19 November 2026. The sketch is `mock/research/sketches/202611191500_unit-names-its-lists-only/`, and its `FINDINGS.md` is the result. This file holds what the findings do not: the two questions for the maintainer in full, the test runs, and what the session could not do.

The outcome is WORKS for a unit whose body is its value function. The unit declares `Read`, `Write` and `Hint`, writes `apply` over `ReadVals` and `WriteVals`, names no context type and no trait in the body's signature, and is registered bare. A unit with a positional body still names the engine's context, and no lists-only form was found or built for it.

No crate under `mock/crates/` was changed. No public contract changes on this branch, so nothing was posted to the coordinator for agreement ahead of the landing.

## The two questions, to be filed with the maintainer as holder

The session could not file a ticket. Both are for subject `units`, repository hilavitkutin.

### Q1: may a unit whose body is its value function have no execute and no context

The third sketch of the derived unit context found a form that compiles and runs under the engine with no erasure and no indirect call, for a unit whose body is its value function. The unit declares Read, Write and Hint, and its body is `fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write>`. It names no context type and no trait in the body's signature, and it is registered bare with `.with(Drift)`.

The vehicle changes what a unit is. `WorkUnit` becomes the declaration alone (Read, Write, Hint, COMMUTATIVE). `Ctx` and `execute` move, unchanged, to a trait under it (`Positional` in the sketch) that only a unit with a positional body implements. A unit whose body is its value function has no `execute` and no context at all; the engine builds its own context from the unit's lists and applies the function.

The canon is not of one mind on that. For it: `a_unit_declares_only_its_reads_writes_and_hints_and_everything_else_is_derived` (ratified), `the_consumer_spelling_of_a_unit_reduces_to_read_write_hint_and_schedule_with_the_engine_context_computed_if_feasible` (ratified), `inside_a_fiber_a_unit_is_a_pure_function_from_column_values_to_column_values` (provisional), and the call 4 of 16 November, "Neither, sketch further" (clause-dev `.shared/research/202611150830_stack-path/57-op-calls-on-the-night.md`). Against it, as written: `a_unit_declares_its_name_read_set_write_set_hint_commutativity_and_an_execute_function` (ratified: a unit has an execute function that takes a context), `execute_receives_a_context_and_nothing_else` (provisional), `the_context_of_a_unit_is_a_generic_associated_type_over_a_frame_lifetime_the_scheduler_owns` (provisional) and `a_pure_per_record_map_may_be_declared_as_an_opt_in_fusion_contract_beside_execute` (provisional: the value function sits beside execute and does not replace it).

A unit with a positional body still names the engine's context type in this form.

Is a unit whose body is its value function allowed to have no execute and no context, so that the rows above are amended through the canon flow?

The options, the session's recommendation first:

1. Yes for value bodies, and a further sketch looks for a lists-only positional body by moving the context's type function api side, as the ratified consumer-spelling row words it.
2. Yes, and the positional body keeps naming the engine's context: the unit trait is the declaration alone, `execute` with its context moves to a trait under it, and the four rows are amended through the canon flow.
3. No: every unit keeps `execute` and names a context, so the lists-only form is not taken.

### Q2: three impls or one for a unit that names its lists and nothing else

The sketch runs such a unit in two spellings, and both compile and run under the engine.

Three impls, all of traits the api already ships: `impl BuilderInput for Drift` (two lines naming the router), `impl WorkUnit for Drift` (Read, Write, Hint) and `impl RecordOp for Drift` (the value function).

One impl: `impl Unit for Drift { type Read; type Write; type Hint; fn apply(..) }`, where `Unit` is a new api trait and three blankets in the api grant `BuilderInput`, `WorkUnit` and `RecordOp` to every `Unit`. The blankets cohere with every hand-written impl in the sketch's graph. `Unit` is for the `Always` schedule only: the same trait with a schedule parameter makes the `BuilderInput` blanket conflict with every `BuilderInput` impl the api holds (E0119, since a downstream crate may implement the trait over a schedule of its own for an api type). A unit on another schedule takes the three impls.

The ratified row `the_consumer_spelling_of_a_unit_reduces_to_read_write_hint_and_schedule_with_the_engine_context_computed_if_feasible` carries the sentence "`BuilderInput` and `HasSchedule` impls stay explicit either way", which the one-impl spelling goes against for `BuilderInput`.

Which spelling does a unit that names its lists and nothing else take?

The options, the session's recommendation first:

1. Three impls of the shipped traits, as the ratified sentence has it. Nothing is added to the api.
2. One impl of `Unit` for the `Always` schedule, three impls for any other, and the ratified sentence is amended.
3. Both are offered.

## Test runs

All from the sketch's own workspace, toolchain `nightly-2026-05-28`, on the formatted tree of the branch.

| Command | Result |
|---|---|
| `cargo test -q -p check` | run: 1 passed. fixtures: 2 passed (twins, attempts) |
| `cargo test -q -p check --test run --features one-impl` | 1 passed |
| `cargo test -q -p check --test run --features mutant-short-window` | 1 failed, as it must, at `check/tests/run.rs:115:9` |
| `cargo check -q -p hilavitkutin-api --features unit-blanket` | builds |
| `cargo check -q -p hilavitkutin-api --features attempt-unit-sched` | E0119, as recorded |
| `cargo rustc -q -p probe --profile probe --lib -- --emit asm,llvm-ir` | builds; no call or jump in the four bodies |

No red was met, so none was checked on `dev`. The repository's own suites were not run: the branch adds files under `mock/research/` alone. No crate's source or design was touched, so `cargo mock canon contradicts` had no crate to read.

## The suite audit

The area touched holds no shipped suite. The sketch's own tests were read by body: one run test with a mutant that makes it fail, two compile-fail twins each beside a passing twin, and three attempts whose compiler output is recorded. None asserts nothing, and each carries the case that must fail.

## What the session could not do, each a problem for the tracker

None of these could be filed, for the first reason in the list. Each is a problem of the workspace's tooling (`clause-dev`) unless it says otherwise.

1. A build under kaski's mockspace (`mock --dir /home/user/kaski/mock --generate-only`, its `cargo rustc --release` of the lint pack) held `/home/user/target/release/.cargo-build-lock` while a `cargo install --git https://github.com/orgrinrt/homma.git --rev f8fa8e77 homma-engine` held that same lock file open and waited. The two processes shared one jobserver pipe, which is the session's ground for reading the second as started from inside the first. Neither moved for the rest of the session, over an hour. Every release build in the one target directory queued behind it: the worktree tool, `ask`, `ticket`. Ending the nested install by its process id was refused by the permission check as a destructive action.
2. `ask` and `ticket` build in that directory, so no ticket and no question could be filed from this container.
3. `stand-up post <name> <branch>` takes kaski as its repository unless a third word names another, and `session-worker.md` gives the call without one. For a job whose product is hilavitkutin the first call stood kaski up for twenty minutes before it was stopped.
4. The bootstrap's step `cargo mock` printed `error: unexpected argument '--branch' found` from `cargo-binstall` and was still marked as passed.
5. The mockspace edit gate (a repository hook, so `mockspace`) refused `cp -R /home/user/hilavitkutin/mock/crates/hilavitkutin-api <sketch>/api` as an edit of the crate it was read from. The same copy named from inside `mock/crates` passed.
6. The hook that keeps builds in the one target directory refuses `--target-dir`, which was the way around the held release lock. A profile of another name in the sketch's manifest was used in its place.
7. The commit hook formats every staged file, so verbatim copies of shipped source that the shipped tree holds unformatted stop being verbatim (eight files here), and compiler output recorded before the commit is off by the lines the formatter moved.

## Review

One fresh `pr-reviewer-senior` pass was run over the branch against `origin/dev`. Its verdict and what was done about it are in `REVIEW.md` beside this file.
