//! `AccumRefused`: the accumulator that refused an append in a frame.
//!
//! An accumulator append past the reserved capacity is refused rather than
//! asserted, and the refusal is sticky for the frame. At the close the engine
//! walks its accumulators in registration order and hands the first refusing
//! one back through the run config's output, `RunCfg::Out`, which is built
//! from `notko::Outcome<(), AccumRefused>`.

use arvo::USize;
use notko::lend::Exhausted;

/// The first accumulator that refused an append in a frame, and by how much.
///
/// `accum()` is the accumulator's position among the scheduler's accumulators
/// in registration order. `exhausted()` carries `wanted`, every append the
/// frame asked of it, and `had`, its reserved capacity. `wanted` counts what
/// the bodies asked, which does not depend on how the records were split
/// over workers, so the same emitter yields the same value at every worker
/// count.
///
/// The fields are private and `new` builds a value from its two parts. The
/// engine builds the frame's own at the close from the counts it kept, and a
/// caller can build one to compare a run's output against. A value claims
/// nothing beyond its two parts; what makes the run's output true is that the
/// engine counted it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccumRefused {
    accum:     USize,
    exhausted: Exhausted,
}

impl AccumRefused {
    /// Record the refusal of the accumulator at registration position `accum`.
    pub const fn new(accum: USize, exhausted: Exhausted) -> Self {
        Self {
            accum,
            exhausted,
        }
    }

    /// The refusing accumulator's position among the accumulators, in
    /// registration order.
    pub const fn accum(&self) -> USize {
        self.accum
    }

    /// What the frame asked of the accumulator (`wanted`) against its
    /// reserved capacity (`had`).
    pub const fn exhausted(&self) -> Exhausted {
        self.exhausted
    }
}
