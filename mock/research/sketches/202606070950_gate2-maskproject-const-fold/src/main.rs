//! GATE-2 round-1 de-risk: the real const MaskProject fold (gap a).
//!
//! Sketch 070800 proved a const-trait fold yields a const-evaluable grouping that
//! DCEs the gate, using a hardcoded per-unit mask. This swaps the hardcode for the
//! engine's real `MaskProject` const fold (`plan::project`) over real access-set
//! types and the global `Stores` numbering (`Locate` + `WitnessIndex`), with the
//! witness-index list threaded as a type parameter (the shape `compute_plan`
//! threads `BWit` / `run` threads `Witnesses`).
//!
//! Q: does a const-trait fold that calls `MaskProject::project_mask` (a const trait
//!    METHOD, with the witness list threaded) stay const-evaluable for a
//!    `const { trunk_of::<...>(POS) == TRUNK }` gate? Outcome at the bottom.

#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::marker::PhantomData;

use arvo_tensor::Capacity;
use hilavitkutin::dispatch::engine_ctx::{Here, There};
use hilavitkutin::plan::project::MaskProject;
use hilavitkutin::plan::{AccessMask, DefaultPlanDims, PlanDims};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::store::Column;

// Store markers and the global Stores access set: [X, Y, Z, V, Wt].
struct X;
struct Y;
struct Z;
struct V;
struct Wt;
type Stores = Cons<Column<X>, Cons<Column<Y>, Cons<Column<Z>, Cons<Column<V>, Cons<Column<Wt>, Empty>>>>>;

// Store capacity (matches the engine: the AccessMask backing width).
type CS = <DefaultPlanDims as PlanDims>::Stores;

// A "unit" names its Read and Write access sets (stand-in for W::Read / W::Write).
trait UnitSets {
    type Read;
    type Write;
}
struct U<R, W>(PhantomData<(R, W)>);
impl<R, W> UnitSets for U<R, W> {
    type Read = R;
    type Write = W;
}

// read/write set type aliases.
type SetX = Cons<Column<X>, Empty>;
type SetY = Cons<Column<Y>, Empty>;
type SetZ = Cons<Column<Z>, Empty>;
type SetV = Cons<Column<V>, Empty>;
type SetWt = Cons<Column<Wt>, Empty>;

// U0: R{X@0} W{Y@1}; U1: R{Z@2} W{V@3}; U2: R{Y@1} W{Wt@4}.
// U2 reads Y (U0's write) -> RAW edge -> same trunk; U1 independent.
type U0 = U<SetX, SetY>;
type U1 = U<SetZ, SetV>;
type U2 = U<SetY, SetWt>;
type Units = Cons<U0, Cons<U1, Cons<U2, Empty>>>;

// Witness index lists (the Stores position of each member), one per access set.
// Each set has one member, so each is a one-element witness list.
type AtX = Cons<Here, Empty>; // X @ 0
type AtY = Cons<There<Here>, Empty>; // Y @ 1
type AtZ = Cons<There<There<Here>>, Empty>; // Z @ 2
type AtV = Cons<There<There<There<Here>>>, Empty>; // V @ 3
type AtWt = Cons<There<There<There<There<Here>>>>, Empty>; // Wt @ 4

// Per-unit (ReadIdx, WriteIdx) pairs, threaded as the witness list (the BWit shape).
type Witnesses = Cons<(AtX, AtY), Cons<(AtZ, AtV), Cons<(AtY, AtWt), Empty>>>;

const CAP: usize = 8;

// Const fold: fill per-unit read/write masks via the real const MaskProject.
const trait ConstMasks<Stores, Witnesses, CSc: Capacity> {
    fn fill(reads: &mut [u64; CAP], writes: &mut [u64; CAP], idx: usize) -> usize;
}

impl<Stores, CSc: Capacity> const ConstMasks<Stores, Empty, CSc> for Empty {
    fn fill(_reads: &mut [u64; CAP], _writes: &mut [u64; CAP], idx: usize) -> usize {
        idx
    }
}

impl<Stores, Un, T, RI, WI, WT, CSc: Capacity> const ConstMasks<Stores, Cons<(RI, WI), WT>, CSc>
    for Cons<Un, T>
where
    Un: UnitSets,
    Un::Read: [const] MaskProject<Stores, RI, CSc>,
    Un::Write: [const] MaskProject<Stores, WI, CSc>,
    T: [const] ConstMasks<Stores, WT, CSc>,
{
    fn fill(reads: &mut [u64; CAP], writes: &mut [u64; CAP], idx: usize) -> usize {
        let r = <Un::Read as MaskProject<Stores, RI, CSc>>::project_mask(AccessMask::empty());
        let w = <Un::Write as MaskProject<Stores, WI, CSc>>::project_mask(AccessMask::empty());
        reads[idx] = r.raw().0 as u64;
        writes[idx] = w.raw().0 as u64;
        <T as ConstMasks<Stores, WT, CSc>>::fill(reads, writes, idx + 1)
    }
}

