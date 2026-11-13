Source: kaski channel (coord/kaski of clause-dev), att/20261113T115204798166439-hv-api-revision-f40703/report-hv-api-revision.md

# hv-api-revision: round one's doc changelist, third writing, after its own review

Branch `work/api-revision` of `orgrinrt/hilavitkutin`, head `94abd16fb1dfcf92d23b8a605bb8d856f6674583`, pushed. Round `202611130617-h6mb`, phase DOC, changelist unlocked at `mock/design_rounds/202611130617-h6mb_changelist.doc.md`. No source is touched. `cargo mock` passes. The trunk at `6e971d1` is merged in.

The changelist carries every passage with its path and line, his answers verbatim with their options, and the readings quoted where they disagree. This report says what was decided, what the reading he asked for found, what the seat's own review found, and what is the coordinator's to weigh.

## What the reading found on buffering

Read whole: round `202605101036` on the trunk, `origin/recover/unlanded-rounds-and-benches`, and ikiuni `origin/dev` as a reference.

The founding record has one value that is a pass behind and says how: the scheduler writes its metrics resource at the start of each pass with the figures of the pass before, every reader comes after, and the stamp's increment makes the readers dispatch (the core design topic, `:595-598`, `:639-641`). The stamp marks the change. Nothing in the record has a counter select between two held values.

A later round (`202605101036_topic.plan-algorithm-chain.md:1181-1208`) sketches a double buffer in one resource value flipped by the adapt unit at `ScheduleEnd`. It is agent work and not the record, and it cannot be written against the designs: a consumer resource is read-only after init (engine design `:877-878`) and the api has no write of one.

The "swap" that was specified, built and benched is `replace_value`, a whole resource value replaced between passes. Its spec and evidence memo are on the trunk. Its bench results reached the trunk at `6524900`. The source that does the install is still only on the recovery branch; on the trunk `replace_value` marks the store dirty and discards the value (`scheduler/dirty.rs:62-73`, under a FIXME). Neither the trunk nor that branch holds a design, sketch or bench of a column in two or three buffers.

The founding research rates loimu's swap buffer "Directly portable" (`research.loimu-synthesis.md:294`) and the spec took it up nowhere. One expert memo on the trunk says hilavitkutin's columns are double-buffered (`mock/research/202606210600_expert-loimu-heritage-storage-model.md:289`) and cites nothing; I found nothing under it and do not build on it.

ikiuni states the model and no mechanism: a tick reads the committed frame, output is mutations applied at the frame barrier.

## The eight, as they now stand

1. Frame-start reads. No store kind, no read marker, no generation wrapper. A store whose every writer is a maintenance unit holds one value for every ordinary unit of a pass: the one left on the leading bands, as the record's scheduler metrics do, or, where the writers are at the close, the one it held when the pass began. Bringing the next value in is a maintenance unit's work in two forms: changes appended by ordinary units and applied by a close unit, or a second column carried into the first by a close unit. Two engine rules make it hold: units of different rank that share a store take the sequence of their ranks (an edge agreeing with the ranks is built, one against them is not), and a store a maintenance unit changed seeds the dirty set (the same pass for a leading-band writer, the next for a close writer). The step from the record's passage to a close writer is this seat's derivation and is open question 1 below. An exchange of two columns' storage is left out, ticket `bench_a_close_unit_s_copy_of_a_column_against_an_exchange_of`.
2. Record counts. Left out, on "Bench both first". One record count per scheduler as on the trunk. Ticket `sketch_and_bench_several_record_sets_in_one_schedule_against`.
3. Two writers. Hints, dependents, registration, in his words. Nothing is refused. The code orders by registration alone, ticket `hilavitkutin_orders_two_writers_of_one_store_by_registration`.
4. Changed. Declared by the writing unit on the member it writes, default differs as the option he chose reads: a plain `Column<T>` in a value function's write list counts only where the returned value differs (it asks `T: PartialEq`), and `Written<Column<T>>` counts wherever the unit wrote, with no bound. The positional `write` counts when called (open question 2).
5. Reach. `Reach<Column<T>, N>` in a read list gives a `Window`, in records along the column's one sequence. Geometry goes to an opt-in crate.
6. The accumulator. Decided: a fourth store marker, `Accum<T>` as source has it, with state still one of three things by what it holds. Storage follows from the registered marker with no look at any unit; a wrong access fails at the accessor's own bound; a column of `T` and an accumulator of `T` stay two stores. A wrapper would make a store's shape a fact of the whole schedule.
7. Content adding stores. No change. Typed registration stays; the question is with the coordinator.
8. The value function. `map` on `EachApi`, called from `execute`. `WorkUnit` and `Plannable` unchanged. `RecordOp` under `WorkUnit` for the fold.

## Maintenance units and the close

