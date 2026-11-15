Source: kaski channel (coord/kaski of clause-dev), att/20261114T005917766123002-hv-api-revision-247734/report-hv-api-revision.md, first posted 2026-11-14 00:59 UTC, read 2026-11-15.

# hv-api-revision: reviewed three times, the last says merge after fixes, and the fixes are in

Branch `work/api-revision` of `orgrinrt/hilavitkutin`, head `e01a42d96eebd0c838dbb7edb83fea729af4bb1e`, pushed, with `dev` at `e99af156` merged in. Review 3, the last of the three, read `d45093ea` and said merge after fixes; its report is `mock/research/202611140036_review-api-revision-3/REVIEW.md`. Four of its five findings are fixed in round `202611140040`, closed. The fifth, the text of thirteen pushed commits, is a call the review hands back and is the coordinator's: it is under "Review 3 and its fixes" below, which is the newest section. The sections above it are in the sequence the work was done in, each with the head it had then.

## Round one

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

## Finding 2, what a buffered column starts from

Head `21e11dafe4ae79234f69ca9630a30706d35582bf`, pushed. Round `202611132230` is closed with both changelists locked, and the parked topic is flat again. `cargo mock` is green. The order of 20:41 was read at 22:15, an hour and a half late, since this seat did not read its notifications while the instruments round was open.

What ships. A buffered column is registered with the value every record of it starts from, the way a resource is registered with its value. `Buffered::<T, M>::start_from(value)` is there for every element type. `Buffered::<T, M>::start()` is there where the type has a trivial constructor (`notko`'s `HasTrivialCtor`) or a `Default`, and takes the constructor where it has both. Both return a `StagedBuffered<T, M>`, and the bare marker is no builder input any more, so `.with(Buffered::<T>::new())` does not compile. The drain writes the value to every record of both reservations, so no safe path reaches a record nothing wrote, and the compare of a write against the frozen side reads a value.

The typestate op asked for. `start` is an inherent function under the constructor's bound and a trait's function (`StartsFromDefault`) under the default's. An inherent function is found first and passed over where its bound does not hold, which gives the sequence between the two tiers with no specialisation. A type with neither has no `start`, and the compiler's refusal names both missing bounds. The sketch `202611132225_buffered-start-by-type` shows it on `notko`'s own trait, with the mutant that fails and the refusal quoted.

Departures from the order, each in the changelists.

1. No null-bit tier. `notko` at the `dev` the manifests take has no trait that gives a type a null value. `NicheFilled`, the nearest name, marks the reverse: a type whose all-zeros pattern is not one of its values. The types that carry a null of their own (`MaybeNull<T>`, `Slot<T>`, `Maybe<T>`) say it through `Default`, so the default tier covers every type the null tier would. Ticket df2e4da2d376 asks `notko` for the trait, and the api's design carries the marker. A third tier ahead of the two needs one more level of lookup and is not designed.
2. The choice between the tiers is made where `start()` is written, from what is known of the type there. A generic caller that knows only `T: Default` gets the default, also for a type that has a constructor too. The trait is in the api's prelude, since the default tier is found only where it is in scope.
3. `start_from` is a `const fn` and `start` is one where the constructor is const. The default tier is not const: a const `Default` sits behind `const_default`, which the workspace has not vetted. Ticket b4df1365a3f7.
4. The refusal of `start` on a type with neither is a `compile_fail` doctest with its error code and not a trybuild fixture: the compiler's message points into the checkout of `notko` by its path on the machine, which a `.stderr` file cannot hold. The bare marker's refusal is a trybuild fixture. It reads as the general `BuilderInput` refusal and does not name `start`.
5. A plain `Column<T>` has the same hole and is not built, since the order named the buffered column: ticket 9b64b95e42d1.

The tests. `tests/buffered_start.rs` in the engine builds every scheduler over memory that is `0xA5` at every byte and writes no side by hand. Its premise test reads a plain column nobody filled as the poison. Under both modes the first pass reads an explicit start, a constructor's and a default's (zero, the start that zeroed memory would pass for), both sides hold it after, and a record no unit wrote holds it after the boundary. They were committed red at `559f3933`, ahead of the write at `77d64d10`, and with the frozen side written alone eight of the nine fail. In `tests/buffered_column.rs` every registration names its start, and a fill by hand stays only where it differs from record to record or is of the live side. The review's `probe_partial_equal` is `a_value_equal_to_the_frozen_one_still_lands_on_the_live_side`, in since the fix round, and keeps its fills, since a start is one value. The api's `tests/buffered_start.rs` holds the tiers, a start made in a constant, and the carrier's `Dispatch` and `Init`.

Breaks counted. Scheduler builders, one: a buffered column is registered with its start. Unit authors, none: the frozen crate's `src` is unchanged and `BREAKS.toml` has no line.

The suites of `hilavitkutin-api`, `hilavitkutin` and `hilavitkutin-frozen-authors` and the tests of the swap bench's common crate are green. The README's paragraph on a buffered column says what it is registered with.

Tickets of this round. Filed: df2e4da2d376, 9b64b95e42d1, b4df1365a3f7, 5dc681e82128, and 6d613ddd8607: the two design markers that cite the sha `8e2e2326d588` where ticket 8c7cc9768579 is due were to be corrected in this round's doc phase and were missed, and the source marker is corrected.

## The column start, and review 2's fixes

Review 2 read `21e11daf` and said merge after fixes. Its report is `mock/research/202611132308_review-api-revision-2/REVIEW.md`. Op's three remarks of 13 November on what pass 1 reads (the type settles a column's start, `HasTrivialCtor` is a tier too, and a constructor that does not apply to a type does not exist for it) reached this seat with the 20:20 and 20:21 mail and are built in round `202611132307` (topic `202611132304_topic.column-start.md`), closed, for the plain column as for the buffered one. A second round of one design edit, `202611140020`, closed after it: the builder section of the engine's design still registered a plain column by the bare marker in three places, and a design is not edited in a source phase. No source moved in it.

