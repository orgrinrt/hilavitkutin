//! Thread handle: pool thread index (domain 20).
//!
//! `#[repr(transparent)]` newtype over `USize`.

use arvo::USize;

/// Thread index in the pool.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ThreadHandle(pub USize);
