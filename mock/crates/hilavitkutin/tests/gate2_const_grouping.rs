//! GATE-2 const grouping: compile-time WAIST-BOUNDED phase + trunk from a
//! registered bundle.
//!
//! Round 202606070839 (R2). The grouping is a pure function of the registered
//! access sets: per-unit read/write store-bit masks (via the engine's const
//! `MaskProject`) form the read-after-write unit DAG, whose waist-delimited
//! sections are the phases (via arvo's const `waist_detect_const`); within a
//! phase, column-disjoint write components are the trunks. The canonical phase
//! axis is WAIST-bounded, not longest-depth (course-correction
//! `202606071400_gate2-phase-axis-course-correction.md`): a producer to consumer
//! chain with no interior narrowing is one phase.
//!
//! The fixture is a topo-valid hourglass where the waist-bounded phase
//! `[0,0,0,0,1,1]` differs from the longest-depth axis `[0,1,1,2,3,3]`, so the
//! test proves the axis is genuinely waist-bounded, not depth.

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::USize;
use hilavitkutin::plan::grouping::{group_n, phase_of, trunk_of, UnitAccess};
use hilavitkutin::plan::{DefaultPlanDims, PlanDims};
use hilavitkutin::dispatch::engine_ctx::{Here, There};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::store::Column;

// Stores: [A, B, C, D, E, F] at positions 0..6.
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
type Adj = <DefaultPlanDims as PlanDims>::AdjRow;

// Hourglass DAG. Registration order U0..U5 is a valid topo order (every producer
// precedes its consumers).
//   U0: write A
//   U1: read A, write B     U2: read A, write C     (parallel)
//   U3: read B,C, write D                            (the waist: wide -> 1 -> wide)
//   U4: read D, write E     U5: read D, write F      (parallel)
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

// Store positions.
type P0 = Here;
type P1 = There<Here>;
type P2 = There<There<Here>>;
type P3 = There<There<There<Here>>>;
type P4 = There<There<There<There<Here>>>>;
type P5 = There<There<There<There<There<Here>>>>>;

// Per-unit (ReadIdx, WriteIdx) witness lists, parallel to each unit's sets.
type W0 = (Empty, Cons<P0, Empty>); // R{} W{A@0}
type W1 = (Cons<P0, Empty>, Cons<P1, Empty>); // R{A@0} W{B@1}
type W2 = (Cons<P0, Empty>, Cons<P2, Empty>); // R{A@0} W{C@2}
type W3 = (Cons<P1, Cons<P2, Empty>>, Cons<P3, Empty>); // R{B@1,C@2} W{D@3}
type W4 = (Cons<P3, Empty>, Cons<P4, Empty>); // R{D@3} W{E@4}
type W5 = (Cons<P3, Empty>, Cons<P5, Empty>); // R{D@3} W{F@5}
type Witnesses = Cons<W0, Cons<W1, Cons<W2, Cons<W3, Cons<W4, Cons<W5, Empty>>>>>>;

#[test]
fn hourglass_waist_bounded_grouping() {
    // N = 6 units registered.
    assert_eq!(group_n::<Units, Stores, Witnesses, CU, CS>().0, 6);

    // WAIST-bounded phase: U3 (read B,C -> write D) is the lone waist (the
    // wide->1->wide narrowing), at topo position 3, so it is the last of phase 0;
    // U4/U5 open phase 1. This is NOT the longest-depth axis [0,1,1,2,3,3].
    let phase = [0usize, 0, 0, 0, 1, 1];
    let mut i = 0;
    while i < 6 {
        assert_eq!(
            phase_of::<Units, Stores, Witnesses, CU, CS, Adj>(USize(i)).0,
            phase[i],
            "phase[{i}]"
        );
        i += 1;
    }

    // TRUNK = component-min within phase. Phase 0 {U0,U1,U2,U3} is one component
    // (A->{B,C}->D all conflict on a shared written column), id = min pos 0.
    // Phase 1: U4 (write E) and U5 (write F) are column-disjoint (both only READ
    // D), so distinct trunks, each its own min pos.
    let trunk = [0usize, 0, 0, 0, 4, 5];
    let mut j = 0;
    while j < 6 {
        assert_eq!(
            trunk_of::<Units, Stores, Witnesses, CU, CS, Adj>(USize(j)).0,
            trunk[j],
            "trunk[{j}]"
        );
        j += 1;
    }
}
