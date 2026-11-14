# The Engine's One Path, and the Order Its Correctness Work Is Built In

Written 14 November 2026 on `work/hv-engine-plan`, first against the trunk `dev` at `e99af156`, then brought onto `dev` at `b4c70cd0`, which landed the canon registry. The api revision was read at `origin/work/api-revision` `d45093ea` (the branch has since moved to `e01a42d9`; the helper reads its tip). Research only: no source, no design text and no registry row is changed by it.

hilavitkutin ships three ways to run a frame, `run`, `run_parallel` and `run_fused`, and they do not compute the same thing. The stack audit found that, the amendment of 6 June says there is one engine, and the founding spec has a per-core program and a strategy per phase. This note says what the one path is, where that is settled and by what, and then lists the build's jobs in the sequence they are done, each with its tickets, its crates, the check that proves it and what it frees.

No human sat in this work. Where the record and op's words settle a point, the point is cited and closed. Where they do not, two opus seats answered it one after the other, the second with no sight of the first, and what both reached is a provisional stamp with a question ticket to op. He may overturn any of it.

The build this plans starts after the api revision has landed on `dev`. Every `file:line` below is of `dev` at `b4c70cd0` unless it says the api revision, and the helper re-reads each one on the tree it starts from. Between `e99af156` and `b4c70cd0` only `scheduler/dirty.rs` moved under the lines cited (one line, so its anchors here are one later than in the seats' reports), and the FIXME tags were renamed to the tickets they cite (`FIXME(74e2b9918149)` is now `FIXME(placeholders_tagged_74e2b9918149)`, and so on).

## What the canon is here

Since `b4c70cd0`, `mockspace.toml` on `dev` declares `canon_paths = ["mock/registry/*.toml", "mock/registry/tenet/*.toml"]` (`mockspace.toml:14`): the canon is the typed tenet rows, asked with `cargo mock canon find|show|related` from the repository root. Rows are cited below by slug with their standing, and every row cited was looked up again on `b4c70cd0`; none moved standing.

A ratified row governs. Most were filled from the written history on op's word ("fill them as ratified by default from written history", `mock/research/202611130850_the-record-read-whole/OP-ANSWERS-18.md`). A provisional row binds as the best current statement and is never cited as settled, and a stamp on one is as good as op's word for now and never a ratification. A held row binds nothing; it is listed for op in `mock/research/202611130850_the-record-read-whole/ROWS-FOR-OP.md`. The standings are defined in the description of the `standing` field (`mockspace.toml:478`).

The rows' passages come from the founding spec (`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`, "the spec" below), the amendment (`mock/research/202606061000_canon-amendment-unified-engine.md`, canon by its own header `:3` on op's ruling of 6 June) and the engine design (`mock/crates/hilavitkutin/DESIGN.md.tmpl`), which are cited by line where a row's words are not enough.

When this note was first written the registry was on the unlanded branch `origin/work/canon-registry`, and the two seats of the first five points read it there.

Everything else is agent work: the stack audit, the later rounds, the api revision's locked changelist, the tickets and the source.

## What the three documents say, and where they part

The amendment says there is no separate single-core path: "The same primitives and the same code paths run at 1 core, 2 cores, or 7 cores" (`:7`), sequential is a strategy a phase may take and no engine of its own (`:9`), the compiled per-core program is the dispatch (`:18`), and a walk of the unit list as the one-core dispatch shape is forbidden (`:25`). The registry carries it as `the_engine_is_one_and_the_core_count_is_configuration`, ratified, with "a separate single-core engine or dispatch path" under what it rejects.

The spec has each physical core run one monomorphised function that holds its phases, its record ranges, its units per fiber, its morsel bounds and its sync points (`:1596-1613`), with the strategy chosen per phase (`:1916-1956`, the range the amendment cites for it) and fusing decided per fiber by the code generator (`:83`, row `the_flattener_emits_one_monomorphised_function_per_fiber_in_the_rust_pipe_pattern`, ratified). It gives a single-trunk phase to two threads from opposite ends (`:770-771`) and resolves two writers of one store by hint priority, then most dependents, then a fixed fallback (`:591-593`).

The stack audit reads the code against both. Its finding on the entries is that they compute differently, and the count of entry functions is not the defect (`mock/research/202611131215_stack-audit/DESIGNS.md:65-71`). It then goes further than either document on one point: it argues the cap of two walkers on a single-trunk phase rests on a misread measurement and proposes a split into as many record ranges as cores behind a commutativity gate (`DESIGNS.md:93-109`).

So the three agree that the engine is one. They part on two things. The amendment and the spec say one set of code paths, where the api revision's engine design keeps three entries and holds them to one meaning from outside (`origin/work/api-revision:mock/crates/hilavitkutin/DESIGN.md.tmpl:1627-1650`, "three ways through one frame"). And the spec says head and tail for a single-trunk phase, where the audit says as many ranges as cores.

## What the code does today

These are facts about the source and say nothing of what is right.

`run` walks on the calling thread with no pool. It asks `dirty_units` (`scheduler/run.rs:130`) and narrows a windowed fiber's members by the mask, leaving accumulator fibers ungated (`run.rs:249-253`). It consumes the plan-dirty bits and opens on the plan band (`run.rs:122`, `:179-191`), folds the per-phase EMA and calls `select_adapt_config` (`run.rs:339`, `:345`).

`run_parallel` publishes a frame and waits (`scheduler/run_parallel.rs:313-314`). Its workers never ask `dirty_units`: the record-less dispatch passes an all-ones mask under a comment that says so (`scheduler/worker.rs:307-328`) and the windowed walk passes the fiber's member mask alone (`worker.rs:340-388`). It consumes the plan-dirty bits (`run_parallel.rs:252`), folds the pass EMA (`run_parallel.rs:423`) and never calls `select_adapt_config`. A carrier with an accumulator leaves the phase loop altogether: each core walks the whole carrier once over its own slice of the records (`worker.rs:107-124`, `:160-266`).

