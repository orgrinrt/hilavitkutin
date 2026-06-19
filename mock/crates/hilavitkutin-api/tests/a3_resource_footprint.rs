//! A3-redo-1: `CollectionBytes` per field kind + `ResourceFootprint` total.
//!
//! Canonical R5: a resource value's L1-morsel write footprint is the sum over its
//! `Seq`/`Map` fields of `N * elem` bytes; `Field` scalars contribute 0 (register
//! budget). This pins the trait machinery on a fixture resource with one `Field`,
//! one `Seq`, one `Map`. The derive that emits `ResourceFootprint` from a value
//! type's fields lands in A3-redo-2; here the impl is hand-written.

use arvo::{Cap, USize};
use hilavitkutin_api::footprint::{CollectionBytes, ResourceFootprint};
use hilavitkutin_api::store::{Field, Map, Seq};

const SEQ_N: Cap = arvo_tensor::cap(11);
const MAP_N: Cap = arvo_tensor::cap(7);

// A fixture resource value type. Its field kinds drive the footprint sum.
struct FixtureRes;
impl ResourceFootprint for FixtureRes {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: hand-summed field footprints; tracked: #121
    const L1_BYTES: USize = USize(
        <Field<u32> as CollectionBytes>::BYTES.0
            + <Seq<u16, SEQ_N> as CollectionBytes>::BYTES.0
            + <Map<u8, u32, MAP_N> as CollectionBytes>::BYTES.0,
    );
}

#[test]
fn collection_bytes_per_field_kind() {
    // Seq<u16, 11>: 11 entries * 2 bytes = 22.
    assert_eq!(<Seq<u16, SEQ_N> as CollectionBytes>::BYTES.0, 11 * 2, "Seq = N * elem");
    // Map<u8, u32, 7>: 7 entries * (1 + 4) bytes = 35.
    assert_eq!(<Map<u8, u32, MAP_N> as CollectionBytes>::BYTES.0, 7 * (1 + 4), "Map = N * (k + v)");
    // Field scalar: 0 (register budget, not L1 morsel budget).
    assert_eq!(<Field<u32> as CollectionBytes>::BYTES.0, 0, "Field contributes 0");
}

#[test]
fn resource_footprint_sums_field_kinds() {
    // 0 (Field) + 22 (Seq) + 35 (Map) = 57.
    assert_eq!(
        <FixtureRes as ResourceFootprint>::L1_BYTES.0,
        57,
        "ResourceFootprint sums its field kinds' CollectionBytes (Field adds 0)"
    );
}
