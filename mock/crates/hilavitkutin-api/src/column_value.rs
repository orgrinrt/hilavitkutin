//! Column-storable value contract.
//!
//! `ColumnValue` certifies a type as storable in a column. The
//! blanket impl uses `min_specialization` so any `Copy + 'static`
//! type auto-qualifies with `BIT_WIDTH = size_of * 8`. Sub-byte arvo
//! types specialise to their true bit width.

use arvo::newtype::{FBits, IBits};
use arvo::strategy::Hot;
use arvo::ufixed::UFixed;
use arvo::USize;

/// Types that can be stored in a `Column<T>`.
///
/// `BIT_WIDTH` informs the storage engine for bitpacking. The
/// blanket default is `size_of::<Self>() * 8`; sub-byte types
/// specialise.
pub trait ColumnValue: Copy + 'static {
    /// Number of storage bits the engine reserves per record.
    const BIT_WIDTH: USize;
}

impl<T: Copy + 'static> ColumnValue for T {
    default const BIT_WIDTH: USize = USize(core::mem::size_of::<Self>() * 8);
}

// Sub-byte specialisations for arvo `UFixed<I, F, Hot>` at 1, 2,
// and 4-bit widths. The `Hot` strategy is the one whose container
// narrows to byte-aligned widths; these are the packed column
// types the engine bitpacks.

impl ColumnValue for UFixed<{ IBits(1) }, { FBits::ZERO }, Hot> {
    const BIT_WIDTH: USize = USize(1);
}

impl ColumnValue for UFixed<{ IBits(2) }, { FBits::ZERO }, Hot> {
    const BIT_WIDTH: USize = USize(2);
}

impl ColumnValue for UFixed<{ IBits(4) }, { FBits::ZERO }, Hot> {
    const BIT_WIDTH: USize = USize(4);
}
