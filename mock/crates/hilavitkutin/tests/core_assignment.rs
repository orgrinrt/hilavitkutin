//! CoreAssignment + Convergence tests (5a4 skeleton).
//!
//! `CoreAssignment` is now sized by the `Capacity` TYPE (`Dim<N>`), so no
//! `generic_const_exprs` gate is needed: the core capacity is a type, not a
//! `Cap` const generic, and the backing arrays are plain `[T; N]`.

use arvo::{Identity, USize};
use arvo_tensor::Dim;
use hilavitkutin::plan::FiberId;
use hilavitkutin::thread::{Convergence, CoreAssignment, NO_TRUNK, ThreadHandle};

type C4 = Dim<4>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649
type C8 = Dim<8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test capacity dimension; Dim<N> array-length root; tracked: #649

#[test]
fn core_assignment_new_is_empty() {
    let a: CoreAssignment<C8> = CoreAssignment::new();
    assert_eq!(a.assigned_count, USize::ZERO);
    for slot in a.trunk_index.as_ref().iter() {
        assert_eq!(*slot, NO_TRUNK);
    }
    for m in a.morsel_size_multiplier.as_ref().iter() {
        assert_eq!(*m, USize(100)); // lint:allow(no-bare-numeric) reason: default morsel-size-multiplier value; tracked: #399
    }
    for f in a.fiber_assignments.as_ref().iter() {
        assert_eq!(*f, FiberId::ZERO);
    }
}

#[test]
fn core_assignment_default_matches_new() {
    let a: CoreAssignment<C4> = CoreAssignment::default();
    let b: CoreAssignment<C4> = CoreAssignment::new();
    assert_eq!(a.assigned_count, b.assigned_count);
    assert_eq!(a.trunk_index.as_ref(), b.trunk_index.as_ref());
    assert_eq!(a.morsel_size_multiplier.as_ref(), b.morsel_size_multiplier.as_ref());
    assert_eq!(a.fiber_assignments.as_ref(), b.fiber_assignments.as_ref());
}

#[test]
fn core_assignment_per_core_slot_mutation_roundtrips() {
    let mut a: CoreAssignment<C4> = CoreAssignment::new();
    a.trunk_index.as_mut()[0] = USize(2); // lint:allow(no-bare-numeric) reason: trunk index value; tracked: #399
    a.fiber_assignments.as_mut()[0] = FiberId::from_constant::<{ USize(5) }>(); // lint:allow(no-bare-numeric) reason: fiber id value; tracked: #426
    a.morsel_size_multiplier.as_mut()[0] = USize(200); // lint:allow(no-bare-numeric) reason: multiplier value; tracked: #399
    a.assigned_count = USize(1); // lint:allow(no-bare-numeric) reason: assigned count value; tracked: #399
    assert_eq!(a.trunk_index.as_ref()[0], USize(2)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(a.fiber_assignments.as_ref()[0], FiberId::from_constant::<{ USize(5) }>()); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #426
    assert_eq!(a.morsel_size_multiplier.as_ref()[0], USize(200)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(a.assigned_count, USize(1)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    // Untouched slots still defaulted.
    assert_eq!(a.trunk_index.as_ref()[1], NO_TRUNK);
    assert_eq!(a.morsel_size_multiplier.as_ref()[3], USize(100)); // lint:allow(no-bare-numeric) reason: default value check; tracked: #399
}

#[test]
fn convergence_new_records_threads_and_zero_counter() {
    let c = Convergence::new(ThreadHandle(USize(3)), ThreadHandle(USize(7))); // lint:allow(no-bare-numeric) reason: thread handle ids; tracked: #399
    assert_eq!(c.head_thread, ThreadHandle(USize(3))); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(c.tail_thread, ThreadHandle(USize(7))); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(c.meeting_record.load(), USize::ZERO);
}
