//! Six units over eight stores, a hand model of their frame, and a helper
//! that runs them under the reference runner and writes each frame down.
//!
//! The units are the smallest set that walks every store kind and every way
//! a frame can be kept wrongly: a plain column moved in place, a reach read
//! of an exchanged column, an accumulator, a virtual with an `On` unit behind
//! it, and a snapshot column only that `On` unit writes, so most frames leave
//! it unwritten.

use arvo::{Bool, Cap, Hot, Uint};
use hilavitkutin::dispatch::engine_ctx::CtxFor;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{EachApi, HasEach};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::{Accum, Buffered, Column, Exchange, Resource, Snapshot, Virtual};
use hilavitkutin_api::values::{ReadVals, ResourceRead, WriteVals};
use hilavitkutin_api::work_unit::{HasSchedule, On, WorkUnit};
use hilavitkutin_api::{Reach, RecordOp, rec};
pub use hilavitkutin_conformance::Refusal;
use hilavitkutin_conformance::Word;
use notko::Maybe;

pub type Num = Uint<32, Hot>;

pub const RECORDS: usize = 6;
pub const ONE: Cap = arvo_tensor::cap(1);
/// The value of `A` at which `Shout` fires.
pub const TRIGGER: u32 = 15;
/// How many appends a frame of the lent accumulator takes.
pub const ROOM: usize = 16;

pub const fn num(raw: u32) -> Num {
    Num::from_raw(raw)
}

#[derive(Copy, Clone, PartialEq)]
pub struct A(pub Num);
#[derive(Copy, Clone, PartialEq)]
pub struct Step(pub Num);
#[derive(Copy, Clone, PartialEq)]
pub struct B(pub Num);
#[derive(Copy, Clone, PartialEq)]
pub struct C(pub Num);
#[derive(Copy, Clone, PartialEq)]
pub struct D(pub Num);
#[derive(Copy, Clone, PartialEq)]
pub struct Seen(pub Num);
#[derive(Copy, Clone)]
pub struct Gain(pub Num);
pub struct Loud;

impl ResourceRead for Gain {
    type Val<'a> = Self;

    fn read<'a>(&'a self) -> Self {
        *self
    }
}

pub type Bx = Buffered<B, Exchange>;
pub type Cs = Buffered<C, Snapshot>;

