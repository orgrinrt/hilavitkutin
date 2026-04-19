//! ResourceSnapshot round-trip tests.

use hilavitkutin::resource::{ResourceSnapshot, Slot};

#[test]
fn snapshot_default_is_zero() {
    let s: ResourceSnapshot<4> = ResourceSnapshot::default();
    for i in 0..4 {
        assert_eq!(s.get(i).0, 0);
    }
}

#[test]
fn snapshot_set_get_roundtrip() {
    let mut s: ResourceSnapshot<3> = ResourceSnapshot::new();
    s.set(0, Slot(10));
    s.set(1, Slot(20));
    s.set(2, Slot(30));
    assert_eq!(s.get(0).0, 10);
    assert_eq!(s.get(1).0, 20);
    assert_eq!(s.get(2).0, 30);
}

#[test]
fn snapshot_overwrite() {
    let mut s: ResourceSnapshot<1> = ResourceSnapshot::new();
    s.set(0, Slot(1));
    s.set(0, Slot(2));
    assert_eq!(s.get(0).0, 2);
}
