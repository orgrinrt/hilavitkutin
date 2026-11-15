Source: kaski channel (coord/kaski of clause-dev), att/20261113T222221166661801-hv-api-revision-fa0d13/report-hv-api-revision.md, first posted 2026-11-13 22:22 UTC, read 2026-11-15.

# hv-api-revision: round one is closed, the contract is in the source

Branch `work/api-revision` of `orgrinrt/hilavitkutin`, head `eeb7f5557a1265f7ceae2aa6c1c3deb424d99e67`, pushed, with `dev` at `271909e` merged in. Round `202611130617-h6mb` is closed: both changelists are locked and archived under `mock/design_rounds/202611130617-h6mb/`. The close took the parked topic `202607200100_topic.b1-cap-lifting-completion.md` with it, and it is put back flat under its own name before the push, since no change of this round touched what it is about. A second, small round, `202611131924`, is closed after it and fixes the bench crates the first one broke (below). `cargo mock` passes with every lint and `docs/` is regenerated from the locked designs. Nothing was pushed to `dev` and no pull request was opened.

The order of 16:10 is done as written. Change 1 is built on the record: `Buffered<T, M = Snapshot>` with a frozen read and a live write, the swap done by the engine after the close. The bench of the swap of a buffered column stands beside the resource swap arms and its numbers are below.

## After review 1

Head `364a77235dbf010ae819221fdba5570550fb0cc8`, pushed. Review 1 at `eeb7f555` said do not merge, with five findings. Findings 1, 3, 4 and 5 are fixed in a third small round, `202611132014`, closed, with the tests committed ahead of the fix where there was one to make. Finding 2 is not built and waits for the stamped shape.

Finding 1, the reach boundary on the unit-outer path. The design text now says the boundary holds on every path that hands the record range to more than one core, and the source keeps it with the barrier the finding offered as its second fix, not with a refusal at build, which would have narrowed what a carrier with an accumulator may hold. `plan::reach_cut` folds where a consumer unit reads a plain column with a reach behind a writer, and `worker_accum_unit_outer` walks the carrier in segments cut there, every core crossing `waist_barrier` between two segments, a core with no slice included. A carrier with no such reader walks as it did. `tests/reach_across_cores.rs` holds it: the probe of the review, red on the commit before the fix with record 4999 read before its writer landed, green after; the same carrier on the phase loop as its control; three passes over the one barrier; two records on four workers and a pass with no records, both shown hanging when only the cores that run cross the barrier; and the fold and the segment walk on their own, each shown failing under a mutant. A writer listed behind a reach reader of its column is refused at `build` as `NonTopologicalRegistration`, which a test holds, so the boundary ahead of the reader is the only one.

Finding 3, the surviving mutant of the buffered write. No source change. `a_value_equal_to_the_frozen_one_still_lands_on_the_live_side` runs under both buffer modes and was shown failing in both under the mutant of the review.

Finding 4, the dimensions of the reach rule. `ReachRule` takes the plan dimensions and `Buildable::RULES` hands it the ones its `plan` uses. With the fold as it is, a bundle wider than the capacity it is folded at indexes past the mask arrays and fails the build, so nothing got past the rule by being wide; the parameter closes the drift for when `build` plans at other dimensions. A departure: the type and `HOLDS` are public, so that a `compile_fail` doctest can force the rule at dimensions with room for one unit. It was shown failing with the rule taking the parameter and folding at the default. The design text for the public type is owed in the next doc phase, ticket `hilavitkutin_s_engine_design_does_not_say_the_reach_rule_typ` (bdf787ead0b5).

Finding 5. `Flip::turned` carries a `// SAFETY:` comment naming the one writer, the window between `frame_await_done` and the next `frame_publish`, and the per-core copy the unit-outer path reads.

Finding 2, what the first pass reads from the frozen side of a buffered column: nothing built.

The engine crate's whole suite and its doctests are green on the fixed tree, and `cargo mock` is green with `docs/` regenerated. The mask folds `masks_of` and `reaches_of` moved unchanged out of `plan/grouping.rs` into `plan/unit_masks.rs`, which takes that file under where it stood.

The providers `smoke` link failure is the trunk's: `red-check` says `branch=fail dev=fail` with `dev` at `271909e`. Its standing ticket is `the_providers_smoke_test_does_not_link_on_dev_for_want_of_th`, bumped. I filed `hilavitkutin_providers_smoke_test_fails_to_link_on_dev` (1ca84946f5c6) for it before reading the record; it is a duplicate, for coord to close. Where this report says 51630fc78396 below, the standing ticket above is meant.

