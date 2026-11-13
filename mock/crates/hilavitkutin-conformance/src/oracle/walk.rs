//! The type-level walk: what one member, one access set and one unit list do
//! at a record.
//!
//! A unit is taken through `RecordOp::apply` and nothing else of it. The walk
//! gathers the unit's read product from the world at one record, applies the
//! value function and stores what the function returned.

use arvo::{Bool, Cap, USize};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::store::{Accum, Buffered, Column, Resource, Virtual};
use hilavitkutin_api::values::{
    PCons,
    PNil,
    RCons,
    RNil,
    ReadSet,
    ReadVals,
    Readable,
    ResourceRead,
    Writable,
    WriteSet,
};
use hilavitkutin_api::work_unit::{Always, HasSchedule, On, WorkUnit};
use hilavitkutin_api::{ColumnValue, Differs, Reach, RecordOp, Window, Written};
use notko::{Maybe, Outcome};
use notko_hlist::{Elem, Project, Val};

use super::Refusal;
use super::keeping::Keeping;
use super::lent::{Brings, Events, Find, Flag, Held, Kept, Plain, Sides};
use super::visiting::Visiting;

/// A window on record `at` of the first `records` records of `side`.
///
/// Panics where `at` is not one of those records or `side` holds fewer. The
/// runner never reaches that: it checks every store against its record count
/// when it is built and visits only records below the count.
fn window<'a, T: ColumnValue, const N: Cap>(
    side: &'a [T],
    at: USize,
    records: USize,
) -> Window<'a, T, N> {
    assert!(
        at.0 < records.0 && records.0 <= side.len(),
        "a record of the column"
    );
    // SAFETY: `side` is borrowed for `'a` and holds at least `records`
    // initialised records, and `at` is one of them, by the assertion above.
    unsafe { Window::over(side.as_ptr(), at, records) }
}

/// A read member the runner can gather.
pub trait Reads: Readable {
    /// The store the member reads.
    type At: Kept;

    /// The member's value at record `at` of `records`.
    fn read<'a, 'w: 'a, K: Keeping>(
        cells: &'a <Self::At as Kept>::Cells<'w>,
        at: USize,
        records: USize,
    ) -> Self::Val<'a>;
}

impl<T: ColumnValue> Reads for Column<T> {
    type At = Self;

    fn read<'a, 'w: 'a, K: Keeping>(cells: &'a Plain<'w, T>, at: USize, _records: USize) -> T {
        cells.cells[at.0]
    }
}

/// The side a read of a buffered column sees.
fn seen<'a, T, M, K: Keeping>(cells: &'a Sides<'_, T, M>) -> &'a [T] {
    if K::READS_LIVE.0 { &*cells.live } else { &*cells.frozen }
}

impl<T: ColumnValue, M: Brings> Reads for Buffered<T, M> {
    type At = Self;

    fn read<'a, 'w: 'a, K: Keeping>(cells: &'a Sides<'w, T, M>, at: USize, _records: USize) -> T {
        seen::<T, M, K>(cells)[at.0]
    }
}

impl<T: ColumnValue, const N: Cap> Reads for Reach<Column<T>, N> {
    type At = Column<T>;

    fn read<'a, 'w: 'a, K: Keeping>(
        cells: &'a Plain<'w, T>,
        at: USize,
        records: USize,
    ) -> Window<'a, T, N> {
        window(&*cells.cells, at, records)
    }
}

impl<T: ColumnValue, M: Brings, const N: Cap> Reads for Reach<Buffered<T, M>, N> {
    type At = Buffered<T, M>;

    fn read<'a, 'w: 'a, K: Keeping>(
        cells: &'a Sides<'w, T, M>,
        at: USize,
        records: USize,
    ) -> Window<'a, T, N> {
        window(seen::<T, M, K>(cells), at, records)
    }
}

impl<T: ResourceRead> Reads for Resource<T> {
    type At = Self;

    fn read<'a, 'w: 'a, K: Keeping>(
        cells: &'a Held<'w, T>,
        _at: USize,
        _records: USize,
    ) -> <T as ResourceRead>::Val<'a> {
        let value: &'a T = cells.value;
        value.read()
    }
}

/// A write member the runner can store.
pub trait Writes: Writable {
    /// The store the member writes.
    type At: Kept;

    /// Store the value a function returned for record `at`.
    fn write<K: Keeping>(
        cells: &mut <Self::At as Kept>::Cells<'_>,
        at: USize,
        val: Self::Val,
    ) -> Outcome<(), Refusal>;
}

