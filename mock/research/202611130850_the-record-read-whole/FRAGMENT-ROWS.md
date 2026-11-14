# Report: 21 tenets ratified on lines of the ported principles fragment

Commit `a30672f1` on `work/canon-registry-frag` in `/home/user/.worktrees/hv-frag`, not pushed. `cargo mock` exits 0 before and after; it regenerated nothing tracked.

Paths below: P = `mock/PRINCIPLES.md.tmpl`, S = `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, T1 = `mock/design_rounds/202604200055/202603111529_topic.paradigm-and-architecture.md` (a round topic, rung 4), F = `mock/research/202611130850_the-record-read-whole/OP-ANSWERS-FRAGMENT.md` (new). Every stamp, written in full in the rows, names the fragment lines and their commit and the passages searched.

## Outcomes

| Row | Outcome | Deciding passage | Searched |
|---|---|---|---|
| allocation_is_managed_by_the_scheduler_and_declared_at_compile_time_and_never_fabricated_at_runtime | re-cite | F:25-25 (quote), 680a672:P:9-11, S:284-286, S:543-545 (founding) | S:271-316, S:531-560, S:1455-1475 |
| every_crate_is_no_std_with_no_allocation_and_the_consumer_provides_the_memory_provider | no change: its founding entry already cites the seed's own lines, 680a672:P:5-7, with the later OP-ANSWERS-0918 entry; pr-ranges listed it by the current file's lines | 680a672:P:5-7 | |
| signatures_take_trait_bounds_and_the_concrete_type_is_whatever_the_caller_brings | split: stays ratified on F:13-17 (quote) and S:2221-2221 (founding), `because` removed, source narrowed to P:44-45, P:47-48; new pending row monomorphisation_compiles_trait_generic_signatures_to_the_same_code_as_concrete_ones_and_call_site_inference_needs_no_turbofish (P:48-50) holds the reason. The ratified row's words do not name the pending one, so no `cites` | F:13-17, S:2221-2221 | S:304-306, S:1528-1616, S:2198-2238 |
| a_concept_that_might_need_extension_is_a_registered_trait_and_enums_are_for_truly_closed_sets_only | stamp | none | S:2082-2196, S:2198-2238, T1:140-162 |
| a_foreign_contract_forcing_a_primitive_is_first_met_by_implementing_it_on_an_arvo_type_and_otherwise_the_crate_is_dropped | stamp | none | S:130-270, S:2198-2256, F:27-63 |
| a_need_for_a_growable_container_means_the_shape_is_wrong_and_is_reshaped_toward_an_emitter_iterator_or_bounded_store | stamp | none | S:271-316, S:531-560, S:2198-2238, F:19-25 |
| a_position_where_two_implementations_with_different_behaviour_are_imaginable_is_a_trait_slot_and_never_an_extensible_enum | stamp | none (test is in T1:151-155 only) | S:2198-2238, T1:140-162 |
| all_dispatch_goes_through_monomorphised_trait_bounds_because_dyn_forces_late_binding_heap_and_unstable_vtable_layouts | stamp | none for the reason | S:1528-1616, S:2198-2238, T1:1032-1050 |
| collections_never_cross_ecosystem_api_boundaries_and_the_boundary_carries_data_flow_and_not_storage | stamp | none | S:531-560, S:1455-1475, S:2198-2238 |
| container_capacity_lives_at_type_level_and_a_consumer_needing_more_picks_the_bigger_type | stamp | none | S:130-270, S:531-560, S:1455-1475, F:19-25 |
| every_registrable_type_has_one_constant_name_and_the_descriptor_name_points_at_it | stamp | none | S:2082-2196, S:2198-2238 |
| extension_across_an_abi_boundary_is_by_registered_traits_and_c_layout_descriptors_and_never_by_dyn | stamp | none | S:2198-2238, T1:1117-1152 |
| schema_kinds_events_settings_domains_storage_locations_and_access_levels_are_zero_sized_markers | stamp | none | S:2198-2238, F:7-17 |
| six_lints_scan_every_line_of_every_source_file_and_block_the_commit_when_a_bare_primitive_is_written | stamp | none | S:130-270, S:2198-2256 |
| standard_trait_methods_whose_signatures_the_language_fixes_are_the_escape_hatch_and_everywhere_else_the_rule_stands | stamp | none | S:2198-2256, F:27-63 |
| the_lint_allow_escape_hatch_is_for_a_genuinely_unavoidable_boundary_and_always_carries_a_tracked_remediation_task | stamp | none; F:62-63 stands against it | S:2198-2256, F:27-63 |
| the_patterns_of_the_stack_are_all_instances_of_one_trait_contract_rule | stamp | none | S:130-270, S:2198-2238, F:7-17 |
| the_primitive_vocabulary_table_names_the_stack_type_that_replaces_each_bare_primitive_and_applies_everywhere | stamp | none | S:130-270, S:2198-2256 |
| the_stack_supplies_its_own_vocabulary_for_each_bare_primitive_in_arvo_notko_and_the_string_crate | stamp | none | S:26-66, S:130-270, S:2198-2256 |
| there_is_one_mechanism_per_concern_and_where_two_subsystems_solve_one_problem_differently_one_must_go | stamp | none | S:2198-2300 |
| trait_signatures_return_iterators_and_take_sinks_and_never_name_a_vector_or_boxed_slice | stamp | none | S:531-560, S:2198-2238, F:7-17 |

Rule I applied: where a rung 1 to 3 passage states part of a row and an existing ratified row already holds that part (dyn_is_banned_entirely_with_no_exceptions, zero_sized_types_are_the_logic_layer, contracts_come_before_implementations_and_traits_come_first_always, a_resource_is_scalars_and_const_sized_collections_only, hilavitkutin_does_not_use_the_alloc_crate_and_dynamic_storage_goes_through_slab_columns_or_stack_arrays, repr_c_and_function_pointers_give_the_minimal_abi_surface, there_is_no_ad_hoc_logic_and_only_reusable_abstractions), I stamped the row and named that row in the stamp rather than splitting off a near-copy of it. Only the signatures row had a stated part no other row holds, so it is the one split.

Judgement on the allocation re-cite: "managed by the scheduler" rests on the seed's plan-time allocation, 680a672:P:9-11, while S:1462 says hilavitkutin does not allocate storage, the consumer providing it. Op at F:25 also allows "large and even dynamic allocations" on a statically typed backing. I read both as consistent with the row; a stricter reader would split off "managed by the scheduler".

## Whether the fragment carries op's words

- `git -C /home/user log -- .shared/principles-fragment.md` shows one commit, `8228da363` of 2026-07-21, a workspace snapshot; the history before it is not in that repository. The April text cannot be traced to him by commit.
- `cargo mock already-said` and `unearth` find the fragment's phrases only in the registry rows themselves and in `mock/research/imported-from-saalis/`, agent-written research. The tool could not read the captures from the worktree (`../.data/op-responses` is not a directory there), so I grepped `/home/user/.data/op-responses/` directly: no capture names the fragment or quotes any of its phrases ("No heap. Ever.", "smells like ass", the decision test, "one mechanism, not two", `Named::NAME`, "ZST markers", "just for this moment").
- The decision test and the plugin-extension line come from T1:151-155, the polka-dots paradigm topic, whose decisions log marks them "Interactive review", with no quote of him.
- His words that bear on single rows, copied by `sed` into F with the source file and range in each heading, nothing reworded:
  - `.data/op-responses/202609040931_contracts-first-is-a-rule-in-shared-rules.md:23-29`, contracts first with bounds and associated types, at F:13-17; cited `quote` on the signatures row.
  - `.data/op-responses/202611121758_op-on-notko-no-alloc-and-his-stack-under-hilavitkutin.md:34-36`, no transient heap and statically typed backing, at F:25; cited `quote` on the allocation row.
  - `.data/op-responses/202609022036_huurre-wayland-lint-allow.md:50-82`, with the options he was asked, at F:31-63. His answer at F:62-63, "The upstream needs to remove the tracked line", stands against the tracked task.

## Not settled, for the coordinator

- the_lint_allow_escape_hatch_is_for_a_genuinely_unavoidable_boundary_and_always_carries_a_tracked_remediation_task, and the tracked line in standard_trait_methods_whose_signatures_the_language_fixes_are_the_escape_hatch_and_everywhere_else_the_rule_stands, stand against op's 2026-09-02 answer (F:62-63). By OP-ANSWERS-18 these are "actively against" rows, candidates for ROWS-FOR-OP or for supersession by a row holding his answer. I left them stamped only, since writing that row was outside the brief.
- a_concept_that_might_need_extension_is_a_registered_trait_and_enums_are_for_truly_closed_sets_only and extension_across_an_abi_boundary_is_by_registered_traits_and_c_layout_descriptors_and_never_by_dyn name `#[register]` and the inventory pattern. S:2105-2107 says hilavitkutin has no registration attribute, and discovery_is_pull_based_only_and_linker_section_scanning_registration_is_rejected rejects inventory. Also candidates for op.
- six_lints_scan_every_line_of_every_source_file_and_block_the_commit_when_a_bare_primitive_is_written: what the code does is a separate fact. In `mockspace.toml`, five of the six lints run at `warn`, and `cargo mock` printed 307 warnings without blocking, so the code does not block on them. The design point sits with no_bare_primitive_exists_anywhere_in_the_stack, already in ROWS-FOR-OP.
- Op's 2026-10-02 kaski words ("Gene can not be an enum ... extensible via mods") and his 2026-10-27 console words ("Enums for closed sets") bear on the enum rows. They are about other projects, so I did not copy or cite them.
