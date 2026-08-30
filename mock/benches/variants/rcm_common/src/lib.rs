//! Shared workload for the rcm_order bench (the A1-1 dispatch-order oracle).
//!
//! K = 8 independent WUs at the same topo depth. WU k reads input columns
//! C{k} and C{k+1} and writes its own output O{k}: consecutive indices share
//! exactly one input column, non-consecutive indices share nothing, and all
//! WUs are write-disjoint, so every permutation is a valid topological order.
//! The only variable between the two bench variants is the WU registration
//! order, which is the carrier order and hence the single-core dispatch
//! order; an execution-order probe asserts that assumption per prepared
//! scheduler instead of trusting it.
//!
//! `prepare` builds the scheduler for one order, fills the inputs from the
//! harness seed, runs one untimed warm frame (verifying the probe), and
//! returns type-erased closures: `run_frame` (mark inputs dirty, run one
//! frame; the variant times exactly this) and `finish` (FNV over the eight
//! output columns, for the harness's cross-variant byte-exact validation).

use core::cell::Cell;
use core::mem::MaybeUninit;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use arvo::USize;
use hilavitkutin::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::platform::MemoryProviderApi;
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;

static PROBE: AtomicBool = AtomicBool::new(false);
static ORDER: Mutex<Vec<usize>> = Mutex::new(Vec::new());

macro_rules! def_col {
    ($name:ident) => {
        #[derive(Copy, Clone)]
        #[allow(dead_code)] // read/written through raw column pointers
        pub struct $name(pub u32);
    };
}

def_col!(C0);
def_col!(C1);
def_col!(C2);
def_col!(C3);
def_col!(C4);
def_col!(C5);
def_col!(C6);
def_col!(C7);
def_col!(C8);
def_col!(O0);
def_col!(O1);
def_col!(O2);
def_col!(O3);
def_col!(O4);
def_col!(O5);
def_col!(O6);
def_col!(O7);

type Two<A, B> = Cons<Column<A>, Cons<Column<B>, Empty>>;
type One<T> = Cons<Column<T>, Empty>;

macro_rules! def_wu {
    ($wu:ident, $idx:expr, $ca:ident, $cb:ident, $o:ident) => {
        pub struct $wu;
        impl BuilderInput for $wu {
            type Init = Self;
            type Dispatch = UnitDispatch<Self>;
        }
        impl WorkUnit<Always> for $wu {
            type Read = Two<$ca, $cb>;
            type Write = One<$o>;
            type Hint = (Immediate, Atomic, Normal);
            type Ctx<'frame> = EngineCtx<
                'frame,
                Two<$ca, $cb>,
                One<$o>,
                SnapNil,
                ColPtrCons<$ca, ColPtrCons<$cb, ColPtrNil>>,
                ColPtrCons<$o, ColPtrNil>,
            >;
            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                ctx.each().run(|i| {
                    if i.0 == 0 && PROBE.load(Ordering::Relaxed) {
                        ORDER.lock().unwrap().push($idx);
                    }
                    // SAFETY: inputs host-populated for the record count; the
                    // output reserved and exclusively written here; the morsel
                    // covers only reserved records.
                    let a = unsafe { ctx.reader().read::<$ca, _>(i) };
                    let b = unsafe { ctx.reader().read::<$cb, _>(i) };
                    let v = a.0.wrapping_mul(0x9E37_79B1) ^ b.0.rotate_left(13);
                    unsafe { ctx.writer().write::<$o, _>(i, $o(v)) };
                });
            }
        }
    };
}

def_wu!(W0, 0, C0, C1, O0);
def_wu!(W1, 1, C1, C2, O1);
def_wu!(W2, 2, C2, C3, O2);
def_wu!(W3, 3, C3, C4, O3);
def_wu!(W4, 4, C4, C5, O4);
def_wu!(W5, 5, C5, C6, O5);
def_wu!(W6, 6, C6, C7, O6);
def_wu!(W7, 7, C7, C8, O7);

// ----- memory provider -----

pub struct HeapBump {
    base: *mut u8,
    cap: usize,
    used: Cell<usize>,
    _buf: Box<[MaybeUninit<u8>]>,
}

