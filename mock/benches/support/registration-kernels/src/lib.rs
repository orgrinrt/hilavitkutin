//! Shared kernels for the registration bench.
//!
//! One column workload, bound five ways, each way a module. Two units run over each morsel of a
//! record range: the first writes `t` from `a` and `b`, the second writes
//! `out` from `t` and `c`. Every arm produces the same bytes. What differs is
//! how a unit reaches its columns and how the morsel loop reaches a unit.
//!
//! The typed arm is the shape of typed registration: the unit bodies are
//! monomorphised into the morsel loop and their column pointers were resolved
//! once, when the pipeline was built. The other four arms read a binding table
//! that was filled at run time from declarations the compiler cannot see
//! (they pass through `black_box`, standing in for a file). The table is
//! erased and shape-described: a binding holds a byte pointer, a stride and
//! the shape of one element, and a unit casts it back to a typed pointer only
//! after the shape matched.
//!
//! Every pointer of every arm, the tables included, comes out of one bump
//! arena, as the columns of a real pipeline do. A write to a column and a
//! read of the table therefore share a provenance, and the compiler may not
//! assume the table survived the write. Separate buffers would hand it that
//! assumption and make the table arms look cheaper than they are.

use std::hint::black_box;
use std::mem::{align_of, size_of};

pub use rsb_kernels::fnv1a_column;
use rsb_kernels::{Arena, arena_bytes, stage1, stage2, stage3, stage4};

/// Records in one morsel. Five `u32` columns of 512 records are 10 KiB, which
/// sits in a first-level cache beside the surrounding work.
pub const MORSEL: usize = 512;

/// Columns of the workload.
pub const COLUMNS: usize = 5;

/// Units of the workload.
pub const UNITS: usize = 2;

pub const A: u32 = 0;
pub const B: u32 = 1;
pub const C: u32 = 2;
pub const T: u32 = 3;
pub const OUT: u32 = 4;

/// The first unit's body for one record.
#[inline(always)]
pub fn unit_one(a: u32, b: u32) -> u32 {
    stage3(stage1(a).wrapping_add(stage2(b)))
}

/// The second unit's body for one record.
#[inline(always)]
pub fn unit_two(t: u32, c: u32) -> u32 {
    stage4(t ^ stage2(c))
}

/// The columns of one call, laid out in the one arena.
pub struct Layout {
    /// Records in every column.
    pub records: usize,
    /// The column of each logical name, indexed by `A` through `OUT`.
    pub column:  [*mut u32; COLUMNS],
    /// Room for the binding table and the unit table of the bound arms. The
    /// typed arm reserves it too, so every arm's columns sit at the same
    /// offsets.
    pub tables:  *mut u8,
    _arena:      Arena,
}

/// Bytes reserved for the two tables.
const TABLE_BYTES: usize = 1024;

/// Build the columns from the seeded input.
///
/// The order in which the five columns are cut from the arena turns with the
/// first input byte, so no arm can know at compile time which region holds
/// which column. `a` is the input read as `u32` records, `b` and `c` are
/// derived from it, `t` and `out` start at zero.
pub fn layout(input: &[u8]) -> Layout {
    let records = (input.len() / 4).max(1);
    let arena = Arena::new(arena_bytes(COLUMNS + 1, records) + TABLE_BYTES);
    let tables = arena.alloc_bytes(TABLE_BYTES);
    let turn = input.first().copied().unwrap_or(0) as usize % COLUMNS;
    let mut column = [std::ptr::null_mut::<u32>(); COLUMNS];
    for cut in 0 .. COLUMNS {
        column[(cut + turn) % COLUMNS] = arena.alloc_col::<u32>(records);
    }
    let len = input.len().max(1);
    for i in 0 .. records {
        let at = (i * 4) % len;
        let a = u32::from_le_bytes([
            input[at],
            input[(at + 1) % len],
            input[(at + 2) % len],
            input[(at + 3) % len],
        ]);
        // SAFETY: each column was reserved for `records` values of `u32`.
        unsafe {
            *column[A as usize].add(i) = a;
            *column[B as usize].add(i) = stage1(a) ^ i as u32;
            *column[C as usize].add(i) = stage2(a).wrapping_add(i as u32);
            *column[T as usize].add(i) = 0;
            *column[OUT as usize].add(i) = 0;
        }
    }
    Layout {
        records,
        column,
        tables,
        _arena: arena,
    }
}

/// The checksum every arm reports: FNV-1a over the `out` column.
pub fn checksum(layout: &Layout) -> [u8; 8] {
    fnv1a_column(layout.column[OUT as usize] as *const u32, layout.records)
}

