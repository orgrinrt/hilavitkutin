//! GATE-2 deviation 9: threaded unit-outer accumulator path.
//!
//! An accumulator-bearing carrier runs unit-outer. Under `run_parallel` on a
//! multi-core pool, each core takes its head+tail record slice, appends into its
//! own per-core region of the reserved buffer (offset to the slice start, fresh
//! live cell), and a post-frame forward compaction merges the per-core regions
//! into the binding's `[0, sum)` prefix. The result must be byte-identical to the
//! single-core `run()` append: same values, same order.
//!
//! The appender keeps records whose host-populated input value is not a multiple
//! of seven, appending `value * 10`. The keep is conditional (at most one append
//! per record), the convergence-accumulator pattern the first slice scopes to.
//!
//! Lives under `tests/` so the bare numeric record values do not trip the
//! src-tree primitive lints.

use core::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

use arvo::{Bool, USize};
use hilavitkutin::OsThreadPool;
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil,
};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{AccumWriterApi, ColumnReaderApi, ColumnWriterApi, EachApi, HasAccumWriter, HasColumnReader, HasColumnWriter, HasEach};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::{Accum, Column};
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use notko::Outcome;

fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

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
        // SAFETY: aligned + len <= N, in bounds of the owned buffer.
        unsafe { base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: Bool, _write: Bool) {}
}

const N: usize = 256;

#[derive(Copy, Clone)]
struct Inv(u32);
#[derive(Copy, Clone)]
struct Av(u32);

type ColIn = Cons<Column<Inv>, Empty>;
type AccW = Cons<Accum<Av>, Empty>;

// Keep-and-map appender: reads In, appends `In*10` for records whose value is not
// a multiple of seven (at most one append per record). The value derives from the
// (windowed) input via the reader, which maps the morsel-relative each() index to
// the absolute record, so the per-core slice [lo, hi) appends the right values.
struct KeepWu;
impl BuilderInput for KeepWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for KeepWu {
    type Read = ColIn;
    type Write = AccW;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColIn,
        AccW,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrNil,
        AccPtrCons<'frame, Av, AccPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In is host-populated for N records; the reader maps the
            // relative index to the absolute column index, in bounds.
            let v = unsafe { ctx.reader().read::<Inv, _>(i) };
            if v.0 % 7 != 0 {
                // SAFETY: the accumulator reserves N records (>= the kept count),
                // and the plan proved this unit the exclusive appender.
                unsafe { ctx.accums().append::<Av, _>(Av(v.0 * 10)) };
            }
        });
    }
}

// Seed a built scheduler: host-populate In[i] = i and poison the accumulator
// buffer. Accum is registered first so the bindings head is the AccumBinding; In
// sits one tail down. Macro (not fn) so the unnameable Scheduler type stays
// inferred at each build site.
macro_rules! build_and_seed {
    ($provider:expr) => {{
        let scheduler = Scheduler::builder()
            .with(Accum::<Av>::new())
            .with(Column::<Inv>::new())
            .with(KeepWu)
            .build(store($provider), USize(N))
            .unwrap_or_else(|_| panic!("build should succeed"));
        {
            // Bindings head is `ColumnBinding<Inv>` (In), tail is the
            // `AccumBinding<Av>`.
            let bindings = scheduler.__bindings();
            let in_base = bindings.__ptr().as_ptr() as *mut u32;
            let acc_base = bindings.__tail().__ptr().as_ptr() as *mut u32;
            for i in 0..N {
                // SAFETY: both buffers reserve N u32 records; written once here.
                unsafe {
                    *in_base.add(i) = i as u32;
                    *acc_base.add(i) = u32::MAX;
                }
            }
        }
        scheduler
    }};
}

// The single-core reference: kept values In*10 in record order.
fn reference() -> ([u32; N], usize) {
    let mut out = [0u32; N];
    let mut k = 0;
    for i in 0..N {
        if (i as u32) % 7 != 0 {
            out[k] = (i as u32) * 10;
            k += 1;
        }
    }
    (out, k)
}

