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
mod tests;
