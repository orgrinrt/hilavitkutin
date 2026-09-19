# Sketch A4: per-fiber morsel windows on the parallel dispatch path

**Date:** 2026-07-19
**Premise (hypothesis):** `run_core_phase` can drive the GATE-2 core-pinned per-trunk
monomorphised dispatch so each fiber walks its own plan-baked L1 window
(fiber-outer/morsel-inner), without (a) reintroducing a per-record indirect call,
(b) breaking the phase-gated const-DCE, (c) disturbing trunk-rank core ownership,
or (d) changing the waist-barrier structure. Leeway: some-shape.

**Outcome: WORKS (source-grounded derivation), with two corrections to the roadmap's
A4 statement.** Same compile-proof shape as sketch `202606202100_a2a`: witness / `GW` /
bindings inference closes only against `Scheduler`'s own where-bounds, so the proven
shape is a `Scheduler` method body and compiling it IS the A4 implementation step.

## Correction 1: A4 is not "a unit-to-fiber-size lookup". It is the same loop inversion A2b did.

The roadmap (r6 Phase A, inherited from blueprint slice 5) states A4 as threading
per-fiber sizes into `run_core_phase` and `worker_main` "through a unit-to-fiber-size
lookup, which needs a `gate2_fiber` mapping alongside the existing `gate2_phase` and
`gate2_trunk` scratch". That framing does not fit the dispatch shape.

`RunTrunkDispatch::dispatch_core` (`dispatch/trunk_dispatch.rs:143-170`) walks the
**entire carrier** in one call, firing every in-phase trunk root the core owns, all
under a **single** `morsel: MorselRange` argument. The morsel loop is outside it
(`scheduler/mod.rs:2150-2170`). So one `dispatch_core` call already spans every fiber
in every trunk the core owns for that phase, and those fibers have different windows.
No per-unit size lookup can express that: the size has to select the loop, and the
loop is outside the call.

A4 therefore requires the same restructuring `run()` received in A2b: fiber-outer,
morsel-inner, one window sequence per fiber. Its ledger row should read `specced`
(sketch first) rather than `ready` (mechanical).

## Correction 2: `gate2_fiber` is not needed at all, which dissolves the #690 decision.

Because the loop is fiber-outer, it is driven by the `FiberDispatch` descriptors, which
already carry everything required: `start` and `len` (the fiber's slice of `topo_order`)
and `morsel_size` (the plan-baked A3b L1 window). `fiber_dispatch` is a `Scheduler`
field and `run_core_phase` takes `&self`, so the descriptors are already in scope. The
worker reads them through its `*const Self`, and they are not mutated during a frame.

So there is no third `[USize; GATE2_MAX_UNITS]` array, and the r6 decision about whether
`gate2_fiber` lands pre-lift or mid-lift against #690 is moot. Recorded rather than
silently dropped: the decision was sound for the shape it assumed, and the shape changed.

## Load-bearing facts (from shipped source)

1. **The dirty mask is a per-unit member gate, and is the mechanism for fiber
   restriction.** `dispatch_core` threads `dirty: M` down to `RunGatedTrunk::run_trunk`,
   which gates each unit on `Member::IS && dirty.bit(Pos::INDEX) && GateWith`
   (`dispatch/trunk_gate.rs`). Passing a fiber-restricted mask therefore runs exactly
   that fiber's units, which is precisely how A2b's `run()` restricts members
   (`scheduler/mod.rs`, `members = fmask & cmask & dirty`). The mask is a runtime value
   read by no DCE site.

2. **Trunk-rank core ownership is invariant under the dirty mask, so calling
   `dispatch_core` once per fiber preserves ownership.** In `dispatch_core`, `rank`
   advances for **every** in-phase root, owned or not, and the advance is outside the
   ownership test and independent of `dirty` (`trunk_dispatch.rs:161-167`). The caller
   resets `rank` to `USize::ZERO` before each call. So every per-fiber call re-walks and
   re-ranks identically, and core `c` owns exactly the same trunk set in every call.
   Constraint (c) holds.

3. **Const-DCE is untouched, by the same argument A2a established.** Every DCE site on
   the path is compile-time and reads neither the range nor the mask: `IsRoot::IS` and
   `PhaseAt::VAL` (`trunk_dispatch.rs:156-160`), `Member::IS` and `GateWith::open`
   (`trunk_gate.rs`). Distinct per-call ranges and masks cannot perturb it, and the
   per-record body is byte-identical. Constraints (a) and (b) hold by construction.

4. **The waist barrier is per phase, not per morsel, so it is unaffected.**
   `worker_main` crosses the interior waist after `run_core_phase` returns, once per
   phase (`scheduler/mod.rs:908-920`). Restructuring the loop inside `run_core_phase`
   does not move the barrier. Constraint (d) holds.

5. **A fiber sits wholly within one phase and one trunk**, so a per-fiber windowed walk
   runs that fiber's whole record sequence in order before the next fiber starts, which
   is the locality the L1 window exists to buy.

6. **The head+tail branch is out of scope**, per r6's settled decision: it dispatches
   over a whole-phase mask rather than one fiber, so a per-fiber window does not map
   onto it. It keeps the scalar `msize`, stated explicitly in source.

## Known cost, and where it gets measured

Each core walks the carrier once per fiber instead of once per morsel. Whether that is
cheaper depends on the fiber count against the morsel count, and a core walks fibers it
does not own (the ownership test is inside the walk, so skipping requires the walk).
This is the same class of overhead A2b accepted on the single-core path and tracked as
a FIXME at #340 (plan-baked member masks plus a fiber-to-phase map would skip the no-op
walks). It is not a correctness issue, and G2C-M is the bench that measures it, since
that bench already covers this path's scaling curve.

## Exact shape A4 implements

Inside `run_core_phase`, replacing the ordinary trunk-rank branch (`scheduler/mod.rs`
~2148-2170). The `tphase == 1` head+tail branch and the `total == 0` branch are
unchanged.

```rust
// fiber-outer / morsel-inner (A4): each morsel_local fiber walks its own
// plan-baked L1 window; ownership stays trunk-rank, so a core runs only the
// fibers in the trunks it owns. rank re-ranks identically per call because
// the advance is independent of the dirty mask.
let descriptors = self.fiber_dispatch.as_ref();
let fcount = self.fiber_dispatch_count.0.min(descriptors.len());
let order = self.topo_order.as_ref();
let mut fi = 0;
while fi < fcount {
    let desc = descriptors[fi];
    // FIXME: rebuilt per (phase, fiber); plan-bake the member masks; tracked #340.
    let mut fmask = <D as PlanDims>::AdjRow::default();
    let kend = (desc.start.0 + desc.len.0).min(order.len());
    let mut k = desc.start.0;
    while k < kend { fmask = fmask.with_bit_set(order[k]); k += 1; }
    let w = if desc.morsel_local.0 && desc.morsel_size.0 != 0 { desc.morsel_size.0 } else { msize.0 };
    let mut start = 0;
    while start < total {
        let len = w.min(total - start);
        let mut rank = USize::ZERO;
        self.wu_values.dispatch_core(
            &self.wu_values, p, core, ncores, &mut rank,
            &self.bindings, &self.meta_block,
            MorselRange::new(USize(start), USize(len)), fmask, epoch,
        );
        start += len;
    }
    fi += 1;
}
```

## What this unblocks and what it does not

Unblocks A4's implementation with a proven shape. Does **not** address the accumulator
path (`worker_accum_unit_outer`), which is A6's windowing question and resolves from
G2C-M, nor the three-site ceil-slice work, which is G2C-0 and G2C-1a.
