# Sketch A2a: per-fiber morsel loop drives the per-trunk monomorphised dispatch

**Date:** 2026-06-19
**Premise (hypothesis):** the GATE-2 per-trunk monomorphised dispatch can be driven
so each fiber processes a distinct `MorselRange` sequence (fiber-outer/morsel-inner),
WITHOUT (a) reintroducing a per-record indirect call and (b) breaking the
phase-gated const-DCE. Leeway: some-shape (prove one shape in the family).

**Outcome: WORKS (source-grounded derivation). Compile proof = A2b itself.** A
standalone driver cannot compile because witness/`GW`/bindings inference only closes
against `Scheduler`'s own where-bounds (a free function cannot name
`<Vals as BindingsFor>::Bindings` nor infer `Witnesses`/`GW`). So the proven shape is
a `Scheduler` method, and compiling it IS the A2b implementation step, not a separable
standalone sketch. (The empirical compile during the sketch was also blocked by a full
build disk, since freed; the conclusion is from the shipped dispatch path, which is
unambiguous.)

## Load-bearing facts (from shipped source)

1. `MorselRange` (`dispatch/morsel.rs`) is a plain runtime value, threaded
   `dispatch_trunks -> RunGatedTrunk::run_trunk -> RunFiber::run_head` purely as an
   argument. Every DCE site on that path is compile-time and reads neither the range
   nor the dirty mask: `IsRoot::IS` / `PhaseAt::VAL` (`trunk_dispatch.rs`),
   `Member::IS` / `GateWith::open` (`trunk_gate.rs`). So distinct per-call ranges
   cannot perturb the const-DCE, and the per-record body is byte-identical.
   Constraints (a) and (b) hold by construction.
2. No new carrier API is needed. The shipped `Scheduler::run_one_trunk::<_,_,TRUNK>`
   (`scheduler/mod.rs` ~1454-1477) already dispatches one trunk's members over a
   `MorselRange`, member-only, devirt-clean. The per-fiber windowing delta is passing
   a non-full range and looping it.
3. `tests/morsel_outer.rs` proves `ctx.each()` iterates exactly the passed
   `MorselRange`, so a per-fiber `while start < total` over windows of `morsel_size`
   yields `ceil(total/window)` morsels per fiber.
4. A fiber sits wholly in one phase (and in single-member trunks, one trunk), so a
   single windowed walk runs that fiber's whole sequence in order before the next.

## Exact shape A2b implements (mechanical)

A `Scheduler` method (NOT a free function), `run_one_trunk` plus an inner window loop:

```rust
#[doc(hidden)]
pub fn run_one_trunk_windowed<Witnesses, GW, const TRUNK: usize>(&mut self, window: USize)
where WuVals: RunGatedTrunk<WuVals, <Vals as BindingsFor>::Bindings, Witnesses, GW,
    Stores, <D as PlanDims>::Units, <D as PlanDims>::Stores, <D as PlanDims>::AdjRow, TRUNK, Here>,
{
    let total = self.record_count.0;
    self.virtual_epoch.fetch_add(1, Ordering::Relaxed);
    let epoch = USize(self.virtual_epoch.load(Ordering::Relaxed));
    let all = <D as PlanDims>::AdjRow::default().bitnot();
    let w = window.0.max(1);
    let mut start = 0;
    while start < total {
        let len = w.min(total - start);
        self.wu_values.run_trunk(&self.bindings, &self.meta_block,
            MorselRange::new(USize(start), USize(len)), all, epoch);
        start += len;
    }
}
```

A2b lifts this loop into `run` (`scheduler/mod.rs` ~1358-1411), fiber-outer over the
real multi-member carrier, keyed by each `FiberDispatch.morsel_size` (the A1 field),
replacing the all-trunks `dispatch_trunks`.

## Hard sub-problems surfaced (feed A2b)

- **Re-walk cost.** Driving the per-trunk carrier per fiber re-walks the whole carrier
  once per fiber per window: O(fibers x windows x carrier_len) const-gate evals. The
  per-record work stays correct and devirt-clean (gate evals are compile-time-folded
  branches, not per-record cost), but the walk overhead multiplies. A2b should prefer a
  DIRECT per-fiber entry (walk to the fiber's members once, then loop windows there)
  over re-walking the full carrier per fiber.
- **Mask orthogonality.** The production per-fiber member selector must be a NEW
  `fiber_mask & dirty`, NOT an overload of `dirty`. Incremental-skip (this frame's
  changed units) and fiber membership (this fiber's members) are orthogonal. The
  single-member-trunk fixture sidesteps it; real `run` needs an explicit `fiber_mask`.

## Devirtualisation

Preserved by construction (the per-record body is the unchanged `run_trunk`/`run_head`
projection the existing `asm_gate` already proves carries zero indirect call). A2b adds
a per-fiber-windowing fixture to the asm gate and asserts the same (this is the first
fixture under the generalized ASM-emission contract discipline, roadmap 202606201900).

## Next

A2b: implement `run_one_trunk_windowed` (or the direct-per-fiber-entry refinement) on
`Scheduler`, lift into `run` keyed by `FiberDispatch.morsel_size`, add the `fiber_mask`,
add the asm-gate fixture. The dispatch-order test from `per_fiber_window.rs` (windows of
4 then 8 over TOTAL=10 -> `[(0,0),(0,4),(0,8),(1,0),(1,8)]`) is the A2b acceptance test.
