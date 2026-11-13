//! Shared kernels for the registration bench.
//!
//! One column workload, bound five ways. Two units run over each morsel of a
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

// ----- arm one: typed ----------------------------------------------------------

pub mod typed {
    //! Typed registration: bodies monomorphised into the loop, column
    //! pointers resolved once.
    use super::*;

    pub struct Bound {
        pub a:       *const u32,
        pub b:       *const u32,
        pub c:       *const u32,
        pub t:       *mut u32,
        pub out:     *mut u32,
        pub records: usize,
    }

    pub fn bind(layout: &Layout) -> Bound {
        Bound {
            a:       layout.column[A as usize],
            b:       layout.column[B as usize],
            c:       layout.column[C as usize],
            t:       layout.column[T as usize],
            out:     layout.column[OUT as usize],
            records: layout.records,
        }
    }

    #[inline(always)]
    pub fn run(s: &Bound) {
        // Copied out once, so the loops below hold their pointers the way the
        // table-bound arms hold theirs after a lookup, and the arms differ in
        // where a pointer comes from and in nothing else.
        let (a, b, c, t, out, records) = (s.a, s.b, s.c, s.t, s.out, s.records);
        let mut lo = 0;
        while lo < records {
            let hi = (lo + MORSEL).min(records);
            // SAFETY: every column holds `records` values and `hi <= records`.
            unsafe {
                let mut i = lo;
                while i < hi {
                    *t.add(i) = unit_one(*a.add(i), *b.add(i));
                    i += 1;
                }
                let mut i = lo;
                while i < hi {
                    *out.add(i) = unit_two(*t.add(i), *c.add(i));
                    i += 1;
                }
            }
            lo = hi;
        }
    }
}

// ----- arm two: typed units over table-bound columns ---------------------------

pub mod bound_columns {
    //! The units stay typed and monomorphised. Their columns are declared from
    //! data, and each unit resolves its three through the table once a morsel.
    use super::*;

    pub struct Bound {
        pub table:   *const Binding,
        pub units:   [UnitDecl; UNITS],
        pub records: usize,
    }

    pub fn bind(layout: &Layout) -> Bound {
        Bound {
            table:   bind_columns(layout),
            units:   declared_units(),
            records: layout.records,
        }
    }

    #[inline(always)]
    pub fn run(s: &Bound) {
        let mut lo = 0;
        while lo < s.records {
            let hi = (lo + MORSEL).min(s.records);
            // SAFETY: the table holds `COLUMNS` bindings of `records` values.
            unsafe {
                let one = &s.units[0];
                let a = column::<u32>(s.table, one.reads[0]) as *const u32;
                let b = column::<u32>(s.table, one.reads[1]) as *const u32;
                let t = column::<u32>(s.table, one.write);
                let mut i = lo;
                while i < hi {
                    *t.add(i) = unit_one(*a.add(i), *b.add(i));
                    i += 1;
                }
                let two = &s.units[1];
                let t = column::<u32>(s.table, two.reads[0]) as *const u32;
                let c = column::<u32>(s.table, two.reads[1]) as *const u32;
                let out = column::<u32>(s.table, two.write);
                let mut i = lo;
                while i < hi {
                    *out.add(i) = unit_two(*t.add(i), *c.add(i));
                    i += 1;
                }
            }
            lo = hi;
        }
    }
}

// ----- the unit table of the three remaining arms -----------------------------

/// A unit body called once per morsel.
pub type MorselBody = unsafe fn(*const Binding, *const UnitRow, usize, usize);

/// A unit body called once per record.
pub type RecordBody = unsafe fn(*const Binding, *const UnitRow, usize);

/// One row of the unit table. Which of the two body fields a run uses is the
/// arm's choice; a row is filled for both.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnitRow {
    pub morsel: MorselBody,
    pub record: RecordBody,
    pub reads:  [u32; 2],
    pub write:  u32,
}

/// A run-time table of units over a run-time table of columns.
pub struct Tables {
    pub columns:    *const Binding,
    pub units:      *const UnitRow,
    pub unit_count: usize,
    pub records:    usize,
}

fn bind_units(layout: &Layout, morsel: [MorselBody; UNITS], record: [RecordBody; UNITS]) -> Tables {
    let columns = bind_columns(layout);
    // The unit table follows the binding table in the same arena region.
    // SAFETY: the region is `TABLE_BYTES` long, which holds both tables.
    let units = unsafe { (layout.tables as *mut Binding).add(COLUMNS) } as *mut UnitRow;
    let decls = declared_units();
    for u in 0 .. UNITS {
        let body = decls[u].body as usize % UNITS;
        // SAFETY: inside the table region, aligned for a row.
        unsafe {
            *units.add(u) = UnitRow {
                morsel: morsel[body],
                record: record[body],
                reads:  decls[u].reads,
                write:  decls[u].write,
            };
        }
    }
    Tables {
        columns,
        units: units as *const UnitRow,
        unit_count: black_box(UNITS),
        records: layout.records,
    }
}

