//! Arm C6: a consumer's cell crossing the extension seam.
//!
//! A slot unit (arm A4) hands an extension a raw column pointer, so host and
//! extension must agree on the cell's layout though they are compiled apart.
//! The trunk's `ColumnValue` asks only `Copy + 'static`
//! (hilavitkutin-api/src/column_value.rs:28), where the founding record kept
//! "repr(C) on stored types" (spec :475). Rust promises nothing about the
//! field order of a default-repr struct between two compilations; the
//! nightly `-Z randomize-layout` flag makes that visible by giving two
//! compilations two layouts.
//!
//! Built and run three ways, each one plain call, outputs beside this file:
//!   rustc --edition 2024 -O layout.rs -o <scratch>/plain
//!   rustc --edition 2024 -O -Zrandomize-layout -Zlayout-seed=1 layout.rs -o <scratch>/seed1
//!   rustc --edition 2024 -O -Zrandomize-layout -Zlayout-seed=2 layout.rs -o <scratch>/seed2
//! The case that must fail: a host built one way reading a column written by
//! an extension built another way. For `Cell` the offsets differ; for
//! `CellC` they cannot.

use core::mem::offset_of;

#[derive(Copy, Clone)]
#[allow(dead_code)]
struct Cell {
    fill:      u32,
    discharge: i16,
    heat:      u8,
    flags:     u8,
    tag:       u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
#[allow(dead_code)]
struct CellC {
    fill:      u32,
    discharge: i16,
    heat:      u8,
    flags:     u8,
    tag:       u64,
}

fn main() {
    println!(
        "Cell  (repr Rust): fill {} discharge {} heat {} flags {} tag {} size {}",
        offset_of!(Cell, fill),
        offset_of!(Cell, discharge),
        offset_of!(Cell, heat),
        offset_of!(Cell, flags),
        offset_of!(Cell, tag),
        size_of::<Cell>()
    );
    println!(
        "CellC (repr C)   : fill {} discharge {} heat {} flags {} tag {} size {}",
        offset_of!(CellC, fill),
        offset_of!(CellC, discharge),
        offset_of!(CellC, heat),
        offset_of!(CellC, flags),
        offset_of!(CellC, tag),
        size_of::<CellC>()
    );
}
