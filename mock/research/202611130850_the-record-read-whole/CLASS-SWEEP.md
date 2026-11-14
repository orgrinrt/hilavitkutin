# Report: the class of every tenet against the record

Worktree `/home/user/.worktrees/hv-rows`, branch `work/canon-registry-rows`, seven commits on `73512e85`, last `3c7b2b3c`. `cargo mock` exits 0 before the work and after every commit. Classes below: `ratified` with its kind, `stamp` for pending on a provisional stamp, `op list` for pending with no stamp and listed in `ROWS-FOR-OP.md`. `ROWS-FOR-OP.md` now lists 137 rows (was 131).

## Step 1: the rows the review named

- `all_engine_runtime_state_lives_in_one_columnar_store_and_a_resource_versus_a_column_is_intent_and_not_mechanism`: ratified founding -> stamp. Its lines `mock/PRINCIPLES.md.tmpl:119-126` were written by round commit `18fcb5d` (2026-05-30), not the seed, and `mock/research/202606210600_resource-storage-model-canonical-addendum.md:17-24` refutes its decomposition of every aggregate into scalar columns.
- `hilavitkutin_does_not_use_the_alloc_crate_and_dynamic_storage_goes_through_slab_columns_or_stack_arrays`: stays ratified founding. `what` and `mechanism` lose the std alloc on std tier clause, and a `later:` entry was added for `mock/research/202611130850_the-record-read-whole/OP-ANSWERS-0918.md:20-32` with `there_is_no_std_platform_tier`. The rest is his founding text, `consolidation.md:284-286, 308-310`.
- `execute_returns_nothing_and_a_failing_unit_aborts_because_abort_gives_better_llvm_codegen`: stays ratified founding. `what` and `mechanism` lose the std-tier catch_unwind clause, and its `later:` entry now cites `OP-ANSWERS-0918.md:20-32` instead of the changelist.
- `whether_dispatch_follows_rcm_order_or_waist_phase_order_is_decided_by_a_bench`: ratified ruled -> stamp, `superseded_by` `rcm_wins_against_rival_ordering_theories_at_dram_scale_and_order_is_near_neutral_below_it_so_step_5_stands`. His A1-1 (`mock/research/202606111400_canonical-design-addendum-a1.md:49-57`) held the fork open until a bench landed, and `mock/research/202607200800_a1-1-ordering-theory-bench-expansion.md:82-90` closes it.
- `rcm_wins_against_rival_ordering_theories_at_dram_scale_and_order_is_near_neutral_below_it_so_step_5_stands`: stamp -> ratified ruled, from `202607200800_a1-1-ordering-theory-bench-expansion.md:82-101` (the bench decided it under his A1-1 call that a bench decides, `a1.md:49-57`). Three `related` entries that pointed at the replaced row now point here. The question `which_order_the_dispatch_follows_the_rcm_order_of_units_or_the_waist_phase_order` stays open on its stamp, which says no word of his takes the bench result as the answer. The two passes read this one point differently.
- `an_entry_is_one_record_and_one_operation_at_register_level`: ratified founding -> op list, added to the group "Entry or record as the innermost unit", `ROWS-FOR-OP.md:357`. One extra fact: the seed `680a672` listed entry among the canonical terms, and the style sweep `59b3c18` moved it to the dead terms.

## Step 2: the class, subject by subject

I read 364 ratified rows, every subject, against the OP-ANSWERS files and the later ratified rows on the same point. 6 moved and 1 was edited:

