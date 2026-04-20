//! Dispatch-stage type surface tests (5a3 skeleton).

use hilavitkutin::dispatch::{
    CoreDispatch, DispatchApproach, FiberDispatch, MorselRange, ProgressCounter, SyncPoint,
};
use hilavitkutin::plan::FiberId;

#[derive(Default)]
struct StubCtx;

#[test]
fn progress_counter_store_load_round_trip() {
    let c = ProgressCounter::new(0);
    assert_eq!(c.load(), 0);
    c.store(42);
    assert_eq!(c.load(), 42);
    c.store(99);
    assert_eq!(c.load(), 99);
}

#[test]
fn progress_counter_default_is_zero() {
    let c = ProgressCounter::default();
    assert_eq!(c.load(), 0);
}

#[test]
fn morsel_range_new_end_is_empty() {
    let r = MorselRange::new(100, 16);
    assert_eq!(r.start, 100);
    assert_eq!(r.len, 16);
    assert_eq!(r.end(), 116);
    assert!(!r.is_empty());

    let empty = MorselRange::new(0, 0);
    assert!(empty.is_empty());
    assert_eq!(empty.end(), 0);
}

#[test]
fn morsel_range_default_is_empty() {
    let r = MorselRange::default();
    assert!(r.is_empty());
    assert_eq!(r.start, 0);
    assert_eq!(r.len, 0);
}

#[test]
fn sync_point_new_equality() {
    let a = SyncPoint::new(FiberId(3), 128);
    let b = SyncPoint::new(FiberId(3), 128);
    let c = SyncPoint::new(FiberId(4), 128);
    let d = SyncPoint::new(FiberId(3), 256);
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
    assert!(f.body.is_none());
    assert_eq!(f.fiber_id, FiberId(0));
    assert_eq!(f.sync_point_count, 0);
    assert!(f.morsel_range.is_empty());
}

#[test]
fn core_dispatch_default_constructs() {
    let c: CoreDispatch<StubCtx, 4> = CoreDispatch::default();
    assert_eq!(c.fiber_count, 0);
    assert_eq!(c.phase_count, 0);
    assert_eq!(c.boundary_count, 0);
    assert_eq!(c.sync_point_count, 0);
}
