//! The plain model every runner is checked against, included by each arm's checks, which
//! name the arm's crates `unit` and `vocab`. It computes the three kernels directly over
//! arrays, and a fourth, the growth probe's, where a kick is given.

use rg_owed::Arr;
use rg_plumb::{ArrOf, EAST, N, NORTH, Outcome, TICKS};
use unit::{All, Crossed, Heat, Lag, Pos, SmoothReach, THRESHOLD, Vel};
use vocab::{Elem, Has, Place, Project, Reach};

pub(crate) fn col<M: Elem<Value = i32>, P: Place>(s: &mut ArrOf<All, N>, _m: M) -> &mut [i32; N]
where
    All: Has<M, P>,
{
    All::pick_mut::<Arr<N>>(s)
}

pub(crate) fn colr<M: Elem<Value = i32>, P: Place>(s: &ArrOf<All, N>, _m: M) -> [i32; N]
where
    All: Has<M, P>,
{
    *All::pick::<Arr<N>>(s)
}

pub(crate) fn init() -> ArrOf<All, N> {
    let mut s = All::made::<N>(&Default::default());
    let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next = || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x
    };
    for r in 0 .. N {
        col(&mut s, Pos)[r] = (next() % 2000) as i32;
        col(&mut s, Vel)[r] = (next() % 121) as i32 - 60;
        col(&mut s, Lag)[r] = (next() % 100) as i32;
    }
    s
}

pub(crate) struct Model {
    pub(crate) pos:    [i32; N],
    pub(crate) vel:    [i32; N],
    pub(crate) lag:    [i32; N],
    pub(crate) heat:   [i32; N],
    pub(crate) events: Vec<(u32, u32, Crossed)>,
}

/// The window's offset on the two-axis key, as the plain model computes it.
pub(crate) fn offset(lag: i32) -> (i32, i32) {
    let (re, rn) = (SmoothReach::EAST, SmoothReach::NORTH);
    let span = 2 * re + 1;
    (
        lag.rem_euclid(span) - re,
        (lag / span).rem_euclid(2 * rn + 1) - rn,
    )
}

/// The three kernels.
pub(crate) fn model(init: &ArrOf<All, N>) -> Model {
    model_with(init, |_, _| 0)
}

/// The three kernels, and `Vel` kicked by `kick(tick, row)` each tick.
pub(crate) fn model_with(init: &ArrOf<All, N>, kick: impl Fn(u32, usize) -> i32) -> Model {
    let mut m = Model {
        pos:    colr(init, Pos),
        vel:    colr(init, Vel),
        lag:    colr(init, Lag),
        heat:   colr(init, Heat),
        events: vec![],
    };
    for tick in 0 .. TICKS {
        let (pos, vel, lag) = (m.pos, m.vel, m.lag);
        for r in 0 .. N {
            let (east, north) = offset(lag[r]);
            let e = (r % EAST) as i32 + east;
            let n = (r / EAST) as i32 + north;
            let nb =
                n.rem_euclid(NORTH as i32) as usize * EAST + e.rem_euclid(EAST as i32) as usize;
            m.heat[r] = pos[r].wrapping_sub(pos[nb]).wrapping_abs();
            m.pos[r] = pos[r].wrapping_add(vel[r]);
            m.vel[r] = vel[r].wrapping_add(kick(tick, r));
            let (from, to) = (pos[r], pos[r].wrapping_add(vel[r]));
            if from < THRESHOLD && to >= THRESHOLD {
                m.events.push((tick, r as u32, Crossed {
                    from,
                    to,
                }));
            }
        }
    }
    m
}

/// The model's columns as a store.
pub(crate) fn store(init: &ArrOf<All, N>, m: &Model) -> ArrOf<All, N> {
    let mut s = *init;
    *col(&mut s, Pos) = m.pos;
    *col(&mut s, Vel) = m.vel;
    *col(&mut s, Lag) = m.lag;
    *col(&mut s, Heat) = m.heat;
    s
}

pub(crate) fn bytes(s: &ArrOf<All, N>) -> Vec<u8> {
    [colr(s, Pos), colr(s, Vel), colr(s, Lag), colr(s, Heat)]
        .iter()
        .flat_map(|c| c.iter().flat_map(|v| v.to_le_bytes()))
        .collect()
}

pub(crate) fn events(o: &Outcome<ArrOf<All, N>, Crossed>) -> Vec<(u32, u32, Crossed)> {
    (0 .. o.events.len())
        .map(|i| *o.events.get(i).unwrap())
        .collect()
}
