//! A 1-bit packed column filled at the close from a queue of "set record r"
//! ops, by one or two drainers. The word write is a relaxed load then a
//! relaxed store, the same read-modify-write model as c2 (free of undefined
//! behaviour and losing updates as a plain load and store would), or an
//! atomic `fetch_or` for the arm that changes the implementation instead of
//! the split. Spike: the queue, the split and the column are whatever reach
//! the check.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::thread;

/// Records per 64-bit word at one bit a record.
pub const RECORDS_PER_WORD: usize = 64;

/// How the queue's ops are shared between the drainers.
#[derive(Copy, Clone, Debug)]
pub enum Drain {
    /// One drainer applies the whole queue.
    Single,
    /// Drainer 0 takes the first half of the queue, drainer 1 the rest.
    ByQueuePosition,
    /// Each drainer scans the whole queue and applies only ops whose record
    /// falls in its range; drainer 0 owns records below `split`.
    ByRecord {
        split: usize,
    },
    /// As `ByQueuePosition`, with an atomic `fetch_or` for the word write.
    ByQueuePositionFetchOr,
}

fn set_rmw(words: &[AtomicU64], r: usize) {
    let w = &words[r / RECORDS_PER_WORD];
    let old = w.load(Ordering::Relaxed);
    w.store(old | (1u64 << (r % RECORDS_PER_WORD)), Ordering::Relaxed);
}

fn set_fetch_or(words: &[AtomicU64], r: usize) {
    words[r / RECORDS_PER_WORD].fetch_or(1u64 << (r % RECORDS_PER_WORD), Ordering::Relaxed);
}

fn drain_share(words: &[AtomicU64], queue: &[usize], drain: Drain, me: usize) {
    match drain {
        Drain::Single => {
            if me == 0 {
                for &r in queue {
                    set_rmw(words, r);
                }
            }
        },
        Drain::ByQueuePosition => {
            let half = queue.len() / 2;
            let mine = if me == 0 { &queue[.. half] } else { &queue[half ..] };
            for &r in mine {
                set_rmw(words, r);
            }
        },
        Drain::ByRecord {
            split,
        } => {
            for &r in queue {
                if (r < split) == (me == 0) {
                    set_rmw(words, r);
                }
            }
        },
        Drain::ByQueuePositionFetchOr => {
            let half = queue.len() / 2;
            let mine = if me == 0 { &queue[.. half] } else { &queue[half ..] };
            for &r in mine {
                set_fetch_or(words, r);
            }
        },
    }
}

/// The queue a frame's units emitted: every record of `[0, total)` once, in
/// an order no drainer chose (a fixed pseudo-random permutation).
pub fn emitted_queue(total: usize) -> Vec<usize> {
    let mut q: Vec<usize> = (0 .. total).collect();
    let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
    for i in (1 .. q.len()).rev() {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        q.swap(i, (x % (i as u64 + 1)) as usize);
    }
    q
}

/// Sense-reversing spin barrier for two threads (as c2's).
struct Spin {
    arrived: AtomicUsize,
    sense:   AtomicUsize,
}

impl Spin {
    fn wait(&self, local: &mut usize) {
        *local ^= 1;
        if self.arrived.fetch_add(1, Ordering::AcqRel) == 1 {
            self.arrived.store(0, Ordering::Relaxed);
            self.sense.store(*local, Ordering::Release);
        } else {
            while self.sense.load(Ordering::Acquire) != *local {
                core::hint::spin_loop();
            }
        }
    }
}

/// Run `trials` closes. In each, both drainers start together on the same
/// queue. Returns the number of closes after which some record is 0.
pub fn closes_with_lost_writes(total: usize, drain: Drain, trials: usize) -> usize {
    let queue = emitted_queue(total);
    let nwords = total.div_ceil(RECORDS_PER_WORD);
    let words: Vec<AtomicU64> = (0 .. nwords).map(|_| AtomicU64::new(0)).collect();
    let bar = Spin {
        arrived: AtomicUsize::new(0),
        sense:   AtomicUsize::new(0),
    };
    let lost = AtomicUsize::new(0);
    thread::scope(|s| {
        let (words, bar, lost, queue) = (&words, &bar, &lost, &queue);
        let a = s.spawn(move || {
            let mut sense = 0;
            for _ in 0 .. trials {
                bar.wait(&mut sense);
                drain_share(words, queue, drain, 0);
                bar.wait(&mut sense);
                let ok = (0 .. total).all(|r| {
                    words[r / RECORDS_PER_WORD].load(Ordering::Relaxed) >> (r % RECORDS_PER_WORD)
                        & 1
                        == 1
                });
                if !ok {
                    lost.fetch_add(1, Ordering::Relaxed);
                }
                for w in words.iter() {
                    w.store(0, Ordering::Relaxed);
                }
                bar.wait(&mut sense);
            }
        });
        let b = s.spawn(move || {
            let mut sense = 0;
            for _ in 0 .. trials {
                bar.wait(&mut sense);
                drain_share(words, queue, drain, 1);
                bar.wait(&mut sense);
                bar.wait(&mut sense);
            }
        });
        a.join().unwrap();
        b.join().unwrap();
    });
    lost.load(Ordering::Relaxed)
}
