//! Sketch: a binding table filled from data, beside the typed contract.
//!
//! The typed contract registers stores and units by type, and everything the
//! plan reads is a constant of the build. This sketch asks what a second,
//! data-bound registration would be made of: columns and units declared by
//! values that arrive at run time, held in a table of fixed capacity, with
//! the erased, shape-described addressing chosen for resources on 2 July.
//!
//! Three parts. `Table` is the column side: a declared column is a byte
//! region plus the shape of one element, and a unit gets a typed view of it
//! only after the shape matched. `UnitDecl` and `Bodies` are the unit side: a
//! declared unit names one of the bodies the build compiled and the slots it
//! reads and writes. `phases` is the plan side: one constant function over
//! read and write masks that a typed registration evaluates in a constant and
//! a table evaluates when it is filled.
//!
//! No allocation, no `dyn`, no `TypeId`, and `no_std` outside the tests.
//! Bare primitives are used where the real crates would use arvo's types;
//! the sketch is about the shape of the table and not about its numerics.

#![cfg_attr(not(test), no_std)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::ptr::NonNull;

// ----- the element shape ------------------------------------------------------

/// The shape of one element of a column: what a declaration says about it and
/// what a unit expects of it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Shape {
    pub size:  usize,
    pub align: usize,
}

impl Shape {
    /// The shape of `X`.
    pub const fn of<X>() -> Self {
        Shape {
            size:  size_of::<X>(),
            align: align_of::<X>(),
        }
    }
}

/// A value type every bit pattern of which is a value.
///
/// A shape says how many bytes an element has and how they are aligned. It
/// does not say which bit patterns are values. `bool` and `u8` have one
/// shape, and a `u8` column read as `bool` holds 254 patterns that are not a
/// `bool`. So a table that hands out typed views on a shape match alone may
/// only hand them out for types that have no invalid pattern, and this trait
/// is that restriction.
///
/// ```compile_fail,E0277
/// use data_bound_registration::{Slot, Table};
/// fn view(table: &Table<'_, 4>, slot: Slot) {
///     let _ = table.column::<bool>(slot);
/// }
/// ```
///
/// ```compile_fail,E0277
/// use data_bound_registration::{Slot, Table};
/// fn view(table: &Table<'_, 4>, slot: Slot) {
///     let _ = table.column::<&'static str>(slot);
/// }
/// ```
///
/// # Safety
///
/// Every bit pattern of `size_of::<Self>()` bytes is a valid `Self`, and
/// `Self` has no padding.
pub unsafe trait Plain: Copy + 'static {}

macro_rules! plain {
    ($($t:ty),*) => { $( unsafe impl Plain for $t {} )* };
}
plain!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

// SAFETY: an array of plain values has no padding and no invalid pattern.
unsafe impl<X: Plain, const N: usize> Plain for [X; N] {}

// ----- the column side --------------------------------------------------------

/// What a data source declares for one column.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ColumnDecl {
    /// The shape of one element.
    pub shape:  Shape,
    /// Bytes from one element to the next. Equal to `shape.size` for a packed
    /// column, larger for a member of a wider record.
    pub stride: usize,
}

impl ColumnDecl {
    /// A packed column of `X`.
    pub const fn packed<X: Plain>() -> Self {
        ColumnDecl {
            shape:  Shape::of::<X>(),
            stride: size_of::<X>(),
        }
    }
}

/// The place of a column in a table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Slot(pub usize);

/// Why the table refused.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refused {
    /// The table holds its capacity already.
    Full,
    /// The region does not start on the declared alignment.
    Misaligned,
    /// The declared stride is smaller than the declared element.
    StrideUnderSize,
    /// No column was declared at this slot.
    NoSuchSlot,
    /// The column's declared shape is not the shape asked for.
    Shape {
        declared: Shape,
        asked:    Shape,
    },
    /// The column is strided and a packed view was asked for.
    NotPacked,
    /// A declared unit names a body the build did not compile.
    NoSuchBody,
}

#[derive(Clone, Copy)]
struct Binding {
    base:    NonNull<u8>,
    decl:    ColumnDecl,
    records: usize,
}

/// A table of columns declared at run time, over storage that outlives it.
pub struct Table<'s, const CAP: usize> {
    bound:   [Option<Binding>; CAP],
    len:     usize,
    storage: PhantomData<&'s mut [u8]>,
}

