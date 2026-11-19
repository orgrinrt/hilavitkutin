# H-pieces: Where The Four Kaski Pieces Belong

This is the read for roadmap step H-pieces. The maintainer answered ticket `does_hilavitkutin_take_the_frame_bracket_a_settings_store_a` with "Decide each piece" (clause-dev `.shared/research/202611150830_stack-path/42-op-settles-the-open-calls.md:48-56`). The options he was shown were [Hilavitkutin takes all four | Kaski keeps three, no macro: frame, settings and delta stay kaski's own, and no log macro is wanted | Decide each piece: each is decided on its own with hilavitkutin's design in hand]. The record reads his intent as no blanket rule, with each piece placed on its own merits and with kaski's tenet `a_store_kind_the_stack_lacks_is_built_upstream_and_kaski_is_its_consumer` bearing on the settings store. Line 112 of the same file makes the work one expert read with a result per piece. Every placement below is a proposal to him. Where this read says the evidence favours an option, nothing has been decided.

The read rests on hilavitkutin at `3177527a` (the same tree as `origin/dev`) and on kaski at `79e7ab198`, read only. It also rests on notko as cargo's git checkout holds it at `15a9531`; notko is not cloned in this workspace, so that checkout was not confirmed against notko's `dev`. Arvo was not read. File 42 holds over `40-the-roadmap-final.md` wherever the two disagree. One part of file 42 decides the weight given to kaski's tree throughout: "A step justified by what kaski's code does today is re-justified from the standing canon or dropped" (42:118), along with his words "We don't trust kaski as it is" (42:66). Kaski's code is quoted below as a claim about kaski, to show what each piece is, and no placement rests on it.

The ladder applied is ratified or provisionally stamped tenets, then designs, then code. Held tenets bind nothing and are cited as context only. Ikiuni's and loimu's tenets in hilavitkutin's `downstream` subject are each stamped as "a reference and not hilavitkutin's record"; they show what another design did and set no requirement on hilavitkutin. A further caveat sits under every finding on the frame and the change set. The api forms `Buffered` and `Reach`, which came in with revision `8236de3c`, are provisional until the bench of both record-set shapes reports, and they can be removed if it disagrees (42:22-24, 42:112). `AccumReaderApi`, the `OnMeta<ScheduleEnd>` close-unit example and the maintenance-unit section entered the api design in the same commit (`git log -S` on `mock/crates/hilavitkutin-api/DESIGN.md.tmpl`). Any finding below that relies on them holds only while those forms stand.

## What the four have in common in hilavitkutin's canon

Three ratified tenets set the boundary every placement is measured against. The first is `the_engine_owns_dag_scheduling_dispatch_morsels_stages_stealing_and_progress_and_not_storage_schema_queries_ui_or_io`: the engine's scope is scheduling, dispatch, morsels, stages, stealing and progress, and it owns neither the data model nor IO. The second is `hilavitkutin_does_not_care_what_is_in_the_columns_only_the_read_and_write_dependency_structure`. The third is `op_ruled_that_work_unit_and_the_unit_vocabulary_stay_in_the_api_crate_with_domain_crates_opt_in_above_it` of 16 November, which leaves room for a new opt-in crate above the api. The ruling of the same day, `op_ruled_that_capability_sink_and_codec_leave_hilavitkutin_and_a_consumer_takes_notko_s_sink` (ratified, `mock/registry/tenet/identity-3.toml:388`), takes every output surface for items outside unit execution out of hilavitkutin, and the api design restates it at `mock/crates/hilavitkutin-api/DESIGN.md.tmpl:1778-1782`. Every hilavitkutin crate is `#![no_std]` with no alloc and no `dyn`, so a piece written today against `std`, `Vec` or Bevy cannot move upstream as it stands, whatever its placement.

Hilavitkutin's canon names none of the four pieces as its own. `cargo mock canon find bracket` returns only the ikiuni reference tenet. `find delta` and `already-said "change set"` return no tenet. `find setting` returns no tenet on a settings store. `find log` and `find logging` return no tenet on a log macro. The rest of this read takes each piece apart and asks, for each part, which canon already says something about it.

## The frame-close bracket

### What it is

