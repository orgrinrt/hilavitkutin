//! The sealed executor, its error, and its one implementation.
//!
//! `Executor` takes the engine's seal, so only the engine writes an
//! executor: an impl outside it could break the cross-thread ordering
//! protocol the workers rely on.
//!
//! The mainloop body for the executor monomorphises against the
//! `CoreProgram` emitted by the dispatch codegen.

use arvo::USize;

use super::pool_frame::{PoolFrame, WakeStrategy};

/// Sealed executor trait.
///
/// The engine ships exactly one impl, `HybridExecutor`. The trait
/// takes the engine's seal, `sealed::EngineOnly`, a public trait in a
/// private module, so an impl outside the engine cannot bypass the
/// cross-thread atomic-ordering protocol.
///
/// Per-worker mainloop entry: `run(pool, core_id, wake_strategy)`
/// walks the pre-computed `CoreProgram`, dispatching morsels, hitting
/// phase barriers, and observing the shutdown signal between morsels.
/// Returns `Outcome<(), ExecutorError>`; cleanly-shut workers return
/// `Outcome::Ok(())`.
pub trait Executor: crate::sealed::EngineOnly {
    /// Per-worker mainloop. Spawned once per core at
    /// `ThreadPool::build()` time; runs until `pool.shutdown` is set.
    ///
    /// `'arena` is the scratch arena's lifetime (where the
    /// `progress_slots` storage lives); `'frame` is the borrow's
    /// lifetime within this `run` call. The borrow checker enforces
    /// `'arena: 'frame` from the `PoolFrame<'arena, ...>` instantiation.
    fn run<'frame, 'arena, const C: usize, const P: usize>(
        &self,
        pool: core::pin::Pin<&'frame PoolFrame<'arena, C, P>>,
        core_id: USize,
        wake_strategy: &WakeStrategy,
    ) -> notko::Outcome<(), ExecutorError>;
}

/// Executor failure modes. `#[non_exhaustive]` so future variants
/// don't break consumers.
#[non_exhaustive]
#[derive(Debug)]
pub enum ExecutorError {
    /// Worker exited because `pool.shutdown` was set. Not a true
    /// failure; the cleanup path.
    Shutdown,
}

/// The substrate's default `Executor` impl.
///
/// Per-worker mainloop: walks the pre-computed `CoreProgram`,
/// dispatches morsels via the engine's `dispatch::run_fiber<S>` per
/// fiber shape, hits the centralised phase barrier between phase
/// boundaries, observes the shutdown signal between morsels.
///
/// Zero-sized marker, sealed with the engine's seal like the trait it
/// implements.
pub struct HybridExecutor;

impl crate::sealed::EngineOnly for HybridExecutor {}

impl Executor for HybridExecutor {
    #[rustfmt::skip] // keeps the allow on the signature it governs
    fn run<'frame, 'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
        &self,
        pool: core::pin::Pin<&'frame PoolFrame<'arena, C, P>>,
        core_id: USize,
        wake_strategy: &WakeStrategy,
    ) -> notko::Outcome<(), ExecutorError> {
        // FIXME(placeholders_tagged_fe682c10859c): the worker mainloop is not written. It
        // would walk the pre-computed CoreProgram, dispatch morsels,
        // hit the phase barriers and observe the shutdown signal,
        // monomorphised per (C, P, CoreProgram) at scheduler-build
        // time. This body does none of that and returns Ok, so the type
        // and its sealed impls are reachable to downstream construction
        // code.
        let _ = (pool, core_id, wake_strategy);
        notko::Outcome::Ok(())
    }
}
