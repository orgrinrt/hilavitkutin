//! `audio_mix`: mixes three input sample columns into one output column.
//!
//! The lead, bass and drums columns hold fixed-point samples. `Mix` is a
//! value function over one record of each, scaled by the gain a `Resource`
//! holds, and it writes the master column. The records span several morsels
//! (a four-byte write column gets a window of 6144 records), so `Mix` runs
//! once per window. `Meter` runs after it, reduces the master column to its
//! highest absolute sample and appends that to an accumulator. A unit that
//! writes an accumulator is dispatched once over the whole range, so the
//! accumulator holds one peak per frame.
//!
//! The scheduler is built over the `hilavitkutin` crate's `mmap` memory
//! provider. The example runs three frames, reads the master column and the
//! peak back, and checks them against values worked out by hand.
//!
//! Run it with `cargo run -p hilavitkutin-providers --example audio_mix`.

use arvo::{Hot, Signed, USize};
use hilavitkutin::OsMemoryProvider;
use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    AccumWriterApi,
    ColumnReaderApi,
    EachApi,
    HasAccumWriter,
    HasColumnReader,
    HasEach,
    HasReduce,
    ReduceApi,
};
use hilavitkutin_api::footprint::ResourceFootprint;
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Accum, Column, Resource, StartsFromDefault};
use hilavitkutin_api::work_unit::WorkUnit;
use hilavitkutin_api::{ResourceRead, rec};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

/// A sample: signed, 15 integer bits and 16 fractional bits.
type Sample = Signed<15, 16, Hot>;

/// How many records each column holds. Over three windows of a four-byte column.
const RECORDS: usize = 20_000;
/// How many frames the example runs.
const FRAMES: usize = 3;
/// The fractional bits of a `Sample`, as a shift of the raw container.
const FRAC: u32 = 16;

#[derive(Copy, Clone, PartialEq, Default)]
struct Lead(Sample);

#[derive(Copy, Clone, PartialEq, Default)]
struct Bass(Sample);

#[derive(Copy, Clone, PartialEq, Default)]
struct Drums(Sample);

#[derive(Copy, Clone, PartialEq, Default)]
struct Master(Sample);

/// The highest absolute sample of the master column in one frame.
#[derive(Copy, Clone, PartialEq, Default)]
struct Peak(Sample);

/// What each input is scaled by before the three are summed.
#[derive(Copy, Clone)]
struct Gains {
    lead:  Sample,
    bass:  Sample,
    drums: Sample,
}

impl ResourceRead for Gains {
    type Val<'a> = Self;

    fn read<'a>(&'a self) -> Self {
        *self
    }
}

// Three scalars, read and never written by a unit.
impl ResourceFootprint for Gains {
    const L1_BYTES: USize = USize(0);
}

type Inputs =
    Cons<Column<Lead>, Cons<Column<Bass>, Cons<Column<Drums>, Cons<Resource<Gains>, Empty>>>>;
type MasterOut = Cons<Column<Master>, Empty>;

/// The master record: each input times its gain, summed.
#[derive(Copy, Clone)]
struct Mix;

impl BuilderInput for Mix {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Mix {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Inputs;
    type Write = MasterOut;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| {
            let gains: Gains = read.get(Resource::<Gains>::default());
            let lead = read.get(Column::<Lead>::new()).0;
            let bass = read.get(Column::<Bass>::new()).0;
            let drums = read.get(Column::<Drums>::new()).0;
            rec![Master(lead * gains.lead + bass * gains.bass + drums * gains.drums)]
        });
    }
}

/// Appends the peak of the master records.
#[derive(Copy, Clone)]
struct Meter;

impl BuilderInput for Meter {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Meter {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Master>, Empty>;
    type Write = Cons<Accum<Peak>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let peak = ReduceApi::run(ctx.reduce(), 0_i32, |highest, i| {
            // SAFETY: `i` is a record of the morsel, the `Mix` unit wrote it
            // earlier in the frame, and the plan gives no unit a write to it
            // while this one runs.
            let master = unsafe { ctx.reader().read::<Master, _>(i) };
            highest.max(master.0.to_raw().abs())
        });
        ctx.accums().append::<Peak, _>(Peak(Sample::from_raw(peak)));
    }
}

