//! Shared workload for the rcm_rivals bench (ordering-theory comparison).
//!
//! The chain topology in `rcm_common` cannot separate ordering theories:
//! on a chain, RCM, spectral, and greedy nearest-neighbour all collapse to
//! the same adjacency order. This workload uses a 3x4 grid sharing
//! topology where they genuinely diverge. Columns are the 17 grid EDGES;
//! WU n (one per grid node) reads the edge columns incident to node n
//! (two at corners, three on borders, four in the middle) and writes its
//! own output O{n}. All 12 WUs are write-disjoint at one topo depth, so
//! every permutation is a valid topological order.
//!
//! The five orders under test, with their linear-arrangement metrics over
//! the 17 shared edges (distance = |position difference| of the two
//! endpoint WUs in the dispatch order; adjacent = consecutive dispatch
//! pair shares an edge column):
//!
//! | order | generator | bandwidth | total distance | adjacent |
//! |---|---|---|---|---|
//! | Rcm | reverse Cuthill-McKee from node 0, degree-ascending levels | 4 | 41 | 4/11 |
//! | Spec | Fiedler sweep along the long axis (column-major) | 3 | 35 | 8/11 |
//! | Snake | greedy nearest-neighbour from node 0 (row serpentine) | 7 | 41 | 11/11 |
//! | RowM | row-major node order (naive registration baseline) | 4 | 41 | 9/11 |
//! | Scr | interleave picked so no consecutive pair shares an edge | 9 | 65 | 0/11 |
//!
//! RCM minimises the MAXIMUM reuse distance (bandwidth), spectral the
//! TOTAL (minimum linear arrangement), snake the immediate reuse. Which
//! objective wins per cache regime is the bench question; canon Step 5
//! names RCM specifically, so a consistent spectral or snake win at the
//! sizes that matter is a canon-level finding.
//!
//! The orders are precomputed by hand from the generators above and
//! hardcoded (WU registration is compile-time, so a runtime-computed
//! order cannot exist); the execution-order probe asserts dispatch =
//! registration per prepared scheduler. Column registration (outputs
//! then edges) is identical across all arms, so the arena layout is
//! constant and dispatch order is the only variable.

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
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_providers::ArenaColumnStorage;
use rcm_common::{fnv1a_u32_slice, HeapBump};

static PROBE: AtomicBool = AtomicBool::new(false);
static ORDER: Mutex<Vec<usize>> = Mutex::new(Vec::new());

macro_rules! def_col {
    ($($name:ident)*) => { $(
        #[derive(Copy, Clone)]
        #[allow(dead_code)] // read/written through raw column pointers
        pub struct $name(pub u32);
    )* };
}

// 17 edge columns: E0..E8 horizontal (row r, col c -> index r*3+c),
// E9..E16 vertical (row r, col c -> index 9 + r*4+c). 12 outputs.
def_col!(E0 E1 E2 E3 E4 E5 E6 E7 E8 E9 E10 E11 E12 E13 E14 E15 E16);
def_col!(O0 O1 O2 O3 O4 O5 O6 O7 O8 O9 O10 O11);

macro_rules! cols_cons {
    () => { Empty };
    ($h:ident $(, $t:ident)*) => { Cons<Column<$h>, cols_cons!($($t),*)> };
}
macro_rules! colptr_cons {
    () => { ColPtrNil };
    ($h:ident $(, $t:ident)*) => { ColPtrCons<$h, colptr_cons!($($t),*)> };
}