/// A typed view of one declared column.
///
/// Reads and writes are by record position and unchecked, as the block
/// form's accessors were before the revision. Who may write which column
/// when is the plan's to settle and not this view's.
#[derive(Clone, Copy)]
pub struct Column<'t, X: Plain> {
    base:    NonNull<X>,
    records: usize,
    table:   PhantomData<&'t X>,
}

impl<'t, X: Plain> Column<'t, X> {
    /// Records in the column.
    pub const fn records(&self) -> usize {
        self.records
    }

    /// # Safety
    ///
    /// `i < self.records()`, and no unit writes the record meanwhile.
    pub unsafe fn get(&self, i: usize) -> X {
        // SAFETY: inside the region the declaration covered.
        unsafe { *self.base.as_ptr().add(i) }
    }

    /// # Safety
    ///
    /// `i < self.records()`, and no other unit reads or writes the record
    /// meanwhile.
    pub unsafe fn set(&self, i: usize, v: X) {
        // SAFETY: inside the region the declaration covered.
        unsafe { *self.base.as_ptr().add(i) = v }
    }
}

impl<'s, const CAP: usize> Table<'s, CAP> {
    /// An empty table.
    pub const fn new() -> Self {
        Table {
            bound:   [None; CAP],
            len:     0,
            storage: PhantomData,
        }
    }

    /// Declare a column over a byte region. Safe, since a declared column
    /// only ever holds plain values and any bytes are plain values.
    pub fn declare(&mut self, decl: ColumnDecl, region: &'s mut [u8]) -> Result<Slot, Refused> {
        if self.len == CAP {
            return Err(Refused::Full);
        }
        if decl.stride < decl.shape.size || decl.stride == 0 {
            return Err(Refused::StrideUnderSize);
        }
        if !(region.as_ptr() as usize).is_multiple_of(decl.shape.align) {
            return Err(Refused::Misaligned);
        }
        let records = if region.len() < decl.shape.size {
            0
        } else {
            (region.len() - decl.shape.size) / decl.stride + 1
        };
        // SAFETY: a slice's pointer is never null.
        let base = unsafe { NonNull::new_unchecked(region.as_mut_ptr()) };
        let slot = Slot(self.len);
        self.bound[self.len] = Some(Binding {
            base,
            decl,
            records,
        });
        self.len += 1;
        Ok(slot)
    }

    /// A packed, typed view of a declared column, given only on a shape
    /// match. This is the cast back from the erased form.
    pub fn column<X: Plain>(&self, slot: Slot) -> Result<Column<'_, X>, Refused> {
        let Some(Some(b)) = self.bound.get(slot.0) else {
            return Err(Refused::NoSuchSlot);
        };
        let asked = Shape::of::<X>();
        if b.decl.shape != asked {
            return Err(Refused::Shape {
                declared: b.decl.shape,
                asked,
            });
        }
        if b.decl.stride != asked.size {
            return Err(Refused::NotPacked);
        }
        Ok(Column {
            base:    b.base.cast(),
            records: b.records,
            table:   PhantomData,
        })
    }

    /// The read and write mask of a slot list, one bit a slot.
    pub const fn mask(slots: &[Slot]) -> u64 {
        let mut m = 0u64;
        let mut i = 0;
        while i < slots.len() {
            m |= 1u64 << slots[i].0;
            i += 1;
        }
        m
    }
}

impl<'s, const CAP: usize> Default for Table<'s, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

// ----- the unit side ----------------------------------------------------------

/// A morsel of records.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Window {
    pub lo: usize,
    pub hi: usize,
}

/// What a data source declares for one unit: one of the compiled bodies, by
/// its place in `Bodies`, and the slots it reads and writes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnitDecl {
    pub body:  usize,
    pub reads: [Slot; 2],
    pub write: Slot,
}

/// A body the build compiled. A pointer, since which declarations call it is
/// not known until they arrive.
pub type Body<const CAP: usize> =
    for<'t, 's> fn(&'t Table<'s, CAP>, &UnitDecl, Window) -> Result<(), Refused>;

/// The bodies a build offers to declarations.
pub struct Bodies<const CAP: usize, const N: usize>(pub [Body<CAP>; N]);

