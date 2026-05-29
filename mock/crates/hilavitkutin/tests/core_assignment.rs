//! CoreAssignment + Convergence tests (5a4 skeleton).

// The lifted Cap-dimension types carry `[(); cap_size(N)]:` bounds. A
// downstream crate that instantiates them must itself enable generic_const_exprs
// so its own trait solver can normalise the bounds, mirroring arvo's cross-crate
// tests. adt_const_params is needed only where a Cap const-generic param is
// declared (the engine crate root), not where a lifted type is named. WATCH-tier
// per the unstable-feature soundness sweep (#626).
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use arvo::{Cap, Identity, USize};
use arvo_tensor::cap;
use hilavitkutin::plan::FiberId;
use hilavitkutin::thread::{Convergence, CoreAssignment, NO_TRUNK, ThreadHandle};

const C4: Cap = cap(4); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121
const C8: Cap = cap(8); // lint:allow(no-bare-numeric) reason: test fixture dimension; tracked: #121

#[test]
fn core_assignment_new_is_empty() {
    let a: CoreAssignment<C8> = CoreAssignment::new();
    assert_eq!(a.assigned_count, USize::ZERO);
    for slot in a.trunk_index.iter() {
        assert_eq!(*slot, NO_TRUNK);
    }
    for m in a.morsel_size_multiplier.iter() {
        assert_eq!(*m, USize(100)); // lint:allow(no-bare-numeric) reason: default morsel-size-multiplier value; tracked: #399
    }
    for f in a.fiber_assignments.iter() {
        assert_eq!(*f, FiberId::ZERO);
    }
}

#[test]
fn core_assignment_default_matches_new() {
    let a: CoreAssignment<C4> = CoreAssignment::default();
    let b: CoreAssignment<C4> = CoreAssignment::new();
    assert_eq!(a.assigned_count, b.assigned_count);
    assert_eq!(a.trunk_index, b.trunk_index);
    assert_eq!(a.morsel_size_multiplier, b.morsel_size_multiplier);
    assert_eq!(a.fiber_assignments, b.fiber_assignments);
}

#[test]
fn core_assignment_per_core_slot_mutation_roundtrips() {
    let mut a: CoreAssignment<C4> = CoreAssignment::new();
    a.trunk_index[0] = USize(2); // lint:allow(no-bare-numeric) reason: trunk index value; tracked: #399
    a.fiber_assignments[0] = FiberId::from_constant::<{ USize(5) }>(); // lint:allow(no-bare-numeric) reason: fiber id value; tracked: #426
    a.morsel_size_multiplier[0] = USize(200); // lint:allow(no-bare-numeric) reason: multiplier value; tracked: #399
    a.assigned_count = USize(1); // lint:allow(no-bare-numeric) reason: assigned count value; tracked: #399
    assert_eq!(a.trunk_index[0], USize(2)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(a.fiber_assignments[0], FiberId::from_constant::<{ USize(5) }>()); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #426
    assert_eq!(a.morsel_size_multiplier[0], USize(200)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(a.assigned_count, USize(1)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    // Untouched slots still defaulted.
    assert_eq!(a.trunk_index[1], NO_TRUNK);
    assert_eq!(a.morsel_size_multiplier[3], USize(100)); // lint:allow(no-bare-numeric) reason: default value check; tracked: #399
}

#[test]
fn convergence_new_records_threads_and_zero_counter() {
    let c = Convergence::new(ThreadHandle(USize(3)), ThreadHandle(USize(7))); // lint:allow(no-bare-numeric) reason: thread handle ids; tracked: #399
    assert_eq!(c.head_thread, ThreadHandle(USize(3))); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(c.tail_thread, ThreadHandle(USize(7))); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(c.meeting_record.load(), USize::ZERO);
}
