//! A3-redo-2a: `#[derive(ResourceFootprint)]` sums a resource value type's
//! `Field`/`Seq`/`Map` field footprints (canonical R5).
//!
//! A resource of plain `Field` scalars derives to a zero L1-morsel footprint
//! (register budget); a resource holding `Seq`/`Map` collections derives to their
//! `N * elem` sum. The derive walks field types syntactically, so the fixtures
//! need only exist as types (never constructed).

#![allow(dead_code)]

use arvo::Cap;
use hilavitkutin_api::footprint::ResourceFootprint;
use hilavitkutin_api::store::{Field, Map, Seq};
use hilavitkutin_api_macros::ResourceFootprint;

const SEQ_N: Cap = arvo_tensor::cap(11);
const MAP_N: Cap = arvo_tensor::cap(7);

#[derive(ResourceFootprint)]
struct FixtureRes {
    scalar: Field<u32>,
    seq: Seq<u16, SEQ_N>,
    map: Map<u8, u32, MAP_N>,
}

#[derive(ResourceFootprint)]
struct ScalarOnly {
    a: Field<u32>,
    b: Field<u16>,
}

#[test]
fn derive_sums_collection_fields() {
    // Seq<u16, 11> = 11 * 2 = 22; Map<u8, u32, 7> = 7 * (1 + 4) = 35; Field = 0.
    assert_eq!(
        <FixtureRes as ResourceFootprint>::L1_BYTES.0,
        57,
        "derive sums Seq (22) + Map (35) + Field (0)"
    );
}

#[test]
fn derive_scalar_only_is_zero() {
    assert_eq!(
        <ScalarOnly as ResourceFootprint>::L1_BYTES.0,
        0,
        "a resource of only Field scalars has zero L1-morsel footprint"
    );
}
