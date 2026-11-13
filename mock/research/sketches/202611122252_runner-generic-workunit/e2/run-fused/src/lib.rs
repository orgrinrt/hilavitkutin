//! E2 runner 3: fused, every kernel in one pass over the rows, columns.

#![no_std]

use e2_unit::{self as u, All, Crossed};
use e2_vocab::{Elem, Emit, Has, Off, Place, Project, Reach, Reads, Window};
use rg_owed::{Arr, Col, SubOf, Vals};
use rg_plumb::{ArrOf, CAP, ColsOf, EvLog, N, Outcome, SoaCore, TICKS};

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

fn handle<'a, L: SubOf<All, Ps>, Ps>(all: &ColsOf<'a, All>, row: usize) -> Handle<'a, L> {
    Handle(SoaCore {
        cols: L::sub::<Col<'a>>(all),
        row,
    })
}

fn vals<L: SubOf<All, Ps>, Ps>(all: &ColsOf<'_, All>, row: usize) -> Vals<L> {
    Vals(L::read_row::<Col<'_>>(&L::sub::<Col<'_>>(all), row))
}

fn put<W: SubOf<All, Ps>, Ps>(next: &mut ArrOf<All, N>, row: usize, w: &Vals<W>) {
    W::put_row::<Arr<N>>(next, row, &w.0);
}

pub fn run(init: &ArrOf<All, N>) -> Outcome<ArrOf<All, N>, Crossed> {
    let mut cur = *init;
    let mut log = EvLog::new();
    for tick in 0 .. TICKS {
        let mut next = cur;
        let cols = All::cols::<N>(&cur);
        for row in 0 .. N {
            let r = vals::<u::IntegrateReads, _>(&cols, row);
            let mut w = vals::<u::IntegrateWrites, _>(&cols, row);
            u::integrate(&r, &mut w);
            put(&mut next, row, &w);

            let h = handle::<u::SmoothReads, _>(&cols, row);
            let mut w = vals::<u::SmoothWrites, _>(&cols, row);
            u::smooth(&h, &h, &mut w);
            put(&mut next, row, &w);

            let h = handle::<u::CrossReads, _>(&cols, row);
            let mut s = Sink {
                log: &mut log,
                tick,
                row: row as u32,
            };
            u::cross(&h, &mut s);
        }
        cur = next;
    }
    log.settle();
    Outcome {
        state:  cur,
        events: log,
    }
}
