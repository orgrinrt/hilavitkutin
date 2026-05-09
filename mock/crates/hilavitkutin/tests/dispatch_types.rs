//! Dispatch-stage type surface tests (5a3 skeleton).

use arvo::{Bool, Identity, USize};
use hilavitkutin::dispatch::{
    CoreDispatch, DispatchApproach, FiberDispatch, MorselRange, ProgressCounter, SyncPoint,
};
use hilavitkutin::plan::FiberId;

#[derive(Default)]
struct StubCtx;

#[test]
fn progress_counter_store_load_round_trip() {
    let c = ProgressCounter::new(USize::ZERO);
    assert_eq!(c.load(), USize::ZERO);
    c.store(USize(42)); // lint:allow(no-bare-numeric) reason: progress counter literal; tracked: #399
    assert_eq!(c.load(), USize(42)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    c.store(USize(99)); // lint:allow(no-bare-numeric) reason: progress counter literal; tracked: #399
    assert_eq!(c.load(), USize(99)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
}

#[test]
fn progress_counter_default_is_zero() {
    let c = ProgressCounter::default();
    assert_eq!(c.load(), USize::ZERO);
}

#[test]
fn morsel_range_new_end_is_empty() {
    let r = MorselRange::new(USize(100), USize(16)); // lint:allow(no-bare-numeric) reason: morsel range literals; tracked: #399
    assert_eq!(r.start, USize(100)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(r.len, USize(16)); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(r.end(), USize(116)); // lint:allow(no-bare-numeric) reason: end-offset check; tracked: #399
    assert_eq!(r.is_empty(), Bool::FALSE);

    let empty = MorselRange::new(USize::ZERO, USize::ZERO);
    assert_eq!(empty.is_empty(), Bool::TRUE);
    assert_eq!(empty.end(), USize::ZERO);
}

#[test]
fn morsel_range_default_is_empty() {
    let r = MorselRange::default();
    assert_eq!(r.is_empty(), Bool::TRUE);
    assert_eq!(r.start, USize::ZERO);
    assert_eq!(r.len, USize::ZERO);
}

#[test]
fn sync_point_new_equality() {
    let a = SyncPoint::new(FiberId(3), USize(128)); // lint:allow(no-bare-numeric) reason: sync point literals; tracked: #399
    let b = SyncPoint::new(FiberId(3), USize(128)); // lint:allow(no-bare-numeric) reason: sync point literals; tracked: #399
    let c = SyncPoint::new(FiberId(4), USize(128)); // lint:allow(no-bare-numeric) reason: sync point literals; tracked: #399
    let d = SyncPoint::new(FiberId(3), USize(256)); // lint:allow(no-bare-numeric) reason: sync point literals; tracked: #399
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_ne!(a, d);
}

#[test]
fn dispatch_approach_variants_distinct() {
    assert_ne!(DispatchApproach::IndirectPerFiber, DispatchApproach::TrunkMega);
    assert_ne!(DispatchApproach::IndirectPerFiber, DispatchApproach::ScheduleMega);
    assert_ne!(DispatchApproach::TrunkMega, DispatchApproach::ScheduleMega);
}

#[test]
fn fiber_dispatch_default_constructs() {
    let f: FiberDispatch<StubCtx, 4> = FiberDispatch::default();
    assert!(f.body.isnt());
    assert_eq!(f.fiber_id, FiberId(0)); // lint:allow(no-bare-numeric) reason: default fiber id; tracked: #399
    assert_eq!(f.sync_point_count, USize::ZERO);
    assert_eq!(f.morsel_range.is_empty(), Bool::TRUE);
}

#[test]
fn core_dispatch_default_constructs() {
    let c: CoreDispatch<StubCtx, 4> = CoreDispatch::default();
    assert_eq!(c.fiber_count, USize::ZERO);
    assert_eq!(c.phase_count, USize::ZERO);
    assert_eq!(c.boundary_count, USize::ZERO);
    assert_eq!(c.sync_point_count, USize::ZERO);
}
