//! What the engine's plan makes of this frame, and what its parallel dispatch
//! then hands each core.
//!
//! `engine_par` ran at 1.08 cores of four on this host while `hand_par` ran at
//! 1.74, and the wall-clock medians alone say only that four cores bought
//! nothing. This test says why, over the real `Map` and `Join` types the
//! engine arm registers: it states the phase and trunk of every unit, and then
//! what each core is handed.
//!
//! The two halves rest on different ground and the difference matters. The
//! phase and trunk assertions call `grouping_arrays`, `phase_of` and
//! `trunk_of`, which are the functions the parallel dispatch itself uses
//! through `PhaseAt` and `IsRoot`, so those are read out of the plan. The
//! per-core assertions call `core_phase_mask`, and the dispatch does not call
//! it: `dispatch/trunk_dispatch.rs:246-253` spells the same `rank % ncores ==
//! core` rule inline over in-phase roots, and a comment there (`:252`) is the
//! only thing tying the two spellings together. So the per-core numbers below
//! are the reference rule's, and they are the dispatch's only for as long as
//! that comment stays true. Ticket `bc858e83ffbc` carries the duplication.
//!
//! The units are the support crate's own, so the access sets are the ones the
//! engine arm builds with, and so are their orders: `frame!` in the support
//! crate spells the columns and units once, the arm registers from it, and
//! `Stores` and `Units` here are that same spelling. What is written out here
//! is the per-unit projection witnesses, which the `Locate` bounds hold to the
//! units at their positions.
//!
//! Nothing here is a verdict on the engine. It records the shape the plan
//! currently finds, so a change to the waist axis or to trunk ownership shows
//! up as this test failing rather than as a timing nobody can explain. Ticket
//! `2e38f730c0e4` carries the finding, `5275557889c9` the measurement.

use arvo::USize;
use arvo_bitmask::BitAccess;
use frame_wide_support::Heavy;
use hilavitkutin::dispatch::core_mask::{core_phase_mask, grouping_arrays};
use hilavitkutin::dispatch::engine_ctx::{Here, There};
use hilavitkutin::plan::grouping::{GATE2_MAX_UNITS, group_n, phase_of, trunk_of};
use hilavitkutin::plan::{DefaultPlanDims, PlanDims};
use hilavitkutin_api::access::{Cons, Empty};

// The store and unit lists are the arm's own: `frame!` spells both once and
// the engine arm registers from that same spelling, so neither order is
// repeated here.
type Stores = frame_wide_support::engine::Stores;
type CS = <DefaultPlanDims as PlanDims>::Stores;
type CU = <DefaultPlanDims as PlanDims>::Units;
type Adj = <DefaultPlanDims as PlanDims>::AdjRow;

// The twelve units, in the order the engine arm registers them.
type Units = frame_wide_support::engine::Units<Heavy>;

// Store positions, 0 through 12.
type P0 = Here;
type P1 = There<P0>;
type P2 = There<P1>;
type P3 = There<P2>;
type P4 = There<P3>;
type P5 = There<P4>;
type P6 = There<P5>;
type P7 = There<P6>;
type P8 = There<P7>;
type P9 = There<P8>;
type P10 = There<P9>;
type P11 = There<P10>;
type P12 = There<P11>;

type One<P> = Cons<P, Empty>;

// Per-unit (ReadIdx, WriteIdx) witnesses, parallel to each unit's access sets.
type W0 = (One<P0>, One<P1>); //  R{In}        W{A1}
type W1 = (One<P1>, One<P2>); //  R{A1}        W{A2}
type W2 = (One<P2>, One<P3>); //  R{A2}        W{A3}
type W3 = (One<P0>, One<P4>); //  R{In}        W{B1}
type W4 = (One<P4>, One<P5>); //  R{B1}        W{B2}
type W5 = (One<P5>, One<P6>); //  R{B2}        W{B3}
type W6 = (One<P0>, One<P7>); //  R{In}        W{C1}
type W7 = (One<P7>, One<P8>); //  R{C1}        W{C2}
type W8 = (One<P8>, One<P9>); //  R{C2}        W{C3}
type W9 = (Cons<P3, Cons<P6, One<P9>>>, One<P10>); // R{A3,B3,C3} W{Jn}
type W10 = (One<P0>, One<P11>); // R{In}        W{T1}
type W11 = (One<P0>, One<P12>); // R{In}        W{T2}

