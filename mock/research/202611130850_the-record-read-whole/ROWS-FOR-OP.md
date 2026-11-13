# The rows the written history does not back, or that stand against it

This file lists the 132 rows of the registry that were left as they were when the rest were ratified by default or put on a stamp. Each of them is here for one of two reasons the record itself gives: the source says in words that the content waits on the maintainer or was an agent's own call, or a passage that is his says otherwise on the same point. None of them is ratified and none carries a stamp.

It follows from his words of 13 November 2026, kept whole in `OP-ANSWERS-18.md` beside this file:

> Only those that are clearly not backed by me or actively against the canon otherwise, you bring to me. The rest can be on provisional stamps to begin with

The rows were classed by readers who opened the record for each row, and every finding below names the passage it rests on. A finding is a reader's, so it can be wrong in either direction, and the passage is there to check it against. Where a row is a question, its options are the ones the registry already holds. Where a group has no question row, the options were written for this file and say so.

The rows are grouped by what the maintainer would have to say to settle them. The first part holds rows that stand against a word of his which the record treats as standing, so nothing is asked beyond whether the row goes. The second holds points the record states two ways with no word of his between them. The third holds work whose own source says it waits on him. The fourth holds rows an agent wrote and a later agent round replaced, where neither side is his.

## Part one: rows against a word of his that stands

These 22 rows say something a later ruling, a later answer or the founding text says otherwise, and the record's own order puts his word above the row. The options were written for this file and are the same for each row:

1. The row is demoted and the word of his named in the finding stands.
2. The row's version is the one he wants, and the word named in the finding is what changes.

#### `waiting_is_parked_in_three_tiers_chosen_from_the_predicted_wait_of_each_phase_with_per_os_atomic_wait_primitives`

A tenet. It says: Workers wait in three tiers. The tier is chosen from the predicted wait of the phase, with spin only for the shortest waits, spin plus an operating-system wait for the middle range and an immediate park for the longest, using each operating system's own atomic-wait primitive.

Finding: The principles and founding spec put the tiers at under 100ns spin, 100ns to 10us spin_loop with backoff and over 10us park, while this row's mechanism sets under 2us, 2 to 50us with an OS wait, and over 50us, and it is the thread-pool topic's own unvalidated default; see row predictive_parking_at_sync_points_chooses_spin_backoff_or_park_from_the_estimated_wait.

Passages: `mock/PRINCIPLES.md.tmpl:238-239`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:2050-2057`.

#### `the_adapt_configuration_is_nine_independent_axes_with_per_axis_sampling_cadence_and_const_presets`

A tenet. It says: The adaptation is configured over nine independent axes and not through opaque modes, each axis with its own sampling cadence. Four convenience presets are provided as constant functions over the multi-axis space.

Finding: The topic says the user reframed adaptation as independent axes and that his Kit reframe replaced the composite struct with const-fn presets by individual axis configs and Kit recipes, while this row keeps four const-fn presets over a multi-axis space. The later state sits in row the_adapt_kits_bundle_the_nine_metric_axes_with_a_standard_recipe_with_validated_defaults_and_an_off_recipe_that_disables_the_observer.

Passages: `mock/design_rounds/202605101036/202605101036_topic.adapt-subsystem.md:48-48`, `mock/design_rounds/202605101036/202605101036_topic.adapt-subsystem.md:265-265`, `mock/DESIGN.md.tmpl:60-60`.

#### `as_of_round_202605101036_plan_affecting_is_a_sealed_marker_and_a_resource_without_it_is_swapped_cheaply`

A tenet. It says: In the round of May 2026 the marker that makes a resource plan-affecting is sealed, taken by consumer opt-in or derived by macro. Replacing the value of a resource that carries it sets the dirty bit for a plan recompute; replacing one that does not is a cheap value swap. The default is not plan-affecting.

Finding: The row says the marker is sealed, while addendum A2-3 records that op ruled it unsealed into an open marker on resource value types (see tenet::plan_affecting_is_an_open_marker_trait_that_consumers_implement_on_resource_value_types); the source topic is also marked as autonomous trivial calls.

Passages: `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:55-62`, `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md:4-4`.

#### `the_ecosystem_crates_are_standalone_and_consumers_opt_into_them`

A tenet. It says: The context, persistence, identity core, string, kit and providers crates are standalone ecosystem crates that consumers opt into. The context, persistence, identity core and string crates are pure ecosystem crates that the engine does not depend on.

Finding: This row lists persistence among the standalone ecosystem crates, but he chose "Builds on str and api" over standalone for persistence; see row op_ruled_that_the_persistence_crate_builds_on_the_str_and_api_crates_and_its_design_holds_over_the_lint. The engine-does-not-depend and opt-in parts are not contested.

Passages: `.data/op-responses/202609181645_hila-std-tier-persistence.md:33-43`, `mock/DESIGN.md.tmpl:32-32`.

#### `the_persistence_crate_is_the_generic_hot_cold_storage_bridge_as_a_standalone_ecosystem_crate`

A tenet. It says: The persistence crate is the generic bridge between hot and cold storage, and a standalone ecosystem crate.

Finding: The row calls persistence standalone, but he chose "Builds on str and api" over "Standalone, lint holds"; see row op_ruled_that_the_persistence_crate_builds_on_the_str_and_api_crates_and_its_design_holds_over_the_lint.

Passages: `.data/op-responses/202609181645_hila-std-tier-persistence.md:33-43`, `mock/DESIGN.md.tmpl:23-23`.

#### `the_founding_research_orders_two_writers_by_registration_order_or_explicit_priority`

A tenet. It says: A research note of the founding round says the scheduler builds an edge from a writer of a resource to each reader of it, and for every pair of units that both write one resource an edge between them, whose direction is determined by registration order or explicit priority.

Finding: The research note orders two writers of one store by registration order or explicit priority; the founding spec's domain 08 resolves write-write ambiguity by scheduling hint priority with most dependents first and then a deterministic fallback. The registry row `two_writers_of_one_store_are_ordered_by_hint_priority` holds the spec's side.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:589-594`.

#### `an_append_past_an_accumulator_s_reserved_capacity_is_a_contract_violation_that_asserts_and_never_drops_silently`

A tenet. It says: As of 2026-06-20, appending past the reserved capacity of a fixed-capacity accumulator is a capacity-contract violation, like an out-of-bounds index, and not a recoverable run-time condition. The append asserts that the live length is below the capacity and panics before the write, so the write never leaves the reserved buffer and the record is never dropped silently. A consumer sizes the accumulator for its maximum appends per frame, and that is the contract.

Finding: The row says over-appending asserts and panics; on 13 November he chose that the overflow behaviour be configurable by the consumer (a const, generic or trait parameter picking refuse or assert, default refuse as a value from run).

Passages: `.data/op-responses/202611130550_four-calls-that-shape-the-hilavitkutin-api-revision.md:20-42`.

#### `an_accumulator_append_saturates_at_the_reserved_capacity_which_equals_the_record_count_at_build`

A tenet. It says: Accumulator appends saturate at the reserved capacity, as a soundness guard, and the capacity equals the record count at build. Fixtures and consumers size with headroom, and a silently dropped append means checking the live count against the capacity first.

Finding: The row says an over-capacity append saturates (silently dropped) as a soundness guard; he chose on 13 November that overflow is configurable by the consumer (refuse or assert, default refuse, nothing silently dropped), and row an_append_past_an_accumulator_s_reserved_capacity_is_a_contract_violation_that_asserts_and_never_drops_silently also says it asserts.

Passages: `.data/op-responses/202611130550_four-calls-that-shape-the-hilavitkutin-api-revision.md:20-42`.

#### `only_a_column_contributes_per_record_bytes_to_the_morsel_working_set_and_the_other_store_markers_contribute_zero`

A tenet. It says: Only a column contributes its element bytes to the per-record morsel working set. A resource and a staged resource are singletons, accumulator fibers dispatch unit-outer with no morsel window, and a virtual is fired, so those four markers contribute zero with no column-value bound, which lets a resource over a value that is not a column value still be sized. The first slice had bounded all four on the column value and broke real carriers.

Finding: The row gives a resource zero bytes in the morsel budget, while the founding spec says write resource collections count toward the L1 morsel budget (the later row a_resource_and_a_staged_resource_contribute_their_declared_footprint_to_the_morsel_budget_and_an_accumulator_contributes_zero restores that).

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:549-555`.

#### `a_resource_handle_s_members_decompose_into_per_member_shape_bound_columns_in_the_unified_store`

A tenet. It says: As first stated in the round of 2026-06-19, a resource is a handle whose field, sequence and map members each decompose into their own column in the unified type-erased columnar store, keyed by shape so that a field type appearing in many resources collapses to one column. The resource record holds only the pointers to those columns, in a separate handle store keyed by resource id with its own pointer provenance, and there is no private value arena. On that statement the one-record blob of the shipped drain was named the drift.

Finding: The row says members decompose into per-member shape-bound columns, while the bench-decided storage addendum retracts exactly that reading and keeps the value as a one-record blob (see tenet::a_resource_value_is_a_one_record_blob_and_the_bench_reverses_the_decomposition_to_shape_bound_columns).

Passages: `mock/research/202606210600_resource-storage-model-canonical-addendum.md:70-79`.

#### `as_of_round_202607193110_a_replaceable_swap_is_a_member_by_member_copy_into_column_slots_and_not_a_blob_write`

A tenet. It says: As of the revert round of 2026-07-19, the design stated that a swap of a replaceable value is a member-by-member copy of each leaf into its column slot, the same path as the initial drain, and not an in-place blob write of the whole value, with scalars written one per leaf and sequences and maps written element by element consecutively. Until that install could be built the swap returned to marking the store dirty without writing, which is honest about doing nothing rather than doing the wrong thing, the signature taking the value stayed, and the test became a catalogued red. The statement was made in the design because the code could not then express it.

Finding: A2-5 (tier 2) says the member-by-member sentence was a tier-4 input memo mis-cited and the storage addendum decided a one-record blob layout, while the row states member-by-member as the design; row a_replaceable_swap_installs_the_whole_value_as_one_blob_write_through_the_drain_s_selector_and_the_member_by_member_wording_is_refuted says the opposite.

Passages: `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:75-91`.

#### `the_effective_canon_is_the_consolidation_spec_as_modified_in_order_by_seven_registered_amendments`

A tenet. It says: The effective canon is the consolidation spec as modified, in order, by seven registered entries: r2, the rechart together with r3, r4, the fairness correction, r5, the round-level amendments, and addendum A1 itself.

Finding: The row says the effective canon is the spec modified by exactly seven entries in A1's order; A2 (later, ruled) says the unified-engine amendment was missing from A1's registry and registers it by date before the rechart, and its registry state adds the storage addendum and A2 itself.

Passages: `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:48-52`, `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:96-102`.

#### `the_run_returns_the_foundation_outcome_type_chosen_by_the_run_configuration_and_the_pipeline_result_enum_is_retired`

A tenet. It says: The scheduler's run returns the foundation outcome type, whose success and error types are determined by the consumer's run configuration and propagated through the builder typestate by the existing dispatch routing. The old three-variant pipeline result enum is retired, and the result type uses the stack's own outcome primitive.

Finding: The founding spec says Pipeline::run returns a PipelineResult status of completed, failed or poisoned, and A1-3 charts that status surface into the completion arc (row the_completion_arc_charts_the_pipeline_result_status_surface_the_stealing_executor_and_schedule_introspection), while this row retires that enum for an outcome type.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:2159-2167`, `mock/research/202606111400_canonical-design-addendum-a1.md:72-91`.

#### `the_single_core_dispatch_is_a_column_capable_inline_walk_with_no_stored_function_pointer_and_the_slot_array_dispatch_is_deleted`

A tenet. It says: The dispatch walk is extended past resource-only to project a unit's read and write columns alongside its resources, so a fiber's unit sequence monomorphises into one straight-line body with no stored function pointer. The column-capable walk is the only single-core dispatch path, and the per-unit erased slot dispatch is deleted.

Finding: His unified-engine ruling of 2026-06-06 says there is no separate single-core dispatch path and names a single-core cons-list walk framing as drift, while this row, from the day before, makes the column-capable walk the single-core dispatch path.

Passages: `mock/research/202606061000_canon-amendment-unified-engine.md:7-9`, `mock/research/202606061000_canon-amendment-unified-engine.md:25-25`.

#### `the_nested_value_carrier_walk_runs_a_trunk_over_a_carrier_of_fibers_and_the_scheduler_run_is_re_pointed_onto_it_in_a_later_step`

A tenet. It says: The trunk level of dispatch is a walk over a nested carrier of fibers, directly above the per-fiber walk, and it produces the same column state as the per-fiber walk over the flat concatenation of those fibers. The scheduler's run is not re-pointed onto the nested carrier in that step; a later step builds the nested phase, trunk and fiber carrier from the plan and replaces the flat walk.

Finding: This row has the trunk level as a nested fiber carrier that a later step builds from the plan to replace the flat walk; the op-locked A1 elevates const data plus const-gated DCE over a flat carrier as canon and says r4 replaced the nested-carrier construction, as does the row `trunk_isolation_comes_from_a_flat_carrier_walk_gated_by_const_phase_and_trunk_and_the_nested_value_carriers_are_superseded`.

Passages: `mock/design_rounds/202606070048/202606070048_topic.gate2-g2-0a-runtrunk-fibercons.md:16-20`, `mock/design_rounds/202606070048/202606070048_topic.gate2-g2-0a-runtrunk-fibercons.md:24-24`, `mock/research/202606111400_canonical-design-addendum-a1.md:147-150`, `mock/research/202606111400_canonical-design-addendum-a1.md:184-185`.

#### `the_scheduler_metrics_were_first_a_consumer_resource_populated_by_a_provided_metrics_unit_with_no_kernel_write`

A tenet. It says: In the first slice-3 design, the scheduler metrics are an ordinary consumer resource populated by a provided metrics unit that reads and writes it through the normal resource accessor, with no kernel special-casing. A kernel that wrote the metrics only when registered would need structural membership testing that walls without specialization.

Finding: The replacing changelist says this thin slice-3a design was deprecated because it hit a hard compile wall, and the source file is marked deprecated; the replacement is the row `the_scheduler_metrics_are_engine_owned_in_a_meta_block_written_by_the_scheduler_and_read_through_a_meta_access_gated_context_accessor`.

Passages: `mock/design_rounds/202606090400/202606090400_changelist.doc.lock.md:62-69`.

#### `a_resource_column_or_virtual_is_registered_through_the_one_with_verb_by_constructing_its_library_wrapper_at_the_call_site`

