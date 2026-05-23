//! Adapt-axis taxonomy: nine sealed BuilderInput configs that drive
//! the adapt subsystem's per-axis sampling + tuning surface.
//!
//! Topic 5 axis A final lock: each axis is its own `BuilderInput<
//! Dispatch = AdaptAxisDispatch<Self>>`, impls sealed `AdaptAxis`,
//! ships `Default` + `Self::off()`. The consumer registers individual
//! axes via `.with(PassDurationAxis::default())`; `StandardAdaptKit`
//! (in hilavitkutin-providers) bundles the default-on combination
//! as a Kit recipe.
//!
//! The nine axes:
//!
//! 1. `PassDurationAxis`: end-to-end pass timing.
//! 2. `PhaseEmaAxis`: per-phase exponential moving average of latency.
//! 3. `FiberEmaAxis`: per-fiber EMA of morsel-completion latency.
//! 4. `ChangeClassAxis`: classify each phase's input drift bucket.
//! 5. `CacheResidencyAxis`: per-column hit / miss ratio.
//! 6. `ThroughputAxis`: per-phase records-per-nanosecond.
//! 7. `PredictiveParkingAxis`: per-phase predicted wait window for the
//!    parking module's tier selection.
//! 8. `MemoryWatermarkAxis`: high-water arena allocation per pass.
//! 9. `CoreIdleTimeAxis`: per-core park-time accumulator.
//!
//! `AdaptAxis` is sealed by the api-internal `sealed::Sealed` mod;
//! every consumer-visible axis must live in this crate so the impl
//! can land. Engine-side modules at `hilavitkutin::adapt::*` re-export
//! the canonical type at the canonical engine path.

use core::marker::PhantomData;

use arvo::{Bool, USize};

use crate::access::Cons;
use crate::builder_input::{BuilderInput, Dispatch};

mod sealed {
    pub trait Sealed {}
}

/// Sealed adapt-axis trait. Topic 5 axis A.
///
/// Every consumer-registered axis impls this; the engine's `AdaptWu`
/// (in hilavitkutin-providers) walks the registered axes via the
/// `Cons<Axis, ...>` typestate the `.with()` chain assembles. The
/// two methods are the runtime-observable knobs: `is_enabled()`
/// gates whether the axis runs at all, `sample_skip()` rate-limits
/// how often the axis's measurement fires.
pub trait AdaptAxis: sealed::Sealed {
    /// Is this axis enabled? `Bool::TRUE` runs the axis on every
    /// `ScheduleEnd`; `Bool::FALSE` skips the axis entirely.
    fn is_enabled(&self) -> Bool;
    /// Sample every Nth `ScheduleEnd` firing. `USize(1)` samples
    /// every pass; `USize(64)` samples one pass in 64.
    fn sample_skip(&self) -> USize;
}

/// Router for adapt-axis inputs. Puts the axis onto the store
/// accumulator (typestate-only; the runtime metrics Resource lives
/// in hilavitkutin-providers and is registered separately).
pub struct AdaptAxisDispatch<A>(PhantomData<A>);

impl<A, Wus, Stores, Platform> Dispatch<Wus, Stores, Platform> for AdaptAxisDispatch<A>
where
    A: 'static,
{
    type NextWus = Wus;
    type NextStores = Cons<A, Stores>;
    type NextPlatform = Platform;
}

macro_rules! axis_config {
    ($name:ident, $default_skip:expr, $doc:literal) => {
        #[doc = $doc]
        pub struct $name {
            /// Run the axis on `ScheduleEnd`?
            pub enabled: Bool,
            /// Sample every Nth pass.
            pub sample_skip: USize,
        }

        impl $name {
            /// Construct with axis enabled at the documented default
            /// sample rate.
            pub const fn new() -> Self {
                Self {
                    enabled: Bool::TRUE,
                    sample_skip: USize($default_skip),
                }
            }

            /// Const constructor that disables the axis entirely.
            /// Equivalent to never registering the axis on the
            /// scheduler builder, but composes inside Kit recipes
            /// that opt out individual axes from a default-on
            /// bundle.
            pub const fn off() -> Self {
                Self {
                    enabled: Bool::FALSE,
                    sample_skip: USize($default_skip),
                }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl sealed::Sealed for $name {}

        impl BuilderInput for $name {
            type Init = Self;
            type Dispatch = AdaptAxisDispatch<Self>;
        }

        impl AdaptAxis for $name {
            fn is_enabled(&self) -> Bool {
                self.enabled
            }
            fn sample_skip(&self) -> USize {
                self.sample_skip
            }
        }
    };
}

axis_config!(
    PassDurationAxis,
    1,
    "End-to-end pass timing axis. Every pass records its total\nduration."
);
axis_config!(
    PhaseEmaAxis,
    1,
    "Per-phase exponential moving average of latency. EMA decay is\n7/8 per the arvo::Norm-based formula in hilavitkutin-providers."
);
axis_config!(
    FiberEmaAxis,
    1,
    "Per-fiber EMA of morsel-completion latency. Same decay shape as\n[`PhaseEmaAxis`]."
);
axis_config!(
    ChangeClassAxis,
    16,
    "Classify each phase's input drift bucket. Drives the\nmorsel-size adapt feedback loop."
);
axis_config!(
    CacheResidencyAxis,
    32,
    "Per-column hit / miss ratio. High miss ratio triggers a\nmorsel-size shrink or a re-plan suggestion."
);
axis_config!(
    ThroughputAxis,
    8,
    "Per-phase records-per-nanosecond. Pairs with [`PhaseEmaAxis`]\nfor latency-vs-throughput tradeoff observability."
);
axis_config!(
    PredictiveParkingAxis,
    1,
    "Per-phase predicted wait window written to\n`PoolFrame.predicted_wait_ns` at `ScheduleEnd`. The parking\nmodule's tier selector reads this on phase entry."
);
axis_config!(
    MemoryWatermarkAxis,
    32,
    "High-water arena allocation per pass. Surfaces growth trends\nso the scheduler can request memory provider expansion ahead of\nactual exhaustion."
);
axis_config!(
    CoreIdleTimeAxis,
    8,
    "Per-core park-time accumulator. Reads `PoolFrame.idle_accumulator`\nand `PoolFrame.park_count` at `ScheduleEnd`."
);
