# Sketch A3-2: type-level L1-morsel write-collection footprint of Resource<T>

**Date:** 2026-06-21
**Scope:** prove premise A3-sketch-2 from `mock/research/202606210000_a3-resource-footprint-rechart.md`
**Outcome:** WORKS

## Hypothesis

A `Resource<T>`'s L1-morsel write-collection footprint can be computed at the type
level: `Σ` over `T`'s `Seq`/`Map` fields of `N * size_of::<elem>()`, with `Field`
fields contributing 0. Three sub-claims:

1. `Seq<U, N>` / `Map<K, V, N>` expose `N * elem` at the type level.
2. A resource value type reports `Σ` of its `Seq`/`Map` field footprints, `Field` -> 0.
3. This composes into the per-store fold so a `store_sizes`-style walk yields the
   correct per-store L1 footprint.

## Outcome: WORKS

`cargo run` (plain cargo, path-dep'd real `hilavitkutin-api` + git-dep `arvo` on the
workspace's pinned dev rev, nightly-2026-05-28) compiled clean and printed
`A3-2 PROVEN: fixture L1_BYTES = 57 (Seq 22 + Map 35 + Field 0)`. Every assertion
passed. The path dep recompiled `hilavitkutin-api`, confirming `Field`/`Seq`/`Map`
are reachable from outside api (reexported at `hilavitkutin_api::store` and the api
prelude; the engine already imports `store::{...}` directly).

## Proven shape for the A3-redo

### Claim 1: Seq/Map N*elem extraction, and how N is read

`N` is an arvo `Cap` const generic, NOT a bare const generic. Source uses
`Seq<T, const N: Cap>` / `Map<K, V, const N: Cap>` (store.rs:209,227), even though
the spec prose at lines 535-566 still says `const N: usize` (spec text is stale vs
shipped source; `Cap` is correct). `N` is read in const via the named projection
`arvo_tensor::cap_size(N) -> usize`. The inline double-unwrap `N.0.0` is rejected in
const-generic position; `cap_size` is the canonical accepted form. Element byte size
comes from `ColumnValue::BIT_WIDTH` (ceil to bytes via `(bits + 7) / 8`), the same
spec hook A3a used for columns.

```rust
trait CollectionBytes { const BYTES: USize; }
impl<U: ColumnValue, const N: Cap> CollectionBytes for Seq<U, N> {
    const BYTES: USize = USize(cap_size(N) * bytes_of_bits(<U as ColumnValue>::BIT_WIDTH).0);
}
impl<K: ColumnValue, V: ColumnValue, const N: Cap> CollectionBytes for Map<K, V, N> {
    const BYTES: USize = USize(cap_size(N)
        * (bytes_of_bits(<K as ColumnValue>::BIT_WIDTH).0 + bytes_of_bits(<V as ColumnValue>::BIT_WIDTH).0));
}
impl<T: ColumnValue> CollectionBytes for Field<T> { const BYTES: USize = USize::ZERO; }
```

`ColumnValue` is blanket-impl'd for every `Copy + 'static`, so any plain Copy element
satisfies the bound. Seq/Map element types are NOT restricted to the 16-byte limit
that only `Field<T>` carries; the blanket impl means the bound does not exclude larger
elements.

### Claim 2: per-resource-type footprint trait (hand-written)

A `ResourceFootprint { const L1_BYTES: USize }` trait the value type implements. The
impl body sums `CollectionBytes::BYTES` over the markers describing T's fields, Field
adding 0. The fixture (one `Field<u32>`, one `Seq<u16, 11>`, one `Map<u8, u32, 7>`)
yields `0 + 22 + 35 = 57`; a Field-only resource yields 0.

```rust
trait ResourceFootprint { const L1_BYTES: USize; }
impl ResourceFootprint for FixtureRes {
    const L1_BYTES: USize = USize(
        <Field<u32> as CollectionBytes>::BYTES.0
        + <Seq<u16, SEQ_N> as CollectionBytes>::BYTES.0
        + <Map<u8, u32, MAP_N> as CollectionBytes>::BYTES.0);
}
```

Impl is on the concrete value type, one per type: NO coherence problem. A derive could
emit this body mechanically from the struct's field types; a marker-fold over a
cons-list of field markers is the third option. Hand impl proves the arithmetic; the
some-shape leeway covers derive-vs-fold. For the redo a derive is the ergonomic choice.

### Claim 3: composition into the per-store fold (and the A3a coherence fix)

`StoreElemBytes`-shaped per-marker `const BYTES`, disjoint concrete impls (no blanket),
as shipped in `plan/project.rs`. The redo changes only the `Resource` impl:

```rust
impl<T: ColumnValue>       StoreElemBytes for Column<T>   { const BYTES = bytes_of_bits(T::BIT_WIDTH); }
impl<T: ResourceFootprint> StoreElemBytes for Resource<T> { const BYTES = T::L1_BYTES; } // was: T: ColumnValue, size_of::<T>()
impl<T>                    StoreElemBytes for Virtual<T>  { const BYTES = USize::ZERO; }
// Accum<T> -> USize::ZERO for the L1 budget (accum fibers dispatch unit-outer).
```

The `StoreSizes` fold over the `Stores` cons-list is unchanged; only the per-marker
`Resource` value is corrected.

## Hard sub-problems and notes

- **The A3a ColumnValue-bound coherence issue is real and this fixes it.** A3a bounded
  `Resource<T>` on `T: ColumnValue` and used `size_of::<T>()`. A resource value type
  holding `Seq`/`Map` fields, a provider struct, or any non-Copy / >16B value is NOT
  `ColumnValue`, so A3a's `impl ... for Resource<T: ColumnValue>` would not resolve for
  such a `Resource` in the fold. Re-bounding `Resource<T>` on `ResourceFootprint` (which
  the value type implements regardless of Copy/size) removes the wall. `Column<T>` keeps
  the `ColumnValue` bound (column records ARE ColumnValue).
- **N is a Cap; the spec prose is stale.** The redo + any doc CL must use `Cap` /
  `cap_size(N)`, not `const N: usize`. Shipped source is the truth on N's type here.
- **Hand-impl vs derive.** Resource value types must implement `ResourceFootprint`.
  Hand-impl works; a derive is the right ergonomic answer so consumers do not enumerate
  fields by hand. The derive folds the field-layout markers (Field/Seq/Map), distinct
  from the struct's runtime Rust fields (`[u16; 11]`, etc.).
- **Accessibility.** `Field`/`Seq`/`Map` are engine-reachable (`hilavitkutin_api::store`
  + prelude). The redo lives in the engine (`plan/project.rs`) and imports them there,
  as it already imports `Column`/`Resource`/`Virtual`.
- **Read/write split unchanged.** Read-only collections never enter the sum; the
  per-fiber walk iterates the WRITE mask against the per-store size array (A3a/A3b
  plumbing), so this footprint only feeds write positions.

## Reproduce

```
cd mock/research/sketches/202606210100_a3-resource-footprint && cargo run
```
