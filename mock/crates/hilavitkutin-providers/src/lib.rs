//! `hilavitkutin-providers`: default Resource-backed providers
//! for the hilavitkutin scheduler.
//!
//! Standalone ecosystem crate. A consumer registers what it ships on
//! the scheduler builder with `.with(...)`, one store at a time or as
//! a Kit. No reverse dep on the engine.
//!
//! The interner surface is [`InternerApi`], [`HasInterner`],
//! [`MemoryArena`], the [`default_interner`] constructor, and the
//! [`InternerKit`] Kit preset that declares the default interner as a
//! `Resource<...>` it owns.

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
pub use crate::interner::{HasInterner, InternerApi, InternerKit, MemoryArena, default_interner};
pub use crate::metrics::{
    CacheResidencyMetrics,
    ChangeClassMetrics,
    CoreIdleTimeMetrics,
    FiberEmaMetrics,
    MemoryWatermarkMetrics,
    MetricsKit,
    PassDurationMetrics,
    PhaseEmaMetrics,
    PredictiveParkingMetrics,
    ThroughputMetrics,
};
pub use crate::storage::{ArenaColumnStorage, StorageError};