`run_fused` folds the whole carrier into one fused body the consumer asked for by calling it (`scheduler/run_fused.rs:70`), because choosing it automatically is not expressible with `min_specialization` (`run_fused.rs:54-59`). It asks `dirty_units` (`:103`), has no plan band and leaves the plan-dirty bits unconsumed under `FIXME(placeholders_tagged_b8637199eb8a)` (`:77-86`), and windows by the run configuration's morsel size where `run` takes the plan's window per fiber (`run.rs:260`).

The api revision changes this in four places that the build inherits. `dirty_units` takes the plan-band flag and adds the leading bands' writes to its seed; every entry ends on `bring_in_changes`, which leaves the next frame's seed where the trunk cleared it; a meta band on the phase-loop path is run by core 0 alone over the whole record range; and step 1 of the plan orders units of different lifecycle rank by rank and carries `FIXME(hilavitkutin_orders_two_writers_of_one_store_by_registration)` on the write-write edge. Its workers still ask no dirty mask, which is ticket 157.

## The one path

Five points were open at the first writing, and a sixth is below. On each of the five, both seats reached the same answer without seeing the other's. Their reports are `SEAT-ONE.md` and `SEAT-TWO.md` beside this file and their brief is `SEAT-BRIEF.md`. The sampled citations of the first report were opened and hold (`mock/Cargo.toml` sets no `panic`, `worker.rs:102` parks on `&s.pool`, `plan/grouping.rs:289-298` builds trunks by union-find, the spec `:1130-1137`, `:1306-1317`, `:1394-1396`). The rest of what the seats cite is theirs and is marked so where this plan leans on it.

### One frame body

The engine has one frame body, and every entry reaches it. The amendment's "same code paths" and the ratified row refuse three walks kept equal by a comparison, which is what the api revision's design text describes. The reference runner of `hilavitkutin-conformance` stays as the oracle the body is tested against. It is a test, and it does not replace the one body.

That there is one set of code paths is settled by the ratified row and the amendment, and is no stamp of this work. The api revision's "three ways through one frame" is agent text that departs from a ratified row.

What both seats agree on is one generic frame body with fusing chosen inside it per fiber, and no fused entry a consumer picks (`SEAT-ONE.md:40`, `SEAT-TWO.md:54`). Which existing code becomes that body is this plan's own reading, and neither seat named it. The plan's reading: the body is the worker's, the frame loop of `worker_main` with its two paths, the phase loop with the waist barrier and the unit-outer walk of an accumulator carrier. It is the nearest thing the source has to the spec's per-core program (`:1596-1613`, the amendment `:18`): each worker walks the carrier and fires the monomorphised body of the trunks it owns. It is not claimed to be that program in full. The provisional row `the_unified_mechanism_is_the_runtime_plan_to_compile_time_per_core_program_bridge_and_fiber_shape_typestate_is_its_unproven_realisation` says the bridge to it is unproven, and that work comes after this track.

`run`'s own walk is retired into the body, with one arriver at one core as the amendment has it (`:7`). `run` takes no pool, so retiring it means the calling thread runs core 0's body, and that is the question below, which neither seat settled. Until op answers it `run` keeps its walk and is held to the body by the differential of job 1.

Fusing becomes the plan's choice per fiber inside that body, as the spec's `:83` and the flattener row have it, so `run_fused` stops being an entry a consumer picks. That fusing is a decision of the strategy and no kind of frame is settled: `a_fiber_is_the_unit_of_column_co_location_and_morsel_windowing_and_fusion_is_a_strategy_decision` (ratified, the spec `:83`, `:944-948`) says whether a fiber's ops fuse into one loop body "is a decision of the execution strategy and not part of what a fiber is". The first seat names an untried encoding: a per-fiber trait that every fiber implements, with an associated constant that selects the fused or the plain body, the way `plan/grouping.rs` already selects by constant (`SEAT-ONE.md:41`). The second names no encoding and asks for a sketch of the one generic body (`SEAT-TWO.md:56`). Nobody has compiled either. The job below starts with the sketch, and a toolchain wall there goes to op as a question. Until he answers, the fused entry stays as it is, and no second entry is added for the wall.

The provisional stamp is what both seats reached: one frame body, fusing per fiber inside it, the fused entry retired. The constant encoding is one seat's and the choice of the worker's body is this plan's; both are readings, held to the sketch. It departs from the api revision's locked design text, which lands first, so the build's first design round rewrites the section "What the entries are held to" before any source moves.

Not settled by either seat: whether the thread that calls `run_parallel` runs core 0's body itself. The canon says one worker at one core and that the pool is the consumer's executor, and does not say which thread. The same answer decides the zero-workers case the engine design leaves open (api revision `DESIGN.md.tmpl:1675-1687`), which the registry holds as the open question `what_the_engine_does_when_the_executor_reports_no_workers` with "run inline on the caller" among its options. The one row that says which thread drives a parallel frame, `the_main_thread_drives_each_phase_of_a_parallel_frame_and_is_itself_the_barrier_between_phases`, is held and binds nothing. The amendment's "the pool has one worker" (`:7`) reads against running inline, and nothing cited here licenses it. The body is one function either way, so the mask, the worker paths and the fused selection do not wait on it. The retirement of `run`'s walk and ticket 16 do. It is a question to op.

### A panic aborts, on every build

Op: "Abort stays, fix hang only" (clause-dev `.data/op-responses/202611131716_op-on-okay-so-while-we-wait-any-commentary-or.md:121-127`; the options were to redesign now, to redesign after the api revision, or this). The ratified row `execute_returns_nothing_and_a_failing_unit_aborts_because_abort_gives_better_llvm_codegen` (the spec `:2448-2451`) says a panic is an abort already on the no_os and os tiers, and that on a std tier "we prefer abort". The row `there_is_no_std_platform_tier` (ratified) leaves the first two.

The hang exists because that premise does not hold in a build that unwinds: `mock/Cargo.toml` sets no `panic = "abort"`, test builds unwind, the dead worker never arrives, and `frame_await_done` loops with no way out (`thread/frame.rs:88-111`).

Both seats refused the three options of the open question ticket. An executor that catches the unwind and reports a poisoned arrival is `catch_unwind` and a new failure policy. Requiring units never to panic leaves the hang. A timeout through the clock cannot tell a dead worker from a long frame, and long frames are what the engine is for ("millions of simulation and crunching work data going constantly", `mock/research/202611130850_the-record-read-whole/OP-ANSWERS-20.md`).

