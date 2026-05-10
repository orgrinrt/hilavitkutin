//! Sketch: PoolFrame lifetime propagation through Scheduler / ExecutionPlan
//! / worker entry, validating Topic 6 axes C + H of round 202605101036.
//!
//! Three escalating experiments in nested modules. Each is a self-contained
//! standalone compile target. See SKETCH.md for full hypothesis and success
//! criteria.

#![no_std]

use core::marker::PhantomData;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

/// Engine-internal pool frame; lives in scheduler-owned arena.
/// Per-pool cache-line invariants from Topic 3 amendment M11.
pub struct PoolFrame<const MAX_CORES: usize, const MAX_PHASES: usize> {
    pub shutdown: AtomicBool,
    pub phase_arrived: AtomicU32,
    pub predicted_wait_ns: [AtomicU32; MAX_PHASES],
    /// Per-core progress counter slots. Workers read/write their own slot
    /// during morsel dispatch.
    pub progress_slots: [AtomicUsize; MAX_CORES],
}

impl<const MAX_CORES: usize, const MAX_PHASES: usize> PoolFrame<MAX_CORES, MAX_PHASES> {
    pub const fn new() -> Self {
        Self {
            shutdown: AtomicBool::new(false),
            phase_arrived: AtomicU32::new(0),
            predicted_wait_ns: [const { AtomicU32::new(0) }; MAX_PHASES],
            progress_slots: [const { AtomicUsize::new(0) }; MAX_CORES],
        }
    }
}

/// Marker: the per-pipeline run-cfg trait (Topic 1 axis 2).
pub trait RunCfg {
    type Ok;
    type Err;
}

/// Sealed executor trait (Topic 6 axis A).
pub trait Sealed {}
pub trait Executor: Sealed {
    fn run<const MAX_CORES: usize, const MAX_PHASES: usize>(
        &self,
        frame: Pin<&PoolFrame<MAX_CORES, MAX_PHASES>>,
        core_id: usize,
    );
}

pub struct HybridExecutor;
impl Sealed for HybridExecutor {}
impl Executor for HybridExecutor {
    fn run<const MAX_CORES: usize, const MAX_PHASES: usize>(
        &self,
        frame: Pin<&PoolFrame<MAX_CORES, MAX_PHASES>>,
        core_id: usize,
    ) {
        // Worker mainloop body: check shutdown, do work, increment progress.
        while !frame.shutdown.load(Ordering::Relaxed) {
            // simulated morsel dispatch
            frame.progress_slots[core_id].fetch_add(1, Ordering::Relaxed);
            // bail out for sketch; real impl loops
            break;
        }
    }
}

// =====================================================================
// Mod 1: Scheduler frame-only. No WorkUnit involvement.
// Validates 'frame propagates through Scheduler<'frame, Cfg, E> +
// ExecutionPlan<'frame, ...> without compile errors.
// =====================================================================

mod scheduler_frame_only {
    use super::*;

    pub struct ExecutionPlan<'frame, const MAX_CORES: usize, const MAX_PHASES: usize> {
        pub frame: Pin<&'frame PoolFrame<MAX_CORES, MAX_PHASES>>,
    }

    pub struct Scheduler<
        'frame,
        Cfg: RunCfg,
        E: Executor,
        const MAX_CORES: usize,
        const MAX_PHASES: usize,
    > {
        pub plan: ExecutionPlan<'frame, MAX_CORES, MAX_PHASES>,
        pub executor: E,
        _cfg: PhantomData<Cfg>,
    }

    impl<
        'frame,
        Cfg: RunCfg,
        E: Executor,
        const MAX_CORES: usize,
        const MAX_PHASES: usize,
    > Scheduler<'frame, Cfg, E, MAX_CORES, MAX_PHASES>
    {
        pub fn new(plan: ExecutionPlan<'frame, MAX_CORES, MAX_PHASES>, executor: E) -> Self {
            Self {
                plan,
                executor,
                _cfg: PhantomData,
            }
        }

        pub fn dispatch_core(&self, core_id: usize) {
            self.executor.run(self.plan.frame, core_id);
        }
    }

    // Compile test: instantiate it.
    pub struct TestCfg;
    impl RunCfg for TestCfg {
        type Ok = ();
        type Err = ();
    }

    pub fn smoke<'frame>(
        frame: Pin<&'frame PoolFrame<4, 8>>,
    ) -> Scheduler<'frame, TestCfg, HybridExecutor, 4, 8> {
        Scheduler::new(ExecutionPlan { frame }, HybridExecutor)
    }
}

// =====================================================================
// Mod 2: WorkUnit declared, but its Ctx does NOT borrow into PoolFrame.
// Validates 'frame stays at the engine boundary: WU impls remain `<>`
// (no lifetime parameter required from consumer).
// =====================================================================

mod with_workunit_no_borrow {
    use super::*;

    pub struct Empty;

    pub trait AccessSet {}
    impl AccessSet for Empty {}