In kaski's canon the close of a frame is where every change lands, and it carries the most ratified weight of the four pieces. `a_change_is_accounted_and_every_change_of_a_frame_lands_together_at_its_close_in_a_fixed_order` (ratified, kaski `mock/registry/tenet/engine.toml:37`) says everything a frame computes reads the frame before it as fixed, and every change it produces lands together at the close in a fixed sequence; inside the frame, work is sequenced by what it reads and writes. `the_worlds_data_is_immutable_for_the_frame_and_safely_read_where_it_is_allocated` (ratified, `:854`) gives the reason. `what_a_close_landed_is_what_the_next_frame_reads_as_changed` (ratified, `:474`), `a_consequence_lands_in_the_next_frame` (ratified, `:252`), `what_is_derived_from_the_state_is_refreshed_at_the_close_over_only_what_changed` (ratified, `:450`) and `a_fold_that_reads_another_fold_is_folded_in_the_same_close_in_dependency_order_and_reads_the_frame_before_only_to_cut_a_cycle` (ratified, `:569`) each give the close a further duty. `a_tally_is_held_by_its_holder_and_conserved_by_folding_each_frames_moves_at_the_close` (ratified, `:591`) adds a conservation check at the close.

`why_the_simulation_writes_only_through_mutations` (`:462`) is filed in kaski's registry as provisional. The capture of 13 November records the maintainer choosing "Ratify as written" for it with an addendum (clause-dev `.data/op-responses/202611130816_arvo-overflow-defaults-and-the-canon-double-check-nobody-did.md:427-446`): "Special contracts exist f or work units that are maintenance (can write in place, primarily do frame close and setup and such)". The registry's standing and the capture disagree, and that is reported here rather than resolved.

Kaski's code (a claim about kaski) is `kaski-engine-frame`, 4694 lines on `bevy_ecs` and `bevy_app`: open, interior and close system sets, a per-system `Mutations` buffer, a `MutationQueue` that the close drains in schedule position, a conservation check, a swap of the change set, a landing at the open that holds the frame back for a late store answer, and keyframes. The purge removes it whatever happens here.

### What hilavitkutin's canon and design say

The lifecycle points are hilavitkutin's and they are ratified. `four_meta_virtuals_fire_plan_stage_schedule_ready_pass_start_and_schedule_end` (`mock/registry/tenet/meta.toml:23`) defines `ScheduleEnd` as fired after all consumer work. `the_scheduler_is_itself_a_pipeline_and_meta_units_run_at_lifecycle_points` makes the close a place where units run. `cadence_is_the_consumer_layer_and_the_engine_provides_the_measurements_while_the_consumer_decides_policy` already shows the split this read keeps finding: the engine owns the point in the pass, and what is done there belongs to the consumer. `the_kernel_fires_real_flags_for_the_meta_virtuals_so_a_consumer_schedule_gate_on_the_end_of_schedule_marker_opens` (provisional) lets a consumer unit gate on `ScheduleEnd`.

The api design fills in the close. "Maintenance units and the close" (`mock/crates/hilavitkutin-api/DESIGN.md.tmpl:595-669`) defines a unit scheduled `OnMeta<V>`: one on `PassStart` sets up the pass, one on `ScheduleEnd` "is the close of the pass", and each is dispatched once by one core over the whole range of records. It may write in place, which matches the maintainer's addendum. An ordinary unit appends the changes it means to an `Accum<T>`, and a close unit applies them through `AccumReaderApi::each` (`:636-669`, the `ApplyHeat` example). The swap of a double buffer is the engine's too, "The boundary is the engine's" (`:701-715`): after the last unit of the close, the live side of every changed `Buffered<T, M>` becomes its frozen side, under `Snapshot` or `Exchange`. The engine design describes the same bands at `mock/crates/hilavitkutin/DESIGN.md.tmpl:152-155` and `:294-296`. `an_end_of_pass_unit_drains_accumulator_columns_through_the_provider_commit_ordered_after_the_phase_barrier_and_before_the_next_phase_s_dispatch` (provisional) is the same shape on the persistence side.

One part is missing. `AccumReaderApi::each` hands out values "region by region, one region for each core, so their sequence depends on the worker count", and the consumer "carries in each change what decides between two changes to one record" (`DESIGN.md.tmpl:653-656`). The H-status read (`mock/research/202611192040_h-status/REPORT.md`, section "The ordered mutation queue") finds that no store kind for an ordered queue of changes exists and that the canon is silent on a sequenced mutation queue, and quotes the maintainer's words of 13 November: "Intended case in the mutation paradigm is writing through queues. Now that can be either a second write buffer OR it can be queued up ops that drain verctoeised and optimized at the close or whatever. All depends what part of that is in hila and what is a kaski impl of hila". The question `who_rotates_a_buffered_store` is open (subject `change`). The ikiuni tenet `in_ikiuni_the_frame_close_bracket_of_the_tick_loop_swaps_the_double_buffer_and_it_is_not_a_system` describes a bracket that drains, checks conservation, swaps and seals, but it is a reference and binds nothing.

