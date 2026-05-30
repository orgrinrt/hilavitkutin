//! Strict tests for the bundle-to-`PlanInputs` projection's core: the
//! `MaskProject` access-set-to-bitmask fold (`project_access_set`).
//!
//! These exercise the projection logic directly over known store-marker
//! access sets, with the per-member index witness solver-inferred. The
//! per-work-unit bundle walk (`BundleProject` / `plan_inputs_from_bundle`)
//! sits on top of this same fold; its nested-witness inference is
//! validated by `mock/research/sketches/202605300626_plan-inputs-locate-witness/`
//! (single set) plus the committed bundle-walk scratch. A full
//! `plan_inputs_from_bundle` integration test needs reusable `WorkUnit`
//! fixtures (the `Ctx` GAT contract makes ad-hoc test units heavy); that
//! is tracked as a follow-up under the test-helper work (#81).

// `AccessMask<CS>` and the projection are now sized by the `Capacity` TYPE
// (`Dim<N>`), so no `generic_const_exprs` gate is needed: the store
// capacity is a type, not a `Cap` const generic.

use arvo::{Bool, USize};
use arvo_tensor::Dim;
use hilavitkutin::plan::project::project_access_set;
use hilavitkutin::plan::AccessMask;
use hilavitkutin_api::access::{Cons, Empty};

type Stores8 = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test store capacity; Dim<N> array-length root; tracked: #649

// Store marker types. Their position in `Stores` is their bit index.
struct SA;
struct SB;
struct SC;
struct SD;

// Global store list: SA=0, SB=1, SC=2, SD=3.
type Stores = Cons<SA, Cons<SB, Cons<SC, Cons<SD, Empty>>>>;

const B0: USize = USize(0); // lint:allow(no-bare-numeric) reason: store bit index; tracked: #121
const B1: USize = USize(1); // lint:allow(no-bare-numeric) reason: store bit index; tracked: #121
const B2: USize = USize(2); // lint:allow(no-bare-numeric) reason: store bit index; tracked: #121
const B3: USize = USize(3); // lint:allow(no-bare-numeric) reason: store bit index; tracked: #121

#[test]
fn projects_members_to_their_store_positions() {
    // Access set {SA, SC} -> bits 0 and 2; nothing else.
    type Set = Cons<SA, Cons<SC, Empty>>;
    let m: AccessMask<Stores8> = project_access_set::<Set, Stores, _, Stores8>();
    assert_eq!(m.contains(B0), Bool::TRUE, "SA at bit 0");
    assert_eq!(m.contains(B2), Bool::TRUE, "SC at bit 2");
    assert_eq!(m.contains(B1), Bool::FALSE, "SB not in the set");
    assert_eq!(m.contains(B3), Bool::FALSE, "SD not in the set");
}

#[test]
fn empty_access_set_projects_to_empty_mask() {
    let m: AccessMask<Stores8> = project_access_set::<Empty, Stores, _, Stores8>();
    assert_eq!(m.is_empty(), Bool::TRUE);
}

#[test]
fn same_store_lands_on_same_bit_across_sets() {
    // A write set {SB} and a read set {SB} must hit the SAME bit so the
    // plan stage sees the write/read overlap as a dependency edge.
    type WriteSet = Cons<SB, Empty>;
    type ReadSet = Cons<SB, Empty>;
    let w: AccessMask<Stores8> = project_access_set::<WriteSet, Stores, _, Stores8>();
    let r: AccessMask<Stores8> = project_access_set::<ReadSet, Stores, _, Stores8>();
    assert_eq!(w.contains(B1), Bool::TRUE);
    assert_eq!(r.contains(B1), Bool::TRUE);
    assert_eq!(w.overlaps(&r), Bool::TRUE, "shared store SB => overlap (dep edge)");
}

#[test]
fn distinct_sets_project_to_distinct_masks() {
    // Flip-verify: changing the set member changes the bit. {SD} sets
    // only bit 3; {SA} sets only bit 0; they do not overlap.
    type SetD = Cons<SD, Empty>;
    type SetA = Cons<SA, Empty>;
    let d: AccessMask<Stores8> = project_access_set::<SetD, Stores, _, Stores8>();
    let a: AccessMask<Stores8> = project_access_set::<SetA, Stores, _, Stores8>();
    assert_eq!(d.contains(B3), Bool::TRUE);
    assert_eq!(d.contains(B0), Bool::FALSE);
    assert_eq!(a.contains(B0), Bool::TRUE);
    assert_eq!(a.contains(B3), Bool::FALSE);
    assert_eq!(d.overlaps(&a), Bool::FALSE, "disjoint stores => no overlap");
}
