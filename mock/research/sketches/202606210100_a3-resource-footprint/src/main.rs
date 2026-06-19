//! Sketch A3-2: type-level L1-morsel write-collection footprint of a Resource<T>.
//!
//! Premise (A3-sketch-2): a `Resource<T>`'s L1-morsel write-collection footprint
//! is `Σ` over `T`'s `Seq`/`Map` fields of `N * size_of::<elem>()`, with `Field`
//! fields contributing 0. This sketch proves the three sub-claims against the real
//! `hilavitkutin-api` store markers (`Field`/`Seq`/`Map`) plus arvo `Cap` /
//! `cap_size`:
//!
//! 1. `Seq<U, N>` / `Map<K, V, N>` expose `N * elem` at the type level. `N` is an
//!    arvo `Cap` (NOT a bare const generic), read in const via `cap_size(N)`.
//! 2. A resource value type reports `Σ` of its `Seq`/`Map` field footprints, with
//!    `Field` fields contributing 0, via a hand-written `ResourceFootprint` trait
//!    (`const L1_BYTES: USize`).
//! 3. Composition into the per-store fold: `StoreElemBytes`-shaped per-marker
//!    `const BYTES`, where `Resource<T: ResourceFootprint>` plugs in `T::L1_BYTES`,
//!    Column -> elem size, Field-only resource -> 0, Virtual -> 0.
//!
//! Built with plain cargo against the path-dep'd real crates.

#![feature(adt_const_params)]
#![allow(incomplete_features)]

use arvo::strategy::Identity; // brings USize::ZERO into scope
use arvo::{Cap, USize};
use arvo_tensor::cap_size;
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::store::{Column, Field, Map, Resource, Seq, Virtual};

// ----------------------------------------------------------------------------
// Claim 1: Seq<U, N> / Map<K, V, N> expose N * elem at the type level.
//
// `N` is a `Cap` const-generic (store.rs:209,227 use `const N: Cap`). Reading it
// in const requires the named `cap_size(c: Cap) -> usize` projection: nightly
// rejects the inline double-unwrap `N.0.0` in const-generic position, but a
// `const fn` returning the same value is accepted (arvo cap.rs). The element
// byte size comes from `ColumnValue::BIT_WIDTH` (ceil to bytes), the same spec
// hook A3a used for columns, so this stays correct once sub-byte bitpacking
// makes BIT_WIDTH diverge from `size_of`. Seq/Map element types are NOT bound by
// the 16-byte ColumnValue limit, but ColumnValue is blanket-impl'd for every
// `Copy + 'static`, so the bound is satisfied by any plain Copy element.
// ----------------------------------------------------------------------------

/// Round a bit count up to whole bytes (mirrors project.rs `bytes_of_bits`).
const fn bytes_of_bits(bits: USize) -> USize {
    USize((bits.0 + 7) / 8)
}

/// Type-level byte footprint of a single resource collection field.
trait CollectionBytes {
    const BYTES: USize;
}

impl<U: ColumnValue, const N: Cap> CollectionBytes for Seq<U, N> {
    // N * elem: N read via cap_size(N), elem via ColumnValue::BIT_WIDTH.
    const BYTES: USize =
        USize(cap_size(N) * bytes_of_bits(<U as ColumnValue>::BIT_WIDTH).0);
}

impl<K: ColumnValue, V: ColumnValue, const N: Cap> CollectionBytes for Map<K, V, N> {
    // N * (size_of::<K>() + size_of::<V>()): one key + one value slot per entry.
    const BYTES: USize = USize(
        cap_size(N)
            * (bytes_of_bits(<K as ColumnValue>::BIT_WIDTH).0
                + bytes_of_bits(<V as ColumnValue>::BIT_WIDTH).0),
    );
}

// Field<T> contributes 0 to the L1 morsel budget (register-cached scalar).
impl<T: ColumnValue> CollectionBytes for Field<T> {
    const BYTES: USize = USize::ZERO;
}

// ----------------------------------------------------------------------------
// Claim 2: a resource value type reports Σ of its Seq/Map field footprints,
// Field fields contributing 0, via a hand-written `ResourceFootprint` trait.
//
// Shape proven: a `ResourceFootprint { const L1_BYTES: USize }` trait the value
// type implements by hand. The impl body sums `CollectionBytes::BYTES` over the
// store-layout markers describing T's fields. No coherence issue: the trait is
// implemented on the concrete value type, one impl per type. A derive could
// generate this body mechanically from the field types; a marker-fold over a
// cons-list of the field markers is the other option. Hand impl proves the
// arithmetic; the some-shape leeway covers derive-vs-fold.
// ----------------------------------------------------------------------------

/// L1-morsel write-collection footprint of a resource value type.
trait ResourceFootprint {
    /// `Σ` over the type's `Seq`/`Map` fields of `N * elem`; `Field` -> 0.
    const L1_BYTES: USize;
}

