//! hilavitkutin — pipeline execution engine.
//!
//! Morsel-driven pipeline engine. Consumes WorkUnit declarations
//! from `hilavitkutin-api`, analyses them into phases/trunks/fibers,
//! compiles dispatch programs, and runs them on a pre-allocated
//! thread pool.
//!
//! `#![no_std]`, no alloc, no `dyn`, no `TypeId`. Platform
//! integration is feature-gated via three mutually exclusive tiers
//! (`platform-os`, `platform-std`, `platform-no-os`).

#![no_std]
#![recursion_limit = "512"]
#![deny(unsafe_op_in_unsafe_fn)]

// Platform tier exclusivity. Exactly one of the three platform
// features must be active at a time. Consumers opting out of the
// default `platform-os` feature must explicitly enable one of the
// alternatives.

#[cfg(all(feature = "platform-os", feature = "platform-std"))]
compile_error!(
    "hilavitkutin: features `platform-os` and `platform-std` are mutually exclusive"
);

#[cfg(all(feature = "platform-os", feature = "platform-no-os"))]
compile_error!(
    "hilavitkutin: features `platform-os` and `platform-no-os` are mutually exclusive"
);

#[cfg(all(feature = "platform-std", feature = "platform-no-os"))]
compile_error!(
    "hilavitkutin: features `platform-std` and `platform-no-os` are mutually exclusive"
);

#[cfg(not(any(feature = "platform-os", feature = "platform-std", feature = "platform-no-os")))]
compile_error!(
    "hilavitkutin: one of `platform-os`, `platform-std`, or `platform-no-os` must be enabled"
);

pub mod adapt;
pub mod dispatch;
pub mod intrinsics;
pub mod platform;
pub mod plan;
pub mod resource;
pub mod scheduler;
pub mod strategy;
pub mod thread;

#[cfg(feature = "platform-os")]
pub use platform::{OsClock, OsMemoryProvider, OsThreadPool};

#[cfg(feature = "platform-std")]
pub use platform::{StdClock, StdMemoryProvider, StdThreadPool};
