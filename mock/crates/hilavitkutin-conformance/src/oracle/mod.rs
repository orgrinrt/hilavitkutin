//! The reference runner: one thread, one frame at a time, and nothing
//! planned.
//!
//! One frame, as the runner keeps it. Every virtual is unfired and every
//! accumulator is empty. The units run in the sequence they are listed, each
//! over every record. A unit scheduled `On<V>` runs only when `V` was fired
//! earlier in the frame. A read of a plain column sees what the column holds
//! at that moment. A read of a buffered column sees the frozen side whatever
//! the frame has written, and a write to one lands on the live side. An
//! append lands behind the appends before it. At the boundary each buffered
//! column that was written brings its live side in, by a copy under
//! `Snapshot` and by trading the two sides under `Exchange`.

use core::marker::PhantomData;

use arvo::USize;
use notko::Outcome;

mod keeping;
mod lent;
pub mod mutant;
mod visiting;
mod walk;

mod sealed {
    /// Closes [`Keeping`](super::Keeping) and [`Visiting`](super::Visiting)
    /// to this crate.
    pub(crate) trait Sealed {}
}

pub use keeping::{Faithful, Keeping};
pub use lent::{Brings, Events, Find, Flag, Held, Kept, Lent, Plain, Sides, World};
pub use visiting::{Ascending, Descending, Visiting};
pub use walk::{Due, Gather, Reads, Scatter, Units, Writes};

/// A frame the runner cannot keep.
///
/// A refusal ends the frame where it was met. The stores are left as they
/// were at that point and the runner is not meant to be used further.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// A lent column or side holds fewer records than the count the runner
    /// was built with.
    Short,
    /// An append found the lent accumulator full.
    Full,
    /// A unit fired a virtual after an `On` unit of that virtual had already
    /// been asked in the frame.
    FiredBehind,
}

/// The reference runner over a world `W` of lent stores.
///
/// `K` is how the frame is kept and `V` the sequence the records are visited
/// in. The defaults are the reference: [`Faithful`] and [`Ascending`].
///
/// ```
/// use arvo::{Hot, USize, Uint};
/// use hilavitkutin_api::store::Column;
/// use hilavitkutin_conformance::{Lent, Oracle};
///
/// type Num = Uint<32, Hot>;
///
/// let mut cells = [Num::from_raw(3), Num::from_raw(4)];
/// let world = (Lent::<Column<Num>>::over(&mut cells), ());
/// let mut oracle = Oracle::<_>::new(world, USize(2)).unwrap();
/// // A frame of no units leaves every store as it was.
/// oracle.frame(&()).unwrap();
/// assert_eq!(oracle.of::<Column<Num>, _>().values()[1].to_raw(), 4);
/// ```
pub struct Oracle<W, K = Faithful, V = Ascending> {
    world:   W,
    records: USize,
    kept:    PhantomData<(K, V)>,
}

impl<W: World, K: Keeping, V: Visiting> Oracle<W, K, V> {
    /// A runner over `world`, every column of which holds `records` records.
    ///
    /// Refuses with [`Refusal::Short`] where a column or a side holds fewer.
    pub fn new(world: W, records: USize) -> Outcome<Self, Refusal> {
        if !world.holds(records).0 {
            return Outcome::Err(Refusal::Short);
        }
        Outcome::Ok(Oracle {
            world,
            records,
            kept: PhantomData,
        })
    }

    /// One frame: the units in the sequence listed, then the boundary.
    pub fn frame<'w, Us: Units<'w, W, P>, P>(&mut self, units: &Us) -> Outcome<(), Refusal> {
        if let Outcome::Err(refusal) = self.units(units) {
            return Outcome::Err(refusal);
        }
        self.boundary();
        Outcome::Ok(())
    }

    /// The first half of a frame: the units in the sequence listed, with the
    /// boundary still to come.
    pub fn units<'w, Us: Units<'w, W, P>, P>(&mut self, units: &Us) -> Outcome<(), Refusal> {
        self.world.open::<K>();
        units.run::<K, V>(&mut self.world, self.records)
    }

    /// The second half of a frame: each buffered column that was written
    /// brings its live side in.
    pub fn boundary(&mut self) {
        self.world.close::<K>(self.records);
    }

    /// The storage of the store `S`.
    pub fn of<'w, S: Kept, P>(&self) -> &S::Cells<'w>
    where
        W: Find<'w, S, P>,
    {
        self.world.cells()
    }
}