A tenet. It says: A resource, a column and a virtual are registered through the one with verb by constructing a library wrapper at the call site, with one value form per kind.

Finding: This row has consumers construct a library wrapper per kind at the call site; the later reshape sketch eliminates the wrapper structs for sized cases, as does the row `a_consumer_type_is_itself_the_builder_input_and_the_kind_wrappers_are_deleted_so_values_are_passed_bare`.

Passages: `mock/research/sketches/202605091700-builder-provider-shape/OUTCOME.md:21-24`, `mock/research/sketches/202605101036-builder-input-reshape/HYPOTHESIS.md:9-11`.

#### `the_capacity_lift_is_made_for_the_one_capacity_the_arvo_wrappers_consume_with_one_witness_clause_per_function`

A tenet. It says: The capacity lift to a Cap is made only for the one capacity parameter that feeds the arvo wrappers, and each function carries exactly one witness clause for it, which propagates up the caller chain as that one clause and not as a product. Witness propagation through the engine's caller graph does not blow up the trait solver. Every other capacity stays a plain usize.

Finding: This sketch lifts one capacity through a witness clause and leaves every other capacity a plain `usize`; the later capacity-as-type findings record his direction to never use bare arrays or `usize` in live code and his refinement to one generic marker with no witness clause, and A1 note 3 records the generic-constant-expression ceilings.

Passages: `mock/research/sketches/202605131500-witness-propagation/OUTCOME.md:29-44`, `mock/research/sketches/202605301145_capacity-as-type/FINDINGS.md:5-8`, `mock/research/sketches/202605301145_capacity-as-type/FINDINGS.md:125-130`.

#### `fixed_capacity_is_a_non_generic_capacity_trait_implemented_by_a_macro_generated_ladder_of_exact_rungs_binding_literal_length_arrays`

A tenet. It says: Fixed capacity is a non-generic capacity trait that carries the array as an associated type, with a macro-generated ladder of rungs, each binding a literal-length array.

Finding: This row has a macro-generated ladder of fixed rungs, while his refinement in addendum 2 of the same findings says to drop the macro rung ladder for one generic dimension marker, as in the row `fixed_capacity_is_one_generic_dimension_marker_implementing_the_capacity_trait_and_the_macro_rung_ladder_is_dropped`.

Passages: `mock/research/sketches/202605301145_capacity-as-type/FINDINGS.md:25-27`, `mock/research/sketches/202605301145_capacity-as-type/FINDINGS.md:125-130`.

#### `gate_one_takes_registration_as_canonical_with_a_build_time_check_that_it_is_the_plan_s_rcm_reordered_topological_order_and_the_reordering_automation_is_deferred`

A tenet. It says: The compile-time unit sequence is walked in registration order, devirtualised, and the engine requires and validates that registration order is the plan's RCM-reordered topological order, so RCM is the dispatch order and not the arena layout only. The automation of the within-equal-depth reordering, where the engine would reorder the sequence itself, is the codegen flattener's and is deferred until the generic-const-expression soundness migration.

Finding: This row defers the reordering automation until the generic-constant-expression migration, while his 2026-06-08 decision (a) makes RCM-row recovery a near-term priority and a later sketch proved a devirt-free auto-RCM walk; his decision (b) accepts the registration constraint only as a provisional compromise.

Passages: `mock/research/sketches/202606060730_runfibercol-accumulator-inline/FINDINGS.md:62-80`, `mock/research/202606081500_engine-roadmap-r2.md:91-92`.

#### `a_per_core_program_over_an_enumerated_set_of_per_trunk_monomorphisations_cannot_be_built_in_pure_rust_for_an_arbitrary_grouping`

A tenet. It says: Enumerating the distinct phase and trunk pairs of a grouping at compile time and dispatching one per-trunk monomorphisation per pair is not expressible in pure Rust on the pinned nightly. Turning the grouping's const data into a type-level list needs a proc-macro. The const-gated mechanism works for one fixed, known phase and trunk and not for an engine-derived enumeration over an arbitrary grouping.

Finding: This row says enumerating phase and trunk pairs into per-trunk monomorphisations is inexpressible in pure Rust; the op-locked A1 makes const data plus const-gated DCE over a flat carrier the canonical mechanism and names its shipped `IsRoot` and `PhaseAt` carrier structs, as does the row `the_outer_dispatcher_drives_the_shipped_per_trunk_walk_keyed_by_trunk_only_and_phase_order_is_a_runtime_loop`.

Passages: `mock/research/sketches/202606070906_gate2-pairs-enumeration-dispatch/FINDINGS.md:12-14`, `mock/research/202606111400_canonical-design-addendum-a1.md:147-150`, `mock/research/202606111400_canonical-design-addendum-a1.md:154-157`.

#### `the_two_writers_refusal_rests_on_the_data_bound_form_because_columns_bound_from_data_have_no_list_for_a_type_level_mask_to_fold`

A tenet. It says: The contract on two writers of one column can rest on the data-bound form. Under the plan's default the stored columns are bound from data, so the type-level mask has no list to fold for them and holds only where a list is fixed in the engine's types. The data-bound form is also safe without the check, since the split hands each staged column to at most one kernel and a second claimant would get nothing rather than an alias. What the check adds is that the second writer is refused at bind, by name, and not found at the step. The price is that the refusals the typed form makes at compile time become values.

Finding: The row has the second writer of one column refused at bind and rests the contract on data-bound binding; asked what should happen with two writers of one frame-buffered column (refuse or priority), he answered "Schedule should exist for this specifically", and asked about the data-bound way he chose "Decide in the api round".

Passages: `mock/research/sketches/202611122242_frame-view-typestate/FINDINGS.md:74-74`, `mock/research/sketches/202611122242_frame-view-typestate/FINDINGS.md:91-91`, `.data/op-responses/202611130726_does-anyone-here-know-what-hilavitkutin-is.md:258-278`, `.data/op-responses/202611130508_the-canon-goes-provisional-and-hilavitkutin-is-revised-in.md:162-185`.


## Part two: points the record states two ways

### Dispatch below ten thousand records

The founding spec keeps the indirect per-fiber dispatch below ten thousand records in two places and lists it as superseded in a third, and the later rounds each took one of the two readings. 6 rows. The question rows carry the options.

#### `the_dispatch_form_depends_on_the_record_count_below_or_above_ten_thousand`

A tenet. It says: Below ten thousand records the dispatch is indirect per fiber or one mega function per trunk, and above ten thousand records it is one mega function for the whole schedule.

Finding: Line 1168 keeps the indirect per-fiber approach below ten thousand records; lines 1653-1654 of the same spec say that approach is superseded by the trunk-level and whole-schedule ones. Open question whether_the_indirect_per_fiber_dispatch_stays_for_fewer_than_ten_thousand_records_or_is_superseded. The reader of slice 01 classed it a and named the tension; classed b here so that both founding conflicts of this kind are treated alike.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1168-1168`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1653-1654`.

#### `the_dispatch_approach_is_chosen_by_record_count_between_fiber_level_trunk_level_and_whole_schedule_functions`

A tenet. It says: Dispatch is measured against hand-fused code in five approaches, and the choice among them follows the record count: below 10K records the indirect per-fiber or the trunk-level approach, whose smaller function bodies give better icache and register allocation, and above 10K records the whole-schedule function, where call setup is amortised over the morsel iterations.

Finding: Lines 1558-1560 give the indirect per-fiber or the trunk-level approach below ten thousand records; lines 1653-1654 of the same spec say the indirect per-fiber approach is superseded. Open question whether_the_indirect_per_fiber_dispatch_stays_for_fewer_than_ten_thousand_records_or_is_superseded. The reader of slice 01 classed it a and named the tension; classed b here so that both founding conflicts of this kind are treated alike.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1547-1562`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1653-1654`.

#### `whether_the_indirect_per_fiber_dispatch_stays_for_fewer_than_ten_thousand_records_or_is_superseded`

A question, open. It asks: The founding spec names the indirect per-fiber approach, beside the trunk-level one, for fewer than ten thousand records, and its own list of replaced material says the indirect per-fiber approach was superseded by the trunk-level and whole-schedule approaches for its overhead. Which holds for fewer than ten thousand records?

Its options on record:

1. below ten thousand records the dispatch is the indirect per-fiber approach or the trunk-level one (the row-size passage: '<10K records: C or D')
2. the indirect per-fiber approach is superseded by the trunk-level and whole-schedule approaches, its 1.17x overhead unacceptable (the list of replaced material: 'Approach C (initially preferred) → superseded by D/E')

Finding: A question row added in this pass: the founding spec keeps the indirect per-fiber approach below ten thousand records in two places and lists it superseded in a third, and nothing of his read settles it.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1168-1168`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1558-1560`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1653-1654`.

#### `which_dispatch_approach_variants_the_first_version_ships`

A question, open. It asks: Does the per-fiber indirect dispatch approach ship in the first version, or does the mono-tuple baseline ship in its place?

Its options on record:

1. Ship `MonoTuple` + `TrunkMega` + `ScheduleMega`; `IndirectPerFiber` is kept available only as a BACKLOG entry (dispatch codegen topic, Axis B).
2. `select_approach` picks one of three `DispatchApproach` variants (`IndirectPerFiber`, `TrunkMega`, `ScheduleMega`), and small record counts with many fibers pick `IndirectPerFiber` (select_approach base round).
3. Three `DispatchApproach` variants ship in v1: ScheduleMega, TrunkMega, IndirectPerFiber (the design template).

Finding: The dispatch-codegen round ships MonoTuple, TrunkMega, ScheduleMega with the per-fiber indirect approach backlog only, while the select_approach base round and the template ship the indirect approach and choose it for many fibers; the founding spec calls it superseded at 1.17x yet still lists it below 10K (line 1168), and nothing of his settles it.

Passages: `mock/design_rounds/202605101036/202605101036_topic.dispatch-codegen.md:216-216`, `mock/design_rounds/202605250600/202605250600_topic.hila-c2-select-approach-base.md:25-29`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1653-1655`.

#### `the_base_approach_selection_picks_by_record_count_and_fiber_count_and_threshold_tuning_by_benchmark_follows`

A tenet. It says: As a first body, approach selection picks the whole-schedule function when the record count is at least ten thousand. Below that, a single fiber picks the trunk-level function because the mega function fits the working set, and several fibers pick the per-fiber indirect call as the lowest-overhead split. Benchmark-tuned thresholds land in a follow-up.

Finding: This row picks the per-fiber indirect call for several fibers below 10K, while the founding spec marks that approach superseded at 1.17x overhead and another row (the_indirect_per_fiber_approach_is_superseded_by_the_trunk_and_schedule_approaches_for_its_overhead) says the same.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1653-1655`, `mock/design_rounds/202605250600/202605250600_topic.hila-c2-select-approach-base.md:25-29`.

#### `the_indirect_per_fiber_approach_is_superseded_by_the_trunk_and_schedule_approaches_for_its_overhead`

A tenet. It says: The indirect per-fiber dispatch approach, which was initially preferred, is superseded by the trunk-level and whole-schedule approaches on the bench findings, because its 1.17x overhead is unacceptable.

Finding: Lines 1653-1654 list the indirect per-fiber approach as superseded by the trunk-level and whole-schedule ones; lines 1168 and 1558-1560 of the same spec keep it below ten thousand records. Open question whether_the_indirect_per_fiber_dispatch_stays_for_fewer_than_ten_thousand_records_or_is_superseded. The reader of its slice classed it a as founding text; it is classed b here so that the founding rows on both sides of this point are treated alike.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1653-1654`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1168-1168`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1558-1560`.


### No external crate, beside the arrayvec line

The founding spec says the crates take no std, no alloc and no external crate, and also names arrayvec as fixed-size storage. His later choice that the std tier is gone settles the std half only. 2 rows.

#### `the_hilavitkutin_crates_depend_on_no_polka_crate_no_std_no_alloc_and_no_external_crate`

A tenet. It says: None of the three hilavitkutin crates depends on any polka crate, on std, on alloc or on any external crate.

Finding: Lines 62-63 say no std, alloc or external crate in the three crates; the same spec lists a std tier at line 280 and arrayvec as fixed-size storage at 308-310, and the registry holds this as open question whether_the_hilavitkutin_crates_use_no_std_and_no_external_crates_given_the_std_tier_and_arrayvec.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:62-63`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:280-280`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:308-310`.

#### `whether_the_hilavitkutin_crates_use_no_std_and_no_external_crates_given_the_std_tier_and_arrayvec`

A question, open. It asks: Is it true that none of the three hilavitkutin crates depends on std or on any external crate, given that the platform tiers include a std tier and the fixed-size structures may use arrayvec?

Its options on record:

1. no dependency on std, alloc or external crates in any of the three crates
2. the std platform tier uses std::alloc, std::thread and std::time::Instant, and fixed-size structures use stack arrays, arrayvec or SlabColumn

Finding: His choice that no hilavitkutin crate spawns threads and that the std tier is gone answers the std half of the question; the founding spec still names arrayvec as fixed-size storage beside its line that the crates take no external crate, and nothing of his read settles that half. The first reader classed it a for the std half alone.

Passages: `mock/design_rounds/202609181402/202609181401_topic.hilavitkutin-on-arvo-dev.md:81-81`, `mock/design_rounds/202609181402/202609181402_changelist.doc.lock.md:13-13`.


### Link-time registration

The founding spec and the principles keep registration through inventory at link time, and the design template of the linking crate rejects inventory and every constructor-style registry. 3 rows.

#### `link_time_registration_through_inventory_is_complete_before_work_starts`

A tenet. It says: Link-time registration through the inventory mechanism is complete before any work starts.

Finding: The founding spec lists inventory link-time registration as complete before work; row discovery_is_pull_based_only_and_linker_section_scanning_registration_is_rejected (agent-written linking template, line 100) says inventory discovery is rejected; the registry already holds both as question whether_a_link_time_registration_mechanism_is_part_of_the_design_or_rejected, open.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:2227-2227`, `mock/crates/hilavitkutin-linking/DESIGN.md.tmpl:100-100`.

#### `whether_a_link_time_registration_mechanism_is_part_of_the_design_or_rejected`

A question, open. It asks: The founding spec lists link-time registration through the inventory mechanism as complete before work starts, while the design template of the linking crate rejects inventory, constructor, distributed-slice and init-array discovery in favour of pull-based discovery. Which holds for registering anything at link time?

Its options on record:

1. link-time registration through inventory is complete before work starts (founding spec cross-cutting constraint)
2. inventory, ctor, distributed_slice and .init_array discovery are all explicitly rejected; discovery is pull-based only (linking design template)

Finding: The founding spec and the principles keep inventory link-time registration, while the linking design template rejects inventory, ctor, distributed_slice and init_array; nothing of his settles it. Row tenet::discovery_is_pull_based_only_and_linker_section_scanning_registration_is_rejected holds the other side.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:2227-2227`, `mock/crates/hilavitkutin-linking/DESIGN.md.tmpl:100-100`, `mock/PRINCIPLES.md.tmpl:78-79`.

#### `discovery_is_pull_based_only_and_linker_section_scanning_registration_is_rejected`

A tenet. It says: The linking crate does not scan linker sections for registration. The inventory, constructor, distributed-slice and init-array discovery mechanisms are all explicitly rejected, and discovery is pull-based only: the host asks for a named symbol and the library must export it.

Finding: The row rejects inventory and constructor-style registration for discovery across the dlopen boundary, while PRINCIPLES.md.tmpl, his direction, says extension across an ABI boundary uses registered traits with repr(C) descriptors "(inventory pattern)" and "#[register] (or the stack's inventory equivalent)".

Passages: `mock/crates/hilavitkutin-linking/DESIGN.md.tmpl:100-100`, `mock/PRINCIPLES.md.tmpl:74-79`, `mock/PRINCIPLES.md.tmpl:140-145`.


### The accumulator as a fourth store kind

The founding spec has three store types. The accumulator round adds a fourth marker, and the standalone spec lists four shapes. On 13 November he left the shape of the accumulator to whichever is best for typestate and plumbing, which does not say whether it counts as a fourth kind. 3 rows.

#### `whether_the_accumulator_is_a_fourth_store_kind`

A question, open. It asks: The founding spec and the api design template say there are three store types; the round that landed accumulators adds a fourth marker beside them and the standalone spec lists four store shapes. Is the accumulator a fourth store kind, or a column under another access?

Its options on record:

1. Three store types, as the founding spec and the api design template say.
2. Four, with the accumulator as a distinct marker, as the accumulator round and the standalone spec say.

Finding: The founding spec's consolidated domain 06 says three store types, while the agent-written accumulator round adds a fourth Accum<T> marker; op's locked input-versus-accumulator model there is about two kinds of columns, not the marker count. Row tenet::an_accumulator_is_a_column_written_by_append_and_a_fourth_store_marker holds the other side.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:491-496`, `mock/design_rounds/202606041800/202606041800_topic.accumulator-columns.md:16-16`.

#### `an_accumulator_is_a_column_written_by_append_and_a_fourth_store_marker`

A tenet. It says: The design round that landed accumulators reports the maintainer's model as two kinds of column: an input column, whose buffer is sized to a build-time record count, populated before the frame and read and written by index, and an accumulator column, which is written by append during the frame, growing from zero toward a capacity, and whose populated prefix a later drain reads into persistence. It says the maintainer chose this two-kind surface deliberately and not a single global record count. The round represents the accumulator as a distinct store marker beside the three existing ones, the fourth on the same pattern, because making it a role or mode of the column would put indexed random write and ordered append on one marker.

Finding: The founding spec lists three store types (Resource, Column, Virtual) while the row adds an accumulator as a fourth marker (see tenet::there_are_three_store_types_and_one_scheduling_mechanism); the two-kind column model is his per the round, but on 13 November he left the fourth-kind question to whichever is best typestate-wise.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:491-504`, `.data/op-responses/202611131026_the-hilavitkutin-record-sits-partly-on-an-unlanded-branch.md:98-116`.

#### `the_standalone_spec_lists_four_store_shapes_with_the_accumulator_among_them`

A tenet. It says: The standalone spec says state lives in one of four store shapes and there is nothing else: a singleton store of one value, a collection store of N records in columnar layout accessed through morsel windows, an accumulator, which is an append-only output store a unit pushes records into with a reserved capacity fixed at build, and a fired marker carrying no data.

Finding: The founding spec's store types section gives three store types and one scheduling mechanism, resource, column and virtual, while the row's standalone passage gives four shapes with the accumulator among them and says there is nothing else; the other row is there_are_three_store_types_and_one_scheduling_mechanism.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:487-496`.


### Entry or record as the innermost unit

The principles list entry among the dead terms and end the vocabulary at record. The founding spec ends its hierarchy at entry. Both documents are his. 3 rows. The options were written for this file:

1. Record is the innermost unit and entry is a dead term, as the principles say.
2. Entry stays the innermost unit of the hierarchy, as the founding spec says, and record names one data point in a column.

#### `entry_is_a_dead_term_and_the_canonical_vocabulary_ends_at_micro_morsel_and_record`

A tenet. It says: The canonical terms are pipeline, core, phase, waist, trunk, fiber, branch, bridge, morsel, micro-morsel and record. The dead terms are chain, chain group, partition, archetype, entity, row, entry and order, and they are never used.

Finding: PRINCIPLES.md.tmpl lists entry among the dead terms and ends the vocabulary at record, while the founding spec's hierarchy ends at entry and defines it as the canonical innermost unit; row tenet::the_hierarchy_runs_from_pipeline_down_through_phase_trunk_fiber_and_morsel_to_entry holds the other side.

Passages: `mock/PRINCIPLES.md.tmpl:281-287`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:71-80`.

#### `the_hierarchy_runs_from_pipeline_down_through_phase_trunk_fiber_and_morsel_to_entry`

A tenet. It says: The vocabulary is a hierarchy from the coarsest to the finest level: pipeline, core, phase with waist, trunk, fiber with branch and bridge, morsel, micro-morsel and entry.

Finding: The founding spec ends its hierarchy at entry and calls it the innermost unit; the principles list entry among the dead terms and end the vocabulary at record, as the row entry_is_a_dead_term_and_the_canonical_vocabulary_ends_at_micro_morsel_and_record says. Both documents are his. The reader of its slice classed it a as founding text; it is classed b here so that both sides of this point are treated alike.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:71-80`, `mock/PRINCIPLES.md.tmpl:281-287`.

#### `an_entry_is_one_record_and_one_operation_at_register_level`

A tenet. It says: An entry is one record taken with one operation, at register level, and it is the innermost unit.

Finding: The founding spec's vocabulary table defines entry as the innermost unit, which takes the second option of this group, while the principles list entry among the dead terms; the seed commit `680a672` listed entry among the canonical terms, and the style sweep `59b3c18` of 2026-05-05 moved it to the dead terms. It was ratified as founding text and is listed here with the two rows above, so that every side of this point waits on him alike.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:80-80`, `mock/PRINCIPLES.md.tmpl:283-287`.


### Whether hilavitkutin's own columns are double-buffered

2 rows.

#### `whether_hilavitkutin_s_own_columns_are_double_buffered`

A question, open. It asks: The founding round resolves that hilavitkutin has two access modes and that the frozen and live views, the double-buffered view system and the arena are loimu's own layer. An expert memo of the storage model round says hilavitkutin's own columns are double-buffered, frozen and live, for the pass boundary. Does hilavitkutin itself buffer a column across the pass boundary, or does the consumer?

Its options on record:

1. The consumer. Hilavitkutin has read and write and no buffered store; a consumer that wants a frozen read holds it as its own columns and rotates them between passes, as the founding round resolved for loimu.
2. Hilavitkutin. A column is double-buffered at the pass boundary by hilavitkutin itself, as the loimu heritage memo says.

Finding: The founding-round Q6 leaves buffering to loimu's own layer and the expert memo says hilavitkutin's own columns are double-buffered (see tenet::an_expert_memo_says_hilavitkutin_s_columns_are_double_buffered_for_the_pass_boundary); on 13 November he answered neither, saying he believed the spec already answers it at least in intent.

Passages: `mock/design_rounds/202604200055/202603141800_topic.hilavitkutin-core-design.md:643-661`, `mock/research/202606210600_expert-loimu-heritage-storage-model.md:285-293`, `.data/op-responses/202611130808_read-the-whole-hilavitkutin-spec-and-port-it-to-a-registry.md:114-140`.

#### `an_expert_memo_says_hilavitkutin_s_columns_are_double_buffered_for_the_pass_boundary`

A tenet. It says: An expert memo of the storage model round says that hilavitkutin's columns are double-buffered, frozen and live, for the pass boundary, and that hilavitkutin's design has no temporal prediction and no triple buffering, because a pipeline execution engine computes column values and does not extrapolate them between passes. The memo names no source for the sentence.

Finding: The memo says hilavitkutin's own columns are double-buffered; the founding core-design round resolves read and write only, with the double-buffered view system loimu's own layer (row loimu_keeps_its_seven_access_modes_and_maps_them_to_read_and_write_at_the_boundary, open question whether_hilavitkutin_s_own_columns_are_double_buffered).

Passages: `mock/design_rounds/202604200055/202603141800_topic.hilavitkutin-core-design.md:640-662`, `mock/research/202606210600_expert-loimu-heritage-storage-model.md:289-293`.


### The alignment on deallocate

The founding spec declares the free path of the memory contract with a pointer and a length. A later change list of an agent adds the alignment. 2 rows. The options were written for this file:

1. The founding signature stands and the free path takes no alignment.
2. The free path takes the alignment of the allocation, and the founding signature is amended.

#### `deallocate_takes_the_alignment_because_every_caller_knows_it_and_a_header_or_a_remembering_provider_were_rejected`

A tenet. It says: Of three shapes for supplying the alignment at the free site, storing it in a header beside each allocation, having each provider remember its own allocations, or passing it back in, the contract passes it back in, as the allocator underneath requires the full layout and not just the pointer. A header costs memory on every block and is wrong for a bump or arena provider that has no per-block header, and a remembering provider makes a stateless provider stateful for no reason. Every caller already knows the alignment it asked for, so the information was dropped on the way down and never missing. The safety contract states that pointer, length and alignment all match the prior allocation, and a provider that ignores alignment may leave the parameter unread. The earlier contract carried only the pointer and the length.

Finding: The founding spec's MemoryProvider declares `deallocate(&self, ptr, len)` with no alignment, while the row makes `deallocate` take `align`; the change was made by an agent changelist and no ruled amendment I opened (A1, A2) speaks to it.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:289-293`.

#### `a_deallocation_is_given_the_same_alignment_as_its_allocation`

A tenet. It says: Freeing memory through the memory provider takes the same alignment the matching allocation was given, beside the same pointer and length, and all three are part of the safety contract. A provider whose free path ignores alignment may leave the parameter unread.

Finding: The founding spec declares the platform contract as `deallocate(&self, ptr: *mut u8, len: usize)`, with alignment on allocate alone; the row says the free path takes the same alignment beside the pointer and the length and that all three are the safety contract. No registry row holds the other side.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:288-296`.


### The default wake: spin then park, or park at once

The founding spec and the principles wake the pool by spinning 128 times and then parking. A wake bench run by an agent set the default budget to zero. The record shows he called the first result too convenient, and no word of his blesses the second. 4 rows.

#### `the_spin_then_park_middle_tier_is_wired_into_both_frame_waits_with_a_consumer_tunable_budget_and_the_shutdown_wait_stays_park_only`

A tenet. It says: The canonical spin-then-park middle tier is wired unconditionally into the frame waits. The worker frame wait and the scheduler done wait spin a bounded budget, re-checking the word each iteration so the lost-wakeup discipline is untouched, and then park through the same load-check-wait pattern as before. The budget is a consumer-tunable run configuration constant defaulting to the canonical performance-core figure of 128, and zero restores park-immediately exactly. The shutdown wait stays park-only, since latency is irrelevant there. Wiring the telemetry-driven tier selection remains a follow-up the ledger names, because the telemetry is undriven and a zero prediction selects the unbounded spin-only tier.

Finding: The row's budget default of 128 is flipped to 0 by row the_spin_budget_default_flips_from_128_to_zero_to_the_measured_winner_while_the_machinery_stays_and_consumers_may_override and by a_wake_bench_rules_for_park_immediately_with_a_spin_budget_default_of_zero_and_the_machinery_shipping_tunable; the 128 figure is the founding spec's (line 778).

Passages: `mock/design_rounds/202607202351/202607202350_topic.wake-budget-default-zero.md:8-16`.

#### `the_spin_budget_default_flips_from_128_to_zero_to_the_measured_winner_while_the_machinery_stays_and_consumers_may_override`

A tenet. It says: The wake policy bench ruled against the bounded spin pre-roll on the measured host, because park-immediately wins at every size, a fixed per-frame spin penalty from wake-word cache-line contention and core competition. Per the prior round's evidence exit, the machinery stays, as the parameter, the constant and the budget-sweep test, and the default becomes the measured winner, so the spin budget default flips from 128 to 0. Consumers on hosts where the trade runs the other way override the constant.

Finding: The founding spec and the principles state a hybrid wake of spin 128 then park, while the row sets the default to 0 on an agent's bench; A2-4 says such a deviation result goes to him for a bless-or-rebuild ruling and I found no word of his blessing it.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:778-778`, `mock/PRINCIPLES.md.tmpl:238-239`, `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:64-73`.

#### `a_wake_bench_rules_for_park_immediately_with_a_spin_budget_default_of_zero_and_the_machinery_shipping_tunable`

A tenet. It says: The GATE-2 deviation 3, park-immediately, is resolved by bench on 2026-07-19. The canonical spin-then-park middle tier was built, with a budget parameter on both frame waits, a consumer-tunable setting and a budget-sweep equivalence test, and the wake policy bench ruled for park-immediately, reproducibly. Across seven wait policies, three sizes and three invocations, park is never worse and no spin policy is ever better. The round-1 point margins are retired as variance-inflated, and the spin iteration is an ISB instruction on aarch64 at 9 to 20 ns. The default is zero and the machinery ships tunable.

Finding: The founding spec says the pre-allocated pool wakes by a hybrid spin 128 then park, while the row makes park-immediately the ruling with the spin budget defaulting to zero. The registry row the_thread_pool_is_pre_allocated_and_wakes_by_spinning_first_and_then_parking carries the founding side, and the record of the re-examination shows he only flagged the first result as too convenient and asked for a second look.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:778-779`, `mock/research/seed/governance.md:152-164`.

#### `whether_the_default_wake_is_the_spin_then_park_hybrid_of_the_spec_or_park_immediately_ruled_by_the_wake_bench`

A question, open. It asks: Whether the default wake of the thread pool is the spin-then-park hybrid, or park-immediately, which the wake bench ruled for with a spin budget default of zero and the spin-then-park machinery shipping tunable.

Its options on record:

1. the spin-then-park hybrid as the default wake
2. park-immediately by default, spin-then-park built and tunable with default budget 0, as the wake bench ruled

Finding: The founding spec says the pool wakes by a hybrid spin 128 then park while the bench round ruled park-immediately with the budget defaulting to zero, so the question's two sides are the record's own conflict and nothing of his settles it, the re-examination record showing only that he called the first result too convenient.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:778-779`, `mock/research/seed/governance.md:152-164`.


