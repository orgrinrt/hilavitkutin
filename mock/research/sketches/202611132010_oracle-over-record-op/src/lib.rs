//! Sketch: a one-thread runner that takes value-form units through
//! `RecordOp::apply` and nothing else of them.
//!
//! The runner keeps every store in storage the caller lends, gathers a unit's
//! read product from it at one record, applies the unit's value function and
//! stores what the function returned. It never calls `execute`, so it needs
//! no context type and no engine, and a unit written for the scheduler runs
//! here as it stands.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(adt_const_params)]

pub mod digest;
pub mod units;

use core::marker::PhantomData;

use arvo::{Bool, Cap, USize};
use hilavitkutin_api::access::{Cons, Empty};
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
use hilavitkutin_api::values::{
    Here,
    PCons,
    PNil,
    Place,
    RCons,
    RNil,
    ReadSet,
    ReadVals,
    Readable,
    ResourceRead,
    There,
    Writable,
    WriteSet,
};
use hilavitkutin_api::work_unit::{Always, HasSchedule, On, WorkUnit};
use hilavitkutin_api::{ColumnValue, Differs, Reach, RecordOp, Window, Written};
use notko::Maybe;
use notko_hlist::{Elem, Project, Val};

/// A store the runner keeps, and what it holds for it.
pub trait Kept: 'static {
    /// The lent storage of one store of this kind.
    type Cells<'w>: Cell;
}

/// What a store's storage does at the two ends of a frame.
pub trait Cell {
    /// Before the first unit of a frame.
    fn open(&mut self);
    /// After the last unit of a frame.
    fn close(&mut self);
}

/// A plain column: one value a record, written in place.
pub struct Plain<'w, T> {
    cells: &'w mut [T],
}

impl<'w, T> Plain<'w, T> {
    /// The column's values, in record sequence.
    pub fn values(&self) -> &[T] {
        self.cells
    }
}

impl<'w, T> Cell for Plain<'w, T> {
    fn open(&mut self) {}

    fn close(&mut self) {}
}

/// How a buffered column's live side becomes the frozen one.
pub trait Brings: BufferMode {
    /// Make what `live` holds the frozen side.
    fn bring<'w, T: Copy>(frozen: &mut &'w mut [T], live: &mut &'w mut [T]);
}

impl Brings for Snapshot {
    fn bring<'w, T: Copy>(frozen: &mut &'w mut [T], live: &mut &'w mut [T]) {
        frozen.copy_from_slice(live);
    }
}

impl Brings for Exchange {
    fn bring<'w, T: Copy>(frozen: &mut &'w mut [T], live: &mut &'w mut [T]) {
        core::mem::swap(frozen, live);
    }
}

/// A buffered column: the side every read of a frame sees, and the side its
/// writes land on.
pub struct Sides<'w, T, M> {
    frozen:  &'w mut [T],
    live:    &'w mut [T],
    written: Bool,
    mode:    PhantomData<M>,
}

impl<'w, T, M> Sides<'w, T, M> {
    /// The values every unit of the next frame reads.
    pub fn frozen(&self) -> &[T] {
        self.frozen
    }
}

impl<'w, T: Copy, M: Brings> Cell for Sides<'w, T, M> {
    fn open(&mut self) {
        self.written = Bool(false);
    }

    fn close(&mut self) {
        if self.written.0 {
            M::bring(&mut self.frozen, &mut self.live);
        }
    }
}

/// An accumulator: the values appended in the frame, in the sequence the
/// runner met them.
pub struct Events<'w, T> {
    cells: &'w mut [T],
    len:   USize,
}

impl<'w, T> Events<'w, T> {
    /// What the last frame appended.
    pub fn appended(&self) -> &[T] {
        &self.cells[.. self.len.0]
    }
}

impl<'w, T> Cell for Events<'w, T> {
    fn open(&mut self) {
        self.len = USize(0);
    }

    fn close(&mut self) {}
}

/// A virtual: whether a unit fired it in the frame.
pub struct Flag<V> {
    fired: Bool,
    of:    PhantomData<V>,
}

impl<V> Flag<V> {
    /// Whether the last frame fired the virtual.
    pub fn fired(&self) -> Bool {
        self.fired
    }
}

