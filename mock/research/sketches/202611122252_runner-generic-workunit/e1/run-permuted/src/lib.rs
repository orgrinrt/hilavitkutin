//! E1 runner 2: rows visited in a seeded permutation, over records (one product of
//! values per row) held in permuted slots rather than columns.
//!
//! The context type is fixed by the declaration alone (`Runner::Cx<'a, D>`), so it
//! cannot carry the places of the declaration's reads in the record, which only the
//! construction site can infer: the projection is erased into a function pointer.

#![no_std]

use e1_unit::{self as u, All, Crossed};
use e1_vocab::{Ctx, Decl, Elem, Has, Off, Place, Reach, Runner};
use rg_owed::{SubOf, Val, Vals};
use rg_plumb::{
    AosFnCore,
    ArrOf,
    CAP,
    EvLog,
    N,
    Outcome,
    SEED,
    TICKS,
    ValsOf,
    invert,
    permutation,
    to_columns,
    to_records,
};

pub struct Permuted;

pub struct Cx<'a, D: Decl> {
    core: AosFnCore<'a, All, D::Reads>,
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

impl Runner for Permuted {
    type Cx<'a, D: Decl>
        = Cx<'a, D>
    where
        D::Event: 'a;
}

fn cx<'a, D: Decl, Ps>(
    recs: &'a [ValsOf<All>],
    slot_of: &'a [u32],
    row: usize,
    tick: u32,
    log: &'a mut EvLog<D::Event, CAP>,
) -> Cx<'a, D>
where
    D::Reads: SubOf<All, Ps>,
{
    Cx {
        core: AosFnCore::new(recs, slot_of, row, <D::Reads as SubOf<All, Ps>>::sub::<Val>),
        log,
        tick,
    }
}

fn vals<L: SubOf<All, Ps>, Ps>(rec: &ValsOf<All>) -> Vals<L> {
    Vals(L::sub::<Val>(rec))
}

fn put<W: SubOf<All, Ps>, Ps>(rec: &mut ValsOf<All>, w: &Vals<W>) {
    W::put_vals(rec, &w.0);
}

pub fn run(init: &ArrOf<All, N>) -> Outcome<ArrOf<All, N>, Crossed> {
    let slot_of = permutation(SEED);
    let row_of = invert(&slot_of);
    let mut cur = to_records::<All, N>(init, &slot_of);
    let mut log = EvLog::new();
    let mut quiet = EvLog::<(), CAP>::new();
    for tick in 0 .. TICKS {
        let mut next = cur;
        for slot in 0 .. N {
            let r = vals::<u::IntegrateReads, _>(&cur[slot]);
            let mut w = vals::<u::IntegrateWrites, _>(&cur[slot]);
            u::integrate(&r, &mut w);
            put(&mut next[slot], &w);
        }
        for slot in 0 .. N {
            let row = row_of[slot] as usize;
            let c = cx::<u::Smooth, _>(&cur, &slot_of, row, tick, &mut quiet);
            let mut w = vals::<u::SmoothWrites, _>(&cur[slot]);
            u::smooth::<Permuted>(&c, &mut w);
            put(&mut next[slot], &w);
        }
        for slot in 0 .. N {
            let row = row_of[slot] as usize;
            let mut c = cx::<u::Cross, _>(&cur, &slot_of, row, tick, &mut log);
            u::cross::<Permuted>(&mut c);
        }
        cur = next;
    }
    log.settle();
    let mut state = *init;
    to_columns::<All, N>(&cur, &slot_of, &mut state);
    Outcome {
        state,
        events: log,
    }
}