### Calls drawn from the benches of the bare engine

On 13 November he said the benches of the engine against hand-written and plain Rust paths are a baseline and that no call is made on them yet (`OP-ANSWERS-21.md`). These 3 rows rest a call on such numbers. The options were written for this file:

1. The call is withdrawn until the headline features are built and benched, and the row is demoted or reworded to report the measurement only.
2. The call stands as an exception he names.

#### `deviation_two_core_ownership_mask_evidence_leaves_the_bless_standing_with_the_build_script_codegen_escalation_armed`

A tenet. It says: The evidence for the GATE-2 deviation 2, the core-ownership mask, was delivered: the five-check asm gate re-ran green with zero indirect calls in every dispatch body, and the perf gate stands as the second oracle. The bless holds, with the escalation to build-script code generation armed.

Finding: The evidence record keeps the bless standing on two oracles, the second of which is the engine-versus-std perf gate, and the escalation armed on either going red. He said on 13 November that those benches are a baseline and no calls should be made on them yet, against the row the_benches_of_the_engine_against_hand_written_and_plain_rust_paths_are_a_baseline_and_no_call_is_made_on_them_yet.

Passages: `mock/research/202611130850_the-record-read-whole/OP-ANSWERS-21.md:3-5`, `mock/research/202607202200_gate2-deviation-2-evidence.md:26-37`.

#### `the_perf_gate_uses_per_arm_calibrated_bars_over_a_persistent_pool_std_baseline_and_stays_blocking`

A tenet. It says: The flat 1.10 times bar of the perf gate retires. The gate demands per-arm calibrated bars, tight where the engine should win and with parity tolerance mid-range, the std baseline upgrades to a persistent-pool variant matching the engine's spawn-once discipline, and measurement variance is fixed with median-of-N runs. The gate stays blocking.

Finding: A1-8 (ruled 2026-06-11) fixes a blocking gate with per-arm bars tight where the engine should win over the engine-against-std benches; his 13 November words say those benches are a baseline and no calls should be made on them yet, so the blocking bars are the call drawn from them.

Passages: `mock/research/202606111400_canonical-design-addendum-a1.md:133-139`, `mock/research/202611130850_the-record-read-whole/OP-ANSWERS-21.md:1-8`.

#### `which_convergence_a_single_trunk_phase_uses`

A question, answered. It asks: For single-trunk phases: head-and-tail convergence or the N-way record split? The records were reversed back to head-and-tail in July, the N-way path is what runs on the trunk, and the design text carries both. Which is the design?

Its options on record:

1. Head-and-tail, finish the deletion: As r8 and the seed say. The N-way path in `worker.rs` is removed, the head-and-tail types get driven, the design's ceil-slice passages are corrected, a commutativity test is added.
2. N-way, retire head-and-tail: The N-way record-order split is the design; the unused head-and-tail types and the seed sentence go.
3. Both, chosen at plan time: Two inhabitants the planner picks between per fiber (commutative single-trunk: head-and-tail; ordered appends: N-way forward slices), stated in the contract.
4. Measure first: A bench under the harness with both arms on single-trunk workloads; the question returns with the numbers. The code stays as it is meanwhile.

His answer on record: Measure first

Finding: The answer "Measure first" rests the design call between head-and-tail convergence and the N-way record split on the numbers of a convergence bench, while he said later the same day that the engine's benches are a bare-bones baseline and no calls are made on them yet; the other row is the_benches_of_the_engine_against_hand_written_and_plain_rust_paths_are_a_baseline_and_no_call_is_made_on_them_yet.

Passages: `mock/research/202611130850_the-record-read-whole/OP-ANSWERS-21.md:3-5`, `mock/research/202611130850_the-record-read-whole/OP-ANSWERS.md:349-375`.


### Bare primitives in wire and disk layouts

The principles allow a bare primitive only behind a tracked allow at a foreign boundary that cannot be avoided. The templates of the extensions and persistence crates grant standing exemptions, and disagree with each other on the length fields. 3 rows. The options were written for this file:

1. The principles hold without exception: length fields take the stack's own types, and a bare primitive stands only behind a tracked allow.
2. Wire and disk layout types are a class of their own that is exempt, as the templates say.

#### `descriptor_length_fields_are_bare_u32_for_a_wire_stable_layout_and_each_is_bounded_by_a_ceiling_of_one_mebi_entries`

A tenet. It says: The three length fields of the descriptor are 32-bit integers so that the layout is wire-stable and independent of the platform, where an earlier shape used the pointer-sized integer and varied in byte width across pointer sizes. Each length is bounded when the descriptor is read by a ceiling of about a million entries, which is well above any plausible extension's surface and bounds the worst-case slice at roughly 16 MiB.

Finding: The row makes the length fields bare u32, while the same template's FFI-wire policy (row ffi_wire_descriptor_fields_use_an_arvo_newtype_where_one_is_wire_identical_and_otherwise_a_bare_primitive_by_deliberate_choice) puts USize on every len field and lists no bare u32 among its allowed ones, and PRINCIPLES says no bare primitives except a lint:allow with a tracked task.

Passages: `mock/crates/hilavitkutin-extensions/DESIGN.md.tmpl:25-25`, `mock/crates/hilavitkutin-extensions/DESIGN.md.tmpl:49-58`, `mock/PRINCIPLES.md.tmpl:17-40`.

#### `ffi_wire_descriptor_fields_use_an_arvo_newtype_where_one_is_wire_identical_and_otherwise_a_bare_primitive_by_deliberate_choice`

A tenet. It says: Descriptor fields that cross the boundary need a wire-stable footprint, so a two-tier rule applies. An arvo transparent newtype is used when its backing is wire-identical to the C type, and a bare primitive is kept where no wire-stable arvo newtype exists, because the width of a fixed-point backing comes from its strategy and is not pinned to the C integer width. Each such bare primitive carries a lint-allow comment recording the contract, raw pointer fields stay as they are, and a future fixed-width wire primitive would collapse the second tier into the first.

Finding: The row says USize slots into every len field, while row descriptor_length_fields_are_bare_u32_for_a_wire_stable_layout_and_each_is_bounded_by_a_ceiling_of_one_mebi_entries makes them bare u32; it also frames bare primitives as a deliberate permanent tier where PRINCIPLES allows them only as a tracked-for-remediation escape hatch.

Passages: `mock/crates/hilavitkutin-extensions/DESIGN.md.tmpl:25-25`, `mock/crates/hilavitkutin-extensions/DESIGN.md.tmpl:49-58`, `mock/PRINCIPLES.md.tmpl:17-40`.

#### `disk_layout_types_are_exempt_from_the_primitive_vocabulary_because_a_semantic_newtype_would_add_a_conversion_on_the_hottest_serialisation_path`

A tenet. It says: Disk-layout types are exempt from the primitive-vocabulary discipline. They exist to pin a byte layout across sessions, and giving them a semantic newtype would add a conversion call on the hottest serialisation path without changing the on-disk format.

Finding: The principles say bare primitives do not exist in this stack anywhere, with one escape, a tracked `lint:allow` at a genuinely unavoidable foreign boundary; the row grants a standing class-wide exemption to disk-layout types on a performance argument, with no tracked allow.

Passages: `mock/PRINCIPLES.md.tmpl:17-41`.


### Design documents of agents that disagree with each other

Each of these 10 rows is one side of a point two agent-written documents state differently, with no word of his on either side. The question rows carry the options.

#### `later_modules_of_the_providers_crate_add_column_storage_clock_and_memory_defaults_per_consumer_demand`

A tenet. It says: Later modules of the providers crate are meant to add default column storage, clock and memory providers, each when a consumer demands it.

Finding: The providers design says the crate already ships the default ColumnStorage backing and only the clock and memory defaults are future, and row the_default_column_storage_is_a_naive_arena_backed_placeholder_the_engine_stays_generic_over_and_the_columnar_engine_replaces_later agrees, while this row lists column storage as still to come.

Passages: `mock/crates/hilavitkutin-providers/DESIGN.md.tmpl:8-14`, `mock/DESIGN.md.tmpl:24-24`.

#### `whether_the_extension_host_dispatches_by_capability_or_by_provider`

A question, open. It asks: The workspace design template describes the extensions crate as dispatching capabilities, while the crate's own design template names the unit of dispatch a provider with a ProviderId. Which term and identifier does the extension host use?

Its options on record:

1. capability dispatch (workspace template: "capability dispatch, failure policy")
2. provider dispatch by stable identifier plus opaque vtable pointer (crate template: "provider dispatch by stable identifier plus opaque vtable pointer")

Finding: The workspace template says "capability dispatch" (row the_extensions_crate_is_the_domain_agnostic_host_orchestration_layered_over_the_linking_crate) and the crate template says provider dispatch (row the_extensions_crate_owns_the_descriptor_shape_pull_based_discovery_the_lifecycle_provider_dispatch_requirement_classes_and_the_failure_policy); the only settlement is an agent-written round retiring "capability", nothing of his.

Passages: `mock/DESIGN.md.tmpl:19-19`, `mock/crates/hilavitkutin-extensions/DESIGN.md.tmpl:3-9`, `mock/design_rounds/202605091700/202605091700_topic.provider-unification.md:28-37`.

#### `what_the_builder_call_is_that_adds_a_kit_or_a_resource`

A question, open. It asks: The workspace design template and the kit design template add a kit with an add-kit call, while the providers design template wires a kit and a resource with the builder's with call. Which builder calls add a kit and a resource?

Its options on record:

1. `.add_kit::<K>()` for a kit and `.resource::<T>(...)` or `builder.add_resource(...)` for a resource (workspace template, kit template)
2. `.with(InternerKit::<BYTES, ENTRIES>)` for a kit and `builder.with(Resource::new(...))` for a resource (providers template)

Finding: The workspace and kit templates add a kit with add_kit (row kits_compose_by_sequential_add_kit_calls_and_dependency_order_is_the_simplest_discipline_for_the_contains_all_proof) while the providers template and the later builder-shape round use one .with(value) verb; the later round is agent-written and nothing of his settles it.

Passages: `mock/DESIGN.md.tmpl:21-21`, `mock/crates/hilavitkutin-providers/DESIGN.md.tmpl:24-24`, `mock/design_rounds/202605091700/202605091700_topic.builder-shape.md:30-60`.

#### `whether_the_providers_crate_already_ships_the_default_column_storage_and_the_adapt_surface_or_only_the_interner`

A question, open. It asks: The workspace design template says the providers crate ships only the interner surface in its first version, with a default column storage added by a future module. The providers crate's own design template says it ships the arena column storage today, and its surface table also lists a default adapt unit, nine adapt metric resources and two adapt kits that its prose scope does not mention. What does the providers crate ship?

Its options on record:

1. v0 ships the interner value-type surface plus InternerKit; future modules add ColumnStorage / Clock / Memory defaults (workspace template)
2. ships the interner surface and the default ColumnStorage backing (ArenaColumnStorage) today, with Clock and libc/Unix MemoryProvider defaults to follow (providers template, prose)
3. also ships AdaptWu, the per-axis metrics resources and StandardAdaptKit / OffAdaptKit (providers template, surface table)

Finding: The workspace template says v0 ships only the interner surface (row the_providers_crate_v0_ships_the_interner_value_type_surface_and_a_declarative_interner_kit) while the crate template ships ArenaColumnStorage and the adapt kits (rows the_default_column_storage_is_a_naive_arena_backed_placeholder_the_engine_stays_generic_over_and_the_columnar_engine_replaces_later and the_adapt_kits_bundle_the_nine_metric_axes_with_a_standard_recipe_with_validated_defaults_and_an_off_recipe_that_disables_the_observer); nothing of his settles it.

Passages: `mock/DESIGN.md.tmpl:24-24`, `mock/crates/hilavitkutin-providers/DESIGN.md.tmpl:8-14`, `mock/crates/hilavitkutin-providers/DESIGN.md.tmpl:24-28`.

#### `whether_the_kit_crate_has_zero_dependencies_or_depends_on_the_api_crate`

A question, open. It asks: The kit design template opens by calling the crate zero-dependency, yet the trait's bounds and the authoring section name core primitives and markers of the api crate. Does the kit crate depend on the api crate?

Its options on record:

1. zero deps (kit template, opening: "`#![no_std]`, zero deps.")
2. the kit declaration depends on hilavitkutin-api for WorkUnitBundle, StoreBundle, Replaceable and the store markers (kit template: "Both bounds (`WorkUnitBundle`, `StoreBundle`) are core primitives in `hilavitkutin-api`")

Finding: The template's opening says zero deps while its trait section and the Cargo.toml say the kit crate depends on hilavitkutin-api (and row kit_author_crates_import_no_engine_item_and_depend_only_on_the_api_markers_and_the_kit_trait states the api dependency); nothing of his settles it.

Passages: `mock/crates/hilavitkutin-kit/DESIGN.md.tmpl:3-3`, `mock/crates/hilavitkutin-kit/DESIGN.md.tmpl:41-45`, `mock/crates/hilavitkutin-kit/Cargo.toml:9-10`.

#### `whether_the_builder_input_trait_gains_a_method_that_extracts_the_construction_value_from_a_carrier`

A question, open. It asks: In the builder value retention round, does the builder input trait gain a method that moves the construction value out of a carrier, or does it stay with its two associated types?

Its options on record:

1. `BuilderInput` gains `into_init(self) -> Self::Init` so `.with` can move the construction value out of the carrier and onto the `Staged` list.
2. No method is added to `BuilderInput` (it stays `Init` + `Dispatch`), so none of the 24 existing `impl BuilderInput` sites change.

Finding: The topic says BuilderInput gains an into_init method, and the source changelist built it without any method; nothing of his settles it.

Passages: `mock/design_rounds/202605282319/202605282319_topic.builder-value-retention.md:12-12`, `mock/design_rounds/202605282319/202605282319_changelist.src.lock.md:3-3`.

#### `the_per_unit_slot_array_dispatch_stays_as_the_shipped_per_fiber_mechanism_and_the_inline_walk_is_deleted_rather_than_extended`

