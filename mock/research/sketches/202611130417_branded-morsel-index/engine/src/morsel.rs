//! The morsel a block kernel is handed, the branded block `each` opens on it,
//! and the one-thread runner that cuts the columns into morsels.

use core::marker::PhantomData;
use core::num::NonZeroUsize;

use crate::index::{Brand, Index, Indices, at};

/// One morsel: a run of records of one read column and one write column, of
/// equal length, which `Morsels` alone builds.
pub struct Morsel<'m, R, W> {
    read:  &'m [R],
    write: &'m mut [W],
}

impl<'m, R, W> Morsel<'m, R, W> {
    /// Opens the morsel under a fresh brand.
    ///
    /// `f` is called once, with the morsel's columns as a `Block` and the
    /// brand `'b` universally quantified, so `f` holds for every `'b` and
    /// nothing it is handed can be named outside it: an index cannot be
    /// returned from `f`, and an index from another `each` does not meet
    /// this one's.
    #[inline]
    pub fn each<O, F>(&mut self, f: F) -> O
    where
        F: for<'b> FnOnce(Block<'_, 'b, R, W>) -> O,
    {
        let records = self.read.len();
        f(Block {
            read:  self.read,
            write: &mut self.write[.. records],
            brand: PhantomData,
        })
    }
}

/// The columns of one morsel under one brand.
///
/// Every `Index<'b>` comes from `indices` on a block of this brand, and every
/// block of this brand is this one, whose columns are as long as the run
/// `indices` yields; so an index of this brand is in range for both columns.
/// The reads and writes below index the slices safely all the same.
pub struct Block<'x, 'b, R, W> {
    read:  &'x [R],
    write: &'x mut [W],
    brand: Brand<'b>,
}

impl<'x, 'b, R, W> Block<'x, 'b, R, W> {
    /// Every record of the morsel, under this block's brand.
    #[inline]
    pub const fn indices(&self) -> Indices<'b> {
        Indices::new(self.read.len())
    }

    /// The read column at one record.
    #[inline]
    pub fn read(&self, index: Index<'b>) -> R
    where
        R: Copy,
    {
        self.read[at(index)]
    }

    /// Writes the write column at one record.
    #[inline]
    pub fn write(&mut self, index: Index<'b>, value: W) {
        self.write[at(index)] = value;
    }
}

/// The columns' lengths differ, so they cannot be cut into morsels.
#[derive(Debug, PartialEq, Eq)]
pub struct Unequal {
    pub read:  usize,
    pub write: usize,
}

/// A read column and a write column, cut into morsels of `len` records, the last
/// one shorter where `len` does not divide them.
pub struct Morsels<'m, R, W> {
    read:  &'m [R],
    write: &'m mut [W],
    len:   NonZeroUsize,
}

impl<'m, R, W> Morsels<'m, R, W> {
    /// Refuses two columns of different length.
    pub const fn new(
        read: &'m [R],
        write: &'m mut [W],
        len: NonZeroUsize,
    ) -> Result<Self, Unequal> {
        if read.len() != write.len() {
            return Err(Unequal {
                read:  read.len(),
                write: write.len(),
            });
        }
        Ok(Morsels {
            read,
            write,
            len,
        })
    }
}

impl<'m, R, W> Iterator for Morsels<'m, R, W> {
    type Item = Morsel<'m, R, W>;

    #[inline]
    fn next(&mut self) -> Option<Morsel<'m, R, W>> {
        if self.read.is_empty() {
            return None;
        }
        let cut = if self.read.len() < self.len.get() { self.read.len() } else { self.len.get() };
        let (read, rest) = self.read.split_at(cut);
        self.read = rest;
        let (write, rest) = core::mem::take(&mut self.write).split_at_mut(cut);
        self.write = rest;
        Some(Morsel {
            read,
            write,
        })
    }
}

/// A block kernel: handed one morsel at a time, it opens it with `each`.
pub trait BlockKernel<R, W> {
    fn block(&self, morsel: Morsel<'_, R, W>);
}

/// The reference runner: one thread, the morsels one after another.
pub fn run<R, W, K>(
    kernel: &K,
    read: &[R],
    write: &mut [W],
    len: NonZeroUsize,
) -> Result<(), Unequal>
where
    K: BlockKernel<R, W>,
{
    for morsel in Morsels::new(read, write, len)? {
        kernel.block(morsel);
    }
    Ok(())
}