#[test]
fn run_parallel_accumulator_matches_single_core() {
    let (refbuf, reflen) = reference();

    // Single-core baseline (sanity: confirms the reference + the append path).
    let mut sc = build_and_seed!(BumpProvider::<8192>::new());
    assert!(matches!(sc.run(), Outcome::Ok(())));
    let b = sc.__bindings().__tail();
    let live = b.__len_cell().get().0;
    assert_eq!(live, reflen, "single-core live length is the kept count");
    let base = b.__ptr().as_ptr();
    for k in 0..live {
        // SAFETY: live records initialised by the appends.
        let v = unsafe { core::ptr::read(base.add(k)) };
        assert_eq!(v.0, refbuf[k], "single-core rec {k} value/order");
    }

    // Threaded path: same setup, dispatched via run_parallel on the pool.
    let parallel = build_and_seed!(BumpProvider::<8192>::new());
    let pool = OsThreadPool::new();
    let mut parallel = core::pin::pin!(parallel);
    let result = parallel.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    let pref = parallel.as_ref();
    let pb = pref.__bindings().__tail();
    let plive = pb.__len_cell().get().0;
    assert_eq!(
        plive, reflen,
        "threaded merged live length equals the single-core kept count"
    );
    let pbase = pb.__ptr().as_ptr();
    for k in 0..plive {
        // SAFETY: merged prefix holds `plive` initialised records.
        let v = unsafe { core::ptr::read(pbase.add(k)) };
        assert_eq!(
            v.0, refbuf[k],
            "threaded merged rec {k} is byte-identical to single-core append order"
        );
    }
}

// ---- probe: an accumulator in a phase that carries a second trunk.
//
// Two column-disjoint units in one phase means two trunks, which takes the phase
// off the tphase == 1 branch and onto the trunk-rank walk. There the core owning
// KeepWu's trunk walks every record rather than a per-core slice, while its
// accumulator region is still sized from that slice. This exists to find out
// whether that configuration can be built and run, not to assert a conclusion.

#[derive(Copy, Clone)]
#[allow(dead_code)] // written by SideWu, never read back; presence forces a second trunk
struct Sv(u32);

type ColSide = Cons<Column<Sv>, Empty>;

/// Writes its own column and nothing else, so it is a second trunk in KeepWu's
/// phase and touches no accumulator.
struct SideWu;
impl BuilderInput for SideWu {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}
impl WorkUnit<Always> for SideWu {
    type Read = ColIn;
    type Write = ColSide;
    type Hint = (
        hilavitkutin_api::hint::Immediate,
        hilavitkutin_api::hint::Atomic,
        hilavitkutin_api::hint::Normal,
    );
    type Ctx<'frame> = EngineCtx<
        'frame,
        ColIn,
        ColSide,
        SnapNil,
        ColPtrCons<Inv, ColPtrNil>,
        ColPtrCons<Sv, ColPtrNil>,
    >;
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: In host-populated for N records; Sv reserved + exclusive.
            let v = unsafe { ctx.reader().read::<Inv, _>(i) };
            unsafe { ctx.writer().write::<Sv, _>(i, Sv(v.0 + 1)) };
        });
    }
}

#[test]
fn accumulator_in_a_two_trunk_phase_matches_single_core() {
    let (refbuf, reflen) = reference();

    let provider = BumpProvider::<16384>::new();
    let scheduler = Scheduler::builder()
        .with(Accum::<Av>::new())
        .with(Column::<Inv>::new())
        .with(Column::<Sv>::new())
        .with(KeepWu)
        .with(SideWu)
        .build(store(provider), USize(N))
        .unwrap_or_else(|_| panic!("build should succeed"));
    {
        // Bindings head: Sv, then In, then the AccumBinding<Av>.
        let b = scheduler.__bindings();
        let in_base = b.__tail().__ptr().as_ptr() as *mut u32;
        let acc_base = b.__tail().__tail().__ptr().as_ptr() as *mut u32;
        for i in 0..N {
            // SAFETY: both buffers reserve N u32 records; written once here.
            unsafe {
                *in_base.add(i) = i as u32;
                *acc_base.add(i) = u32::MAX;
            }
        }
    }

    let pool = OsThreadPool::new();
    let mut scheduler = core::pin::pin!(scheduler);
    let result = scheduler.as_mut().run_parallel(&pool);
    assert!(matches!(result, Outcome::Ok(())));

    let sched_ref = scheduler.as_ref();
    let b = sched_ref.__bindings().__tail().__tail();
    let live = b.__len_cell().get().0;
    assert_eq!(
        live, reflen,
        "two-trunk parallel live length is the kept count; a different count means \
         the appends landed outside the per-core region arithmetic"
    );
    let base = b.__ptr().as_ptr();
    for k in 0..live {
        // SAFETY: live records initialised by the appends.
        let v = unsafe { core::ptr::read(base.add(k)) };
        assert_eq!(v.0, refbuf[k], "two-trunk parallel rec {k} value/order");
    }
}
