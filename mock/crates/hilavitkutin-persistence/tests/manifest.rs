//! Manifest / TableMeta / ColumnMeta construction + access.

use hilavitkutin_persistence::{
    ColumnMeta, Manifest, TableMeta, MAX_COLUMNS_PER_TABLE, MAX_TABLES,
};

#[test]
fn manifest_default_is_empty() {
    let m = Manifest::default();
    assert_eq!(m.count, 0);
    assert_eq!(m.tables.len(), MAX_TABLES);
}

#[test]
fn manifest_new_is_empty() {
    let m = Manifest::new();
    assert_eq!(m.count, 0);
}

#[test]
fn table_meta_default_is_empty() {
    let t = TableMeta::default();
    assert_eq!(t.name_hash, 0);
    assert_eq!(t.version, 0);
    assert_eq!(t.row_count, 0);
    assert_eq!(t.column_count, 0);
    assert_eq!(t.columns.len(), MAX_COLUMNS_PER_TABLE);
}

#[test]
fn column_meta_default_is_empty() {
    let c = ColumnMeta::default();
    assert_eq!(c.name_hash, 0);
    assert_eq!(c.bit_width, 0);
    assert_eq!(c.cardinality, 0);
}

#[test]
fn manifest_holds_populated_tables() {
    let mut m = Manifest::new();
    let mut t = TableMeta::default();
    t.name_hash = 0x0AAA_BBBB;
    t.version = 1;
    t.row_count = 42;
    let mut c = ColumnMeta::default();
    c.name_hash = 0x0CCC_DDDD;
    c.bit_width = 32;
    c.cardinality = 10;
    t.columns[0] = c;
    t.column_count = 1;
    m.tables[0] = t;
    m.count = 1;

    assert_eq!(m.count, 1);
    assert_eq!(m.tables[0].name_hash, 0x0AAA_BBBB);
    assert_eq!(m.tables[0].column_count, 1);
    assert_eq!(m.tables[0].columns[0].bit_width, 32);
    assert_eq!(m.tables[0].columns[0].cardinality, 10);
}

#[test]
fn const_limits_match_cl() {
    assert_eq!(MAX_TABLES, 256);
    assert_eq!(MAX_COLUMNS_PER_TABLE, 64);
}