A tenet. It says: The per-unit slot array and its collecting machinery stay as the shipped per-fiber dispatch mechanism. The sequential inline walk that was to be extended into a column-capable range-gated dispatch path is deleted instead of extended.

Finding: This row keeps the slot array and deletes the inline `RunFiber` walk; the row `the_single_core_dispatch_is_a_column_capable_inline_walk_with_no_stored_function_pointer_and_the_slot_array_dispatch_is_deleted` says the column-capable inline walk is the only path and the slot dispatch is deleted. Agent-written on both sides, no word of his settles it, and later rounds call `RunFiber` shipped.

Passages: `mock/design_rounds/202606051659/202606062330_changelist.src.lock.md:14-14`, `mock/design_rounds/202606051659/202606051659_changelist.doc.lock.md:22-22`.

#### `column_buffers_are_sized_by_the_per_run_record_count_and_belong_to_the_run_loop_and_not_to_build`

A tenet. It says: Resource buffers are sized once at build, as a singleton for the scheduler's lifetime. Column buffers are sized by the record count, which varies per run, so columns are not allocated at build: they belong to the run loop and plan phase, where the per-frame record count is known, and there is no standalone round for columns at build.

Finding: This row has columns sized per run and not allocated at build, while the later sketch has columns reserved at build to capacity with a stable pointer; the row `columns_reserve_at_build_to_capacity_with_a_stable_pointer_and_the_per_frame_record_count_is_a_metadata_field` states the other side, and nothing of his settles it.

Passages: `mock/research/sketches/202605290101_ctx-projection-selector.md:57-59`, `mock/research/sketches/202606041631_column-source-dual-selector/FINDINGS.md:36-36`.

#### `whether_column_buffers_are_allocated_at_build_to_capacity_or_per_run_once_the_record_count_is_known`

A question, open. It asks: Column buffers are sized by a record count that can vary per run. Are they reserved at build to capacity, or allocated by the run loop once the per-frame record count is known?

Its options on record:

1. Columns are NOT allocated at build(); they belong to the run-loop / plan phase (C/E) where the per-frame record_count is known
2. columns reserve at build() to capacity, stable pointer for the scheduler lifetime; per-frame live record count is a metadata field, no per-frame allocation

Finding: The two agent sketches answer it opposite ways; A1 note 4 sets capacity equal to record count at build only for accumulator appends, and no passage or capture of his settles it for columns.

Passages: `mock/research/sketches/202605290101_ctx-projection-selector.md:57-59`, `mock/research/sketches/202606041631_column-source-dual-selector/FINDINGS.md:36-36`.

#### `columns_reserve_at_build_to_capacity_with_a_stable_pointer_and_the_per_frame_record_count_is_a_metadata_field`

A tenet. It says: Columns reserve at build to capacity, with a stable pointer for the scheduler's lifetime, and the per-frame live record count is a metadata field, with no per-frame allocation. The text records this as a separate decision on reservation timing.

Finding: This row reserves columns at build to capacity, while the earlier sketch has columns sized per run and not allocated at build; the text calls it a separate decision, no word of his settles it and A1 speaks only of accumulator capacity.

Passages: `mock/research/sketches/202606041631_column-source-dual-selector/FINDINGS.md:36-36`, `mock/research/sketches/202605290101_ctx-projection-selector.md:57-59`.


### The two copies of the standalone spec

The copy of the standalone spec under `research/` says that where it and an older document disagree it is the design. The copy under `research/pre_seed/` carries a corrected header that calls it a reading aid with no authority of its own. Amendment A2-1 says the first claim exceeds the commission, and A2-1 is a ruling he delegated to a trace of the record, so the rows written from A2-1 are ratified by default as ruled and these 2 rows are here for his own word. The options were written for this file:

1. The delegated trace stands as his ruling: the corrected copy is the reading aid, and the row from the uncorrected copy is demoted.
2. He rules on the standalone spec himself, and the rows wait for that.

#### `the_uncorrected_copy_of_the_standalone_spec_says_it_is_the_design_where_it_and_an_older_document_disagree`

A tenet. It says: The copy of the standalone spec that stands directly under the research directory says of itself that it supersedes, for the purpose of what the design is, the founding consolidation spec, the chain of design-round amendments and the reconciliation addendum, and that where it and any older document disagree, it is the design.

Finding: The copy says that where it and an older document disagree it is the design; A2-1 (ruled by delegated trace) says that claim exceeds its commission and the standalone is a reading aid whose disagreements are transcription defects.

Passages: `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:15-25`, `mock/research/202606111800_canonical-spec-standalone.md:18-22`.

#### `which_copy_of_the_standalone_spec_stands`

A question, open. It asks: Two copies of the standalone spec stand in the tree. The one under the pre_seed directory carries the header addendum A2 corrected: a commissioned reading aid with no independent design authority. The one directly under the research directory still says that where it and any older document disagree, it is the design. Which copy stands?

Its options on record:

1. The corrected copy under pre_seed, as addendum A2 rules.
2. The copy directly under the research directory, with its own header.

Finding: The pre_seed copy says it is a commissioned reading aid whose disagreement with the spec or an amendment is its own transcription defect, while the copy under research says that where it and any older document disagree it is the design. Nothing of his settles which stands, since the A2-1 correction was delegated to a provenance trace.

Passages: `mock/research/pre_seed/202606111800_canonical-spec-standalone.md:18-26`, `mock/research/202606111800_canonical-spec-standalone.md:18-22`.


## Part three: work whose own source says it waits on him

### The swap specification, S1 to S7

The specification of what replacing a resource value does is a draft whose status line says it waits on his ratification, and the record says he held that ratification until it was implemented and benched. The governance chapter of the seed says it is implemented and benched and still waits. These 25 rows are its clauses and the rows about its state. The options are those of the question row `whether_the_maintainer_ratifies_s1_through_s7_of_the_swap_spec_as_the_swap_semantics_canon`, listed below with the rest.

#### `a_replaceable_swap_installs_the_whole_value_as_one_blob_write_through_the_drain_s_selector_and_the_member_by_member_wording_is_refuted`

A tenet. It says: The engine design states a swap of a replaceable value as installing the whole value in one blob write through the same selector witness that the drain and the projection read use, plus a store-dirty mark, with no other pointer derivation on the swap path. The member-by-member wording was grounded on the storage layout from before the bench and is refuted under the bench-decided blob layout. The earlier revert of the one-line install was a mis-citation, and the blob write is the specified install and not the excluded one. Collection-bearing values gain per-record element writes only when the collection wiring lands, and until then the collection halves of a swapped value are out of scope, tracked by the catalogued test.

Finding: The topic says he held ratification of the S1 to S7 swap spec until it was implemented and benched, and the seed manifest still lists it as awaiting his ratification; A2-5 only commissions the spec under the blob layout.

Passages: `mock/design_rounds/202607200900/202607200500_topic.replaceable-swap-semantics.md:5-5`, `mock/research/seed/MANIFEST.md:229-232`.

#### `as_of_2026_11_12_replace_resource_and_replace_value_discard_the_value_and_the_suite_round_reports_the_swap_install_as_not_reached`

A tenet. It says: As of 2026-11-12, the suite round records that the replace-resource and replace-value methods discard their argument, that the plan-dirty flag has no setter anywhere in the source, and that deleting the two methods would remove a documented API. Installing the value here would redo what an earlier ruling of op's reverted. Neither installing nor deleting is a change a test-suite round can make alone, so the swap install is reported as not reached, with this reason and a ticket.

Finding: The row reasons that installing would redo what op's "revert it now" ruling reverted, but A2-5 (ruled amendment) says the revert round was grounded on a mis-citation, the bench-decided layout is the one-record blob, and the install spec S1 awaits his ratification; rows a_replaceable_swap_installs_the_whole_value_as_one_blob_write_through_the_drain_s_selector_and_the_member_by_member_wording_is_refuted and as_of_round_202607193110_a_replaceable_swap_is_a_member_by_member_copy_into_column_slots_and_not_a_blob_write hold the two sides.

Passages: `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:75-95`, `mock/design_rounds/202607200900/202607200500_topic.replaceable-swap-semantics.md:5-10`, `mock/design_rounds/202607193110/202607193100_topic.revert-the-blob-install.md:1-8`.

#### `whether_a_swap_of_a_replaceable_value_installs_the_whole_value_in_one_blob_write_or_is_left_to_a_decomposed_member_by_member_drain`

A question, open. It asks: Whether replacing a replaceable value installs the whole value in one blob write, or whether the install was removed on op's ruling and left to a decomposed member-by-member drain.

Its options on record:

1. the install was removed from replace_value on op's ruling, because an in-place write of the whole value contradicts the member-by-member swap, and left to the decomposed drain (#654); installing here would redo what that ruling reverted
2. replace_value performs the whole-blob write through the same selector witness the drain and the projection read use, plus mark_dirty, with no other pointer derivation; the member-by-member layout is bench-refuted

Finding: Option 1 (install removed on his ruling, member-by-member) is the layout A2-5 calls bench-refuted and mis-cited, option 2 (whole-blob write) is the draft S1 whose ratification "op held" until it is implemented and benched; nothing of his settles it yet.

Passages: `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:75-95`, `mock/design_rounds/202607200900/202607200500_topic.replaceable-swap-semantics.md:5-10`.

#### `replace_value_is_the_supported_every_frame_swap_and_costs_one_blob_write_then_the_store_s_dirty_cone`

A tenet. It says: Replacing a value costs one blob write plus, in the next frame, the dirty cone of the store: the cost is proportional to the size of the value and then to the cone. It is the supported every-frame path.

Finding: The swap specification is a draft pending his ratification, which the record shows he has not given; the whole clause set including this cost claim waits on him.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `replace_resource_swaps_must_be_rare_relative_to_frame_rate_and_an_every_frame_plan_affecting_swap_is_misuse`

A tenet. It says: Replacing a resource costs one blob write plus, in the next frame, the entire leading plan band. Such swaps must be rare relative to the frame rate, and an every-frame plan-affecting swap is misuse regardless of correctness.

Finding: Clause of the swap specification, a draft pending his ratification; no passage of his covers the rarity rule.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `the_sound_swap_path_and_the_fast_swap_path_coincide_because_there_is_one_candidate_write_shape`

A tenet. It says: The sound path and the fast path coincide for swaps by structure: once the storage addendum killed per-member decomposition, there was exactly one candidate write shape.

Finding: Statement of the draft swap specification pending his ratification; it leans on the bench-decided storage addendum but the conclusion is the draft's own.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `the_catalogued_red_swap_test_maps_to_the_replace_value_clause_and_only_its_ignore_reason_is_refuted`

A tenet. It says: The catalogued red test of the resource bindings maps to the replace-value clause. Its assertions are already the spec's and only its ignore reason is refuted. The implementing round rewrites the reason to cite the swap spec, and removes the ignore when the install lands.

Finding: Statement of the draft swap specification pending his ratification; the refuted ignore reason is traced in A2-5 but the mapping to clause S1 is the draft's.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `the_swap_spec_names_seven_new_tests_including_double_swap_zst_swap_unread_swap_and_a_compile_fail_double_impl`

A tenet. It says: The swap spec names new tests: byte-identity of the blob address across a swap; double swap before a run with the last value winning; a swap of a zero-sized type, with dirty set, no byte write and no undefined behaviour; a swap of an unread resource, with dirty set and cleared and no unit re-running; replace-resource causing plan-band entry on a non-first frame, catalogued red until the wiring; a collection swap asserting base-pointer stability and length update, catalogued red on #344; and a compile-fail test that a double impl is rejected with E0751.

Finding: Test list of the draft swap specification pending his ratification.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `replacing_a_plan_affecting_resource_does_the_replace_value_write_and_sets_the_plan_dirty_bit_the_next_run_consumes`

A tenet. It says: Replacing a plan-affecting resource performs the identical write and store-dirty mark as replacing a value, and additionally sets the plan-dirty bit for the type's plan-affecting identity in the scheduler's plan-dirty bit array. The very next run consumes that array: the band gate is the first frame or any plan-dirty bit, a plan-dirty frame enters the leading plan band at phase 0, and the consumed bits clear.

Finding: Clause S2 of the draft swap specification pending his ratification; A2-3 (lines 55-62) rules only that PlanAffecting is an open marker, not the plan-dirty bit array or band gate mechanism.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `the_replace_resource_signature_rides_the_open_plan_affecting_marker_and_the_locate_target_unifies_with_the_value_swap`

A tenet. It says: The signature obligation of the replace-resource clause rides #696: the plan-affecting marker becomes open on resource value types, with the builder-input supertrait and the seal drop, per addendum A2-3, and the locate target unifies with S1 to a locate bound on the resource store. The shipped locate shape is the form from before A2-3, and the implementing round corrects it.

Finding: The open marker on resource value types is his ruling (A2-3), but the builder-input supertrait detail and the unified locate bound are in the draft swap specification pending his ratification.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`, `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:55-62`.

#### `a_swap_never_changes_the_base_pointer_of_a_seq_or_map_member_because_a_foreign_base_pointer_breaks_provenance`

A tenet. It says: A swap never changes the base pointer of the pointer and length view of a sequence or map member. Columns are sized at plan time and never resized, and a foreign base pointer breaks provenance.

Finding: Normative clause of the draft swap specification pending his ratification.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `a_swap_never_writes_past_the_member_s_cap_bound_because_the_new_value_shares_the_old_value_s_exact_type`

A tenet. It says: A swap never writes past the capacity bound N of the member, because the new value shares the exact sequence type of the old value, so the bound holds by construction.

Finding: Normative clause of the draft swap specification pending his ratification.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `swap_writes_element_bytes_into_the_member_s_existing_collection_column_region_and_then_updates_the_length`

A tenet. It says: The element bytes of a swap are written into the member's existing collection-column region, up to the new element count, and then the length updates. The blob carries only the pointer and length view.

Finding: Normative clause S3 of the draft swap specification pending his ratification.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `until_the_344_write_side_lands_a_collection_bearing_swap_installs_only_the_scalar_members_and_the_ptr_len_view`

A tenet. It says: The concrete accessor that writes the element bytes is contingent on the write-side wiring of #344. Until that lands, a collection-bearing swap installs the scalar members and the pointer and length view only, and the element-write half is unimplementable and not underspecified.

Finding: Contingency clause of the draft swap specification pending his ratification.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `the_swap_specification_is_a_draft_that_becomes_the_ruling_source_for_the_swap_rows_once_the_maintainer_ratifies_it`

A tenet. It says: The Replaceable swap specification is a draft pending the maintainer's ratification, as part of the seed pre-freeze resolution batch. On ratification it is the ruling source for the registry items of the swap rulings and for the replacement section of the seed storage chapter. It was commissioned by the citation-correction section of addendum A2, which found swap semantics unspecified and the prior grounding a mis-citation.

Finding: The row records that the swap spec waits on his ratification, and governance still says it awaits it; the commissioning part is his direction in A2-5 lines 88-94.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`, `mock/research/seed/governance.md:96-106` (as it stood at c5f337a, before the trunk rewrote the passage).

