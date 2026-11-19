//! `AccumOverflow`: what a run does when an accumulator refused an append.
//!
//! An append past an accumulator's reserved capacity is never written, under
//! either choice. At the close every run entry has the frame's verdict, an
//! `Outcome<(), AccumRefused>`, and the run configuration's `AccumOverflow`
//! turns it into the run's output. The choice is the consumer's, made at compile
//! time through `RunCfg::Overflow`; `Refuse` is the default.
//!
//! The set is the engine's own fixed vocabulary, two behaviours of one branch at
//! the close, and it is sealed.

use notko::Outcome;

use crate::accum_refused::AccumRefused;
use crate::sealed;

/// What a run does with the frame's verdict on accumulator overflow.
pub trait AccumOverflow: sealed::Sealed + 'static {
    /// What a run entry returns under this choice.
    type Out;

    /// Turn the frame's verdict into the run's output. Called once per frame,
    /// at the close, on the thread that called the run, after every worker has
    /// re-parked.
    fn conclude(verdict: Outcome<(), AccumRefused>) -> Self::Out;
}

/// The default: the run returns the verdict as it is.
#[derive(Copy, Clone, Debug, Default)]
pub struct Refuse;

impl sealed::Sealed for Refuse {}

impl AccumOverflow for Refuse {
    type Out = Outcome<(), AccumRefused>;

    #[inline]
    fn conclude(verdict: Outcome<(), AccumRefused>) -> Self::Out {
        verdict
    }
}

/// The run returns `()` and panics when an accumulator refused an append.
#[derive(Copy, Clone, Debug, Default)]
pub struct Assert;

impl sealed::Sealed for Assert {}

impl AccumOverflow for Assert {
    type Out = ();

    #[inline]
    fn conclude(verdict: Outcome<(), AccumRefused>) -> Self::Out {
        if let Outcome::Err(refused) = verdict {
            panic!(
                "hilavitkutin: an accumulator append went past the reserved capacity under \
                 the assert configuration: {refused:?}"
            );
        }
    }
}
