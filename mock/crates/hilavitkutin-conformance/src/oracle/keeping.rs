//! How a frame is kept: faithfully, or wrongly in one named way.
//!
//! The members of [`Keeping`] are constants a frame folds over, so the
//! faithful runner carries no branch a mutant would take.

use arvo::Bool;

use super::sealed;

/// How the runner keeps a frame.
///
/// [`Faithful`] keeps it as the frame's meaning says. Every other member,
/// under [`mutant`](super::mutant), keeps it wrongly in one named way, and is
/// there to be run through the laws and seen to fail. The trait is sealed.
pub trait Keeping: sealed::OracleSealed + 'static {
    /// A read of a buffered column sees the live side.
    const READS_LIVE: Bool = Bool(false);
    /// A write to a buffered column lands on the frozen side, in place.
    const WRITES_FROZEN: Bool = Bool(false);
    /// The boundary brings in a live side nobody wrote.
    const BRINGS_UNWRITTEN: Bool = Bool(false);
    /// The last record visited is left out.
    const SKIPS_LAST: Bool = Bool(false);
    /// An accumulator is not emptied at the start of a frame.
    const KEEPS_APPENDS: Bool = Bool(false);
    /// A virtual stays fired across the boundary.
    const KEEPS_FLAGS: Bool = Bool(false);
    /// An `On` unit runs whether its virtual fired or not.
    const RUNS_UNTRIGGERED: Bool = Bool(false);
}

/// The frame kept as its meaning says.
pub struct Faithful;

impl sealed::OracleSealed for Faithful {}
impl Keeping for Faithful {}
