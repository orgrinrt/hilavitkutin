//! E1 runner 5: a registry of 64 stores, of which the unit reads 4, columns, rows in
//! order. The unit's lists are projected out of the registry's list by place.

#![no_std]

use e1_unit::{self as u, All, Crossed, Heat, Lag, Pos, Vel};
use e1_vocab::{Ctx, Decl, Elem, Has, Off, Place, Project, Reach, Runner};
use rg_owed::{Arr, Col, SubOf, Vals};
use rg_plumb::{ArrOf, CAP, ColsOf, EvLog, N, Outcome, SoaCore, TICKS, registry};

/// The world this runner keeps: 64 stores.
pub type Reg = registry!(Pos, Vel, Lag, Heat);

pub struct Registry;

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

impl Runner for Registry {
    type Cx<'a, D: Decl>
        = Cx<'a, D>
    where
        D::Event: 'a;
}

fn cx<'a, D: Decl, Ps>(
    cur: &'a ArrOf<Reg, N>,
    row: usize,
    tick: u32,
    log: &'a mut EvLog<D::Event, CAP>,
) -> Cx<'a, D>
where
    D::Reads: SubOf<Reg, Ps>,
{
    let reg = Reg::cols::<N>(cur);
    Cx {
        core: SoaCore {
            cols: <D::Reads as SubOf<Reg, Ps>>::sub::<Col<'a>>(&reg),
            row,
        },
        log,
        tick,
    }
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
    let mut quiet = EvLog::<(), CAP>::new();
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
            let c = cx::<u::Smooth, _>(&cur, row, tick, &mut quiet);
            let mut w = vals::<u::SmoothWrites, _>(&cols, row);
            u::smooth::<Registry>(&c, &mut w);
            put(&mut next, row, &w);
        }
        for row in 0 .. N {
            let mut c = cx::<u::Cross, _>(&cur, row, tick, &mut log);
            u::cross::<Registry>(&mut c);
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
