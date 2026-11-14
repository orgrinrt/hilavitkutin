//! The storage a caller lends the runner, one form for each store kind.
//!
//! The runner allocates nothing. Every store the units name is kept in slices
//! the caller owns, and a world is a nested pair of [`Lent`] stores ending in
//! `()`, found by type the way the api finds a member of an access set.

use core::marker::PhantomData;

use arvo::{Bool, USize};
use hilavitkutin_api::ColumnValue;
use hilavitkutin_api::store::{
    Accum,
    BufferMode,
    Buffered,
    Column,
    Exchange,
    Resource,
    Snapshot,
    Virtual,
};
use hilavitkutin_api::values::{Here, Place, There};

use super::keeping::Keeping;

/// A store the runner keeps: its lent form, and what that form does at the
/// two ends of a frame.
pub trait Kept: 'static {
    /// The lent storage of one store of this kind.
    type Cells<'w>;

    /// Whether the storage has room for `records` records.
    fn holds(cells: &Self::Cells<'_>, records: USize) -> Bool;
    /// Before the first unit of a frame.
    fn open<K: Keeping>(cells: &mut Self::Cells<'_>);
    /// The boundary, after the last unit of a frame.
    fn close<K: Keeping>(cells: &mut Self::Cells<'_>, records: USize);
}

/// A plain column: one value a record, written in place.
pub struct Plain<'w, T> {
    pub(super) cells: &'w mut [T],
}

impl<'w, T> Plain<'w, T> {
    /// The column's values, in record sequence.
    pub const fn values(&self) -> &[T] {
        self.cells
    }
}

/// How a buffered column's live side becomes the frozen one at the boundary.
pub trait Brings: BufferMode {
    /// Make what `live` holds over the first `records` records the frozen
    /// side.
    fn bring<'w, T: Copy>(frozen: &mut &'w mut [T], live: &mut &'w mut [T], records: USize);
}

impl Brings for Snapshot {
    fn bring<'w, T: Copy>(frozen: &mut &'w mut [T], live: &mut &'w mut [T], records: USize) {
        frozen[.. records.0].copy_from_slice(&live[.. records.0]);
    }
}

impl Brings for Exchange {
    fn bring<'w, T: Copy>(frozen: &mut &'w mut [T], live: &mut &'w mut [T], _records: USize) {
        core::mem::swap(frozen, live);
    }
}

/// A buffered column: the side every read of a frame sees, and the side its
/// writes land on.
pub struct Sides<'w, T, M> {
    pub(super) frozen:  &'w mut [T],
    pub(super) live:    &'w mut [T],
    pub(super) written: Bool,
    mode:               PhantomData<M>,
}

impl<'w, T, M> Sides<'w, T, M> {
    /// The side every read of the next frame sees.
    pub const fn frozen(&self) -> &[T] {
        self.frozen
    }
}

/// An accumulator: the values appended in the frame, in the sequence the
/// runner met them.
pub struct Events<'w, T> {
    pub(super) cells: &'w mut [T],
    pub(super) len:   USize,
}

impl<'w, T> Events<'w, T> {
    /// What the last frame appended.
    pub fn appended(&self) -> &[T] {
        &self.cells[.. self.len.0]
    }
}

/// A virtual: whether a unit fired it in the frame.
pub struct Flag<V> {
    pub(super) fired: Bool,
    /// Whether an `On` unit of the virtual has been asked in the frame.
    pub(super) asked: Bool,
    of:               PhantomData<V>,
}

impl<V> Flag<V> {
    /// Whether the last frame fired the virtual.
    pub const fn fired(&self) -> Bool {
        self.fired
    }
}

/// A resource: one value, read and never written inside a frame.
pub struct Held<'w, T> {
    pub(super) value: &'w T,
}

impl<T: ColumnValue> Kept for Column<T> {
    type Cells<'w> = Plain<'w, T>;

    fn holds(cells: &Plain<'_, T>, records: USize) -> Bool {
        Bool(cells.cells.len() >= records.0)
    }

