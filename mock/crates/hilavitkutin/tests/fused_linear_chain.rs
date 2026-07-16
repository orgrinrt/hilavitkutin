//! Within-fiber linear fusion round-trip (D4).
//!
//! Three `RecordOp` work units form a linear RAW chain `In -> Av -> Bv -> Cv`.
//! Registered as separate units, they fold (via `FuseCarrier`) into one
//! `ChainWu` that `Scheduler::run_fused` dispatches: it reads `In`, runs the
//! three maps with the intermediates `Av` / `Bv` held in registers, and writes
//! `Cv`. This pins the D4 contract: the engine folds the registered carrier (the
//! consumer never hand-authors the chain), the fused unit produces the chained
//! computation for every record, and the fused dispatch goes through the
//! ordinary `RunFiber` walk.
//!
//! Red first: before `RecordOp` / `FuseCarrier` / `ChainWu` / `run_fused` exist,
//! the file does not compile. Once they exist but the fold or the chain
//! threading is wrong, the readback diverges from the reference chain. Both
//! precede the green round-trip.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::resource::ColumnPtr;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::RecordOp;
use hilavitkutin_providers::ArenaColumnStorage;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

// Stack-backed test memory provider (mirrors tests/column_dispatch.rs).
struct BumpProvider<const N: usize> {
    buf: UnsafeCell<[MaybeUninit<u8>; N]>,
    used: Cell<usize>,
}
impl<const N: usize> BumpProvider<N> {
    fn new() -> Self {
        Self { buf: UnsafeCell::new([const { MaybeUninit::uninit() }; N]), used: Cell::new(0) }
    }
}
unsafe impl<const N: usize> Send for BumpProvider<N> {}
unsafe impl<const N: usize> Sync for BumpProvider<N> {}
impl<const N: usize> MemoryProviderApi for BumpProvider<N> {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let base = self.buf.get() as *mut u8;
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > N {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: `aligned + len <= N`, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

const N: usize = 16;

// The three pure per-record maps the chain composes.
fn s1(i: u32) -> u32 {
    i.wrapping_mul(2654435761)
}
fn s2(a: u32) -> u32 {
    a.wrapping_mul(2246822519).wrapping_add(1)
}
fn s3(b: u32) -> u32 {
    (b >> 13) ^ b
}
fn chain(i: u32) -> u32 {
    s3(s2(s1(i)))
}

#[derive(Copy, Clone)]
struct InV(u32);
#[derive(Copy, Clone)]
struct Av(u32);
#[derive(Copy, Clone)]
struct Bv(u32);
#[derive(Copy, Clone)]
struct Cv(u32);

type One<T> = Cons<Column<T>, Empty>;

// S1: In -> Av. Implements both WorkUnit (so the build plans + reserves columns)
// and RecordOp (so the engine can fuse the chain).
#[derive(Copy, Clone)]
struct S1;
impl BuilderInput for S1 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S1 {
    type Read = One<InV>;
    type Write = One<Av>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, One<InV>, One<Av>, SnapNil, ColPtrCons<InV, ColPtrNil>, ColPtrCons<Av, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In reserved + host-populated; Av reserved + exclusively
            // written here; the morsel covers only reserved records.
            let v = unsafe { ctx.reader().read::<InV, _>(i) };
            unsafe { ctx.writer().write::<Av, _>(i, Av(s1(v.0))) };
        });
    }
}
impl RecordOp for S1 {
    type In = InV;
    type Out = Av;
    fn apply(&self, x: InV) -> Av {
        Av(s1(x.0))
    }
}

#[derive(Copy, Clone)]
struct S2;
impl BuilderInput for S2 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S2 {
    type Read = One<Av>;
    type Write = One<Bv>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, One<Av>, One<Bv>, SnapNil, ColPtrCons<Av, ColPtrNil>, ColPtrCons<Bv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let a = unsafe { ctx.reader().read::<Av, _>(i) };
            unsafe { ctx.writer().write::<Bv, _>(i, Bv(s2(a.0))) };
        });
    }
}
impl RecordOp for S2 {
    type In = Av;
    type Out = Bv;
    fn apply(&self, x: Av) -> Bv {
        Bv(s2(x.0))
    }
}

#[derive(Copy, Clone)]
struct S3;
impl BuilderInput for S3 {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for S3 {
    type Read = One<Bv>;
    type Write = One<Cv>;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> =
        EngineCtx<'frame, One<Bv>, One<Cv>, SnapNil, ColPtrCons<Bv, ColPtrNil>, ColPtrCons<Cv, ColPtrNil>>;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            let b = unsafe { ctx.reader().read::<Bv, _>(i) };
            unsafe { ctx.writer().write::<Cv, _>(i, Cv(s3(b.0))) };
        });
    }
}
impl RecordOp for S3 {
    type In = Bv;
    type Out = Cv;
    fn apply(&self, x: Bv) -> Cv {
        Cv(s3(x.0))
    }
}

#[test]
fn fused_chain_produces_chained_computation() {
    let provider = BumpProvider::<16384>::new();
    // Columns registered Cv, Bv, Av, In: In is the bindings head (last
    // registered, easy to populate), Cv is three tails deep (the chain output).
    // Units registered S1, S2, S3: the carrier is topological (S1 writes what S2
    // reads, and so on), which `build` requires.
    let mut sched = Scheduler::builder()
        .with(Column::<Cv>::new())
        .with(Column::<Bv>::new())
        .with(Column::<Av>::new())
        .with(Column::<InV>::new())
        .with(S1)
        .with(S2)
        .with(S3)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Host-populate In[i] = i (the bindings head).
    // SAFETY: In's buffer was reserved for N records of InV (repr u32); the
    // scheduler (hence the arena) is alive; each reserved slot is written once.
    let in_base = sched.__bindings().__ptr().as_ptr() as *mut InV;
    for i in 0..N {
        unsafe { *in_base.add(i) = InV(i as u32) };
    }

    // Fused dispatch: the engine folds [S1, S2, S3] into one ChainWu and walks it.
    let _ = sched.run_fused();

    // Cv is the deepest registered column (In -> Av -> Bv -> Cv reversed in the
    // prepend order; registered Cv first, so it sits three tails down from In).
    // The `ColumnPtr<Cv>` annotation pins the node type at compile time: a future
    // change to column registration order that moves Cv off the third tail makes
    // this line fail to compile, rather than silently reading a different column.
    let cv_ptr: ColumnPtr<Cv> = sched.__bindings().__tail().__tail().__tail().__ptr();
    let cv_base = cv_ptr.as_ptr() as *const u32;
    // SAFETY: Cv holds N reserved records; the scheduler (hence storage) is alive.
    let out = unsafe { core::slice::from_raw_parts(cv_base, N) };
    for i in 0..N {
        assert_eq!(
            out[i],
            chain(i as u32),
            "fused chain output at record {i} must equal s3(s2(s1(i)))"
        );
    }
}
