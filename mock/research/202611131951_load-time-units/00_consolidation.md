# Load-Time Work Units, Consolidation

This consolidates the load-time units panel of 13 November 2026: seat 01 (`01_aaltonen.md`), seat 02 written blind to it and then reconciling (`02_tiark-rompf.md`, reconciliation from its section "Reconciliation with seat 01"), and seat 01's reply (`03_aaltonen-reply.md`). Op asked for the panel in op-answers-28, choosing "Panel it first" over "Content is data, units fixed" and "A seam for load-time units". The panel also took in the growth question and the consumer-owned lane and cell format, re-filed here against `hilavitkutin-api` because the tickets that asked them were written against the voided `hilavitkutin-contracts`, and op's word on kaski's narrow-lane ticket: "Upstream on hila it should be entirely generic and configurable by user".

hilavitkutin declares no `canon_paths`, so the founding record and the designs stand in for a canon. The record is `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, cited here as `spec :N`. Everything below is research: a candidate for op's ratification and for design rounds, never authority by itself. Provisional stamps are two seats agreeing on a call the record cannot derive, and each goes to op as a question ticket.

On independence. Seat 02's cold open was blind to seat 01, so where the two cold opens agree, that is two instruments. Seat 01's reply was written after reading seat 02, so its agreement with seat 02 is confirmation, not corroboration, as seat 01 says itself (`03:20`). Each item below says which it is.

## The composition

1. The schedule is the build's. Units, the graph, the topology and the dispatch functions are fixed at build for every consumer and every load (R6, spec `:2435-2440`). Nothing that arrives at load becomes a unit. Two instruments: both cold opens.
2. What arrives at load is data, an interpreted program, or a native kernel. Data is rows in typed columns with kinds interned at load (seat 02 `s3`). An interpreted program is checked at load against a compiled unit's declared slots and runs a vector at a time per morsel (seat 01 `a3`). A native kernel is compiled by its author, never by the player. Two instruments.
3. Programs and kernels are occupants of a compiled host unit, held in an ordinary `Resource` the unit declares, as long as the occupant is a `Copy + 'static` C-ABI table (seat 02 `s6`, seat 01 `m3`). Two instruments.
4. Swapping an occupant is a `Replaceable` swap: a dirty seed and no plan band, for a swap that changes no plan input. A load that also changes a count or a gate replaces a `PlanAffecting` resource beside it and buys one band (`m3`, `s6`; spec `:2125`). Seat 02 first wrote the occupant itself as `PlanAffecting`; seat 01's reply refined it. One instrument on the refinement.
5. The call into an occupant crosses once a morsel, never once a record. The occupant gets a wire view a morsel: one base pointer per declared column and the count (spec `:397-398`, raw pointers per R5, spec `:2433`). The trunk's context has no such slice (`hilavitkutin-api/src/context.rs:117`, `:134`). What the boundary costs against the same kernel compiled and fused is unpriced.
6. An occupant's library is released only through a path that swaps the occupant out first, and a plain drop leaks the mapping. The owner sits above the engine and `hilavitkutin-extensions`, in the consumer or an opt-in bridge crate. Without that owner the bare form segfaults on the first dirty frame after the drop (`m3`, `OUTPUT-unload.txt`). One instrument.
7. Admission of a native occupant is the consumer's; native code is not bound by a declaration (`s2` red). An interpreted program is checked by its own interpreter.
8. No interned handle crosses the C ABI. An occupant names stores and keys as text in its descriptor, the host interns them at load, and the occupant refers to them by position. The base game is compiled before any mod exists, so a mod's names can reach it only as data (`03` M5).
9. Growth is the record's: reserve and commit on the os tier, pages sized in records elsewhere, the count a plan parameter (core `:683-685`, spec `:2443`; seat 02 `s4`, seat 01 `b3`). `reserve` keeps contents, because its design says it resizes (seat 01 `b2`). Two instruments.
10. Deletion is the R3 tombstone. Identity is data in a column the consumer owns, and a compaction reaches the plan as a count change (spec `:385-386`, `:859`).
11. A consumer's value type is a column value at its native stride up to sixteen bytes (R3, R4; `s5`, `c1`). Where a fiber's live set passes L1 at the morsel floor, the answer is domain 12's micro-morsel tiling (spec `:845-852`), not a per-cell bound (`s7`). This is the generic, consumer-configured cell op asked for on the narrow-lane ticket: the type is the consumer's, and the cap is the record's.
12. `repr(C)` binds hilavitkutin's and arvo's own stored types through the internal lint, consumer types that opt into `#[derive(ColumnValue)]`, and every value type that crosses an image seam (spec `:462`, `:474`; T3 `:2988-3022`; seat 01 `c6`, three sizes from one source across three builds).
13. A packed word written by more than one core in a phase needs word-aligned shares or an atomic word write, whoever the writer is (seat 01 `c2`, `c7`).

## Stamps and calls put to op

Each is a question ticket with `--holder op` and real options.