type Witnesses = Cons<
    W0,
    Cons<
        W1,
        Cons<
            W2,
            Cons<
                W3,
                Cons<
                    W4,
                    Cons<W5, Cons<W6, Cons<W7, Cons<W8, Cons<W9, Cons<W10, Cons<W11, Empty>>>>>>>,
                >,
            >,
        >,
    >,
>;

const N_UNITS: usize = 12;

/// The grouping arrays the parallel dispatch computes on its first frame.
fn arrays() -> ([USize; GATE2_MAX_UNITS], [USize; GATE2_MAX_UNITS], usize) {
    let mut phase = [USize(0); GATE2_MAX_UNITS];
    let mut trunk = [USize(0); GATE2_MAX_UNITS];
    let n = grouping_arrays::<Units, Stores, Witnesses, CU, CS, Adj>(&mut phase, &mut trunk);
    (phase, trunk, n.0)
}

/// Twelve units group, and no two of them share a trunk by writing one
/// column.
///
/// This does not reach the builder, but it reads the same frame: `Stores` and
/// `Units` are the lists `frame!` hands the engine arm, so a column or unit
/// moved there moves here. The witnesses are written out by hand and held to
/// the units at their positions by the `Locate` bounds, so a witness naming the
/// wrong column, or a unit moved under its witness, does not compile.
#[test]
fn twelve_units_group_with_no_two_sharing_a_write() {
    assert_eq!(group_n::<Units, Stores, Witnesses, CU, CS>().0, N_UNITS);
    let (_phase, trunk, n) = arrays();
    assert_eq!(n, N_UNITS);
    // Each unit's write column is distinct, so no two units share a trunk by
    // writing the same column. A store order that had collapsed two columns
    // into one position would show here as a trunk joining units that write
    // different columns of the frame.
    assert_eq!(trunk[10].0, 10, "T1 writes a column nothing else touches");
    assert_eq!(trunk[11].0, 11, "T2 writes a column nothing else touches");
}

/// The whole frame lands in one phase.
///
/// One phase is what the design's own mechanism produces here, so this
/// asserts conformance rather than a snapshot.
///
/// The design puts waist detection in `arvo_graph::waist_detect`
/// (`mock/crates/hilavitkutin/DESIGN.md.tmpl:1052-1054`), and the engine calls
/// it: `plan/grouping.rs:277` runs `waist_detect_const` over the adjacency.
/// Arvo defines a waist as a depth whose level width is a strict local minimum
/// "relative to the occupied depths on either side" (`arvo-graph/src/waist.rs:22-25`),
/// and both variants scan the interior only, `k` from 1 while `k + 1 < occ_n`
/// (`waist.rs:98-111`, `waist_const.rs:100-111`). Arvo's own comment says why:
/// "A length-one or length-two occupied list has no interior; no minima are
/// emitted." The first and last occupied depths are not candidates at all.
///
/// Apply that to this frame. The occupied depths carry 5, 3, 3 and 1 units:
/// the five units reading the input, the three second stages, the three third
/// stages, and the join. The interior is depths 1 and 2. Depth 1 has width 3
/// against 5 and 3, so it is not strictly less than its successor. Depth 2 has
/// width 3 against 3 and 1, so it is not strictly less than its predecessor.
/// Neither qualifies, the terminal join is not eligible, and the twelve units
/// share phase 0. Every step follows from the cited rule.
///
/// So the engine is not misreading this frame. The question the occupancy
/// finding actually raises is a design one: whether a terminal fan-in, which
/// is the only narrowing this shape has, should close a phase even though it
/// cannot be an interior local minimum. The design does not say, and this test
/// does not decide it. Question ticket `ecbd88ff1a6a` puts it to op. If the
/// answer is yes, the waist axis changes and this test is expected to fail,
/// which is the point of writing the rule down here rather than the number.
#[test]
fn the_frame_is_one_phase() {
    let (phase, _trunk, n) = arrays();
    for u in 0 .. n {
        assert_eq!(
            phase_of::<Units, Stores, Witnesses, CU, CS, Adj>(USize(u)).0,
            0,
            "unit {u} is not in phase 0"
        );
        assert_eq!(phase[u].0, 0, "unit {u} is not in phase 0");
    }
}

