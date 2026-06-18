# run_parallel meta-pipeline parity: findings and sequencing

**Date:** 2026-06-08
**Context:** E4 slice-2 single-core meta pipeline shipped (round 202606082100, closed). This note records the truth-of-impl of `run_parallel`'s dispatch structure as it bears on bringing the meta lifecycle (plan-band skip + lifecycle ordering) to the parallel path, and the resulting sequencing decision.

## What single-core slice-2 shipped

`Scheduler::run` (single-core) sequences the meta lifecycle by: the rank-outer grouping renumber places meta work units in lifecycle bands (plan / schedule-ready / pass-start / consumer / schedule-end); `dispatch_trunks` loops phases `0..phase_count` and, on a clean frame, skips the leading plan band (`0..plan_phase_count`); `GateWith for OnMeta<V>` is const-open. Correct and tested (`gate2_meta_pipeline`, unit-outer path).

## run_parallel has two paths, and meta parity differs per path

`run_parallel` (`scheduler/mod.rs`) precomputes the grouping arrays (`gate2_phase` / `gate2_trunk` / `gate2_nphases`) once at first call, spawns the persistent pool, then per frame publishes once and awaits. The workers (`worker_main`) branch on `carrier_unit_outer()`:

1. **Phase-loop path** (record-bearing, no accumulator): each worker loops `p in 0..gate2_nphases`, calling `run_core_phase` per phase, crossing interior waists via the worker-side `waist_barrier`. Bringing the plan-band skip here is a clean mirror of `dispatch_trunks`: precompute `plan_phase_count` into a `gate2_plan_phases` field at first call; thread a per-frame plan-dirty bool (set before `frame_publish`, read by workers after the publish under the frame happens-before); start the worker phase loop at the plan-band offset on a clean frame. Tractable.

2. **Unit-outer path** (`worker_accum_unit_outer`, accumulator-bearing): there is NO phase loop. Each core takes a head+tail record slice `[lo, hi)`, dispatches the WHOLE carrier once over its slice into a per-core accumulator region, then the main thread merges the per-core regions. This path cannot express lifecycle band ordering (no phases), and a frame-level meta work unit (resource-only, no records) would run once PER CORE, i.e. N times, not once. Bringing meta here is a genuine redesign: a designated thread (e.g. core 0, or the barrier-releaser) must run the meta bands once per frame around the per-core record work, under the frame happens-before. This is the "designated-thread at waist barrier" reservation noted in the slice-2 breadcrumb.

The slice-2 single-core test uses the unit-outer path (an accumulator, to dodge the incremental-skip confound on the morsel path). So the unit-outer path is the one a meta+accumulator pipeline most naturally hits, and it is the harder of the two.

## Testing confound (both single-core and parallel)

The clean-frame plan-band skip is observable across two frames only on a path that does NOT apply incremental skip. The morsel path (single-core) and `run_core_phase` (parallel phase-loop) apply the dirty-unit skip, so on a clean second frame all units skip regardless of the plan-band skip, making the plan-band skip unobservable there. The unit-outer path ignores the dirty mask (runs every unit every frame), which is why the single-core slice-2 test uses it. A parallel meta test therefore also wants the unit-outer path, which is exactly the path that needs the redesign in (2).

## Sequencing decision

Do single-core meta first, then one focused parallel pass. Concretely:

1. **Next: slice-3 (single-core)** — consumer `On<meta::V>` hooks (real `Virtual<meta::V>` firing by the kernel so a consumer-schedule `On<meta::V>` work unit observes a lifecycle event), the `MetaAccess` compile-time `Context` enforcement bound, and the E8 adaptation work unit body on `On<meta::ScheduleEnd>` (spec `:2042-2048`). Builds on slice-2's single-core meta lifecycle; does not need the parallel path.
2. **Then: run_parallel meta parity (one pass)** — bring the whole single-core meta pipeline (band-skip + lifecycle ordering + real firing) to both `run_parallel` paths: the phase-loop path via the `dispatch_trunks`-mirror skip; the unit-outer path via the designated-thread meta mechanism. Doing this once, after single-core meta is complete, avoids two partial parallel passes (one for slice-2's skip, another for slice-3's firing).

Rationale: slice-3 is higher downstream leverage (it delivers the actual self-hosting + adaptation capability and unblocks domain-22), is single-core-buildable, and the parallel-meta work is cleaner as a single pass over a complete single-core meta pipeline than as two incremental parallel passes. No meta consumer uses `run_parallel` today, so the parallel-meta gap is a marked follow-on, not a live regression. The single-core path stays the correct, tested reference.

## See also

Round 202606082100 (closed, slice-2 single-core); breadcrumb `[[engine-completion-roadmap-routine]]` LATEST; canonical Q9 `:716-814`, adaptation `:2042-2048`.
