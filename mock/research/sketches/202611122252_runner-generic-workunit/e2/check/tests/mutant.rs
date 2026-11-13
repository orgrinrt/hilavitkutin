//! A runner with one defect in its plumbing, its window dropping the north axis, run on
//! the unchanged unit: the comparison against the model must see it.

extern crate e2_unit as unit;
extern crate e2_vocab as vocab;

#[path = "../../../shared/model.rs"]
mod model;

use rg_owed::{Arr, Col, SubOf, Vals};
use rg_plumb::{ArrOf, CAP, ColsOf, EvLog, N, Outcome, SoaCore, TICKS};
use unit::{self as u, All, Crossed};
use vocab::{Elem, Emit, Has, Off, Place, Project, Reach, Reads, Window};

use crate::model::{bytes, events, init, model, store};

struct Handle<'a, L: Project>(SoaCore<'a, L>);

impl<'a, L: Project> Reads<L> for Handle<'a, L> {
    fn get<M: Elem, P: Place>(&self, _member: M) -> M::Value
    where
        L: Has<M, P>,
    {
        self.0.get::<M, P>()
    }
}

impl<'a, L: Project, R: Reach> Window<L, R> for Handle<'a, L> {
    /// The defect: the north offset is dropped, a one-axis neighbour read.
    fn at<M: Elem, P: Place>(&self, _member: M, off: Off) -> M::Value
    where
        L: Has<M, P>,
    {
        self.0.at::<M, P>((off.east, 0), (R::EAST, R::NORTH))
    }
}

struct Sink<'l, E> {
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

fn run(init: &ArrOf<All, N>) -> Outcome<ArrOf<All, N>, Crossed> {
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
        cur = next;
    }
    log.settle();
    Outcome {
        state:  cur,
        events: log,
    }
}

#[test]
fn a_runner_whose_window_drops_an_axis_is_caught() {
    let s = init();
    let m = model(&s);
    let o = run(&s);
    assert!(
        bytes(&o.state) != bytes(&store(&s, &m)),
        "a one-axis window ends on the model's bytes"
    );
    assert_eq!(events(&o), m.events, "the emitter does not read the window");
}
