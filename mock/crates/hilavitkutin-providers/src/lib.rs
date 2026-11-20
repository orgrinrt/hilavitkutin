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
//! `Resource<...>` it owns. Beside it, the [`ArenaColumnStorage`]
//! column store, the [`StaticBumpMemory`] memory provider, and the
//! adapt kits [`StandardAdaptKit`] and [`OffAdaptKit`] with the
//! per-axis metrics resources of [`metrics`]. The default clock and
//! the operating-system memory provider are the engine's, under its
//! `os` feature.
//!
//! Every implementation sits behind a feature the consumer turns on.
//! `static` carries the memory providers over a buffer the consumer
//! hands in, `StaticBumpMemory` and `StaticFreeListMemory`. `portable`
//! carries the rest: the column store, the interner arena and its kit,
//! the adapt unit and kits, and the metrics. A build on default
//! features carries only the contracts `InternerApi` and `HasInterner`.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(feature = "portable")]
pub mod adapt_ema;
#[cfg(feature = "portable")]
pub mod adapt_kits;
#[cfg(feature = "portable")]
pub mod adapt_wu;
pub mod interner;
#[cfg(feature = "static")]
pub mod memory;
#[cfg(feature = "portable")]
pub mod metrics;
#[cfg(feature = "portable")]
pub mod storage;

#[cfg(feature = "portable")]
pub use crate::adapt_ema::{BlendFactor, NORM_1_OVER_8, NORM_7_OVER_8, ema_update};
#[cfg(feature = "portable")]
pub use crate::adapt_kits::{OffAdaptKit, StandardAdaptKit};
#[cfg(feature = "portable")]
pub use crate::adapt_wu::AdaptWu;
pub use crate::interner::{HasInterner, InternerApi};
#[cfg(feature = "portable")]
pub use crate::interner::{InternerKit, MemoryArena, default_interner};
#[cfg(feature = "static")]
pub use crate::memory::{StaticBumpMemory, StaticFreeListMemory};
#[cfg(feature = "portable")]
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
#[cfg(feature = "portable")]
pub use crate::storage::{ArenaColumnStorage, StorageError};