    /// Mirror of hilavitkutin-api's WorkUnit (simplified for sketch).
    /// Note: NO 'frame lifetime here. Consumer Ctx is independent of PoolFrame.
    pub trait WorkUnit {
        type Read: AccessSet;
        type Write: AccessSet;
        type Ctx;
        fn execute(&self, ctx: &Self::Ctx);
    }

    /// Consumer-side: a normal WU. No lifetime in the impl block.
    pub struct MyWu;
    pub struct MyCtx {
        pub some_data: usize,
    }
    impl WorkUnit for MyWu {
        type Read = Empty;
        type Write = Empty;
        type Ctx = MyCtx;
        fn execute(&self, ctx: &Self::Ctx) {
            let _ = ctx.some_data;
        }
    }

    /// Engine-side: Scheduler holds 'frame to PoolFrame AND the WU registry.
    /// WU registry has its own type parameter unrelated to 'frame.
    pub struct Scheduler<
        'frame,
        Cfg: RunCfg,
        E: Executor,
        Wu: WorkUnit,
        const MAX_CORES: usize,
        const MAX_PHASES: usize,
    > {
        pub frame: Pin<&'frame PoolFrame<MAX_CORES, MAX_PHASES>>,
        pub executor: E,
        pub wu: Wu,
        _cfg: PhantomData<Cfg>,
    }

    pub struct TestCfg2;
    impl RunCfg for TestCfg2 {
        type Ok = ();
        type Err = ();
    }

    pub fn smoke<'frame>(
        frame: Pin<&'frame PoolFrame<4, 8>>,
    ) -> Scheduler<'frame, TestCfg2, HybridExecutor, MyWu, 4, 8> {
        Scheduler {
            frame,
            executor: HybridExecutor,
            wu: MyWu,
            _cfg: PhantomData,
        }
    }
}

// =====================================================================
// Mod 3: WorkUnit Ctx exposes a *borrowed* slot from PoolFrame.
// THIS is the risky case. Tests whether the borrow can be threaded
// through Ctx WITHOUT forcing the consumer's `impl WorkUnit` to carry
// a lifetime parameter.
//
// The trick: the borrow lives behind an associated type that the engine
// instantiates with the right lifetime at dispatch time; the WU impl
// declares Ctx as a TYPE that itself happens to borrow.
// =====================================================================

mod with_workunit_borrowed_ctx {
    use super::*;

    pub struct Empty;
    pub trait AccessSet {}
    impl AccessSet for Empty {}

    /// WorkUnit trait. Ctx is generic OVER a lifetime, but the WU impl
    /// hides it via an associated type that's a HRTB-like wrapper.
    pub trait WorkUnit {
        type Read: AccessSet;
        type Write: AccessSet;
        /// Ctx is a type-constructor (CtxFor<'frame> at use site). We
        /// model this with a generic param on the trait impl side, but
        /// AT THE CONSUMER SITE the impl block is `<>`-only.
        type Ctx<'frame>;
        fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>);
    }

    /// A pool-frame-borrowing context handle.
    pub struct ProgressView<'frame> {
        pub slot: &'frame AtomicUsize,
    }

    /// Consumer-side WU. Note: NO `'frame` on the impl block. The GAT
    /// hides the lifetime entirely.
    pub struct AdaptWu;
    impl WorkUnit for AdaptWu {
        type Read = Empty;
        type Write = Empty;
        type Ctx<'frame> = ProgressView<'frame>;

        fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
            let _ = ctx.slot.load(Ordering::Relaxed);
        }
    }

    /// Engine-side dispatcher.
    pub fn dispatch_adapt<'frame, Wu>(
        wu: &Wu,
        frame: Pin<&'frame PoolFrame<4, 8>>,
        core_id: usize,
    ) where
        Wu: for<'a> WorkUnit<Ctx<'a> = ProgressView<'a>>,
    {
        let ctx = ProgressView {
            slot: &frame.progress_slots[core_id],
        };
        wu.execute(&ctx);
    }

    pub fn smoke<'frame>(frame: Pin<&'frame PoolFrame<4, 8>>) {
        dispatch_adapt(&AdaptWu, frame, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::pin::pin;

    #[test]
    fn mod1_compiles_and_runs() {
        let frame: PoolFrame<4, 8> = PoolFrame::new();
        let frame = pin!(frame);
        let sched = scheduler_frame_only::smoke(frame.into_ref());
        sched.dispatch_core(0);
        // smoke: just confirm no UB / panic; full assertions deferred.
    }

    #[test]
    fn mod2_compiles_and_runs() {
        let frame: PoolFrame<4, 8> = PoolFrame::new();
        let frame = pin!(frame);
        let _sched = with_workunit_no_borrow::smoke(frame.into_ref());
        // confirms the scheduler with WU registry compiles; no further work.
    }

    #[test]
    fn mod3_compiles_and_runs() {
        let frame: PoolFrame<4, 8> = PoolFrame::new();
        let frame = pin!(frame);
        with_workunit_borrowed_ctx::smoke(frame.into_ref());
    }
}