- `the_consolidation_is_the_single_source_of_truth_for_the_current_design`: ratified founding -> stamp. His ruling `mock/research/202606061000_canon-amendment-unified-engine.md:3-3` gives the unified-engine amendment the same standing.
- `four_meta_resources_hold_the_graph_the_plan_the_lane_assignment_and_the_metrics_and_are_restricted_by_a_marker_trait`: ratified founding -> op list, new group "Where the mutable meta state of the scheduler lives" (`consolidation.md:2130-2134` against `a1.md:161-164`).
- `the_scheduler_metrics_are_engine_owned_in_a_meta_block_written_by_the_scheduler_and_read_through_a_meta_access_gated_context_accessor`: ratified ratified-elsewhere -> op list, same group. A1 constraint note 5 is not said to be his ruling, and the founding text says otherwise.
- `the_type_taxonomy_is_trait_newtype_repr_c_enum_or_primitive_and_it_governs_the_form_of_every_type`: ratified founding -> op list, new group "Primitives as a fourth form of type" (`consolidation.md:2205-2206, 2240-2241`).
- `every_type_is_a_trait_a_newtype_or_a_c_layout_enum_and_there_is_no_fourth_bare_primitive_category`: ratified founding -> op list, same group. Its lines `PRINCIPLES.md.tmpl:81-94` were written by `8e63891` and `04e1cb9`, not by the seed.
- `no_bare_primitive_exists_anywhere_in_the_stack`: ratified founding -> op list, same group (`PRINCIPLES.md.tmpl:17-27`, by `04e1cb9`).
- `rcm_always_runs_and_reorders_the_units_so_column_sharing_units_are_adjacent_for_fiber_grouping`: stays ratified, with a `later:` entry for `202607200800_a1-1-ordering-theory-bench-expansion.md:82-90`.

## Step 3: rows classed founding that later rounds wrote

`git blame` confirms the three commits. 119-138 is `18fcb5d`. 217-223 is `d77e744`. 199-202 and 227-236 are `e3fa3f4`. Line 198 is the seed.

- `no_subsystem_has_bespoke_storage_and_a_subsystem_needing_to_hold_n_things_uses_a_column`: ratified founding -> stamp (`PRINCIPLES.md.tmpl:128-133`, `18fcb5d`).
- `the_pool_frame_synchronisation_structure_is_the_single_sanctioned_fixed_array_and_a_storage_lint_enforces_the_rule`: ratified founding -> stamp (`135-138`, `18fcb5d`).
- `the_context_of_a_unit_is_a_generic_associated_type_over_a_frame_lifetime_the_scheduler_owns`: ratified founding -> stamp (`217-223`, `d77e744`).
- `every_crate_is_no_std_with_no_allocation_and_the_consumer_provides_the_memory_provider`: stays ratified founding, now cited to the seed text `680a672:mock/PRINCIPLES.md.tmpl:5-7`, with a `later:` entry for `OP-ANSWERS-0918.md:20-32`. The changelist entry was dropped.
- `there_is_no_std_platform_tier`: ratified founding -> ratified quote, `OP-ANSWERS-0918.md:20-32` and changelist `202609181402_changelist.doc.lock.md:13-13`. The question names the std tier as the OS-thread tier, and he chose that it lives outside the family.
- `no_crate_spawns_threads_and_the_executor_behind_the_thread_pool_contract_is_the_consumer_s_or_lives_outside_the_family`: ratified founding -> ratified quote, same passages. This is option 1 of his answer.
- `the_pool_is_the_consumer_s_executor_and_the_engine_hands_it_its_closures_once_on_the_first_parallel_run`: split. This id is now a stamp, narrowed to the engine handing out its closures once on the first parallel run (`PRINCIPLES.md.tmpl:227-229`, `e3fa3f4`). New ratified row `the_pool_is_the_consumer_s_executor_behind_the_executor_contract_and_its_workers_persist_for_the_pipeline_s_lifetime` holds his part: quote `OP-ANSWERS-0918.md:20-32` and founding `consolidation.md:1799-1800`.
- `the_engine_chooses_which_core_owns_which_trunk_and_pinning_a_thread_to_a_core_is_the_executor_s`: ratified founding -> stamp. The trunk assignment is his and is already held by `each_trunk_is_pinned_to_a_pool_thread_at_plan_time_and_leftover_threads_take_convergence_branches_and_bridges`. The pinning rule is the round's derivation. I made no split, because his part already has a row.
- `the_closures_handed_out_number_the_smaller_of_the_clamped_worker_count_and_the_parallelisable_width_plus_the_convergence_worker`: ratified founding -> stamp. The seed line `680a672:...:24` is held by `the_thread_count_is_the_smaller_of_the_physical_cores_and_the_parallelisable_width_plus_one`.
- `platform_tiers_are_os_and_no_os_only_and_the_std_tier_is_deferred_indefinitely`: stays ratified quote. Its `quote: PRINCIPLES.md.tmpl:196-201` entry (e3fa3f4 text) is replaced by `OP-ANSWERS-0918.md:20-32`.

