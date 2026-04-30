//! Manifest / TableMeta / ColumnMeta construction + access.

use arvo::USize;
use arvo_hash::ContentHash;
use hilavitkutin_persistence::{
    BitWidth, Cardinality, ColumnCount, ColumnMeta, Manifest, RowCount, SchemaVersion, TableMeta,
    MAX_COLUMNS_PER_TABLE, MAX_TABLES,
};

#[test]
fn manifest_default_is_empty() {
    let m = Manifest::default();
    assert_eq!(m.count, ColumnCount(USize(0)));
    assert_eq!(m.tables.len(), MAX_TABLES);
}

#[test]
fn manifest_new_is_empty() {
    let m = Manifest::new();
    assert_eq!(m.count, ColumnCount(USize(0)));
}

#[test]
fn table_meta_default_is_empty() {
    let t = TableMeta::default();
    assert_eq!(t.name_hash, ContentHash::from_raw(0));
    assert_eq!(t.version, SchemaVersion::new(0));
    assert_eq!(t.row_count, RowCount(USize(0)));
    assert_eq!(t.column_count, ColumnCount(USize(0)));
    assert_eq!(t.columns.len(), MAX_COLUMNS_PER_TABLE);
}

#[test]
fn column_meta_default_is_empty() {
    let c = ColumnMeta::default();
    assert_eq!(c.name_hash, ContentHash::from_raw(0));
    assert_eq!(c.bit_width, BitWidth::new(0));
    assert_eq!(c.cardinality, Cardinality(USize(0)));
}

#[test]
fn manifest_holds_populated_tables() {
    let mut m = Manifest::new();
    let mut t = TableMeta::default();
    t.name_hash = ContentHash::from_raw(0x0AAA_BBBB);
    t.version = SchemaVersion::new(1);
    t.row_count = RowCount(USize(42));
    let mut c = ColumnMeta::default();
    c.name_hash = ContentHash::from_raw(0x0CCC_DDDD);
    c.bit_width = BitWidth::new(32);
    c.cardinality = Cardinality(USize(10));
    t.columns[0] = c;
    t.column_count = ColumnCount(USize(1));
    m.tables[0] = t;
    m.count = ColumnCount(USize(1));

    assert_eq!(m.count, ColumnCount(USize(1)));
    assert_eq!(m.tables[0].name_hash, ContentHash::from_raw(0x0AAA_BBBB));
    assert_eq!(m.tables[0].column_count, ColumnCount(USize(1)));
    assert_eq!(m.tables[0].columns[0].bit_width, BitWidth::new(32));
    assert_eq!(m.tables[0].columns[0].cardinality, Cardinality(USize(10)));
}

#[test]
fn const_limits_match_cl() {
    assert_eq!(MAX_TABLES, 256);
    assert_eq!(MAX_COLUMNS_PER_TABLE, 64);
}
