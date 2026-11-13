//! Lanes, the two kinds a lane is, and the lists a kernel or a close unit
//! declares.

use notko_hlist::{Cons, Empty, List};

use crate::view::{Row, StagedLane};

/// One store of the frame: N records of `Item`, one per row.
///
/// A lane is a unit struct an author declares, so it can be named as a type
/// in a list and passed as a value to a read (`view.get(Pos, row)`).
pub trait Lane: 'static {
    /// What one row of the lane holds.
    type Item: Copy + Send + Sync + 'static;
    /// What every row holds before anything is written.
    const INIT: Self::Item;
}

/// A lane the step advances: committed at the step's start, staged by the
/// step's kernels, committed again at its end. A step kernel names only these.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a frame lane",
    note = "A step kernel's read and write lists name frame lanes only. A landed store is named by a close unit."
)]
pub trait FrameLane: Lane {}

/// A store written only after the step has landed, by a close unit. No step
/// kernel names one.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a landed store",
    note = "A close unit's list names landed stores only. A frame lane is read through the step's view."
)]
pub trait LandedStore: Lane {}

/// A list of frame lanes, which is what a step kernel's read and write lists
/// are. Carries the shape of the staged handles and of a value kernel's
/// returned values for the same list.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a list of frame lanes",
    note = "Every member of a step kernel's read or write list is a frame lane. A landed store in it is the usual cause."
)]
pub trait FrameList: List {
    /// One staged handle per member, in declared order.
    type Parts<'s>;
    /// One value per member, in declared order.
    type Values;

    /// Writes one row of a value kernel's result through the staged handles.
    fn put(parts: &mut Self::Parts<'_>, row: Row, values: Self::Values);
}

impl FrameList for Empty {
    type Parts<'s> = ();
    type Values = ();

    fn put(_parts: &mut (), _row: Row, _values: ()) {}
}

impl<H: FrameLane, T: FrameList> FrameList for Cons<H, T> {
    type Parts<'s> = (StagedLane<'s, H>, T::Parts<'s>);
    type Values = (H::Item, T::Values);

    fn put(parts: &mut Self::Parts<'_>, row: Row, values: Self::Values) {
        parts.0.set(row, values.0);
        T::put(&mut parts.1, row, values.1);
    }
}

/// A list of landed stores, which is what a close unit names.
#[diagnostic::on_unimplemented(
    message = "`{Self}` is not a list of landed stores",
    note = "Every member of a close unit's list is a landed store. A frame lane in it is the usual cause."
)]
pub trait LandedList: List {}

impl LandedList for Empty {}

impl<H: LandedStore, T: LandedList> LandedList for Cons<H, T> {}
