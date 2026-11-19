//! The value function under a name, for within-fiber linear fusion.
//!
//! The engine folds units together at compile time, and a fold cannot see
//! into a closure built inside `execute`. `RecordOp` is the unit's value
//! function under a name on the unit's own type. A unit that offers it
//! writes `execute` as `ctx.each().map(|r| self.apply(r))`, so the function
//! exists once.
//!
//! The trait sits under `WorkUnit` and reads the unit's own lists: it
//! declares no stores, no schedule and no hint of its own. A unit that
//! carries state across records (an accumulator read) has no such function
//! and does not implement it.

use crate::values::{ReadSet, ReadVals, WriteSet, WriteVals};
use crate::work_unit::{Always, WorkUnit};

/// A unit's value function, named so the engine can fold a run of units.
///
/// `apply` takes the values of the unit's read list at one record and
/// returns the values of its write list at that record. It must be a pure
/// function of its reads: the engine composes it with the functions of the
/// units around it and may apply it in any sequence.
///
/// The fold takes a unit whose read list is one `Column` and whose write
/// list is one `Column`. A `RecordOp` over longer lists, a wrapper member or
/// another store runs per unit through its own `execute`.
#[diagnostic::on_unimplemented(
    message = "`{Self}` does not name its value function for fusion",
    note = "Implement `RecordOp` on a WorkUnit whose body is a value function: `fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write>`, and write `execute` as `ctx.each().map(|r| self.apply(r))`."
)]
pub trait RecordOp<S = Always>: WorkUnit<S>
where
    <Self as WorkUnit<S>>::Read: ReadSet,
    <Self as WorkUnit<S>>::Write: WriteSet,
{
    /// The values of the write list at one record, from the values of the
    /// read list at that record. Must be a pure function of `read`.
    fn apply(
        &self,
        read: &ReadVals<'_, <Self as WorkUnit<S>>::Read>,
    ) -> WriteVals<<Self as WorkUnit<S>>::Write>;
}
