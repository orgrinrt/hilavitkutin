//! Seat 01 reply, M11. The property asserted in every test: after the close,
//! every record a queued op set is 1. One test per drain arm; 136 records,
//! three words, 200 000 closes each.
//!
//! Expected: the single drainer, the word-aligned record split and the
//! `fetch_or` drain hold (green); the queue-position split and the 4-aligned
//! record split at 68 do not (red). If so, a queue makes a packed column
//! single-writer only when one core drains it, and a parallel drain needs
//! the same word rule as a unit writing directly, or an atomic word write.

use c7_packed_queue_drain::{Drain, closes_with_lost_writes};

const TOTAL: usize = 136;
const TRIALS: usize = 200_000;

fn run(label: &str, drain: Drain) {
    let lost = closes_with_lost_writes(TOTAL, drain, TRIALS);
    println!("{label}: {lost} of {TRIALS} closes lost a write ({drain:?})");
    assert_eq!(lost, 0, "{label}: a queued write was lost at the close");
}

#[test]
fn a_single_drainer_loses_nothing() {
    run("single drainer", Drain::Single);
}

#[test]
fn two_drainers_split_by_queue_position_lose_nothing() {
    run("two drainers, queue halves", Drain::ByQueuePosition);
}

#[test]
fn two_drainers_split_by_record_at_68_lose_nothing() {
    run(
        "two drainers, records split at 68 (4-aligned)",
        Drain::ByRecord {
            split: 68,
        },
    );
}

#[test]
fn two_drainers_split_by_record_at_64_lose_nothing() {
    run(
        "two drainers, records split at 64 (word-aligned)",
        Drain::ByRecord {
            split: 64,
        },
    );
}

#[test]
fn two_drainers_split_by_queue_position_with_fetch_or_lose_nothing() {
    run(
        "two drainers, queue halves, fetch_or",
        Drain::ByQueuePositionFetchOr,
    );
}