unsafe fn record_never(_: *const Binding, _: *const UnitRow, _: usize) {
    refuse()
}

unsafe fn morsel_never(_: *const Binding, _: *const UnitRow, _: usize, _: usize) {
    refuse()
}

// ----- arm three: table-bound units, resolved once a morsel --------------------

pub mod bound_units {
    //! Units and columns both come from the tables. The loop calls a unit's
    //! body through the row's pointer once a morsel, and the body resolves its
    //! columns and then runs a typed loop.
    use super::*;

    unsafe fn one(table: *const Binding, row: *const UnitRow, lo: usize, hi: usize) {
        // SAFETY: the caller hands a row of the unit table and a morsel inside
        // the columns.
        unsafe {
            let row = &*row;
            let a = column::<u32>(table, row.reads[0]) as *const u32;
            let b = column::<u32>(table, row.reads[1]) as *const u32;
            let t = column::<u32>(table, row.write);
            let mut i = lo;
            while i < hi {
                *t.add(i) = unit_one(*a.add(i), *b.add(i));
                i += 1;
            }
        }
    }

    unsafe fn two(table: *const Binding, row: *const UnitRow, lo: usize, hi: usize) {
        // SAFETY: as `one`.
        unsafe {
            let row = &*row;
            let t = column::<u32>(table, row.reads[0]) as *const u32;
            let c = column::<u32>(table, row.reads[1]) as *const u32;
            let out = column::<u32>(table, row.write);
            let mut i = lo;
            while i < hi {
                *out.add(i) = unit_two(*t.add(i), *c.add(i));
                i += 1;
            }
        }
    }

    pub fn bind(layout: &Layout) -> Tables {
        bind_units(layout, [one, two], [record_never, record_never])
    }

    #[inline(always)]
    pub fn run(s: &Tables) {
        let mut lo = 0;
        while lo < s.records {
            let hi = (lo + MORSEL).min(s.records);
            let mut u = 0;
            while u < s.unit_count {
                // SAFETY: `u` is inside the unit table.
                unsafe {
                    let row = s.units.add(u);
                    ((*row).morsel)(s.columns, row, lo, hi);
                }
                u += 1;
            }
            lo = hi;
        }
    }
}

// ----- arm four: table-bound units stepping by the declared stride -------------

pub mod bound_strided {
    //! As `bound_units`, and the body does not assume a column is packed: it
    //! steps by the stride the binding declares. This is the addressing a
    //! column needs when it is a member of a wider shaped store.
    use super::*;

    unsafe fn one(table: *const Binding, row: *const UnitRow, lo: usize, hi: usize) {
        // SAFETY: the caller hands a row of the unit table and a morsel inside
        // the columns.
        unsafe {
            let row = &*row;
            let (Some((a, sa)), Some((b, sb)), Some((t, st))) = (
                resolve_strided::<u32>(table, row.reads[0]),
                resolve_strided::<u32>(table, row.reads[1]),
                resolve_strided::<u32>(table, row.write),
            ) else {
                refuse()
            };
            let mut i = lo;
            while i < hi {
                let x = *(a.add(i * sa) as *const u32);
                let y = *(b.add(i * sb) as *const u32);
                *(t.add(i * st) as *mut u32) = unit_one(x, y);
                i += 1;
            }
        }
    }

    unsafe fn two(table: *const Binding, row: *const UnitRow, lo: usize, hi: usize) {
        // SAFETY: as `one`.
        unsafe {
            let row = &*row;
            let (Some((t, st)), Some((c, sc)), Some((out, so))) = (
                resolve_strided::<u32>(table, row.reads[0]),
                resolve_strided::<u32>(table, row.reads[1]),
                resolve_strided::<u32>(table, row.write),
            ) else {
                refuse()
            };
            let mut i = lo;
            while i < hi {
                let x = *(t.add(i * st) as *const u32);
                let y = *(c.add(i * sc) as *const u32);
                *(out.add(i * so) as *mut u32) = unit_two(x, y);
                i += 1;
            }
        }
    }

    pub fn bind(layout: &Layout) -> Tables {
        bind_units(layout, [one, two], [record_never, record_never])
    }

    #[inline(always)]
    pub fn run(s: &Tables) {
        super::bound_units::run(s)
    }
}

// ----- arm five: table-bound units, resolved at every record -------------------

pub mod bound_record {
    //! The loop calls a unit's body through the row's pointer once a record,
    //! and the body resolves its columns each time. This is what a table costs
    //! when nothing hoists the lookup out of the record loop.
    use super::*;

