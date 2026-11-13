//! The engine arms: the frame as twelve `WorkUnit`s over thirteen columns.
//!
//! Two generic units cover all twelve: `Map` reads one column and writes one,
//! `Join` reads three and writes one. Each registered unit is a distinct
//! monomorphisation, so each has its own type identity and access set, and the
//! plan sees the DAG the frame describes: three independent three-stage
//! pipelines off the input, a join over their tails, and two independent
//! transforms.
//!
//! What the bench needs and the engine lacks is built here, named as such:
//! the heap-backed bump provider, the `ThreadPoolApi` executor over
//! `std::thread`, and the host-side column access in `column_base`.

use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::pin::Pin;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, ColSelector, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi,
    ColumnWriterApi,
    EachApi,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::{MemoryProviderApi, ThreadPoolApi};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

use crate::{
    COLUMNS,
    FrameArm,
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
};

// ----- the column identities -----

/// A column value that is one u32 word under its own type identity.
///
/// The engine keys columns by type, so thirteen columns need thirteen types.
/// `repr(transparent)` lets the digest read a column back as a `[u32]`.
pub trait Word: Copy + 'static {
    /// The record's word.
    fn get(self) -> u32;
    /// Wrap a word.
    fn of(v: u32) -> Self;
}

macro_rules! words {
    ($($t:ident),* $(,)?) => {$(
        #[derive(Copy, Clone)]
        #[repr(transparent)]
        pub struct $t(u32);
        impl Word for $t {
            #[inline(always)]
            fn get(self) -> u32 {
                self.0
            }
            #[inline(always)]
            fn of(v: u32) -> Self {
                Self(v)
            }
        }
    )*};
}

words!(In, A1, A2, A3, B1, B2, B3, C1, C2, C3, Jn, T1, T2);

type One<T> = Cons<Column<T>, Empty>;
type Three<A, B, C> = Cons<Column<A>, Cons<Column<B>, Cons<Column<C>, Empty>>>;

// ----- the two unit shapes -----

/// One stage: `D[i] = KN::apply(S[i], K)`.
pub struct Map<KN, S, D, const K: u32>(PhantomData<fn() -> (KN, S, D)>);

impl<KN, S, D, const K: u32> Map<KN, S, D, K> {
    /// The unit value. It carries no state; the type is the identity.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<KN: Kernel, S: Word, D: Word, const K: u32> BuilderInput for Map<KN, S, D, K> {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl<KN: Kernel, S: Word, D: Word, const K: u32> WorkUnit<Always> for Map<KN, S, D, K> {
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<S>,
        One<D>,
        SnapNil,
        ColPtrCons<S, ColPtrNil>,
        ColPtrCons<D, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = One<S>;
    type Write = One<D>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: `S` is written for every record before this unit runs
            // (the input by the host, a pipeline column by its producer, ordered
            // before by the RAW edge); `D` is reserved for the record count
            // and written only here; the morsel covers reserved records only.
            let s = unsafe { ctx.reader().read::<S, _>(i) };
            unsafe { ctx.writer().write::<D, _>(i, D::of(KN::apply(s.get(), K))) };
        });
    }
}

/// The fan-in: `D[i] = join(A[i], B[i], C[i])`.
pub struct Join<KN, A, B, C, D>(PhantomData<fn() -> (KN, A, B, C, D)>);

impl<KN, A, B, C, D> Join<KN, A, B, C, D> {
    /// The unit value.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<KN: Kernel, A: Word, B: Word, C: Word, D: Word> BuilderInput for Join<KN, A, B, C, D> {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl<KN: Kernel, A: Word, B: Word, C: Word, D: Word> WorkUnit<Always> for Join<KN, A, B, C, D> {
    type Ctx<'frame> = EngineCtx<
        'frame,
        Three<A, B, C>,
        One<D>,
        SnapNil,
        ColPtrCons<A, ColPtrCons<B, ColPtrCons<C, ColPtrNil>>>,
        ColPtrCons<D, ColPtrNil>,
    >;
    type Hint = (Immediate, Atomic, Normal);
    type Read = Three<A, B, C>;
    type Write = One<D>;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: the three tails are written by their producers, ordered
            // before this unit by the RAW edges; `D` is exclusive here.
            let a = unsafe { ctx.reader().read::<A, _>(i) };
            let b = unsafe { ctx.reader().read::<B, _>(i) };
            let c = unsafe { ctx.reader().read::<C, _>(i) };
            unsafe {
                ctx.writer()
                    .write::<D, _>(i, D::of(join::<KN>(a.get(), b.get(), c.get())))
            };
        });
    }
}

// ----- what the bench supplies that the engine does not -----

/// A heap-backed bump `MemoryProviderApi`: the columns are too big for any
/// fixed arena, and the engine ships the contract but no heap provider.
/// The shape `engine_vs_std` uses.
pub struct HeapBump {
    base: *mut u8,
    cap:  usize,
    used: core::cell::Cell<usize>,
    // Owns the block `base` points into. The heap block does not move when the
    // Box moves into the struct, so `base` stays valid.
    _buf: Box<[MaybeUninit<u8>]>,
}

