# Review pass 3 of 3: `work/hv-engine-path`, 7315943a..e8f85c8a

The third and last review pass over jobs 1 to 3 of `mock/research/202611140100_engine-path-plan/PLAN.md`, by the same reviewer as passes 1 and 2, read on a detached worktree at e8f85c8a. Verdict: MERGE, nothing blocks landing; what is left is recorded as follow-ups below.

The canon gate passed against the ratified rows `ordering_is_guaranteed_by_construction`, `a_fiber_is_commutative_only_if_all_its_units_are_and_resource_accumulation_uses_per_thread_accumulators_merged_after`, `the_engine_is_one_and_the_core_count_is_configuration`, `a_unit_whose_inputs_are_all_clean_skips_the_pass` and `execute_returns_nothing_and_a_failing_unit_aborts_because_abort_gives_better_llvm_codegen`, and the provisional rows `a_mixed_carrier_keeps_incremental_skip_per_fiber` and `clock_and_libc_unix_memory_provider_defaults_land_in_their_own_rounds_when_consumer_demand_surfaces`, cited as provisional.

The test gate passed: the wide differential 7 pass, and with `--include-ignored` exactly the 6 ignored fail; `unit_panic_aborts` 3 of 3; `static_bump_memory` 7 of 7. Every test body in the touched surface was read, none is tautological, and every control has a committed failing counterpart.

## Prior findings

N1, the unit-outer count model: resolved. `tiled_cores`, the `parallel:` factor and the tiling test are deleted, and counts are judged against the cone by `against_the_cone` with the expected values from the canon rows. The control is red at frame 0 on producer 0 at every count (got 1, 2, 3, 4, 255; want 5), ignored under `the_ungated_n_way_record_split_ships_where_its_own_round_del`.

The one-worker divergence is not asked of op, and the reviewer agrees: `the_engine_is_one_and_the_core_count_is_configuration` requires the same code paths at one core as at seven, which settles it and outranks the provisional wording on mixed carriers. Ticket 115, the one frame body, is the repair.

N2, counts from the cone and the record-less stores: resolved. `run` and `run_parallel` share one judge. The record-less count is red at frame 1 (got 1, want 0) and the windowed count red at frame 1 (got 5, want 0) at all counts, both under `run_parallel_dispatches_every_unit_each_pass_and_never_asks`. The empty-against-empty stores test is deleted.

N3, the negative controls: resolved. `the_block_check_rejects_a_bump_that_aligns_by_offset` feeds a bump aligning by offset through the same `first_broken` as the real provider; it is not pass 2's hoisted-pad mutant, but it proves the checker sees misalignment, and the hoisted-pad evidence stays in `controls.txt`. `the_boundary_check_rejects_an_abort_with_no_panic` asserts the child died by SIGABRT, then that the check rejects it.

N4, a control for the accumulator comparison: resolved. `the_comparison_reports_a_runner_that_dropped_one_accumulator_record` asserts `Bytes { frame: 3, column: ROOTS + 1, record: kept - 1 }` at every count and exercises the length branch; without the alteration the same check returns none.

N5, the providers design and `from_raw_parts`: resolved for the design; the new test's weakness is F1.

N6, the fixture's casts: resolved, `#[repr(transparent)]` on `Out<K>`, `Sink` and `Seed`.

N7 and finding 10, the landing message: owed at landing. 30888615 is a breaking change under `fix:`; e6cd7de4 adds the public `unsafe` constructor `StaticBumpMemory::from_raw_parts` and rewrites the bench providers under a tests subject.

Finding 5, swap frames in the wide history: ticket 481e12d87e45, acceptable since job 5 rewrites that history.

The nine new commit subjects are within 72 characters and in shape, with no em-dash, byline or tool advert.

## Follow-ups, ranked

1. F1, low: `a_provider_over_raw_parts_is_dropped_before_its_block_is_freed` in `hilavitkutin-providers/tests/static_bump_memory.rs` cannot observe the order its name claims. `StaticBumpMemory` has no `Drop`, so swapping the provider drop and `Box::from_raw` passes natively and under Miri; the Miri red in `controls.txt` is a different mutant (the box freed before the write). Rename it to what it proves, or give it a control that fails.
2. The Miri controls are run by hand; there is no Miri tool under `mock/tools/` yet (the existing ticket), so the SB and TB passes stay hand evidence.
3. The landing message names 30888615 and e6cd7de4, as N7.
4. Low: the unit-outer count test is ignored under 116 alone while 116, 115 and 157 all block it; its reason names all three. When 116 closes it stays red and is re-ignored, not deleted.
5. Tickets the repairs wait on: 82085a68780d, 7285fd5aaa24, 7d4dfdf8c6a8, 481e12d87e45, 115, 116, 157, and ff15f8cbbdae (out of lane).
