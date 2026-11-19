//! Column-storable value contract.
//!
//! `ColumnValue` certifies a type as storable in a column. The shape is
//! spec-free: a trait-body default const supplies `BIT_WIDTH = size_of * 8`,
//! and an empty blanket impl lets any `Copy + 'static` type inherit it. No
//! `specialization` (the full feature is forbidden) is involved.
//!
//! `BIT_WIDTH` is a future hook for sub-byte bitpacking; nothing reads it today
//! (column reservation sizes by `size_of`). When bitpacking lands, an arvo
//! sub-byte type's packed width is read from arvo's own width-reporting traits
//! at the storage layer that consumes it, not re-encoded as a `ColumnValue`
//! specialisation here. See `#631`.

use arvo::USize;

/// Types that can be stored in a `Column<T>`.
///
/// `BIT_WIDTH` informs the storage engine for bitpacking. The
/// blanket default is `size_of::<Self>() * 8`; sub-byte types
/// specialise.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be stored in a `Column`",
    note = "ColumnValue requires `Copy + 'static`. Reduce or transform the value to a fixed-size `Copy` type. arvo's `UFixed`, `IFixed`, `Bits<N, S>`, `Bool`, and `USize` are valid; `String`, `Vec<T>`, and `Box<T>` are not."
)]
pub trait ColumnValue: Copy + 'static {
    /// Number of storage bits the engine reserves per record. Trait-body
    /// default `size_of::<Self>() * 8`; a future bitpacking layer sources
    /// sub-byte widths from arvo width traits rather than overriding here.
    const BIT_WIDTH: USize = USize(core::mem::size_of::<Self>() * 8);
}

// Empty blanket impl: any `Copy + 'static` type is a column value, inheriting
// the trait-body `BIT_WIDTH` default. No `default const`, no `specialization`.
impl<T: Copy + 'static> ColumnValue for T {}
