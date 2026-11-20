//! The ratified sixteen-byte limit on a column value (registry row
//! `the_sixteen_byte_limit_is_enforced_at_compile_time_and_forces_interned_identifiers_for_strings_and_paths`)
//! says a type wider than sixteen bytes is refused at compile time on the
//! blanket implementation. This file compiling is the finding: a 64-byte and a
//! 4096-byte array are column values in the shipped crate.

use hilavitkutin_api::ColumnValue;
use hilavitkutin_api::store::Column;

fn is_column_value<T: ColumnValue>() -> usize {
    core::mem::size_of::<T>()
}

#[test]
fn a_value_wider_than_sixteen_bytes_is_accepted() {
    assert_eq!(is_column_value::<[u8; 64]>(), 64);
    assert_eq!(is_column_value::<[u8; 4096]>(), 4096);
    let _registrable = Column::<[u8; 4096]>::start_from([0u8; 4096]);
}
