//! What the trunk's `ColumnValue` (`hilavitkutin-api/src/column_value.rs:25-34`)
//! admits. Each line compiles only if the type is a column value, so the file
//! building is the first finding: the premise "Cell64 is the widest cell and Value
//! is sealed" belongs to the voided contracts crate and not to the trunk.

use s5_cell_format::*;

#[test]
fn a_consumers_own_types_are_column_values_at_their_native_width() {
    assert_eq!(width_of::<Q16>(), 32);
    assert_eq!(width_of::<Q32>(), 64);
    assert_eq!(width_of::<i128>(), 128);
    assert_eq!(width_of::<u8>(), 8);
    assert_eq!(width_of::<Lanes4x16>(), 64);
}

/// The founding spec caps a value at sixteen bytes (consolidation `:2416-2419`,
/// R3, and R4 at `:2421-2426`). The trunk's blanket has no cap, so a 32-byte
/// record is a column value. Per R3 this line should not compile; it does.
#[test]
fn a_value_wider_than_sixteen_bytes_is_admitted_against_r3() {
    assert_eq!(width_of::<Wide32>(), 256);
}

/// The sub-byte hook: BIT_WIDTH reports the container, nothing packs. A packed
/// narrow cell today is a type the consumer writes (`Lanes4x16`), not a width the
/// store packs for it.
#[test]
fn nothing_below_a_byte_is_packed_by_the_store() {
    assert_eq!(width_of::<bool>(), 8);
}

#[test]
fn the_two_lane_formats_compute_the_same_product_where_both_fit() {
    let a: Vec<Q16> = (0..64).map(|i| Q16((i - 32) << 14)).collect();
    let b: Vec<Q16> = (0..64).map(|i| Q16((i % 7 + 1) << 15)).collect();
    let mut o16 = vec![Q16(0); 64];
    s5_mul_q16(&a, &b, &mut o16);
    let a32: Vec<Q32> = a.iter().map(|x| Q32((x.0 as i64) << 16)).collect();
    let b32: Vec<Q32> = b.iter().map(|x| Q32((x.0 as i64) << 16)).collect();
    let mut o32 = vec![Q32(0); 64];
    s5_mul_q32(&a32, &b32, &mut o32);
    for i in 0..64 {
        assert_eq!((o16[i].0 as i64) << 16, o32[i].0, "record {i}");
    }
}
