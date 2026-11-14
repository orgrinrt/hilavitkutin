//! The arms that are not the engine: the bars it is measured against.
//!
//! Three shapes, each written once and used by the arms that share it. The
//! fused shape computes the whole frame per record in registers and writes
//! only the three output columns; `hand` runs it on one thread, `hand_par`
//! over the engine's core budget, and the negative control runs it with one
//! stage dropped. The preallocated column shape materialises every
//! intermediate, one pass per stage, into buffers allocated before the timed
//! region; that is `rust_nostd`. The collected column shape is the same twelve
//! passes written the way most std code is, `map` and `collect` into fresh
//! vectors every frame; `rust_std` runs it over all records and `rust_std_par`
//! over one chunk of records per worker.
//!
//! Both chunked arms run on a `Crew` built with the arm: its workers are
//! spawned once and parked between frames on the engine pool's own frame
//! protocol, as the engine arm's are, and the core budget is read once there.
//! Threads spawned and joined inside every timed frame, as these arms first
//! did, are a cost one side of the comparison carries and the other does not.

use evs_support::Crew;

use crate::engine::threads;
use crate::{
    Arm,
    ArmKind,
    FrameOut,
    Heavy,
    K_A1,
    K_A2,
    K_A3,
    K_B1,
    K_B2,
    K_B3,
    K_C1,
    K_C2,
    K_C3,
    K_T1,
    K_T2,
    Kernel,
    Light,
    Regime,
    StateArm,
    fnv1a_u32,
    join,
    record,
    record_without_a2,
};

/// The input buffer and the three output columns, which every plain arm
/// keeps between calls.
struct Outs {
    inp: Vec<u32>,
    j:   Vec<u32>,
    t1:  Vec<u32>,
    t2:  Vec<u32>,
}

impl Outs {
    fn new(n: usize) -> Self {
        Self {
            inp: vec![0; n],
            j:   vec![0; n],
            t1:  vec![0; n],
            t2:  vec![0; n],
        }
    }

    fn load(&mut self, records: &[u32]) {
        assert_eq!(
            records.len(),
            self.inp.len(),
            "frame_wide: input carries a different record count"
        );
        self.inp.copy_from_slice(records);
    }

    fn digest(&self) -> FrameOut {
        [fnv1a_u32(&self.j), fnv1a_u32(&self.t1), fnv1a_u32(&self.t2)]
    }
}

// ----- the fused shape -----

/// The fused frame over one range of records, for any per-record function.
#[inline(always)]
fn fused_range<R: Fn(u32) -> (u32, u32, u32)>(
    rec: R,
    inp: &[u32],
    j: &mut [u32],
    t1: &mut [u32],
    t2: &mut [u32],
) {
    for (((&v, j), t1), t2) in inp
        .iter()
        .zip(j.iter_mut())
        .zip(t1.iter_mut())
        .zip(t2.iter_mut())
    {
        let (a, b, c) = rec(v);
        *j = a;
        *t1 = b;
        *t2 = c;
    }
}

/// The fused frame split into one contiguous chunk of records per worker.
fn fused_par<R: Fn(u32) -> (u32, u32, u32) + Copy + Sync>(rec: R, o: &mut Outs, crew: &mut Crew) {
    crew.for_chunks(
        &o.inp,
        [&mut o.j[..], &mut o.t1[..], &mut o.t2[..]],
        &|inp, [j, t1, t2]| fused_range(rec, inp, j, t1, t2),
    );
}

// ----- the preallocated column shape -----

/// The nine pipeline columns `rust_nostd` materialises, allocated once.
struct Mid {
    a1: Vec<u32>,
    a2: Vec<u32>,
    a3: Vec<u32>,
    b1: Vec<u32>,
    b2: Vec<u32>,
    b3: Vec<u32>,
    c1: Vec<u32>,
    c2: Vec<u32>,
    c3: Vec<u32>,
}

impl Mid {
    fn new(n: usize) -> Self {
        Self {
            a1: vec![0; n],
            a2: vec![0; n],
            a3: vec![0; n],
            b1: vec![0; n],
            b2: vec![0; n],
            b3: vec![0; n],
            c1: vec![0; n],
            c2: vec![0; n],
            c3: vec![0; n],
        }
    }
}

/// One stage over preallocated slices: an index loop and nothing else.
#[inline(always)]
fn pass<KN: Kernel>(src: &[u32], dst: &mut [u32], k: u32) {
    let n = dst.len();
    let src = &src[.. n];
    let mut i = 0;
    while i < n {
        dst[i] = KN::apply(src[i], k);
        i += 1;
    }
}

/// The twelve passes over preallocated slices, in dependency order.
fn columns_preallocated<KN: Kernel>(o: &mut Outs, m: &mut Mid) {
    pass::<KN>(&o.inp, &mut m.a1, K_A1);
    pass::<KN>(&m.a1, &mut m.a2, K_A2);
    pass::<KN>(&m.a2, &mut m.a3, K_A3);
    pass::<KN>(&o.inp, &mut m.b1, K_B1);
    pass::<KN>(&m.b1, &mut m.b2, K_B2);
    pass::<KN>(&m.b2, &mut m.b3, K_B3);
    pass::<KN>(&o.inp, &mut m.c1, K_C1);
    pass::<KN>(&m.c1, &mut m.c2, K_C2);
    pass::<KN>(&m.c2, &mut m.c3, K_C3);
    let n = o.j.len();
    let mut i = 0;
    while i < n {
        o.j[i] = join::<KN>(m.a3[i], m.b3[i], m.c3[i]);
        i += 1;
    }
    pass::<KN>(&o.inp, &mut o.t1, K_T1);
    pass::<KN>(&o.inp, &mut o.t2, K_T2);
}

