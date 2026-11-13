//! The proxy numbers: the names of arvo's value page, held here until arvo's own types land.
//!
//! Every item in this module stands in for an item of `arvo-numeral` and is replaced by an alias
//! or a re-export of it at the swap, with the surface fixture (`tests/surface.rs`) building
//! unchanged and the digests unchanged. `Index` and `Count` are this crate's own and stay its own
//! after the swap, with arvo's `USize` beneath them.

mod bool;
mod identity;
mod index;
mod int;
mod lane;
mod usize;

pub use bool::Bool;
pub use identity::{Additive, Identity};
pub use index::{ColumnNo, Count, Index, Ordinal};
pub use int::{Int, Wide};
pub use lane::Lane;
pub use usize::USize;
