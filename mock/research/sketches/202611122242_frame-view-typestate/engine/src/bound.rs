//! The data-bound twin of the position mask.
//!
//! When columns are bound from data rather than fixed in the engine's types,
//! no write list exists for the compiler to fold, so "two kernels of one step
//! write one column" cannot be a type error. Here it is a value: a column
//! handle is minted at bind, each kernel's write set is a list of handles, and
//! the only way to a checked table is `Checked::bind`, which refuses an overlap
//! with an `Overlap` naming the column and both claimants. A runner obtains
//! staged handles only from a `Checked`.
//!
//! The staged columns are split among the kernels by safe borrows, one column
//! to at most one kernel, so a step is sound without the check; what the check
//! buys is that a second writer is refused as a value at bind instead of being
//! silently handed nothing at the step.

use core::marker::PhantomData;

use crate::view::{Row, Rows};

/// A column of `T`, minted at bind.
///
/// Typed by its value: a handle minted for `T` reads and writes only `T`. It
/// is not branded by the store it came from, so a handle carried to another
/// store of the same `T` reaches that store's column at the same index (and
/// panics on a bounds check where that store is smaller).
pub struct Column<T> {
    index: u32,
    _t:    PhantomData<fn() -> T>,
}

impl<T> Clone for Column<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Column<T> {}

impl<T> Column<T> {
    const fn at(self) -> usize {
        self.index as usize
    }

    const fn bit(self) -> u128 {
        1u128 << self.index
    }
}

/// A second claim on one column, refused at bind.
#[derive(Debug, PartialEq, Eq)]
pub struct Overlap {
    /// The column claimed twice.
    pub column: u32,
    /// The kernel that claimed it first.
    pub first:  u32,
    /// The kernel whose claim was refused.
    pub second: u32,
}

/// A write to a column the kernel's write set does not name, refused.
#[derive(Debug, PartialEq, Eq)]
pub struct Undeclared {
    /// The column the write named.
    pub column: u32,
}

/// The write sets of one step's `K` kernels, checked disjoint.
///
/// The fields are private and `bind` is the only constructor, so holding one
/// is the witness that no column is claimed twice, within one kernel's set or
/// across two.
pub struct Checked<T, const K: usize> {
    masks: [u128; K],
    _t:    PhantomData<fn() -> T>,
}

impl<T, const K: usize> Checked<T, K> {
    /// Checks the table and keeps each kernel's set as a mask.
    pub fn bind(table: [&[Column<T>]; K]) -> Result<Self, Overlap> {
        let mut masks = [0u128; K];
        let mut kernel = 0;
        while kernel < K {
            for &column in table[kernel] {
                let mut other = 0;
                while other <= kernel {
                    if masks[other] & column.bit() != 0 {
                        return Err(Overlap {
                            column: column.index,
                            first:  other as u32,
                            second: kernel as u32,
                        });
                    }
                    other += 1;
                }
                masks[kernel] |= column.bit();
            }
            kernel += 1;
        }
        Ok(Checked {
            masks,
            _t: PhantomData,
        })
    }
}

/// The committed columns, for the step. Copy, and Sync when `T` is.
pub struct ColumnView<'s, T, const C: usize, const N: usize> {
    columns: &'s [[T; N]; C],
}

impl<T, const C: usize, const N: usize> Clone for ColumnView<'_, T, C, N> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const C: usize, const N: usize> Copy for ColumnView<'_, T, C, N> {}

impl<T: Copy, const C: usize, const N: usize> ColumnView<'_, T, C, N> {
    /// Every row.
    pub const fn rows(&self) -> Rows {
        Rows::new(N as u32)
    }

    /// One row of a column, as committed at the step's start.
    pub fn get(&self, column: Column<T>, row: Row) -> T {
        self.columns[column.at()][row.at()]
    }
}

