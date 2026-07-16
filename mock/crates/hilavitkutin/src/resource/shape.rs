//! Static value-shape descriptor for erased resource addressing.
//!
//! The resource binding records its drained blob's base in erased form
//! (`ErasedResourcePtr`) plus this descriptor. In-process the descriptor
//! witnesses the projection-time backcast (a debug assertion); across a
//! future dynamic-library or wasm extension boundary it is the addressing
//! contract a host and an extension agree on without either side
//! monomorphising the value type. Per-member offsets (the `Decompose`
//! seam fold) join the descriptor when collection-member wiring lands.

use core::mem::{align_of, size_of};

use arvo::USize;

/// The static shape of one resource value: its blob size and alignment.
///
/// Derived from the value type once at compile time via [`of`], recorded
/// next to the erased base at drain, compared against the target type at
/// backcast.
///
/// [`of`]: ValueShape::of
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ValueShape {
    /// Blob size in bytes.
    pub size: USize,
    /// Blob alignment in bytes.
    pub align: USize,
}

impl ValueShape {
    /// The static shape of value type `T`.
    #[inline(always)]
    pub const fn of<T>() -> Self {
        Self {
            // lint:allow(no-bare-numeric) reason: size_of/align_of return usize by contract; tracked: #654
            size: USize(size_of::<T>()),
            // lint:allow(no-bare-numeric) reason: size_of/align_of return usize by contract; tracked: #654
            align: USize(align_of::<T>()),
        }
    }
}
