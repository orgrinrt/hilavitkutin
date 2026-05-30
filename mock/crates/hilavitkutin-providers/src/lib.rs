//! `hilavitkutin-providers`: default Resource-backed providers
//! for the hilavitkutin scheduler.
//!
//! Standalone ecosystem crate. Consumed by the engine via Kit
//! installation or via direct `builder.add_resource(...)` wiring. No
//! reverse dep on the engine.
//!
//! Ships the interner surface today: [`InternerApi`],
//! [`HasInterner`], [`MemoryArena`], the [`default_interner`]
//! constructor, and the [`InternerKit`] Kit preset that registers
//! the default interner as a `Resource<...>` on the scheduler
//! builder via `add_kit`.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod adapt_ema;
pub mod adapt_kits;
pub mod adapt_wu;
pub mod interner;
pub mod metrics;
pub mod storage;

pub use crate::adapt_ema::{BlendFactor, NORM_1_OVER_8, NORM_7_OVER_8, ema_update};
pub use crate::adapt_kits::{OffAdaptKit, StandardAdaptKit};
pub use crate::adapt_wu::AdaptWu;
pub use crate::interner::{
    HasInterner, InternerApi, InternerKit, MemoryArena, default_interner,
};
pub use crate::metrics::{
    CacheResidencyMetrics, ChangeClassMetrics, CoreIdleTimeMetrics, FiberEmaMetrics, MetricsKit,
    MemoryWatermarkMetrics, PassDurationMetrics, PhaseEmaMetrics, PredictiveParkingMetrics,
    ThroughputMetrics,
};
pub use crate::storage::{ArenaColumnStorage, StorageError};
