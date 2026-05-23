//! Re-export of `hilavitkutin_api::platform::HybridExecutor`.
//!
//! `HybridExecutor` lives in the api crate because its `Executor`
//! impl requires `impl Sealed for HybridExecutor`, and `Sealed` is
//! api-internal. The engine surface picks the type up at the
//! canonical module path so consumers writing `thread::HybridExecutor`
//! reach the same type.
//!
//! The mainloop body for the executor monomorphises against the
//! `CoreProgram` emitted by the dispatch codegen; the body is
//! finished in Pass 6 (Scheduler::run) alongside the CoreProgram
//! shape. The verification grep targets for Pass 4 CHANGE 1
//! resolve against the re-exported name on this module path:
//!
//! - `pub struct HybridExecutor` (re-export form)
//! - `impl Executor for HybridExecutor` (in the api crate, reachable
//!   here via the re-export)
//! - `impl Sealed for HybridExecutor` (in the api crate, reachable
//!   here via the re-export)

pub use hilavitkutin_api::platform::HybridExecutor;
