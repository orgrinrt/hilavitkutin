//! The `frame_wide` workload, defined once for every arm that runs it.
//!
//! One simulation frame over a wide DAG: three independent three-stage chains
//! off a shared input column, a fan-in stage joining their tails, and two
//! independent single-stage transforms off the same input. Twelve work units
//! over thirteen columns, so the engine's plan has real phases, trunks and
//! fibers to find rather than one linear chain.
//!
//! Every arm computes the same three output columns from the same input, and
//! the digest over those three columns is what the harness compares byte-exact
//! across arms before any timing counts. The stage kernels, the join, the
//! column identities and the digest live here and nowhere else: an arm that
//! disagreed with another would have to disagree with this file first.
//!
//! The arms measure an engine without its headline features. Fusing and the
//! adaptive trunk, fiber and morsel work are not built, so a number from this
//! bench is a baseline for a later comparison and decides nothing about the
//! engine.
//!
//! std and alloc are allowed here. This is a bench crate outside the no_std,
//! no_alloc `mock/crates` members, as `engine_vs_std` and `resource_storage`
//! both are.

use core::cell::RefCell;
use core::mem::ManuallyDrop;

pub mod engine;
pub mod plain;

/// How much per-record work each stage does.
///
/// The answer to what this frame costs differs by regime and one number for
/// both would be a lie. `Heavy` gives each record real compute, so per-record
/// work dominates dispatch. `Light` makes each stage a single wrapping
/// multiply, so the frame is memory-bound and the cost of materialising the
/// intermediate columns is the whole story.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Regime {
    /// Real per-record compute: the four-step mix iterated `HEAVY_ROUNDS` times.
    Heavy,
    /// One wrapping multiply per stage.
    Light,
}

/// The regime tag carried in the first word of the input image.
pub const TAG_HEAVY: u32 = 0x4845_4156;
/// The regime tag carried in the first word of the input image.
pub const TAG_LIGHT: u32 = 0x4C49_4748;

impl Regime {
    /// The tag this regime writes into the input image.
    pub const fn tag(self) -> u32 {
        match self {
            Regime::Heavy => TAG_HEAVY,
            Regime::Light => TAG_LIGHT,
        }
    }

    /// The regime a tag names. Panics on a tag no regime writes, which can
    /// only mean the input image and this crate disagree about the layout.
    pub fn from_tag(tag: u32) -> Regime {
        match tag {
            TAG_HEAVY => Regime::Heavy,
            TAG_LIGHT => Regime::Light,
            other => {
                panic!("frame_wide: input carries regime tag {other:#x}, which no regime writes")
            },
        }
    }
}

/// A per-record stage kernel, selected at compile time.
///
/// The regime cannot be a runtime branch inside a work unit: a `WorkUnit`'s
/// `execute` sees only its context, so the kernel has to be part of the unit's
/// type identity. It is a trait rather than a `const HEAVY: bool` because the
/// scheduler type then carries one type parameter instead of threading a
/// boolean through every generic position.
pub trait Kernel: Copy + 'static {
    /// Transform one record under one stage constant.
    fn apply(x: u32, k: u32) -> u32;
}

/// The heavy regime's kernel.
#[derive(Copy, Clone, Debug)]
pub struct Heavy;
/// The light regime's kernel.
#[derive(Copy, Clone, Debug)]
pub struct Light;

/// Knuth multiplicative constants for the heavy mix, as `engine_vs_std` uses
/// them, so a heavy record here costs what a heavy record costs there.
const M1: u32 = 2654435761;
const M2: u32 = 2246822519;
const SH: u32 = 13;
const M4: u32 = 3266489917;

/// How many times the heavy kernel iterates the four-step mix.
pub const HEAVY_ROUNDS: usize = 8;

#[inline(always)]
fn mix(x: u32) -> u32 {
    let a = x.wrapping_mul(M1);
    let b = a.wrapping_mul(M2).wrapping_add(1);
    let c = (b >> SH) ^ b;
    c.wrapping_mul(M4)
}

impl Kernel for Heavy {
    #[inline(always)]
    fn apply(x: u32, k: u32) -> u32 {
        let mut v = x ^ k;
        let mut r = 0;
        while r < HEAVY_ROUNDS {
            v = mix(v);
            r += 1;
        }
        v
    }
}