macro_rules! def_wu {
    ($wu:ident, $idx:expr, [$($c:ident),+], $o:ident) => {
        pub struct $wu;
        impl BuilderInput for $wu {
            type Init = Self;
            type Dispatch = UnitDispatch<Self>;
        }
        impl WorkUnit<Always> for $wu {
            type Read = cols_cons!($($c),+);
            type Write = cols_cons!($o);
            type Hint = (Immediate, Atomic, Normal);
            type Ctx<'frame> = EngineCtx<
                'frame,
                cols_cons!($($c),+),
                cols_cons!($o),
                SnapNil,
                colptr_cons!($($c),+),
                colptr_cons!($o),
            >;
            fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                ctx.each().run(|i| {
                    if i.0 == 0 && PROBE.load(Ordering::Relaxed) {
                        ORDER.lock().unwrap().push($idx);
                    }
                    // SAFETY: inputs host-populated for the record count; the
                    // output reserved and exclusively written here; the morsel
                    // covers only reserved records.
                    let mut v: u32 = 0x9E37_79B1u32.wrapping_mul($idx as u32 + 1);
                    $(
                        let x = unsafe { ctx.reader().read::<$c, _>(i) };
                        v = v.wrapping_mul(0x85EB_CA6B) ^ x.0.rotate_left(11);
                    )+
                    unsafe { ctx.writer().write::<$o, _>(i, $o(v)) };
                });
            }
        }
    };
}

// One WU per grid node; reads = incident edges of node n on the 3x4 grid.
def_wu!(G0, 0, [E0, E9], O0);
def_wu!(G1, 1, [E0, E1, E10], O1);
def_wu!(G2, 2, [E1, E2, E11], O2);
def_wu!(G3, 3, [E2, E12], O3);
def_wu!(G4, 4, [E3, E9, E13], O4);
def_wu!(G5, 5, [E3, E4, E10, E14], O5);
def_wu!(G6, 6, [E4, E5, E11, E15], O6);
def_wu!(G7, 7, [E5, E12, E16], O7);
def_wu!(G8, 8, [E6, E13], O8);
def_wu!(G9, 9, [E6, E7, E14], O9);
def_wu!(G10, 10, [E7, E8, E15], O10);
def_wu!(G11, 11, [E8, E16], O11);

/// Which precomputed dispatch order to prepare. Generators and metrics in
/// the module docs above.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GridOrder {
    /// Reverse Cuthill-McKee: 11,10,7,6,9,3,5,2,8,4,1,0.
    Rcm,
    /// Fiedler/column-major sweep: 0,4,8,1,5,9,2,6,10,3,7,11.
    Spec,
    /// Greedy nearest-neighbour serpentine: 0,1,2,3,7,6,5,4,8,9,10,11.
    Snake,
    /// Row-major 0..12, the naive registration baseline.
    RowM,
    /// Interleave 0,6,1,7,2,8,3,9,4,10,5,11: no consecutive pair shares.
    Scr,
}

/// A prepared scheduler behind type-erased closures. `run_frame` marks all
/// inputs dirty and runs one frame (the timed unit); `finish` hashes the
/// twelve output columns.
pub struct Prepared {
    pub run_frame: Box<dyn FnMut()>,
    pub finish: Box<dyn FnMut() -> u64>,
}

// ----- bindings depth walk -----
//
// Columns register O0..O11 then E0..E16 in every arm (identical
// registration keeps the arena layout identical, isolating dispatch order
// as the only variable). Bindings head is the last-registered column:
// E16 at depth 0, E{i} at depth 16-i, O{j} at depth 28-j.

macro_rules! tails {
    ($e:expr;) => { $e };
    ($e:expr; T $($r:tt)*) => { tails!($e.__tail(); $($r)*) };
}