#### `the_swap_d_bench_is_commissioned_with_the_trigger_when_the_344_write_side_lands`

A tenet. It says: A bench named Swap-D, a cold write then a stream, is commissioned with the trigger that #344's write side lands. It decides whether the write-resource-collection-sizes morsel term absorbs the write-then-read traffic of a swap or a separate budget line is owed. There is no accessor-shape arm, since there is one write per swap between frames and the stakes do not justify one.

Finding: Clause S7 of the draft swap specification pending his ratification; no passage of his commissions the bench.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `collection_access_read_and_write_is_the_ptr_len_view_over_the_member_s_collection_column_region_behind_the_erased_descriptor`

A tenet. It says: The swap spec answers the open item of the founding spec on the resource collection accessor shape: collection access, on the read side and the write side alike, is the pointer and length view over the member's collection-column region behind the erased descriptor. The concrete consumer-facing API lands with #344, shaped by this clause.

Finding: Draft swap specification pending his ratification; the read-side view follows the bench-decided addendum but the write-side accessor shape is the draft's.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `a_plan_affecting_type_can_never_take_the_replace_value_path_because_of_a_negative_impl_wall`

A tenet. It says: The api crate ships a negative implementation that no plan-affecting type is replaceable, behind the negative-impls feature gate, which is in the watch tier and allowed. The plan-affecting trait is dominant: a plan-affecting type can never take the cheap replace-value path, structurally, with the compiler rejecting the consumer's impl at the consumer's impl site, across crates. The evidence is the exclusivity sketch, both arms.

Finding: Clause S4 of the draft swap specification pending his ratification; A2-3 mirrors Replaceable as an opt-in marker but says nothing of the negative impl.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `the_supertrait_alternatives_to_the_negative_impl_fail_and_a_lint_is_advisory_where_a_structural_wall_exists`

A tenet. It says: The supertrait alternatives to the negative-impl wall fail on the merits. Making plan-affecting a subtrait of replaceable reopens the silent-skip hole, and making replaceable a subtrait of plan-affecting routes every app-state swap through the plan band. A lint is advisory where a structural wall exists.

Finding: Clause S4 reasoning of the draft swap specification pending his ratification.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `ratifying_the_swap_spec_replaces_the_seed_chapter_s_open_question_wording_and_closes_the_governance_items_and_the_two_issues`

A tenet. It says: The ask is to ratify S1 through S7 as the swap-semantics canon. On ratification the seed storage chapter replaces its open-question wording with the spec, governance item 1 and the A2-3 remainder in item 6 close, and #696 and #697 complete. S3 and the Swap-D bench remain gated on #344 inside the ratified spec by name, and not as open questions.

Finding: This is the ask of the evidence memo to him, and the governance chapter still says it awaits his ratification of S1 through S7; the record shows no answer.

Passages: `mock/research/202607201100_swap-semantics-ratification-evidence.md:77-83`, `mock/research/seed/governance.md:96-106` (as it stood at c5f337a, before the trunk rewrote the passage).

#### `whether_the_maintainer_ratifies_s1_through_s7_of_the_swap_spec_as_the_swap_semantics_canon`

A question, open. It asks: Whether the maintainer ratifies S1 through S7 of the swap spec as the swap-semantics canon, which the evidence memo puts to him as its ask.

Its options on record:

1. ratify S1 through S7, after which the seed storage chapter replaces its open-question wording with the spec and #696 and #697 complete
2. do not ratify

Finding: Open question waiting on his ratification: the memo puts the ask to him and governance says it still awaits it, with no answer of his in the record or the captures.

Passages: `mock/research/202607201100_swap-semantics-ratification-evidence.md:77-83`, `mock/research/seed/governance.md:96-106` (as it stood at c5f337a, before the trunk rewrote the passage).

#### `a_replaceable_swap_installs_the_whole_value_as_one_blob_write_and_marks_the_store_dirty`

A tenet. It says: Swapping a replaceable resource installs the whole value as one blob write and marks the store dirty, and there is no other pointer derivation on the swap path. The whole-blob install is the specified mechanism, since under the bench-decided layout a resource value is one reserved blob record; the earlier rule of copying leaf by leaf rested on a layout the bench refuted. The collection halves of a swapped value are outside the swap until the collection wiring lands.

Finding: The whole-blob install is clause S1 of a specification whose own status line says DRAFT pending op ratification, and the seed's governance records that it awaits his ratification of S1 through S7. The row states as the design what the record says waits on him.

Passages: `mock/research/202607200200_replaceable-swap-semantics-spec.md:3-8`, `mock/research/seed/governance.md:96-104` (as it stood at c5f337a, before the trunk rewrote the passage).

#### `a_swap_of_a_resource_value_is_legal_only_between_frames_and_is_observed_from_the_next_run`

A tenet. It says: The draft swap specification says that replacing a resource value installs the new value as the whole one-record blob and sets the store's dirty bit, without touching the plan-dirty state. The call is legal only between frames, so no swap can race a live-streamed read or a snapshot in flight. The swap is observable starting at the next run and at no earlier point, and of several swaps before a run only the last is ever observed.

Finding: The memo header says the persona's lens is to "draft the spec for op's ruling" and the spec it fed carries the status "DRAFT pending op ratification", so the clause is an expert's draft awaiting him.

Passages: `mock/research/pre_seed/202607193300_swap-semantics-panel/03-giesen-synthesis.md:4`, `mock/research/202607200200_replaceable-swap-semantics-spec.md:4-6`.

#### `the_swap_semantics_item_is_implemented_and_benched_and_awaits_the_ratification_of_s1_through_s7_with_s3_and_swap_d_gated_on_344`

A tenet. It says: The Replaceable swap semantics item is recorded as the spec implemented and benched on 2026-07-19, with the witnessed blob install, the plan-dirty band trigger and the exclusivity shipped and the test suite green. The benches confirm the cost asymmetry: the install is the linear memcpy, a value swap pays its cone, a plan swap pays the band, and neither leaks. The item awaits the maintainer's ratification of S1 through S7, and S3 and Swap-D stay gated on #344 inside the spec.

Finding: The item says in words that it awaits the op ratification of S1 through S7, with S3 and Swap-D staying gated on #344, so the content is implemented work pending his ruling.

Passages: `mock/research/seed/governance.md:96-106` (as it stood at c5f337a, before the trunk rewrote the passage).

#### `what_the_shape_of_consumer_reads_of_seq_and_map_after_the_pipeline_is`

A question, open. It asks: What the shape of the consumer's reads of sequence and map members after the pipeline is. The seed lists the resource collection accessor shape as open, waiting on the storage layout work it depends on.

Its options on record:

1. the item waits on the storage layout work it depends on (the seed's open-question statement)
2. collection access on both the read side and the write side is the ptr+len view over the member's collection-column region behind the erased descriptor, with the API landing with #344 (the swap spec's answer)

Finding: The seed says the accessor shape waits on the storage layout work it depends on, while the swap spec answers it with the ptr and len view over the member's collection-column region, the API landing with #344; that spec is itself a draft pending his ratification, and the other side is the row collection_access_read_and_write_is_the_ptr_len_view_over_the_member_s_collection_column_region_behind_the_erased_descriptor.

Passages: `mock/research/seed/governance.md:128-130`, `mock/research/202607200200_replaceable-swap-semantics-spec.md:68-75`.


### The deviations of the shipped engine that wait on a bless

Amendment A2-4 records that the shipped engine deviates from the canon in six places, that each gets evidence first and then his bless or a rebuild, and that none is blessed by default. These 9 rows are the evidence and the proposed rulings. The two question rows carry the options.

#### `thread_spawn_copies_a_pointer_sized_closure_under_a_compile_time_size_check_and_never_casts_a_function_pointer_or_allocates`

A tenet. It says: Spawning a thread takes a generic once-callable closure and copies a pointer-sized closure by value into the thread-creation argument under a compile-time size and alignment assertion. The closure captures a data pointer, so no function pointer is cast and the question of whether function and data pointers have matching size does not arise. A closure that is too fat is a compile error and never an allocation. This is neither option the old backlog note proposed: boxing a function pointer would allocate, which the engine path forbids, and the static work-queue slot was never built.

Finding: The pointer-size spawn is GATE-2 deviation 4, an agent-call listed in A2-4 as pending his bless-or-rebuild ruling with none blessed by default, and its memo's own heading is "Proposed ruling (op's call)".

Passages: `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:64-73`, `mock/research/202607202000_gate2-deviation-4-memo.md:50-57`.

#### `deviations_one_and_six_are_answered_by_the_whole_plane_arena_route_and_the_proposed_ruling_is_the_relocation_round`

A tenet. It says: The evidence for the GATE-2 deviations 1 and 6, the inline pool frame with its pin and the inline GATE-2 scratch, was delivered on 2026-07-19. A sketch shows the whole-plane arena route mechanically proven, and it dissolves the consumer pin and about 48 KiB of single-core dead weight. The proposed ruling is the relocation round.

Finding: The item opens by saying the six GATE-2 deviations each await an evidence-then-bless ruling, and the relocation round is named as the proposed ruling, so the content is an agent's proposal pending his review.

Passages: `mock/research/seed/governance.md:131-139`.

#### `deviation_five_has_a_confirmed_aliasing_hole_so_the_plane_relocation_is_soundness_required_and_resolves_deviations_one_five_and_six_in_one_round`

A tenet. It says: The GATE-2 deviation 5, raw scheduler aliasing, was audited on 2026-07-19 with a confirmed hole: the between-frames mutable surface aliases the shared scheduler reference held by a parked worker, which is miscompilation-class and independent of timing. The plane relocation is therefore required for soundness, and deviations 1, 5 and 6 resolve in one round. The finding is catalogued as an ignored test.

Finding: The same item states the deviations await the evidence-then-bless ruling; the audit is delivered evidence and the one-round resolution is still the proposal put to him, not a ruling.

Passages: `mock/research/seed/governance.md:131-144`.

#### `deviation_four_pointer_size_spawn_is_sound_as_shipped_and_the_proposed_ruling_blesses_it_with_a_compile_time_size_check`

A tenet. It says: The design memo for the GATE-2 deviation 4, the pointer-size spawn, was delivered: the mechanism is sound as shipped and a contract gap is named. The proposed ruling blesses the mechanism, lifts the size check to compile time, and folds the widening of inline storage into the plane relocation round.

Finding: The deviations await an evidence-then-bless ruling and the passage calls the blessing with the compile-time size check "the proposed ruling", an agent's proposal pending his review.

Passages: `mock/research/seed/governance.md:131-132`, `mock/research/seed/governance.md:148-152`.

#### `the_spectral_role_deviation_evidence_favours_canon_s_role_split_of_greedy_fibers_and_spectral_trunks`

A tenet. It says: The evidence on the spectral role deviation was delivered on 2026-07-19. Canon forms trunks spectrally while the shipped code forms fibers spectrally within wide blocks. Both the cost and the grouping character favour canon's role split: greedy is linear and fiber-grained, and spectral is two to three orders heavier and trunk-grained. The proposed ruling restores the roles of steps 7 and 8 through a plan-chain corrective round and awaits the maintainer's bless.

Finding: The item calls the restoration of the step 7 and 8 roles "the proposed ruling" and says it "Awaits the op bless per evidence-then-bless".

Passages: `mock/research/seed/governance.md:120-127`.

#### `whether_the_maintainer_blesses_the_plane_relocation_round_for_deviations_one_five_and_six_and_the_bless_of_deviation_four`

A question, open. It asks: Whether the maintainer rules on the GATE-2 deviations that the seed lists as awaiting an evidence-then-bless ruling: the relocation round proposed for deviations 1, 5 and 6, and the blessing of the pointer-size spawn of deviation 4 with a compile-time size check.

Its options on record:

1. bless the proposed rulings: the plane relocation round for deviations 1, 5 and 6, and the blessing of deviation 4 with the size check lifted to compile time
2. rebuild the deviations to the canon shape

Finding: A2-4 records that the shipped shapes deviate from the canon shapes and that none is blessed or rebuilt by default, and the seed carries the proposed rulings for deviations 1, 5, 6 and 4; the canon side and the shipped side conflict and nothing of his rules between them.

Passages: `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:64-71`, `mock/research/seed/governance.md:131-152`.

#### `whether_the_maintainer_blesses_the_proposed_restoration_of_the_canon_s_trunk_and_fiber_roles_for_spectral_and_greedy_grouping`

A question, open. It asks: Whether the maintainer blesses the proposed ruling that restores the roles of canon's steps 7 and 8, through a plan-chain corrective round, in place of the shipped code that forms fibers spectrally within wide blocks. The evidence has been delivered and the item awaits his bless under evidence-then-bless.

Its options on record:

1. bless the proposed ruling: canon's role split, spectral forms trunks and greedy forms fibers, is restored via a plan-chain corrective round
2. keep the shipped code, which forms fibers spectrally within wide blocks

Finding: Canon forms trunks spectrally while the shipped code forms fibers spectrally within wide blocks, and A2-4's closing sentence puts that deviation under evidence-then-bless; the evidence is delivered and the item still awaits his bless, so nothing of his settles the two sides.

Passages: `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:72-73`, `mock/research/seed/governance.md:120-127`.

#### `the_engine_holds_no_thread_handle_and_dropping_the_scheduler_stops_the_workers_by_flag_and_wake_and_waits_on_an_exit_counter`

A tenet. It says: Dropping the scheduler sets the shutdown flag, wakes every parked worker and waits until every worker has left its main loop. The engine holds no thread handle to join, because the threads belong to the consumer's executor.

Finding: The ledger tags the exit-counter shutdown instead of a thread join (section 6) as an agent call pending op review, and A2-4 says that entry is blessed by default neither way until benched and presented to him; the row states it as settled.

Passages: `mock/research/202606072100_gate2-canonical-deviation-ledger.md:23-23`, `mock/research/202606072100_gate2-canonical-deviation-ledger.md:65-71`, `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:64-73`.

#### `the_main_thread_drives_each_phase_of_a_parallel_frame_and_is_itself_the_barrier_between_phases`

