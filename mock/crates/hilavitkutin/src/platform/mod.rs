//! Platform tier dispatch.
//!
//! Selects the active platform backend based on feature flags.
//! Exactly one tier is active per build; the `compile_error!`
//! guards at the crate root enforce mutual exclusion.
//!
//! - `platform-os`: raw syscalls via `libc` (mmap, clock_gettime).
//! - `platform-no-os`: no backend; consumer ships its own
//!   implementations of the `hilavitkutin-api` platform traits.
//!
//! Neither tier spawns threads. The executor behind `ThreadPoolApi`
//! is always the consumer's.

#[cfg(feature = "platform-os")]
mod os;

#[cfg(feature = "platform-os")]
pub use os::{OsClock, OsMemoryProvider};
