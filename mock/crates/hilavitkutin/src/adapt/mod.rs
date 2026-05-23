//! Runtime adaptation (domain 22).
//!
//! Per-axis sampling + tuning fires between frames. Topic 5 axis A
//! splits the legacy `AdaptConfig` into nine independent
//! `BuilderInput` configs that the consumer registers individually
//! via `.with(...)`; `StandardAdaptKit` (in hilavitkutin-providers)
//! bundles the default-on combination.
//!
//! The nine axes (each re-exported from `hilavitkutin_api::adapt`):
//!
//! - `pass_duration`: end-to-end pass timing.
//! - `phase_ema`: per-phase EMA of latency.
//! - `fiber_ema`: per-fiber EMA of morsel-completion latency.
//! - `change_class`: input drift classification.
//! - `cache_residency`: per-column hit / miss ratio.
//! - `throughput`: per-phase records-per-nanosecond.
//! - `predictive_parking`: per-phase predicted wait window.
//! - `memory_watermark`: high-water arena allocation per pass.
//! - `core_idle_time`: per-core park-time accumulator.
//!
//! The `arena` module holds the runtime sidecar storage for axis
//! measurements (hot lines per fiber, cold SoA per pass).

pub mod arena;
pub mod cache_residency;
pub mod change_class;
pub mod core_idle_time;
pub mod fiber_ema;
pub mod memory_watermark;
pub mod pass_duration;
pub mod phase_ema;
pub mod predictive_parking;
pub mod throughput;

pub use cache_residency::CacheResidencyAxis;
pub use change_class::ChangeClassAxis;
pub use core_idle_time::CoreIdleTimeAxis;
pub use fiber_ema::FiberEmaAxis;
pub use hilavitkutin_api::adapt::{AdaptAxis, AdaptAxisDispatch};
pub use memory_watermark::MemoryWatermarkAxis;
pub use pass_duration::PassDurationAxis;
pub use phase_ema::PhaseEmaAxis;
pub use predictive_parking::PredictiveParkingAxis;
pub use throughput::ThroughputAxis;

/// Legacy `AdaptMode` alias kept for the strategy module's existing
/// `PhaseStrategy` use site. New code uses per-axis configs directly;
/// this alias retires when the strategy module's adapt-feedback loop
/// migrates to per-axis reads (Pass 7 follow-up).
pub type AdaptMode = crate::strategy::PhaseStrategy;