A tenet. It says: In the first threaded executor, the main thread drives the phases of a frame by publishing each phase and waiting for all workers to finish it, so the main thread is itself the barrier between phases and there is no worker-side sense-reversing barrier. Workers derive the phase from the sequence value.

Finding: The founding spec has phase sync points inside each core's compiled program and no scheduler loop, while this row has the main thread as the barrier; the ledger tags it an agent-call pending his review and the row `the_waist_barrier_moves_worker_side_as_a_sense_reversing_barrier_so_a_frame_is_one_publish_and_await_round_trip` reverses it.

Passages: `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1604-1604`, `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md:1615-1617`, `mock/research/202606072100_gate2-canonical-deviation-ledger.md:49-55`.


### The calls of the runtime round made while he was away

The runtime round of May 2026 says of itself that its topics closed on autonomous calls, drafted overnight to land at his next session, and it flags some of its constants for his attention. These 11 rows are such calls. The options were written for this file:

1. They are blessed as a set and become his.
2. They stay provisional defaults until the sketches and benches the round itself names exist.
3. The flagged constants are walked one by one and the rest are blessed.

#### `a_resource_snapshot_is_a_read_only_typed_view_bounded_to_the_current_phase`

A tenet. It says: A resource snapshot is a read-only typed view captured at a phase boundary. Its lifetime is bounded by the current phase and it cannot be lifted out of it, and its concrete signature waits for the data plane and is to be reconciled with the current builder input contract.

Finding: Decided in Topic 8 of the runtime megaround, whose header says it closed on autonomous calls, and the doc changelist says it was drafted autonomously overnight to land at the next user session. Nothing of his backs the snapshot shape, and the row itself says the signature waits for the data plane.

Passages: `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md:3-12`, `mock/design_rounds/202605101036/202605101036_changelist.doc.lock.md:4-4`.

#### `performance_and_efficiency_cores_are_classified_per_operating_system_in_the_first_version_with_wake_strategy_per_core_class`

A tenet. It says: Classification of performance and efficiency cores is done for each operating system from the first version, and each core class has its own wake strategy with different default spin counts.

Finding: The status section lists the WakeStrategy default ratio of 128 and 32 among five items flagged from the autonomous topic locks for his attention, and the topic itself leaves the defaults to a sketch or bench. The per-OS detection is in founding domain 20 but the per-class defaults are not his.

Passages: `mock/DESIGN.md.tmpl:146-156`, `mock/design_rounds/202605101036/202605101036_topic.thread-pool.md:104-104`.

#### `micro_morsels_end_every_64_records_and_workers_sync_by_a_spin_budget_within_a_drift_of_32_records_with_no_futex_inside_the_loop`

A tenet. It says: A micro-morsel boundary falls every 64 records. Workers synchronise by a spin-loop budget with a tolerance of 32 records of drift, no operating-system wait happens inside the morsel loop, and the phase barrier handles parking across phases.

Finding: The morsel-loop topic says 64 and 32 are concrete defaults it picked autonomously and not bench-validated, and the status section flags them for his attention. The founding spec also uses micro-morsel for inner tiling when peak live exceeds L1, not for a sync boundary.

Passages: `mock/design_rounds/202605101036/202605101036_topic.morsel-loop.md:46-48`, `mock/DESIGN.md.tmpl:148-154`.

#### `plan_recompute_is_sequential_and_single_threaded_in_the_first_version_and_parallelising_it_waits_for_pipelines_of_256_units_or_more`

A tenet. It says: In the first version the plan recompute runs sequentially on one thread. Making it parallel is deferred until consumers ship pipelines of 256 units or more.

Finding: Topic 9 axis F is among the autonomous calls of the overnight 2026-05-11 stretch, with a revisit note for large pipelines; no word of his on it.

Passages: `mock/design_rounds/202605101036/202605101036_topic.plan-caching.md:3-4`, `mock/design_rounds/202605101036/202605101036_topic.plan-caching.md:56-58`.

#### `the_runtime_layer_was_crystallised_in_design_round_202605101036_and_its_flagged_constants_are_revisitable_with_low_impact`

A tenet. It says: The runtime layer was crystallised in a design round, closed pending sketches on 2026-05-11, across eleven topics, and implementation lands bottom-up. Five items that the autonomous topic locks flagged for the maintainer's attention are constants or naming choices that can be revisited with low impact, and none of them blocks downstream implementation.

Finding: The status says the round closed pending sketches and the flagged constants come from autonomous topic locks for his attention, and the doc changelist says it was drafted autonomously overnight to land at the next user session.

Passages: `mock/DESIGN.md.tmpl:146-156`, `mock/design_rounds/202605101036/202605101036_changelist.doc.lock.md:3-4`.

#### `persistence_is_wired_in_by_the_consumer_through_a_provider_and_the_engine_never_depends_on_the_persistence_crate_s_types`

A tenet. It says: Persistence is supplied by the consumer through a persistence provider trait and a provider tuple. The engine never depends on the persistence crate's types directly, and an end-of-pass unit drains the accumulator columns into persistence through the provider's commit.

Finding: The PersistenceProvider route and the unit that drains accumulator columns are Topic 8 axis C, closed on autonomous calls overnight. The founding spec backs only the evict and inject route with on-disk state the consumer's.

Passages: `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md:3-4`, `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md:45-53`.

#### `a_resource_is_plan_affecting_only_when_the_consumer_opts_it_in_and_the_number_of_them_is_capped`

A tenet. It says: A resource is not plan-affecting by default. A consumer opts a resource in when the shape of its value changes the plan, and the number of plan-affecting resources is capped by a default that the consumer can override.

Finding: The cap of 16 is a call the topic says it made autonomously and can be revisited, flagged for his attention; the doc changelist's defaults table gives 256 for the same constant.

Passages: `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md:64-70`, `mock/DESIGN.md.tmpl:148-154`.

#### `a_resource_snapshot_is_a_read_only_view_captured_at_a_phase_barrier_and_it_cannot_outlive_the_phase`

A tenet. It says: A resource snapshot is a read-only typed view of a resource captured at a named phase boundary, after the wake and before the next phase's morsels begin. While it lives the storage beneath it is not being written, its content is what the resource held when it was captured, and it cannot escape across a phase barrier.

Finding: The topic's own phase line calls its locked decisions autonomous trivial calls plus one user-attention flag, so the snapshot semantics are the agent's call with none of his words on it.

Passages: `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md:4-4`.

#### `the_adapt_unit_reads_its_metrics_through_a_snapshot_at_the_end_of_the_schedule_and_mutates_nothing_through_it`

A tenet. It says: At the end of the schedule the adapt unit observes the adaptation metrics through a resource snapshot, to derive the predicted waits it writes for the next frame. Through the snapshot it only reads.

Finding: Source lines 27 and 78 sit in topic 8, whose phase line calls its decisions autonomous trivial calls, so this is the agent's own call with no word of his.

Passages: `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md:4-4`.

#### `the_engine_reaches_persistence_only_through_a_resource_the_consumer_wires_and_never_names_the_persistence_crate_s_types`

A tenet. It says: Persistence is the cold-store sibling of the runtime data plane, and the integration is narrow on purpose: the engine never depends on the persistence crate's types directly, and the consumer wires persistence in as a pluggable resource. Reads on the hot side copy from cold into a column at the plan stage, so the runtime touches the column and not persistence; writes on the cold side are a unit at the end of the pass draining accumulator columns into persistence.

Finding: The persistence wiring is in topic 8, which the phase line calls autonomous trivial calls; the founding spec instead has evict and inject APIs with the consumer owning persistence (lines 2168-2175 and 2406-2412), a different mechanism.

Passages: `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md:4-4`.

#### `the_plan_dirty_bit_array_is_keyed_by_the_plan_affecting_id_under_a_default_cap_of_sixteen_and_exceeding_it_fails_at_compile_time`

A tenet. It says: The plan-dirty bit array holds one bit for each plan-affecting input type of a pipeline, keyed by an identifier for each, under a default cap of sixteen. Registering more than the cap fails at compile time with a diagnostic pointing at the cap, and the cap is a const generic on the scheduler that a consumer overrides.

Finding: The topic heading calls the cap of sixteen an autonomous user-attention call that can be revisited, and the round's doc changelist later gives the dirty-mask cap a default of 256.

Passages: `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md:4-4`, `mock/design_rounds/202605101036/202605101036_topic.resource-resolution.md:64-70`.


### The erased resource boundary

The hybrid storage model is his pick. That every resource is erased, and not only those that opt in, is called a call derived from his criteria in the same passage. 2 rows.

#### `whether_the_erased_resource_boundary_is_per_resource_or_global`

A question, answered. It asks: Is the erased static-shape boundary per-resource, with only the resources that cross a plugin boundary going erased, or global, with every resource using the erased addressing?

Its options on record:

1. Per-resource: only resources that cross a plugin boundary go erased and the rest stay native blobs
2. Global: every resource uses the erased static-shape addressing

His answer on record: Global: the erased static-shape addressing is the uniform resource value model and every resource is plugin-capable at parity cost. The call is derived from op's stated criteria and not from an explicit op instruction on the sub-question, and it is cheap to narrow to a per-resource opt-in if op prefers on review.

Finding: The answer global is called a call derived from his stated criteria and not an explicit op instruction on the sub-question, and cheap to narrow to per-resource if he prefers on review, so it waits on him.

Passages: `mock/design_rounds/202607161000/202606210600_topic.hybrid-decision.md:42-61`.

#### `a_resource_value_is_addressed_through_an_erased_static_shape_so_it_can_cross_a_plugin_boundary`

A tenet. It says: The maintainer picked the hybrid: the value bytes of a resource stay one contiguous record, and they are addressed through an erased static-shape descriptor and not through a monomorphised concrete-type pointer, so that a resource value can cross a plugin boundary and interoperate with builtin resources near-natively, as in loimu's model. The bench measured this addressing at parity with the native monomorphised one. He chose it citing future-proofing and parity. Every resource uses the erased addressing, so any resource is plugin-capable without a design change.

Finding: The hybrid itself is his pick (hybrid-decision lines 12-19), but the passage says global-capable, every resource erased, is a call derived from his criteria and not an explicit instruction of his, cheap to narrow to per-resource opt-in if he prefers on review.

Passages: `mock/design_rounds/202607161000/202606210600_topic.hybrid-decision.md:58-61`, `mock/research/202606210600_resource-storage-model-canonical-addendum.md:53-56`.


### The abandoned round on phase-sectioned dispatch

The round is marked abandoned, and its own course-correction note says it is not to be built as written and waits on his decision. 2 rows. The options were written for this file:

1. The rows are demoted with the round.
2. He decides the round's fate first, and the rows wait.

#### `phase_sectioned_dispatch_with_a_waist_between_phases_is_established_before_parallelising`

A tenet. It says: Single-core dispatch executes the registered units phase by phase in phase order with a waist barrier between phases, the waist sectioning being established before parallelising. A following round swaps the degenerate barrier for the real one and pins each phase's trunks to cores.

Finding: The round directory is marked abandoned and the course-correction note says round 2b is not to be implemented as written and awaits his decision, because it broke the shipped morsel-outer test and the spec's phase meaning.

Passages: `mock/design_rounds/202606071300-abandoned/.meta:1-4`, `mock/research/202606071400_gate2-phase-axis-course-correction.md:1-7`.

#### `single_core_run_executes_phase_by_phase_under_runtime_phase_masks_with_no_per_trunk_enumeration`

A tenet. It says: Single-core output equivalence does not need per-phase-and-trunk monomorphisation, so the run is a runtime loop over the phases, each phase walking the shipped mask-gated walk under a runtime unit mask for that phase with a waist barrier between phases. Const enumeration of every phase and trunk is not done at single-core, since it would hit a count-bound monomorphisation explosion; the const-gated per-trunk programs remain the building block for the per-core case.

Finding: Same abandoned round 2b: the note supersedes its plan, says it is not to be implemented as written and is pending his decision.

Passages: `mock/design_rounds/202606071300-abandoned/.meta:1-4`, `mock/research/202606071400_gate2-phase-axis-course-correction.md:1-7`.


### Forks a round handed back to him

Each of these 5 questions was handed back by the round that met it, in words, and no answer of his was found.

#### `whether_a_shape_should_be_trusted_with_any_id_space_arithmetic_or_the_extension_point_narrows_to_widths_and_an_origin_type`

A question, open. It asks: Whether a shape should be trusted with any id-space arithmetic, or whether the extension point should narrow to widths and an origin type with everything numeric derived.

Its options on record:

1. a shape keeps some id-space arithmetic, as the open version has it, which five rounds of evidence say keeps reopening the defect it was built alongside
2. the extension point narrows to widths and an origin type, with everything numeric derived

Finding: The topic says in words that this design fork "goes to op rather than being decided here", and no capture or later passage of his answers it.

Passages: `mock/design_rounds/202608101900/202608101900_topic.the-sixth-route-and-what-a-shape-may-still-decide.md:55-62`.

#### `whether_hilavitkutin_grows_its_own_numeral_mask_graph_sparse_spectral_combinatorial_and_hash_code_or_drops_the_design_steps_those_arvo_crates_serve`

A question, open. It asks: Whether moving onto arvo's newer dev revision means hilavitkutin grows its own numeral, mask, graph, sparse, spectral, combinatorial and hash code, or drops the design steps those crates serve.

Its options on record:

1. hilavitkutin grows its own numeral, mask, graph, sparse, spectral, combinatorial and hash code, which puts arvo's obligations inside the consumer that stated them
2. hilavitkutin drops the design steps those crates serve, which is a redesign of the plan chain

Finding: The topic says neither option is its to decide and hands the call back, and the changelist says it "stays handed back"; I found no answer of his in the captures.

Passages: `mock/design_rounds/202609181402/202609181401_topic.hilavitkutin-on-arvo-dev.md:32-37`, `mock/design_rounds/202609181402/202609181402_changelist.doc.lock.md:7-7`.

#### `what_the_engine_does_when_the_executor_reports_no_workers`

A question, open. It asks: What the engine does when an executor reports no workers: refuse, run inline on the caller, or hand out one closure.

Its options on record:

1. the engine refuses: run_parallel returns an error before publishing a frame, and the frame done-wait is never reached
2. the engine runs inline on the caller: the calling thread runs core 0's worker body itself, and the done-wait counts one arrival, its own
3. the engine hands out one closure: the done-wait expects one arrival from the executor, and where the executor reported zero because it has no thread to run anything on, the closure never runs and the done-wait never returns

Finding: The topic says the one-closure behaviour "settles a question nobody ratified" and "marks the question provisional, waiting on op", and the changelist says "The choice waits on op"; no answer of his found.

