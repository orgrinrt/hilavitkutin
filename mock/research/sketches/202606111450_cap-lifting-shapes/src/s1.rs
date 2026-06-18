//! s1: route every cap through the Capacity associated-type pattern.
//!
//! The pattern PlanDims already uses for topo_order / fiber_dispatch:
//! the cap is a TYPE (`Dim<N>: Capacity`), the field is the GAT
//! projection `<C::X as Capacity>::Array<T>`, and no const expression
//! ever sits in array-length position, so generic_const_exprs is not
//! needed in this consumer at all (this module compiles without the
//! gate; see main.rs cfg_attr).
//!
//! Three probes:
//!
//! 1. Scheduler-shaped struct: plan_dirty (the #345 field), worker_ctxs
//!    (the MAX_CORES array), and the per-(core,accum) publish array as
//!    the nested 2-D composition `Cores::Array<Accums::Array<_>>`
//!    replacing the flat `[_; MAX_CORES * GATE2_MAX_ACCUMS]` (a
//!    capacity product in type position would need GCE; nesting does
//!    not).
//! 2. classify_cores lift: return `<C::Cores as Capacity>::Array<_>`
//!    instead of `[CoreClass; MAX_CORES]`.
//! 3. The const-grouping scratch (the GATE2_MAX_UNITS wall): a
//!    masks_of-shaped 3-layer const fn chain over a BundleMasks-shaped
//!    [const] cons recursion, with the scratch typed
//!    `<CU as ConstCapacity>::Array<_>` instead of
//!    `[_; GATE2_MAX_UNITS]`. The one missing piece upstream is a
//!    const slice accessor on the GAT array (Capacity's AsRef/AsMut is
//!    not const-callable); the sketch-local `CapSliceMut` const trait
//!    below stands in for that arvo-tensor addition (a one-impl,
//!    Dim<N>-only `&mut [T; N] -> &mut [T]` coercion).

use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use arvo::{Identity, USize};
use arvo_tensor::{Capacity, ConstCapacity, Dim};
use hilavitkutin::plan::AccessMask;

// ---------------------------------------------------------------- caps

/// The consumer-tunable cap bundle: each engine cap as a Capacity type.
/// The engine's PlanDims already carries Units (Capacity + ConstCapacity)
/// and Cores; PlanAffecting and AccumsPerCore are the new dimensions a
/// real lift would add to it.
pub trait EngineCaps {
    type PlanAffecting: Capacity;
    type Units: Capacity + ConstCapacity;
    type Cores: Capacity;
    type AccumsPerCore: Capacity;
}

/// Mirror of today's hardcoded budget.
pub struct DefaultCaps;
impl EngineCaps for DefaultCaps {
    type PlanAffecting = Dim<256>;
    type Units = Dim<256>;
    type Cores = Dim<256>;
    type AccumsPerCore = Dim<16>;
}

/// A consumer-shrunk budget, proving the tunability direction.
pub struct TinyCaps;
impl EngineCaps for TinyCaps {
    type PlanAffecting = Dim<8>;
    type Units = Dim<8>;
    type Cores = Dim<4>;
    type AccumsPerCore = Dim<2>;
}

// ------------------------------------------- probe 1: scheduler fields

struct WorkerCtx {
    sched: *const (),
    core_id: usize,
}

/// Scheduler-shaped struct: the three capped fields, capacity-typed.
pub struct Sched1<C: EngineCaps> {
    plan_dirty: <C::PlanAffecting as Capacity>::Array<AtomicBool>,
    worker_ctxs: <C::Cores as Capacity>::Array<WorkerCtx>,
    /// Nested 2-D replaces the flat MAX_CORES * GATE2_MAX_ACCUMS array.
    accum_live: <C::Cores as Capacity>::Array<<C::AccumsPerCore as Capacity>::Array<AtomicUsize>>,
    _caps: PhantomData<C>,
}

impl<C: EngineCaps> Sched1<C> {
    pub fn new() -> Self {
        Self {
            plan_dirty: <C::PlanAffecting as Capacity>::from_fn(|_| AtomicBool::new(false)),
            worker_ctxs: <C::Cores as Capacity>::from_fn(|_| WorkerCtx {
                sched: core::ptr::null(),
                core_id: 0,
            }),
            accum_live: <C::Cores as Capacity>::from_fn(|_| {
                <C::AccumsPerCore as Capacity>::from_fn(|_| AtomicUsize::new(0))
            }),
            _caps: PhantomData,
        }
    }

