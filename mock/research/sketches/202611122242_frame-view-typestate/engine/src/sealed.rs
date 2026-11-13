//! The seal on `Stage` and `Kernel`.
//!
//! The runner's one unsafe cut trusts `MASK` and `LEN`. Those are safe
//! associated consts, so an impl outside the engine could state any mask it
//! liked. Sealing keeps every impl in this crate, where each mask is folded
//! from the list it describes: `Sealed` is public in a private module, so a
//! crate outside can name `Stage` and `Kernel` in bounds but cannot implement
//! either.

use notko_hlist::{Cons, Empty};

use crate::kernel::{Block, Value};

pub trait Sealed {}

impl Sealed for Empty {}

impl<H, T> Sealed for Cons<H, T> {}

impl<K> Sealed for Block<K> {}

impl<K> Sealed for Value<K> {}