impl<V> Cell for Flag<V> {
    fn open(&mut self) {
        self.fired = Bool(false);
    }

    fn close(&mut self) {}
}

/// A resource: one value, read and never written inside a frame.
pub struct Held<'w, T> {
    value: &'w T,
}

impl<'w, T> Cell for Held<'w, T> {
    fn open(&mut self) {}

    fn close(&mut self) {}
}

impl<T: ColumnValue> Kept for Column<T> {
    type Cells<'w> = Plain<'w, T>;
}

impl<T: ColumnValue, M: Brings> Kept for Buffered<T, M> {
    type Cells<'w> = Sides<'w, T, M>;
}

impl<T: ColumnValue> Kept for Accum<T> {
    type Cells<'w> = Events<'w, T>;
}

impl<V: 'static> Kept for Virtual<V> {
    type Cells<'w> = Flag<V>;
}

impl<T: 'static> Kept for Resource<T> {
    type Cells<'w> = Held<'w, T>;
}

/// The lent storage of the store `S`, named by its marker.
pub struct Lent<'w, S: Kept>(S::Cells<'w>);

impl<'w, T: ColumnValue> Lent<'w, Column<T>> {
    /// A plain column over `cells`, one record a cell.
    pub fn over(cells: &'w mut [T]) -> Self {
        Lent(Plain {
            cells,
        })
    }
}

impl<'w, T: ColumnValue, M: Brings> Lent<'w, Buffered<T, M>> {
    /// A buffered column whose frozen side starts as `frozen`.
    pub fn over(frozen: &'w mut [T], live: &'w mut [T]) -> Self {
        assert!(
            frozen.len() == live.len(),
            "the two sides hold one count of records"
        );
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
    pub fn over(cells: &'w mut [T]) -> Self {
        Lent(Events {
            cells,
            len: USize(0),
        })
    }
}

impl<'w, V: 'static> Lent<'w, Virtual<V>> {
    /// A virtual nobody has fired.
    pub fn unfired() -> Self {
        Lent(Flag {
            fired: Bool(false),
            of:    PhantomData,
        })
    }
}

impl<'w, T: 'static> Lent<'w, Resource<T>> {
    /// A resource holding `value`.
    pub fn of(value: &'w T) -> Self {
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
    /// Before the first unit of a frame.
    fn open(&mut self);
    /// After the last unit of a frame.
    fn close(&mut self);
}

impl World for () {
    fn open(&mut self) {}

    fn close(&mut self) {}
}

impl<'w, S: Kept, T: World> World for (Lent<'w, S>, T) {
    fn open(&mut self) {
        self.0.0.open();
        self.1.open();
    }

    fn close(&mut self) {
        self.0.0.close();
        self.1.close();
    }
}

/// A read member the runner can gather.
pub trait Reads: Readable {
    /// The store the member reads.
    type At: Kept;

    /// The member's value at record `at`.
    fn read<'a, 'w: 'a>(cells: &'a <Self::At as Kept>::Cells<'w>, at: USize) -> Self::Val<'a>;
}

impl<T: ColumnValue> Reads for Column<T> {
    type At = Self;

    fn read<'a, 'w: 'a>(cells: &'a Plain<'w, T>, at: USize) -> T {
        cells.cells[at.0]
    }
}

impl<T: ColumnValue, M: Brings> Reads for Buffered<T, M> {
    type At = Self;

    fn read<'a, 'w: 'a>(cells: &'a Sides<'w, T, M>, at: USize) -> T {
        if cfg!(feature = "reads-live") {
            return cells.live[at.0];
        }
        cells.frozen[at.0]
    }
}

impl<T: ColumnValue, const N: Cap> Reads for Reach<Column<T>, N> {
    type At = Column<T>;

    fn read<'a, 'w: 'a>(cells: &'a Plain<'w, T>, at: USize) -> Window<'a, T, N> {
        let count = cells.cells.len();
        assert!(at.0 < count, "a record of the column");
        // SAFETY: `count` initialised records borrowed for `'a`, and `at` is one of them.
        unsafe { Window::over(cells.cells.as_ptr(), at, USize(count)) }
    }
}

