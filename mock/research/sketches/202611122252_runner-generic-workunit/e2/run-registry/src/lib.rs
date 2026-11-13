//! E2 runner 5: a registry of 64 stores, of which the unit reads 4, columns, rows in
//! order. The unit's lists are projected out of the registry's list by place.

#![no_std]

use e2_unit::{self as u, All, Crossed, Heat, Lag, Pos, Vel};
use e2_vocab::{Elem, Emit, Has, Off, Place, Project, Reach, Reads, Window};
use rg_owed::{Arr, Col, SubOf, Vals};
use rg_plumb::{ArrOf, CAP, ColsOf, EvLog, N, Outcome, SoaCore, TICKS, registry};

/// The world this runner keeps: 64 stores.
pub type Reg = registry!(Pos, Vel, Lag, Heat);

pub struct Handle<'a, L: Project>(SoaCore<'a, L>);

impl<'a, L: Project> Reads<L> for Handle<'a, L> {
    #[inline(always)]
    fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        L: Has<M, P>,
    {
        self.0.get::<M, P>()
    }
}

impl<'a, L: Project, R: Reach> Window<L, R> for Handle<'a, L> {
    #[inline(always)]
    fn at<M: Elem, P: Place>(&self, _member: M, off: Off) -> M::Value
    where
        L: Has<M, P>,
    {
        self.0
            .at::<M, P>((off.east, off.north), (R::EAST, R::NORTH))
    }
}

pub struct Sink<'l, E> {
    log:  &'l mut EvLog<E, CAP>,
    tick: u32,
    row:  u32,
}

impl<E> Emit<E> for Sink<'_, E> {
    fn emit(&mut self, e: E) {
        self.log.push(self.tick, self.row, e);
    }
}

fn handle<'a, L: SubOf<Reg, Ps>, Ps>(reg: &ColsOf<'a, Reg>, row: usize) -> Handle<'a, L> {
    Handle(SoaCore {
        cols: L::sub::<Col<'a>>(reg),
        row,
    })
}

fn vals<L: SubOf<Reg, Ps>, Ps>(reg: &ColsOf<'_, Reg>, row: usize) -> Vals<L> {
    Vals(L::read_row::<Col<'_>>(&L::sub::<Col<'_>>(reg), row))
}

fn put<W: SubOf<Reg, Ps>, Ps>(next: &mut ArrOf<Reg, N>, row: usize, w: &Vals<W>) {
    W::put_row::<Arr<N>>(next, row, &w.0);
}

fn load<Ps>(reg: &mut ArrOf<Reg, N>, init: &ArrOf<All, N>)
where
    All: SubOf<Reg, Ps>,
{
    for row in 0 .. N {
        All::put_row::<Arr<N>>(reg, row, &All::read_row::<Arr<N>>(init, row));
    }
}

fn unload<Ps>(reg: &ArrOf<Reg, N>, into: &mut ArrOf<All, N>)
where
    All: SubOf<Reg, Ps>,
{
    let cols = All::sub::<Col<'_>>(&Reg::cols::<N>(reg));
    for row in 0 .. N {
        All::write_row::<Arr<N>>(into, row, &All::read_row::<Col<'_>>(&cols, row));
    }
}

pub fn run(init: &ArrOf<All, N>) -> Outcome<ArrOf<All, N>, Crossed> {
    let mut cur = Reg::made::<N>(&Default::default());
    let mut next = Reg::made::<N>(&Default::default());
    load(&mut cur, init);
    let mut log = EvLog::new();
    for tick in 0 .. TICKS {
        Reg::copy_from::<N>(&mut next, &cur);
        let cols = Reg::cols::<N>(&cur);
        for row in 0 .. N {
            let r = vals::<u::IntegrateReads, _>(&cols, row);
            let mut w = vals::<u::IntegrateWrites, _>(&cols, row);
            u::integrate(&r, &mut w);
            put(&mut next, row, &w);
        }
        for row in 0 .. N {
            let h = handle::<u::SmoothReads, _>(&cols, row);
            let mut w = vals::<u::SmoothWrites, _>(&cols, row);
            u::smooth(&h, &h, &mut w);
            put(&mut next, row, &w);
        }
        for row in 0 .. N {
            let h = handle::<u::CrossReads, _>(&cols, row);
            let mut s = Sink {
                log: &mut log,
                tick,
                row: row as u32,
            };
            u::cross(&h, &mut s);
        }
        core::mem::swap(&mut cur, &mut next);
    }
    log.settle();
    let mut state = *init;
    unload(&cur, &mut state);
    Outcome {
        state,
        events: log,
    }
}
