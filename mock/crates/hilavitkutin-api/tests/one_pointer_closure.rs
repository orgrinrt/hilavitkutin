//! `OnePointerClosure::FITS` admits every closure one pointer slot can
//! carry. The closures it refuses are `compile_fail` doctests on
//! `OnePointerClosure` itself: the refusal is a monomorphisation error,
//! which a trybuild fixture never reaches because trybuild only checks.
//!
//! Each case forces the gate, then runs the closure and checks it saw
//! what it captured, so a case that compiled by capturing nothing would
//! be caught.

use core::cell::Cell;
use core::mem::{align_of, size_of};

use hilavitkutin_api::platform::OnePointerClosure;

fn force_and_run<F: FnOnce() -> usize>(f: F) -> usize {
    let () = OnePointerClosure::<F>::FITS;
    f()
}

#[test]
fn a_captureless_closure_fits() {
    let f = || 7usize;
    assert_eq!(size_of_val(&f), 0);
    assert_eq!(force_and_run(f), 7);
}

#[test]
fn a_closure_capturing_one_reference_fits() {
    let seen = Cell::new(0usize);
    let r = &seen;
    let f = move || {
        r.set(r.get() + 1);
        r.get()
    };
    assert_eq!(size_of_val(&f), size_of::<*const ()>());
    assert_eq!(force_and_run(f), 1);
    assert_eq!(seen.get(), 1);
}

#[test]
fn a_closure_capturing_one_pointer_sized_value_fits() {
    let v = 41usize;
    let f = move || v + 1;
    assert_eq!(size_of_val(&f), size_of::<*const ()>());
    assert_eq!(force_and_run(f), 42);
}

#[test]
fn a_closure_capturing_a_value_smaller_than_a_pointer_fits() {
    let v = 5u8;
    let f = move || usize::from(v) * 2;
    assert!(size_of_val(&f) < size_of::<*const ()>());
    assert_eq!(force_and_run(f), 10);
}

#[test]
fn a_closure_capturing_a_zero_sized_value_fits() {
    #[derive(Copy, Clone)]
    struct Marker;
    impl Marker {
        fn value(self) -> usize {
            3
        }
    }
    let m = Marker;
    let f = move || m.value();
    assert_eq!(size_of_val(&f), 0);
    assert!(align_of_val(&f) <= align_of::<*const ()>());
    assert_eq!(force_and_run(f), 3);
}

fn size_of_val<T>(_: &T) -> usize {
    size_of::<T>()
}

fn align_of_val<T>(_: &T) -> usize {
    align_of::<T>()
}