// Const grouping: union-find over shared columns (proven shape, sketch 070800).
const fn compute_trunks(reads: [u64; CAP], writes: [u64; CAP], n: usize) -> [u64; CAP] {
    let mut parent = [0usize; CAP];
    let mut i = 0;
    while i < CAP {
        parent[i] = i;
        i += 1;
    }
    let mut a = 0;
    while a < n {
        let mut b = a + 1;
        while b < n {
            if ((reads[a] | writes[a]) & (reads[b] | writes[b])) != 0 {
                let mut ra = a;
                while parent[ra] != ra {
                    ra = parent[ra];
                }
                let mut rb = b;
                while parent[rb] != rb {
                    rb = parent[rb];
                }
                if ra != rb {
                    parent[rb] = ra;
                }
            }
            b += 1;
        }
        a += 1;
    }
    let mut trunk = [0u64; CAP];
    let mut k = 0;
    while k < CAP {
        let mut r = k;
        while parent[r] != r {
            r = parent[r];
        }
        trunk[k] = r as u64;
        k += 1;
    }
    trunk
}

// Grouping carrier: associated const over the unit list + threaded witnesses.
trait Grouped<Stores, Witnesses, CSc: Capacity> {
    const N: usize;
    const GROUPING: [u64; CAP];
}

impl<U, Stores, Witnesses, CSc: Capacity> Grouped<Stores, Witnesses, CSc> for U
where
    U: const ConstMasks<Stores, Witnesses, CSc>,
{
    const N: usize = {
        let mut r = [0u64; CAP];
        let mut w = [0u64; CAP];
        <U as ConstMasks<Stores, Witnesses, CSc>>::fill(&mut r, &mut w, 0)
    };
    const GROUPING: [u64; CAP] = {
        let mut r = [0u64; CAP];
        let mut w = [0u64; CAP];
        let n = <U as ConstMasks<Stores, Witnesses, CSc>>::fill(&mut r, &mut w, 0);
        compute_trunks(r, w, n)
    };
}

const fn trunk_of<U, Stores, Witnesses, CSc: Capacity>(pos: usize) -> u64
where
    U: Grouped<Stores, Witnesses, CSc>,
{
    <U as Grouped<Stores, Witnesses, CSc>>::GROUPING[pos]
}

fn main() {
    let n = <Units as Grouped<Stores, Witnesses, CS>>::N;
    let grouping = <Units as Grouped<Stores, Witnesses, CS>>::GROUPING;

    assert_eq!(n, 3);
    // Masks via real MaskProject: READ=[1,4,2], WRITE=[2,8,16] (sketch 071330 values).
    // Grouping: U0,U2 share Y (bit 1) -> same trunk; U1 alone.
    assert_eq!(grouping[0], grouping[2]);
    assert_ne!(grouping[0], grouping[1]);

    // const gate over the grouping (the DCE driver).
    const T0: u64 = trunk_of::<Units, Stores, Witnesses, CS>(0);
    const T1: u64 = trunk_of::<Units, Stores, Witnesses, CS>(1);
    const T2: u64 = trunk_of::<Units, Stores, Witnesses, CS>(2);
    assert_eq!(T0, T2);
    assert_ne!(T0, T1);

    println!("WORKS: N={n}, GROUPING(0..3)={:?}, T0={T0} T1={T1} T2={T2}", &grouping[..3]);
}

// ---------------------------------------------------------------------------
// OUTCOME (2026-06-07): WORKS. Gap (a) closed against real engine types.
//
// The engine's real const `MaskProject::project_mask` (a const trait METHOD)
// composes inside a NEW const-trait fold (`ConstMasks`, `impl const`, `[const]`
// tail bound), with the witness-index list threaded as a type parameter exactly
// as `compute_plan` threads `BWit` and `run` threads `Witnesses`. The per-unit
// masks computed via the real fold match the hardcoded sketch-071330 values
// (READ=[1,4,2], WRITE=[2,8,16]); the const grouping is GROUPING=[0,1,0]
// (U0,U2 share Y -> trunk 0, U1 -> trunk 1); the `const { trunk_of(POS) }` gate
// const-evaluates (T0==T2==0, T1==1).
//
// Features: const_trait_impl + generic_const_exprs (both WATCH-allowed, already
// enabled in the engine crates). The `Grouped` impl bound is `const ConstMasks`
// (not `[const]`) because an associated-const initializer is always-const context.
//
// src CL for round 202606070900 is now on proven ground: a `plan/grouping.rs`
// const-trait fold reusing `MaskProject` filling a fixed-cap (or Capacity::Array)
// mask array, the const-fn grouping over it, exposed as an associated const keyed
// by (Wus, Stores, Witnesses) so the later const-gated walk indexes it. The
// witness list threads from build()/run() (the BWit/Witnesses param), concrete at
// the gate.
// ---------------------------------------------------------------------------
