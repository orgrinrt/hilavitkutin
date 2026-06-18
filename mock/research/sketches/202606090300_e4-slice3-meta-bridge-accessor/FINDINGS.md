# Findings: E4 slice-3 engine-to-meta bridge accessor

**Date:** 2026-06-08
**Round:** 202606090000 (E4 slice-3, task #685), at TOPIC
**Outcome:** candidate 1 (engine-owned meta block + `OnMeta`-gated Ctx accessor) WORKS. It is the chosen bridge mechanism over candidate 2 (auto-register meta resources in `Stores`).

## Context

The resource `Copy` wall (resolution `202606090100` Correction 2) proved mutable
meta state cannot ride a consumer `Resource<T>` (arena values are `Copy`,
read-only via `ctx.resource()`). So meta state is engine-owned mutable state and
needs a bridge: an `OnMeta` work unit must read engine-owned meta state that is
not in its access set. Two candidate mechanisms were posed.

## Candidate 1 (chosen, proven WORKS)

An engine-owned meta-state block (a scheduler field, interior-mutable via `Cell`,
NOT `Copy`-constrained, NOT registered in `Stores`), read by an `OnMeta` work
unit through a dedicated `meta::<T>()` accessor on its `Ctx`, distinct from the
normal access-set resource accessor.

Proven by the sketch (`cargo run` prints WORKS):
- the meta block holds a `Cell<u32> pass_count` the engine writes directly (a
  plain field write, no registration, no `Copy`, no `Selector` witness, no
  specialization);
- the `Ctx` is generic over a meta-pointer parameter `MP`, defaulted `MetaNil`
  (so existing consumer Ctx aliases need no change, mirroring slice-1's
  `write_virtuals = VirtNil` default); the engine wires a real `MetaRef<'f>`
  only for `OnMeta` work units;
- the `meta::<T: MetaAccess + MetaField>()` accessor is impl'd ONLY on
  `Ctx<'f, MetaRef<'f>>`, so a consumer's `Ctx<'f, MetaNil>` does not have it. A
  consumer cannot reach meta state at compile time: the `MetaAccess` enforcement
  falls out of the gating for free, no extra bound, no negative trait, no
  specialization. (The `wall` mod documents the consumer-reaches-meta case that
  does not compile: `no method named meta found for Ctx<MetaNil>`.)
- the `OnMeta<ScheduleEnd>` hook reads `pass_count` across two frames and
  observes 1 then 2.

Cost: a new meta-pointer Ctx parameter + the accessor impl, localized to the
`EngineCtx` machinery; the scheduler wires the meta block into `OnMeta` work
units' Ctx. No consumer-`Stores` ripple.

## Candidate 2 (viable but heavier, NOT chosen)

Auto-register the meta resources into `Stores` as plain `Copy` values; the kernel
overwrites the arena slot each frame via a known `Selector` index (always
present, so no optional-registration wall; plain `Copy` value, so no `Copy`
wall); work units read via the existing `ctx.resource()`. This dodges both walls
and reuses proven machinery, but:
- every pipeline grows by N meta resources, and the const-grouping `MaskProject`
  store-numbering shifts (consumer store bit positions move), touching the
  builder and every store-count assumption / test: a pervasive blast radius;
- the meta resources are then ordinary stores with a marker, less faithful to
  the canonical "meta resources are restricted / distinct"; the `MetaAccess`
  enforcement needs an extra bound to keep consumers from reading them, where
  candidate 1 enforces it naturally;
- `Copy` plain values cannot use interior mutation, so the values are
  engine-overwritten-whole each frame rather than cell-updated, fine for metrics
  but more rigid.

## Decision

Candidate 1. Lower blast radius (localized to the meta/Ctx machinery, no
consumer-`Stores` ripple), faithful to the canonical "meta resources are
engine-owned and restricted," and the `MetaAccess` enforcement is free from the
accessor gating. Candidate 2's only edge (reusing `ctx.resource()`) does not
outweigh its pervasive store-numbering ripple.

## Leeway

SOME-SHAPE: the sketch proves the accessor + gating; the exact meta-pointer type
name (`MetaRef` vs a richer handle), whether the meta block is one struct or a
`Selector`-style typed list, and where the block lives on the `Scheduler` are
src-CL choices. The load-bearing facts (engine-owned `Cell` block written
directly; `OnMeta`-only Ctx accessor; consumer Ctx lacks it) are settled.

## Next

Fresh doc CL (slice-3 bridge: engine-owned `MetaBlock` on the `Scheduler`, the
`MetaRef`/`MetaNil` Ctx parameter, the `meta::<T>()` accessor, the engine writing
`SchedulerMetrics` each pass, the consumer `OnMeta<ScheduleEnd>` hook reading it,
and `MetaAccess` enforcement falling out of the accessor gating) -> src CL ->
TDD (the consumer hook reads a per-pass metric; a consumer WU cannot name the
accessor) -> lock -> close. Then `run_parallel` meta parity (one pass,
202606082300).

## See also

Topic `202606090000_topic.gate2-e4-slice3-consumer-meta-hooks.md`; resolution
`202606090100_e4-slice3-resolution.md` (Corrections 1+2, the Copy wall);
deprecated thin-3a CLs `202606090200_changelist.{doc,src}.deprecated.md`;
slice-2 round 202606082100 (closed); breadcrumb
`[[engine-completion-roadmap-routine]]`.
