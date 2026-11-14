# Handoff: hv-engine-path, stopped at the start of job 4

The helper stopped on op's word that the week's limits were nearly spent. Jobs 1 to 3 of `mock/research/202611140100_engine-path-plan/PLAN.md` are done, reviewed (pass 3 MERGE) and posted `ready 749eba00`; the landing report is `/home/user/report-hv-engine-path.md` on the helper's container and is restated in the `ready` status. This commit sits on top of 749eba00 and adds only this directory and the empty doc changelist stub, so landing 749eba00 or this tip lands the same source.

## Where job 4 is

A design round is open in its DOC window: `mock/design_rounds/202611140804_changelist.doc.md` is the stub `work-in-mockspace doc` wrote, nothing filled. The two flat topics `202607200100_topic.b1-cap-lifting-completion.md` (foreign, leave it) and `202611140606_topic.parked-worker-aliasing-measured.md` (this job's) sit beside it.

Evidence in this directory: `miri-replace-swap-plan-band-stacked-borrows.txt` is Miri under Stacked Borrows on `tests/replace_swap_plan_band.rs`, which reports a data race between a non-atomic write on the calling thread and a retag read of the `Scheduler` on a worker, at `crates/hilavitkutin/src/scheduler/worker.rs:101`. Tree Borrows was not run yet. `parked_worker_aliasing.rs.draft` is the driver test, not yet in the tree: two consecutive frames, a frame then `replace_value` then a frame, a frame then `replace_resource` then a frame, over `stable_provider(32768, 0xA5)` and `FixedWorkers::<2>`. It must use `mod fixed_workers; use fixed_workers::FixedWorkers;` in place of `common::FixedWorkers`. `job4-notes.md` is the plan's job 4 text. The sketch of the arms is `mock/research/sketches/202611140610_parked-worker-plane/`.

## Plan to finish job 4

1. DOC: a note in `crates/hilavitkutin/DESIGN.md.tmpl` after the sentence ending "live reference under the aliasing model even though the value itself is race-free." saying the engine does not hold to that claim today: the calling thread's plain writes (the spawned flag at `run_parallel.rs:212`, the next core's context at `:188-192`, the frame-start `&mut` bindings at `:223-225`, the swaps at `dirty.rs:49` and `:97`), measured under Miri, citing this directory. Only measured claims. Fill the doc changelist, lock.
2. SRC: add `tests/parked_worker_aliasing.rs` from the draft. Run natively (green), then Miri under Stacked and Tree Borrows, with and without `-Zmiri-disable-data-race-detector`; record the outputs here. Miri: `MIRIFLAGS="-Zmiri-ignore-leaks [-Zmiri-tree-borrows]" CARGO_TARGET_DIR=/home/user/target/miri cargo miri test --config 'build.rustc-workspace-wrapper=""' -p hilavitkutin --test <t>` from `mock/`.
3. Lock, close; move the b1 topic back flat after the close (the close sweeps it), push.
4. Put the evidence to op on `bless_the_scheduler_plane_relocation_that_closes_the_worker` with `ask`, holder op. No repair is built before he rules.
5. Post `ready` for job 4.

## What is left after job 4

Jobs 5 to 13 of the plan, in order. Fold in: triage 30 (dead ids `#341` at `plan/mod.rs:95` and `#344` at `dispatch/engine_ctx/projection.rs:151`) in the first round touching `plan/` or `dispatch/`; the api carriers `TrunkCons`, `TrunkNil`, `PhaseCons`, `PhaseNil` (`hilavitkutin-api/src/work_unit_values.rs:57-90`) with their re-export at `dispatch/mod.rs:50-53` in one commit; the `dispatch/standard.rs:4` doc to stop naming `LockFreeDispatch` and `Scheduled`; triage 7 and 66 (drop `generic_const_exprs` at `lib.rs:24`, port `plan/core_program.rs` to `CoreProgram<PC: Capacity, TC: Capacity, FC: Capacity>`) after job 5. Ticket 161 is left alone. The ticket tool's slowness (it pushes kaski `dev` through the pre-push lint build, minutes per call) is not yet filed.
