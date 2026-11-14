//! The memory provider every engine test builds a scheduler with.
//!
//! The scheduler records the base address of every column and accumulator it
//! reserves at build, then moves. `MemoryProviderApi` is an unsafe trait whose
//! promises include that no move of the provider moves a block, so a test
//! cannot keep its buffer inline in its provider; it builds
//! `StaticBumpMemory` over a leaked buffer instead, here.
//!
//! Included by name from each test that needs it (`mod stable_provider;`).
//! Cargo does not build a directory under `tests/` as a test target, so this is
//! not one.

use core::mem::MaybeUninit;

pub use hilavitkutin_providers::StaticBumpMemory;

/// A provider over a fresh leaked buffer of `cap` bytes, each of them `byte`.
/// Every test says what its memory starts as, so one that must not pass on
/// memory that happens to be zero says another byte.
pub fn stable_provider(cap: usize, byte: u8) -> StaticBumpMemory {
    StaticBumpMemory::new(std::vec![MaybeUninit::new(byte); cap].leak())
}