    unsafe fn one(table: *const Binding, row: *const UnitRow, i: usize) {
        // SAFETY: the caller hands a row of the unit table and a record inside
        // the columns.
        unsafe {
            let row = &*row;
            let a = column::<u32>(table, row.reads[0]) as *const u32;
            let b = column::<u32>(table, row.reads[1]) as *const u32;
            let t = column::<u32>(table, row.write);
            *t.add(i) = unit_one(*a.add(i), *b.add(i));
        }
    }

    unsafe fn two(table: *const Binding, row: *const UnitRow, i: usize) {
        // SAFETY: as `one`.
        unsafe {
            let row = &*row;
            let t = column::<u32>(table, row.reads[0]) as *const u32;
            let c = column::<u32>(table, row.reads[1]) as *const u32;
            let out = column::<u32>(table, row.write);
            *out.add(i) = unit_two(*t.add(i), *c.add(i));
        }
    }

    pub fn bind(layout: &Layout) -> Tables {
        bind_units(layout, [morsel_never, morsel_never], [one, two])
    }

    #[inline(always)]
    pub fn run(s: &Tables) {
        let mut lo = 0;
        while lo < s.records {
            let hi = (lo + MORSEL).min(s.records);
            let mut u = 0;
            while u < s.unit_count {
                let mut i = lo;
                while i < hi {
                    // SAFETY: `u` is inside the unit table, `i` inside the
                    // columns.
                    unsafe {
                        let row = s.units.add(u);
                        ((*row).record)(s.columns, row, i);
                    }
                    i += 1;
                }
                u += 1;
            }
            lo = hi;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seeded(len: usize, seed: u64) -> Vec<u8> {
        let mut x = seed | 1;
        (0 .. len)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                x as u8
            })
            .collect()
    }

    fn all_arms(input: &[u8]) -> [[u8; 8]; 5] {
        let l = layout(input);
        typed::run(&typed::bind(&l));
        let typed = checksum(&l);
        let l = layout(input);
        bound_columns::run(&bound_columns::bind(&l));
        let columns = checksum(&l);
        let l = layout(input);
        bound_units::run(&bound_units::bind(&l));
        let units = checksum(&l);
        let l = layout(input);
        bound_strided::run(&bound_strided::bind(&l));
        let strided = checksum(&l);
        let l = layout(input);
        bound_record::run(&bound_record::bind(&l));
        let record = checksum(&l);
        [typed, columns, units, strided, record]
    }

    #[test]
    fn every_arm_writes_the_same_column() {
        for &len in &[4usize, 2044, 4096, 16384, 65536] {
            for seed in 1 .. 6u64 {
                let sums = all_arms(&seeded(len, seed));
                assert!(
                    sums.iter().all(|s| *s == sums[0]),
                    "len {len} seed {seed}: {sums:?}"
                );
            }
        }
    }

    #[test]
    fn the_checksum_follows_the_input() {
        let a = all_arms(&seeded(4096, 1))[0];
        let b = all_arms(&seeded(4096, 2))[0];
        assert_ne!(a, b);
    }

    #[test]
    fn the_typed_arm_matches_a_plain_scalar_pass() {
        let input = seeded(4096, 9);
        let l = layout(&input);
        let mut expect = Vec::new();
        for i in 0 .. l.records {
            // SAFETY: the columns hold `records` values.
            let (a, b, c) = unsafe {
                (
                    *l.column[A as usize].add(i),
                    *l.column[B as usize].add(i),
                    *l.column[C as usize].add(i),
                )
            };
            expect.push(unit_two(unit_one(a, b), c));
        }
        typed::run(&typed::bind(&l));
        assert_eq!(checksum(&l), fnv1a_column(expect.as_ptr(), expect.len()));
    }

    #[test]
    fn a_binding_of_another_shape_is_refused() {
        let l = layout(&seeded(64, 3));
        let table = bind_columns(&l);
        // SAFETY: the table holds `COLUMNS` bindings.
        unsafe {
            assert!(resolve::<u32>(table, A).is_some());
            assert!(resolve::<u64>(table, A).is_none());
            assert!(resolve::<u16>(table, A).is_none());
            assert!(resolve::<u32>(table, COLUMNS as u32).is_none());
        }
    }

    #[test]
    fn a_wider_stride_is_refused_as_packed_and_taken_as_strided() {
        let l = layout(&seeded(64, 4));
        let table = bind_columns(&l) as *mut Binding;
        // SAFETY: the table holds `COLUMNS` bindings.
        unsafe {
            (*table.add(B as usize)).stride = 12;
            assert!(resolve::<u32>(table, B).is_none());
            assert_eq!(resolve_strided::<u32>(table, B).map(|(_, s)| s), Some(12));
            (*table.add(B as usize)).stride = 2;
            assert!(resolve_strided::<u32>(table, B).is_none());
        }
    }
}
