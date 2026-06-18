//! GATE-2 R4a: per-(core, phase) unit masks for the N-core runtime-mask dispatch.
//!
//! `core_phase_mask` is the pure builder a worker uses to select the units it
//! runs in a given phase: bit `u` set iff `phase[u] == target_phase` and the
//! within-phase rank of `trunk[u]` modulo `ncores` equals this core. Round-robin
//! per-phase trunk distribution (the simplest correct policy; load-balanced is a
//! later perf fork). `grouping_arrays` fills the per-unit phase/trunk arrays from
//! the shipped R2 const grouping.
//!
//! Red first: `dispatch::core_mask` does not exist, so the file does not compile
//! until the module lands.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Bits, Hot, USize, Unsigned};
use arvo_bitmask::BitAccess;
use hilavitkutin::dispatch::core_mask::{core_phase_mask, grouping_arrays};
use hilavitkutin::plan::grouping::UnitAccess;
use hilavitkutin::plan::{DefaultPlanDims, PlanDims};
use hilavitkutin::dispatch::engine_ctx::{Here, There};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::store::Column;

type Adj = <DefaultPlanDims as PlanDims>::AdjRow;

// Collect the set bits of an AdjRow mask into a sorted Vec for assertions.
fn bits(m: Adj, n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut u = 0;
    while u < n {
        if m.bit(USize(u)).0 {
            v.push(u);
        }
        u += 1;
    }
    v
}

// The R2 hourglass grouping output: phase [0,0,0,0,1,1], trunk component-mins
// [0,0,0,0,4,5] (U0..U3 one phase-0 trunk; U4 trunk 4, U5 trunk 5 in phase 1).
const PHASE: [USize; 6] = [USize(0), USize(0), USize(0), USize(0), USize(1), USize(1)];
const TRUNK: [USize; 6] = [USize(0), USize(0), USize(0), USize(0), USize(4), USize(5)];
const N: USize = USize(6);

#[test]
fn two_core_masks_select_the_right_units() {
    // Phase 0 has one trunk (0, rank 0): core 0 owns it, core 1 idle.
    assert_eq!(bits(core_phase_mask::<Adj>(&PHASE, &TRUNK, N, USize(0), USize(0), USize(2)), 6), [0, 1, 2, 3]);
    assert_eq!(bits(core_phase_mask::<Adj>(&PHASE, &TRUNK, N, USize(1), USize(0), USize(2)), 6), [] as [usize; 0]);
    // Phase 1 has two trunks (4 rank 0 -> core 0, 5 rank 1 -> core 1).
    assert_eq!(bits(core_phase_mask::<Adj>(&PHASE, &TRUNK, N, USize(0), USize(1), USize(2)), 6), [4]);
    assert_eq!(bits(core_phase_mask::<Adj>(&PHASE, &TRUNK, N, USize(1), USize(1), USize(2)), 6), [5]);
}

#[test]
fn every_unit_dispatched_exactly_once_across_cores_and_phases() {
    let ncores = 2usize;
    let nphases = 2usize;
    let mut seen = [0u32; 6];
    let mut c = 0;
    while c < ncores {
        let mut p = 0;
        while p < nphases {
            for u in bits(core_phase_mask::<Adj>(&PHASE, &TRUNK, N, USize(c), USize(p), USize(ncores)), 6) {
                seen[u] += 1;
            }
            p += 1;
        }
        c += 1;
    }
    assert_eq!(seen, [1, 1, 1, 1, 1, 1], "each unit dispatched exactly once across the per-core programs");
}

#[test]
fn single_core_owns_all_trunks_per_phase() {
    // ncores = 1: every rank % 1 == 0 == core 0, so core 0 runs all of each phase.
    assert_eq!(bits(core_phase_mask::<Adj>(&PHASE, &TRUNK, N, USize(0), USize(0), USize(1)), 6), [0, 1, 2, 3]);
    assert_eq!(bits(core_phase_mask::<Adj>(&PHASE, &TRUNK, N, USize(0), USize(1), USize(1)), 6), [4, 5]);
}

// --- grouping_arrays over the real R2 const grouping (hourglass fixture) ---

struct A;
struct B;
struct C;
struct D;
struct E;
struct F;
type Stores = Cons<
    Column<A>,
    Cons<Column<B>, Cons<Column<C>, Cons<Column<D>, Cons<Column<E>, Cons<Column<F>, Empty>>>>>,
>;
type CS = <DefaultPlanDims as PlanDims>::Stores;
type CU = <DefaultPlanDims as PlanDims>::Units;

struct U0;
struct U1;
struct U2;
struct U3;
struct U4;
struct U5;
type SA = Cons<Column<A>, Empty>;
type SB = Cons<Column<B>, Empty>;
type SC = Cons<Column<C>, Empty>;
type SD = Cons<Column<D>, Empty>;
type SE = Cons<Column<E>, Empty>;
type SF = Cons<Column<F>, Empty>;
type SBC = Cons<Column<B>, Cons<Column<C>, Empty>>;
impl UnitAccess for U0 {
    type Read = Empty;
    type Write = SA;
}
impl UnitAccess for U1 {
    type Read = SA;
    type Write = SB;
}
impl UnitAccess for U2 {
    type Read = SA;
    type Write = SC;
}
impl UnitAccess for U3 {
    type Read = SBC;
    type Write = SD;
}
impl UnitAccess for U4 {
    type Read = SD;
    type Write = SE;
}
impl UnitAccess for U5 {
    type Read = SD;
    type Write = SF;
}
type Units = Cons<U0, Cons<U1, Cons<U2, Cons<U3, Cons<U4, Cons<U5, Empty>>>>>>;

type P0 = Here;
type P1 = There<Here>;
type P2 = There<There<Here>>;
type P3 = There<There<There<Here>>>;
type P4 = There<There<There<There<Here>>>>;
type P5 = There<There<There<There<There<Here>>>>>;
type W0 = (Empty, Cons<P0, Empty>);
type W1 = (Cons<P0, Empty>, Cons<P1, Empty>);
type W2 = (Cons<P0, Empty>, Cons<P2, Empty>);
type W3 = (Cons<P1, Cons<P2, Empty>>, Cons<P3, Empty>);
type W4 = (Cons<P3, Empty>, Cons<P4, Empty>);
type W5 = (Cons<P3, Empty>, Cons<P5, Empty>);
type Witnesses = Cons<W0, Cons<W1, Cons<W2, Cons<W3, Cons<W4, Cons<W5, Empty>>>>>>;

#[test]
fn grouping_arrays_reproduces_the_r2_hourglass() {
    let mut phase = [USize(0); 6];
    let mut trunk = [USize(0); 6];
    let n = grouping_arrays::<Units, Stores, Witnesses, CU, CS, Adj>(&mut phase, &mut trunk);
    assert_eq!(n.0, 6);
    assert_eq!(phase.map(|p| p.0), [0, 0, 0, 0, 1, 1], "waist-bounded phase");
    assert_eq!(trunk.map(|t| t.0), [0, 0, 0, 0, 4, 5], "within-phase trunk component-mins");
}
