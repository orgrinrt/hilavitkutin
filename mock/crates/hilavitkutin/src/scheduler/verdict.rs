//! The frame's verdict, built once for every run entry.

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use hilavitkutin_api::run_cfg::RunCfg;
use notko::{Maybe, Outcome};

use crate::resource::bindings::FirstRefusal;

/// The run config's output for a closed frame: `Ok(())`, or `Err` with the first
/// accumulator, in registration order, that refused an append.
///
/// Called by `run`, `run_fused` and `run_parallel` after the frame's last band,
/// on the thread that owns the bindings with no worker running, so the counts
/// it reads are final for the frame.
pub(crate) fn frame_verdict<Cfg: RunCfg, B: FirstRefusal>(bindings: &B) -> Cfg::Out {
    let mut seen = <USize as Identity<Additive>>::IDENTITY;
    match bindings.first_refusal(&mut seen) {
        Maybe::Is(refused) => Cfg::Out::from(Outcome::Err(refused)),
        Maybe::Isnt => Cfg::Out::from(Outcome::Ok(())),
    }
}
