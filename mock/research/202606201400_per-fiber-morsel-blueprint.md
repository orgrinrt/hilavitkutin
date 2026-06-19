# Per-fiber morsel-loop wiring: implementation blueprint

**Date:** 2026-06-19
**Status:** design-reviewed blueprint (code-architect) for the per-fiber morsel slice
**Follows:** `202606201300_adapt-actuation-chart.md` (which resolved: per-fiber morsel sizing is canonical per spec)

## Two findings that reshape the slice

1. **`plan.morsel_sizes` is a record-count PARTITION, not a window size.** `size_morsels`
   (`plan/steps.rs`) computes a sum-preserving split (`Σ morsel_sizes[f] == record_count`,
   per the `plan/mod.rs:84-89` doc). The spec (domain 12, :832-843) defines
   `morsel_size = (L1_usable / Σ write_sizes).clamp(MIN,MAX) & !3` — a window CHUNK
   size, with each fiber covering `[0, total)` in `ceil(total/window)` morsels. These are
   categorically different. Wiring the current field value as a window would give one
   morsel per fiber, contradicting spec line 82 ("multiple morsels per fiber"). The field
   is intermediate-round drift; the slice corrects it, does not consume it as-is.

2. **The dispatch loop SHAPE is canonically wrong.** Current `run` (`scheduler/mod.rs:1342-1390`)
   is morsel-outer-SHARED: it windows `[0,total)` by the uniform const `Cfg::MORSEL_SIZE` and
   calls `dispatch_trunks` once per window; every fiber in every trunk in every phase sees the
   same window. The spec shape is fiber-outer/morsel-inner: each fiber walks its OWN window
   sequence over `[0,total)` before the next fiber, keeping that fiber's co-located columns
   L1-hot for its whole window run. Same output, completely different cache behaviour. This is
   a loop inversion (the load-bearing change), not a localized tweak. `dispatch_trunks`
   (:1468) / `RunTrunkDispatch` / `RunGatedTrunk::run_trunk` pass ONE `MorselRange` to the
   whole carrier; there is no per-fiber range hook today.

So this is a canonical-design drift in its own right (independent of adapt), and fixing it is
what gives the adapt re-chunk actuation a live runtime surface.

## Vehicle

`FiberDispatch` (`scheduler/mod.rs:482-490`, the per-fiber dispatch descriptor already carrying
`start`/`len`/`morsel_local`) is the right place for per-fiber dispatch state. Add
`morsel_size: USize` and `fiber_plan_idx: USize` (the plan CSR fiber index, needed because the
`fiber_dispatch[..]` dispatch-order index space differs from `plan.morsel_windows[f]` CSR order).

## Slice decomposition (each independently landable + testable)

1. **Semantic rename + signature.** `ExecutionPlan::morsel_sizes` -> `morsel_windows`; redefine
   as window sizes; fix the doc; `size_morsels` -> `compute_fiber_morsel_windows` with the real
   signature `(record_count, per_fiber_write_bytes, l1_usable, min, max)` but a placeholder body
   that reproduces the current numeric output (so existing tests pass unchanged). Update the
   sole reader `plan/core_program.rs:72` (rename only). Catalogue-red test: `morsel_windows[f]`
   matches the L1 formula for a known fiber (red until slice 4).
2. **Per-fiber size on the descriptor.** Add `morsel_size`/`fiber_plan_idx` to `FiberDispatch`;
   thread `plan.morsel_windows` into `derive_phase_dispatch_order` (:400-471, the `f` CSR index
   is the loop var); populate the descriptor. Dispatch still uses the uniform const; the field
   is available, not yet consumed. White-box test: `scheduler.fiber_dispatch[fi].morsel_size ==
   plan.morsel_windows[fi]`.
3. **Dispatch loop inversion (load-bearing).** Refactor `run` (:1358-1390) to fiber-outer/
   morsel-inner for `morsel_local` fibers: per descriptor, `window = morsel_size.0.max(1)`
   (fallback `Cfg::MORSEL_SIZE` when zero), inner `while start < total` driving that fiber's
   units (`topo_order[desc.start..desc.start+desc.len]`) over `[start, start+window)`. Unit-outer
   (`morsel_local=false`, accumulator) fibers unchanged. Test: two-fiber pipeline, sizes 8 and
   32, assert dispatch-call count `ceil(total/8)+ceil(total/32)`.
4. **L1 formula.** Implement `compute_fiber_morsel_windows` for real: add a plan step summing
   per-fiber write-column bytes (needs column type sizes as a plan array, e.g. `write_sizes:
   <D::Stores as Capacity>::Array<USize>` from `ColumnClassMap`), apply `(l1_usable/sum).clamp
   (min,max) & !3`; add an `L1_USABLE`/clamp source (`RunCfg` const or builder). Test: known
   write-column sizes -> formula match. Flips the slice-1 catalogue-red green.
5. **GATE-2 parallel path.** Wire per-fiber sizes into `run_core_phase` (:1879+) / `worker_main`
   (:820+) via a unit->fiber-size lookup (needs `gate2_fiber[u]` mapping alongside
   `gate2_phase`/`gate2_trunk`). Deferred until 1-4 are stable; `Cfg::MORSEL_SIZE` is the
   parallel fallback until then.

## Const fate + spec alignment

`Cfg::MORSEL_SIZE` is NOT removed: it is the named fallback for resource-only fibers (no write
columns, formula undefined), degenerate zero windows, and the whole parallel path until slice 5;
it re-reads as the effective `MIN_MORSEL` floor. Spec line 1641 ("morsel size is an immediate
constant, no `bl morsel_size`") is satisfied: the per-fiber size is plan-baked at `build()` into
`FiberDispatch::morsel_size` (not re-evaluated per frame), read as a literal into the inner-loop
`len` arithmetic; the adaptive path re-bakes it on plan-recompute (spec :1486).

## Capacity-typing note

`morsel_windows` / `per_fiber_write_bytes` are `<D::Fibers as Capacity>::Array<USize>`;
`write_sizes` is `<D::Stores as Capacity>::Array<USize>`. No new const-generic obligations
beyond the existing `D::Fibers`/`D::Stores` capacity bounds.

## Sequence

Slices 1->2->3->4 single-core first (3 is the structural lift, design-reviewable on its own), then
5 for the parallel path. After 1-4: the tier-1 morsel re-chunk adapt actuation reads/rewrites
`morsel_windows` on the `adapt_reconfigure` trigger; bench-verify it improves the imbalanced
workload -> the catalogued `morsel_rechunk_reduces_idle` / `ema_adaptation_improves_imbalanced`
contracts flip green.