A unit scheduled `OnMeta<V>` is a maintenance unit. Its context holds the whole record range with no ordinary unit beside it, so it is where a write at a record named from data belongs, and it reads the pass's appends through `AccumReaderApi`, an accessor on a maintenance context alone. The design says plainly that the contract does not hold an ordinary unit to this (`write` stays `unsafe` for all), and that the sequence a close unit meets appended values in follows the worker count. The rule that a simulation's changes go through mutations is the consumer's to hold; the api gives the mechanism.

`append` is safe, and the two accumulator bridges become `unsafe trait` with safe methods, so the implementer of a context holds the obligation.

## The seat's own review

Run at his word ("Ask them to run their own reviewers with the skill and rules, I.e neutral and in fresh worktree and context"): a fresh `pr-reviewer-senior` in a worktree of its own, a neutral brief naming the founding record and his answers as governing, existence and locus challengeable.

Pass 1, on `3581ac4`: not lockable. Found and fixed in `91d6758`: the record's pass-start passage left out of change 1 and the question decided where the record is ambiguous (the passage is now quoted, the derivation is named as the seat's, and the question is open question 1); a third form, two buffers in one resource value, not writable against the designs (withdrawn); a later round treated as the record in one place and as agent work in another (it is agent work throughout now); the default of change 4 against his chosen option (turned to differs, wrapper renamed `Written`); a safe `append` over an `unsafe fn` bridge in a safe trait (bridges are `unsafe trait`); a maintenance paragraph more normative than the contract (reworded); the close apply not deterministic across worker counts (said in the design); five citations on a wrong line base (corrected to the trunk at `c42d707`); round and process wording in the public design text (removed); the prelude listing and the accessor count (corrected).

Not taken from pass 1, with the reason. Its fix for change 1 was to take the section out until he answers: kept as the round's proposal with its derivation stated, since the coordinator asked for the restatement and decides whether it goes to him. Its vocabulary finding on "order": the dead term is the old engine concept, and the trunk's design text uses the word in its plain sense throughout.

Pass 2, a second fresh reviewer on `91d6758`: not ready to lock, on other grounds. It read the record before the changelist's account and found what this seat and the first reviewer had not cited: a consumer's unit on `ScheduleEnd` writing columns after all consumer work (core design topic `:730-731`, `:763-779`), and a counter bumped on every write and checked before each pass (spec `:862`, `:1420-1424`). I opened each passage and they read as it says. On them it finds the placement of a writer at the close to be the record's, where pass 1 found the point ambiguous. Fixed in `419aec4`: those passages are in change 1 with what is the record's and what is this round's sentence; four open questions written into the design as settled now have a ticket each and a marker in the design text; the one-core dispatch of a maintenance unit is stated for both worker paths of `run_parallel` (new for the phase-loop path, to be checked against source in the source window); the reach sentence is narrowed to writers that run before the reader; the engine's seed from a unit's write is said as a gap against the spec's counter on every write, with a ticket; a later round's accumulator topic is no longer headed as the record; the second column of form 2 needs a value type of its own, said; the kit author's break is counted.

Not taken from pass 2, with the reason. It asked that `T: PartialEq` be asked only where the compare is made: kept on every value function, since a bound added later breaks unit authors and one asked now does not, and `Written` is there for a type that cannot be compared. It found the contract short of his "Special contracts exist f or work units that are maintenance (can write in place": true, and not built in this writing; the three forms are open question 3.

Pass 3, the last, the second reviewer resumed on `419aec4`: not ready to lock, two things left, every earlier finding resolved. Its report is attached whole at the end. First, the one-core dispatch of a meta band on the phase-loop path was missing from the changelist's own lists of what changes: fixed in `94abd16`, in the per-part list and as a third change of behaviour. Second, it asks that the ticket on the frame-start read sit on op's own queue, since two reviewers differed and such a call is his: not done from this seat, which was told to put no question on hilavitkutin to him. The ticket is held by coord, and this report asks coord to carry it to him. No fourth pass was run; three is the limit.

What is left after three passes, as follow-ups: the ticket markers pass into the generated `docs/` as html comments with tracker ids; the phase-loop dispatch is stated in design and unchecked against source until the source window.