    pub fn mark_dirty(&self, i: usize) {
        self.plan_dirty.as_ref()[i].store(true, Ordering::Relaxed);
    }

    pub fn dirty_count(&self) -> usize {
        self.plan_dirty
            .as_ref()
            .iter()
            .filter(|b| b.load(Ordering::Relaxed))
            .count()
    }

    pub fn publish(&self, core: usize, accum: usize, v: usize) {
        self.accum_live.as_ref()[core].as_ref()[accum].store(v, Ordering::Relaxed);
    }

    pub fn read_publish(&self, core: usize, accum: usize) -> usize {
        self.accum_live.as_ref()[core].as_ref()[accum].load(Ordering::Relaxed)
    }

    pub fn core_budget(&self) -> usize {
        self.worker_ctxs.as_ref().len()
    }
}

// ----------------------------------------- probe 2: classify_cores lift

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CoreClass {
    P,
    E,
}

/// The thread/class.rs surface with the MAX_CORES return array lifted.
pub fn classify_cores_lifted<C: EngineCaps>(
    total_cores: USize,
) -> <C::Cores as Capacity>::Array<CoreClass> {
    let mut classes = <C::Cores as Capacity>::from_fn(|_| CoreClass::P);
    let budget = classes.as_ref().len();
    let count = core::cmp::min(total_cores.0, budget);
    // stand-in for detect_into: mark the tail of the populated range E
    for c in classes.as_mut()[..count].iter_mut().skip(count / 2) {
        *c = CoreClass::E;
    }
    classes
}

// ------------------------- probe 3: const-grouping scratch, CU-capacity

/// The const slice bridge ConstCapacity lacks today. A real lift adds
/// these two methods (or a sibling const trait) to arvo-tensor; for
/// Dim<N> each body is the built-in unsized coercion.
pub const trait CapSliceMut: [const] ConstCapacity {
    fn slice<T: Copy>(a: &Self::Array<T>) -> &[T];
    fn slice_mut<T: Copy>(a: &mut Self::Array<T>) -> &mut [T];
}

impl<const N: usize> const CapSliceMut for Dim<N> {
    fn slice<T: Copy>(a: &[T; N]) -> &[T] {
        a
    }
    fn slice_mut<T: Copy>(a: &mut [T; N]) -> &mut [T] {
        a
    }
}

/// Per-unit fixture: rank + the store bit the unit writes.
pub struct RankUnit<const R: usize, const W: usize>;

pub trait UnitFixture {
    const RANK: USize;
    const WRITES: USize;
}

impl<const R: usize, const W: usize> UnitFixture for RankUnit<R, W> {
    const RANK: USize = USize(R);
    const WRITES: USize = USize(W);
}

pub struct UNil;
pub struct UCons<H, T>(PhantomData<(H, T)>);

/// BundleMasks-shaped const fold: slice-taking (like the real
/// grouping.rs trait after the slice refactor), recursing with a
/// [const] bound on the tail.
pub const trait MiniMasks<CS: Capacity> {
    fn fill(writes: &mut [AccessMask<CS>], ranks: &mut [USize], idx: USize) -> USize;
}

impl<CS: Capacity> const MiniMasks<CS> for UNil {
    fn fill(_writes: &mut [AccessMask<CS>], _ranks: &mut [USize], idx: USize) -> USize {
        idx
    }
}

impl<H: UnitFixture, T: [const] MiniMasks<CS>, CS: Capacity> const MiniMasks<CS> for UCons<H, T> {
    fn fill(writes: &mut [AccessMask<CS>], ranks: &mut [USize], idx: USize) -> USize {
        let i = idx.0;
        writes[i] = AccessMask::empty().set(H::WRITES);
        ranks[i] = H::RANK;
        <T as MiniMasks<CS>>::fill(writes, ranks, USize(i + 1))
    }
}

/// Layer 1, the masks_of analog: scratch typed by the CU capacity
/// instead of [_; GATE2_MAX_UNITS]. No const expression in any array
/// length; the array type is the ConstCapacity GAT.
const fn masks_of_cap<Wus, CU, CS>() -> (
    <CU as ConstCapacity>::Array<AccessMask<CS>>,
    <CU as ConstCapacity>::Array<USize>,
    USize,
)
where
    Wus: [const] MiniMasks<CS>,
    CU: [const] CapSliceMut,
    CS: Capacity,
{
    let mut writes = <CU as ConstCapacity>::filled(AccessMask::empty());
    let mut ranks = <CU as ConstCapacity>::filled(USize::ZERO);
    let n = <Wus as MiniMasks<CS>>::fill(
        <CU as CapSliceMut>::slice_mut(&mut writes),
        <CU as CapSliceMut>::slice_mut(&mut ranks),
        USize::ZERO,
    );
    (writes, ranks, n)
}

