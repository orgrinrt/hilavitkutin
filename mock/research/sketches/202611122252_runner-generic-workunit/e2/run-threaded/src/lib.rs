//! E2 runner 4: the rows split across two scoped threads, columns. The split is written
//! generic over the world and the event, so the bounds it needs are the compiler's.

use e2_unit::{self as u, All, Crossed};
use e2_vocab::{Elem, Emit, Has, Off, Place, Project, Reach, Reads, Window};
use rg_owed::{Col, ColMut, SubOf, Vals};
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

/// Rows `lo..hi`, written into `out`, which holds exactly those rows.
fn span(
    cols: &ColsOf<'_, All>,
    mut out: <All as Project>::Prod<ColMut<'_>>,
    lo: usize,
    hi: usize,
    tick: u32,
    log: &mut EvLog<Crossed, CAP>,
) {
    fn put<W: SubOf<All, Ps>, Ps>(
        out: &mut <All as Project>::Prod<ColMut<'_>>,
        i: usize,
        w: &Vals<W>,
    ) {
        W::put_row::<ColMut<'_>>(out, i, &w.0);
    }
    for row in lo .. hi {
        let r = vals::<u::IntegrateReads, _>(cols, row);
        let mut w = vals::<u::IntegrateWrites, _>(cols, row);
        u::integrate(&r, &mut w);
        put(&mut out, row - lo, &w);
    }
    for row in lo .. hi {
        let h = handle::<u::SmoothReads, _>(cols, row);
        let mut w = vals::<u::SmoothWrites, _>(cols, row);
        u::smooth(&h, &h, &mut w);
        put(&mut out, row - lo, &w);
    }
    for row in lo .. hi {
        let h = handle::<u::CrossReads, _>(cols, row);
        let mut s = Sink {
            log: &mut *log,
            tick,
            row: row as u32,
        };
        u::cross(&h, &mut s);
    }
}

/// Two scoped threads over two halves. Generic, so its bounds are what threading costs.
fn split2<'a, S: Project, E>(
    cols: &S::Prod<Col<'a>>,
    out: S::Prod<ColMut<'a>>,
    mid: usize,
    n: usize,
    logs: (&mut EvLog<E, CAP>, &mut EvLog<E, CAP>),
    f: impl Fn(&S::Prod<Col<'a>>, S::Prod<ColMut<'a>>, usize, usize, &mut EvLog<E, CAP>) + Sync,
) where
    S::Prod<Col<'a>>: Sync,
    S::Prod<ColMut<'a>>: Send,
    E: Send,
{
    let (a, b) = S::split_mut(out, mid);
    let (la, lb) = logs;
    let f = &f;
    std::thread::scope(|s| {
        s.spawn(move || f(cols, a, 0, mid, la));
        s.spawn(move || f(cols, b, mid, n, lb));
    });
}

pub fn run(init: &ArrOf<All, N>) -> Outcome<ArrOf<All, N>, Crossed> {
    let mut cur = *init;
    let mut log = EvLog::new();
    for tick in 0 .. TICKS {
        let mut next = cur;
        let mut la = EvLog::new();
        let mut lb = EvLog::new();
        {
            let cols = All::cols::<N>(&cur);
            let out = All::cols_mut::<N>(&mut next);
            split2::<All, Crossed>(
                &cols,
                out,
                N / 2,
                N,
                (&mut la, &mut lb),
                |c, o, lo, hi, l| span(c, o, lo, hi, tick, l),
            );
        }
        log.append(&mut la);
        log.append(&mut lb);
        cur = next;
    }
    log.settle();
    Outcome {
        state:  cur,
        events: log,
    }
}