/// One kernel's staged columns: those its checked write set names, and no
/// other. Write only, not Copy, not Clone; Send when `T` is.
pub struct ColumnStaged<'s, T, const C: usize, const N: usize> {
    columns: [Option<&'s mut [T; N]>; C],
}

impl<T, const C: usize, const N: usize> ColumnStaged<'_, T, C, N> {
    /// Stages one row, or refuses a column the write set does not name.
    pub fn set(&mut self, column: Column<T>, row: Row, value: T) -> Result<(), Undeclared> {
        match &mut self.columns[column.at()] {
            Some(lane) => {
                lane[row.at()] = value;
                Ok(())
            },
            None => {
                Err(Undeclared {
                    column: column.index,
                })
            },
        }
    }
}

/// One step's handles: the view, and each kernel's staged columns in the
/// order of the checked table.
pub struct Step<'s, T, const C: usize, const N: usize, const K: usize> {
    pub view:   ColumnView<'s, T, C, N>,
    pub staged: [ColumnStaged<'s, T, C, N>; K],
}

/// A kernel over data-bound columns. It carries its handles as data.
pub trait BoundKernel<T, const C: usize, const N: usize> {
    fn run(&self, view: ColumnView<'_, T, C, N>, out: ColumnStaged<'_, T, C, N>);
}

/// `C` columns of `T`, `N` rows each, committed and staged.
pub struct Columns<T, const C: usize, const N: usize> {
    committed: [[T; N]; C],
    staged:    [[T; N]; C],
    minted:    u32,
}

impl<T: Copy, const C: usize, const N: usize> Columns<T, C, N> {
    const FITS: () = {
        assert!(C <= 128, "a column set holds at most 128 columns");
        assert!(
            N <= u32::MAX as usize,
            "a column holds at most u32::MAX rows"
        );
    };

    pub fn new(init: T) -> Self {
        let () = Self::FITS;
        Columns {
            committed: [[init; N]; C],
            staged:    [[init; N]; C],
            minted:    0,
        }
    }

    /// Mints the next column, or refuses when all `C` are bound.
    pub fn bind(&mut self) -> Option<Column<T>> {
        if self.minted as usize == C {
            return None;
        }
        let column = Column {
            index: self.minted,
            _t:    PhantomData,
        };
        self.minted += 1;
        Some(column)
    }

    /// Opens a step: staged starts as committed, and each kernel of the
    /// checked table gets the staged columns it names, by safe split borrows.
    pub fn begin<const K: usize>(&mut self, checked: &Checked<T, K>) -> Step<'_, T, C, N, K> {
        self.staged = self.committed;
        let mut staged: [ColumnStaged<'_, T, C, N>; K] = core::array::from_fn(|_| {
            ColumnStaged {
                columns: core::array::from_fn(|_| None),
            }
        });
        for (index, lane) in self.staged.iter_mut().enumerate() {
            let bit = 1u128 << index;
            if let Some(owner) = checked.masks.iter().position(|mask| mask & bit != 0) {
                staged[owner].columns[index] = Some(lane);
            }
        }
        Step {
            view: ColumnView {
                columns: &self.committed,
            },
            staged,
        }
    }

    /// Lands the step opened by the last `begin`.
    pub fn land(&mut self) {
        core::mem::swap(&mut self.committed, &mut self.staged);
    }

    /// The reference runner: one step with two kernels, one after the other.
    pub fn step2<A, B>(&mut self, checked: &Checked<T, 2>, a: &A, b: &B)
    where
        A: BoundKernel<T, C, N>,
        B: BoundKernel<T, C, N>,
    {
        let Step {
            view,
            staged: [sa, sb],
        } = self.begin(checked);
        a.run(view, sa);
        b.run(view, sb);
        self.land();
    }

    /// The committed rows of one column, for whoever owns the store.
    pub fn committed(&self, column: Column<T>) -> &[T; N] {
        &self.committed[column.at()]
    }
}