// ----- the erased, shape-described table --------------------------------------

/// The shape of one element of a column.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Shape {
    pub size:  u32,
    pub align: u32,
}

impl Shape {
    /// The shape of `X`, known when the unit is compiled.
    pub const fn of<X>() -> Self {
        Shape {
            size:  size_of::<X>() as u32,
            align: align_of::<X>() as u32,
        }
    }
}

/// One row of the binding table.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Binding {
    pub base:    *mut u8,
    pub stride:  usize,
    pub shape:   Shape,
    pub records: usize,
}

/// What a data source declares for one column.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ColumnDecl {
    pub shape:  Shape,
    pub stride: u32,
}

/// What a data source declares for one unit: which body, and which columns it
/// reads and writes, by slot.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnitDecl {
    pub body:  u32,
    pub reads: [u32; 2],
    pub write: u32,
}

/// The column declarations, opaque to the compiler.
pub fn declared_columns() -> [ColumnDecl; COLUMNS] {
    black_box(
        [ColumnDecl {
            shape:  Shape::of::<u32>(),
            stride: 4,
        }; COLUMNS],
    )
}

/// The unit declarations, opaque to the compiler.
pub fn declared_units() -> [UnitDecl; UNITS] {
    black_box([
        UnitDecl {
            body:  0,
            reads: [A, B],
            write: T,
        },
        UnitDecl {
            body:  1,
            reads: [T, C],
            write: OUT,
        },
    ])
}

/// Write the binding table into the arena from the declarations.
pub fn bind_columns(layout: &Layout) -> *const Binding {
    let table = layout.tables as *mut Binding;
    let decls = declared_columns();
    for slot in 0 .. COLUMNS {
        // SAFETY: the table region holds `COLUMNS` bindings and is aligned to
        // a cache line.
        unsafe {
            *table.add(slot) = Binding {
                base:    layout.column[slot] as *mut u8,
                stride:  decls[slot].stride as usize,
                shape:   decls[slot].shape,
                records: layout.records,
            };
        }
    }
    table as *const Binding
}

/// Look a slot up and cast it back to `X`, or say that it does not fit.
///
/// The slot has to be in the table, the element shape has to be `X`'s, and
/// the stride has to be `X`'s size, since the caller indexes the result as a
/// plain `X` column.
///
/// # Safety
///
/// `table` points at `COLUMNS` bindings.
#[inline(always)]
pub unsafe fn resolve<X>(table: *const Binding, slot: u32) -> Option<*mut X> {
    if slot as usize >= COLUMNS {
        return None;
    }
    // SAFETY: the slot is inside the table.
    let binding = unsafe { &*table.add(slot as usize) };
    if binding.shape != Shape::of::<X>() || binding.stride != size_of::<X>() {
        return None;
    }
    Some(binding.base as *mut X)
}

/// Look a slot up for a strided read: the element shape has to be `X`'s and
/// the stride at least `X`'s size, and the caller steps by the stride it gets.
///
/// # Safety
///
/// `table` points at `COLUMNS` bindings.
#[inline(always)]
pub unsafe fn resolve_strided<X>(table: *const Binding, slot: u32) -> Option<(*mut u8, usize)> {
    if slot as usize >= COLUMNS {
        return None;
    }
    // SAFETY: the slot is inside the table.
    let binding = unsafe { &*table.add(slot as usize) };
    if binding.shape != Shape::of::<X>() || binding.stride < size_of::<X>() {
        return None;
    }
    Some((binding.base, binding.stride))
}

#[cold]
#[inline(never)]
fn refuse() -> ! {
    std::process::abort()
}

/// `resolve`, with a mismatch ending the process. The check stays in the
/// measured path, which is where a sound table has to pay for it.
///
/// # Safety
///
/// `table` points at `COLUMNS` bindings.
#[inline(always)]
pub unsafe fn column<X>(table: *const Binding, slot: u32) -> *mut X {
    // SAFETY: forwarded.
    match unsafe { resolve::<X>(table, slot) } {
        Some(p) => p,
        None => refuse(),
    }
}

// The arms, in the order the module doc gives them. The first two keep their
// units typed. The last three read their units from `unit_table`.
pub mod bound_columns;
pub mod bound_record;
pub mod bound_strided;
pub mod bound_units;
pub mod typed;
mod unit_table;

pub use unit_table::{MorselBody, RecordBody, Tables, UnitRow};
use unit_table::{bind_units, morsel_never, record_never};

#[cfg(test)]
mod tests;
