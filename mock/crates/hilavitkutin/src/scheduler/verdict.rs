//! The frame's verdict, built once for every run entry.

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use hilavitkutin_api::accum_overflow::AccumOverflow;
use hilavitkutin_api::run_cfg::RunCfg;
use notko::{Maybe, Outcome};

use crate::resource::bindings::FirstRefusal;

/// The run config's output for a closed frame. The verdict is `Ok(())`, or `Err`
/// with the first accumulator, in registration order, that refused an append;
/// the run config's overflow choice concludes it into `Cfg::Out`.
///
/// Called by `run`, `run_fused` and `run_parallel` after the frame's last band,
/// on the thread that owns the bindings with no worker running, so the counts
/// it reads are final for the frame, and a panic under `Assert` is raised with
/// every worker re-parked.
pub(crate) fn frame_verdict<Cfg: RunCfg, B: FirstRefusal>(bindings: &B) -> Cfg::Out {
    let mut seen = <USize as Identity<Additive>>::IDENTITY;
    let verdict = match bindings.first_refusal(&mut seen) {
        Maybe::Is(refused) => Outcome::Err(refused),
        Maybe::Isnt => Outcome::Ok(()),
    };
    <Cfg::Overflow as AccumOverflow>::conclude(verdict)
}