macro_rules! always_unit {
    ($(#[$doc:meta])* $name:ident reads $read:ty, writes $write:ty, $body:expr) => {
        $(#[$doc])*
        #[derive(Copy, Clone)]
        pub struct $name;

        impl BuilderInput for $name {
            type Dispatch = UnitDispatch<Self>;
            type Init = Self;
        }

        impl WorkUnit for $name {
            type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write>;
            type Hint = (Immediate, Atomic, Normal);
            type Read = $read;
            type Write = $write;

            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                ctx.each().map(|r| self.apply(r));
            }
        }

        impl RecordOp for $name {
            fn apply(&self, read: &ReadVals<'_, Self::Read>) -> WriteVals<Self::Write> {
                let body: fn(&ReadVals<'_, $read>) -> WriteVals<$write> = $body;
                body(read)
            }
        }
    };
}

always_unit! {
    /// The value plus its step plus the gain, in place.
    Walk reads Cons<Column<A>, Cons<Column<Step>, Cons<Resource<Gain>, Empty>>>,
    writes Cons<Column<A>, Empty>,
    |read| {
        let a = read.get(Column::<A>::new()).0;
        let step = read.get(Column::<Step>::new()).0;
        let gain = read.get(Resource::<Gain>::default()).0;
        rec![A(a + step + gain)]
    }
}

fn part(side: Maybe<B>) -> Num {
    match side {
        Maybe::Is(b) => b.0,
        Maybe::Isnt => num(0),
    }
}

always_unit! {
    /// A record's value plus its two neighbours', from the frozen side.
    Blur reads Cons<Reach<Bx, ONE>, Empty>,
    writes Cons<Bx, Empty>,
    |read| {
        let around = read.get(Reach::<Bx, ONE>::new());
        rec![B(around.here().0 + part(around.before::<ONE>()) + part(around.after::<ONE>()))]
    }
}

always_unit! {
    /// An append for each odd `A`, as `Walk` left it in this frame.
    Note reads Cons<Column<A>, Empty>,
    writes Cons<Accum<Seen>, Empty>,
    |read| {
        let a = read.get(Column::<A>::new()).0;
        if a.to_raw() % 2 == 1 { rec![Maybe::Is(Seen(a))] } else { rec![Maybe::Isnt] }
    }
}

always_unit! {
    /// Fires where an `A` stands on the trigger.
    Shout reads Cons<Column<A>, Empty>,
    writes Cons<Virtual<Loud>, Empty>,
    |read| rec![Bool(read.get(Column::<A>::new()).0.to_raw() == TRIGGER)]
}

always_unit! {
    /// Twice the frozen `C`.
    Echo reads Cons<Cs, Empty>,
    writes Cons<Column<D>, Empty>,
    |read| {
        let c = read.get(Cs::new()).0;
        rec![D(c + c)]
    }
}

/// Runs in the frames `Loud` fired in: the frozen `C` plus one.
#[derive(Copy, Clone)]
pub struct Bump;

impl BuilderInput for Bump {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl HasSchedule for Bump {
    type Sched = On<Loud>;
}

impl WorkUnit<On<Loud>> for Bump {
    type Ctx<'frame> = CtxFor<'frame, Self::Read, Self::Write, On<Loud>>;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Cons<Cs, Empty>;
    type Write = Cons<Cs, Empty>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().map(|r| self.apply(r));
    }
}

impl RecordOp<On<Loud>> for Bump {
    fn apply(
        &self,
        read: &ReadVals<'_, <Self as WorkUnit<On<Loud>>>::Read>,
    ) -> WriteVals<<Self as WorkUnit<On<Loud>>>::Write> {
        rec![C(read.get(Cs::new()).0 + num(1))]
    }
}

/// The units in the sequence the model keeps them in.
pub type Listed = (Walk, (Blur, (Note, (Shout, (Bump, (Echo, ()))))));

pub const LISTED: Listed = (Walk, (Blur, (Note, (Shout, (Bump, (Echo, ()))))));

/// The frame of the six units, spelled out over plain arrays.
#[derive(Clone, PartialEq, Debug)]
pub struct Model {
    pub a:    [u32; RECORDS],
    pub step: [u32; RECORDS],
    pub b:    [u32; RECORDS],
    pub c:    [u32; RECORDS],
    pub d:    [u32; RECORDS],
    pub gain: u32,
    pub seen: Vec<u32>,
    pub loud: bool,
}

impl Model {
    /// The state every run starts from. `Loud` fires in the second frame and
    /// in no other of the first six.
    pub fn new() -> Self {
        Model {
            a:    [1, 4, 7, 10, 13, 16],
            step: [1, 2, 3, 1, 2, 3],
            b:    [1, 2, 3, 4, 5, 6],
            c:    [10, 20, 30, 40, 50, 60],
            d:    [0; RECORDS],
            gain: 1,
            seen: Vec::new(),
            loud: false,
        }
    }

    pub fn frame(&mut self) {
        self.loud = false;
        self.seen.clear();
        for i in 0 .. RECORDS {
            self.a[i] += self.step[i] + self.gain;
        }
        let mut b = self.b;
        for i in 0 .. RECORDS {
            let before = if i > 0 { self.b[i - 1] } else { 0 };
            let after = if i + 1 < RECORDS { self.b[i + 1] } else { 0 };
            b[i] = self.b[i] + before + after;
        }
        for i in 0 .. RECORDS {
            if self.a[i] % 2 == 1 {
                self.seen.push(self.a[i]);
            }
        }
        self.loud = self.a.iter().any(|a| *a == TRIGGER);
        let mut c = self.c;
        if self.loud {
            for i in 0 .. RECORDS {
                c[i] = self.c[i] + 1;
            }
        }
        for i in 0 .. RECORDS {
            self.d[i] = self.c[i] + self.c[i];
        }
        // The boundary.
        self.b = b;
        self.c = c;
    }

    /// The frame's state as words. `sorted` writes the appends sorted, for a
    /// comparison across visitings.
    pub fn words(&self, sorted: bool) -> Vec<Word> {
        words(
            &self.a, &self.b, &self.c, &self.d, self.loud, &self.seen, sorted,
        )
    }
}

fn word(raw: u32) -> Word {
    Word::from_raw(u64::from(raw))
}

/// One frame's stores in a fixed sequence.
pub fn words(
    a: &[u32],
    b: &[u32],
    c: &[u32],
    d: &[u32],
    loud: bool,
    seen: &[u32],
    sorted: bool,
) -> Vec<Word> {
    let mut out: Vec<Word> = Vec::new();
    for column in [a, b, c, d] {
        out.extend(column.iter().map(|v| word(*v)));
    }
    out.push(word(u32::from(loud)));
    out.push(word(seen.len() as u32));
    let mut seen = seen.to_vec();
    if sorted {
        seen.sort_unstable();
    }
    out.extend(seen.into_iter().map(word));
    out
}

/// The frozen sides of the two buffered columns, as words.
pub fn sides(b: &[u32], c: &[u32]) -> Vec<Word> {
    b.iter().chain(c.iter()).map(|v| word(*v)).collect()
}

/// What a run under the reference runner wrote down, frame by frame.
#[derive(Default)]
pub struct Recorded {
    /// The whole state after each frame's boundary.
    pub frames:    Vec<Vec<Word>>,
    /// The frozen sides ahead of each frame's first unit.
    pub began:     Vec<Vec<Word>>,
    /// The frozen sides between each frame's last unit and its boundary.
    pub before:    Vec<Vec<Word>>,
    /// The frozen sides after each frame's boundary.
    pub committed: Vec<Vec<Word>>,
}

/// How a run is set up apart from how it is kept and visited.
#[derive(Copy, Clone)]
pub struct Setup {
    /// How many frames to run.
    pub frames: usize,
    /// What the live sides of the buffered columns hold at the start.
    pub poison: u32,
    /// Whether the appends are written down sorted.
    pub sorted: bool,
    /// How many records each lent column holds.
    pub lent:   usize,
    /// How many appends the lent accumulator takes.
    pub room:   usize,
}

impl Setup {
    pub const fn frames(frames: usize) -> Self {
        Setup {
            frames,
            poison: 0,
            sorted: false,
            lent: RECORDS,
            room: ROOM,
        }
    }
}

/// Run `units` from the model's starting state, kept as `K` and visited as
/// `V`, and write every frame down. `$units` is a value of any unit list over
/// the eight stores. A macro, so that the world's type, a nested pair eight
/// deep, is never written out.
#[macro_export]
macro_rules! record_run {
    ($keeping:ty, $visiting:ty, $setup:expr, $units:expr) => {{
        // The caller's two expressions are taken ahead of the imports below,
        // so the names in them are the caller's.
        let setup: $crate::common::Setup = $setup;
        let units = $units;
        $crate::record_run!(@run $keeping, $visiting, setup, units)
    }};
    (@run $keeping:ty, $visiting:ty, $setup:ident, $units:ident) => {{
        use arvo::USize;
        use hilavitkutin_api::store::{Accum, Column, Resource, Virtual};
        use hilavitkutin_conformance::{Lent, Oracle};
        use notko::Outcome;
        use $crate::common::*;

        let setup: Setup = $setup;
        let units = $units;
        let start = Model::new();
        let lend = |from: [u32; RECORDS]| -> Vec<Num> {
            from.iter().take(setup.lent).map(|v| num(*v)).collect()
        };
        let mut a: Vec<A> = lend(start.a).into_iter().map(A).collect();
        let mut step: Vec<Step> = lend(start.step).into_iter().map(Step).collect();
        let mut b: Vec<B> = lend(start.b).into_iter().map(B).collect();
        let mut b_live = vec![B(num(setup.poison)); setup.lent];
        let mut c: Vec<C> = lend(start.c).into_iter().map(C).collect();
        let mut c_live = vec![C(num(setup.poison)); setup.lent];
        let mut d = vec![D(num(0)); setup.lent];
        let mut seen = vec![Seen(num(0)); setup.room];
        let gain = Gain(num(start.gain));
        let world = (
            Lent::<Column<A>>::over(&mut a),
            (
                Lent::<Column<Step>>::over(&mut step),
                (
                    Lent::<Bx>::over(&mut b, &mut b_live),
                    (
                        Lent::<Cs>::over(&mut c, &mut c_live),
                        (
                            Lent::<Column<D>>::over(&mut d),
                            (
                                Lent::<Accum<Seen>>::over(&mut seen),
                                (
                                    Lent::<Virtual<Loud>>::unfired(),
                                    (Lent::<Resource<Gain>>::of(&gain), ()),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );
        let raw = |values: &[Num]| -> Vec<u32> { values.iter().map(|v| v.to_raw()).collect() };
        (|| -> Result<Recorded, Refusal> {
            let mut oracle =
                match Oracle::<_, $keeping, $visiting>::new(world, USize(RECORDS)) {
                    Outcome::Ok(oracle) => oracle,
                    Outcome::Err(refusal) => return Err(refusal),
                };
            let mut recorded = Recorded::default();
            macro_rules! frozen {
                () => {{
                    let b: Vec<Num> =
                        oracle.of::<Bx, _>().frozen().iter().map(|v| v.0).collect();
                    let c: Vec<Num> =
                        oracle.of::<Cs, _>().frozen().iter().map(|v| v.0).collect();
                    (raw(&b), raw(&c))
                }};
            }
            for _ in 0 .. setup.frames {
                let (b, c) = frozen!();
                recorded.began.push(sides(&b, &c));
                if let Outcome::Err(refusal) = oracle.units(&units) {
                    return Err(refusal);
                }
                let (b, c) = frozen!();
                recorded.before.push(sides(&b, &c));
                oracle.boundary();
                let (b, c) = frozen!();
                recorded.committed.push(sides(&b, &c));
                let a: Vec<Num> =
                    oracle.of::<Column<A>, _>().values().iter().map(|v| v.0).collect();
                let d: Vec<Num> =
                    oracle.of::<Column<D>, _>().values().iter().map(|v| v.0).collect();
                let seen: Vec<Num> = oracle
                    .of::<Accum<Seen>, _>()
                    .appended()
                    .iter()
                    .map(|v| v.0)
                    .collect();
                let loud = oracle.of::<Virtual<Loud>, _>().fired().0;
                recorded.frames.push(words(
                    &raw(&a),
                    &b,
                    &c,
                    &raw(&d),
                    loud,
                    &raw(&seen),
                    setup.sorted,
                ));
            }
            Ok(recorded)
        })()
    }};
}

/// The model's words, frame by frame, from the starting state.
pub fn model_frames(frames: usize, sorted: bool) -> Vec<Vec<Word>> {
    let mut model = Model::new();
    (0 .. frames)
        .map(|_| {
            model.frame();
            model.words(sorted)
        })
        .collect()
}

/// Every frame's words, one after the other.
pub fn flat(frames: &[Vec<Word>]) -> Vec<Word> {
    frames.iter().flatten().copied().collect()
}
