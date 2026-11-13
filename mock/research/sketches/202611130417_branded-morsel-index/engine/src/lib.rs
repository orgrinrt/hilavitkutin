//! The engine's side of sketch S2: a block kernel is handed a morsel, and
//! `each` on it hands back the morsel's columns and its indices under one
//! brand. An index has no ordering, equality, hash, formatting, arithmetic or
//! integer conversion, it is minted only by `each`, and it is good only for
//! the columns of the `each` that minted it.

#![no_std]
#![forbid(unsafe_code)]

mod index;
mod morsel;

pub use index::{Index, Indices};
pub use morsel::{Block, BlockKernel, Morsel, Morsels, Unequal, run};