Both chose the same fourth: the frame body runs behind a boundary that cannot unwind, so a panic in a unit's body aborts the process whatever the build's panic strategy. Two mechanisms are named, an `extern "C"` worker entry and a guard whose drop aborts. Neither is compiled against the generic `worker_main` yet. A guard that reports the arrival is the wrong one: the other workers stay parked at the waist barrier and the frame returns over partial output (`thread/barrier.rs:65-70`).

The api revision's design sentence "Catching it is the executor's" (`DESIGN.md.tmpl:687` there) goes with this. Provisional stamp.

### The plane relocation closes the aliasing hole

A worker holds `&Scheduler` across every park (`worker.rs:77-84`) and parks on frame words that are a field of the scheduler (`worker.rs:102`), while `replace_resource` and `replace_value` take `&mut self` between frames (`scheduler/dirty.rs:49`, `:97`, with `FIXME(placeholders_tagged_74e2b9918149)` at `:63-66` and `:114-115`) and every `run_parallel` call takes `Pin<&mut Self>` (`run_parallel.rs:103`).

Both seats found that the narrower repair, a raw pointer kept across the park with the reference formed per frame, cannot work: the pointer the worker parks through is derived from the scheduler, and the next exclusive borrow of the scheduler invalidates it under Stacked Borrows whether it is held as a reference or raw. Both chose the relocation the deviation ledger and the deviation 5 audit name: the frame words, the per-worker context and what a worker reads move to a plane the provider allocates and no exclusive borrow of the scheduler covers.

The rows that carry the relocation are held and bind nothing: `deviation_five_has_a_confirmed_aliasing_hole_so_the_plane_relocation_is_soundness_required_and_resolves_deviations_one_five_and_six_in_one_round` and `deviations_one_and_six_are_answered_by_the_whole_plane_arena_route_and_the_proposed_ruling_is_the_relocation_round` (both held). Beside them `the_pool_frame_is_the_engine_internal_data_plane_on_per_pool_cache_lines_and_the_phase_barrier_is_one_atomic_counter` (provisional, stamped) names the pool frame the engine's data plane and says nothing of where it is allocated. The registry holds the round as an open question to op (`whether_the_maintainer_blesses_the_plane_relocation_round_for_deviations_one_five_and_six_and_the_bless_of_deviation_four`), with two options: bless the proposed rulings, or "rebuild the deviations to the canon shape". The seats were asked about the relocation and the narrower repair and not about the rebuild, which is the spec's per-core program holding its own ranges and sync points and is the larger work that follows this track. Undefined behaviour is no design, so the hole closes either way.

What may be built is bounded by a ratified row this note did not cite at its first writing: `the_gate_2_agent_call_deviations_are_blessed_or_rebuilt_only_on_evidence_put_to_the_maintainer` (ratified, `mock/research/pre_seed/202607193200_canon-amendment-a2-precedence-and-rulings.md:64-71`) says that for each of these deviations "the canon shape is built or sketched, benched against the shipped shape where the ledger names a bench trigger, and presented to the maintainer with evidence for a ruling to bless it or rebuild it. None is blessed by default and none is rebuilt by default." So the seats' agreement stamps the relocation only as the arm put to op, and does not license building it. Job 4 builds the evidence, the Miri drivers and a sketch of each arm, and the relocation is built after op rules on `bless_the_scheduler_plane_relocation_that_closes_the_worker`, whose note names the rebuild beside its options.

The first seat predicts that two plain `run_parallel` frames with no swap between them already violate Stacked Borrows, since each call retags the scheduler. Nobody has run it. The Miri job below runs it first, because it says how wide the hole is.

### A terminal fan-in closes no phase, and the trunks are formed wrong

The open question ticket `should_a_terminal_fan_in_close_a_phase` (ticket 135) offers three ways to make the last join of a frame a phase boundary. Both seats refused the question as asked. The ratified waist row (`a_waist_is_found_by_counting_live_paths_per_topological_level_and_a_local_minimum_is_a_phase_boundary`, the spec `:1306-1317`) counts live paths, a path being a unit with an unsatisfied dependency downstream, and a terminal join has none, so it is no waist. The spec's own example ends its last phase on leaves with no waist after them (`:1323`). And op ruled that no call is made on the benches that raised it (`mock/research/202611130850_the-record-read-whole/OP-ANSWERS-21.md`).

Both found the measured 1.08 cores of four to come from somewhere else. The engine forms a trunk as a write-conflict component by union-find (`plan/grouping.rs:289-298`), so the join pulls the three pipelines into one trunk of ten units. The spec decomposes a phase into a trunk, its branches and its bridges, and runs a fan-in as a bridge (`:1130-1137`; rows `trunks_branches_and_bridges_are_found_within_each_phase_by_a_weighted_spanning_tree_decomposition` and `a_bridge_is_a_fan_in_node_that_runs_after_its_parent_trunks_reach_the_required_record_range`, both ratified).

The stamp: the frame stays one phase, the waist criterion is left alone, and trunk formation is a ticket of its own. The second seat's caveat stands with it: the spec makes a phase of five fibers or fewer one trunk (`:1394-1396`), and whether this frame falls under that depends on the fibers the plan forms, which nobody dumped. The first step of that ticket is the plan dump.

Ticket 128 (the twelve-unit frame spread over one core) is the measurement this question came from. It stays open and folds into the trunk-formation ticket, `hilavitkutin_forms_a_trunk_as_a_write_conflict_component_whe`, whose plan dump is its first step. Ticket 135 is answered by the stamp once op makes it formal.

Both seats also found that `arvo_graph::waist_detect` counts the width of a level where the ratified row counts live paths. The two agree on this frame and differ in general. Ticketed.

### One dirty mask, on every walk

A unit whose inputs are all clean skips the pass (the spec `:1418-1424`, row `a_unit_whose_inputs_are_all_clean_skips_the_pass`, ratified), and the one-engine row allows no entry its own rule. So the body gates every walk on one mask, computed once between frames on the calling thread and published with the frame the way the plan-band flag already is (`dirty.rs:134-144`, `worker.rs:135`).

