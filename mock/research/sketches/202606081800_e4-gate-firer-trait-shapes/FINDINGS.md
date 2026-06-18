# Findings: E4 slice-1 gate + firer trait shapes

**Date:** 2026-06-08
**Round:** 202606081200 (E4 slice 1, task #685)
**Outcome:** WORKS.

## Hypothesis

The engine can fire virtuals and gate a heterogeneous `Always` / `On<V>` carrier
without specialization and without `E0207`, reusing the codebase's
witness-list-inference idiom, with the firer and the gate reaching the SAME
`VirtualBinding<V>` stamp cell by identity (no global virtual index).

## What compiled and ran

`cargo run` prints
`WORKS: firer + gate share the V-keyed cell; epoch decay gates On<V> correctly`
and all three pass assertions hold:

1. epoch=1, producer fires `Tick`, then the gated walk runs Producer, Consumer
   (`On<Tick>`), Plain. Consumer gated open because the stamp == current epoch.
2. epoch=2, no fire: the gated walk runs Producer, Plain only. The stale stamp
   (still 1) != epoch 2, so the `On<Tick>` consumer gates shut. This is the
   per-pass epoch decay (no explicit clear needed).
3. epoch=3, fire again: Consumer runs again.

## The proven shapes (lift directly into the engine)

1. **Shared keying primitive** `VirtualStampSelector<V, Index>`: structural over
   the binding nodes (`Here` matches the `VBind<V, _>` head, `There<I>` recurses
   any node kind), returns `&Cell<u64>`. The index infers. Mirrors
   `AccumSelector<T, Index>` exactly. Both firer and gate resolve through it, so
   they hit the same cell.

2. **Gate** `GateWith<A, GI>` dispatched on the unit's schedule:
   - `impl<A> GateWith<A, Here> for Always` returns `true` (const-foldable, DCE).
     Pinning `GI = Here` means an `Always` unit's witness element is forced to
     `Here` and infers cleanly with no ambiguity.
   - `impl<A, V, GI> GateWith<A, GI> for On<V> where A: VirtualStampSelector<V, GI>`
     reads the cell and compares to the epoch.
   `GI` is a trait parameter (appears in the trait ref), so neither impl trips
   `E0207`. At the walk, `GI` is destructured from the parallel per-unit witness
   list `Cons<GI, SWTail>` (a constrained position, exactly how `RunFiber`
   destructures `(RIdx, RCIdx, WCIdx, WAIdx)`), so it is never a free impl param.

3. **Carrier bound** `W: HasSchedule + WorkUnit<<W as HasSchedule>::Sched>` works
   as the per-cell bound in the gated walk (replaces the current `W: WorkUnit`).
   The blanket `impl<W: WorkUnit<Always>> HasSchedule` gives every existing
   Always WU `HasSchedule` with zero churn; `Consumer` adds an explicit
   `impl HasSchedule { type Sched = On<Tick>; }`. They cohere.

4. **Firer** `VirtualProject<'a, WSet, Idx>` mirrors `AccumProject`: pulls each
   `Virtual<T>` member of the WU's write set out of the bindings into a
   `VCons<&Cell, _>` bundle (the `EngineCtx::write_virtuals` field shape).
   `VirtualFire<T, Index>` over that bundle fires by type with the index
   inferred (mirrors `ctx.append`'s method-generic index). `FireCtx::fire<T, I>`
   is the `VirtualFirerApi::fire<V>` body shape.

## Inference confirmation (the real risk)

No turbofish was needed on any witness list:

- `carrier.run(&bindings, epoch)` infers the whole `SchedW` list (`Always` ->
  `Here`, `On<Tick>` -> the selector index for `Tick`).
- `VirtualProject::<Cons<Virtual<Tick>, Nil>, _>::vproject(&bindings)` infers the
  index list (only the write-set is pinned, exactly as a WU declares its writes).
- `fctx.fire::<Tick, _>()` infers the `VirtualFire` index.

## Engine mapping

- `VirtualStampSelector` -> new trait in `engine_ctx.rs` over the real binding
  nodes (`VirtualBinding` head + `There` recursion over Resource/Column/
  Accum/Virtual bindings), returning `&Cell<USize>` (CHANGE 3's `stamp`).
- `GateWith` -> new trait; the gate call lands in `trunk_gate.rs` ANDed with
  `Member::IS && dirty.bit(...)`; `GI` threaded as a new parallel `SchedW`
  witness param on `RunGatedTrunk` (+ `RunTrunkDispatch`, + the scheduler
  dispatch sites), inferred at `Scheduler::run` like `Witnesses` / `GW`.
- `VirtualProject` + `VirtualFire` -> `engine_ctx.rs`; `EngineCtx` gains a
  `write_virtuals: WVirt = VirtNil` field projected in `project`; the real
  `fire<V>` replaces the B3 no-op.
- epoch -> `USize` (CHANGE 3), threaded from a scheduler `virtual_epoch` field
  incremented per pass.

## Caveat / leeway accepted

The sketch uses `u64` for the epoch and `Vec`/`thread_local` for observation
(sketch-only; the engine uses `USize` and a test sentinel column). The shapes
proven are the trait structure and inference, which are toolchain-identical to
the engine forms. The `Always` -> `Here` pin is a real constraint to carry into
the engine: the plan/builder must place `Here` (or any concrete dummy) as the
`SchedW` element for Always units; inference does this automatically through the
`GateWith` bound, so the call site needs no extra annotation.