impl HeapBump {
    /// A provider over a fresh block of `bytes`.
    pub fn new(bytes: usize) -> Self {
        let mut buf: Box<[MaybeUninit<u8>]> = vec![MaybeUninit::uninit(); bytes].into_boxed_slice();
        let base = buf.as_mut_ptr() as *mut u8;
        Self {
            base,
            cap: bytes,
            used: core::cell::Cell::new(0),
            _buf: buf,
        }
    }
}

// SAFETY: the provider is only allocated from during `build`, on the building
// thread; workers read and write columns through pointers it handed out, never
// through the provider itself.
unsafe impl Send for HeapBump {}
unsafe impl Sync for HeapBump {}

impl MemoryProviderApi for HeapBump {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = used.div_ceil(align) * align;
        if aligned + len.0 > self.cap {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: `aligned + len <= cap`, so the result is inside the block.
        unsafe { self.base.add(aligned) }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}

    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: arvo::Bool, _write: arvo::Bool) {}
}

/// Bytes the arena needs for the frame's columns at `n` records, with
/// 64-byte alignment slack per column and headroom for the plan's own stored
/// columns.
pub fn arena_bytes(n: usize) -> usize {
    COLUMNS * n * 4 + COLUMNS * 64 + (1 << 20)
}

/// The column store for `n` records, at the store's default column cap.
///
/// A function because the cap is a defaulted type parameter, and a default is
/// applied only where the type is written out, never inferred in an
/// expression.
fn store(n: usize) -> ArenaColumnStorage<HeapBump> {
    ArenaColumnStorage::new(HeapBump::new(arena_bytes(n)))
}

/// The thread count every threaded arm uses, so the engine and the threaded
/// bars run on the same core budget.
pub fn threads() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

/// A `ThreadPoolApi` over `std::thread`. The engine ships the contract and
/// spawns nothing itself. The scheduler spawns its workers once, on the first
/// `run_parallel`, which lands in the harness's untimed warmup.
#[derive(Copy, Clone, Debug, Default)]
pub struct BenchExecutor;

impl ThreadPoolApi for BenchExecutor {
    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = std::thread::spawn(f);
    }

    fn worker_count(&self) -> USize {
        USize(threads())
    }
}

/// The base pointer of column `T` in a built scheduler's bindings.
///
/// White-box, and the one place this bench reaches through it. The engine has
/// no supported host-side column write: `Column<T>` is written by index from
/// inside a `WorkUnit` during a frame, and a host populating an input column
/// outside the frame has nothing on the supported surface to call.
/// `Scheduler::__bindings()` is `#[doc(hidden)]` and documented as "not part
/// of the supported surface". The type-keyed lookup is the engine's own
/// `ColSelector`, the same one the dispatch uses, so the column is found by
/// its type rather than by counting cons-cells from the head.
// FIXME(ef57305fb5f4): replace with the engine's supported host-side column
// write and read once it has one.
fn column_base<T, B, I>(bindings: &B) -> *mut T
where
    B: ColSelector<T, I>,
{
    // The columns this bench reaches are plain, so the base reads see is the
    // one base the column has and writes land on it too.
    <B as ColSelector<T, I>>::get(bindings).__read().as_ptr()
}

/// Hash `n` records of a column read back through its base pointer.
///
/// # Safety
///
/// `base` must point at `n` initialised records of `T`, alive for the call.
unsafe fn hash_column<T: Word>(base: *const T, n: usize) -> u64 {
    // SAFETY: `T` is a `repr(transparent)` u32; the caller vouches for `n`.
    fnv1a_u32(unsafe { core::slice::from_raw_parts(base as *const u32, n) })
}