The frame after a plan-affecting swap opens on the plan band whatever its fibers' strategies, fused ones included, since the recompute is what decides which fibers fuse (swap spec S2, `mock/research/202607200200_replaceable-swap-semantics-spec.md`, landed provisional on op's "Land as provisional", clause-dev `.data/op-responses/202611131305_op-on-yeah-if-the-double-buffering-is-in-fact.md:152-168`).

What an accumulator fiber does under the mask neither seat settled. A skipped unit must leave what it would have written, and an accumulator is reset every frame, so either its writer always runs or a skipped writer's appends are kept across the reset. The code does the first (`run.rs:242-253`) and a provisional row says the first (`a_mixed_carrier_keeps_incremental_skip_per_fiber_and_accumulator_fibers_always_run_the_whole_range`), where the ratified skip row carves out nothing for accumulators. Both seats handed it back (`SEAT-ONE.md:160`, `SEAT-TWO.md:121-126`), so it is no stamp and no call of this plan: it is asked of op as `does_a_skipped_accumulator_writer_run_anyway_in_hilavitkutin`. Until he answers the build changes nothing here, the one body keeping what the code does today, and the differential asserts the same accumulator contents whether a unit ran or rested, which either answer must keep.

### Commutativity gates the accumulator tiling

A sixth point was left by the first writing as this plan's own reading. It went to two opus seats one after the other, the second blind to the first, on a brief of its own (`SEAT-BRIEF-COMMUTATIVITY.md`; reports `SEAT-THREE.md` and `SEAT-FIVE.md`). A third read, on sonnet and blind to both, is kept beside them as `SEAT-FOUR.md` and carries no weight of its own.

Both opus seats found the canon settles it, and no stamp is needed. `ordering_is_guaranteed_by_construction` (ratified, the spec `:604-607`) says "a non-commutative fiber runs on a single core", and `a_fiber_is_commutative_only_if_all_its_units_are_and_resource_accumulation_uses_per_thread_accumulators_merged_after` (ratified, `:1171-1176`) gives multi-core morsel dispatch to commutative fibers only. The engine design says the same of the accumulator (`mock/crates/hilavitkutin/DESIGN.md.tmpl:646-648`, "Non-commutative accumulation keeps the sequential single-core fallback"). The provisional row `each_core_appends_into_its_own_region_of_an_accumulator_and_the_main_thread_merges_the_regions_in_order` describes the regioning and is silent on the gate, so it is read under the ratified two.

The gate is per fiber, on every unit of it and not only the one that appends, at every worker count. Since the tiling walks the whole carrier on each core (`scheduler/worker.rs:107-124`, the record range taken from `accum_slice` at `:191` and passed at `:232`, `:247`), a carrier is tiled only when every fiber it holds is commutative. Today nothing reads the flag: `COMMUTATIVE` defaults to false (`hilavitkutin-api/src/work_unit.rs:84`), is copied at `plan/mod.rs:423`, and the tiling is chosen by `carrier_unit_outer` on `morsel_local` alone (`scheduler/run_parallel.rs:467-478`).

Both seats refuted the in-order merge as making the split invisible. Each core's live length starts at zero (`resource/bindings/per_core.rs:135`) and a unit can read it (`dispatch/engine_ctx/ctx_writers.rs:172-174`), so a unit that appends its accumulator's length gives a different buffer under the tiling. The seats name it as the case that must fail. Neither ran it.

What the seats did not settle is timing, and they read it differently: whether op's "Measure first" on single-trunk convergence ("The code stays as it is meanwhile", registry question `which_convergence_a_single_trunk_phase_uses`) holds the gate back. One reads his words as not reaching the accumulator path, the other holds the repair until he says. It is asked of him as `does_measure_first_hold_the_accumulator_tiling_s_commutativi`. Job 9 lands the failing test either way and the gate when he answers.

What the flag promises is open as well. The ratified rows make it a declaration on a unit, conjoined over a fiber, and speak of resource accumulation; the api's doc says "the WU's writes commute across record order" (`hilavitkutin-api/src/work_unit.rs:80`), which read literally lets no record-ordered appender declare it. Both opus seats propose segment invariance, marked as their reading and not the canon's (`SEAT-THREE.md:30`, `SEAT-FIVE.md`): a unit's effect over a record range equals the in-order concatenation, or for a fold the declared combine, of its effects over any contiguous split of it. Under that reading two shapes may not declare it, since the tiling changes them: a unit that appends once per `execute` (one value under `run`, which gives an accumulator fiber one morsel of the whole range, `run.rs:276-283`; one per core when tiled), and two appenders into one accumulator, whose appends interleave by core (`SEAT-FOUR.md`, a hypothesis). It is asked of op as `what_does_commutative_promise_for_an_accumulator_append_in_h`, and job 9 rewrites the doc at `work_unit.rs:80` to his answer.

## What is already settled, and what is held

The order of two writers of one store is op's: "Hints, dependents, registration" (`mock/research/202611130850_the-record-read-whole/OP-ANSWERS-13.md:25-31`, item 3; the options were that, or hints then registration; registry question `whether_the_most_dependents_step_stays_between_hints_and_registration`, answered). It is a job below.

The swap landed provisional, and "ratification waits until the two gaps are fixed" (clause-dev `.data/op-responses/202611131305_op-on-yeah-if-the-double-buffering-is-in-fact.md:152-168`), the gaps being tickets 106 and 107. Both are jobs below.

Which convergence a single-trunk phase uses is held, and no job touches it. Op answered "Measure first" (the options were head and tail with the deletion finished, as many ranges as cores with head and tail retired, both chosen at plan time, or measure first), recorded as "the question returns with the numbers. The code stays as it is meanwhile" (registry question `which_convergence_a_single_trunk_phase_uses`). The bench exists (`mock/benches/single-trunk-convergence/FINDINGS.md`) and its own header says no design call rests on it and the choice stays open until the engine has its headline features, on his word that the benches so far are a baseline (`mock/research/202611130850_the-record-read-whole/OP-ANSWERS-21.md`). So ticket 105 (build head and tail) waits, and so does any removal of the record tiling. The one frame body is written so that the record range a core takes in a single-trunk phase comes from one place, which either answer then fills.