impl Kernel for Light {
    #[inline(always)]
    fn apply(x: u32, k: u32) -> u32 {
        x.wrapping_mul(k | 1)
    }
}

/// The twelve stage constants, one per work unit, so no two stages compute the
/// same function of their input.
pub const K_A1: u32 = 0x9E37_79B1;
pub const K_A2: u32 = 0x85EB_CA6B;
pub const K_A3: u32 = 0xC2B2_AE35;
pub const K_B1: u32 = 0x27D4_EB2F;
pub const K_B2: u32 = 0x1656_67B1;
pub const K_B3: u32 = 0xD3A2_646D;
pub const K_C1: u32 = 0xFF51_AFD7;
pub const K_C2: u32 = 0x4F27_4143;
pub const K_C3: u32 = 0x2545_F491;
pub const K_J: u32 = 0x7FEB_352D;
pub const K_T1: u32 = 0x846C_A68B;
pub const K_T2: u32 = 0xBF58_476D;

/// The fan-in: one record of the join column from the three chain tails.
#[inline(always)]
pub fn join<KN: Kernel>(a: u32, b: u32, c: u32) -> u32 {
    KN::apply(a ^ b.rotate_left(7) ^ c.rotate_left(19), K_J)
}

/// The whole frame for one record, fused: the three chains, the join and the
/// two transforms, returning `(join, t1, t2)`.
///
/// This is the reference the hand-written arms run and the thing every other
/// arm has to agree with. Nothing else states what the frame computes.
#[inline(always)]
pub fn record<KN: Kernel>(v: u32) -> (u32, u32, u32) {
    frame_record::<KN, false>(v)
}

/// The negative control: `record` with the second stage of the first chain
/// dropped, so `a3` is computed from `a1` directly.
///
/// It exists to be rejected. The cross-arm validation is only worth something
/// once it has been seen to refuse an arm that computes the wrong frame, and
/// this is the smallest wrong frame: one stage of twelve missing, every other
/// stage and every output column in place.
#[inline(always)]
pub fn record_without_a2<KN: Kernel>(v: u32) -> (u32, u32, u32) {
    frame_record::<KN, true>(v)
}

/// The one statement of the per-record frame, with the negative control's
/// dropped stage as a compile-time switch so the two cannot drift apart.
#[inline(always)]
fn frame_record<KN: Kernel, const DROP_A2: bool>(v: u32) -> (u32, u32, u32) {
    let a1 = KN::apply(v, K_A1);
    let a2 = if DROP_A2 { a1 } else { KN::apply(a1, K_A2) };
    let a3 = KN::apply(a2, K_A3);
    let b3 = KN::apply(KN::apply(KN::apply(v, K_B1), K_B2), K_B3);
    let c3 = KN::apply(KN::apply(KN::apply(v, K_C1), K_C2), K_C3);
    (
        join::<KN>(a3, b3, c3),
        KN::apply(v, K_T1),
        KN::apply(v, K_T2),
    )
}

/// The digest the frame must produce for these input records, computed from
/// `record` alone, one record at a time, with no column materialised.
///
/// This is what `validate_output` checks every arm against, so an arm is held
/// to the reference on its own and not only to its neighbours: a defect every
/// arm shared would pass the cross-arm comparison and still fail here.
pub fn reference_digest<KN: Kernel>(records: &[u32]) -> FrameOut {
    let (mut j, mut t1, mut t2) = (Fnv1a::new(), Fnv1a::new(), Fnv1a::new());
    for &v in records {
        let (rj, r1, r2) = record::<KN>(v);
        j.push(rj);
        t1.push(r1);
        t2.push(r2);
    }
    [j.finish(), t1.finish(), t2.finish()]
}

/// How many columns the frame reserves: the input, nine chain columns, the
/// join column and the two transform columns.
pub const COLUMNS: usize = 13;

/// How many per-record stage applications one frame performs, counting the
/// join as one. Feeds the harness throughput table.
pub const STAGES_PER_RECORD: u64 = 12;

// ----- the input image -----

