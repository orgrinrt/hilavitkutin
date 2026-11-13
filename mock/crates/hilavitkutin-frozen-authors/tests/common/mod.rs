//! The harness that runs the frozen authors: under the hand model, under the
//! reference runner and under each entry of the scheduler, every frame written
//! down by the crate's own `record::words` so that any two of them compare
//! word for word.
//!
//! The workers and the memory provider are the engine's own test support,
//! taken by path so there is one of each.

#![allow(dead_code)]

#[path = "../../../hilavitkutin/tests/stable_provider/mod.rs"]
pub mod stable_provider;
#[path = "../../../hilavitkutin/tests/common/mod.rs"]
pub mod workers;

use arvo::Bool;
use hilavitkutin_conformance::{Verdict, Word};
use hilavitkutin_frozen_authors::{
    Alarms,
    Ash,
    Crossed,
    Edge,
    Glow,
    Heat,
    Level,
    Model,
    Num,
    Peak,
    Pos,
    RECORDS,
    Vel,
    record,
};
use hilavitkutin_providers::ArenaColumnStorage;
use stable_provider::StableProvider;

/// How many frames a run is held over. Long enough for the plane to overheat
/// and for the pool to go even, short enough that no sum leaves 32 bits.
pub const FRAMES: usize = 8;
/// How many records the plane is, as a bare count for the harness.
pub const N: usize = RECORDS.0;
/// What a poisoned live side holds before the first frame.
pub const POISON: u32 = 0xDEAD;

pub fn store() -> ArenaColumnStorage<StableProvider> {
    ArenaColumnStorage::new(StableProvider::new(1 << 19))
}

pub fn num(raw: u32) -> Num {
    Num::from_raw(raw)
}

/// One frame's state as words. `sorted` writes the crossed values sorted, for
/// a comparison between runners that append in different sequences.
#[allow(clippy::too_many_arguments)]
pub fn words_of(
    pos: &[Pos],
    heat: &[Heat],
    alarms: &[Alarms],
    level: &[Level],
    peak: &[Peak],
    glow: &[Glow],
    ash: &[Ash],
    overheat: bool,
    crossed: &[Crossed],
    sorted: bool,
) -> Vec<Word> {
    let mut crossed = crossed.to_vec();
    if sorted {
        crossed.sort_unstable_by_key(|c| c.0.to_raw());
    }
    let mut out = Vec::new();
    record::words(
        pos,
        heat,
        alarms,
        level,
        peak,
        glow,
        ash,
        Bool(overheat),
        &crossed,
        |word| out.push(word),
    );
    out
}

/// The hand model's words, frame by frame, from the starting state. `cross`
/// says whether `Cross` is among the units of the run compared with; without
/// it nothing is appended.
pub fn model_frames(frames: usize, sorted: bool, cross: bool) -> Vec<Vec<Word>> {
    let mut model = Model::new();
    (0 .. frames)
        .map(|_| {
            model.frame();
            let crossed: &[Crossed] = if cross { &model.appended[.. model.crossed.0] } else { &[] };
            words_of(
                &model.pos,
                &model.heat,
                &model.alarms,
                &model.level,
                &model.peak,
                &model.glow,
                &model.ash,
                model.overheat.0,
                crossed,
                sorted,
            )
        })
        .collect()
}

/// Every frame's words, one after the other.
pub fn flat(frames: &[Vec<Word>]) -> Vec<Word> {
    frames.iter().flatten().copied().collect()
}

/// The first frame of `run` that is not `want`, with the law's verdict on it.
pub fn first_failing(run: &[Vec<Word>], want: &[Vec<Word>]) -> Option<(usize, Verdict)> {
    assert_eq!(run.len(), want.len(), "the two runs are as many frames");
    run.iter()
        .zip(want)
        .map(|(run, want)| hilavitkutin_conformance::model(run, want))
        .enumerate()
        .find(|(_, verdict)| *verdict != Verdict::Holds)
}

/// The stores of a frame's words, in the sequence `record::words` writes
/// them, each `N` words long. What follows them is the flag, the count of
/// crossed values and the values.
const STORES: [&str; 7] = ["pos", "heat", "alarms", "level", "peak", "glow", "ash"];

/// The first eight words of `run` that differ from `want`, each as the store
/// it belongs to, the record, what the run wrote and what was wanted.
pub fn wrong(run: &[Word], want: &[Word]) -> Vec<(&'static str, usize, u64, u64)> {
    run.iter()
        .zip(want)
        .enumerate()
        .filter(|(_, (run, want))| run != want)
        .map(|(at, (run, want))| {
            let store = STORES
                .get(at / N)
                .copied()
                .unwrap_or("flag, count or crossed");
            let record = if at < STORES.len() * N { at % N } else { at - STORES.len() * N };
            (store, record, run.to_raw(), want.to_raw())
        })
        .take(8)
        .collect()
}