Passages: `mock/design_rounds/202609191001/202609191000_topic.second-review-on-arvo-dev.md:23-31`, `mock/design_rounds/202609191001/202609191001_changelist.doc.lock.md:19-19`.

#### `compile_time_detection_of_a_duplicate_marker_registration_cannot_be_encoded_under_the_workspace_constraints_and_the_first_wins_fallback_promise_is_unsubstantiated`

A tenet. It says: A compile-time error on registering a marker twice cannot be delivered with the sketched non-membership encoding under the workspace's constraints, and the negative-impl path is dead. The earlier promise of a compile-time error preferred with a runtime first-wins fallback is unsubstantiated, and the fallback's semantics were left vague. The proposal must commit to one of four directions, which is a decision point for the user.

Finding: The sketch ends "Decision point for the user" on which of four directions to commit to, so the call this row carries waits on him; the compile result itself (E0751) is the agent's and no answer of his on it was found in the record.

Passages: `mock/research/sketches/registrable-not-in-202605051200/FINDINGS.md:54-66`.

#### `what_the_builder_does_about_a_marker_registered_twice_given_that_compile_time_detection_cannot_be_encoded`

A question, open. It asks: Compile-time detection of a duplicate marker registration cannot be encoded. The proposal must commit to one of four directions for a marker registered twice. Which does the design take?

Its options on record:

1. Silent shadowing, Bevy-style: two entries of one resource are tolerated in the stores cons-list, membership proofs match the first occurrence, the consumer is responsible for not creating diamonds, documented loudly with a diagnostic hint
2. Runtime panic at the engine resource call: the engine maintains a runtime registry of registered markers without using TypeId, possibly by type-name string equality or a sealed enum-of-markers pattern, and panics on a duplicate at one runtime check per registration
3. An explicit replace operator on the builder with silent shadowing as the default: a replace operator gated on the stores already containing the marker, matching Bevy's init_resource versus insert_resource distinction
4. Drop diamond detection entirely for v0 and track it as a future follow-up when negative_impls matures or a new approach surfaces

Finding: The four directions are listed as recommendations for round 4 and closed with "Decision point for the user"; no capture, amendment or row of his in the record picks one, so it waits on him.

Passages: `mock/research/sketches/registrable-not-in-202605051200/FINDINGS.md:54-66`.


## Part four: an agent's row that a later agent round replaced

These 13 rows were written from one agent round, and a later agent round says in words that it replaces or retracts them. Neither side is his, so the later row is on a stamp and the earlier one is here. The options were written for this file and are the same for each row:

1. The earlier row is demoted and the later row keeps its stamp.
2. Both wait until he has read the round.

#### `anomalies_are_virtual_marker_types_so_that_observers_compose_through_access_set_union`

A tenet. It says: Each kind of anomaly is its own virtual marker type, so that observers of anomalies compose through the union of access sets.

Finding: The row says each anomaly is its own virtual marker type, while the same topic's audit M7 retracts per-anomaly virtuals for a single AnomalyFired flag with detail on the metrics resources, which row the_adapt_unit_fires_one_anomaly_flag_and_the_metrics_resources_carry_which_anomaly_it_was states.

Passages: `mock/design_rounds/202605101036/202605101036_topic.adapt-subsystem.md:284-284`, `mock/DESIGN.md.tmpl:64-64`.

#### `the_identity_handle_is_four_bytes_with_a_four_bit_kind_over_a_twenty_eight_bit_id_and_handles_of_different_kind_are_never_equal`

A tenet. It says: The identity handle of the generic interned-identity core is a four-byte handle with a four-bit kind over a twenty-eight-bit id. This reinterprets the four high bits of the current string handle layout, one origin bit plus three reserved, as one kind field, so every bit pattern a string handle holds today is a valid handle. Equality is integer equality over the whole thirty-two bits, so two handles with different kinds are never equal whatever their ids, and that disjointness by kind is the core property the whole design rests on.

Finding: The same round's doc changelist says in words that it refines the topic's "4-bit kind" into a 3-bit tag plus a 1-bit domain-private flag; row the_identity_handle_carries_a_three_bit_domain_tag_and_a_one_bit_domain_private_flag_so_the_string_domain_is_one_kind states the replacing layout.

Passages: `mock/design_rounds/202607241915/202607241915_changelist.doc.lock.md:20-20`.

#### `the_identity_core_owns_the_kind_assignment_so_two_domains_cannot_claim_one_kind_and_the_string_kinds_keep_the_values_the_current_layout_produces`

A tenet. It says: The identity core owns the convention that assigns kinds and documents it, so two domains cannot claim one kind. The four-bit field gives sixteen kinds. Two are reserved for the string domains at exactly the values the current string layout already produces, const strings at zero and runtime strings at the high-bit pattern, which is mandatory for byte-compatibility. The rest are assignable to other domains, and the first consumer generative domain is recorded in the table. The string-domain kind constants live in the string crate, since they are string-domain facts, and the identity core documents the reservation.

Finding: The topic's 16-kind table with const strings at 0b0000 and runtime at 0b1000 is replaced by the changelist's 3-bit table where const and runtime strings differ by the flag bit, per row the_kind_assignment_table_is_a_three_bit_tag_with_the_string_domain_told_apart_by_the_flag_bit_and_not_by_tag.

Passages: `mock/design_rounds/202607241915/202607241915_changelist.doc.lock.md:21-21`.

#### `the_width_a_shape_declares_is_counted_from_a_mask_the_layout_generates_by_a_function_no_implementation_can_override`

A tenet. It says: A width the shape trait declares is one something constrains: a layout carries its own generated mask, and the width is counted from that mask by a function no implementation can override. Nobody writes the number, so nobody can write it wrong.

Finding: The later doc changelist says in words that the mask-derived width defence "is false and the review proved it by construction" and replaces it with width from the associated types, per row the_width_a_shape_declares_comes_from_the_associated_types_its_accessors_are_typed_against_and_not_from_a_forwarded_mask.

Passages: `mock/design_rounds/202608091100/202608091100_changelist.doc.lock.md:16-30`.

#### `the_width_a_shape_declares_comes_from_the_associated_types_its_accessors_are_typed_against_and_not_from_a_forwarded_mask`

A tenet. It says: The width a shape declares comes from the associated types the layout's accessors are typed against, read through a width trait that no layout can supply for itself. A layout cannot lie about those, because the accessor must return what the bitfield hands it, and a wrong associated type is a type error rather than a silent disagreement. The forwarded mask constants have no reader and go.

Finding: The next round's doc changelist says the claim that a layout cannot lie about the associated type holds only for forwarding accessors and that an accessor may convert, so the type alone is not sufficient; row the_declared_width_reaches_a_value_the_code_uses_through_the_associated_type_and_the_round_trip_together_and_neither_alone_is_sufficient says so.

Passages: `mock/design_rounds/202608091600/202608091600_changelist.doc.lock.md:11-19`.

#### `the_id_width_in_the_shape_trait_signature_is_the_default_shape_s_twenty_eight_bits_and_making_it_follow_the_shape_was_left_open`

A tenet. It says: The id width in the shape trait's signature is the default shape's twenty-eight bits, sitting where a shape cannot override it. Making the type follow the shape is a design question the round does not have the answer to, so it is documented instead of fixed, and two laws contain the hole meanwhile.

Finding: The row says the 28-bit id width sits in the trait signature and was left open, while a later doc changelist moves the id projection and its width to the layout, per row a_shape_supplies_no_base_ceiling_or_counter_projection_and_the_layout_still_supplies_the_projection_so_the_freedom_is_reduced_and_not_eliminated.

Passages: `mock/design_rounds/202608101900/202608101900_changelist.doc.lock.md:7-19`.

#### `the_plan_stage_reserves_cache_budget_from_the_axes_the_consumer_registered_and_not_from_a_worst_case`

A tenet. It says: With no composite adaptation configuration structure, the plan stage reserves its cache budget by walking the axes the consumer actually registered and computing how many are active in a pass from their literal sampling skip values. The reservation follows the consumer's own configuration and is tighter than a worst case where every axis is active.

Finding: This row says the reservation follows the registered axes' cadence; Audit-2 M6 in the same document picks worst-case all-axes-active and retracts exactly this claim (row the_plan_stage_reserves_cache_budget_as_if_every_enabled_axis_writes_every_pass_and_cadence_skipping_lowers_only_cpu_cost).

Passages: `mock/design_rounds/202605101036/202605101036_topic.adapt-subsystem.md:277-283`, `mock/design_rounds/202605101036/202605101036_topic.adapt-subsystem.md:288-288`.

#### `the_per_fiber_unit_and_column_caps_are_derived_from_the_outer_caps_by_division_which_cuts_the_independent_const_generics_from_nine_to_seven`

A tenet. It says: The per-fiber unit cap and per-fiber column cap are derived from the outer caps by division rounded up, which brings the independent const generic count of the plan from nine to seven. A hybrid of const-generic outer caps and runtime-checked inner sizes is a deferred alternative, and the first version uses const generics only.

Finding: The plan chain derives the per-fiber caps from the outer caps by division, while Audit-2 m3 locks that wrong for realistic workloads and makes them their own const generics (row the_per_fiber_unit_and_column_caps_are_their_own_const_generics_defaulting_to_the_outer_caps_and_the_division_is_only_a_recommended_heuristic).

Passages: `mock/design_rounds/202605101036/202605101036_topic.adapt-subsystem.md:285-285`, `mock/design_rounds/202605101036/202605101036_topic.plan-algorithm-chain.md:1017-1028`.

#### `the_accumulator_store_set_is_produced_at_the_store_drain_as_a_flat_bitmask_and_a_fiber_is_morsel_local_when_no_unit_in_it_writes_an_accumulator`

A tenet. It says: A fiber is morsel-local exactly when no unit in it writes an accumulator. The set of accumulator stores is produced at the store drain, where the accumulator stores are visible as distinct type-level facts, as a small flat bitmask shaped over the drain's stores, with no allocation and no new trait machinery beyond the existing const-fold pattern.

Finding: The doc changelist produces the accumulator set at the store drain, and the source changelist records that as the wrong store-index space and moves it to a fold over the global stores list (row the_accumulator_store_set_is_a_type_level_fold_over_the_global_stores_list_in_the_same_index_space_as_the_plan_s_access_masks).

Passages: `mock/design_rounds/202606051500/202606051500_changelist.src.lock.md:9-13`.

#### `per_trunk_dispatch_is_a_const_range_walk_that_selects_the_members_of_one_trunk_and_each_trunk_is_its_own_monomorphisation`

A tenet. It says: Per-trunk isolation is a const-gated walk over the flat unit carrier that runs only the members of one trunk, so that each trunk value monomorphises once and dead-code elimination removes every non-member position, leaving a member-only program equivalent to the flat walk restricted to that trunk. The flat carrier stays flat and the partition lives in const data.

Finding: This topic proposes a new `RunTrunkSel` plus `run_one_trunk` walk, and the later topic of the same round says it reinvents shipped infrastructure and supersedes its framing; the row `the_outer_dispatcher_drives_the_shipped_per_trunk_walk_keyed_by_trunk_only_and_phase_order_is_a_runtime_loop` holds the replacement. Agent-written on both sides.

Passages: `mock/design_rounds/202606080400/202606080300_topic.gate2-ge-drive-shipped-rungatedtrunk.md:6-7`, `mock/design_rounds/202606080400/202606080100_topic.gate2-ge-const-gated-trunk-dispatch.md:16-19`.

#### `when_the_grouping_scratch_probe_compiles_a_caps_trait_carries_the_unit_accum_and_plan_dirty_capacities_and_every_hardcoded_cap_site_migrates`

A tenet. It says: If the in-engine probe shows that the grouping scratch compiles as a projected associated-type array under the real compounded obligations, a caps trait is introduced for the unit, accumulator and plan-dirty capacities, threaded the way the plan dimensions are, and every hardcoded cap site migrates onto the projections. If the probe overflows, the grouping caps stay bounded at the documented 256 and 16 ceiling with the tracked lift condition, and the other caps still lift.

Finding: This topic introduces a `Caps` trait when the probe compiles, while the locked changelist of the same round extends `PlanDims` and adds no separate trait; the row `to_lift_the_hardcoded_caps_the_plan_dimensions_trait_gains_an_accums_per_core_and_a_plan_affecting_capacity_and_no_separate_caps_trait_is_added` states the other side.

Passages: `mock/design_rounds/202606191200/202606191200_topic.gate2-caps-lifting.md:35-45`, `mock/design_rounds/202606191200/202606191200_changelist.doc.lock.md:7-15`.

#### `a_drifted_locked_changelist_is_reconciled_by_a_new_round_that_deprecates_it_and_carries_a_corrected_replacement`

A tenet. It says: A locked changelist whose wording drifted from shipped source is reconciled in a new round: a new topic file, a deprecation of the original, a replacement changelist with the changelist block corrected and a comparison to the deprecated changelist, a lock of the replacement, and a close of the round. The list, called Option A, stays canonical.

Finding: This row keeps Option A, reconciliation by a new deprecating round, but the sketch of the same day finds that reading mistaken, since the tool cannot deprecate a past round, and the row `a_locked_changelist_stays_immutable_and_its_drift_from_shipped_source_is_documented_by_a_sketch_and_a_new_round_is_for_a_design_change_only` holds the opposite. The source outcome is INCONCLUSIVE and agent-written.

Passages: `mock/research/sketches/202605251400_v2-phase-machine-field-notes.md:72-78`, `mock/research/sketches/202605251500_cl-drift-closure-clarification.md:10-16`, `mock/research/sketches/202605251500_cl-drift-closure-clarification.md:56-63`.

#### `the_arity_twelve_cap_continues_to_govern_unit_read_and_write_declarations_and_not_the_builder_accumulator`

A tenet. It says: The access set needs no extension to higher arities: the arity-twelve cap continues to govern the unit read and write declarations, which are flat tuples, and it does not govern the builder's accumulator, which is a cons-list.

Finding: This row says the arity-twelve cap keeps governing unit read and write declarations; the later update of the same findings file (row a_unit_s_read_and_write_declarations_are_cons_lists_written_through_macros_and_the_satisfaction_proof_is_recursive_with_no_arity_cap) says the cap is removed through read! and write! macros and a recursive proof with no cap.

Passages: `mock/research/sketches/app-builder-typestate-202605010900/FINDINGS.md:166-178`, `mock/research/sketches/app-builder-typestate-202605010900/FINDINGS.md:26-30`.

