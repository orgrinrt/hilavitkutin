//! E2 runner 2: rows visited in a seeded permutation, over records (one product of
//! values per row) held in permuted slots rather than columns.

#![no_std]

use e2_unit::{self as u, All, Crossed};
use e2_vocab::{Elem, Emit, Has, Off, Place, Project, Reach, Reads, Window};
use rg_owed::{SubOf, Val, Vals};
use rg_plumb::{
    AosCore,
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

pub struct Handle<'a, L: Project, Ps>(AosCore<'a, All, L, Ps>);

impl<'a, L: SubOf<All, Ps>, Ps> Reads<L> for Handle<'a, L, Ps> {
    #[inline(always)]
    fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        L: Has<M, P>,
    {
        self.0.get::<M, P>()
    }
}

impl<'a, L: SubOf<All, Ps>, Ps, R: Reach> Window<L, R> for Handle<'a, L, Ps> {
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

fn handle<'a, L: SubOf<All, Ps>, Ps>(
    recs: &'a [ValsOf<All>],
    slot_of: &'a [u32],
    row: usize,
) -> Handle<'a, L, Ps> {
    Handle(AosCore::new(recs, slot_of, row))
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
            let h = handle::<u::SmoothReads, _>(&cur, &slot_of, row);
            let mut w = vals::<u::SmoothWrites, _>(&cur[slot]);
            u::smooth(&h, &h, &mut w);
            put(&mut next[slot], &w);
        }
        for slot in 0 .. N {
            let row = row_of[slot] as usize;
            let h = handle::<u::CrossReads, _>(&cur, &slot_of, row);
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
    let mut state = *init;
    to_columns::<All, N>(&cur, &slot_of, &mut state);
    Outcome {
        state,
        events: log,
    }
}
