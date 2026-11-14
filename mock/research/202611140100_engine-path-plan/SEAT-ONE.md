# Seat One: Five Open Points on the Engine Path

Persona: Sebastian Aaltonen. Opus tier. Repository /home/user/hilavitkutin at e99af156 (branch work/hv-engine-plan). Canon rows were read through `cargo mock canon` in /home/user/.worktrees/hv-canon. This is my own reading, formed without seeing the other seat's. Under the two-expert rule it is the first read on every canon call here, and a second independent read is owed.

## The two gates

Test gate: not run. The brief forbids building or running the suites (about 3 GB of disk free), so it is still owed. I read the test bodies in the area these points touch instead, and found gaps rather than a pass:
- No test panics a unit on a worker.
- No test drives `run_fused` after `replace_resource`.
- There is no Miri gate anywhere (a grep for "miri" returns nothing).
- The deviation 5 audit says it left an ignored catalogue test in `tests/replace_swap.rs` (`mock/research/202607201600_gate2-deviation-5-audit.md:46-52`). That file has no `#[ignore]`. The nearest test, `tests/replace_swap_plan_band.rs:17-20`, runs the pattern natively and says it does not assert aliasing. The audit's citation is void; its reasoning has to stand on its own.

Canon gate: run per point.
- Point 4 is mis-framed as asked.
- The `run_fused` half of point 1 is mis-framed as asked.
- Points 1 and 5 each carry one part the canon does not settle, which I hand back.

## 1. The shape of the one path

Arms.
- (a) One frame body, with the calling thread running the worker body at one core.
- (b) Three walks held to one meaning by a differential.
- (c), added: one frame body, with fusion as a per-fiber strategy the plan picks inside it.

What governs.
- `the_engine_is_one_and_the_core_count_is_configuration` (ratified; `mock/research/202606061000_canon-amendment-unified-engine.md:7-9`): "There is no separate single-core or single-threaded engine, run or dispatch path ... the same primitives and the same code paths run at one core, two cores or seven ... sequential is a per-phase strategy the plan may select, not a separate engine."
- `the_consolidation_spec_never_special_cases_single_core_as_a_code_path` (ratified): "one program, one arriver ... one worker at one core, so nothing forks."
- `a_fiber_is_the_unit_of_column_co_location_and_morsel_windowing_and_fusion_is_a_strategy_decision` (ratified; consolidation:83, :944-948).
- `the_flattener_emits_one_monomorphised_function_per_fiber_in_the_rust_pipe_pattern` (ratified).
- `each_physical_core_runs_one_monomorphised_function_that_encodes_its_whole_pipeline` (ratified; consolidation:1596-1613).

What the code does.
- `run` (`src/scheduler/run.rs:79-347`) is a calling-thread walk with no pool.
- In `run_parallel` (`run_parallel.rs:103-456`) the caller only publishes and waits. The workers walk `run_core_phase` (`worker.rs:278-390`), which goes through `dispatch_core` instead of `dispatch`.
- `run_fused` (`run_fused.rs:70-157`) folds the whole carrier into one chain.
- `run_parallel.rs:88-90` says single core is "not a separate path", while `run` exists beside it as exactly that.

Reading.
- (b) is refused: three walks are three code paths, and the ratified text says "the same code paths".
- `run_fused` becomes neither an entry nor a carrier-wide strategy. Fusion is a per-fiber decision made by the plan and the flattener, not by the caller.
- Its stated reason for being a separate entry (`run_fused.rs:54-59`: auto-detection needs `min_specialization`) is a claim about one encoding. The ratified `a_pragmatic_shortcut_is_never_taken_and_the_design_changes_and_not_the_implementation` (consolidation:2230) points at the untried encoding: a per-fiber trait that is always implemented, with an associated const that selects the fused or unfused body. That is the same const-fn approach `grouping.rs` already uses.

