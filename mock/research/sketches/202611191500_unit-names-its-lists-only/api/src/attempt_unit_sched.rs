//! Attempt s1, behind the feature `attempt-unit-sched`: the one-impl trait
//! over a schedule parameter, so a unit run on a virtual firing is one impl
//! too. Built once, and its compiler output is kept under `diagnostics/`.

use crate::builder_input::{BuilderInput, UnitDispatch};
use crate::hint::SchedulingHint;
use crate::values::{ReadSet, ReadVals, WriteSet, WriteVals};
use crate::work_unit::Always;

/// `Unit`, with the schedule as a parameter the way `WorkUnit` has it.
pub trait UnitOn<S = Always>: Send + Sync + Sized + 'static {
    /// The stores the function reads.
    type Read: ReadSet;
    /// The stores the function returns values for.
    type Write: WriteSet;
    /// The scheduling hint.
    type Hint: SchedulingHint;

    /// The value function.
    fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write>;
}

impl<U: UnitOn<S>, S> BuilderInput for U {
    type Dispatch = UnitDispatch<U>;
    type Init = U;
}