A sketch committed between the rounds, `mock/research/sketches/202611132010_oracle-over-record-op`, is groundwork for round two: a one-thread runner over `RecordOp::apply` alone, outcome WORKS.

## Round two, the instruments

Head `db9d2094e9603b94856d8dbe5397cc8ef1da91ab`, pushed. Round `202611132054` is closed at `fe108a99` with both changelists locked, and the parked topic is flat again. `cargo mock` is green and `docs/` is regenerated, with `docs/CONFORMANCE.md` and `docs/FROZEN_AUTHORS.md` new. Finding 2 of review 1 is not in this round: its shape came from op while the round was open, and it is built next as a round of its own.

`cargo mock test` over every tree after the close: the lints' sixty tests pass. The tool's tree showed one red of this round's, fixed in the one commit after the close: the two tests that build the probe, as it is and skewed, ran at once, cargo puts both wasm builds at one path, and the skewed test ran the other's module. They now build one at a time. The other reds are the trunk's. The workspace tree stops at the link of the persistence crate's `archive_str`, so that pass runs no member's tests, and the suites of the touched crates were run one by one. Ten bench crates fail on the two causes named under "Not green, and whose". The `frame_wide` bench's `checks_gate` asks for arms built in release beside it and is red where they are not (ticket 5dc681e82128).

Where the instruments live. `hilavitkutin-conformance` is one crate with a module each for the reference runner (`Oracle`), the digest (`Fold`) and the laws (`model`, `split`, `isolation`), written against `hilavitkutin-api` alone, with a forbidden-imports rule that keeps the engine out of it. `hilavitkutin-frozen-authors` is the consumer's crate: nine units in the value form over eleven stores, a hand model of a frame of them, and the one function that writes a frame down as words. The three frozen lints are in `mock/lints/` and the cross-target check is `mock/tools/cross-target/`, the repository's first tool. No crate is named `-contracts`.

What the brief asked for among the units. `Spread` reads its neighbours on two axes of a plane. The trigger kinds are an accumulator append (`Cross`), a fired virtual with a unit scheduled on it (`Alarm`, `Tally`), and the changed mark under `Written` (`Tally`) and under `Differs` (`Smooth`, `Crest`). Both buffer modes are in, and `Warm` and `Fade` are a carrier `run_fused` admits.

The mutants are seven ways of keeping a frame wrongly, shipped beside the faithful one, and each is held to the law it has to fail and to the word it first fails at. The ledger lint fails when a frozen file's content is not its ledger hash, when a file has no line, and when a count of breaks is not the number recorded; its tests show each.

The scheduler against the reference, on the frozen authors, in `mock/crates/hilavitkutin-frozen-authors/tests/authors.rs`. Eighteen tests pass: the reference runner keeps the hand model with all nine units in both visitings, and each entry keeps the reference's frames on the units it computes rightly. Nine tests are committed ignored, each naming its row of `mock/catalogue.md`, and each was run with `--include-ignored` and seen failing. They are five disagreements, none fixed and no law weakened:

1. `run` rests a unit that reads a plain column it writes itself from the second frame on, unless the host marks the column. `run_keeps_the_frames_where_the_host_marks_nothing`, frame 2, a position 1 where the reference has 2. Ticket 8c7cc9768579, bumped.
2. A unit scheduled on a virtual takes no edge from the unit that fires it, so `run` rests it in the frame its trigger fires. `run_runs_the_triggered_unit_in_the_frame_its_trigger_fires`, frame 6, alarms 0 where the reference has 1. Ticket 4d72d32b6c78.
3. The same missing edge in the phase loop of `run_parallel` on more than one worker: the gate is read on one core while another fires, unsynchronised, and the outcome differs from run to run. `the_phase_loop_runs_the_triggered_unit_on_two_workers` and `..._on_four_workers`, eight runs each. Ticket 4d72d32b6c78.
4. Beside an accumulator `run_parallel` walks the carrier without asking the trigger gate, so the triggered unit runs every frame, and a fire is stamped on a per-core copy the host never reads. `run_parallel_gates_the_triggered_unit_beside_an_accumulator_on_one_worker`, on two and on four, and `a_fire_beside_an_accumulator_reaches_the_host`. Ticket 2518568d1e3b.
5. `run_fused` leaves the column between two fused units unwritten where `run` and the reference write it. `the_fused_entry_stores_the_column_between_the_two_units`. Ticket 59906ce22012. The engine's design says so of the fused entry, and op's ruling of 6 June that the entries are one engine says they compute the same.

