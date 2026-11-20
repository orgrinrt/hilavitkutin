//! Platform tier dispatch.
//!
//! The `os` feature, off by default, carries the os tier: raw
//! syscalls via `libc` (mmap, clock_gettime) behind `OsMemoryProvider`
//! and `OsClock`. Without it there is no backend, the no_os tier, and
//! the consumer ships its own implementations of the
//! `hilavitkutin-api` platform traits.
//!
//! Neither tier spawns threads. The executor behind `ThreadPoolApi`
//! is always the consumer's.

#[cfg(feature = "os")]
mod os;

#[cfg(feature = "os")]
pub use os::{OsClock, OsMemoryProvider};