    fn open<K: Keeping>(_cells: &mut Plain<'_, T>) {}

    fn close<K: Keeping>(_cells: &mut Plain<'_, T>, _records: USize) {}
}

impl<T: ColumnValue, M: Brings> Kept for Buffered<T, M> {
    type Cells<'w> = Sides<'w, T, M>;

    fn holds(cells: &Sides<'_, T, M>, records: USize) -> Bool {
        Bool(cells.frozen.len() >= records.0 && cells.live.len() >= records.0)
    }

    fn open<K: Keeping>(cells: &mut Sides<'_, T, M>) {
        cells.written = Bool(false);
    }

    fn close<K: Keeping>(cells: &mut Sides<'_, T, M>, records: USize) {
        if cells.written.0 || K::BRINGS_UNWRITTEN.0 {
            M::bring(&mut cells.frozen, &mut cells.live, records);
        }
    }
}

impl<T: ColumnValue> Kept for Accum<T> {
    type Cells<'w> = Events<'w, T>;

    fn holds(_cells: &Events<'_, T>, _records: USize) -> Bool {
        Bool(true)
    }

    fn open<K: Keeping>(cells: &mut Events<'_, T>) {
        if !K::KEEPS_APPENDS.0 {
            cells.len = USize(0);
        }
    }

    fn close<K: Keeping>(_cells: &mut Events<'_, T>, _records: USize) {}
}

impl<V: 'static> Kept for Virtual<V> {
    type Cells<'w> = Flag<V>;

    fn holds(_cells: &Flag<V>, _records: USize) -> Bool {
        Bool(true)
    }

    fn open<K: Keeping>(cells: &mut Flag<V>) {
        cells.asked = Bool(false);
        if !K::KEEPS_FLAGS.0 {
            cells.fired = Bool(false);
        }
    }

    fn close<K: Keeping>(_cells: &mut Flag<V>, _records: USize) {}
}

impl<T: 'static> Kept for Resource<T> {
    type Cells<'w> = Held<'w, T>;

    fn holds(_cells: &Held<'_, T>, _records: USize) -> Bool {
        Bool(true)
    }

    fn open<K: Keeping>(_cells: &mut Held<'_, T>) {}

    fn close<K: Keeping>(_cells: &mut Held<'_, T>, _records: USize) {}
}

/// The lent storage of the store `S`, named by its marker.
pub struct Lent<'w, S: Kept>(S::Cells<'w>);

impl<'w, T: ColumnValue> Lent<'w, Column<T>> {
    /// A plain column over `cells`, one record a cell.
    pub const fn over(cells: &'w mut [T]) -> Self {
        Lent(Plain {
            cells,
        })
    }
}

impl<'w, T: ColumnValue, M: Brings> Lent<'w, Buffered<T, M>> {
    /// A buffered column whose frozen side starts as `frozen`. What `live`
    /// holds is never read by a faithful frame.
    pub const fn over(frozen: &'w mut [T], live: &'w mut [T]) -> Self {
        Lent(Sides {
            frozen,
            live,
            written: Bool(false),
            mode: PhantomData,
        })
    }
}

impl<'w, T: ColumnValue> Lent<'w, Accum<T>> {
    /// An accumulator that keeps at most `cells.len()` appends a frame.
    pub const fn over(cells: &'w mut [T]) -> Self {
        Lent(Events {
            cells,
            len: USize(0),
        })
    }
}

impl<'w, V: 'static> Lent<'w, Virtual<V>> {
    /// A virtual nobody has fired.
    pub const fn unfired() -> Self {
        Lent(Flag {
            fired: Bool(false),
            asked: Bool(false),
            of:    PhantomData,
        })
    }
}

impl<'w, T: 'static> Lent<'w, Resource<T>> {
    /// A resource holding `value`.
    pub const fn of(value: &'w T) -> Self {
        Lent(Held {
            value,
        })
    }
}

/// The world holds the store `S` at the place `P`.
pub trait Find<'w, S: Kept, P> {
    /// The store's storage.
    fn cells(&self) -> &S::Cells<'w>;
    /// The store's storage, to write.
    fn cells_mut(&mut self) -> &mut S::Cells<'w>;
}

impl<'w, S: Kept, T> Find<'w, S, Here> for (Lent<'w, S>, T) {
    fn cells(&self) -> &S::Cells<'w> {
        &self.0.0
    }

    fn cells_mut(&mut self) -> &mut S::Cells<'w> {
        &mut self.0.0
    }
}

impl<'w, S: Kept, H, T: Find<'w, S, P>, P: Place> Find<'w, S, There<P>> for (H, T) {
    fn cells(&self) -> &S::Cells<'w> {
        self.1.cells()
    }

    fn cells_mut(&mut self) -> &mut S::Cells<'w> {
        self.1.cells_mut()
    }
}

/// Every store of a world, at the two ends of a frame.
pub trait World {
    /// Whether every store has room for `records` records.
    fn holds(&self, records: USize) -> Bool;
    /// Before the first unit of a frame.
    fn open<K: Keeping>(&mut self);
    /// The boundary, after the last unit of a frame.
    fn close<K: Keeping>(&mut self, records: USize);
}

impl World for () {
    fn holds(&self, _records: USize) -> Bool {
        Bool(true)
    }

    fn open<K: Keeping>(&mut self) {}

    fn close<K: Keeping>(&mut self, _records: USize) {}
}

impl<'w, S: Kept, T: World> World for (Lent<'w, S>, T) {
    fn holds(&self, records: USize) -> Bool {
        Bool(S::holds(&self.0.0, records).0 && self.1.holds(records).0)
    }

    fn open<K: Keeping>(&mut self) {
        S::open::<K>(&mut self.0.0);
        self.1.open::<K>();
    }

    fn close<K: Keeping>(&mut self, records: USize) {
        S::close::<K>(&mut self.0.0, records);
        self.1.close::<K>(records);
    }
}