/// The input one call receives: a regime tag and the record column.
///
/// `Routine::Input` has to be `Copy` and flat, and it is the only channel the
/// harness gives a variant, so the regime travels in it. The driver resolves a
/// different routine per bench section and that routine writes the tag; a
/// variant reads it and picks its kernel. Nothing else tells a variant which
/// sweep it is running under.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameInput<const N: usize> {
    /// `TAG_HEAVY` or `TAG_LIGHT`.
    pub regime:  u32,
    /// Keeps `records` eight-byte aligned inside the image.
    pub pad:     u32,
    /// One u32 per record.
    pub records: [u32; N],
}

/// The digest one call writes: the FNV-1a hash of the join column and of the
/// two transform columns.
///
/// The three chain tails are not hashed directly. A fused arm keeps them in
/// registers and never materialises them, and hashing them would force it to.
/// Every chain feeds the join, so a stage dropped anywhere inside a chain
/// moves the first word, which is what the negative control relies on.
pub type FrameOut = [u64; 3];

const FNV_OFFSET: u64 = 0xCBF2_9CE4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01B3;

/// A running FNV-1a hash over the little-endian bytes of u32 records.
#[derive(Copy, Clone)]
pub struct Fnv1a(u64);

impl Fnv1a {
    /// The empty hash.
    pub const fn new() -> Self {
        Self(FNV_OFFSET)
    }

    /// Fold one record in.
    #[inline]
    pub fn push(&mut self, v: u32) {
        for b in v.to_le_bytes() {
            self.0 = (self.0 ^ b as u64).wrapping_mul(FNV_PRIME);
        }
    }

    /// The hash so far.
    pub const fn finish(self) -> u64 {
        self.0
    }
}

/// FNV-1a over the little-endian bytes of a u32 slice.
pub fn fnv1a_u32(vals: &[u32]) -> u64 {
    let mut h = Fnv1a::new();
    for &v in vals {
        h.push(v);
    }
    h.finish()
}

// ----- the routine -----

/// The bench routine for `frame_wide`, parameterised by record count and
/// regime.
///
/// `max_call_us` is what makes this a `Routine` rather than a `ByteRoutine`:
/// with no per-call bound the harness kills a worker at 300 seconds and the
/// cell yields nothing, and a frame over millions of records is measured in
/// milliseconds rather than nanoseconds.
pub struct FrameRoutine<const N: usize, const HEAVY: bool>;

/// The heavy sweep's routine.
pub type FrameHeavy<const N: usize> = FrameRoutine<N, true>;
/// The light sweep's routine.
pub type FrameLight<const N: usize> = FrameRoutine<N, false>;

/// Nanoseconds of per-call budget per record, heavy regime.
///
/// About ten times the slowest arm measured at 65536 records on the first
/// run (52 ns a record, `hand`, 2026-11-13, four-core Xeon at 2.1GHz). The
/// preflight probe skips a variant only past ten times the bound and the
/// worker deadline is three times it times the per-pass run count, so a
/// generous value costs nothing but a later kill on a genuinely hung arm.
const HEAVY_NS_PER_RECORD: u64 = 500;
/// The same, light regime: the slowest arm was 9.2 ns a record
/// (`rust_std_par`).
const LIGHT_NS_PER_RECORD: u64 = 100;

/// The least per-call bound any cell gets, 400 milliseconds.
///
/// Forced by the harness on x86_64, not by the frame. Every timing worker
/// calibrates its counter by sleeping three times one second before its first
/// call (`counter_frequency` in `mockspace-bench-core`), and the preflight
/// probe that validation runs is killed after ten times this bound, at least
/// two seconds. Below 400 milliseconds the probe is killed during the
/// calibration and every arm is skipped as slow, whatever it costs.
// FIXME(3ec38cb5c7c8): drop the floor once the harness calibrates its counter
// without a three second sleep per worker, or gives the probe room for it.
const MAX_CALL_US_FLOOR: u64 = 400_000;

impl<const N: usize, const HEAVY: bool> FrameRoutine<N, HEAVY> {
    /// The regime this routine builds inputs for.
    pub const fn regime() -> Regime {
        if HEAVY { Regime::Heavy } else { Regime::Light }
    }
}