/// Layer 2, the final_phases_of analog: re-calls layer 1 and walks the
/// capacity-typed scratch through ConstCapacity get/set.
const fn phases_of_cap<Wus, CU, CS>() -> (<CU as ConstCapacity>::Array<USize>, USize)
where
    Wus: [const] MiniMasks<CS>,
    CU: [const] CapSliceMut,
    CS: Capacity,
{
    let (_writes, ranks, n) = masks_of_cap::<Wus, CU, CS>();
    let mut phase = <CU as ConstCapacity>::filled(USize::ZERO);
    let mut i = 0;
    while i < n.0 {
        let r = <CU as ConstCapacity>::get(&ranks, USize(i));
        <CU as ConstCapacity>::set(&mut phase, USize(i), USize(r.0 * 2));
        i += 1;
    }
    (phase, n)
}

/// Layer 3, the phase_of analog the const-gated dispatch reads.
pub const fn phase_at_cap<Wus, CU, CS>(pos: USize) -> USize
where
    Wus: [const] MiniMasks<CS>,
    CU: [const] CapSliceMut,
    CS: Capacity,
{
    let (phase, _n) = phases_of_cap::<Wus, CU, CS>();
    <CU as ConstCapacity>::get(&phase, pos)
}

type Units8 = UCons<
    RankUnit<0, 1>,
    UCons<
        RankUnit<0, 2>,
        UCons<
            RankUnit<1, 3>,
            UCons<
                RankUnit<1, 4>,
                UCons<RankUnit<2, 5>, UCons<RankUnit<2, 6>, UCons<RankUnit<3, 7>, UNil>>>,
            >,
        >,
    >,
>;

/// Const-evaluated through the capacity-typed chain: the load-bearing
/// instantiation. CU is a consumer-picked Dim, not GATE2_MAX_UNITS.
const PHASE_AT_4: USize = phase_at_cap::<Units8, Dim<64>, Dim<64>>(USize(4));
const PHASE_AT_6: USize = phase_at_cap::<Units8, Dim<64>, Dim<64>>(USize(6));

// A second capacity at the same call shape, proving per-consumer reuse.
const PHASE_TINY: USize = phase_at_cap::<Units8, Dim<8>, Dim<64>>(USize(2));

// Depth stress, parallel to s2c/s4b: 64 units through the same chain.
macro_rules! unit_list {
    () => { UNil };
    ($r:literal $(, $rest:literal)*) => { UCons<RankUnit<$r, $r>, unit_list!($($rest),*)> };
}

type Units64 = unit_list!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
    49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
);

const PHASE_DEEP: USize = phase_at_cap::<Units64, Dim<128>, Dim<64>>(USize(63));

pub fn run() {
    let big: Sched1<DefaultCaps> = Sched1::new();
    let tiny: Sched1<TinyCaps> = Sched1::new();
    big.mark_dirty(7);
    big.mark_dirty(200);
    tiny.mark_dirty(3);
    big.publish(255, 15, 42);
    tiny.publish(3, 1, 7);
    println!(
        "s1 sched: default dirty={} cores={} publish={} | tiny dirty={} cores={} publish={}",
        big.dirty_count(),
        big.core_budget(),
        big.read_publish(255, 15),
        tiny.dirty_count(),
        tiny.core_budget(),
        tiny.read_publish(3, 1),
    );
    let classes = classify_cores_lifted::<TinyCaps>(USize(4));
    println!("s1 classify (Dim<4> budget): {:?}", classes.as_ref());
    println!(
        "s1 const grouping (CU-typed scratch): phase_at(4)={} phase_at(6)={} tiny phase_at(2)={} deep phase_at(63)={}",
        PHASE_AT_4.0, PHASE_AT_6.0, PHASE_TINY.0, PHASE_DEEP.0
    );
    assert_eq!(PHASE_AT_4.0, 4); // rank 2 doubled
    assert_eq!(PHASE_AT_6.0, 6); // rank 3 doubled
    assert_eq!(PHASE_TINY.0, 2); // rank 1 doubled
    assert_eq!(PHASE_DEEP.0, 126); // rank 63 doubled
    println!("s1: WORKS");
}