## Step 4: stamps rewritten to say what of the row is his

All seven stay stamps. Each stamp now names the passage:

- `n_core_dispatch_walks_the_flat_carrier_gated_by_a_runtime_mask_of_the_trunks_a_core_owns_and_the_const_gated_enumeration_is_superseded`: his in full, as his decision of 2026-06-07 (`202606071109_topic.gate2-r4a-core-phase-masks.md:4-4, 13-17`). It was replaced later by the const-gated per-trunk dispatch (`202606072100_gate2-canonical-deviation-ledger.md:3-3`, round `202606080100` line 6) and by the row `the_single_core_run_walks_...`.
- `the_unified_engine_amendment_carries_authority_over_intermediate_artifacts_and_the_standalone_carries_none`: nothing of it is his in words. It comes from the A2-1 trace (`a2.md:43-46`), whose standing waits on question `hilavitkutin_is_the_a2_1_precedence_trace_your_ruling`. Its authority rests on A2-2 (`a2.md:50-51`).
- `the_os_tier_creates_no_threads_and_ships_a_memory_provider_and_a_clock_while_the_engine_hands_the_executor_one_pointer_sized_closure_per_worker`: his part is "Consumer supplies it" (`OP-ANSWERS-0918.md:20-32`). The mmap, clock_gettime and one-closure parts are the changelist's derivation.
- `the_plan_affecting_marker_is_an_open_per_type_opt_in_and_not_a_sealed_builder_input_marker_with_one_run_configuration_implementor`: his part is A2-3 (`a2.md:55-62`, row `plan_affecting_is_an_open_marker_trait_that_consumers_implement_on_resource_value_types`). The rest is the changelists.
- `the_maintainer_ordered_a_redo_of_verification_pass_one_against_the_original_sources`: his part is the order (`202607210300_seed-verification-redo.md:3-6`). The verdict and the fixes are the agents'.
- `the_drift_fix_arc_is_the_storage_model_work_and_no_open_storage_model_fork_remains`: his part is the hybrid call (`202606210600_topic.hybrid-decision.md:12-19`). The layout is the bench row. The framing is the agent addendum's.
- `the_single_core_run_walks_the_flat_value_carrier_gated_by_const_phase_and_trunk_so_dead_code_elimination_leaves_one_member_only_program_per_phase_and_trunk`: his part is the correction that trunks come before parallelism (`202606071100_topic.gate2-const-gated-dispatch.md:10-10`), under a mandate whose words the repository does not hold.

Not settled: the n_core and single_core rows carry his words on opposite sides, the runtime mask against the const-gated programs, so they may belong in a part-two group for him. I left them on stamps as the step asked.

## Step 5: A2-1 entries