One thing outside the review. `cargo mock` updated its launcher and engine to mockspace `a81e1956` mid-session, and hilavitkutin's two local lints stopped building against it (`ctx.all_sources` and `ctx.source` are methods now), so nothing could be regenerated or committed. Fixed on this branch in `fix: call the lint context's sources as methods in the local lints`; the trunk needs the same two calls. Ticket `hilavitkutin_s_two_local_lints_do_not_build_against_the_mock` (92d416895392).

## Departure from the coordinator's starting point

The message of 10:35 UTC gave "generation read and cross-record reach as wrappers in the access set" as the starting point, to be checked against the recovery branch. Checked: the generation read has nothing under it in the record or on that branch. The reach, the value function and the change declaration stand as given.

Nothing is left to loimu's layer.

## Open questions for the coordinator

None goes to op from this seat. The first four are question tickets held by coord, each with a marker in the design text.

1. The frame-start read, ticket `does_a_store_only_a_close_unit_writes_hold_its_pass_start_va` (2143d534c715). The record has the pass-start write, the close unit writing columns, the bands and the counters; the sentence that puts them together is this round's, and the two reviewers differ on whether the record settles it. Options: (a) the section as proposed, said of any store whose writers are all maintenance units, with the two forms; (b) only the leading-band half, the close writer left out until he answers; (c) a read marker or wrapper in the access set; (d) the engine rotates two buffers at the pass boundary; (e) nothing until he answers. Proposed: (a).
2. Whether his "default differs" reaches the positional `write`, ticket `does_default_differs_reach_the_positional_write_in_hilavitku` (29426931e825). Options: (a) no, it counts when called; (b) yes, with a load and a `PartialEq` bound on every existing positional write. Proposed: (a).
3. Which write in place a maintenance unit has that an ordinary unit lacks, ticket `which_write_in_place_a_maintenance_unit_has_that_an_ordinary` (7f93b9f3e621). Options: (a) none, `write` stays `unsafe` on every context, as written; (b) `write` only on a maintenance context, which breaks every positional unit; (c) a safe write checked against the record count on a maintenance context alone, beside the `unsafe` one, no break. As written: (a). I would take (c) next; it is closest to his word and breaks nothing.
4. A unit written as a value function still names its context type in `Ctx`, ticket `a_unit_written_as_a_value_function_still_names_its_context_t` (3df7466ff25a).
5. Whether an engine exchange of two columns is wanted now and not after its bench.
6. `replace_value` on the trunk does not install the value. The landing of the recovery branch owes it; this job does not touch it.

## Breaks counted

Unit authors, two: `append` loses `unsafe`; a one-value `RecordOp` is written over one-member lists. Scheduler builders, none. Runner and context authors, three: `EachApi` gains `map`; folds read `Member::Store`; `ResolveAccumAppend` becomes an `unsafe trait`. Kit authors, one where it applies: a `ContainsAll` bound written to reach `.build()` becomes `HoldsAll`. Data structure authors, none. Three changes of behaviour with no change of signature: the sequence of two writers by hint, a meta unit sharing a store with a consumer unit placed by rank, and a meta band run once by one core on the phase-loop path of `run_parallel`.

## Departures

From the brief: items 1, 3, 4 and 5 are not built as written (nothing for domains, a reach in records, no frame-buffered kind, the positional `write` stays `unsafe`), each on his answers of 13 November.

From the flow: the mutation order (code revised in place, on his word). `ready-doc 288811a` was posted before its review was read and is withdrawn. The second writing at `845c92d` was never posted as ready.

## Tickets

New in this writing: `sketch_and_bench_several_record_sets_in_one_schedule_against` (8bcd16133d14), `bench_a_close_unit_s_copy_of_a_column_against_an_exchange_of` (ecb940c91d26).

Filed on the reviews: the four question tickets above, `hilavitkutin_reads_a_write_s_changed_mark_for_maintenance_un` (8e2e2326d588), and a tooling one, `writing_style_counts_the_exclamation_mark_of_an_html_comment` (261222dc6493): the ticket markers in the engine design raise a writing-style warning for the `!` of an html comment.

From the second writing: `an_opt_in_hilavitkutin_crate_gives_axes_grid_shapes_and_edge` (c7dbcaeb1beb), `bench_the_value_function_under_each_against_the_position_clo` (f8955d2897cc), `design_doc_source_mismatch_blocks_a_push_in_the_doc_phase_fo` (f2294fa7f7dc).

Standing: `hilavitkutin_orders_two_writers_of_one_store_by_registration` (4b4f40f32a11), `arvo_implements_notko_hlist_s_cardinal_for_usize`, `which_names_the_access_list_s_cell_and_end_take_in_hilavitku`, `which_registration_hilavitkutin_s_contract_takes_beside_the`, `a_readable_member_that_gives_a_record_its_own_key`, `a_product_over_a_list_built_by_member_name`, `the_first_sections_of_the_api_design_still_name_provider_pro`, `the_engine_crate_enables_generic_const_exprs_which_the_works`, `the_bench_drivers_in_hilavitkutin_do_not_build_against_the_b`, `work_in_mockspace_stamps_a_bare_minute_where_a_repo_mints_a`, `stand_up_cuts_the_job_branch_in_the_kaski_clone_when_no_repo`. To close: `approach_selection_takes_the_sum_of_the_domain_extents_where`.

## The last reviewer's report

Pass 3, on `419aec4`, as returned. Line numbers are the reviewer's, of the files at that head.

Verdict: not ready to lock. Two small fixes remain, both cheap.

The canon gate still passes against tiers 1 and 2. I did not run the suite: this is the DOC phase and no source changed. I judged the head `419aec4` on its own, using the diff from `91d6758` and the current files.

Blocker 1. The changelist's own account of what changes now contradicts the design change it makes. The engine design gains a new mechanism (`mock/crates/hilavitkutin/DESIGN.md.tmpl`, new paragraph after the accumulator-reader text): "on the phase-loop path one core runs the band's phases over the whole range while the others wait at the waist barrier". Changelist `:248` says this is "this round's addition, which the source window checks against what the source does and makes so". So it is a behaviour change. The per-part list still says otherwise: `:318` "Per-core dispatch programs, the executor and the thread pool: nothing."; `:324` "...the meta pipeline: the cross-rank order and the accumulator reader on the meta context. Nothing else."; `:350` "Two changes of behaviour that are no change of signature" does not include it. A locked changelist is the claim the source window builds from, and these three lines would send it the wrong way. Fix: name the phase-loop meta-band dispatch at `:318` or `:324`, and as a third behaviour change at `:350`.

Blocker 2. The ticket recording the two reviewers' disagreement is held by the coordinator, not by op. Changelist `:206` and `:277` say question ticket `2143d534c715` is "held by the coordinator" and that none of the questions is "put to him from this seat". The changelist itself records that two reviewers disagreed on this point (`:206`). Governing text, from the dispatch rules: "If the two of you disagree, the call goes to the human. It does not go to a third expert as a tiebreak". It also says a stamp needs two experts who agree, which does not exist here. The design text is now marked provisional (api `:570` marker), which is right. But the call itself has to reach op. Fix: put `2143d534c715` on op's queue and say so at `:206` and `:277`. The other three question tickets are not disagreements between reviewers, and holding them with the coordinator is acceptable.

Earlier findings at this head. B1: partly resolved; four question tickets and one task have ids, each with a marker where its point is stated; what remains is Blocker 2. B2: resolved as a design change; the engine subsection states one-core, whole-range dispatch on `run` and on both `run_parallel` paths, and api `:516-519` matches it; the citations hold at `c42d707` (`:265-267`, `:940-943`, `:947-954`); the change stands under the ladder, tightening tier 3 so that the guarantee the `unsafe` write by position rests on has a stated mechanism; its only defect is Blocker 1. B3: resolved; api `:464-472` limits the claim to writers on the reader's band or earlier. N1: resolved differently, and the change stands; the bound is kept on purpose with a reason, which follows op's "prefer not to break the public facing api. Mostly thinking about unit authors" (topic 1 `:93`), and `Written` remains the way out. N2: resolved; the engine states the shortfall as a gap and carries a FIXME, and changelist `:215` attributes the close-writer seed to spec `:862` and `:1420-1424`. N3: resolved; changelist `:55-59` adds core `:730-731`, `:763-779`, `:386-389`, `:550-553` and spec `:862`, `:1420-1424`, every quotation verbatim; its reading that `:389` and `:553` are said "of signals and not of columns" is more careful than mine was, and it is correct. N4: resolved. N5: resolved. N6: resolved. N7: resolved. N8: resolved.

The `<!-- FIXME(<id>): .. -->` comments are acceptable: the workspace rule `mark-placeholders-fixme.md` names this exact form for markdown and requires the ticket id, and every marker has one. One cost, as a follow-up: the comments pass through into the generated `docs/API.md` and `docs/HILAVITKUTIN.md`; rendered views hide them, but anyone reading the raw file sees tracker ids. The marker at api `:570` also reads "proposed and not yet confirmed", which describes how a decision stands; `documentation-writing.md` keeps that out of tier 1. This does not block the lock.

Anything new that would stop a lock: only Blocker 1. No new claim is unsupported by the record or the designs; every new derivation is labelled as the round's own. The changelist's account of my earlier review is accurate. The new design sentences do not contradict each other. No em-dashes or en-dashes in the templates, and no new round, sketch or workflow names in them, apart from the comment wording noted above. The topic file only had a section appended.

Follow-ups, not blocking: a pointer from the meta-pipeline paragraph on the phase-loop path to the new subsection; the FIXME comments in generated `docs/`; the bench on copy versus exchange (`ecb940c91d26`) and on record sets (`8bcd16133d14`).

Not checked: whether the tickets exist and who holds them; whether the phase-loop path's source matches the new design sentence; whether `docs/` is byte-identical to a fresh render; whether the `no-todo` push-gate lint fires on the FIXME comments.

After that report, in `94abd16`: Blocker 1 fixed as it asked; the marker's wording changed to say what is undecided; the pointer added; the push gate ran with the markers in place and passed. Blocker 2 is with coord, as said above.
