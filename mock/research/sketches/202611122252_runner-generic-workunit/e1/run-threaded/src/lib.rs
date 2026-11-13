//! E1 runner 4: the rows split across two scoped threads, columns. The split is written
//! generic over the world and the event, so the bounds it needs are the compiler's.

use e1_unit::{self as u, All, Crossed};
use e1_vocab::{Ctx, Decl, Elem, Has, Off, Place, Project, Reach, Runner};
use rg_owed::{Arr, Col, ColMut, SubOf, Vals};
use rg_plumb::{ArrOf, CAP, ColsOf, EvLog, N, Outcome, SoaCore, TICKS};

pub struct Threaded;

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

impl Runner for Threaded {
    type Cx<'a, D: Decl>
        = Cx<'a, D>
    where
        D::Event: 'a;
}

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

/// Rows `lo..hi`, written into `out`, which holds exactly those rows.
fn span(
    cur: &ArrOf<All, N>,
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
    let cols = All::cols::<N>(cur);
    let mut quiet = EvLog::<(), CAP>::new();
    for row in lo .. hi {
        let r = vals::<u::IntegrateReads, _>(&cols, row);
        let mut w = vals::<u::IntegrateWrites, _>(&cols, row);
        u::integrate(&r, &mut w);
        put(&mut out, row - lo, &w);
    }
    for row in lo .. hi {
        let c = cx::<u::Smooth, _>(cur, row, tick, &mut quiet);
        let mut w = vals::<u::SmoothWrites, _>(&cols, row);
        u::smooth::<Threaded>(&c, &mut w);
        put(&mut out, row - lo, &w);
    }
    for row in lo .. hi {
        let mut c = cx::<u::Cross, _>(cur, row, tick, log);
        u::cross::<Threaded>(&mut c);
    }
}

/// Two scoped threads over two halves. Generic, so its bounds are what threading costs.
fn split2<'a, S: Project, E>(
    cur: &S::Prod<Arr<N>>,
    out: S::Prod<ColMut<'a>>,
    mid: usize,
    n: usize,
    logs: (&mut EvLog<E, CAP>, &mut EvLog<E, CAP>),
    f: impl Fn(&S::Prod<Arr<N>>, S::Prod<ColMut<'a>>, usize, usize, &mut EvLog<E, CAP>) + Sync,
) where
    S::Prod<Arr<N>>: Sync,
    S::Prod<ColMut<'a>>: Send,
    E: Send,
{
    let (a, b) = S::split_mut(out, mid);
    let (la, lb) = logs;
    let f = &f;
    std::thread::scope(|s| {
        s.spawn(move || f(cur, a, 0, mid, la));
        s.spawn(move || f(cur, b, mid, n, lb));
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
            let out = All::cols_mut::<N>(&mut next);
            split2::<All, Crossed>(
                &cur,
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
