//! ResourceSnapshot round-trip tests.

use arvo::ufixed::UFixed;
use arvo::USize;
use hilavitkutin::resource::{ResourceSnapshot, Slot};

#[test]
fn snapshot_default_is_zero() {
    let s: ResourceSnapshot<4> = ResourceSnapshot::default();
    for i in 0..4 {
        assert_eq!(s.get(USize(i)).0.to_raw(), 0); // lint:allow(no-bare-numeric) reason: index + raw literal; tracked: #399
    }
}

#[test]
fn snapshot_set_get_roundtrip() {
    let mut s: ResourceSnapshot<3> = ResourceSnapshot::new();
    s.set(USize(0), Slot(UFixed::from_raw(10))); // lint:allow(no-bare-numeric) reason: slot payload literal; tracked: #399
    s.set(USize(1), Slot(UFixed::from_raw(20))); // lint:allow(no-bare-numeric) reason: slot payload literal; tracked: #399
    s.set(USize(2), Slot(UFixed::from_raw(30))); // lint:allow(no-bare-numeric) reason: slot payload literal; tracked: #399
    assert_eq!(s.get(USize(0)).0.to_raw(), 10); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(s.get(USize(1)).0.to_raw(), 20); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
    assert_eq!(s.get(USize(2)).0.to_raw(), 30); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
}

#[test]
fn snapshot_overwrite() {
    let mut s: ResourceSnapshot<1> = ResourceSnapshot::new();
    s.set(USize(0), Slot(UFixed::from_raw(1))); // lint:allow(no-bare-numeric) reason: slot payload literal; tracked: #399
    s.set(USize(0), Slot(UFixed::from_raw(2))); // lint:allow(no-bare-numeric) reason: slot payload literal; tracked: #399
    assert_eq!(s.get(USize(0)).0.to_raw(), 2); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
}