- The `ruled: a2.md:29-36` or `30-36` entry was dropped and the row stays ratified, because the storage addendum and the bench findings back each one whole: `the_noalias_invariant_is_kept_as_an_architectural_guarantee_and_not_claimed_as_a_scalar_speedup`, `the_scalar_members_of_a_resource_are_snapshotted_to_a_stack_local_before_the_morsel_loop`, `the_value_of_a_resource_is_one_contiguous_record_blob_and_the_per_member_column_decomposition_was_refuted_by_bench`, `a_seq_or_map_value_is_a_pointer_and_a_length_and_its_elements_are_live_streamed_from_their_column_and_never_copied`, `the_spec_s_three_field_types_are_value_composition_read_by_the_decompose_seam_and_not_a_column_per_member`, `a_resource_value_is_a_one_record_blob_and_the_bench_reverses_the_decomposition_to_shape_bound_columns`.
- The `later:` A2-1 entry was dropped and the row stays ratified: `the_unified_engine_amendment_is_canon_with_the_standing_of_the_consolidation_spec_and_wins_over_intermediate_artifacts` (its own status line `202606061000:3-3` backs it) and `the_consolidation_is_the_single_source_of_truth_for_the_current_design`. Step 2 later moved the second row to a stamp for its own reason.
- Ratified -> stamp, each stamp naming A2-1 and the question id:
  - `addendum_a1_is_an_amendment_to_canon_and_where_it_is_silent_the_spec_and_registered_chain_stand`
  - `a_new_standalone_canonical_spec_is_written_self_contained_and_spares_cross_reading_the_chain`: the item is in A1's sequence section, not its rulings.
  - `the_standalone_spec_is_a_commissioned_reading_aid_by_its_own_corrected_header`
  - `the_maintainer_delegated_the_precedence_ruling_to_a_provenance_trace_of_the_record`: his quoted words exist only inside A2-1 at lines 12-13.

## Step 6: words that are not his

- `op_ruled_that_the_consumer_supplies_the_os_thread_executor_and_the_engine_ships_only_the_executor_contract`: stays ratified quote, with the tests and bench sentence taken out of `what` and `mechanism`. That sentence is now the new stamp row `tests_and_the_engine_versus_std_bench_define_their_own_executors_in_test_and_bench_code` (`202609181401_topic.hilavitkutin-on-arvo-dev.md:81-81`).
- `op_ruled_that_the_persistence_crate_builds_on_the_str_and_api_crates_and_never_on_the_engine_or_the_context_crate`: ratified quote -> stamp, keeping the round's lint and cargo-rule wording, including the context crate that its id names. New ratified quote row `op_ruled_that_the_persistence_crate_builds_on_the_str_and_api_crates_and_its_design_holds_over_the_lint` holds his part, from `OP-ANSWERS-0918.md:36-46`. The two part-one findings that named the old id now name the new one.

## Step 7: part one of ROWS-FOR-OP

These seven get `superseded_by` (whole replacement). Each heir got a `lineage` entry so the lint passes, `for_the_sized_cases_...` got one too, and one question's `related` dropped the replaced id:

- `only_a_column_contributes_per_record_bytes_to_the_morsel_working_set_and_the_other_store_markers_contribute_zero` -> `a_resource_and_a_staged_resource_contribute_their_declared_footprint_to_the_morsel_budget_and_an_accumulator_contributes_zero`
- `a_resource_handle_s_members_decompose_into_per_member_shape_bound_columns_in_the_unified_store` -> `a_resource_value_is_a_one_record_blob_and_the_bench_reverses_the_decomposition_to_shape_bound_columns`
- `as_of_round_202607193110_a_replaceable_swap_is_a_member_by_member_copy_into_column_slots_and_not_a_blob_write` -> `a_replaceable_swap_installs_the_whole_value_as_one_blob_write_through_the_drain_s_selector_and_the_member_by_member_wording_is_refuted`
- `the_nested_value_carrier_walk_runs_a_trunk_over_a_carrier_of_fibers_and_the_scheduler_run_is_re_pointed_onto_it_in_a_later_step` -> `trunk_isolation_comes_from_a_flat_carrier_walk_gated_by_const_phase_and_trunk_and_the_nested_value_carriers_are_superseded`
- `the_scheduler_metrics_were_first_a_consumer_resource_populated_by_a_provided_metrics_unit_with_no_kernel_write` -> `the_scheduler_metrics_are_engine_owned_in_a_meta_block_...` (that heir is now on the op list itself)
- `a_resource_column_or_virtual_is_registered_through_the_one_with_verb_by_constructing_its_library_wrapper_at_the_call_site` -> `a_consumer_type_is_itself_the_builder_input_and_the_kind_wrappers_are_deleted_so_values_are_passed_bare`
- `fixed_capacity_is_a_non_generic_capacity_trait_implemented_by_a_macro_generated_ladder_of_exact_rungs_binding_literal_length_arrays` -> `fixed_capacity_is_one_generic_dimension_marker_implementing_the_capacity_trait_and_the_macro_rung_ladder_is_dropped`