/// Build one engine arm for one regime.
///
/// The builder call sequence is written once, in this macro, and expanded once
/// per kernel. It cannot be a function generic over the kernel: the scheduler's
/// type is the output of that sequence and `run` carries where-clauses over it,
/// which a generic body would have to prove for every kernel and cannot name.
/// Each expansion is concrete, so they discharge there. The state is a tuple
/// so the scheduler's type is never written down, and `StateArm` erases it.
///
/// The first pipeline's stages come in as a token list rather than written
/// inline, so the engine negative control is the same builder call sequence
/// with one `Map` left out of that list and nothing else changed. Stating the
/// twelve-unit topology twice would defeat the control: a reader could not tell
/// a deliberate omission from a typo in a second copy.
macro_rules! engine_arm {
    // The first pipeline as the frame describes it: three stages, `In` to `A1`
    // to `A2` to `A3`.
    ($kn:ty, $n:expr, $parallel:expr, full) => {
        engine_arm!(@build $kn, $n, $parallel, [
            Map::<$kn, In, A1, K_A1>,
            Map::<$kn, A1, A2, K_A2>,
            Map::<$kn, A2, A3, K_A3>,
        ])
    };
    // The same pipeline with the `A1` to `A2` stage left out and `A3` taken
    // from `A1` instead: eleven units where the frame has twelve, and the same
    // wrong frame `record_without_a2` computes. `Column::<A2>` stays
    // registered, so the difference from `full` is one `Map` and nothing else.
    ($kn:ty, $n:expr, $parallel:expr, without_a2) => {
        engine_arm!(@build $kn, $n, $parallel, [
            Map::<$kn, In, A1, K_A1>,
            Map::<$kn, A1, A3, K_A3>,
        ])
    };
    (@build $kn:ty, $n:expr, $parallel:expr, [$($a_stage:ty),+ $(,)?]) => {{
        let n: usize = $n;
        let sched = Scheduler::builder()
            .with(Column::<In>::new())
            .with(Column::<A1>::new())
            .with(Column::<A2>::new())
            .with(Column::<A3>::new())
            .with(Column::<B1>::new())
            .with(Column::<B2>::new())
            .with(Column::<B3>::new())
            .with(Column::<C1>::new())
            .with(Column::<C2>::new())
            .with(Column::<C3>::new())
            .with(Column::<Jn>::new())
            .with(Column::<T1>::new())
            .with(Column::<T2>::new())
            $(.with(<$a_stage>::new()))+
            .with(Map::<$kn, In, B1, K_B1>::new())
            .with(Map::<$kn, B1, B2, K_B2>::new())
            .with(Map::<$kn, B2, B3, K_B3>::new())
            .with(Map::<$kn, In, C1, K_C1>::new())
            .with(Map::<$kn, C1, C2, K_C2>::new())
            .with(Map::<$kn, C2, C3, K_C3>::new())
            .with(Join::<$kn, A3, B3, C3, Jn>::new())
            .with(Map::<$kn, In, T1, K_T1>::new())
            .with(Map::<$kn, In, T2, K_T2>::new())
            .build(store(n), USize(n))
            .unwrap_or_else(|_| panic!("frame_wide: the engine scheduler build failed at n={n}"));
        let state = (Box::pin(sched), BenchExecutor, $parallel, n);
        Box::new(StateArm::new(
            state,
            |s, records| {
                assert_eq!(
                    records.len(),
                    s.3,
                    "frame_wide: input carries a different record count"
                );
                let base = column_base::<In, _, _>(Pin::get_ref(s.0.as_ref()).__bindings());
                for (i, &v) in records.iter().enumerate() {
                    // SAFETY: `In` was reserved for `s.3` records and
                    // `records.len() == s.3`; no frame is running.
                    unsafe { base.add(i).write(In::of(v)) };
                }
            },
            |s| {
                // SAFETY: `mark_dirty` and `run` take `&mut self` and neither
                // moves the scheduler out of its pin.
                unsafe { s.0.as_mut().get_unchecked_mut() }.mark_dirty::<Column<In>, _>();
                // A failed frame panics rather than leaving last frame's
                // columns in place for the digest to read as this one's.
                let failed = if s.2 {
                    s.0.as_mut().run_parallel(&s.1).is_err()
                } else {
                    unsafe { s.0.as_mut().get_unchecked_mut() }.run().is_err()
                };
                assert!(!failed, "frame_wide: the engine frame failed at n={}", s.3);
            },
            |s| -> FrameOut {
                let b = Pin::get_ref(s.0.as_ref()).__bindings();
                let n = s.3;
                // SAFETY: each output column holds `n` records, written by the
                // frame that just ran; the scheduler is alive.
                unsafe {
                    [
                        hash_column(column_base::<Jn, _, _>(b), n),
                        hash_column(column_base::<T1, _, _>(b), n),
                        hash_column(column_base::<T2, _, _>(b), n),
                    ]
                }
            },
        )) as Box<dyn FrameArm>
    }};
}

/// Build the single-core or the parallel engine arm for one regime.
pub fn build(regime: Regime, n: usize, parallel: bool) -> Box<dyn FrameArm> {
    match regime {
        Regime::Heavy => engine_arm!(Heavy, n, parallel, full),
        Regime::Light => engine_arm!(Light, n, parallel, full),
    }
}

/// Build the engine negative control for one regime: the same twelve-unit
/// builder call sequence with the first pipeline's second `Map` omitted.
///
/// The fused negative control (`record_without_a2`) proves the validation
/// refuses a wrong frame stated in the `frame_record` switch. It says nothing
/// about the engine arm, whose topology is stated independently in the builder
/// call sequence above, and that spelling is the one a reader can least check
/// by eye: thirteen `Column` lines and twelve `Map` and `Join` lines whose
/// correctness is in which type parameter sits in which position. This arm is
/// that spelling with one unit missing, run through the same validation path.
pub fn build_without_a2(regime: Regime, n: usize) -> Box<dyn FrameArm> {
    match regime {
        Regime::Heavy => engine_arm!(Heavy, n, false, without_a2),
        Regime::Light => engine_arm!(Light, n, false, without_a2),
    }
}