/// Run declared units over a record range, morsel by morsel, in the order of
/// `phase`. `phase[u]` is the phase of `units[u]`; a lower phase runs first
/// inside each morsel.
pub fn run<const CAP: usize, const N: usize, const U: usize>(
    table: &Table<'_, CAP>,
    bodies: &Bodies<CAP, N>,
    units: &[UnitDecl; U],
    phase: &[u8; U],
    records: usize,
    morsel: usize,
) -> Result<(), Refused> {
    let mut lo = 0;
    while lo < records {
        let hi = if lo + morsel < records { lo + morsel } else { records };
        let mut p = 0u8;
        loop {
            let mut any_later = false;
            let mut u = 0;
            while u < U {
                if phase[u] == p {
                    let Some(body) = bodies.0.get(units[u].body) else {
                        return Err(Refused::NoSuchBody);
                    };
                    body(table, &units[u], Window {
                        lo,
                        hi,
                    })?;
                } else if phase[u] > p {
                    any_later = true;
                }
                u += 1;
            }
            if !any_later {
                break;
            }
            p += 1;
        }
        lo = hi;
    }
    Ok(())
}

// ----- the plan side ----------------------------------------------------------

/// Why no phase assignment exists.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlanRefused {
    /// Two units write one column.
    TwoWriters,
    /// The read-after-write edges form a cycle.
    Cycle,
}

/// Assign each unit a phase from its read and write masks: a unit that reads
/// a column runs in a later phase than the unit that writes it, whatever the
/// order of declaration.
///
/// A constant function, so a typed registration evaluates it in a constant
/// and a table evaluates the same function when its declarations arrive.
pub const fn phases<const U: usize>(
    reads: &[u64; U],
    writes: &[u64; U],
) -> Result<[u8; U], PlanRefused> {
    let mut a = 0;
    while a < U {
        let mut b = a + 1;
        while b < U {
            if writes[a] & writes[b] != 0 {
                return Err(PlanRefused::TwoWriters);
            }
            b += 1;
        }
        a += 1;
    }
    let mut phase = [0u8; U];
    let mut pass = 0;
    loop {
        let mut moved = false;
        let mut r = 0;
        while r < U {
            let mut w = 0;
            while w < U {
                if w != r && reads[r] & writes[w] != 0 && phase[r] <= phase[w] {
                    phase[r] = phase[w] + 1;
                    moved = true;
                }
                w += 1;
            }
            r += 1;
        }
        if !moved {
            return Ok(phase);
        }
        pass += 1;
        if pass > U {
            return Err(PlanRefused::Cycle);
        }
    }
}

/// The masks of declared units, for `phases`.
pub fn masks<const U: usize>(units: &[UnitDecl; U]) -> ([u64; U], [u64; U]) {
    let mut reads = [0u64; U];
    let mut writes = [0u64; U];
    let mut u = 0;
    while u < U {
        reads[u] = (1u64 << units[u].reads[0].0) | (1u64 << units[u].reads[1].0);
        writes[u] = 1u64 << units[u].write.0;
        u += 1;
    }
    (reads, writes)
}

/// The same two units as a typed registration would see them: the masks are
/// constants of the build, and so is the plan.
///
/// A plan over declarations that arrive at run time cannot be a constant:
///
/// ```compile_fail,E0435
/// use data_bound_registration::{masks, phases, UnitDecl};
/// fn plan(units: &[UnitDecl; 2]) {
///     let (reads, writes) = masks(units);
///     const PLAN: Result<[u8; 2], data_bound_registration::PlanRefused> =
///         phases(&reads, &writes);
/// }
/// ```
pub const TYPED_PLAN: Result<[u8; 2], PlanRefused> =
    phases(&[(1 << 0) | (1 << 1), (1 << 3) | (1 << 2)], &[
        1 << 3,
        1 << 4,
    ]);

/// Does not compile: a `bool` view of a declared column. Kept behind a
/// feature so the refusal can be shown again.
#[cfg(feature = "refused-bool-view")]
pub fn refused_bool_view(table: &Table<'_, 4>, slot: Slot) {
    let _ = table.column::<bool>(slot);
}