Already superseded before this work: `the_founding_research_orders_two_writers_by_registration_order_or_explicit_priority`.

The heir replaces only in part, so no field was added:

- `waiting_is_parked_in_three_tiers_...` (the per-OS atomic wait is not in `predictive_parking_...`)
- `the_adapt_configuration_is_nine_independent_axes_...` (the independent axes and cadence are not in the kits row, which is itself pending)
- `as_of_round_202605101036_plan_affecting_is_a_sealed_marker_...` (the dirty bit, cheap swap and default are not in the open-marker row)
- `the_ecosystem_crates_are_standalone_...` and `the_persistence_crate_is_the_generic_hot_cold_storage_bridge_...` (only the standalone part)
- `the_run_returns_the_foundation_outcome_type_...` (the completion-arc row is about charting)
- `a_per_core_program_over_an_enumerated_set_...` (the outer-dispatcher row is a mechanism, not the impossibility claim)

The finding names no tenet holding his word, so no field was added: `an_append_past_an_accumulator_s_reserved_capacity_...`, `an_accumulator_append_saturates_...`, `the_effective_canon_is_the_consolidation_spec_as_modified_in_order_by_seven_registered_amendments`, `the_single_core_dispatch_is_a_column_capable_inline_walk_...`, `the_capacity_lift_is_made_for_the_one_capacity_...`, `gate_one_takes_registration_as_canonical_...`, `the_two_writers_refusal_rests_on_the_data_bound_form_...`.

## Rows the review named and I left as they were

- `the_value_of_a_resource_is_one_contiguous_record_blob_and_the_per_member_column_decomposition_was_refuted_by_bench` stays ratified. The bench addendum is bench-decided with his hybrid call, and only its A2-1 entry went.
- `the_os_and_std_tiers_are_mutually_exclusive_and_the_os_tier_gets_the_optimisation_investment` was already on the right stamp.

## Not settled

- About 30 rows classed `founding` cite `mock/PRINCIPLES.md.tmpl` lines 1 to 195. Those lines were written by `8e63891` (2026-04-20, a port of a shared principles fragment held at the workspace's `.shared/principles-fragment.md`) and `04e1cb9` (2026-04-20), not by the seed or a style sweep. They cover bare primitives, trait contracts, growable containers, enums and capacity. By the brief's ladder they are not founding. Step 3 named only the `18fcb5d`, `d77e744` and `e3fa3f4` ranges, so I moved only the three of these rows that conflict with his founding text (step 2). The rest stay ratified founding and want the same commit check.
- `ai_tooling_is_used_responsibly_with_a_co_authored_by_line` (founding `2236`) may stand against the workspace byline rules. Those rules are outside this repository's ladder, so I did not move it.
- `fill-rows.sh` refills every row in a file. On a row that carries `rev` (in `governance-3.toml`, `kits.toml`, `memory-2.toml`) it moves the `rev` line and refills the quote from the working tree. I caught this in `memory-2.toml`, restored the file and filled the new row by hand. This is a tooling problem worth a ticket. I filed none, because the brief did not allow pushes or other clones.