macro_rules! column_writes {
    ($($member:ty where [$($bound:tt)*];)*) => {$(
        impl<T: $($bound)*> Writes for $member {
            type At = Column<T>;

            fn write<K: Keeping>(
                cells: &mut Plain<'_, T>,
                at: USize,
                val: T,
            ) -> Outcome<(), Refusal> {
                cells.cells[at.0] = val;
                Outcome::Ok(())
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

            fn write<K: Keeping>(
                cells: &mut Sides<'_, T, M>,
                at: USize,
                val: T,
            ) -> Outcome<(), Refusal> {
                if K::WRITES_FROZEN.0 {
                    cells.frozen[at.0] = val;
                } else {
                    cells.live[at.0] = val;
                    cells.written = Bool(true);
                }
                Outcome::Ok(())
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

    fn write<K: Keeping>(
        cells: &mut Events<'_, T>,
        _at: USize,
        val: Maybe<T>,
    ) -> Outcome<(), Refusal> {
        if let Maybe::Is(value) = val {
            if cells.len.0 >= cells.cells.len() {
                return Outcome::Err(Refusal::Full);
            }
            cells.cells[cells.len.0] = value;
            cells.len = USize(cells.len.0 + 1);
        }
        Outcome::Ok(())
    }
}

impl<V: 'static> Writes for Virtual<V> {
    type At = Self;

    fn write<K: Keeping>(cells: &mut Flag<V>, _at: USize, val: Bool) -> Outcome<(), Refusal> {
        if val.0 {
            if cells.asked.0 && !cells.fired.0 {
                return Outcome::Err(Refusal::FiredBehind);
            }
            cells.fired = Bool(true);
        }
        Outcome::Ok(())
    }
}

/// The world can gather the read list `R` at one record.
pub trait Gather<'w, R: ReadSet, P> {
    /// The read product at record `at` of `records`.
    fn gather<'a, K: Keeping>(&'a self, at: USize, records: USize) -> R::Vals<'a>
    where
        'w: 'a;
}

impl<'w, W> Gather<'w, Empty, ()> for W {
    fn gather<'a, K: Keeping>(&'a self, _at: USize, _records: USize) -> RNil
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
    fn gather<'a, K: Keeping>(&'a self, at: USize, records: USize) -> RCons<H::Val<'a>, T::Vals<'a>>
    where
        'w: 'a,
    {
        RCons {
            head: H::read::<K>(<W as Find<'w, H::At, Ph>>::cells(self), at, records),
            tail: <W as Gather<'w, T, Pt>>::gather::<K>(self, at, records),
        }
    }
}

/// The world can store the write list `L` at one record.
pub trait Scatter<'w, L: WriteSet, P> {
    /// Store the write product a function returned for record `at`.
    fn scatter<K: Keeping>(
        &mut self,
        at: USize,
        vals: <L as Project>::Prod<Val>,
    ) -> Outcome<(), Refusal>;
}

impl<'w, W> Scatter<'w, Empty, ()> for W {
    fn scatter<K: Keeping>(&mut self, _at: USize, _vals: PNil) -> Outcome<(), Refusal> {
        Outcome::Ok(())
    }
}

impl<'w, W, H, T, Ph, Pt> Scatter<'w, Cons<H, T>, (Ph, Pt)> for W
where
    H: Writes + Elem<Value = <H as Writable>::Val>,
    T: WriteSet,
    W: Find<'w, H::At, Ph> + Scatter<'w, T, Pt>,
{
    fn scatter<K: Keeping>(
        &mut self,
        at: USize,
        vals: PCons<Val, H, <T as Project>::Prod<Val>>,
    ) -> Outcome<(), Refusal> {
        if let Outcome::Err(refusal) =
            H::write::<K>(<W as Find<'w, H::At, Ph>>::cells_mut(self), at, vals.head)
        {
            return Outcome::Err(refusal);
        }
        <W as Scatter<'w, T, Pt>>::scatter::<K>(self, at, vals.tail)
    }
}

/// Whether a unit of this schedule runs in the frame, as the world stands.
pub trait Due<'w, W, P> {
    /// True when the unit runs. Asking marks the virtual of an `On` schedule
    /// as asked, which is what a later firing of it is refused by.
    fn due<K: Keeping>(world: &mut W) -> Bool;
}

impl<'w, W> Due<'w, W, ()> for Always {
    fn due<K: Keeping>(_world: &mut W) -> Bool {
        Bool(true)
    }
}

impl<'w, W: Find<'w, Virtual<V>, P>, V: 'static, P> Due<'w, W, P> for On<V> {
    fn due<K: Keeping>(world: &mut W) -> Bool {
        let flag = world.cells_mut();
        flag.asked = Bool(true);
        Bool(flag.fired.0 || K::RUNS_UNTRIGGERED.0)
    }
}

/// A sequence of value-form units the runner can take over the world `W`.
pub trait Units<'w, W, P> {
    /// Run every unit that is due, in the sequence listed, each over
    /// `records` records.
    fn run<K: Keeping, V: Visiting>(&self, world: &mut W, records: USize) -> Outcome<(), Refusal>;
}

impl<'w, W> Units<'w, W, ()> for () {
    fn run<K: Keeping, V: Visiting>(
        &self,
        _world: &mut W,
        _records: USize,
    ) -> Outcome<(), Refusal> {
        Outcome::Ok(())
    }
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
    fn run<K: Keeping, V: Visiting>(&self, world: &mut W, records: USize) -> Outcome<(), Refusal> {
        if <<U as HasSchedule>::Sched as Due<'w, W, Pd>>::due::<K>(world).0 {
            let steps =
                if K::SKIPS_LAST.0 && records.0 > 0 { USize(records.0 - 1) } else { records };
            let mut step = USize(0);
            while step.0 < steps.0 {
                let at = V::record(step, records);
                let out = {
                    let read = ReadVals(<W as Gather<
                        'w,
                        <U as WorkUnit<<U as HasSchedule>::Sched>>::Read,
                        Pr,
                    >>::gather::<K>(&*world, at, records));
                    self.0.apply(&read)
                };
                if let Outcome::Err(refusal) = <W as Scatter<
                    'w,
                    <U as WorkUnit<<U as HasSchedule>::Sched>>::Write,
                    Pw,
                >>::scatter::<K>(world, at, out.0)
                {
                    return Outcome::Err(refusal);
                }
                step = USize(step.0 + 1);
            }
        }
        self.1.run::<K, V>(world, records)
    }
}