Ticket 116 is stale as written. It cites a split of a single-trunk phase into one record range per core at `worker.rs:323-348`. That branch is gone from `run_core_phase` on `dev`. The tiling that remains is the accumulator carrier's (`worker.rs:107-124`, `resource/bindings/per_core.rs:110-117`), which the engine design asks for on the accumulator (`DESIGN.md.tmpl:604-607` on the api revision). What is still ungated is commutativity: `COMMUTATIVE` is copied into the plan (`plan/project.rs:162`, `plan/mod.rs:423`) and read by nothing, where the spec keeps non-commutative resource accumulation off head and tail (`:1171-1176`). That is a job below, restated.

## The jobs, in sequence

Every job is in `mock/crates/hilavitkutin` unless it names another crate. The sequence puts the instrument first, then soundness on the walk that survives, then the one body, then what the body frees. A job is a design round and lands when its round closes, with three folded: job 6 is the test of job 5 and lands in its round, and jobs 11 and 13 share one round.

Three rules hold for every job's checks. A case shown red is committed as a negative test that asserts the check rejects the wrong construction, never shown once and dropped (`a-hand-check-becomes-a-test-every-time`). The worker counts are 1, 2, 3, 4 and `MAX_CORES` wherever this note says 1, 2 and 4, since trunk ownership is round-robin (`dispatch/trunk_dispatch.rs:246-253`) and `accum_slice` leaves a remainder at an odd count. A Miri driver is marked `#[cfg_attr(not(miri), ignore = "<ticket>: runs under miri")]`, since natively it passes and nothing in the repository runs Miri; until a Miri tool exists under `mock/tools/` (ticket `hilavitkutin_has_no_tool_that_runs_miri_so_a_miri_driver_is`) the job runs it by hand and records each red driver as a `known_red` row.

### 1. The differential, before anything it will judge

Tickets 49 and 157 (its test half). Crates: `hilavitkutin` tests, `hilavitkutin-frozen-authors` tests, `hilavitkutin-conformance` as it lands with the api revision.

The seven-step history of `tests/incremental_skip.rs:481-519` (a skipping scheduler against an every-store-dirty one, equal column bytes at every frame) is driven through `run_parallel` at 1, 2 and 4 workers, with the set of executed units compared as well as the bytes. The history gains a `replace_value`, a `replace_resource` and a clean frame after each.

The case that must fail: the executed-set comparison fails at 1, 2 and 4 workers on the tree as it stands, since the workers ask no mask at any count and `run_parallel` has no walk of its own for one worker. It is committed catalogued and ignored under ticket 157 and un-ignored by job 5.

The byte comparison needs a control of its own, because while `run_parallel` skips nothing its bytes equal the every-store-dirty run's for a trivial reason. The twin of `incremental_skip.rs:532` (an input changed without a mark makes the two runs differ) cannot serve yet: with no skip, the unmarked change reaches both runs and they agree, so that twin is committed ignored under ticket 157 as well and turns green with job 5. The control that works today is a broken runner: a negative test in which the comparison is run against a worker count whose columns were altered by one record after the frame, asserting the comparison reports the difference, the way `hilavitkutin-frozen-authors` already catches a wrong runner (`a_wrong_runner_is_caught_on_the_authors`, `tests/authors.rs:194` on the api revision).

Frees: every later job is judged by it.

### 2. Sound test providers and an `unsafe` provider contract

Tickets 26 and 48. Crates: `hilavitkutin` tests, `hilavitkutin-persistence` tests, `hilavitkutin-api`. This job touches `hilavitkutin-api/src/platform.rs` (`:28`, `MemoryProviderApi` becomes an `unsafe trait`), which is the api lane's file until the api revision has landed. It does not start before then.

Thirty-two files under `mock/crates` still define a provider whose buffer is an inline array moved into `build` (the grep of ticket 26 on `e99af156`; `tests/stable_provider/mod.rs` is the sound one to move them to).

The cases that must fail. For 26: one of those tests under Miri before the move, on the moved-from base; it passes after. For 48: a compile-fail test under `tests/ui/` in which a provider is implemented with a plain `impl MemoryProviderApi`, refused once the trait is `unsafe` and accepted today.

Frees: Miri over anything, so jobs 3 and 4. The stable provider of the tests leaks one `Vec` (`tests/stable_provider/mod.rs:30`), so the Miri runs of jobs 2 and 4 pass `-Zmiri-ignore-leaks`, or a leak report hides the borrow result.

### 3. A panic in a unit aborts

Ticket 144, with the question ticket on the stall answered by the stamp above. Crates: `hilavitkutin` (`scheduler/worker.rs`, `thread/`), and the design text of the panic paragraph.

A sketch first, committed under `mock/research/sketches/`: the no-unwind boundary on the generic `worker_main`, both mechanisms, built with unwinding. Then the boundary, on the body wherever it runs.

The check: a test that runs a child process with two workers and two phases, where a unit panics on core 1 in the first phase, and asserts the child died by abort within a bound. The case that must fail: on today's tree the child hangs and the bound trips.

Frees: nothing waits on it, and it is small. It goes early because every later parallel test can hang on it.

### 4. The aliasing hole, measured and put to op

Ticket 93, deviations 1, 5 and 6 of the ledger. Crates: `hilavitkutin` tests and `mock/research/sketches/`; after op's ruling, `hilavitkutin` (`scheduler/`, `thread/`) and `hilavitkutin-api` if `PoolFrame`'s placement moves (it is in `platform.rs:267`; if so the job touches that file too).

The job has two halves, because the ratified evidence-then-bless row above governs these deviations. The first half is the evidence and builds no repair: the Miri drivers, then a sketch of each arm, the relocation to a provider-allocated plane and the canon shape, the spec's per-core program holding its own ranges and sync points, each run under the drivers. That evidence goes to op on `bless_the_scheduler_plane_relocation_that_closes_the_worker`. The second half builds what he rules, and does not start before.

