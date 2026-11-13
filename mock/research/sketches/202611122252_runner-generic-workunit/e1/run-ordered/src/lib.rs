//! E1 runner 1: rows in order, one pass per kernel, columns.

#![no_std]

use e1_unit::{self as u, All, Crossed};
use e1_vocab::{Ctx, Decl, Elem, Has, Off, Place, Project, Reach, Runner};
use rg_owed::{Arr, Col, SubOf, Vals};
use rg_plumb::{ArrOf, CAP, ColsOf, EvLog, N, Outcome, SoaCore, TICKS};

pub struct Ordered;

pub struct Cx<'a, D: Decl> {
    core: SoaCore<'a, D::Reads>,
    log:  &'a mut EvLog<D::Event, CAP>,
    tick: u32,
}

impl<'a, D: Decl> Ctx<D> for Cx<'a, D> {
    #[inline(always)]
    fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        D::Reads: Has<M, P>,
    {
        self.core.get::<M, P>()
    }

    #[inline(always)]
    fn at<M: Elem, P: Place>(&self, _member: M, off: Off) -> M::Value
    where
        D::Reads: Has<M, P>,
    {
        self.core
            .at::<M, P>((off.east, off.north), (D::Reach::EAST, D::Reach::NORTH))
    }

    fn emit(&mut self, e: D::Event) {
        self.log.push(self.tick, self.core.row as u32, e);
    }

    fn draw(&self) -> u32 {
        rg_plumb::draw(self.tick, self.core.row as u32)
    }
}

impl Runner for Ordered {
    type Cx<'a, D: Decl>
        = Cx<'a, D>
    where
        D::Event: 'a;
}

/// The context borrows the store afresh: its one lifetime covers both the columns and
/// the sink, and a product over a carrier is invariant in it, so columns borrowed for
/// the whole tick would hold the sink for the whole tick too.
fn cx<'a, D: Decl, Ps>(
    cur: &'a ArrOf<All, N>,
    row: usize,
    tick: u32,
    log: &'a mut EvLog<D::Event, CAP>,
) -> Cx<'a, D>
where
    D::Reads: SubOf<All, Ps>,
{
    let all = All::cols::<N>(cur);
    Cx {
        core: SoaCore {
            cols: <D::Reads as SubOf<All, Ps>>::sub::<Col<'a>>(&all),
            row,
        },
        log,
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
    let mut quiet = EvLog::<(), CAP>::new();
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
            let c = cx::<u::Smooth, _>(&cur, row, tick, &mut quiet);
            let mut w = vals::<u::SmoothWrites, _>(&cols, row);
            u::smooth::<Ordered>(&c, &mut w);
            put(&mut next, row, &w);
        }
        for row in 0 .. N {
            let mut c = cx::<u::Cross, _>(&cur, row, tick, &mut log);
            u::cross::<Ordered>(&mut c);
        }
        cur = next;
    }
    log.settle();
    Outcome {
        state:  cur,
        events: log,
    }
}
