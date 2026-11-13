//! Two stores and two visiting orders written against the contracts and nothing else, frozen:
//! once landed they change only through a line in `BREAKS.toml`.
//!
//! [`Columns`] keeps every stored column contiguous and is visited [`Forward`]; [`Records`] keeps
//! every record's cells together, last key first, and is visited [`Permuted`]. A law that holds
//! must hold over both.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

mod columns;
mod keys;
mod lanes;
mod records;
mod visit;

pub use columns::{Columns, ColumnsRead, ColumnsWrite};
pub use lanes::{Ahead, Behind, Lanes, Order};
pub use records::{Records, RecordsRead, RecordsWrite};
pub use visit::{Forward, Permuted};
