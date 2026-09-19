//! `ProgressCounter`: a Release store publishes, an Acquire load observes.
//!
//! The counter is how one core tells another how far through a fiber's
//! records it got, so the property that matters is message passing: once a
//! reader loads a count of `p`, every record the writer wrote before storing
//! `p` is visible to it. The two-thread test writes each record's payload with
//! a Relaxed store, publishes the count with the counter's Release store, and
//! has the reader check every newly covered payload after its Acquire load.
//! The payloads' own ordering is Relaxed on purpose, so the counter's pair is
//! the only thing ordering them.
//!
//! With the counter's store and load both made Relaxed, the test failed six
//! runs of six on an aarch64 host. A strongly ordered host such as x86_64
//! keeps stores in order whatever the ordering says, so there it cannot tell
//! the two apart.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use arvo::USize;
use hilavitkutin::dispatch::ProgressCounter;

const RECORDS: usize = 4096;
const TRIALS: usize = 4096;

fn payload_of(record: usize) -> usize {
    record.wrapping_mul(2654435761).wrapping_add(1)
}

#[test]
fn a_new_counter_loads_its_start() {
    for start in [0, 1, 63, 64, RECORDS, usize::MAX] {
        assert_eq!(ProgressCounter::new(USize(start)).load(), USize(start));
    }
}

#[test]
fn a_default_counter_starts_at_zero() {
    assert_eq!(ProgressCounter::default().load(), USize(0));
}

#[test]
fn a_store_replaces_the_value_a_later_load_sees() {
    let c = ProgressCounter::new(USize(5));
    c.store(USize(9));
    assert_eq!(c.load(), USize(9));
    c.store(USize(2));
    assert_eq!(c.load(), USize(2), "the counter stores what it is given");
}

#[test]
fn every_record_before_a_published_count_is_visible_to_the_reader() {
    for trial in 0 .. TRIALS {
        let payloads: Arc<Vec<AtomicUsize>> =
            Arc::new((0 .. RECORDS).map(|_| AtomicUsize::new(0)).collect());
        let counter = Arc::new(ProgressCounter::new(USize(0)));

        let writer = {
            let payloads = Arc::clone(&payloads);
            let counter = Arc::clone(&counter);
            std::thread::spawn(move || {
                for r in 0 .. RECORDS {
                    payloads[r].store(payload_of(r), Ordering::Relaxed);
                    counter.store(USize(r + 1));
                }
            })
        };

        let mut seen = 0;
        while seen < RECORDS {
            let p = counter.load().0;
            assert!(
                p >= seen,
                "trial {trial}: the count went back from {seen} to {p}"
            );
            for r in seen .. p {
                assert_eq!(
                    payloads[r].load(Ordering::Relaxed),
                    payload_of(r),
                    "trial {trial}: record {r} was published by count {p} but not visible"
                );
            }
            seen = p;
            std::hint::spin_loop();
        }
        writer.join().expect("the writer does not panic");
        assert_eq!(counter.load(), USize(RECORDS));
    }
}