/// Run `units` under the reference runner from the model's starting state and
/// write every frame down. `$units` is a value of any unit list over the
/// eleven stores. A macro, so that the world's type, a nested pair eleven
/// deep, is never written out.
#[macro_export]
macro_rules! oracle_frames {
    ($keeping:ty, $visiting:ty, $frames:expr, $poison:expr, $sorted:expr, $units:expr) => {{
        let frames: usize = $frames;
        let poison: u32 = $poison;
        let sorted: bool = $sorted;
        let units = $units;
        $crate::oracle_frames!(@run $keeping, $visiting, frames, poison, sorted, units)
    }};
    (@run $keeping:ty, $visiting:ty, $frames:ident, $poison:ident, $sorted:ident, $units:ident) => {{
        use hilavitkutin_api::store::{Accum, Column, Virtual};
        use hilavitkutin_conformance::{Lent, Oracle, Refusal};
        use hilavitkutin_frozen_authors::*;
        use notko::Outcome;
        use $crate::common::{num, words_of};

        let mut start = Model::new();
        let mut heat_live = [Heat(num($poison)); RECORDS.0];
        let mut level_live = [Level(num($poison)); RECORDS.0];
        let world = (
            Lent::<Column<Pos>>::over(&mut start.pos),
            (
                Lent::<Column<Vel>>::over(&mut start.vel),
                (
                    Lent::<Field>::over(&mut start.heat, &mut heat_live),
                    (
                        Lent::<Column<Edge>>::over(&mut start.edge),
                        (
                            Lent::<Accum<Crossed>>::over(&mut start.appended),
                            (
                                Lent::<Virtual<Overheat>>::unfired(),
                                (
                                    Lent::<Column<Alarms>>::over(&mut start.alarms),
                                    (
                                        Lent::<Pool>::over(&mut start.level, &mut level_live),
                                        (
                                            Lent::<Column<Peak>>::over(&mut start.peak),
                                            (
                                                Lent::<Column<Glow>>::over(&mut start.glow),
                                                (Lent::<Column<Ash>>::over(&mut start.ash), ()),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );
        (|| -> Result<Vec<Vec<hilavitkutin_conformance::Word>>, Refusal> {
            let mut oracle = match Oracle::<_, $keeping, $visiting>::new(world, RECORDS) {
                Outcome::Ok(oracle) => oracle,
                Outcome::Err(refusal) => return Err(refusal),
            };
            let mut out = Vec::new();
            for _ in 0 .. $frames {
                if let Outcome::Err(refusal) = oracle.frame(&$units) {
                    return Err(refusal);
                }
                out.push(words_of(
                    oracle.of::<Column<Pos>, _>().values(),
                    oracle.of::<Field, _>().frozen(),
                    oracle.of::<Column<Alarms>, _>().values(),
                    oracle.of::<Pool, _>().frozen(),
                    oracle.of::<Column<Peak>, _>().values(),
                    oracle.of::<Column<Glow>, _>().values(),
                    oracle.of::<Column<Ash>, _>().values(),
                    oracle.of::<Virtual<Overheat>, _>().fired().0,
                    oracle.of::<Accum<Crossed>, _>().appended(),
                    $sorted,
                ));
            }
            Ok(out)
        })()
    }};
}

/// Which entry of the scheduler runs the frames.
#[derive(Copy, Clone, Debug)]
pub enum Entry {
    /// `run`, on the calling thread.
    Run,
    /// `run`, with the host marking the positions changed ahead of every
    /// frame, the way a host marks a column it filled itself.
    RunMarked,
    /// `run_parallel` on this many workers: one, two or four.
    Parallel(usize),
}

/// The stores of a built scheduler, reached through its bindings.
pub struct Stores<'s> {
    pub pos:     *mut Pos,
    pub vel:     *mut Vel,
    pub edge:    *mut Edge,
    pub alarms:  *mut Alarms,
    pub peak:    *mut Peak,
    pub glow:    *mut Glow,
    pub ash:     *mut Ash,
    /// The frozen and the live side of the heat, as they are now.
    pub heat:    Box<dyn Fn() -> [*mut Heat; 2] + 's>,
    /// The frozen and the live side of the levels, as they are now.
    pub level:   Box<dyn Fn() -> [*mut Level; 2] + 's>,
    /// Whether `Overheat` fired in the frame of this number, counted from one.
    pub fired:   Box<dyn Fn(usize) -> bool + 's>,
    /// What the last frame appended.
    pub crossed: Box<dyn Fn() -> Vec<Crossed> + 's>,
}

impl Stores<'_> {
    /// Fill every store from the model's starting state, and the live sides
    /// of the two buffered columns with `poison`.
    ///
    /// # Safety
    ///
    /// Every pointer is a column of `N` records and no frame is running.
    pub unsafe fn fill(&self, poison: u32) {
        let start = Model::new();
        let [heat, heat_live] = (self.heat)();
        let [level, level_live] = (self.level)();
        for i in 0 .. N {
            // SAFETY: inside each reservation, per the contract above.
            unsafe {
                self.pos.add(i).write(start.pos[i]);
                self.vel.add(i).write(start.vel[i]);
                self.edge.add(i).write(start.edge[i]);
                self.alarms.add(i).write(start.alarms[i]);
                self.peak.add(i).write(start.peak[i]);
                self.glow.add(i).write(start.glow[i]);
                self.ash.add(i).write(start.ash[i]);
                heat.add(i).write(start.heat[i]);
                heat_live.add(i).write(Heat(num(poison)));
                level.add(i).write(start.level[i]);
                level_live.add(i).write(Level(num(poison)));
            }
        }
    }

    /// The state after the frame of this number, counted from one, as words.
    ///
    /// # Safety
    ///
    /// Every pointer is a column of `N` initialised records and no frame is
    /// running.
    pub unsafe fn words(&self, frame: usize, sorted: bool) -> Vec<Word> {
        let [heat, _] = (self.heat)();
        let [level, _] = (self.level)();
        // SAFETY: `N` initialised records each, per the contract above.
        unsafe {
            words_of(
                core::slice::from_raw_parts(self.pos, N),
                core::slice::from_raw_parts(heat, N),
                core::slice::from_raw_parts(self.alarms, N),
                core::slice::from_raw_parts(level, N),
                core::slice::from_raw_parts(self.peak, N),
                core::slice::from_raw_parts(self.glow, N),
                core::slice::from_raw_parts(self.ash, N),
                (self.fired)(frame),
                &(self.crossed)(),
                sorted,
            )
        }
    }
}

/// A unit list as the reference runner takes one: `units!(A, B)` is
/// `(A, (B, ()))`.
#[macro_export]
macro_rules! units {
    () => { () };
    ($head:expr $(, $tail:expr)* $(,)?) => { ($head, $crate::units!($($tail),*)) };
}

/// Build a scheduler over the ten stores, with the accumulator ahead of them
/// for `with_cross`, register the units in the sequence given, and write
/// `FRAMES` frames through `$entry` down.
#[macro_export]
macro_rules! scheduled {
    (@stores $builder:expr, [$($unit:expr),+], $which:ident, $entry:expr, $poison:expr, $sorted:expr) => {{
        use hilavitkutin_api::store::{Column, Virtual};
        use hilavitkutin_frozen_authors::*;

        let scheduler = $builder
            .with(Column::<Pos>::new())
            .with(Column::<Vel>::new())
            .with(Field::new())
            .with(Column::<Edge>::new())
            .with(Virtual::<Overheat>::new())
            .with(Column::<Alarms>::new())
            .with(Pool::new())
            .with(Column::<Peak>::new())
            .with(Column::<Glow>::new())
            .with(Column::<Ash>::new())
            $(.with($unit))+
            .build($crate::common::store(), RECORDS)
            .unwrap_or_else(|_| panic!("the units build"));
        $crate::drive!($which, scheduler, $entry, $crate::common::FRAMES, $poison, $sorted)
    }};
    (with_cross, [$($unit:expr),+], $entry:expr, $poison:expr, $sorted:expr) => {
        $crate::scheduled!(
            @stores hilavitkutin::scheduler::Scheduler::builder().with(
                hilavitkutin_api::store::Accum::<hilavitkutin_frozen_authors::Crossed>::new()
            ),
            [$($unit),+], with_cross, $entry, $poison, $sorted
        )
    };
    (without_cross, [$($unit:expr),+], $entry:expr, $poison:expr, $sorted:expr) => {
        $crate::scheduled!(
            @stores hilavitkutin::scheduler::Scheduler::builder(),
            [$($unit),+], without_cross, $entry, $poison, $sorted
        )
    };
}

/// The same units under the scheduler and under the reference runner: the
/// frames of each, the scheduler's first.
#[macro_export]
macro_rules! both {
    ($which:ident, $entry:expr, $poison:expr, $sorted:expr; $($unit:expr),+) => {
        (
            $crate::scheduled!($which, [$($unit),+], $entry, $poison, $sorted),
            $crate::oracle_frames!(
                hilavitkutin_conformance::Faithful,
                hilavitkutin_conformance::Ascending,
                $crate::common::FRAMES,
                0,
                $sorted,
                $crate::units!($($unit),+)
            )
            .expect("no refusal"),
        )
    };
}

/// The stores of a scheduler built by `scheduled!`. The last store registered
/// is the head of the bindings, and the accumulator, where there is one, was
/// registered first.
#[macro_export]
macro_rules! stores {
    (@columns $b:expr, $crossed:ident, $appended:expr) => {{
        let ash = $b;
        let glow = ash.__tail();
        let peak = glow.__tail();
        let pool = peak.__tail();
        let alarms = pool.__tail();
        let overheat = alarms.__tail();
        let edge = overheat.__tail();
        let field = edge.__tail();
        let vel = field.__tail();
        let pos = vel.__tail();
        let $crossed = pos.__tail();
        $crate::common::Stores {
            pos:     pos.__ptr().as_ptr(),
            vel:     vel.__ptr().as_ptr(),
            edge:    edge.__ptr().as_ptr(),
            alarms:  alarms.__ptr().as_ptr(),
            peak:    peak.__ptr().as_ptr(),
            glow:    glow.__ptr().as_ptr(),
            ash:     ash.__ptr().as_ptr(),
            heat:    Box::new(move || [field.__frozen().as_ptr(), field.__live().as_ptr()]),
            level:   Box::new(move || [pool.__frozen().as_ptr(), pool.__live().as_ptr()]),
            fired:   Box::new(move |frame| overheat.__is_set(arvo::USize(frame)).0),
            crossed: Box::new($appended),
        }
    }};
    (with_cross $b:expr) => {
        $crate::stores!(@columns $b, crossed, move || {
            let len = crossed.__len_cell().get().0;
            let base = crossed.__ptr().as_ptr();
            // SAFETY: the last frame appended `len` values from `base` on.
            (0 .. len).map(|i| unsafe { base.add(i).read() }).collect()
        })
    };
    (without_cross $b:expr) => {
        $crate::stores!(@columns $b, _nothing, Vec::new)
    };
}

/// Run a built scheduler for `$frames` frames through `$entry` and write
/// every frame down. `$which` is `with_cross` or `without_cross`, as the
/// scheduler was built. A macro, since the scheduler's type differs with its
/// registration and with the entry's receiver.
#[macro_export]
macro_rules! drive {
    ($which:ident, $scheduler:ident, $entry:expr, $frames:expr, $poison:expr, $sorted:expr) => {{
        use notko::Outcome;
        use $crate::common::Entry;
        use $crate::common::workers::FixedWorkers;

        let mut out = Vec::new();
        match $entry {
            entry @ (Entry::Run | Entry::RunMarked) => {
                let mut scheduler = $scheduler;
                {
                    let stores = $crate::stores!($which scheduler.__bindings());
                    // SAFETY: the reservations of a built scheduler, before
                    // the first frame.
                    unsafe { stores.fill($poison) };
                }
                for frame in 1 ..= $frames {
                    if matches!(entry, Entry::RunMarked) {
                        scheduler.mark_dirty::<hilavitkutin_api::store::Column<
                            hilavitkutin_frozen_authors::Pos,
                        >, _>();
                    }
                    assert!(matches!(scheduler.run(), Outcome::Ok(())));
                    out.push({
                        let stores = $crate::stores!($which scheduler.__bindings());
                        // SAFETY: the frame is over.
                        unsafe { stores.words(frame, $sorted) }
                    });
                }
            },
            Entry::Parallel(workers) => {
                let mut scheduler = core::pin::pin!($scheduler);
                {
                    let stores = $crate::stores!($which scheduler.as_ref().get_ref().__bindings());
                    // SAFETY: the reservations of a built scheduler, before
                    // the first frame.
                    unsafe { stores.fill($poison) };
                }
                for frame in 1 ..= $frames {
                    let outcome = match workers {
                        1 => scheduler.as_mut().run_parallel(&FixedWorkers::<1>),
                        2 => scheduler.as_mut().run_parallel(&FixedWorkers::<2>),
                        4 => scheduler.as_mut().run_parallel(&FixedWorkers::<4>),
                        other => panic!("no pool of {other} workers in the harness"),
                    };
                    assert!(matches!(outcome, Outcome::Ok(())));
                    out.push({
                        let stores =
                            $crate::stores!($which scheduler.as_ref().get_ref().__bindings());
                        // SAFETY: the frame is over and every worker is parked.
                        unsafe { stores.words(frame, $sorted) }
                    });
                }
            },
        }
        out
    }};
}
