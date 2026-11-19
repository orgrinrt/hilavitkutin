//! Sketch addition (202611191500), behind the feature `unit-blanket`: a unit
//! as one impl.
//!
//! Without this file a unit that names its lists and nothing else is three
//! impls: `BuilderInput`, `WorkUnit` (the declaration) and `RecordOp` (the
//! body). `Unit` is the three as one trait, and three blankets grant them.
//! It adds no contract of its own: every item of it is an item of one of the
//! three.

use arvo::Bool;

use crate::builder_input::{BuilderInput, UnitDispatch};
use crate::hint::SchedulingHint;
use crate::record_op::RecordOp;
use crate::values::{ReadSet, ReadVals, WriteSet, WriteVals};
use crate::work_unit::WorkUnit;

/// A unit whose body is its value function, declared in one impl.
///
/// The read list, the write list, the hint, and `apply`: the values of the
/// write list at one record, from the values of the read list at that record.
/// The schedule is `Always`.
pub trait Unit: Send + Sync + Sized + 'static {
    /// The stores the function reads.
    type Read: ReadSet;
    /// The stores the function returns values for.
    type Write: WriteSet;
    /// The scheduling hint.
    type Hint: SchedulingHint;

    /// True if the unit's writes commute across record order, as
    /// `WorkUnit::COMMUTATIVE` has it.
    const COMMUTATIVE: Bool = Bool::FALSE;

    /// The values of the write list at one record, from the values of the read
    /// list at that record. Must be a pure function of `read`.
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write>;
}

impl<U: Unit> BuilderInput for U {
    type Dispatch = UnitDispatch<U>;
    type Init = U;
}

impl<U: Unit> WorkUnit for U {
    type Hint = <U as Unit>::Hint;
    type Read = <U as Unit>::Read;
    type Write = <U as Unit>::Write;

    const COMMUTATIVE: Bool = <U as Unit>::COMMUTATIVE;
}

impl<U: Unit> RecordOp for U {
    #[inline(always)]
    fn apply(&self, read: &ReadVals<'_, <U as Unit>::Read>) -> WriteVals<<U as Unit>::Write> {
        <U as Unit>::apply(self, read)
    }
}
