//! s0: the documented wall, verbatim.
//!
//! scheduler/mod.rs:621 says the intended lift is
//! `[AtomicBool; Cfg::MAX_PLAN_AFFECTING_RESOURCES.0]` under
//! generic_const_exprs, and that rustc rejects it ("overly complex
//! generic constant: field access is not supported in generic
//! constants"). This module reproduces that field, against the real
//! `RunCfg` from hilavitkutin-api, to capture the exact current error
//! on nightly-2026-05-28. Expected: FAILS.

use core::marker::PhantomData;
use core::sync::atomic::AtomicBool;

use hilavitkutin_api::RunCfg;

/// The scheduler-shaped struct with the documented intended-lift field.
pub struct Sched0<Cfg: RunCfg> {
    plan_dirty: [AtomicBool; Cfg::MAX_PLAN_AFFECTING_RESOURCES.0],
    _cfg: PhantomData<Cfg>,
}

pub fn run() {
    println!("s0: if this printed, the documented wall no longer exists");
}