impl<T: ColumnValue, M: Brings, const N: Cap> Reads for Reach<Buffered<T, M>, N> {
    type At = Buffered<T, M>;

    fn read<'a, 'w: 'a>(cells: &'a Sides<'w, T, M>, at: USize) -> Window<'a, T, N> {
        let side: &[T] = if cfg!(feature = "reads-live") { &*cells.live } else { &*cells.frozen };
        let count = side.len();
        assert!(at.0 < count, "a record of the column");
        // SAFETY: `count` initialised records borrowed for `'a`, and `at` is one of them.
        unsafe { Window::over(side.as_ptr(), at, USize(count)) }
    }
}

impl<T: ResourceRead> Reads for Resource<T> {
    type At = Self;

    fn read<'a, 'w: 'a>(cells: &'a Held<'w, T>, _at: USize) -> <T as ResourceRead>::Val<'a> {
        let value: &'a T = cells.value;
        value.read()
    }
}

/// A write member the runner can store.
pub trait Writes: Writable {
    /// The store the member writes.
    type At: Kept;

    /// Store the value a function returned for record `at`.
    fn write<'w>(cells: &mut <Self::At as Kept>::Cells<'w>, at: USize, val: Self::Val);
}

macro_rules! column_writes {
    ($($member:ty where [$($bound:tt)*];)*) => {$(
        impl<T: $($bound)*> Writes for $member {
            type At = Column<T>;

            fn write<'w>(cells: &mut Plain<'w, T>, at: USize, val: T) {
                cells.cells[at.0] = val;
            }
        }
    )*};
}

column_writes! {
    Column<T> where [ColumnValue + PartialEq];
    Written<Column<T>> where [ColumnValue];
    Differs<Column<T>> where [ColumnValue + PartialEq];
}

macro_rules! buffered_writes {
    ($($member:ty where [$($bound:tt)*];)*) => {$(
        impl<T: $($bound)*, M: Brings> Writes for $member {
            type At = Buffered<T, M>;

            fn write<'w>(cells: &mut Sides<'w, T, M>, at: USize, val: T) {
                cells.live[at.0] = val;
                cells.written = Bool(true);
            }
        }
    )*};
}

buffered_writes! {
    Buffered<T, M> where [ColumnValue + PartialEq];
    Written<Buffered<T, M>> where [ColumnValue];
    Differs<Buffered<T, M>> where [ColumnValue + PartialEq];
}

impl<T: ColumnValue> Writes for Accum<T> {
    type At = Self;

    fn write<'w>(cells: &mut Events<'w, T>, _at: USize, val: Maybe<T>) {
        if let Maybe::Is(value) = val {
            assert!(
                cells.len.0 < cells.cells.len(),
                "the lent accumulator is full"
            );
            cells.cells[cells.len.0] = value;
            cells.len = USize(cells.len.0 + 1);
        }
    }
}

impl<V: 'static> Writes for Virtual<V> {
    type At = Self;

    fn write<'w>(cells: &mut Flag<V>, _at: USize, val: Bool) {
        if val.0 {
            cells.fired = Bool(true);
        }
    }
}

/// The world can gather the read list `R` at one record.
pub trait Gather<'w, R: ReadSet, P> {
    /// The read product at record `at`.
    fn gather<'a>(&'a self, at: USize) -> R::Vals<'a>
    where
        'w: 'a;
}

impl<'w, W> Gather<'w, Empty, ()> for W {
    fn gather<'a>(&'a self, _at: USize) -> RNil
    where
        'w: 'a,
    {
        RNil
    }
}

impl<'w, W, H, T, Ph, Pt> Gather<'w, Cons<H, T>, (Ph, Pt)> for W
where
    H: Reads,
    T: ReadSet,
    W: Find<'w, H::At, Ph> + Gather<'w, T, Pt>,
{
    fn gather<'a>(&'a self, at: USize) -> RCons<H::Val<'a>, T::Vals<'a>>
    where
        'w: 'a,
    {
        RCons {
            head: H::read(<W as Find<'w, H::At, Ph>>::cells(self), at),
            tail: <W as Gather<'w, T, Pt>>::gather(self, at),
        }
    }
}

