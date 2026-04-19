//! Platform tier dispatch.
//!
//! Selects the active platform backend based on feature flags.
//! Exactly one tier is active per build; the `compile_error!`
//! guards at the crate root enforce mutual exclusion.
//!
//! - `platform-os`: raw syscalls via `libc` (mmap, pthread,
//!   clock_gettime).
//! - `platform-std`: `std::alloc` / `std::thread` /
//!   `std::time::Instant` fallback.
//! - `platform-no-os`: no backend; consumer ships its own
//!   implementations of the `hilavitkutin-api` platform traits.

#[cfg(feature = "platform-os")]
mod os;

#[cfg(feature = "platform-std")]
mod std_tier;

#[cfg(feature = "platform-os")]
pub use os::{OsClock, OsMemoryProvider, OsThreadPool};

#[cfg(feature = "platform-std")]
pub use std_tier::{StdClock, StdMemoryProvider, StdThreadPool};