// Fixture resource value type: one Field, one Seq, one Map field.
// Non-power-of-two widths and caps per the arvo exact-width discipline.
#[derive(Clone, Copy)]
struct FixtureRes {
    // Field<u32>: a <=16B scalar, register-cached, contributes 0 to L1.
    _scalar: u32,
    // Seq<u16, 11>: 11 entries * 2 bytes = 22 bytes.
    _seq: [u16; 11],
    // Map<u8, u32, 7>: 7 entries * (1 + 4) = 35 bytes.
    _map: [(u8, u32); 7],
}

const SEQ_N: Cap = arvo_tensor::cap(11);
const MAP_N: Cap = arvo_tensor::cap(7);

impl ResourceFootprint for FixtureRes {
    // Σ of the Seq + Map field footprints; the Field field adds 0.
    const L1_BYTES: USize = USize(
        <Field<u32> as CollectionBytes>::BYTES.0
            + <Seq<u16, SEQ_N> as CollectionBytes>::BYTES.0
            + <Map<u8, u32, MAP_N> as CollectionBytes>::BYTES.0,
    );
}

// A Field-only resource: footprint must be 0.
#[derive(Clone, Copy)]
struct ScalarOnlyRes {
    _a: u32,
    _b: u16,
}

impl ResourceFootprint for ScalarOnlyRes {
    const L1_BYTES: USize =
        USize(<Field<u32> as CollectionBytes>::BYTES.0 + <Field<u16> as CollectionBytes>::BYTES.0);
}

// ----------------------------------------------------------------------------
// Claim 3: composition into the per-store fold.
//
// `StoreElemBytes`-shaped per-marker `const BYTES`, with DISJOINT concrete impls
// (no blanket), mirroring the shipped project.rs. The KEY DIFFERENCE from A3a:
//   - A3a bounded `Resource<T>` on `T: ColumnValue` and used `size_of::<T>()`.
//     That is wrong: a Resource value is a struct of Field/Seq/Map fields, not a
//     single ColumnValue scalar.
//   - Here `Resource<T>` is bounded on `T: ResourceFootprint` and uses
//     `T::L1_BYTES`. Column stays on `T: ColumnValue` (per-record stride).
//     Virtual -> 0.
//
// COHERENCE NOTE: A3a bounded ALL markers on ColumnValue. That breaks for
// resources whose value type holds non-ColumnValue collection element types or
// provider values: `Resource<SomeProviderStruct>` is not `ColumnValue` (not
// Copy, or >16B), so the A3a `impl StoreElemBytes for Resource<T: ColumnValue>`
// would not apply and the fold would fail to resolve. Re-bounding Resource on
// `ResourceFootprint` (which the value type implements regardless of Copy/size)
// is the fix. Column keeps the ColumnValue bound (column records ARE ColumnValue).
// ----------------------------------------------------------------------------

trait StoreElemBytes {
    const BYTES: USize;
}

impl<T: ColumnValue> StoreElemBytes for Column<T> {
    const BYTES: USize = bytes_of_bits(<T as ColumnValue>::BIT_WIDTH);
}

impl<T: ResourceFootprint> StoreElemBytes for Resource<T> {
    const BYTES: USize = T::L1_BYTES;
}

impl<T> StoreElemBytes for Virtual<T> {
    const BYTES: USize = USize::ZERO;
}

fn main() {
    // Claim 1: Seq/Map N*elem extraction.
    assert_eq!(<Seq<u16, SEQ_N> as CollectionBytes>::BYTES.0, 11 * 2, "Seq N*elem");
    assert_eq!(
        <Map<u8, u32, MAP_N> as CollectionBytes>::BYTES.0,
        7 * (1 + 4),
        "Map N*(k+v)"
    );
    assert_eq!(<Field<u32> as CollectionBytes>::BYTES.0, 0, "Field contributes 0");

    // Claim 2: Σ over the fixture's Seq/Map fields, Field -> 0.
    let expected = 11 * 2 + 7 * (1 + 4); // 22 + 35 = 57
    assert_eq!(<FixtureRes as ResourceFootprint>::L1_BYTES.0, expected, "fixture Σ");
    assert_eq!(<ScalarOnlyRes as ResourceFootprint>::L1_BYTES.0, 0, "field-only -> 0");

    // Claim 3: per-store fold composition.
    assert_eq!(<Resource<FixtureRes> as StoreElemBytes>::BYTES.0, 57, "Resource footprint");
    assert_eq!(<Resource<ScalarOnlyRes> as StoreElemBytes>::BYTES.0, 0, "field-only Resource -> 0");
    assert_eq!(<Column<u32> as StoreElemBytes>::BYTES.0, 4, "Column -> elem stride");
    assert_eq!(<Virtual<()> as StoreElemBytes>::BYTES.0, 0, "Virtual -> 0");

    println!("A3-2 PROVEN: fixture L1_BYTES = {} (Seq 22 + Map 35 + Field 0)", expected);
}
