# Sketch A3: per-store size collection + per-fiber write-byte sum

**Date:** 2026-06-20
**Scope:** prove the unproven plumbing for the A3 per-fiber L1 morsel-window formula
**Outcome:** WORKS

## Hypothesis

The A3 per-fiber window formula
`morsel_window[f] = (L1_usable / Sum write_bytes_of_fiber_f).clamp(MIN_MORSEL, MAX_MORSEL) & !3`
needs two pieces of plumbing that did not exist:

1. A per-store element byte-size array, indexed by `Stores` carrier position, sourced
   from `ColumnValue::BIT_WIDTH` (bytes = ceil(bits / 8)), collected the same way the
   access masks are collected.
2. A per-fiber sum of those sizes over the stores set in the fiber's write `AccessMask`.

Hypothesis: both mirror existing machinery (`AccumStoresMask` for the per-store fold,
`project_access_set` / `AccessMask::contains` for the per-fiber sum) and compile + run
correct on a known-types fixture.

## Outcome: WORKS

The sketch (`a3_per_store_size.rs`) compiled against the real engine crates (rlibs in
`mock/target/debug/deps`, nightly-2026-05-28, all features) and ran green (1 passed).

## Proven shape for the A3 implementation

### Part 1: per-store size collection

A per-marker trait with DISJOINT concrete impls (no blanket), mirroring
`plan::project::StoreAccumKind`:

```rust
trait StoreElemBytes { const BYTES: USize; }
const fn bytes_of_bits(bits: USize) -> USize { USize((bits.0 + 7) / 8) }
impl<T: ColumnValue> StoreElemBytes for Column<T>   { const BYTES: USize = bytes_of_bits(<T as ColumnValue>::BIT_WIDTH); }
impl<T: ColumnValue> StoreElemBytes for Resource<T> { const BYTES: USize = bytes_of_bits(<T as ColumnValue>::BIT_WIDTH); }
// Virtual<T> -> USize::ZERO (fired marker, no record bytes); Accum<T>/StagedResource<T> follow Column/Resource.
```

A fold over the `Stores` cons-list, mirroring `AccumStoresMask`, writing each store's
byte size into a position slot instead of setting a mask bit:

```rust
trait StoreSizes<CS: Capacity> { fn fill_sizes(out: &mut [USize], idx: USize); }
impl<CS: Capacity> StoreSizes<CS> for Empty { fn fill_sizes(_, _) {} }
impl<H: StoreElemBytes, T: StoreSizes<CS>, CS: Capacity> StoreSizes<CS> for Cons<H, T> {
    fn fill_sizes(out: &mut [USize], idx: USize) {
        out[idx.0] = <H as StoreElemBytes>::BYTES;
        <T as StoreSizes<CS>>::fill_sizes(out, USize(idx.0 + 1));
    }
}
fn store_sizes<Stores, CS: Capacity>() -> <CS as Capacity>::Array<USize>
where Stores: StoreSizes<CS>, <CS as Capacity>::Array<USize>: Copy { /* filled + slice_mut, like masks_of */ }
```

Sizes land in a `<Stores as Capacity>::Array<USize>`, the same GAT-array shape
`accum_stores_mask` / `masks_of` already build. Slack tail past the live store count
stays zero. This used the runtime `Capacity` surface (`filled`/`as_mut`); a const
version mirrors `BundleMasks`/`masks_of` over `ConstCapacity` (`filled`/`slice_mut`).

### Part 2: per-fiber write-byte sum

Given a fiber's write `AccessMask<CS>` (existing `BundleMasks` writes output, or
`project_access_set` for a fixture) and the size array, sum the set bits:

```rust
fn write_bytes_of_fiber<CS: Capacity>(write_mask: &AccessMask<CS>, sizes: &[USize]) -> USize {
    let mut total = 0; let cap = cap_size(<CS as Capacity>::CAP);
    for i in 0..min(cap, 64, sizes.len()) { if write_mask.contains(USize(i)).0 { total += sizes[i].0; } }
    USize(total)
}
```

The clamp + `& !3` is plain arithmetic on top.

### Fixture verified

Stores `[Column<Uint<14>>, Column<Uint<11>>, Column<Uint<27>>, Resource<Bool>]` at
positions 0..3. Asserted: each `sizes[i]` equals `size_of` its lowered container; slack
slot is zero; per-fiber sums equal the right stores' sizes; the heavier fiber gets a
narrower-or-equal window. All green.

## A3 adoption checklist

- New per-marker trait `StoreElemBytes` (or fold byte size into the `StoreAccumKind`
  file `plan/project.rs`), one disjoint impl per store marker; `Virtual<T>` is zero.
- New fold trait `StoreSizes<CS>` (sibling of `AccumStoresMask`) + `store_sizes::<Stores, CS>()`
  entry (sibling of `accum_stores_mask`); use `ConstCapacity` (mirror `masks_of`) if A3
  wants it const-evaluated.
- Per-fiber sum reuses the per-fiber write `AccessMask` already produced by `BundleMasks`
  (engine) / `BundleProject` (`PlanInputs.writes`). No new projection machinery.
- `L1_USABLE` / `MIN_MORSEL` / `MAX_MORSEL` belong on `RunCfg` as associated consts
  (toolbox-not-policer, consumer-tunable), not hardcoded in the engine.

## Hard sub-problems surfaced

1. Marker coherence: a blanket + specific impl pair would overlap and demand
   specialization (forbidden). DISJOINT concrete impls per marker (the `StoreAccumKind`
   pattern) is the only sound shape; a future marker added without an impl is a compile
   error at the fold (fail-loud).
2. Bytes from `BIT_WIDTH`, not raw `size_of` at the call site: `ColumnValue::BIT_WIDTH`
   is the spec hook (default `size_of * 8`); reading it keeps A3 correct once the
   sub-byte bitpacking layer (#631) makes `BIT_WIDTH` diverge. `(bits + 7) / 8` bridges.
3. 64-store skeleton cap: `AccessMask` backs bits in one `USize` word
   (`StoreCeiling::ASSERT_FITS`, cap 64); the size array and the sum walk share that
   `< 64` ceiling until the arvo-bitmask multi-container swap lands. Not A3-specific.

## Step-reorder verdict

NO REORDER REQUIRED. The per-store size array is a pure function of the `Stores` type
list and `ColumnValue::BIT_WIDTH`; it is independent of `classify_columns` (which runs
at `plan/mod.rs:409`, after `compute_fiber_morsel_windows` at :395) and of per-fiber
column classification entirely. `compute_fiber_morsel_windows` needs only (a) the
per-store size array (computable from `D::Stores` at any point) and (b) the per-fiber
write masks, which the fiber grouping already produces upstream of :395. The current
step order stands.