impl HeapBump {
    pub fn new(bytes: usize) -> Self {
        let mut buf: Box<[MaybeUninit<u8>]> = vec![MaybeUninit::uninit(); bytes].into_boxed_slice();
        let base = buf.as_mut_ptr() as *mut u8;
        Self {
            base,
            cap: bytes,
            used: Cell::new(0),
            _buf: buf,
        }
    }
}

unsafe impl Send for HeapBump {}
unsafe impl Sync for HeapBump {}

impl MemoryProviderApi for HeapBump {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8 {
        let used = self.used.get();
        let align = align.0.max(1);
        let aligned = (used + align - 1) / align * align;
        if aligned + len.0 > self.cap {
            return core::ptr::null_mut();
        }
        self.used.set(aligned + len.0);
        // SAFETY: `aligned + len <= cap`, in bounds of the owned buffer.
        unsafe { self.base.add(aligned) }
    }
    unsafe fn deallocate(&self, _ptr: *mut u8, _len: USize, _align: USize) {}
    unsafe fn protect(&self, _ptr: *mut u8, _len: USize, _read: arvo::Bool, _write: arvo::Bool) {}
}

pub fn fnv1a_u32_slice(vals: &[u32]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for v in vals {
        for b in v.to_le_bytes() {
            h = (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    h
}

// ----- bindings depth walk -----
//
// Columns register O0..O7 then C0..C8 in both variants (identical
// registration keeps the arena layout identical, isolating dispatch order as
// the only variable). Bindings head is the last-registered column: C8 at
// depth 0, C{i} at depth 8-i, O{j} at depth 16-j.

macro_rules! tails {
    ($e:expr;) => { $e };
    ($e:expr; T $($r:tt)*) => { tails!($e.__tail(); $($r)*) };
}

macro_rules! mark_all_inputs_dirty {
    ($s:expr) => {
        $s.mark_dirty::<Column<C0>, _>();
        $s.mark_dirty::<Column<C1>, _>();
        $s.mark_dirty::<Column<C2>, _>();
        $s.mark_dirty::<Column<C3>, _>();
        $s.mark_dirty::<Column<C4>, _>();
        $s.mark_dirty::<Column<C5>, _>();
        $s.mark_dirty::<Column<C6>, _>();
        $s.mark_dirty::<Column<C7>, _>();
        $s.mark_dirty::<Column<C8>, _>();
    };
}

macro_rules! reg_columns {
    ($b:expr) => {
        $b.with(Column::<O0>::new())
            .with(Column::<O1>::new())
            .with(Column::<O2>::new())
            .with(Column::<O3>::new())
            .with(Column::<O4>::new())
            .with(Column::<O5>::new())
            .with(Column::<O6>::new())
            .with(Column::<O7>::new())
            .with(Column::<C0>::new())
            .with(Column::<C1>::new())
            .with(Column::<C2>::new())
            .with(Column::<C3>::new())
            .with(Column::<C4>::new())
            .with(Column::<C5>::new())
            .with(Column::<C6>::new())
            .with(Column::<C7>::new())
            .with(Column::<C8>::new())
    };
}

/// Which registration (= dispatch) order to prepare.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Order {
    /// Column-adjacent order 0..8: the RCM-like row order. Consecutive
    /// fibers share one input column, so the second read finds it warm.
    Adj,
    /// Stride-4 scramble 0,4,1,5,2,6,3,7: an equally valid topological
    /// order in which no two consecutive fibers share a column.
    Scr,
    /// Reverse order 7..0: full adjacency preserved (WU k and k-1 share
    /// column C{k}) under a different numbering. Control arm: if the
    /// mechanism is column adjacency and not the specific 0..8 numbering,
    /// this arm matches `Adj`.
    Rev,
    /// Half-adjacent order 0,1,4,5,2,3,6,7: four of the seven consecutive
    /// transitions share a column. Dose-response midpoint between `Adj`
    /// (seven of seven) and `Scr` (zero of seven).
    Half,
}

/// A prepared scheduler behind type-erased closures. `run_frame` marks all
/// inputs dirty and runs one frame (the timed unit); `finish` hashes the
/// eight output columns.
pub struct Prepared {
    pub run_frame: Box<dyn FnMut()>,
    pub finish: Box<dyn FnMut() -> u64>,
}

macro_rules! prepare_order {
    ($seed:expr, $records:expr, [$($wu:expr),* $(,)?], $expected:expr) => {{
        let seed = $seed;
        let records = $records;
        let provider = HeapBump::new(17 * records * 4 + 17 * 64 + (1 << 16));
        let sched = reg_columns!(Scheduler::builder())
            $(.with($wu))*
            .build(
                ArenaColumnStorage::<_, arvo_tensor::Dim<256>>::new(provider),
                USize(records),
            )
            .unwrap_or_else(|_| panic!("engine build should succeed"));
        let sched = Rc::new(RefCell::new(sched));

        {
            let s = sched.borrow();
            let b = s.__bindings();
            let cols = [
                tails!(b; T T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T).__ptr().as_ptr() as *mut u32,
                tails!(b;).__ptr().as_ptr() as *mut u32,
            ];
            for (k, &base) in cols.iter().enumerate() {
                let mix = seed.wrapping_add(k as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
                for i in 0..records {
                    // SAFETY: each input buffer was reserved for the record
                    // count; each reserved slot is written exactly once here.
                    unsafe {
                        *base.add(i) =
                            (i as u32).wrapping_mul(2 * k as u32 + 1) ^ (mix >> 32) as u32
                    };
                }
            }
        }

        // Untimed warm frame with the execution-order probe on: verify the
        // registration order IS the dispatch order, or fail loudly (a
        // mismatch voids the bench premise and is itself a finding about
        // the A1-1 fork's mechanics).
        ORDER.lock().unwrap().clear();
        PROBE.store(true, Ordering::Relaxed);
        {
            let mut s = sched.borrow_mut();
            mark_all_inputs_dirty!(s);
            let _ = s.run();
        }
        PROBE.store(false, Ordering::Relaxed);
        // The probe fires once per MORSEL (the each() index is
        // morsel-local), so collapse consecutive repeats to recover
        // the fiber dispatch order.
        let raw = ORDER.lock().unwrap().clone();
        let mut observed: Vec<usize> = Vec::new();
        for v in raw {
            if observed.last() != Some(&v) {
                observed.push(v);
            }
        }
        assert_eq!(
            observed, $expected,
            "dispatch order does not match registration order; the bench \
             premise (carrier order = dispatch order) is void, record this \
             as an A1-1 finding instead of trusting the numbers"
        );

        let frame_rc = Rc::clone(&sched);
        let run_frame = Box::new(move || {
            let mut s = frame_rc.borrow_mut();
            mark_all_inputs_dirty!(s);
            let r = s.run();
            core::hint::black_box(&r);
        }) as Box<dyn FnMut()>;

        let finish_rc = Rc::clone(&sched);
        let finish = Box::new(move || {
            let s = finish_rc.borrow();
            let b = s.__bindings();
            let outs = [
                tails!(b; T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T).__ptr().as_ptr() as *const u32,
            ];
            let mut h = 0u64;
            for &base in &outs {
                // SAFETY: each output holds `records` reserved records
                // written by its WU on the last frame.
                let slice = unsafe { core::slice::from_raw_parts(base, records) };
                h ^= fnv1a_u32_slice(slice);
            }
            h
        }) as Box<dyn FnMut() -> u64>;

        Prepared { run_frame, finish }
    }};
}

pub fn prepare(order: Order, seed: u64, records: usize) -> Prepared {
    match order {
        Order::Adj => prepare_order!(
            seed,
            records,
            [W0, W1, W2, W3, W4, W5, W6, W7],
            vec![0usize, 1, 2, 3, 4, 5, 6, 7]
        ),
        Order::Scr => prepare_order!(
            seed,
            records,
            [W0, W4, W1, W5, W2, W6, W3, W7],
            vec![0usize, 4, 1, 5, 2, 6, 3, 7]
        ),
        Order::Rev => prepare_order!(
            seed,
            records,
            [W7, W6, W5, W4, W3, W2, W1, W0],
            vec![7usize, 6, 5, 4, 3, 2, 1, 0]
        ),
        Order::Half => prepare_order!(
            seed,
            records,
            [W0, W1, W4, W5, W2, W3, W6, W7],
            vec![0usize, 1, 4, 5, 2, 3, 6, 7]
        ),
    }
}