The bracket therefore has four parts:

1. The two points in the pass.
2. The boundary swap.
3. A drain of queued changes in a sequence that does not depend on the worker count.
4. The domain work done at the close: conservation, the journal, keyframes and the landing.

### Options

Option A gives hilavitkutin the whole bracket as one engine construct. The maintainer's addendum on maintenance units supports it, and so does the fact that parts 1 and 2 are already designed there. Against it, parts of it are kaski's ratified domain (tally, ledger and the journal name kaski's concepts), the ratified scope tenet excludes the data model, and the only canon text describing such a bracket is ikiuni's, which is a reference.

Option B leaves everything as designed. Hilavitkutin keeps parts 1 and 2 as it designs them, and kaski writes parts 3 and 4 as its own `OnMeta<ScheduleEnd>` units over `Accum` and `Buffered`. For it, nothing new is needed in hilavitkutin, and the api already leaves the deciding key to the consumer. Against it, kaski's ratified tenet needs a fixed sequence at the close however the frame ran, so every close unit of kaski's would build the same worker-count-independent drain over `each` for itself, which is the mirrored logic the reuse rule forbids. Determinism across worker counts is also a property hilavitkutin holds itself to elsewhere: bench B1a asks for byte-equal results at five worker counts (`40-the-roadmap-final.md:97`), and the conformance laws hold `run_parallel` to the one-thread reference.

Option C splits the bracket. Hilavitkutin keeps parts 1 and 2, adds part 3 as a contract in `hilavitkutin-api` with its implementation in the engine runner, and kaski keeps part 4 as its own close units. Part 3 is a read of an accumulator in a sequence fixed by a key the consumer's change type supplies (a trait, never a closed set), or a store kind for a keyed queue. For it, kaski's ratified `a_store_kind_the_stack_lacks_is_built_upstream_and_kaski_is_its_consumer` (`engine.toml:833`) applies directly, since a sequenced drain is a kind the stack lacks and it can be reused; the regions it has to merge are the runner's own (`each_core_appends_into_its_own_region_of_an_accumulator_and_the_main_thread_merges_the_regions_in_order`, provisional), so only the engine can merge them without a copy; and it needs no new crate, because `AccumReaderApi` is already in the api and implemented by the runner. Against it, no hilavitkutin tenet asks for it yet, its cost has not been benched, and it rests on the provisional `Accum` reader form.

Option D, the question's option 2, keeps the whole bracket in kaski. On the evidence read here nothing supports this beyond today's tree, and hilavitkutin already holds parts 1 and 2.

The evidence favours option C. That holds for hilavitkutin at `3177527a` with the `Buffered`, `Accum` reader and `OnMeta` forms standing. If the bench of both record-set shapes removes them, B and C both lose their footing and the piece is read again.

## The settings store

### What it is

Kaski's canon on settings is thinner than its tree. Both tenets that `kaski-settings/DESIGN.md.tmpl` cites as its ground are demoted: `the_settings_are_read_from_one_engine_settings_crate_and_file` (kaski `mock/registry/tenet/workflow.toml:157`) and `a_setting_states_when_a_change_to_it_takes_effect_and_every_default_is_fixed_at_build` (`:168`), which carried the build, start, load and run moments. What stands is provisional: `a_tunable_value_is_a_build_time_setting_with_a_sane_default_and_its_polish_waits` (`:99`), `a_value_used_in_more_than_one_place_is_defined_once_a_true_constant_in_its_domain_crate_and_everything_else_a_knob` (`:569`), `the_tick_rate_its_cadences_and_how_stepping_is_paced_are_chosen_at_build_time` (`time.toml:162`), `the_display_frame_rate_is_a_setting_the_player_chooses` (`look.toml:121`) and `the_worlds_axes_are_knobs_with_y_up_as_the_default_and_a_change_of_them_re_orients_everything` (`engine.toml:821`). Two are ratified: `a_value_is_derived_before_it_is_stored_tuned_before_it_is_authored` (`engine.toml:604`) puts tuning after deriving and storing, and `whether_a_world_may_be_taken_up_again_from_an_earlier_point_is_a_knob_of_that_world` (`distribution.toml:323`) is one knob. The maintainer said on 15 November that "settings/knobs primarily" are what everyone needs (`INTENTS.md` C7, line 501), in a question about a shared bottom group of kaski crates and not about the stack.

Kaski's code (a claim about kaski) is `kaski-settings`, 2130 lines on `std`, `tomlfuse` and `toml`: a `When` of four moments, shadowing layers, a `Store` of what a player sets, a `Row` with its `Shape`, a `Lands` answer and a `Refusal`.

Such a store has three layers: build-time constants bound from a file; values a unit reads during a run that change between runs; and a layered, persisted, player-facing file that has to be parsed and written.

### What hilavitkutin's canon and design say

The second layer is already hilavitkutin's store kind. `a_resource_takes_its_initial_value_from_the_consumer_through_the_builder` (ratified) and `a_resource_is_scalars_and_const_sized_collections_only` (ratified) give a singleton set at build. Its replacement between frames is in the design: `replace_value_is_the_supported_every_frame_swap_and_costs_one_blob_write_then_the_store_s_dirty_cone` (held, context only), `a_swap_of_a_resource_value_is_legal_only_between_frames_and_is_observed_from_the_next_run` (held), and `replaceable_is_the_only_enforced_annotation_on_owned_types_and_it_is_opt_in_so_override_of_a_kit_s_resource_is_rejected_at_compile_time_by_default` (provisional). The kit design's own example of an overridable resource is a `LintConfig` (`mock/crates/hilavitkutin-kit/DESIGN.md.tmpl:194-198`). The api design says the same at `DESIGN.md.tmpl:738-741`: no unit writes a resource inside a pass, and between passes the scheduler replaces a `Replaceable` value, which the next pass reads. The provisional `schema_kinds_events_settings_domains_storage_locations_and_access_levels_are_zero_sized_markers` (`mock/PRINCIPLES.md.tmpl:153-157`) names settings domains as zero-sized markers, a rule of style for whatever declares them and not a store.

The third layer is outside hilavitkutin's scope by the ratified scope tenet (no IO, no schema) and by `the_consumer_materialises_external_data_into_columns_through_its_own_loader_units_and_manages_storage_lifecycle` (ratified). The first layer is what `tomlfuse`, a workspace repository, already does at build time.

### Options

Option A adds nothing to hilavitkutin. Settings a unit reads are `Resource<T>` values, `Replaceable` where they may change between frames and fixed otherwise. For it, the stack has this store kind, so kaski's upstream tenet is met by what exists, and it adds no crate. Against it, the moment at which a change lands, and the shadowing of layers, are left to each consumer.

Option B is a new opt-in crate above the api, for example `hilavitkutin-settings`, giving typed settings domains and the moments as a typestate over `Resource` and `replace_value`, shipped as a kit. For it, kaski's ratified upstream tenet, the maintainer's sentence that everyone needs knobs, and the PRINCIPLES naming of settings domains. Against it, hilavitkutin's canon is silent on a settings store, the moments it would encode stand only on a demoted kaski tenet, and any part that reads a file breaks no-alloc and the scope tenet.

Option C binds the build-time layer through `tomlfuse`. For it, binding constants is what `tomlfuse` exists for. Against it, it answers the build-time layer only.

Option D is a narrow kaski crate for the parsed, layered, player-facing file. For it, IO and kaski's own vocabulary of knobs are kaski's. Against it, the superseded `a_store_kind_the_stack_lacks_is_a_narrow_kaski_crate_on_the_stacks_contracts` (`engine.toml:842`) was replaced by the upstream tenet, so D holds only for what needs kaski's vocabulary.

The evidence favours A for the run-time store, with C for build-time constants and D for parsing and persisting a player's file. No new hilavitkutin crate is favoured, since no standing tenet establishes the moments. This holds while kaski's moment tenet stays demoted; a ratified kaski tenet on when a setting's change lands would make option B worth reading again.

## The change-set merge

### What it is

Kaski's ratified canon asks for a list of what changed and for its merge across skipped frames. `what_a_close_landed_is_what_the_next_frame_reads_as_changed` (`engine.toml:474`) makes what the close landed the one place anything learns what changed. `nothing_a_frame_makes_is_lost_to_the_presentation_however_many_frames_run_between_two_it_draws` (ratified, `:543`) has a presentation that looks less often than the simulation runs shown everything it missed. `the_simulation_is_a_headless_world_of_its_own_and_the_presentation_is_its_client` (ratified, `:508`) puts that merge on the border between two of kaski's own worlds. `one_concept_has_one_owner_and_everything_else_reads_it` (ratified, `workflow.toml:36`) asks for one definition of it.

Kaski's code (a claim about kaski) is `kaski-delta`, 218 lines. Its `Delta<K: Copy + Ord>` holds keys in the sequence they were first written, or a top that says everything changed; its merge is associative and idempotent, the top absorbs it, and a bounded merge turns into the top past a bound. It is built on `Vec` and `BTreeSet`. Its design names two users, the simulation's change set and the border's, both kaski's.

### What hilavitkutin's canon and design say

Hilavitkutin owns change knowledge at the granularity of its stores and morsels, for scheduling. Ratified: `the_engine_owns_the_generation_counters_and_the_skip_logic`, `change_detection_is_per_morsel_and_skips_what_is_unchanged` and `version_stamps_tell_which_stores_a_unit_actually_modified`. Provisional: `hilavitkutin_should_own_the_version_stamp_and_early_cutoff_and_phantom_write_detection_is_the_consumer_s`, and `the_per_morsel_generation_counters_are_not_built_and_the_engine_has_a_coarse_per_store_dirty_only`. The api's "What counts as changed" (`DESIGN.md.tmpl:562`) and the engine's seed of the dirty set (`mock/crates/hilavitkutin/DESIGN.md.tmpl:117-150`) use that knowledge to skip units. None of it is a set of consumer keys published to a reader across a border. `canon find delta` and `already-said "change set"` return no tenet. The panel's `DROPLIST.md` L11 (clause-dev `.shared/research/202611150830_stack-path/DROPLIST.md:36`) dropped "Hilavitkutin owns change sets" for want of ground, to be reopened by "A ratified hilavitkutin row", and none has appeared.

### Options

Option A has hilavitkutin expose its generation counters or version stamps as the change set a reader reads, with merge taken as the maximum over generations. For it, one owner of change knowledge, and the cost is only what changed. Against it, the counters are per store and per morsel, not per domain key; the per-morsel counters are not built; the scope tenet excludes the data model; and the canon is silent, so L11's reopening condition is not met.

Option B is a generic bounded set with this merge law in arvo's container family, since the value is pure algebra over a key and carries no engine concept. For it, a crate holds only its own thing, a bounded set with a top is a container, and arvo holds the capacity type a version without alloc needs. Against it, arvo's plan, not read here, is not known to name it, and only kaski instantiates it.

Option C keeps it in kaski, beside its change set. For it, the keys, the change set and the border are all kaski's ratified concepts, and no second user outside kaski is in evidence. Against it, the law is general, and if a second user appears the reuse rule asks for it to move.

Option D is not wanted as a piece of its own: the published frame's landed stores serve as the change set and nothing merges. Against it, `nothing_a_frame_makes_is_lost_to_the_presentation_however_many_frames_run_between_two_it_draws` needs the merge across skipped publishes.

The evidence favours option C. It holds while kaski is the only user in evidence and while hilavitkutin's canon is silent on change sets. A second user outside kaski would shift it to B. A ratified hilavitkutin tenet making its generations a consumer-readable change set would reopen A.

## The log macro

### What it is

Kaski's canon is silent on a log macro and on switching diagnostic output. A grep over every file under kaski's `canon_paths` finds no ratified or provisional tenet on either. The only tenet on a log, `a_host_always_sends_the_lossless_log_and_sends_a_lossy_state_channel_where_the_connection_allows`, concerns the network stream. `kaski-diag/DESIGN.md.tmpl` carries `FIXME(kaski_diag_no_tenet_says_that_diagnostic_output_is_switched)` and rests on a call of 24 October recorded in a design round topic, not in canon. Kaski's code (a claim about kaski) is 130 lines: `trace!` to `error!` wrapping Bevy's log, compiled out unless the `diagnostics` feature is on. The Bevy purge removes what it wraps.

### What hilavitkutin and notko say

`op_ruled_that_capability_sink_and_codec_leave_hilavitkutin_and_a_consumer_takes_notko_s_sink` (ratified) and `DESIGN.md.tmpl:1778-1782` send the diagnostic stream to notko's sink. Inside units a diagnostic is data: `a_kit_captures_a_bundle_of_units_and_stores_that_consumer_apps_register_together` (provisional) names "a logger unit with its diagnostic store", and the api's own example writes `write![Column<Diagnostic>]` (`DESIGN.md.tmpl:1337`). Notko at `15a9531` has `Emit<T>` (`src/sink.rs:81`), "the shape of a destination ... A logger, a serial port, a file behind a lock, a channel", fallible and reached through `&self`. It has no macro.

### Options

Option A puts it in hilavitkutin. The 16 November ruling excludes it: a macro there would be a front for a sink hilavitkutin no longer holds. Nothing is for it.

Option B is a macro in notko over `Emit<T>`, switched by a feature the consumer asks for, under the rule that gates both feature and target. For it, notko owns the sink, and the macro is that sink's front. Against it, notko's design is silent on one, and no canon in any repository asks for switched diagnostics.

Option C is a thin kaski macro over notko's `Emit`. For it, switching per build is a decision of kaski's builds. Against it, no kaski tenet grounds it, and the FIXME says so.

Option D is not wanted. Diagnostics inside the simulation are a store written by a unit, as the kit tenet and the api example have them, and the host decides what to print, through `Emit` outside units. For it, it keeps hilavitkutin's model of state as stores, and the deterministic simulation emits nothing as a side effect. Against it, prints from host or presentation code outside units still need a destination, which `Emit` gives without a macro.

The evidence favours option D, with notko (option B) as the home if a tenet ever asks for a switched macro. That holds while no kaski or notko tenet asks for switched diagnostic output.

## What each placement still needs

### The frame-close bracket

A hilavitkutin tenet stating which part of the mutation paradigm is the engine's. Its source is the maintainer's words of 13 November quoted in the H-status read, so it is a question first, with three options: hilavitkutin gives a drain of an accumulator whose sequence is fixed by a consumer key; hilavitkutin gives a store kind for a keyed queue; the drain is the consumer's in each close unit. After that, a sketch of the keyed drain held equal at 1, 2, 3, 4 and the maximum worker count, compared unsorted, with a twin keyed by arrival that must fail. This is H-proof's queue premise (`40-the-roadmap-final.md:87`), and the H-status read names the same. Then a design round in `hilavitkutin-api` and the engine if the sketch works. Also the question `who_rotates_a_buffered_store` answered from the record, as its own entry says, and the standing of kaski's `why_the_simulation_writes_only_through_mutations` reconciled with the capture of 13 November. All of it waits on the bench of both record-set shapes, which can remove the forms it stands on.

### The settings store

A kaski tenet on when a change to a setting lands, since the one carrying the moments is demoted. It is a question for the maintainer with the options of the demoted tenet's own words (build, start, load, run) or none. Until then nothing in hilavitkutin is needed. If such a tenet is ratified and asks for the moments across consumers, a question on an opt-in crate above the api, with a sketch of the moments as a typestate over `Resource` and `replace_value` and no file reading.

### The change-set merge

Nothing in hilavitkutin. Kaski needs a design of the change set in its core work (K8) to carry it, written without alloc. A question to arvo, as a ticket there, on whether a bounded set with a top belongs in arvo's containers; if it does, kaski consumes it. A ratified hilavitkutin tenet would be needed before option A is opened, and DROPLIST L11 names that condition.

### The log macro

Nothing in hilavitkutin. A kaski tenet on whether diagnostic output is switched per build, which the `kaski-diag` FIXME already names as missing. It goes to the maintainer as a question with the options: no macro, diagnostics as stores and host output through `Emit`; a notko macro over `Emit` behind a requested feature; a kaski macro over `Emit`. If notko is chosen, a design round in notko.

## What was tried and not settled

`cargo mock canon find` in kaski did not return within five minutes on its first run and was stopped; a second run, `find knob`, completed and confirmed the tool works. Kaski's canon was otherwise read by grep over the files its `canon_paths` declares (`mockspace.toml:30`), filtering on ratified and provisional standing, so this read relies on that grep for kaski's silence on a log macro and on switched diagnostics. Notko was read from a cargo checkout, not from notko's `dev`. Arvo was not read, so option B of the change-set merge has no check against arvo's plan. `cargo mock unearth` was not run in hilavitkutin; `already-said` and `canon find` covered the subjects named. Nothing was built, tested or benched.
