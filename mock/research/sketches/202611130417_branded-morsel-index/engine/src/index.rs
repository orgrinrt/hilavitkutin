//! The branded index and the iterator that mints it.

use core::marker::PhantomData;

/// The brand: a lifetime the index and the block of one `each` share.
///
/// Invariant, through a function pointer that takes and returns it, so the
/// compiler can neither lengthen nor shorten it to meet another `each`'s.
/// A function pointer and not a `Cell` keeps the index `Send` and `Sync`.
#[cfg(not(feature = "mutant-covariant"))]
pub(crate) type Brand<'b> = PhantomData<fn(&'b ()) -> &'b ()>;

/// The mutant: a covariant brand, which the compiler may shorten.
#[cfg(feature = "mutant-covariant")]
pub(crate) type Brand<'b> = PhantomData<&'b ()>;

/// One record of a morsel, as an opaque brand.
///
/// Built only by `Indices`, which only `Block::indices` makes. Its fields are
/// private and it derives nothing past `Clone` and `Copy`: no `PartialEq`,
/// `PartialOrd`, `Hash`, `Debug`, arithmetic, or conversion to or from an
/// integer. The perimeter is the type's own surface; what an author can learn
/// from the sequence `Indices` yields in is outside it (FINDINGS.md).
#[cfg(not(feature = "mutant-bare"))]
#[derive(Clone, Copy)]
pub struct Index<'b> {
    at:    usize,
    brand: Brand<'b>,
}

/// The mutant: the index is a bare integer.
#[cfg(feature = "mutant-bare")]
pub type Index<'b> = usize;

#[cfg(not(feature = "mutant-bare"))]
#[inline]
pub(crate) const fn mint<'b>(at: usize) -> Index<'b> {
    Index {
        at,
        brand: PhantomData,
    }
}

#[cfg(feature = "mutant-bare")]
#[inline]
pub(crate) const fn mint<'b>(at: usize) -> Index<'b> {
    at
}

#[cfg(not(feature = "mutant-bare"))]
#[inline]
pub(crate) const fn at(index: Index<'_>) -> usize {
    index.at
}

#[cfg(feature = "mutant-bare")]
#[inline]
pub(crate) const fn at(index: Index<'_>) -> usize {
    index
}

/// Every record of one morsel, under its brand, in a sequence the engine chooses.
pub struct Indices<'b> {
    next:  usize,
    end:   usize,
    brand: Brand<'b>,
}

impl<'b> Indices<'b> {
    #[inline]
    pub(crate) const fn new(end: usize) -> Self {
        Indices {
            next: 0,
            end,
            brand: PhantomData,
        }
    }
}

impl<'b> Iterator for Indices<'b> {
    type Item = Index<'b>;

    #[inline]
    fn next(&mut self) -> Option<Index<'b>> {
        if self.next == self.end {
            return None;
        }
        let index = mint(self.next);
        self.next += 1;
        Some(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.end - self.next;
        (left, Some(left))
    }
}