Disagreements 2 to 4 mean no entry of the scheduler runs a unit scheduled on a virtual rightly on more than one worker today, and `run` does only where the host marks an input.

The S1 property on the real api: the frozen authors compile and pass unchanged under the reference runner with all nine units. Under the scheduler they pass unchanged on the subsets each entry computes rightly, and fail as above on the full set. So the property holds for the reference and holds for the scheduler only short of the five disagreements.

The cross-target check: `cargo mock cross-target wasm32-unknown-unknown` builds a probe for the host and for wasm, runs the wasm build under node, and reports the same eight digests from both. Its tests run the path as it is and with a skewed wasm build, which has to be told apart at frame 8, and is.

Departures from the brief, each in the changelists with its reason. One crate for the instruments where the map of the earlier branch has two. Three of that branch's four frozen lints: the one refusing a turbofish would refuse what the api's value form documents. The scheduler's test sits in the frozen crate's suite, since the engine cannot take that crate as a dev-dependency without a cycle. No new compile-fail twin: the reach twin is the pair of doctests on `Window`, two writers of a buffered column are not refused by the api's design, and a unit still names the engine's `CtxFor`. The cross-target probe takes the four frozen files by `#[path]` and is a workspace root of its own, where the design's sentence has it build "the frozen units": the frozen crate takes the engine's default platform tier, which does not compile for wasm, and cargo would turn both tiers on for a second crate in its graph (ticket 3b60507373b9). The tool takes the target as an argument where the design says "for the host and for wasm"; wasm is the one target it can run.

Departures of the source from the locked designs: `World` is a public trait the conformance design does not name, the design's private `Cell` is a method of `Kept`, and `Fold` has `words` beside `word` (ticket 86dbab62c217 for the design text). `Lent`'s constructors for a buffered column do not assert the two sides equal in length; `Oracle::new` refuses a short side. The ledger lint has no dependency arm and no line for `tests/`. `model.rs` carries clippy's `is_multiple_of` from the commit hook.

