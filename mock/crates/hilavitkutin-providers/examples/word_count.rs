//! `word_count`: maps a column of interned words to kinds, then reduces the
//! kinds to counts.
//!
//! The words are interned host-side through the providers crate's
//! `default_interner`, and the corpus column holds the resulting `Str`
//! handles. `Classify` is the map: it compares each word with the handles a
//! `Resource` holds and writes a kind per record. `Tally` is the reduce: it
//! folds the kind column into one count per kind and appends the counts to an
//! accumulator. The corpus spans more than one window of the kind column, so
//! `Classify` runs once per window.
//!
//! The scheduler is built over the `hilavitkutin` crate's `mmap` memory
//! provider. The example runs two frames, reads the counts back, and checks
//! them against values worked out by hand.
//!
//! Run it with `cargo run -p hilavitkutin-providers --example word_count`.

use arvo::USize;
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
use hilavitkutin_providers::{ArenaColumnStorage, default_interner};
use hilavitkutin_str::Str;
use notko::{Maybe, Outcome};

/// How many words the corpus holds. Two windows of the kind column.
const RECORDS: usize = 12_000;
/// How many frames the example runs.
const FRAMES: usize = 2;

/// The vocabulary, in the order its handles are kept.
const VOCABULARY: [&str; 4] = ["alpha", "beta", "gamma", "delta"];
/// Which vocabulary word each record of a twelve-record cycle holds:
/// six of the first, three of the second, two of the third, one of the last.
const CYCLE: [usize; 12] = [0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 3];

/// A corpus word, as its interned handle.
#[derive(Copy, Clone, PartialEq, Default)]
struct Word(Str);

/// What `Classify` makes of a word: 1, 2 or 3 for the words the lexicon
/// names, 0 for any other.
#[derive(Copy, Clone, PartialEq)]
struct Kind(USize);

impl Default for Kind {
    fn default() -> Self {
        Self(USize(0))
    }
}

/// How many records of each kind a frame holds.
#[derive(Copy, Clone, PartialEq)]
struct Counts {
    other:  USize,
    first:  USize,
    second: USize,
    third:  USize,
}

impl Default for Counts {
    fn default() -> Self {
        Self {
            other:  USize(0),
            first:  USize(0),
            second: USize(0),
            third:  USize(0),
        }
    }
}

/// The three handles a word is compared with.
#[derive(Copy, Clone)]
struct Lexicon {
    first:  Str,
    second: Str,
    third:  Str,
}

impl ResourceRead for Lexicon {
    type Val<'a> = Self;

    fn read<'a>(&'a self) -> Self {
        *self
    }
}

// Three handles, read and never written by a unit.
impl ResourceFootprint for Lexicon {
    const L1_BYTES: USize = USize(0);
}

/// The kind of each word.
#[derive(Copy, Clone)]
struct Classify;

impl BuilderInput for Classify {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Classify {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Word>, Cons<Resource<Lexicon>, Empty>>;
    type Write = Cons<Column<Kind>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|read| {
            let lexicon: Lexicon = read.get(Resource::<Lexicon>::default());
            let word = read.get(Column::<Word>::new()).0;
            let kind = if word == lexicon.first {
                1
            } else if word == lexicon.second {
                2
            } else if word == lexicon.third {
                3
            } else {
                0
            };
            rec![Kind(USize(kind))]
        });
    }
}

/// Folds the kinds to counts and appends them.
#[derive(Copy, Clone)]
struct Tally;

impl BuilderInput for Tally {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl WorkUnit for Tally {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Column<Kind>, Empty>;
    type Write = Cons<Accum<Counts>, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        let counts = ReduceApi::run(ctx.reduce(), Counts::default(), |mut counts, i| {
            // SAFETY: `i` is a record of the morsel, `Classify` wrote it
            // earlier in the frame, and the plan gives no unit a write to it
            // while this one runs.
            let kind = unsafe { ctx.reader().read::<Kind, _>(i) };
            match kind.0.0 {
                1 => counts.first = USize(counts.first.0 + 1),
                2 => counts.second = USize(counts.second.0 + 1),
                3 => counts.third = USize(counts.third.0 + 1),
                _ => counts.other = USize(counts.other.0 + 1),
            }
            counts
        });
        ctx.accums().append::<Counts, _>(counts);
    }
}

/// The column storage over the `mmap` provider. The return type leaves the
/// column cap at its default, which anchors it at the `build` call.
fn store() -> ArenaColumnStorage<OsMemoryProvider> {
    ArenaColumnStorage::new(OsMemoryProvider::new())
}

fn main() {
    // Each distinct word is interned once, and the corpus repeats the handles.
    let interner = default_interner::<256, 16>();
    let handles = [
        interner.intern(VOCABULARY[0]),
        interner.intern(VOCABULARY[1]),
        interner.intern(VOCABULARY[2]),
        interner.intern(VOCABULARY[3]),
    ];
    assert!(
        matches!(interner.resolve(handles[2]), Maybe::Is("gamma")),
        "a handle resolves back to its word"
    );

    let mut scheduler = Scheduler::builder()
        .with(Column::<Word>::start())
        .with(Column::<Kind>::start())
        .with(Accum::<Counts>::new())
        .with(Resource::new(Lexicon {
            first:  handles[0],
            second: handles[1],
            third:  handles[2],
        }))
        .with(Classify)
        .with(Tally)
        .build(store(), USize(RECORDS))
        .unwrap_or_else(|_| panic!("the word counter builds"));

    // The last store registered leads the bindings.
    let counts = scheduler.__bindings().__tail();
    let words = counts.__tail().__tail();
    let word_base = words.__ptr().as_ptr();
    for i in 0 .. RECORDS {
        // SAFETY: the column holds `RECORDS` records and no frame is running.
        unsafe {
            word_base
                .add(i)
                .write(Word(handles[CYCLE[i % CYCLE.len()]]))
        };
    }

    for _ in 0 .. FRAMES {
        // The corpus is the host's, so each frame marks it changed.
        scheduler.mark_dirty::<Column<Word>, _>();
        assert!(matches!(scheduler.run(), Outcome::Ok(())));
    }

    // Each of the 1000 twelve-record cycles holds six, three, two and one.
    let counts = scheduler.__bindings().__tail();
    assert_eq!(
        counts.__len_cell().get().0,
        1,
        "one set of counts per frame"
    );
    // SAFETY: the frame appended one `Counts` at the base.
    let total = unsafe { counts.__ptr().as_ptr().read() };
    assert_eq!(total.first.0, 6_000, "alpha");
    assert_eq!(total.second.0, 3_000, "beta");
    assert_eq!(total.third.0, 2_000, "gamma");
    assert_eq!(total.other.0, 1_000, "delta");

    println!(
        "word_count: {RECORDS} words, {FRAMES} frames, alpha {} beta {} gamma {} other {}",
        total.first.0, total.second.0, total.third.0, total.other.0
    );
}