What ships. A column of either kind is registered with its start: `Column::<T>::start()` or `start_from(value)`, and the same two on `Buffered<T, M>`. One sealed trait `Starts` covers the two markers and one carrier `Started<S>` registers both, so `StagedBuffered` is gone. The bare marker registers nothing any more, for the plain column as for the buffered one, and a trybuild fixture holds each. The engine has one arm for the carrier and one drain, which writes the start to every record of every reservation. The first pass reads the start from a plain column nothing wrote, over memory that is `0xA5` at every byte, and the compare a close unit's `Differs` write makes in the first pass is against the start: a close that restates it marks nothing changed, and the test fails with four windows handed to the resting reader when the api's compare is forced to write.

Every registration in the engine's tests, the frozen authors' tests, the provider examples and the bench tree carries a start (two sonnet sweeps did the mechanical part on worktrees, each read and run before it was taken). A whole column filled with one value before the first pass is now that column's start. What is still planted by hand is data that differs from record to record, one side of a buffered column, a sentinel between two passes, and an accumulator's seed.

Review 2's findings and what was done.

1. The frozen ledger did not hold an edit whose hash was rewritten with no break. `frozen-ledger` now reads `file` and `hash` of every break, refuses a last break whose hash is not the line's, and compares the ledger with the one at the merge base with `origin/dev`. Tests first, red, then the fix; two mutants turn four tests red. What it does not look at, the unlocking of a line and the removal of a frozen file with its line, no design decides: question ticket 174ba049e794 for op.
2. The frozen attribute lints read one line at a time. They read an attribute from `#` to its closing bracket now, both of them.
3. The reach cuts were computed on every core of every pass where the design says they fold at compile time. `plan::reach_cut::CarrierCuts` holds the cut mask, the consumer mask and the unit count as associated constants, and the walk reads those. A departure the doc changelist did not count: `Scheduler::run_parallel` and the worker loop ask `const BundleMasks`, `const ConstCapacity` and `const BitAccess` where they asked the plain bounds. `build` asks the same of the unit list already and every caller in the tree builds unchanged; a caller generic over the unit list would have to carry the `const` form. With the consumer mask swapped for the cut mask three tests of `reach_across_cores` fail.
4. The public surface. The root README registers its columns with a start, says how a column of either kind starts, names `hilavitkutin-conformance` and drops three builder methods that do not exist. `generic_const_exprs` is still enabled by the engine for one projection into the api's `CoreProgram`; the gate carries a marker on ticket 2311ca65fd66 and the README says it is a leftover. No publishable crate has a readme of its own, which is the whole class behind the finding on the conformance crate: ticket e372d72cd856.
5. The text of commits `57dc2c99` and `0025165d` is not changed. Both are pushed, and their text changes only by rewriting pushed history. The one merge made since has a subject of the house form.

Departures of this round, each in its src changelist: `Started::into_inner` is not `const` (E0493); the engine's arm asks `S: Member` beside `Kept`; a column a kit owns still has no binding and no start (`a_column_registered_twice_no_longer_builds_and_a_kit_s_owned`, bumped); the `const` bounds of finding 3.

Green on the closed tree: the engine's whole suite with its doctests, the api, kit, providers, frozen authors' and conformance crates, and `cargo mock` with every lint and `docs/` regenerated. The whole walk of the bench tree (`cargo mock test`, 159 crates) ends with eleven red, all red on `dev` for reasons the branch does not touch: the support crate of `single-trunk-convergence` and the eight crates that link it, `resource_storage/axis_ce`, and `frame_wide`, whose `checks_gate` asks for release arms built beside it. That support crate is the one caller of `run_parallel` in the tree the `const` bounds are not shown against, since it does not build for the other reason; its call is on a concrete scheduler like the rest.