Tickets of this round. Filed: 4d72d32b6c78, 2518568d1e3b, 59906ce22012, 3b60507373b9 (the engine's default tier does not build on wasm32 and a unit crate takes a tier with it), 86dbab62c217, c5bffc5b1e31 (a doc phase commit that designs a new crate cannot pass the push gate until the source exists). Bumped: 8c7cc9768579, 429c63bdea5c. Two ids this report and the designs use are shas of the commits that filed the tickets and not ticket ids, which the ticket tool resolves: `8e2e2326d588` is 8c7cc9768579 and `3df7466ff25a` is 429c63bdea5c. The three markers that cite the first are corrected in the next doc phase that opens those files. Closed by this round's design text: bdf787ead0b5.

## What the source now holds

The api crate carries the access sets on the shared list crate, `Member` with the three wrappers `Reach`, `Written` and `Differs`, the `Window`, the buffered column with its two sealed modes, the value function (`map` on `EachApi`, `RecordOp` as its named form, `rec!`), positional accessors that follow the member, and accumulator appends that are safe. The derive writes `ResourceRead` beside the footprint.

The engine walks members where it walked markers. It runs the value function, keeps a buffered column as one binding with two sides and brings a changed live side in at the boundary of every run path, orders by rank, leaves a buffered column out of the read-after-write wait, cuts a phase at a reach behind a writer, refuses at build a unit that writes a plain column it reads with a reach, places every column by the store list, and runs a maintenance band once by one core.

The src changelist (`202611131627_changelist.src.lock.md` in the round's directory) lists every file, every test with the mutant that turns it red, and every departure under the change it belongs to.

## Proving checks

Every refusal has its compile-fail twin with a passing control: eight pairs in the api crate, the pairs under the engine's `tests/ui/`, and the doctests for the const assertions, which fail at monomorphisation where a check does not see them. Every new test was shown failing under a changed line and the line put back.

On the tree with `dev` merged in, the suites of `hilavitkutin-api`, `hilavitkutin-api-macros`, `hilavitkutin-kit` and `hilavitkutin` pass with their doctests and compile-fail pairs, the providers crate's `column_storage` test passes, and the bench carrier's four tests pass, all in the debug profile. They were run from the repository root, where cargo drops the flags `hilavitkutin-build` chooses for want of the wrapper `mock/.cargo/config.toml` names.

The sketch S1 property, the frozen authors unchanged under the reference runner and the scheduler, is round two's, and is in that section above.

`cargo mock` is green with the generated documents. The root readme now names the buffered column and says the two resource swaps install their value, which the trunk's readme still denied.

## The bench

`swap_column` in `mock/benches/bench.toml` shares the carrier crate of the two resource swap benches. Four arms: a plain column, `Buffered` under `Snapshot`, the same built twice as the noise gauge, and `Buffered` under `Exchange`. One frame is a producer over every record, a reader, and the boundary, with four-byte values on one core. The harness checked the arms byte for byte over 100 seeds before timing. Results are in `mock/benches/results/swap_column/` and the note that reads them is `mock/research/202611131900_buffered-column-swap-cost.md`.

| records | plain | `Exchange` | `Snapshot` | `Snapshot`, second build |
|---|---|---|---|---|
| 64 K | 22.8 us | 25.3 us | 49.9 us | 30.1 us |
| 1 M | 584.8 us | 725.3 us | 1093.2 us | 1008.5 us |
| 8 M | 10.16 ms | 11.54 ms | 16.22 ms | 15.61 ms |

`Snapshot` over `Exchange` is one copy of the column: 0.3 to 0.4 ms a frame for each million records, a quarter to a third of the frame. `Exchange` over the plain column is 12 to 23 % of the frame. At 64 thousand records the two `Snapshot` builds stand as far apart as either from the plain column, so that size says nothing on this machine. It is one run on a shared four-core virtual machine, and the ranking is the result more than any figure.

What it decides: a writer of every record names `Exchange`; `Snapshot` stays the default as the one mode right under any writer, now with its price known; the copy for each changed morsel (8e2e2326d588) is where the default gets cheaper. Only a total writer was measured.

The two resource benches were not run again on this machine, which is a narrowing of "beside the existing resource swap arms": a run rewrites their tracked results, and the note `202607201100_swap-semantics-ratification-evidence.md` quotes those. The new arms sit in the same tree on the same carrier, and the note sets the two side by side in words and not in figures.

To make `cargo mock bench run` drive the bench package at all, its second binary `asm_gate` now asks for a feature `asm-gate`, and the arms this branch added take the harness at the revision the trunk pins (ticket `the_bench_drivers_in_hilavitkutin_do_not_build_against_the_b`, bumped). Each arm needed a `target` link to the shared build directory, which is not tracked.

## After the close: the bench tree

The round closed on the touched crates' suites, and a walk of every crate with `cargo mock test` after it found bench crates the revised contract had broken: `engine_vs_std` and `asm_gate_fixtures` on the old shape of `RecordOp`, and all eleven crates of `frame_wide`, which came in with the merge of `dev`, on `ColSelector::get` answering a `ColRef` whose fields no host could read. This was mine and is fixed in a second, small round, `202611131924`, closed as well: `ColRef` gains two hidden accessors with a test under both buffer modes, the four units take the revised `RecordOp` with their lists bare and their written types `PartialEq`, and four `unsafe` blocks around the now safe `append` are gone. The measured loops are unchanged, since every write in them is by position and compares nothing. No number of these benches was taken again.

Checked on the fixed tree: `engine_vs_std` and `asm_gate_fixtures` build with their tests, the eleven crates of `frame_wide` build and that member's own `run-checks.sh` passes against its arms built in release, the engine's suite passes with the new test, and `cargo mock` is green. The seven tests of `engine_vs_std` are ignored timing oracles and were not run, so the ported value functions of that crate are held by the compiler and by the engine's own test of the fused sequence, and not by a checksum of their own.

That a round can close this way is ticket `a_round_that_changes_a_contract_can_close_with_the_bench_tre`.

## Departures from the brief

Item 1, record counts per domain, is not built: op's "Bench both first" leaves it to the sketch and bench of ticket 8bcd16133d14. Item 2 is built in the body and not in the declaration: a value function names nothing of a runner, and the unit still names its context type in `Ctx`, as on the trunk (question ticket 3df7466ff25a, with coord). So the proving check's twin "a unit naming an engine type" does not exist. Item 3 is a reach in records over one column and no grid; axes and shapes go to an opt-in crate on his "Opt-in crate over the api" (c7dbcaeb1beb). The twin for a read past the declared reach is the pair of failing doctests on `Window`. Item 4, the frame-buffered kind, is in. Item 5: the positional `write` stays `unsafe` and the value function is the safe form; two writers of one buffered column are not refused at build, they are ordered as for any store, on his "Hints, dependents, registration", so that twin does not exist either. Item 8 is done as asked: the sketch, the bench `registration_binding` with its findings, and the question ticket for him, `which_registration_hilavitkutin_s_contract_takes_beside_the`. The mutation order of the flow was set aside on his "Revise the api in place now": the source was revised in place and not taken down first.

The brief asks for the published prose to name no workflow. The readme and the crate readmes name none. The rustdoc of the api and kit crates is swept in this round. The other crates still name rounds, topics and audits in about seventy source files, which is ticket f7645a40b8f7 and not done here.

## Departures of the source from the locked designs

Each is in the src changelist with its reason. By ticket:

- 76a880867ba4: `ResolveMap`, `ResourceRead::read` with `Copy`, the constructors on the wrappers and `Window::over`, `Link`, `Buildable`, `Appended`, and `ExtensionSurface` and `DefaultRunCfg` as members. All are names or bounds the designs do not carry and the language forced.
- e86353ea1157: the read product is the api's own until the shared list crate's product can hold a borrowed value.
- f337323c33e9: a buffered column takes two storage ids where the design states two reservations under one.
- 8e2e2326d588: the `Snapshot` copy is of the whole column.
- 29cdc662ca14: a bare column written by position is taken as under `Written`.
- `hilavitkutin_orders_two_writers_of_one_store_by_registration` (4b4f40f32a11): two writers are ordered by registration and not by hint first.
- ac91dfb44d6e: a column registered twice no longer builds, and a kit's owned stores have no binding.
- ecef262b242d: a leading band's write dispatches its readers once more in the next pass.
- 8d8ccf1d9da2: `run_parallel` dispatches every unit each pass, as before this branch.
- 1fc4e97f596f: a value function names a resource by `Default`, since the marker has no const constructor.

## Breaks counted

Unit authors, three: `append` loses `unsafe`; a one-value `RecordOp` is written over one-member lists; a value type of scalar fields that derives `ResourceFootprint` and is not `Copy` is refused by name. Scheduler builders, one: a column registered twice, once by hand and once by a kit, no longer builds. Runner and context authors, five, as counted at the doc changelist. Kit authors, one where it applies (`HoldsAll`).

## Not green, and whose

The link of the providers crate's `smoke` test fails for want of the `hilavitkutin_strings` section, on `dev` as on this branch (`red-check`: `branch=fail dev=fail`), ticket 51630fc78396. The persistence crate's `archive_str` test fails to link on the same symbol.

Two bench crates do not build on `dev` at `271909e` and so not here, each seen with `cargo check` in a worktree of `dev`: the support crate of `single-trunk-convergence`, which calls `frame_await` and `frame_await_done` with an argument fewer than the engine takes and fails its nine arms with it, and `resource_storage/axis_ce`, which has two `notko` crates in one build. Ticket `hilavitkutin_bench_crates_stc_support_and_rsb_axis_ce_do_not`.

## Tickets

Filed in the source window: 51630fc78396, 8d8ccf1d9da2, ecef262b242d, 978d0e86e86b (the engine's test builds print a page of warnings), 373dbd4b275e (the three providers examples are comment skeletons), f7645a40b8f7, e86353ea1157, 76a880867ba4, 1fc4e97f596f, ac91dfb44d6e, f337323c33e9, `hilavitkutin_bench_crates_stc_support_and_rsb_axis_ce_do_not`, `a_round_that_changes_a_contract_can_close_with_the_bench_tre`. Bumped: `the_bench_drivers_in_hilavitkutin_do_not_build_against_the_b`, ecb940c91d26.

Answered by op earlier and out of the design text: 2143d534c715, 29426931e825, 7f93b9f3e621. With coord: 3df7466ff25a.

Standing tasks: 29cdc662ca14, 24fa3df63a9d, 8e2e2326d588, 8bcd16133d14, c7dbcaeb1beb, f8955d2897cc, 4b4f40f32a11, 92d416895392, 261222dc6493, f2294fa7f7dc, `work_in_mockspace_stamps_a_bare_minute_where_a_repo_mints_a`.

For coord to close: `approach_selection_takes_the_sum_of_the_domain_extents_where`, and 0fb716c5a463, which the compile-fail pair `appending_unit_fused` resolves.

## Next

Round two is closed. Finding 2 of review 1 is built next, as a round of its own, on the shape op gave on 13 November: the element type decides what a buffered column starts from, and a constructor that does not apply to a type does not exist for it. After it one reviewer reads everything since review 1.
