//! What a kernel or a close unit is handed: the committed view, a staged
//! handle, and the landed stores.

use core::marker::PhantomData;

use notko_hlist::Contains;

use crate::lane::{FrameLane, LandedStore, Lane};
use crate::registry::{Pick, Registry};

/// A row of the frame, as an opaque brand.
///
/// Built only by the engine and only from `Rows`: no order, no arithmetic and
/// no conversion to or from an integer, so a kernel cannot reach a row the
/// step did not hand it.
#[derive(Clone, Copy)]
pub struct Row(u32);

impl Row {
    pub(crate) const fn at(self) -> usize {
        self.0 as usize
    }
}

/// Every row of the frame, in order.
pub struct Rows {
    next: u32,
    end:  u32,
}

impl Rows {
    pub(crate) const fn new(end: u32) -> Self {
        Rows {
            next: 0,
            end,
        }
    }
}

impl Iterator for Rows {
    type Item = Row;

    fn next(&mut self) -> Option<Row> {
        if self.next == self.end {
            return None;
        }
        let row = Row(self.next);
        self.next += 1;
        Some(row)
    }
}

/// The committed frame, for the step.
///
/// `'s` is the step: the view borrows the committed buffers, which the runner
/// replaces when the step ends, so nothing can keep a view past it. `R` is the
/// holder's read list, and a read of a lane outside it does not compile.
///
/// Copy, and Sync whenever every lane's item is, which `Lane::Item` requires:
/// it is shared slices and a phantom, so it is readable from every thread with
/// no `unsafe impl`. It has no write method.
pub struct View<'s, Reg: Registry, R> {
    lanes:  Reg::Shared<'s>,
    rows:   u32,
    _reads: PhantomData<fn() -> R>,
}

impl<Reg: Registry, R> Clone for View<'_, Reg, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Reg: Registry, R> Copy for View<'_, Reg, R> {}

impl<'s, Reg: Registry, R> View<'s, Reg, R> {
    pub(crate) const fn new(lanes: Reg::Shared<'s>, rows: u32) -> Self {
        View {
            lanes,
            rows,
            _reads: PhantomData,
        }
    }

    /// Every row of the frame.
    pub const fn rows(&self) -> Rows {
        Rows::new(self.rows)
    }

    /// One row of a lane the read list names, as committed at the step's
    /// start.
    pub fn get<X, P>(&self, _lane: X, row: Row) -> X::Item
    where
        X: FrameLane,
        R: Contains<X>,
        Reg::Shared<'s>: Pick<X, P, V = &'s [X::Item]>,
    {
        self.lanes.pick()[row.at()]
    }
}

/// One staged lane of a block kernel's write list, held for the step.
///
/// Write only: there is no read method, so what a kernel stages it cannot read
/// back within the step. Not Copy and not Clone, so it is handed on by move and
/// held by one owner at a time.
pub struct StagedLane<'s, X: Lane> {
    lane: &'s mut [X::Item],
}

impl<'s, X: Lane> StagedLane<'s, X> {
    pub(crate) const fn new(lane: &'s mut [X::Item]) -> Self {
        StagedLane {
            lane,
        }
    }

    /// Stages one row.
    pub fn set(&mut self, row: Row, value: X::Item) {
        self.lane[row.at()] = value;
    }
}

/// The landed stores a close unit names, after the step has landed.
///
/// No view can be had from it, and no frame lane can be named through it.
pub struct Landed<'c, Reg: Registry, S> {
    lanes:   Reg::Excl<'c>,
    rows:    u32,
    _stores: PhantomData<fn() -> S>,
}

impl<'c, Reg: Registry, S> Landed<'c, Reg, S> {
    pub(crate) const fn new(lanes: Reg::Excl<'c>, rows: u32) -> Self {
        Landed {
            lanes,
            rows,
            _stores: PhantomData,
        }
    }

    /// Every row of the stores.
    pub const fn rows(&self) -> Rows {
        Rows::new(self.rows)
    }

    /// One row of a store the close unit names.
    pub fn get<X, P>(&self, _store: X, row: Row) -> X::Item
    where
        X: LandedStore,
        S: Contains<X>,
        Reg::Excl<'c>: Pick<X, P, V = &'c mut [X::Item]>,
    {
        self.lanes.pick()[row.at()]
    }

    /// Writes one row of a store the close unit names.
    pub fn set<X, P>(&mut self, _store: X, row: Row, value: X::Item)
    where
        X: LandedStore,
        S: Contains<X>,
        Reg::Excl<'c>: Pick<X, P, V = &'c mut [X::Item]>,
    {
        self.lanes.pick_mut()[row.at()] = value;
    }
}