macro_rules! mark_all_inputs_dirty {
    ($s:expr) => {
        $s.mark_dirty::<Column<E0>, _>();
        $s.mark_dirty::<Column<E1>, _>();
        $s.mark_dirty::<Column<E2>, _>();
        $s.mark_dirty::<Column<E3>, _>();
        $s.mark_dirty::<Column<E4>, _>();
        $s.mark_dirty::<Column<E5>, _>();
        $s.mark_dirty::<Column<E6>, _>();
        $s.mark_dirty::<Column<E7>, _>();
        $s.mark_dirty::<Column<E8>, _>();
        $s.mark_dirty::<Column<E9>, _>();
        $s.mark_dirty::<Column<E10>, _>();
        $s.mark_dirty::<Column<E11>, _>();
        $s.mark_dirty::<Column<E12>, _>();
        $s.mark_dirty::<Column<E13>, _>();
        $s.mark_dirty::<Column<E14>, _>();
        $s.mark_dirty::<Column<E15>, _>();
        $s.mark_dirty::<Column<E16>, _>();
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
            .with(Column::<O8>::new())
            .with(Column::<O9>::new())
            .with(Column::<O10>::new())
            .with(Column::<O11>::new())
            .with(Column::<E0>::new())
            .with(Column::<E1>::new())
            .with(Column::<E2>::new())
            .with(Column::<E3>::new())
            .with(Column::<E4>::new())
            .with(Column::<E5>::new())
            .with(Column::<E6>::new())
            .with(Column::<E7>::new())
            .with(Column::<E8>::new())
            .with(Column::<E9>::new())
            .with(Column::<E10>::new())
            .with(Column::<E11>::new())
            .with(Column::<E12>::new())
            .with(Column::<E13>::new())
            .with(Column::<E14>::new())
            .with(Column::<E15>::new())
            .with(Column::<E16>::new())
    };
}

macro_rules! prepare_order {
    ($seed:expr, $records:expr, [$($wu:expr),* $(,)?], $expected:expr) => {{
        let seed = $seed;
        let records = $records;
        let provider = HeapBump::new(30 * records * 4 + 29 * 64 + (1 << 17));
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
            // E{i} sits at bindings depth 16-i: E16 first, E0 last.
            let cols = [
                tails!(b;).__ptr().as_ptr() as *mut u32,
                tails!(b; T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T T T T T T T T T).__ptr().as_ptr() as *mut u32,
                tails!(b; T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *mut u32,
            ];
            // cols[d] is depth d = edge E{16-d}; iterate as edge index k.
            for k in 0..17usize {
                let base = cols[16 - k];
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
        // mismatch voids the bench premise).
        ORDER.lock().unwrap().clear();
        PROBE.store(true, Ordering::Relaxed);
        {
            let mut s = sched.borrow_mut();
            mark_all_inputs_dirty!(s);
            let _ = s.run();
        }
        PROBE.store(false, Ordering::Relaxed);
        // The probe fires once per MORSEL; collapse consecutive repeats to
        // recover the fiber dispatch order.
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
             as a finding instead of trusting the numbers"
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
            // O{j} sits at bindings depth 28-j: O11 at 17, O0 at 28.
            let outs = [
                tails!(b; T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
                tails!(b; T T T T T T T T T T T T T T T T T T T T T T T T T T T T).__ptr().as_ptr() as *const u32,
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

pub fn prepare(order: GridOrder, seed: u64, records: usize) -> Prepared {
    match order {
        GridOrder::Rcm => prepare_order!(
            seed,
            records,
            [G11, G10, G7, G6, G9, G3, G5, G2, G8, G4, G1, G0],
            vec![11usize, 10, 7, 6, 9, 3, 5, 2, 8, 4, 1, 0]
        ),
        GridOrder::Spec => prepare_order!(
            seed,
            records,
            [G0, G4, G8, G1, G5, G9, G2, G6, G10, G3, G7, G11],
            vec![0usize, 4, 8, 1, 5, 9, 2, 6, 10, 3, 7, 11]
        ),
        GridOrder::Snake => prepare_order!(
            seed,
            records,
            [G0, G1, G2, G3, G7, G6, G5, G4, G8, G9, G10, G11],
            vec![0usize, 1, 2, 3, 7, 6, 5, 4, 8, 9, 10, 11]
        ),
        GridOrder::RowM => prepare_order!(
            seed,
            records,
            [G0, G1, G2, G3, G4, G5, G6, G7, G8, G9, G10, G11],
            vec![0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        ),
        GridOrder::Scr => prepare_order!(
            seed,
            records,
            [G0, G6, G1, G7, G2, G8, G3, G9, G4, G10, G5, G11],
            vec![0usize, 6, 1, 7, 2, 8, 3, 9, 4, 10, 5, 11]
        ),
    }
}
