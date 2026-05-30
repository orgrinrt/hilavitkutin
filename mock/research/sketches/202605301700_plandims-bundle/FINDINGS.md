# Findings: PlanDims bundle de-risk (#652)

**Date:** 2026-05-30
**Hypothesis:** the `PlanDims` bundle (a trait whose associated types are each
a `Capacity`) resolves through fully-generic engine code, including the nested
two-level projection `<<D as PlanDims>::Units as Capacity>::Array<T>` and the
2-D case `D::Edges::Array<D::Units::Array<T>>` over two different dims, with no
`generic_const_exprs` and no `#![feature(...)]` gate.

**Outcome: WORKS.** `cargo +nightly-2026-05-28 run` compiles and runs clean
(`plandims-bundle sketch OK: unit_sum=10`), no feature gate, no solver error,
no GCE overflow.

## What was proven

- A `trait PlanDims { type Units: Capacity; type Edges: Capacity; }` with a
  concrete `DefaultPlanDims` impl (`Units = Dim<13>`, `Edges = Dim<7>`) is
  consumed by a function fully generic over `D: PlanDims`.
- The flat projection `<D::Units as Capacity>::Array<u32>` builds (via both
  `from_fn` and `filled`), writes through `.as_mut()`, reads through
  `.as_ref()`, and sums, all generically.
- The nested 2-D projection `<D::Edges as Capacity>::Array<<D::Units as
  Capacity>::Array<u32>>` (two DIFFERENT dims) builds via `D::Edges::filled`
  and walks generically. This is the load-bearing question the convention memo
  (202605301130, lines 336-350) flagged as the #652 de-risk: it resolves with
  no solver grief.
- Associated-const access `<D::Units as Capacity>::CAP` works generically.
- The only `where` bound the generic consumer needs is
  `<D::Units as Capacity>::Array<u32>: Copy` (so the inner array can be the
  `filled` element type of the outer edges array). This mirrors the
  `C::Array<W>: Copy` propagation finding from the arvo container migration
  (#651): wherever a 2-D array is built via `filled`, the inner array's `Copy`
  bound surfaces on the generic consumer.

## Why this is GCE-free

No const generic sits in dimension/array-length position anywhere in the
generic code. `D`, `D::Units`, `D::Edges` are all TYPES; the array lengths
(`[T; 13]`, `[T; 7]`) are literals inside the concrete `Dim<N>` impl, never a
const expression in the consumer's type position. `generic_const_exprs` has
nothing to evaluate, so the round-trip overflow that ICE'd C2 slice 3 cannot
arise.

## What this unblocks

The #652 engine adoption (collapse the ~209 `const _: Cap` params across
plan/thread/dispatch into one `D: PlanDims` type parameter) is viable as
designed. No fallback to the bare-usize-dim convention (memo T12) is needed.
The remaining #652 work is the (large, architectural) engine refactor itself,
not a feasibility question.

## Caveats not covered by this sketch (handle during the round)

- The real arvo `Capacity` carries `const CAP: Cap` (not `usize`) and the real
  engine dims number ~12, not 2. Scaling the bundle to ~12 assoc types is
  mechanical (no new solver shape); the projection mechanism is identical.
- `Copy` bound propagation across the ~209 sites will surface per-site, same as
  the arvo migration; not a feasibility risk, just volume.