Tickets of this round. Questions for op: 2d660de45432 (where `HasTrivialCtor` stands in his sequence), 6fe661a77c64 (whether `start_from` exists for a type with a start of its own), 34fb71119410, 174ba049e794. Filed: 6bab1240889d, 2311ca65fd66, e372d72cd856, 63f55f605465 (the kit and providers designs name builder methods the builder no longer has). Bumped: `the_bench_drivers_in_hilavitkutin_do_not_build_against_the_b` (the `single-trunk-convergence` support crate calls `frame_await_done` with an argument fewer than the engine takes, on `dev` as here), `a_column_registered_twice_no_longer_builds_and_a_kit_s_owned`. Built: 9b64b95e42d1. Done: 6d613ddd8607.

## Review 3 and its fixes

Review 3 was one fresh `pr-reviewer-senior` on opus, alone, on a worktree at `d45093ea`, with a brief that named the branch, its base, the rounds and the governing words by path and no verdict of this seat. Its report is `mock/research/202611140036_review-api-revision-3/REVIEW.md`, with two things left out of the copy and said so in its header: the paths of its scratch directory, and a dash character. Verdict: merge after fixes. Its own runs at that sha: the engine's whole suite, the api, conformance and frozen authors' suites, 78 lint tests, and four mutations of its own, each caught.

Its findings and what was done, in round `202611140040` (topic `202611140040_topic.third-review-fixes.md`), closed at `e6375fb9`:

1. The conformance crate had no readme and its manifest named none. `mock/crates/hilavitkutin-conformance/README.md` is written, the manifest's `readme` names it, and the description is the readme's tagline without the "no_std, no alloc" tail op's word leaves out of a tagline. The other publishable crates are still the standing ticket `no_publishable_hilavitkutin_crate_has_a_readme_of_its_own_or`, bumped.
2. Thirteen pushed commits carry a subject outside the house form, and the body of `0025165d` names workflow. Not changed: all are pushed, and the text changes only by rewriting pushed history, which this seat does not do. The thirteen are five `research:` subjects (`5a2c531a`, `c0bb6a07`, `22e342b0`, `673963bf`, `27abd8a8`), two `bench:` (`44f7de28`, `ab63d610`) and six merges with git's default text (`1a219430`, `5e6887fd`, `7d23d52c`, `3581ac42`, `a32fa32f`, `57dc2c99`). The review hands the call back: rewrite the branch before it lands, or land it with them. It is the coordinator's. Every merge made since has a subject of the house form.
3. The refusal of a bare column marker ended on a note telling the author to write an impl nobody outside the crate can write. `BuilderInput` has a second note naming `start()` and `start_from(value)`, the api's design says the error names the two, and the two trybuild goldens are blessed again with one added line each and no other arm moved.
4. The lint `frozen-no-recursion-limit` gave a reason no design states. Its module text and its message now say what the frozen crate's design says and no more.
5. The freeze had a way round it: a `path` attribute or an `include!` in a frozen file compiles in a file the ledger has no line for. The frozen crate's design names the fourth lint, and `frozen-no-outside-source` is in `mock/lints/` at error on commit, build and push. A departure, in the src changelist: the lint refuses the three macro names as whole words and not only as macro calls, so that an import under another name is refused at the import. Thirteen tests; with the path test forced false five fail, and with the macro names changed six fail.

Green after the fixes: 91 lint tests, the api crate's whole suite with trybuild and doctests, and `cargo mock` with every lint and `docs/` regenerated. Not run again after the fixes: the engine's whole suite. The one crate source change since the reviewer ran it at `d45093ea` is the note on an attribute in `builder_input.rs`, which the api's suite holds.

The merge of `dev` at `e99af156` after the close brought in research files only, and `cargo mock --lint-only` is green on the merged tree. The merge commit is `e01a42d9`.

Tickets of this stretch. Bumped: `no_publishable_hilavitkutin_crate_has_a_readme_of_its_own_or` (e372d72cd856), `the_kit_and_providers_designs_of_hilavitkutin_name_builder_m` (63f55f605465), `the_bench_drivers_in_hilavitkutin_do_not_build_against_the_b`. The questions for op stand as review 3 read them and none is new: 2d660de45432, 6fe661a77c64, 34fb71119410, 174ba049e794, and `which_registration_hilavitkutin_s_contract_takes_beside_the`.

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

The branch is ready at `e01a42d96eebd0c838dbb7edb83fea729af4bb1e` and reviewed. The coordinator reads the diff, decides the commit text of finding 2 of review 3, and lands. No review is left to run from this seat: three were the limit.

Left for later jobs and ticketed: the five disagreements of the scheduler with the reference runner, the readmes of the other publishable crates, the rustdoc that still names workflow in about seventy files, the null-bit tier that waits on `notko`, and op's four questions.