The Miri drivers come first and are committed red: two consecutive `run_parallel` frames; a frame, `replace_value`, a frame; a frame, `replace_resource`, a frame (the driver of `tests/replace_swap_plan_band.rs`, by the seats' reading at `:252-330`). Each runs under Stacked Borrows and under Tree Borrows, through a test provider and not raw `mmap`.

The cases that must fail: the two swap drivers under Stacked Borrows on today's tree. The first driver's result is recorded either way, since it says whether a plain second frame is already a violation. A sketch of the narrower repair runs under the same drivers: if it passes, both seats' argument against it is wrong, and that goes to op with the rest.

The test the deviation 5 audit cites, `parallel_swap_pattern_is_aliasing_clean`, is on neither `dev` nor the api revision. It exists on `origin/recover/unlanded-rounds-and-benches` alone (`tests/replace_swap.rs:278-297` there), ignored, with a body of comments and no assertion, so it cannot fail. These drivers replace it, and if that stub reaches the trunk before this job the job deletes it.

Frees: op's ruling on the deviations, and after it a sound base for the one body. The swap's ratification does not wait on this job; op tied it to the two gaps alone, jobs 6 and 7.

### 5. The one frame body

Tickets 115 and 157, and the residue of 36. Crates: `hilavitkutin` (`scheduler/`, `dispatch/`), `hilavitkutin-api` design text, `hilavitkutin-frozen-authors` tests. If the dirty mask is published on `PoolFrame`, the job touches `hilavitkutin-api/src/platform.rs` (`:267`) and is marked so; publishing it on the engine's own per-frame words keeps it out.

In order inside the job. The design round rewrites "What the entries are held to". The dirty mask is computed between frames and published, and both worker paths gate on it, which turns job 1's catalogued test green. `select_adapt_config` and the per-phase EMA move into the body so that every count folds them. `run`'s walk is retired into the body once op has answered the caller question, and not before; until then it stays and the differential holds it to the body. Then the fused sketch: per-fiber selection by constant, over a carrier of one fusible fiber and one that is not. If it compiles and the assembly checklist shows the fused loop unchanged, `run_fused` is retired into the body. If it does not, the entry stays, the wall is written down with the sketch, and it goes to op.

Ticket 36 is closed on the api revision by a compile-time refusal (`tests/ui/appending_unit_fused_fail.rs` there, `tests/fused_append.rs` deleted). The job checks that refusal survives the per-fiber selection, where an appending fiber is simply not a fusible one.

The checks: job 1's differential green at 1, 2 and 4 workers with equal executed sets; the comparison against the reference runner in `hilavitkutin-frozen-authors` green for every count; the assembly checklist at one core showing the same inner loop as `run` has today, with no indirect call. The case that must fail: the executed-set test of job 1, red until the mask reaches the workers.

Frees: jobs 6 to 9, the adaptation work and fusion after this track, and kaski's engine work, which needs one meaning of a frame.

### 6. The fused walk and the plan band

Ticket 107. Crates: `hilavitkutin`.

With fusing a strategy inside the body this is mostly done by job 5. What is left is its test: a fusible fiber, a `replace_resource` of a plan-affecting value, and the next frame opens on the plan band. If job 5 kept the fused entry, the entry consumes the bits and opens the band.

The case that must fail: that test on today's `run_fused` (`run_fused.rs:77-86`). It lands in job 5's round.

Frees: with job 7, the ratification of S1 to S7, which is op's.

### 7. The plan-dirty bit keyed by the plan-affecting id

Ticket 106. Crates: `hilavitkutin` (`scheduler/dirty.rs:78-85`, `FIXME(placeholders_tagged_0ba7325f0216)`), and the api's `PlanAffecting` if the id is declared there.

The case that must fail. The clamp cannot be reached through `build` today: store positions are below 64 and the plan-affecting capacity is 256 (`plan/dims.rs:81-82`), so the position never clamps and the capacity is never zero. The defect is the key. Swap spec S2 keys the bit by the plan-affecting id, and the source keys it by store position (`dirty.rs:78-85`). There is no plan-affecting id to key by: `PlanAffecting` is an empty marker (`hilavitkutin-api/src/run_cfg.rs:48`), and the bits are private to the scheduler and folded by OR (`dirty.rs:134-144`), so no outside test can see which bit was set. The first step is an in-crate unit test that names the id the spec keys by, two plan-affecting resources whose store positions and ids differ in sequence, one replaced, and the bit asserted at its id and no other. It does not compile today, for want of the id, and that is the finding the job starts from (the design round declares the id). Once it compiles it is the case, red while the bit sits at the store position. The clamp and the zero capacity are covered by a unit test of the indexing function over a small capacity, which is where they can be reached.

Frees: with job 6, the ratification of S1 to S7, which is op's.

### 8. Two writers ordered by hints, dependents, registration

Ticket 72. Crates: `hilavitkutin` (`plan/steps/step1_dag.rs`, the `FIXME` the api revision leaves there), engine design `:43-44`.

The case that must fail: two writers of one store, the later-registered one with the higher hint, asserted to run first; and two with equal hints, where the later-registered one has more dependents downstream, asserted to run first. The registration is pinned that way in both, since with the favoured writer registered first the test passes on the index rule. Both fail today, where the lower input index wins (`step1_dag.rs:45-49`). The api revision's engine design states the three steps (`DESIGN.md.tmpl:52-55` there), but its step 1 makes registration the dispatch sequence and adds write-write edges only from the lower index to the higher (`plan/steps/step1_dag.rs:78-94` there), so a later-registered writer running first is a back edge in the carrier's walk. The job is a design round on that walk before its source.

Independent of jobs 3 to 7, so it may be done whenever the helper waits on a round. Frees: op's ordering rule in force, and the removal of `FIXME(hilavitkutin_orders_two_writers_of_one_store_by_registration)`.

### 9. Accumulator regions

Tickets 58 and 59, and 116 restated. Crates: `hilavitkutin` (`resource/bindings/per_core.rs`, `scheduler/worker.rs`).

Ticket 59 reads as done on `dev`: `tests/leading_band_appends.rs` (landed with `c42d7074`) asserts the leading bands' appends stay in front at 1, 2, 4 and `MAX_CORES` workers, and `rebase_accums` starts each region after them (`per_core.rs:121-126`). The job runs that test and closes the ticket on it, or reopens it with the failure. The case that must fail, committed as a negative test: a rebase that starts a region at its slice's own offset, declared in the test module, under which the test's assertion is shown to reject the lost leading records.

Ticket 58: a core's region is capped at its record slice, so a skew that fits the reservation is refused at more than one worker. The check is the ignored test `a_skew_that_fits_the_reservation_is_accepted_at_every_worker_count` (`tests/overflow_is_a_value.rs:377`) un-ignored and green, and it is the case that fails today.

Ticket 116, restated: the tiling reads commutativity, as the canon has it (the section above). The cases that must fail, committed first, each at `FixedWorkers` of 2, 4 and `MAX_CORES` and never the host's count: a carrier whose unit leaves `COMMUTATIVE` at its default and appends its accumulator's `len()` per kept record, asserted byte-equal to `run` and run by one thread; and a carrier whose units all declare it, asserted still tiled. The first fails today. The existing `run_parallel_accumulator_matches_single_core` (`tests/gate2_accumulator.rs:182-221`) cannot see the point, since its one appender depends on its own record alone and its executor takes the host's worker count. The gate lands when op answers `does_measure_first_hold_the_accumulator_tiling_s_commutativi`; until then the first case is committed ignored under that ticket. Two more cases are committed with them, ignored under `what_does_commutative_promise_for_an_accumulator_append_in_h` until he answers it: a commutative unit that appends once per `execute`, asserted equal to `run`; and two appenders into one accumulator, asserted in `run`'s sequence. Both fail today if the reading is right.

The same job owes the resource half. A tiled carrier hands every core the same resource binding (`rebase_accums` returns the erased pointer unchanged, `resource/bindings/per_core.rs:48-54`), where `each_convergence_thread_accumulates_into_its_own_stack_local_resource_and_non_commutative_accumulation_runs_sequentially` (ratified, the spec `:1750-1763`) gives each thread its own accumulator and a combine after. This is read from the code by the sonnet seat and not run (`hilavitkutin_a_tiled_accumulator_carrier_shares_one_resource`). Its case, first, under Miri: a resource-writing unit on an accumulator carrier at 2 workers.

Frees: the accumulator's ticket set, and declared commutativity in force, which head and tail (ticket 105) needs when op's measurement returns.

### 10. The plan cache and the plan handle

Tickets 9, 92 and 108. Crates: `hilavitkutin` (`scheduler/run.rs:126`, `scheduler/plan_handle.rs:240`, `:296-300`).

Ticket 9 narrows to `plan_cache`: the plan-dirty half is live since the swap landed (`dirty.rs:134-144`), and the cache is still only named (`run.rs:126`). The row `the_cached_plan_lives_with_the_scheduler_for_the_pipeline_s_lifetime_and_is_recomputed_only_when_a_plan_affecting_resource_dirties` (provisional) says what it is for. The engine never rebuilds its plan today (`run.rs:123-126` names the cache and leaves rebuilding to the adapt subsystem), so a counter that must not move on a clean frame passes already. The case that must fail is the other direction: a counter on the recompute, asserted to move on the frame after a plan-affecting `replace_resource`, red today; its twin, asserted still on a frame with no plan-affecting change, guards the cache once it exists.

Tickets 92 and 108 go together: `build` returns a scheduler where the plan does not fit, and the fiber descriptor array is sized apart from the plan's own dimensions. Neither has a case that can be written against the tree as it stands, and the job says so before it starts. `build` is fixed to `DefaultPlanDims`, where the descriptor length and the plan's fiber capacity are the same 64 (`plan_handle.rs:226-241`, `plan/dims.rs:78`), and ticket 92's own row says its fixture file is gone and a real one is not reachable from an integration test. So the first step is the test that cannot compile: a scheduler built over plan dimensions of its own, smaller than the default, with a registration that passes the descriptor length. That it does not compile is the finding the job starts from. Once `build` takes the dimensions, the same test is the case: red where the registration is returned as a scheduler, green where it is refused (92), and a second registration within the plan's capacity and past the old fixed length builds and dispatches every fiber (108).

Frees: a plan that is reused across frames, as the spec's "schedule once" asks, and `build` over any plan dimensions.

### 11. The small catalogued ones

Tickets 16 (with its FIXME, 29), 17 and 18. Crates: `hilavitkutin`.

Ticket 16 has a real ignored test, `tests/gate2_worker_count.rs:152` (closures handed out number the smaller of the clamped worker count and the parallelisable width plus one). It is the case that fails today and the check when un-ignored. It waits on the caller question, since the count depends on whether the caller is a worker.

Tickets 17 and 18 have placeholders and no test: `tests/r6_morsel_window_formula.rs:191` is an `unreachable!` and `tests/a1_fiber_morsel_size.rs:140` an `unimplemented!`, so un-ignoring either is red whatever the source does. Each job writes its fixture and assertion first.

For 17 the placeholder's own text is wrong and is not copied: it asks for a window of at least 64 under a floor of 66, which `plan/steps/step9_morsel.rs:75` already gives (`clamped & !3`, and 66 masked is 64). The case that fails today is the floor itself: with a floor of 66 and a write sum that drives the raw window to it, the window is asserted to be at least 66; and with a floor of 2, asserted to be above zero.

Ticket 18 is a gap in coverage and no defect in the source: `scheduler/plan_handle.rs:277` records `fiber_plan_idx: USize(f)` already. There is no red on the unfixed source to show. The job writes the multi-trunk fixture whose dispatch reorders its fibers, asserts each fiber's recorded plan index is its own, and commits beside it a negative test that declares the mutation (the dispatch position in place of `f`) in the test module and asserts the same check rejects it.

Frees: the catalogued ignores of 16, 17 and 18 off the catalogue. Jobs 11 and 13 share one round.

### 12. Masks baked at build, and the ownership rule written once

Tickets 85 and 132. Crates: `hilavitkutin` (`scheduler/run.rs:231-234`, `scheduler/worker.rs:354-356`, `dispatch/core_mask.rs:55`, `dispatch/trunk_dispatch.rs:246-253`).

The per-fiber member mask is rebuilt at two sites today, `run.rs:231-234` and `worker.rs:354-356`. If `run`'s walk is retired by then (it waits on the caller question) 85 is done once; if not, the mask is baked once at build and both walks read it. The check is the differential, unchanged, and a bench under the harness before and after; no number is quoted from outside it. The case that must fail, committed as a negative test: a baked mask with one member bit cleared, declared in the test module, and the differential asserted to report it.

For 132, `core_phase_mask` has no caller in `src`, and the rule lives inline in `trunk_dispatch.rs`. One of them goes. The check is a test that computes ownership through the surviving function for every core and trunk of a multi-trunk fixture and compares it with which core's dispatch ran each trunk. The case that must fail: the same test against the function that is to be deleted, with its rule changed by one (rank plus one modulo the core count), which the dispatch does not follow. If trunk formation is rebuilt (the ticket from the stamp above), 132 folds into it.

Frees: one source for trunk ownership, which trunk formation then rebuilds in one place, and no per-frame rebuild of the member mask.

### 13. The prefetch stubs

Ticket 155. Crates: `hilavitkutin` (`intrinsics/mod.rs:19-30`).

The stub's own doc names the intrinsic as what it is to become (`intrinsics/mod.rs:19-30`), so the intrinsic behind the vetted gate is the job where the workspace's vetted features carry `core::intrinsics::prefetch_read_data`; the helper checks the vetted list first. Where it is not vetted, the stub gets a marker naming the ticket and the ticket stays open. For the intrinsic the check is the assembly of a caller showing the prefetch instruction, and the case that fails is the same assembly today, which shows none. For the marker there is no lint in the repository that finds an unmarked stub (its lint list has none), so the job writes one under `mock/lints/`, and the case that fails is that lint on today's `intrinsics/mod.rs`.

Frees: nothing waits on it; it lands in job 11's round.

## Tickets filed or bumped from this work

All of them are rows of the workspace agenda in clause-dev, each keyed as below: `.shared/scripts/life/tickets-hilavitkutin.toml`, but for `arvo_waist_detect_counts_the_width_of_a_level_where_hilavitk` in `tickets-arvo.toml`.

The question tickets to op: `one_frame_body_for_hilavitkutin_and_the_fused_entry_retired`, `a_unit_panic_aborts_behind_a_no_unwind_boundary_in_hilavitku`, `bless_the_scheduler_plane_relocation_that_closes_the_worker`, `does_the_thread_calling_run_parallel_run_core_0_s_body`, `a_terminal_fan_in_closes_no_phase_and_trunks_follow_the_spec`, and `does_the_accumulator_tiling_read_commutativity_in_hilavitkut`.

The task tickets: `hilavitkutin_forms_a_trunk_as_a_write_conflict_component_whe` (trunk formation by union-find where the spec decomposes), `arvo_waist_detect_counts_the_width_of_a_level_where_hilavitk`, `run_parallel_never_calls_select_adapt_config_or_folds_the_pe`, `hilavitkutin_run_fused_windows_by_the_run_configuration_s_mo`, `the_aliasing_test_the_deviation_5_audit_cites_is_an_assertio`, `hilavitkutin_triage_ticket_116_cites_a_branch_dev_no_longer`, `hilavitkutin_engine_design_keeps_fiber_failure_poisons_depen` (job 3 states the split of a poisoned fiber from an aborting panic), `hilavitkutin_phases_are_full_barriers_where_the_ratified_ove`, `hilavitkutin_ignored_head_and_tail_test_targets_a_walk_the_c`, `hilavitkutin_n_way_single_trunk_branch_deleted_the_day_op_sa`, `hilavitkutin_a_tiled_accumulator_carrier_shares_one_resource` and `hilavitkutin_api_revision_design_accepts_close_results_that` (for the api lane).

The questions added by this writing: `does_measure_first_hold_the_accumulator_tiling_s_commutativi`, `does_a_skipped_accumulator_writer_run_anyway_in_hilavitkutin` and `what_does_commutative_promise_for_an_accumulator_append_in_h`. `does_the_accumulator_tiling_read_commutativity_in_hilavitkut` is bumped with the seats' answer, that the ratified rows settle it. The task added for the build's tooling: `hilavitkutin_has_no_tool_that_runs_miri_so_a_miri_driver_is`.

`cargo_mock_canon_already_said_and_unearth_do_not_exist_on_hi` was filed by the first writing and is bumped as fixed by `b4c70cd0`, for the coordinator to close.

## What a review of this note found

One reviewer pass on the first writing (`c082438a`) found it not ready on four counts, each fixed where it sits: the aliasing test said to exist nowhere exists as an empty stub on an unlanded branch; the ignored tests of tickets 17 and 18 are placeholders that cannot go green; five jobs had no failing case; and the retirement of `run` leaned on the caller question the plan leaves to op.

A second pass, by a reviewer with none of the first one's context, on `f503c854`, found it not ready again, on the job list: the cases for tickets 92, 108 and 106 could not be built through `build` as it is, the case for 17 passed on the unfixed source, ticket 18 is a coverage gap and no defect, the differential of job 1 had no control that can fail before job 5, and ticket 128 had no disposition. Each is rewritten where it sits.

A third pass, fresh, on `f0890d91` after the note was brought onto `dev` and the sixth point seated, found it not ready on thirteen counts, and this is the last pass. Fixed where they sit: job 4 now builds the evidence op's ruling needs and not the relocation, under the ratified evidence-then-bless row it had not cited; the accumulator under the skip and what `COMMUTATIVE` promises are questions to op, no longer the plan's own calls; the vehicle is labelled as one seat's and the plan's reading; the cases of jobs 7 and 10 now fail as stated; job 8 is a design round; Miri drivers, negative tests and worker counts follow the rules at the head of the job list; op's words are cited where they are committed; the swap's ratification hangs on jobs 6 and 7 alone; every job has its frees; three jobs are folded into two rounds. Left as follow-ups, since a fourth pass is not run: the round-one seats' blindness rests on their own reports, both having landed in one commit (`c082438a`), so the history cannot show it; and the reviewer's two canon calls (the relocation and the skip) rest on its reading alone, which is why both went to op rather than to a stamp.
