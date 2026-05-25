//! Integration test: `Scheduler::run` no-op transitional body.
//!
//! Pins the post-megaround state: `Scheduler::run` returns
//! `Cfg::Out::default()` without panic. The real morsel loop +
//! per-core dispatch walk + meta-virtual event firing wait on
//! `codegen_fiber` and `codegen_core` real bodies per the
//! `Scheduler::run real morsel loop body` BACKLOG entry. This
//! test pins the no-op contract; real-runtime behaviour tests
//! land alongside the real body.

use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::DefaultRunCfg;
use notko::Outcome;

#[test]
fn scheduler_run_returns_default_outcome() {
    let mut scheduler: Scheduler<DefaultRunCfg> = Scheduler::default();
    let result = scheduler.run();
    assert!(matches!(result, Outcome::Ok(())));
}