Handed back. Whether the calling thread is the one-core worker. The canon says "one worker" and "the pool is the consumer's executor" (`the_pool_is_the_consumer_s_executor...`, ratified), but not which thread. "Run inline on the caller" is an option in the open question `what_the_engine_does_when_the_executor_reports_no_workers`.

Also owed. Op's "Measure first" means the one path must not fix the single-trunk convergence choice. Head-and-tail is not built (`tests/gate2_convergence.rs:181`).

Answer: (c).

What would show it wrong.
- A compile probe of the per-fiber const selection over a two-fiber carrier (one fiber a `RecordOp` chain, one not), under fat LTO. If the refused-bound steps fail, the design changes, and that is op's call.
- The ASM checklist at ncores=1 must show the same inner loop as today's `run` and `run_fused`, with no indirect call.

## 2. The worker panic hang

Arms.
- (a) The executor catches the unwind and reports a poisoned arrival.
- (b) Units must not panic, and the stall stays documented.
- (c) A timeout on the done-wait through `Clock`.
- (d), added and held: a no-unwind boundary on the worker's own frame, so the abort actually happens.

What governs.
- Op: "Abort stays, fix hang only".
- `execute_returns_nothing_and_a_failing_unit_aborts_because_abort_gives_better_llvm_codegen` (ratified; consolidation:2147-2157, :2448-2452): "Failure aborts, and on the no_os and os tiers panic is abort already". Its mechanism notes that op's later answer removes the founding `catch_unwind`.
- `there_is_no_std_platform_tier` (ratified).

The hang exists only because the build does not hold the canon's premise: `mock/Cargo.toml:83-86` sets no `panic = "abort"`, and test builds unwind.

Arm by arm.
- (a) is refused. It is `catch_unwind` under another name, and a poisoned arrival is a failure-policy redesign.
- (b) is today's state (engine `DESIGN.md.tmpl` panic paragraph; `src/thread/frame.rs:88-111`) and does not fix anything.
- (c) is refused. A timeout cannot tell a dead worker from a long frame, and long frames are the normal case (ratified `hilavitkutin_is_for_millions...`). Its outcome is a new failure policy, and it puts a clock read on the done-wait loop (`frame.rs:98-110`) on every frame.
- (d) is `no_std` and needs nothing from the executor. Either run the worker body in an `extern "C"` fn (nounwind, so an unwind there aborts), or put a guard on the worker's frame whose drop panics (a panic during cleanup aborts), forgotten on the normal path.
  - Under `panic = "abort"` it costs nothing.
  - Under unwind it costs one terminate pad, outside the morsel loop.
  - It also defeats an executor that catches panics, which would otherwise produce the same hang.

The brief's example of a guard that reports the arrival is the wrong half. An arrival turns the hang into a silent partial frame with a dead worker, and the next `frame_await` then waits on a thread that no longer exists.

Answer: (d), with the guard aborting.

What must fail first. A unit that panics on core 1 only, built with unwind and run through `run_parallel` in a child process, must die by SIGABRT within a time bound. Today the child hangs and the bound fails.

What would show it wrong. Op ruling that abort was never meant to cover unwind builds.

## 3. Aliasing across the park

Arms.
- (a) Plane relocation.
- (b) A raw pointer across the park, with the reference re-formed per frame.
- No new arm. My finding is that (b) collapses into (a).

What governs.
- `each_physical_core_runs_one_monomorphised_function...` (ratified; consolidation:1596-1613): each core program owns its ranges and "Phase sync points (stack AtomicUsize + spin loop)". The canon's shape is workers that hold their own program, not the scheduler.
- `the_gate_2_agent_call_deviations_are_blessed_or_rebuilt_only_on_evidence_put_to_the_maintainer` (ratified; A2:64-71) names "discipline-sound raw scheduler aliasing" and "PoolFrame inline + Pin". Neither is blessed or rebuilt by default.
- The round itself is the open question `whether_the_maintainer_blesses_the_plane_relocation_round_for_deviations_one_five_and_six_and_the_bless_of_deviation_four`.
- UB is never canon, so the hole closes either way. Op rules on the shape that closes it.