| Merged call | Question ticket | Standing |
|---|---|---|
| M1, does loaded code ever become a unit | `does_code_arriving_at_load_ever_become_a_unit_in_hilavitkuti` | derived from R6, put to op because he asked for the panel |
| M2, what a plugin load is to the plan | `what_a_plugin_load_is_to_hilavitkutin_s_plan` | provisional stamp |
| M3, the fate of `ExtensionSurface` and `LinkedBin` | `what_becomes_of_extensionsurface_in_hilavitkutin_api` | recommendation; deleting a tier-2 section is op's |
| M4, M5, M6 and M12, admission, names, isolation, `repr(C)` at the seam | `admission_names_isolation_and_repr_c_at_hilavitkutin_s_code` | stamps, M6 derived |
| M8, identity across removal | `what_a_record_s_position_means_across_a_removal_in_hilavitku` | provisional stamp; re-files the growth row |
| M11, the default writer of a packed column | `the_default_writer_of_a_packed_column_in_hilavitkutin_api` | op's, the queue's locus is open by his own words; re-files the cell row |

M7 is the coordinator's: merge this panel's evidence into `which_registration_hilavitkutin_s_contract_takes_beside_the`, whose typed-only option both seats support. M9 and M10 are not op's: the record answers both, and they are owed work below. M13 is derived (the cap restored), and M14 is the coordinator's.

## Closed, retired and corrected

The three re-filed tickets asked their questions against `hilavitkutin-contracts`, which the trunk no longer has, and are void. `decide_positions_beyond_dense_key_order_ranks_with_no_resize` is answered by item 9 and asked again as M8. `decide_the_storage_word_and_the_widest_cell_in_hilavitkutin` is answered by items 11 and 13 and asked again as M11, with the packing rule filed as work. `decide_how_a_consumer_gets_its_own_value_type_in_hilavitkuti` is answered by the trunk itself: any `Copy + 'static` type is a column value (`hilavitkutin-api/src/column_value.rs:25-34`), and what is owed in its place is the derive. The coordinator closes the three by naming them.

Seat 01's `c1` is withdrawn as the rationale for the sixteen-byte cap, and its derived bound with it; R4's own reason stands (spec `:461`). Seat 01's "`repr(C)` on every stored type" read the summary line without its source and is corrected by `03` section 3.1. Seat 01's first suite count hid two crates' 19 passing tests; the whole count is 483 passed, 0 failed, 24 ignored, with two binaries that do not link (`03:24-26`).

## Benches owed

None of the panel's sketches carries a timing figure, and nothing here is priced.

1. M1: an occupant once a morsel over a wire view, against the same kernel compiled as a unit and fused, against A3's interpreter, against Approach C, on the intended region of millions of records, opt-level 3 with fat LTO.
2. M11: {single drainer at the close, record-split drain rounded to the word, queue-split drain with atomic word ops, direct writes with rounded windows, the byte-per-record `bool` column} at 1e5 to 1e7 records and one to four cores.
3. M10: `s7`'s forty-column fiber and `c1`'s wide-cell fiber, {single-level at `MIN_MORSEL`, tiled by peak live}, against the same data in narrow fibers.

## Owed work, as tickets

Filed in this job:

- `a_second_reserve_of_a_column_drops_its_records_against_the_r`
- `two_api_tests_assert_a_stub_defined_beside_them_and_test_not`
- `column_values_lost_the_record_s_sixteen_byte_cap_with_no_rat`
- `extensionsurface_breaks_build_when_registered_and_promises_a`
- `the_record_count_is_fixed_at_build_and_no_plan_band_moves_it`
- `a_unit_s_context_has_no_per_morsel_window_slice_of_its_colum`
- `the_memory_provider_has_no_reserve_and_commit_and_protect_is`
- `str_const_leaks_arvo_bits_and_a_feature_gate_and_its_lookup`
- `a_derive_column_value_enforces_repr_c_on_domain_newtypes`
- `micro_morsel_tiling_answers_a_fiber_whose_write_set_passes_l`
- `symbol_get_yields_a_fn_pointer_that_outlives_its_library`
- `the_planner_owes_the_packed_word_rule_before_bitpacked_lands`

Bumped: `hilavitkutin_linking_passes_the_macos_rtld_local_value_on_li`, `linking_has_no_test_of_rtld_local_isolation_and_lto_smoke_ma`, `hilavitkutin_str_collects_strings_through_linker_sections`, `hilavitkutin_dev_tests_linking_hilavitkutin_str_fail_on_unde`.

## Test gate

Both seats ran the suite from `mock/` on the pinned toolchain. It does not build whole: `hilavitkutin-persistence`'s `archive_str` and `hilavitkutin-providers`' `smoke` fail to link on the `hilavitkutin_strings` section bounds (bumped above). Both seats named two tautological tests for deletion (`hilavitkutin-api/tests/column_storage.rs:47-55`, `column_value.rs:48-62`) and the linking suite loading only libc. The write scope forbade fixing them, so every claim here rests on a committed sketch that calls the shipped code, never on a suite test.
