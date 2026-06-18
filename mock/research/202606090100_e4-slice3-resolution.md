# E4 slice-3 resolution: the consumer meta hook is an OnMeta WU, not an On-stamp consumer

**Date:** 2026-06-08
**Round:** 202606090000 (E4 slice-3, task #685), at TOPIC
**Outcome:** the topic's two hard de-risk targets (Q1 optional real firing, Q2 consumer-hook rank) DISSOLVE on re-grounding; slice-3 re-scopes to a tractable shape with no new firing mechanism.

## The re-grounding

The slice-3 topic assumed a consumer adaptation hook is a consumer-schedule
`On<meta::ScheduleEnd>` work unit that gates on a fired `Virtual<meta::ScheduleEnd>`
stamp (the slice-1 `On<V>` stamp mechanism). That assumption drove both hard
questions: Q1 (the kernel must fire a real meta virtual, optionally, without a
specialization wall) and Q2 (a consumer `On<V>` carries consumer rank, not
epilogue, so it would not be ordered after consumers).

Re-reading the canonical (`:1985-1991` cadence, `:2119-2137` self-hosting) against
slice-2's established mechanism shows the assumption is wrong. The canonical
"consumer reads `SchedulerMetrics` via `On<meta::ScheduleEnd>` WU" is, under
slice-2's forced surface deviation, an **`OnMeta<ScheduleEnd>` work unit**. Slice 2
already:

- gives `OnMeta<ScheduleEnd>` the epilogue lifecycle rank (`RANK_SCHEDULE_END` = 4),
- renumbers it into the final phase band, after the consumer band (rank 3),
- dispatches it there every frame (it is not plan-stage, so the clean-frame plan
  skip does not touch it).

So the adaptation hook RUNS at the schedule-end lifecycle point purely by band
placement. "Fired after all consumer work" is realized as "dispatched in the
epilogue band, after the consumer bands." There is no fired stamp to observe;
the hook is a meta work unit, not a stamp-gated consumer.

## Consequence: Q1 and Q2 dissolve

- **Q1 (real meta firing with optional registration) is not needed for slice 3.**
  The adaptation hook does not gate on a `Virtual<meta::ScheduleEnd>` stamp; it
  runs by band dispatch. So the kernel does not need to stamp meta virtuals, and
  the "stamp-if-present-else-no-op" specialization wall never arises. Real meta
  stamp firing would only be needed if a NON-meta work unit (an `Always` or
  `On<consumerV>` unit) had to observe "schedule-end happened this pass" via the
  stamp gate. That is not a canonical pattern (the canonical hook is the OnMeta
  WU) and is YAGNI for slice 3. If such a use case ever arises, it is an additive
  later feature, not a slice-3 blocker.
- **Q2 (consumer-hook rank) dissolves.** The hook is `OnMeta<ScheduleEnd>`, which
  already carries epilogue rank from slice 2. No rank reconciliation, no
  `On<meta::X>` vs `On<consumerV>` classification (the E0119 wall slice 2 dodged
  with `OnMeta`). It is solved by the slice-2 machinery as-is.

## Re-scoped slice 3

With the firing question gone, slice 3 is two tractable pieces on top of slice-2's
OnMeta band dispatch:

**slice-3a (core deliverable, this round): `SchedulerMetrics` + kernel population
+ adaptation-hook test.**
- `SchedulerMetrics` gains its real fields (`pass_count`, `ema_pass_duration_ns`,
  `active_units`, `stolen_count`, `idle_ns`) and becomes a registrable
  `Resource<SchedulerMetrics>` (the slice-2 marker `meta::SchedulerMetrics` is the
  resource value type).
- The single-core `run` kernel populates the single-core-meaningful fields:
  `pass_count` (per-frame counter) and `active_units` (the live unit count from
  the grouping). `ema_pass_duration_ns` needs a `Clock` + frame timing (adapt /
  parallel territory, #341); `stolen_count` / `idle_ns` are parallel-path data
  (work-stealing / parking). Those land with their data sources; slice-3a ships
  the fields with the single-core-meaningful ones populated and the rest at a
  documented default, NOT a stopgap (the field exists; its data source does not
  yet).
- The kernel writes `SchedulerMetrics` at frame start (increment `pass_count`,
  set `active_units`), so an epilogue `OnMeta<ScheduleEnd>` hook reads the
  current frame's metrics. Frame-start write avoids a mid-phase-loop hook.
- Test: an `OnMeta<ScheduleEnd>` work unit reads `SchedulerMetrics`, observes
  `pass_count` incrementing across frames, and runs after a consumer (its
  epilogue band placement, on the unit-outer path to dodge incremental skip,
  as in the slice-2 test).

**slice-3b (safety guard, can be its own round): `MetaAccess` compile-time
`Context` enforcement.** A consumer work unit whose access set contains a meta
resource must be rejected unless its schedule is `OnMeta`. This is a cross-cutting
conditional constraint (access-set-contains-meta implies schedule-is-meta) and is
the one genuinely intricate type-level piece. It is a NEGATIVE safety guard, not
the positive capability; slice-3a delivers the capability, slice-3b adds the
guard. If the bound shape carries trait-solver risk it gets its own de-risk
sketch then. The `MetaAccess` marker already exists (slice 2); 3b adds the bound.

## No runnable sketch needed for 3a

slice-3a uses only proven machinery: `Resource<T>` registration + `ctx.resource()`
read (existing), the kernel writing a resource binding directly (existing pattern,
as the engine writes other scheduler state), and slice-2's `OnMeta` band dispatch
(shipped, tested). No new trait-solver mechanism, so no runnable sketch is
required. slice-3b's `MetaAccess` bound is where a sketch may be warranted, at its
own round.

## Next

DOC CL for slice-3a (the `## Self-hosting meta pipeline` consumer-hook + metrics
paragraphs in engine `DESIGN.md.tmpl`) -> src CL (`SchedulerMetrics` fields +
`Resource` registration + kernel population + `OnMeta<ScheduleEnd>` hook test) ->
lock -> close. Then slice-3b (`MetaAccess` enforcement), then `run_parallel` meta
parity (one pass, per 202606082300).

## Correction (deeper impl pass): metrics population is a provided WU, not a kernel write

The first draft said slice-3a's kernel "writes a resource binding directly
(existing pattern)." A deeper pass shows that walls on the SAME
optional-registration problem as the deferred firing: a kernel that writes
`ResourceBinding<SchedulerMetrics>` only when registered needs a structural
"write-if-present-else-no-op" over the bindings, whose head-is-`SchedulerMetrics`
vs head-is-other impls overlap (E0119), and the witness-resolution path
(`Selector<SchedulerMetrics, I>`) requires the resource be present, so it cannot
express "absent -> no-op." Structural membership testing walls without
specialization; the only clean shapes are always-present (a meta-state block /
auto-register, both heavier) or caller-threaded-witness (requires present).

The clean escape that keeps "no new mechanism": do NOT have the kernel write
`SchedulerMetrics`. Ship an engine-provided `SchedulerMetricsWu` (schedule
`OnMeta<PassStart>`) that READS and WRITES the `Resource<SchedulerMetrics>`
itself, self-incrementing `pass_count` each frame. It is an ordinary work unit
on slice-2's `OnMeta` band machinery over a normal consumer-registered
`Resource<SchedulerMetrics>`; the grouping tolerates a unit reading and writing
the same store (the RAW-edge check skips `i == j`). No kernel special-casing, no
meta-state block, no `Selector`, no optional-registration wall. "hilavitkutin
provides the measurement data" = it ships the work unit; a consumer that wants
metrics registers `SchedulerMetricsWu` plus their `OnMeta<ScheduleEnd>` hook.

Field scope (no caricature): `pass_count` is self-sourceable by the WU and ships
in slice-3a. The other canonical fields need engine internals or other data
sources and land with them, NOT as dead fields now: `active_units` needs the
grouping's live count (the engine->meta bridge), `ema_pass_duration_ns` needs a
`Clock` + frame timing (#341), `stolen_count` / `idle_ns` are parallel-path data
(work-stealing / parking). `SchedulerMetrics` starts with `pass_count` and grows
as sources land (pre-1.0 churn, no legacy shims). The engine->meta bridge for
engine-internal metrics (active_units etc.) is the slice that needs the
always-present meta-state mechanism; it is deferred behind slice-3a's
self-sourced `pass_count` deliverable and folded with `MetaAccess` enforcement
(slice-3b) where the meta-state-block / accessor question is actually decided
(by sketch then).

So slice-3a = `SchedulerMetrics { pass_count }` + engine `SchedulerMetricsWu`
(`OnMeta<PassStart>`, self-increments `pass_count`) + a consumer
`OnMeta<ScheduleEnd>` hook reading `pass_count` + a behavioral test. No kernel
change, no new trait-solver mechanism. slice-3b (MetaAccess enforcement + the
engine->meta bridge for internal metrics) is where the always-present meta-state
mechanism gets sketched and decided.

## Correction 2 (hard compile wall): there is no thin slice-3a; the bridge is irreducible

The provided-WU escape was tried in source and hit a hard wall: a consumer
`Resource<T>` value must be `Copy` (the arena `DrainStores` requires it:
`Sv<StagedResource<SchedulerMetrics>, ...>: DrainStores` needs
`SchedulerMetrics: Copy`). A `Cell<USize>` is not `Copy`, so a mutable
cell-bearing `SchedulerMetrics` resource does not compile. And a work unit reads
a resource via `ctx.resource() -> &T` (shared, read-only after init; there is no
resource-writer accessor). So a resource cannot carry mutable per-pass state at
all: resources are `Copy` read-only inputs.

That kills every thin escape:
- kernel writes the resource: walls on optional-registration (Correction 1);
- provided WU mutates the resource: walls on `Copy` (a cell is not `Copy`, and
  there is no resource writer);
- column-backed singleton metric: would compile (columns are work-unit-writable),
  but it is a stopgap, a 1-record column standing in for engine-owned meta state
  purely to flip the deliverable green, which the bridge would later replace.
  Forbidden by the no-stopgap rule.

So mutable meta state (the metrics, and the meta resources generally) is
engine-owned mutable state, not any consumer store. `SchedulerMetrics` cannot
ride `Resource<T>` / `Column<T>` / `Accum<T>`. It needs the always-present
engine-owned meta-state block plus an accessor that lets an `OnMeta` work unit
read it (the engine-to-meta bridge). That bridge IS slice 3; there is no smaller
real slice that avoids it.

The thin-slice-3a round (`202606090200` doc + src CLs) was deprecated (audit
trail) and its source backed out. The `SchedulerMetrics` marker stays a ZST in
api `meta.rs` until the bridge gives it real engine-owned fields.

## Re-scope: slice 3 is the engine-to-meta bridge (needs a sketch)

The next step is a de-risk sketch of the engine-owned meta-state mechanism:
an always-present meta-state value the engine writes (a scheduler field, no
registration, no `Copy` constraint, no specialization), read by an `OnMeta` work
unit through a `MetaAccess`-gated accessor distinct from the normal access-set
resource accessor. Candidate shapes to compare in the sketch:
1. a meta-state block as a scheduler field, exposed to an `OnMeta` work unit's
   `Ctx` via a new `MetaAccess` accessor path (cleaner, no consumer-Stores
   ripple, but a new Ctx accessor);
2. auto-registered meta resources in `Stores` (normal accessor, but every
   pipeline grows by N stores and the const grouping store-numbering shifts);
the sketch decides which compiles cleanly with the smaller blast radius. Once the
bridge mechanism is proven, the metrics population, the consumer hook reading
real engine metrics, and `MetaAccess` enforcement all follow on it.

## See also

Topic `202606090000_topic.gate2-e4-slice3-consumer-meta-hooks.md`; slice-2 round
202606082100 (closed); run_parallel findings 202606082300; canonical Q9
`:2119-2137`, cadence `:1985-1991`; breadcrumb `[[engine-completion-roadmap-routine]]`.