Why (b) is not narrower.
- The worker parks on `frame_await(&s.pool, ...)` (`worker.rs:102`), and `pool` is an inline field of the scheduler (`run_parallel.rs:312`). So it holds a reference into the scheduler across the park, not just a pointer.
- Under Stacked Borrows (Miri's default), the main thread's `&mut Scheduler` retag pops every tag above it, atomics included. That covers the worker's shared tags and any raw pointer derived from an earlier frame's `get_unchecked_mut` (`run_parallel.rs:139`).
- For (b) to be sound, the frame words and `worker_ctxs` (read at `worker.rs:75-76`) must move out of the scheduler's allocation, and the scheduler pointer must be republished per frame from outside it. That is the plane relocation with a smaller payload.

Wider than the tickets say. Every `run_parallel` call takes `self: Pin<&mut Self>`, so the caller's `as_mut()` is a fresh `&mut` retag on every frame. I predict, but have not run, that two plain consecutive `run_parallel` frames already violate Stacked Borrows. If so, FIXME(74e2b9918149) at `dirty.rs:62-65` and `:113-114` understates the problem.

Answer: (a), scoped to what Miri needs. Frame words, per-worker context and frame-read state go into a provider-allocated plane, and the `Scheduler` handle keeps a pointer to it. The bless of the round stays op's.

What Miri must run over. Default Stacked Borrows and `-Zmiri-tree-borrows`, over three drivers:
- (1) two consecutive `run_parallel` frames;
- (2) `run_parallel`, `replace_value`, `run_parallel`;
- (3) `run_parallel`, `replace_resource`, `run_parallel`. This is already `tests/replace_swap_plan_band.rs:252-330`, so it only needs a Miri gate.

What must fail first.
- Drivers (2) and (3) must fail under Stacked Borrows before the repair, and all three must pass under both models after it.
- If (1) fails, the hole is wider than the tickets say. If (1) passes, my prediction is wrong.
- If (2) and (3) pass on today's code, my mechanism is wrong.

## 4. Whether a terminal fan-in closes a phase

Arms. (a), (b) and (c) as listed, plus (d), added: none of the waist arms; the cause is trunk formation.

The canon gate fails on the framing.
- Evidence. Op (ratified `the_benches_..._no_call_is_made_on_them_yet`): "no calls should be made based on those yet".
  - The 1.08 is "a floor ... not a measure" of in-frame spread, per the header of `origin/work/api-revision:mock/benches/frame_wide/occupancy.txt`.
  - The memo that found it says its grouping account does not explain the timing (`202611131400_frame-wide-first-run.md:72`, 3.01 ms against 2.99 ms).
- Criterion. `a_waist_is_found_by_counting_live_paths...` (ratified; consolidation:1306-1317) counts live paths, a path being "a WU that has at least one unsatisfied downstream dependency", converging "before fanning out again". A terminal join has no downstream dependency and nothing fans out after it, so it is not a waist. `a_waist_is_a_narrow_bottleneck...` (ratified) makes waists the only phase boundaries.
- Therefore (b) rewrites a ratified algorithm for every consumer, and (c) invents a phase boundary that is not a waist. Both are refused.

What the canon prescribes inside the phase.
- `trunks_branches_and_bridges_are_found_within_each_phase_by_a_weighted_spanning_tree_decomposition` (ratified; consolidation:1130-1137): the longest weighted path is the trunk, a fan-out's lighter children are branches, and a fan-in is recorded as a bridge.
- `a_bridge_is_a_fan_in_node...`, `each_trunk_is_pinned_..._leftover_threads_take_convergence_branches_and_bridges` and `phases_overlap_through_progress_counters...` (all ratified).

What the code does instead.
- `compute_trunks` (`src/plan/grouping.rs:289-344`) builds a trunk as a union-find component over write conflict. The join glues ten units into one trunk (memo:68).
- That is not "a column-disjoint maximal path" (ratified `a_trunk_is_a_column_disjoint_maximal_path...`).
- There are no branches, no bridges and no progress counters (deviation ledger, item 8).

A second drift. arvo's `waist_detect` counts level width, not live paths (`arvo-graph/src/waist.rs:3-9`, `:96-107` at rev 1b2796e). hilavitkutin uses it as its waist detector (`grouping.rs:277`, `plan/steps/step3_waists.rs:8`).

Answer: (d). One phase, with the canon's decomposition built inside it. Under the ratified ≤5-fibers single-trunk row (consolidation:1394-1396), the parallelism comes from branches, bridges and convergence, and the convergence choice stays under "Measure first".

What would show it wrong. A plan-level assertion on the twelve-unit frame: the three pipelines are not in one trunk, and the join is a bridge. If a faithful implementation of consolidation:1130-1143 still gives one ten-unit trunk, it goes to op as a canon question.

## 5. The dirty skip and the plan band

What the code does.
- `run` gates windowed fibers on the dirty mask and runs accumulator and record-less fibers ungated (`run.rs:130`, `:249-253`).
- `run_fused` gates its one chain on dirty bit 0 (`run_fused.rs:98-103`).
- `run_parallel` never computes the mask (`worker.rs:340-388`; "no incremental skip" at `:309-311`), yet it clears `store_dirty` (`run_parallel.rs:410-414`).
- `run` and `run_parallel` consume the plan-dirty bits (`run.rs:122`, `run_parallel.rs:252`, `dirty.rs:133-143`). `run_fused` does neither (FIXME(b8637199eb8a), `run_fused.rs:77-86`).

What governs.
- `a_unit_whose_inputs_are_all_clean_skips_the_pass` (ratified; consolidation:1418-1431), with no carve-out per entry. `the_engine_is_one...` forbids one.
- `plan_affecting_is_an_open_marker_trait...` (ratified): "a swapped plan-affecting resource marks the plan for recompute".
- `the_plan_reruns_in_full_on_any_pipeline_structure_change...` (ratified).
- Swap spec S1 to S7 is provisional, so it is a claim, not governance.

Answers.
- Every walk gates on the dirty mask.
- The fused walk opens the plan band after a plan-affecting swap. The band precedes strategy choice, and a recompute can change which fibers fuse.
- The accumulator under the skip is handed back. The skip law rests on a clean unit's output being identical whether it runs or not. That is false for an accumulator the frame resets (`run.rs:105`, `:242-248`). Two shapes keep the law: no reset when the cone is clean, or always dirty. Which one is the open question `which_shape_the_accumulator_takes_is_left_to_the_api_revision`. What the one path owes either way is identical accumulator contents whether the unit runs or is skipped.

What would show it wrong. A differential over the sequence cold, clean, `replace_value`, `replace_resource`, clean, at ncores 1, 2 and 4 plus the fused strategy. It must assert identical columns, identical accumulators and the same set of executed units.
- The executed-set assertion must fail today at ncores above 1.
- A fused frame after `replace_resource` must fail today.

## Found and not asked

- `run_parallel` never calls `select_adapt_config` or folds the per-phase EMA. `run` does both (`run.rs:327-345`).
- `run_fused` windows by `Cfg::MORSEL_SIZE` (`run_fused.rs:110-137`), not by the plan's per-fiber window that `run` uses (`run.rs:259-260`).
- The zero-workers hang is separate and still open.
- FIXME(0ba7325f0216) (`dirty.rs:77-84`): the plan-dirty bit is keyed by store position and silently dropped at zero capacity.

I wrote no code and ran no Miri, bench or build. Every behavioural claim above is a check to run, not a result.