// ----- the collected column shape -----

/// One stage the way most std code writes it: map and collect.
#[inline(always)]
fn collect<KN: Kernel>(src: &[u32], k: u32) -> Vec<u32> {
    src.iter().map(|&v| KN::apply(v, k)).collect()
}

/// The twelve passes over one range of records, each collected into a fresh
/// vector, returning the three output columns for that range.
fn columns_collected<KN: Kernel>(inp: &[u32]) -> [Vec<u32>; 3] {
    let a3 = collect::<KN>(&collect::<KN>(&collect::<KN>(inp, K_A1), K_A2), K_A3);
    let b3 = collect::<KN>(&collect::<KN>(&collect::<KN>(inp, K_B1), K_B2), K_B3);
    let c3 = collect::<KN>(&collect::<KN>(&collect::<KN>(inp, K_C1), K_C2), K_C3);
    let j = a3
        .iter()
        .zip(&b3)
        .zip(&c3)
        .map(|((&a, &b), &c)| join::<KN>(a, b, c))
        .collect();
    [j, collect::<KN>(inp, K_T1), collect::<KN>(inp, K_T2)]
}

/// The collected shape over one chunk of records per worker, each worker
/// writing its own chunk of the three output columns.
///
/// The copy out of the collected vectors stays, because collecting into fresh
/// vectors every frame is what this arm is the bar for: `rust_std` ends a frame
/// holding three new vectors and moves them into place, and a chunked form has
/// nothing to move into, since the outputs are one allocation each. What
/// changed is where the copy runs. It was serial on the calling thread after
/// every chunk had been joined; it now runs on the worker that produced the
/// chunk, as `fused_par` writes its own chunk, so the one piece of per-chunk
/// work the arm cannot shed is spread like the rest of it.
fn columns_collected_par<KN: Kernel>(o: &mut Outs, crew: &mut Crew) {
    crew.for_chunks(
        &o.inp,
        [&mut o.j[..], &mut o.t1[..], &mut o.t2[..]],
        &|inp, [j, t1, t2]| {
            let [pj, pt1, pt2] = columns_collected::<KN>(inp);
            j.copy_from_slice(&pj);
            t1.copy_from_slice(&pt1);
            t2.copy_from_slice(&pt2);
        },
    );
}

// ----- the arms -----

fn build_for<KN: Kernel + Send>(kind: ArmKind, n: usize) -> Box<dyn Arm<FrameOut>> {
    let load = |s: &mut Outs, r: &[u32]| s.load(r);
    let digest = |s: &mut Outs| s.digest();
    match kind {
        ArmKind::Hand => {
            Box::new(StateArm::new(
                Outs::new(n),
                load,
                |o: &mut Outs| fused_range(record::<KN>, &o.inp, &mut o.j, &mut o.t1, &mut o.t2),
                digest,
            ))
        },
        ArmKind::HandPar => {
            Box::new(StateArm::new(
                (Outs::new(n), Crew::new(threads())),
                |s: &mut (Outs, Crew), r: &[u32]| s.0.load(r),
                |s: &mut (Outs, Crew)| fused_par(record::<KN>, &mut s.0, &mut s.1),
                |s: &mut (Outs, Crew)| s.0.digest(),
            ))
        },
        ArmKind::DropStage => {
            Box::new(StateArm::new(
                Outs::new(n),
                load,
                |o: &mut Outs| {
                    fused_range(
                        record_without_a2::<KN>,
                        &o.inp,
                        &mut o.j,
                        &mut o.t1,
                        &mut o.t2,
                    )
                },
                digest,
            ))
        },
        ArmKind::RustNostd => {
            Box::new(StateArm::new(
                (Outs::new(n), Mid::new(n)),
                |s: &mut (Outs, Mid), r: &[u32]| s.0.load(r),
                |s: &mut (Outs, Mid)| columns_preallocated::<KN>(&mut s.0, &mut s.1),
                |s: &mut (Outs, Mid)| s.0.digest(),
            ))
        },
        ArmKind::RustStd => {
            Box::new(StateArm::new(
                Outs::new(n),
                load,
                |o: &mut Outs| {
                    let [j, t1, t2] = columns_collected::<KN>(&o.inp);
                    o.j = j;
                    o.t1 = t1;
                    o.t2 = t2;
                },
                digest,
            ))
        },
        ArmKind::RustStdPar => {
            Box::new(StateArm::new(
                (Outs::new(n), Crew::new(threads())),
                |s: &mut (Outs, Crew), r: &[u32]| s.0.load(r),
                |s: &mut (Outs, Crew)| columns_collected_par::<KN>(&mut s.0, &mut s.1),
                |s: &mut (Outs, Crew)| s.0.digest(),
            ))
        },
        ArmKind::Engine | ArmKind::EnginePar | ArmKind::EngineDropStage => {
            unreachable!("frame_wide: the engine arms are built in `engine`")
        },
    }
}

/// Build one plain arm for one regime.
pub fn build(kind: ArmKind, regime: Regime, n: usize) -> Box<dyn Arm<FrameOut>> {
    match regime {
        Regime::Heavy => build_for::<Heavy>(kind, n),
        Regime::Light => build_for::<Light>(kind, n),
    }
}
