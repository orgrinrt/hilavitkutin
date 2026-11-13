//! E2 runner 6: rows in order, one pass per kernel, columns, and the four kernels.

use e2_unit::{self as u, All, Crossed};
use e2_vocab::{Draw, Elem, Emit, Has, Off, Place, Project, Reach, Reads, Window};
use rg_owed::{Arr, Col, SubOf, Vals};
use rg_plumb::{ArrOf, CAP, ColsOf, EvLog, N, Outcome, SoaCore, TICKS};

use crate::kernel as k;

pub struct Handle<'a, L: Project> {
    core: SoaCore<'a, L>,
    tick: u32,
}

impl<'a, L: Project> Reads<L> for Handle<'a, L> {
    #[inline(always)]
    fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        L: Has<M, P>,
    {
        self.core.get::<M, P>()
    }
}

impl<'a, L: Project, R: Reach> Window<L, R> for Handle<'a, L> {
    #[inline(always)]
    fn at<M: Elem, P: Place>(&self, _member: M, off: Off) -> M::Value
    where
        L: Has<M, P>,
    {
        self.core
            .at::<M, P>((off.east, off.north), (R::EAST, R::NORTH))
    }
}

impl<'a, L: Project> Draw for Handle<'a, L> {
    #[inline(always)]
    fn draw(&self) -> u32 {
        rg_plumb::draw(self.tick, self.core.row as u32)
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

fn handle<'a, L: SubOf<All, Ps>, Ps>(
    all: &ColsOf<'a, All>,
    row: usize,
    tick: u32,
) -> Handle<'a, L> {
    Handle {
        core: SoaCore {
            cols: L::sub::<Col<'a>>(all),
            row,
        },
        tick,
    }
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
        }
        for row in 0 .. N {
            let h = handle::<u::SmoothReads, _>(&cols, row, tick);
            let mut w = vals::<u::SmoothWrites, _>(&cols, row);
            u::smooth(&h, &h, &mut w);
            put(&mut next, row, &w);
        }
        for row in 0 .. N {
            let h = handle::<u::CrossReads, _>(&cols, row, tick);
            let mut s = Sink {
                log: &mut log,
                tick,
                row: row as u32,
            };
            u::cross(&h, &mut s);
        }
        for row in 0 .. N {
            let h = handle::<k::JitterReads, _>(&cols, row, tick);
            let mut w = vals::<k::JitterWrites, _>(&cols, row);
            k::jitter(&h, &h, &mut w);
            put(&mut next, row, &w);
        }
        cur = next;
    }
    log.settle();
    Outcome {
        state:  cur,
        events: log,
    }
}