/// One phase means the join's fan-in merges the three pipelines into one
/// trunk of ten units, beside two trunks of one unit each.
///
/// A trunk is a within-phase component over column conflict, canonicalised to
/// its smallest member position. The three pipelines are column-disjoint from
/// each other, so in a phase of their own they would be three trunks. The join
/// reads all three tails, so it conflicts with each of them, and with every
/// phase being one phase the component closes over all ten units. `T1` and
/// `T2` read only the input, which nothing in the frame writes, and each
/// writes a column nothing else reads, so each is a trunk of its own.
#[test]
fn the_join_merges_three_pipelines_into_one_trunk_of_ten() {
    let (phase, trunk, n) = arrays();
    let expected = [0usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11];
    for u in 0 .. n {
        assert_eq!(
            trunk_of::<Units, Stores, Witnesses, CU, CS, Adj>(USize(u)).0,
            expected[u],
            "unit {u} is in the wrong trunk"
        );
        assert_eq!(trunk[u].0, expected[u], "unit {u} is in the wrong trunk");
    }
    // A trunk root is a unit that is its own trunk: the grouping names each
    // trunk by its least unit. The engine had a helper for this count and
    // dropped it with the convergence path that used it, so it is counted
    // here from the arrays the test already holds.
    let roots_in_phase_0 = (0 .. n)
        .filter(|&u| phase[u].0 == 0 && trunk[u].0 == u)
        .count();
    assert_eq!(roots_in_phase_0, 3, "phase 0 carries three trunks");
}

/// What each of four cores is handed, which is the finding in one assertion.
///
/// Trunk-to-core ownership is per-phase round-robin over the within-phase
/// trunk rank. Three trunks over four cores gives core 0 the trunk of ten,
/// core 1 `T1`, core 2 `T2` and core 3 nothing. The single-trunk head+tail
/// record split does not apply, because `run_core_phase` takes it only when
/// the phase carries exactly one trunk, and this one carries three.
///
/// Ten of the twelve units on one core is the 1.08 cores the process-wide
/// instrument measured. The record count does not change it: ownership is by
/// trunk, and a morsel loop over 65536 records at the default morsel size runs
/// 256 morsels, every one of them firing the same one core's trunks.
#[test]
fn four_cores_split_ten_one_one_and_nothing() {
    let (phase, trunk, n) = arrays();
    let ncores = 4usize;
    let mut owned = [0usize; 4];
    for core in 0 .. ncores {
        let mask: Adj = core_phase_mask::<Adj>(
            &phase,
            &trunk,
            USize(n),
            USize(core),
            USize(0),
            USize(ncores),
        );
        for u in 0 .. n {
            if mask.bit(USize(u)).0 {
                owned[core] += 1;
            }
        }
    }
    assert_eq!(owned, [10, 1, 1, 0], "the per-core unit counts");
    assert_eq!(
        owned.iter().sum::<usize>(),
        n,
        "every unit is owned by exactly one core"
    );
}

/// No worker count spreads this frame, so the bench cannot configure its way
/// out of the finding.
///
/// The ten-unit trunk is owned whole by one core at every count, because a
/// trunk is the unit of ownership and nothing below it is. Past three cores
/// the surplus cores own nothing at all. Under three, a core picks up a second
/// trunk and the busiest core gets worse rather than better.
#[test]
fn no_worker_count_spreads_the_ten_unit_trunk() {
    let (phase, trunk, n) = arrays();
    // Busiest core's unit count at each worker count from one to eight.
    let expected = [12usize, 11, 10, 10, 10, 10, 10, 10];
    for (i, want) in expected.iter().enumerate() {
        let ncores = i + 1;
        let mut busiest = 0usize;
        for core in 0 .. ncores {
            let mask: Adj = core_phase_mask::<Adj>(
                &phase,
                &trunk,
                USize(n),
                USize(core),
                USize(0),
                USize(ncores),
            );
            let mut owned = 0usize;
            for u in 0 .. n {
                if mask.bit(USize(u)).0 {
                    owned += 1;
                }
            }
            busiest = busiest.max(owned);
        }
        assert_eq!(busiest, *want, "busiest core at {ncores} workers");
    }
}

/// The control for `four_cores_split_ten_one_one_and_nothing`: with one core,
/// that core owns every unit.
///
/// Without it, a `core_phase_mask` that returned an empty mask for every core
/// would leave `[10, 1, 1, 0]` looking like a finding about this frame rather
/// than a defect in the instrument.
#[test]
fn one_core_owns_every_unit() {
    let (phase, trunk, n) = arrays();
    let mask: Adj = core_phase_mask::<Adj>(&phase, &trunk, USize(n), USize(0), USize(0), USize(1));
    let mut owned = 0usize;
    for u in 0 .. n {
        if mask.bit(USize(u)).0 {
            owned += 1;
        }
    }
    assert_eq!(owned, n);
}