/// The column storage over the `mmap` provider. The return type leaves the
/// column cap at its default, which anchors it at the `build` call.
fn store() -> ArenaColumnStorage<OsMemoryProvider> {
    ArenaColumnStorage::new(OsMemoryProvider::new())
}

/// The lead input at record `i`, in whole units.
fn lead_at(i: usize) -> i32 {
    (i % 7) as i32 - 3
}

/// The bass input at record `i`, in whole units.
fn bass_at(i: usize) -> i32 {
    (i % 5) as i32 * 2 - 4
}

/// The drums input at record `i`, in whole units.
fn drums_at(i: usize) -> i32 {
    if i % 4 == 0 { 8 } else { 0 }
}

/// The master record the inputs above come to under gains of one half for the
/// lead and one quarter for the bass and the drums, as the raw container.
fn master_at(i: usize) -> i32 {
    (lead_at(i) << (FRAC - 1)) + (bass_at(i) << (FRAC - 2)) + (drums_at(i) << (FRAC - 2))
}

fn main() {
    let half = Sample::from_raw(1 << (FRAC - 1));
    let quarter = Sample::from_raw(1 << (FRAC - 2));

    let mut scheduler = Scheduler::builder()
        .with(Column::<Lead>::start())
        .with(Column::<Bass>::start())
        .with(Column::<Drums>::start())
        .with(Column::<Master>::start())
        .with(Accum::<Peak>::new())
        .with(Resource::new(Gains {
            lead:  half,
            bass:  quarter,
            drums: quarter,
        }))
        .with(Mix)
        .with(Meter)
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|_| panic!("the mixer builds"));

    // The last store registered leads the bindings.
    let peaks = scheduler.__bindings().__tail();
    let master = peaks.__tail();
    let drums = master.__tail();
    let bass = drums.__tail();
    let lead = bass.__tail();
    let master_base = master.__ptr().as_ptr();
    let (lead_base, bass_base, drums_base) = (
        lead.__ptr().as_ptr(),
        bass.__ptr().as_ptr(),
        drums.__ptr().as_ptr(),
    );
    for i in 0 .. RECORDS {
        // SAFETY: each column holds `RECORDS` records and no frame is running.
        unsafe {
            lead_base
                .add(i)
                .write(Lead(Sample::from_raw(lead_at(i) << FRAC)));
            bass_base
                .add(i)
                .write(Bass(Sample::from_raw(bass_at(i) << FRAC)));
            drums_base
                .add(i)
                .write(Drums(Sample::from_raw(drums_at(i) << FRAC)));
        }
    }

    for _ in 0 .. FRAMES {
        // The inputs are the host's, so each frame marks them changed.
        scheduler.mark_dirty::<Column<Lead>, _>();
        scheduler.mark_dirty::<Column<Bass>, _>();
        scheduler.mark_dirty::<Column<Drums>, _>();
        assert!(matches!(scheduler.run(), Outcome::Ok(())));
    }

    let mut loudest = 0_i32;
    for i in 0 .. RECORDS {
        // SAFETY: the frames are over and the column holds `RECORDS` records.
        let got = unsafe { master_base.add(i).read() }.0.to_raw();
        assert_eq!(
            got,
            master_at(i),
            "record {i} is the gained sum of the inputs"
        );
        loudest = loudest.max(got.abs());
    }

    let peaks = scheduler.__bindings().__tail();
    let metered = peaks.__len_cell().get().0;
    let peak_base = peaks.__ptr().as_ptr();
    let mut highest = 0_i32;
    for n in 0 .. metered {
        // SAFETY: the frame appended `metered` peaks from the base on.
        highest = highest.max(unsafe { peak_base.add(n).read() }.0.to_raw());
    }
    assert_eq!(
        metered, 1,
        "the meter appends once per frame, over the whole range"
    );
    assert_eq!(highest, loudest, "the peak is the highest master sample");

    println!("audio_mix: {RECORDS} records, {FRAMES} frames, mix checked, peak {highest}");
}