/// Does not compile: a constant plan over declarations that arrive at run
/// time.
#[cfg(feature = "refused-constant-plan")]
pub fn refused_constant_plan(units: &[UnitDecl; 2]) {
    let (reads, writes) = masks(units);
    const PLAN: Result<[u8; 2], PlanRefused> = phases(&reads, &writes);
    let _ = PLAN;
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Slot = Slot(0);
    const B: Slot = Slot(1);
    const C: Slot = Slot(2);
    const T: Slot = Slot(3);
    const OUT: Slot = Slot(4);
    const RECORDS: usize = 1500;
    const MORSEL: usize = 512;

    fn one(a: u32, b: u32) -> u32 {
        a.wrapping_mul(2654435761).wrapping_add(b.rotate_left(7))
    }

    fn two(t: u32, c: u32) -> u32 {
        (t ^ c.wrapping_mul(2246822519)).rotate_left(13)
    }

    fn body_one(table: &Table<'_, 8>, unit: &UnitDecl, w: Window) -> Result<(), Refused> {
        let a = table.column::<u32>(unit.reads[0])?;
        let b = table.column::<u32>(unit.reads[1])?;
        let t = table.column::<u32>(unit.write)?;
        for i in w.lo .. w.hi {
            // SAFETY: the window is inside the columns, and the plan gave
            // this unit the only write of its column in this phase.
            unsafe { t.set(i, one(a.get(i), b.get(i))) };
        }
        Ok(())
    }

    fn body_two(table: &Table<'_, 8>, unit: &UnitDecl, w: Window) -> Result<(), Refused> {
        let t = table.column::<u32>(unit.reads[0])?;
        let c = table.column::<u32>(unit.reads[1])?;
        let out = table.column::<u32>(unit.write)?;
        for i in w.lo .. w.hi {
            // SAFETY: as `body_one`.
            unsafe { out.set(i, two(t.get(i), c.get(i))) };
        }
        Ok(())
    }

    const BODIES: Bodies<8, 2> = Bodies([body_one, body_two]);

    fn bytes(words: &mut [u32]) -> &mut [u8] {
        // SAFETY: `u32` has no padding and any bytes are `u32` values.
        unsafe { core::slice::from_raw_parts_mut(words.as_mut_ptr().cast::<u8>(), words.len() * 4) }
    }

    struct Store {
        a:   Vec<u32>,
        b:   Vec<u32>,
        c:   Vec<u32>,
        t:   Vec<u32>,
        out: Vec<u32>,
    }

    fn store() -> Store {
        let a: Vec<u32> = (0 .. RECORDS as u32)
            .map(|i| i.wrapping_mul(40503) ^ 0x9E37)
            .collect();
        let b: Vec<u32> = a.iter().map(|x| x.rotate_left(11) ^ 7).collect();
        let c: Vec<u32> = a.iter().map(|x| x.wrapping_add(0x85EB_CA6B)).collect();
        Store {
            a,
            b,
            c,
            t: vec![0; RECORDS],
            out: vec![0; RECORDS],
        }
    }

    fn expected(s: &Store) -> Vec<u32> {
        (0 .. RECORDS)
            .map(|i| two(one(s.a[i], s.b[i]), s.c[i]))
            .collect()
    }

    fn declare_all<'s>(s: &'s mut Store) -> Table<'s, 8> {
        let mut table = Table::<8>::new();
        let u = ColumnDecl::packed::<u32>();
        assert_eq!(table.declare(u, bytes(&mut s.a)), Ok(A));
        assert_eq!(table.declare(u, bytes(&mut s.b)), Ok(B));
        assert_eq!(table.declare(u, bytes(&mut s.c)), Ok(C));
        assert_eq!(table.declare(u, bytes(&mut s.t)), Ok(T));
        assert_eq!(table.declare(u, bytes(&mut s.out)), Ok(OUT));
        table
    }

    const UNITS: [UnitDecl; 2] = [
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
    ];

    #[test]
    fn declared_units_over_declared_columns_compute_the_typed_result() {
        let mut s = store();
        let want = expected(&s);
        let table = declare_all(&mut s);
        let (reads, writes) = masks(&UNITS);
        let phase = phases(&reads, &writes).unwrap();
        assert_eq!(phase, [0, 1]);
        run(&table, &BODIES, &UNITS, &phase, RECORDS, MORSEL).unwrap();
        drop(table);
        assert_eq!(s.out, want);
    }

    #[test]
    fn the_order_of_declaration_does_not_change_the_result() {
        let mut s = store();
        let want = expected(&s);
        let table = declare_all(&mut s);
        let turned = [UNITS[1], UNITS[0]];
        let (reads, writes) = masks(&turned);
        let phase = phases(&reads, &writes).unwrap();
        assert_eq!(phase, [1, 0]);
        run(&table, &BODIES, &turned, &phase, RECORDS, MORSEL).unwrap();
        drop(table);
        assert_eq!(s.out, want);
    }

    #[test]
    fn a_plan_that_ignores_the_edge_computes_something_else() {
        // The case that must fail: with both units in one phase and the
        // reader declared first, the reader sees the column before its
        // writer ran. This is what `phases` is there to prevent.
        let mut s = store();
        let want = expected(&s);
        let table = declare_all(&mut s);
        let turned = [UNITS[1], UNITS[0]];
        run(&table, &BODIES, &turned, &[0, 0], RECORDS, MORSEL).unwrap();
        drop(table);
        assert_ne!(s.out, want);
    }

    #[test]
    fn the_typed_plan_and_the_table_plan_are_one_function() {
        let (reads, writes) = masks(&UNITS);
        assert_eq!(phases(&reads, &writes), TYPED_PLAN);
        assert_eq!(TYPED_PLAN, Ok([0, 1]));
    }

    #[test]
    fn two_writers_and_a_cycle_are_refused() {
        let twice = [UNITS[0], UnitDecl {
            body:  0,
            reads: [A, C],
            write: T,
        }];
        let (r, w) = masks(&twice);
        assert_eq!(phases(&r, &w), Err(PlanRefused::TwoWriters));
        let ring = [
            UnitDecl {
                body:  0,
                reads: [A, OUT],
                write: T,
            },
            UnitDecl {
                body:  1,
                reads: [T, C],
                write: OUT,
            },
        ];
        let (r, w) = masks(&ring);
        assert_eq!(phases(&r, &w), Err(PlanRefused::Cycle));
    }

    #[test]
    fn a_view_of_another_shape_is_refused() {
        let mut s = store();
        let table = declare_all(&mut s);
        assert!(table.column::<u32>(A).is_ok());
        assert_eq!(
            table.column::<u64>(A).err(),
            Some(Refused::Shape {
                declared: Shape::of::<u32>(),
                asked:    Shape::of::<u64>(),
            })
        );
        assert_eq!(
            table.column::<[u8; 4]>(A).err(),
            Some(Refused::Shape {
                declared: Shape::of::<u32>(),
                asked:    Shape::of::<[u8; 4]>(),
            })
        );
        // Same alignment, another size: the case a check of the alignment
        // alone lets through to the stride check.
        assert_eq!(
            table.column::<[u32; 2]>(A).err(),
            Some(Refused::Shape {
                declared: Shape::of::<u32>(),
                asked:    Shape::of::<[u32; 2]>(),
            })
        );
        assert_eq!(
            table.column::<u32>(Slot(5)).err(),
            Some(Refused::NoSuchSlot)
        );
        assert_eq!(
            table.column::<u32>(Slot(99)).err(),
            Some(Refused::NoSuchSlot)
        );
    }

    #[test]
    fn a_shape_cannot_tell_two_types_of_one_layout_apart() {
        // Not a refusal, and recorded as a limit: `f32` has the shape of
        // `u32`, so the table hands the view out. Both are plain, so nothing
        // is unsound, and the meaning of the bits is whatever the reader
        // takes it to be. Telling them apart takes a name in the
        // declaration, which a shape does not carry.
        let mut s = store();
        let table = declare_all(&mut s);
        assert!(table.column::<f32>(A).is_ok());
    }

    #[test]
    fn a_bad_declaration_is_refused() {
        let mut words = [0u32; 8];
        let region = bytes(&mut words);
        let mut table = Table::<1>::new();
        let odd = &mut region[1 ..];
        assert_eq!(
            table.declare(ColumnDecl::packed::<u32>(), odd),
            Err(Refused::Misaligned)
        );
        let mut words = [0u32; 8];
        assert_eq!(
            table.declare(
                ColumnDecl {
                    shape:  Shape::of::<u32>(),
                    stride: 2,
                },
                bytes(&mut words)
            ),
            Err(Refused::StrideUnderSize)
        );
        let mut one = [0u32; 8];
        let mut two = [0u32; 8];
        assert!(
            table
                .declare(ColumnDecl::packed::<u32>(), bytes(&mut one))
                .is_ok()
        );
        assert_eq!(
            table.declare(ColumnDecl::packed::<u32>(), bytes(&mut two)),
            Err(Refused::Full)
        );
    }

    #[test]
    fn a_strided_column_is_not_handed_out_as_packed() {
        let mut words = [0u32; 12];
        let mut table = Table::<1>::new();
        let wide = ColumnDecl {
            shape:  Shape::of::<u32>(),
            stride: 12,
        };
        let slot = table.declare(wide, bytes(&mut words)).unwrap();
        assert_eq!(table.column::<u32>(slot).err(), Some(Refused::NotPacked));
    }

    #[test]
    fn a_unit_naming_no_body_is_refused() {
        let mut s = store();
        let table = declare_all(&mut s);
        let stray = [UnitDecl {
            body:  7,
            reads: [A, B],
            write: T,
        }];
        assert_eq!(
            run(&table, &BODIES, &stray, &[0], RECORDS, MORSEL),
            Err(Refused::NoSuchBody)
        );
    }
}
