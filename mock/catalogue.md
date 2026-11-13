# Catalogue of ignored tests

Each `#[ignore = "catalogue: <the gap>; tracked #<id>"]` in this repository names one of the rows below. A row is the id of the ticket that tracks the gap, and the gap it states. The `catalogue-ignore` lint reads this file through its `ledgers` key in `mockspace.toml`. The older ids `#341` (adapt) and `#344` (persistence) came from a tracker that no longer exists, and each row names the one it replaces.

- hilavitkutin_catalogue_adapt_non_degradation_on_balanced_wor: adapt does not degrade balanced workloads; needs the adapt on/off toggle (was #341)
- hilavitkutin_catalogue_select_adapt_config_improves_imbalanc: adapt improves imbalanced workloads and select_adapt_config reduces the bottleneck phase_ema; needs select_adapt_config (was #341)
- hilavitkutin_catalogue_morsel_re_chunk_lowers_idle_ns_and_ne: a morsel re-chunk lowers idle_ns without raising total time; needs the tier-1 re-chunk (was #341)
- hilavitkutin_catalogue_strategy_reselect_keeps_throughput_no: a strategy reselect leaves throughput non-decreasing; needs the domain-14 strategy plan-shaping and the reselect (was #341)
- hilavitkutin_catalogue_predictive_parking_cuts_wait_waste_an: predictive parking cuts wait waste without added latency; needs the wait-tier actuation (was #341)
- hilavitkutin_catalogue_p_e_core_routing_beats_uniform_routin: P/E-core routing beats uniform routing on a P/E machine; needs morsel-temperature and CoreClass routing (was #341)
- hilavitkutin_catalogue_run_parallel_hands_out_one_closure_pe: run_parallel hands out one closure per clamped worker, not the principles' bound
- hilavitkutin_catalogue_a_min_morsel_that_is_not_a_multiple_o: a MIN_MORSEL that is not a multiple of 4 lets the morsel window land below the floor (was #341)
- hilavitkutin_catalogue_fiber_plan_idx_records_the_csr_index: fiber_plan_idx records the CSR index when dispatch order reorders fibers; needs a multi-trunk fixture and the accessor (was #341)
- hilavitkutin_catalogue_seq_and_map_collection_members_are_no: Seq and Map collection members are not wired into resource values (was #344)
- hilavitkutin_catalogue_trampoline_survival_under_fat_lto_is: trampoline survival under fat LTO is unchecked off unix; needs a symbol reader
- hilavitkutin_catalogue_cold_store_flush_and_open_need_the_mm: cold store flush writes nothing and open reads no manifest; needs the mmap-backed file I/O (was #344)
- hilavitkutin_catalogue_engine_vs_std_perf_oracles_are_timing: the engine_vs_std perf oracles are timing assertions that only mean something in release; they belong on the bench harness
- hilavitkutin_catalogue_cold_store_snapshot_copies_nothing_an: cold store snapshot copies nothing; needs the backup mechanism (was #344)
- run_fused_does_not_refuse_a_recordop_unit_that_writes_an_acc: run_fused runs only the map and never the body of a RecordOp unit that also appends; needs the fused entry to refuse the carrier, or to run the body
- run_parallel_refuses_a_multi_append_skew_that_fits_the_reser: run_parallel caps a core's accumulator region at its record slice, so a frame whose appends fit the reservation but put more than one append per record on one slice is refused at more than one worker and accepted at one; needs region caps sized for the per-region worst case
- hilavitkutin_catalogue_build_returns_a_scheduler_when_the_pl: build returns a scheduler when the plan has more fibers than the descriptor pool holds, and in release the units past the pool never dispatch; needs build to refuse a plan it cannot fully describe (was #340)
- hilavitkutin_catalogue_a_single_trunk_phase_is_walked_by_one: a single-trunk phase under run_parallel is walked by its one owning core while the other workers take none of it; needs head+tail convergence, which is designed and not built, so that a second core walks the trunk from its tail
- hilavitkutin_catalogue_a_swap_between_two_run_parallel_frame: a swap between two run_parallel frames takes a mutable scheduler while parked workers hold a shared one; needs the scheduler-plane relocation, after which the case is a Miri-gated check (was #689)