/// Fill one record of the input column from the seed stream.
#[inline]
fn record_from(x: &mut u64) -> u32 {
    *x ^= *x >> 30;
    *x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    (*x >> 32) as u32
}

impl<const N: usize, const HEAVY: bool> mockspace_bench_core::Routine for FrameRoutine<N, HEAVY> {
    type Input = FrameInput<N>;
    type Output = FrameOut;

    /// The by-value constructor. The harness never calls it: the override of
    /// `build_input_bytes` below is its only caller on that path, and a frame
    /// over millions of records would not fit on the stack here. It stays
    /// written, and correct, because the trait requires it and a test calling
    /// it directly at a small `N` is the usual way it gets reached.
    fn build_input(seed: u64) -> FrameInput<N> {
        let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let mut out = FrameInput {
            regime:  Self::regime().tag(),
            pad:     0,
            records: [0u32; N],
        };
        for r in out.records.iter_mut() {
            *r = record_from(&mut x);
        }
        out
    }

    /// Fill the byte image directly so a multi-megabyte input never lives on
    /// the stack. `ByteRoutine::build_input_bytes` is the precedent.
    fn build_input_bytes(seed: u64) -> Vec<u8> {
        let mut buf = vec![0u8; core::mem::size_of::<FrameInput<N>>()];
        buf[0 .. 4].copy_from_slice(&Self::regime().tag().to_le_bytes());
        let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        for chunk in buf[8 ..].chunks_mut(4) {
            chunk.copy_from_slice(&record_from(&mut x).to_le_bytes());
        }
        buf
    }

    /// Hold the arm to the reference: the digest `record` alone produces for
    /// this input. The harness rebuilds the input from the seed for this, so
    /// it is the arm's output against an independent recomputation.
    fn validate_output(input: &FrameInput<N>, output: &FrameOut) -> Result<(), &'static str> {
        if input.regime != Self::regime().tag() {
            return Err("the input carries the other sweep's regime tag");
        }
        let expected = if HEAVY {
            reference_digest::<Heavy>(&input.records)
        } else {
            reference_digest::<Light>(&input.records)
        };
        if *output == expected {
            Ok(())
        } else {
            Err("the digest differs from the reference frame computed by `record`")
        }
    }

    fn ops_per_call(_input: &FrameInput<N>) -> u64 {
        N as u64 * STAGES_PER_RECORD
    }

    fn max_call_us(n: usize) -> Option<u64> {
        let per_record = if HEAVY { HEAVY_NS_PER_RECORD } else { LIGHT_NS_PER_RECORD };
        Some(((n as u64).saturating_mul(per_record) / 1000).max(MAX_CALL_US_FLOOR))
    }
}

// ----- the arms -----

/// One arm of the bench: get the records in, run the frame, read the digest.
///
/// Loading and the digest sit outside the timed region for every arm, so what
/// is measured is the frame and nothing else. Every arm copies the records
/// into an input buffer of its own in `load`, the engine into its input
/// column and the others into a `Vec`, so at the start of every frame each
/// arm's input has just been written and the cache state it starts from is
/// the same.
pub trait FrameArm {
    /// Put the frame's input records in place. Untimed.
    fn load(&mut self, records: &[u32]);
    /// Run one frame. This is the measured region.
    fn frame(&mut self);
    /// Hash the three output columns. Untimed.
    fn digest(&mut self) -> FrameOut;
}

/// A `FrameArm` built from a state value and three closures over it.
///
/// Every arm is some state plus three operations on it, and one arm's state is
/// a `Scheduler` whose type cannot be written down: it is the output of a
/// builder chain over thirteen columns and twelve units. Holding it in a
/// generic field and erasing the whole thing behind `Box<dyn FrameArm>` is
/// what lets the engine arms and the plain arms share one shape.
pub struct StateArm<S, L, F, D> {
    state:  S,
    load:   L,
    frame:  F,
    digest: D,
}

impl<S, L, F, D> StateArm<S, L, F, D>
where
    L: FnMut(&mut S, &[u32]),
    F: FnMut(&mut S),
    D: FnMut(&mut S) -> FrameOut,
{
    /// Assemble an arm. The bounds are here rather than on the impl so the
    /// closures' parameter types are inferred at the call site.
    pub fn new(state: S, load: L, frame: F, digest: D) -> Self {
        Self {
            state,
            load,
            frame,
            digest,
        }
    }
}

