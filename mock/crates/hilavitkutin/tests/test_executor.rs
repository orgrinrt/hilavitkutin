//! The test executor's slot check, which the scheduler's worker spawn is held
//! to in every threaded test. A check that accepted everything would let a
//! scheduler change that fattens the worker closure pass unnoticed, so both
//! directions are pinned here, along with the executor itself: it runs what it
//! is handed off the calling thread, refuses a closure wider than one slot at
//! the call site, and reports the host's parallelism as its worker count.

mod common;

use core::hint::black_box;

use common::fits_one_pointer_slot;
use hilavitkutin_api::platform::ThreadPoolApi;

fn slot_fit_of<F: FnOnce()>(_: &F) -> bool {
    fits_one_pointer_slot::<F>()
}

#[repr(align(32))]
struct OverAligned;

#[test]
fn a_closure_capturing_nothing_fits() {
    let f = || {};
    assert!(slot_fit_of(&f));
}

#[test]
fn a_closure_capturing_one_pointer_fits() {
    let x = 7u8;
    let p: *const u8 = &x;
    let f = move || {
        black_box(p);
    };
    assert_eq!(
        core::mem::size_of_val(&f),
        core::mem::size_of::<*const u8>()
    );
    assert!(slot_fit_of(&f));
}

#[test]
fn a_closure_capturing_two_pointers_does_not_fit() {
    let (a, b) = (1u8, 2u8);
    let (pa, pb): (*const u8, *const u8) = (&a, &b);
    let f = move || {
        black_box((pa, pb));
    };
    assert_eq!(
        core::mem::size_of_val(&f),
        2 * core::mem::size_of::<*const u8>()
    );
    assert!(!slot_fit_of(&f));
}

#[test]
fn an_over_aligned_capture_does_not_fit() {
    let v = OverAligned;
    let f = move || {
        black_box(&v);
    };
    assert_eq!(core::mem::align_of_val(&f), 32);
    assert!(!slot_fit_of(&f));
}

#[test]
fn spawn_runs_the_closure_on_another_thread() {
    use std::sync::{OnceLock, mpsc};

    static SENDER: OnceLock<std::sync::Mutex<mpsc::Sender<std::thread::ThreadId>>> =
        OnceLock::new();
    let (tx, rx) = mpsc::channel();
    SENDER.set(std::sync::Mutex::new(tx)).unwrap();

    common::TestExecutor::new().spawn(|| {
        let tx = SENDER.get().unwrap().lock().unwrap().clone();
        tx.send(std::thread::current().id()).unwrap();
    });
    let ran_on = rx
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("the spawned closure never ran");
    assert_ne!(ran_on, std::thread::current().id());
}

#[test]
#[should_panic(expected = "does not fit one pointer-sized slot")]
fn spawn_refuses_a_closure_wider_than_one_slot() {
    let (a, b) = (1u8, 2u8);
    let (pa, pb): (*const u8, *const u8) = (&a, &b);
    let (pa, pb) = (pa as usize, pb as usize);
    common::TestExecutor::new().spawn(move || {
        black_box((pa, pb));
    });
}

#[test]
fn worker_count_is_the_host_parallelism() {
    let expected = std::thread::available_parallelism().map_or(1, |n| n.get());
    assert_eq!(
        common::TestExecutor::new().worker_count(),
        arvo::USize(expected)
    );
}
