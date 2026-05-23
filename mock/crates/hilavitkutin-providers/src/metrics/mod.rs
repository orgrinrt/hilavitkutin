//! Per-axis metrics Resources (Topic 5 axis F).
//!
//! Each axis from `hilavitkutin_api::adapt` has a matching Resource
//! that carries its runtime snapshot: per-axis measurements plus
//! bool anomaly flags `AdaptWu` sets at `ScheduleEnd`. Observer
//! WorkUnits read these Resources via `ctx.resource::<XMetrics>()`
//! after checking `Virtual<AnomalyFired>`.
//!
//! `MetricsKit` (the next CHANGE in the round) bundles all nine as
//! a Kit recipe; consumers register the bundle with `.add_kit(
//! MetricsKit::default())` or pick individual Resources with
//! `.add_resource(...)`.

pub mod cache_residency;
pub mod change_class;
pub mod core_idle_time;
pub mod fiber_ema;
pub mod kit;
pub mod memory_watermark;
pub mod pass_duration;
pub mod phase_ema;
pub mod predictive_parking;
pub mod throughput;

pub use cache_residency::CacheResidencyMetrics;
pub use change_class::ChangeClassMetrics;
pub use core_idle_time::CoreIdleTimeMetrics;
pub use fiber_ema::FiberEmaMetrics;
pub use kit::MetricsKit;
pub use memory_watermark::MemoryWatermarkMetrics;
pub use pass_duration::PassDurationMetrics;
pub use phase_ema::PhaseEmaMetrics;
pub use predictive_parking::PredictiveParkingMetrics;
pub use throughput::ThroughputMetrics;

/// Internal macro: emit a minimal per-axis Metrics Resource shape.
///
/// Each axis declares:
///
/// - one snapshot field (`last_sample: Nanos` for timing axes,
///   `last_sample: USize` for count axes; the units differ per axis
///   but the wire shape is a single arvo numeric per axis at v0).
/// - one bool anomaly flag `AdaptWu` sets when the axis crosses its
///   threshold.
/// - `BuilderInput<Dispatch = StoreDispatch<Self>>` registration.
///
/// Per-axis field expansion (axis-specific anomaly counters,
/// per-phase / per-fiber breakdown) lands in Pass 7 alongside the
/// bench-validated EMA path.
#[macro_export]
macro_rules! metrics_resource {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        pub struct $name {
            /// Latest sample. Units vary per axis: nanoseconds for
            /// timing axes, record counts for count axes, ratios for
            /// residency axes.
            pub last_sample: arvo::USize,
            /// `AdaptWu` sets `true` at `ScheduleEnd` when this
            /// axis's threshold trips. Observer WUs read after
            /// checking `Virtual<AnomalyFired>`.
            pub anomaly: arvo::Bool,
        }

        impl $name {
            /// Zero-initialised snapshot.
            pub const fn new() -> Self {
                Self {
                    last_sample: arvo::USize::ZERO,
                    anomaly: arvo::Bool::FALSE,
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl hilavitkutin_api::builder_input::BuilderInput for $name {
            type Init = Self;
            type Dispatch = hilavitkutin_api::builder_input::StoreDispatch<Self>;
        }
    };
}