impl<S, L, F, D> FrameArm for StateArm<S, L, F, D>
where
    L: FnMut(&mut S, &[u32]),
    F: FnMut(&mut S),
    D: FnMut(&mut S) -> FrameOut,
{
    fn load(&mut self, records: &[u32]) {
        (self.load)(&mut self.state, records);
    }

    fn frame(&mut self) {
        (self.frame)(&mut self.state);
    }

    fn digest(&mut self) -> FrameOut {
        (self.digest)(&mut self.state)
    }
}

/// Which arm a variant cdylib is.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ArmKind {
    /// `Scheduler::run`, single core.
    Engine,
    /// `Scheduler::run_parallel` on a `ThreadPoolApi` executor.
    EnginePar,
    /// The hand-written fused path, single threaded.
    Hand,
    /// The hand-written fused path over the same core budget.
    HandPar,
    /// The obvious loop over preallocated slices, core only.
    RustNostd,
    /// The idiomatic std shape.
    RustStd,
    /// The idiomatic std shape threaded with `std::thread::scope`.
    RustStdPar,
    /// The negative control: the hand path with one chain stage dropped.
    /// Never listed in the manifest; it exists to be rejected.
    DropStage,
}

/// Build one arm for one regime and record count.
pub fn build_arm(kind: ArmKind, regime: Regime, n: usize) -> Box<dyn FrameArm> {
    match kind {
        ArmKind::Engine => engine::build(regime, n, false),
        ArmKind::EnginePar => engine::build(regime, n, true),
        _ => plain::build(kind, regime, n),
    }
}

thread_local! {
    /// The per-process arm, built on the first call and reused.
    ///
    /// The framework sanctions this: each worker is a dedicated subprocess per
    /// variant, mode and pass, and at least one warmup call always runs before
    /// any timed batch, so a built scheduler lands in untimed territory. It is
    /// a `thread_local` rather than a `static` because a `Scheduler` is
    /// neither `Sync` nor `Unpin`, and the entry point is only ever called on
    /// the worker's own thread.
    ///
    /// `ManuallyDrop` keeps the arm alive until the process exits. A parallel
    /// engine arm has worker threads parked on its scheduler, and a
    /// thread-local destructor running at exit would free the scheduler under
    /// them. The process reclaims the memory either way.
    static ARM: ManuallyDrop<RefCell<Option<(ArmKind, Regime, usize, Box<dyn FrameArm>)>>> =
        const { ManuallyDrop::new(RefCell::new(None)) };
}

/// One call of the bench, for any arm: the whole body of every variant.
///
/// The arm is built on the first call of the process and reused. The records
/// are loaded and the digest is read outside the timed region; the `run` block
/// holds the frame and nothing else.
///
/// The key is fixed for the life of a worker process: one dylib is one
/// `ArmKind`, the worker is spawned with one `--n`, and the regime comes from
/// the bench section that spawned it. A key that changed would mean dropping a
/// scheduler whose worker threads are still parked on it, so a change is a
/// panic rather than a rebuild.
pub fn bench_call<const N: usize>(
    kind: ArmKind,
    input: &FrameInput<N>,
    output: &mut FrameOut,
) -> mockspace_bench_core::FfiBenchCall {
    let regime = Regime::from_tag(input.regime);
    ARM.with(|cell| {
        let mut slot = cell.borrow_mut();
        match slot.as_ref() {
            None => {
                *slot = Some((kind, regime, N, build_arm(kind, regime, N)));
            },
            Some((k, r, m, _)) => {
                assert!(
                    *k == kind && *r == regime && *m == N,
                    "frame_wide: this process already holds a {k:?}/{r:?}/n={m} arm and was \
                     asked for {kind:?}/{regime:?}/n={N}. One worker process runs one arm at \
                     one size under one regime."
                );
            },
        }
        let arm = &mut slot.as_mut().expect("the arm was just placed").3;
        let call = mockspace_bench_core::timed! {
            setup { arm.load(&input.records); }
            run { arm.frame(); }
        };
        *output = arm.digest();
        call
    })
}