/// The world can store the write list `L` at one record.
pub trait Scatter<'w, L: WriteSet, P> {
    /// Store the write product a function returned for record `at`.
    fn scatter(&mut self, at: USize, vals: <L as Project>::Prod<Val>);
}

impl<'w, W> Scatter<'w, Empty, ()> for W {
    fn scatter(&mut self, _at: USize, _vals: PNil) {}
}

impl<'w, W, H, T, Ph, Pt> Scatter<'w, Cons<H, T>, (Ph, Pt)> for W
where
    H: Writes + Elem<Value = <H as Writable>::Val>,
    T: WriteSet,
    W: Find<'w, H::At, Ph> + Scatter<'w, T, Pt>,
{
    fn scatter(&mut self, at: USize, vals: PCons<Val, H, <T as Project>::Prod<Val>>) {
        H::write(<W as Find<'w, H::At, Ph>>::cells_mut(self), at, vals.head);
        <W as Scatter<'w, T, Pt>>::scatter(self, at, vals.tail);
    }
}

/// Whether a unit of this schedule runs in the frame, as the world stands.
pub trait Due<'w, W, P> {
    /// True when the unit runs.
    fn due(world: &W) -> Bool;
}

impl<'w, W> Due<'w, W, ()> for Always {
    fn due(_world: &W) -> Bool {
        Bool(true)
    }
}

impl<'w, W: Find<'w, Virtual<V>, P>, V: 'static, P> Due<'w, W, P> for On<V> {
    fn due(world: &W) -> Bool {
        world.cells().fired
    }
}

/// A sequence of value-form units the runner can take over the world `W`.
pub trait Units<'w, W, P> {
    /// Run every unit that is due, each over `records` records.
    fn run(&self, world: &mut W, records: USize);
}

impl<'w, W> Units<'w, W, ()> for () {
    fn run(&self, _world: &mut W, _records: USize) {}
}

impl<'w, W, U, Rest, Pd, Pr, Pw, Pn> Units<'w, W, (Pd, Pr, Pw, Pn)> for (U, Rest)
where
    U: HasSchedule + RecordOp<<U as HasSchedule>::Sched>,
    <U as WorkUnit<<U as HasSchedule>::Sched>>::Read: ReadSet,
    <U as WorkUnit<<U as HasSchedule>::Sched>>::Write: WriteSet,
    <U as HasSchedule>::Sched: Due<'w, W, Pd>,
    W: Gather<'w, <U as WorkUnit<<U as HasSchedule>::Sched>>::Read, Pr>
        + Scatter<'w, <U as WorkUnit<<U as HasSchedule>::Sched>>::Write, Pw>,
    Rest: Units<'w, W, Pn>,
{
    fn run(&self, world: &mut W, records: USize) {
        if <<U as HasSchedule>::Sched as Due<'w, W, Pd>>::due(world).0 {
            let mut at = USize(0);
            while at.0 < records.0 {
                let out = {
                    let read = ReadVals(<W as Gather<
                        'w,
                        <U as WorkUnit<<U as HasSchedule>::Sched>>::Read,
                        Pr,
                    >>::gather(&*world, at));
                    self.0.apply(&read)
                };
                <W as Scatter<'w, <U as WorkUnit<<U as HasSchedule>::Sched>>::Write, Pw>>::scatter(
                    world, at, out.0,
                );
                at = USize(at.0 + 1);
            }
        }
        self.1.run(world, records);
    }
}

/// The runner: a world of lent stores and how many records each column holds.
pub struct Oracle<W> {
    world:   W,
    records: USize,
}

impl<W: World> Oracle<W> {
    /// A runner over `world`, every column of which holds `records` records.
    pub fn new(world: W, records: USize) -> Self {
        Oracle {
            world,
            records,
        }
    }

    /// One frame: the units in the sequence given, then the boundary.
    pub fn frame<'w, Us: Units<'w, W, P>, P>(&mut self, units: &Us) {
        self.world.open();
        units.run(&mut self.world, self.records);
        self.world.close();
    }

    /// The storage of the store `S`.
    pub fn of<'w, S: Kept, P>(&self) -> &S::Cells<'w>
    where
        W: Find<'w, S, P>,
    {
        self.world.cells()
    }
}
