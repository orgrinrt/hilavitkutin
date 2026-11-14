//! The sequence a unit's records are visited in.

use arvo::USize;

use super::sealed;

/// The sequence the runner visits a unit's records in.
///
/// For every unit list the engine admits, the two visitings leave the same
/// columns and the same flags behind. An accumulator's appends come out in
/// the sequence of the visiting, so a comparison across visitings records
/// them sorted. The trait is sealed.
pub trait Visiting: sealed::OracleSealed + 'static {
    /// The record visited at `step` of `records`, `step` counted from zero
    /// and below `records`.
    fn record(step: USize, records: USize) -> USize;
}

/// Record zero first. The reference.
pub struct Ascending;

/// The last record first.
pub struct Descending;

impl sealed::OracleSealed for Ascending {}
impl sealed::OracleSealed for Descending {}

impl Visiting for Ascending {
    #[inline(always)]
    fn record(step: USize, _records: USize) -> USize {
        step
    }
}

impl Visiting for Descending {
    #[inline(always)]
    fn record(step: USize, records: USize) -> USize {
        USize(records.0 - 1 - step.0)
    }
}
