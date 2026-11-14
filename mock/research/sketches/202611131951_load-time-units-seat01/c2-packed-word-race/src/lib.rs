//! A 1-bit packed column written by two cores, each over its own record
//! window, the way a per-record positional write would lower: read the word
//! that holds the record, set the bit, store the word. Two relaxed atomic
//! operations rather than one RMW, so the code is free of undefined behaviour
//! and still loses updates exactly as a plain load and store would.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::thread;

/// Records per 64-bit word at one bit a record.
pub const RECORDS_PER_WORD: usize = 64;

/// Write every record of `[lo, hi)` to 1 through a non-atomic
/// read-modify-write of the word that holds it.
fn write_window(words: &[AtomicU64], lo: usize, hi: usize) {
    for r in lo .. hi {
        let w = &words[r / RECORDS_PER_WORD];
        let old = w.load(Ordering::Relaxed);
        w.store(old | (1u64 << (r % RECORDS_PER_WORD)), Ordering::Relaxed);
    }
}

/// Sense-reversing spin barrier for two threads, tight enough that both
/// threads reach the shared word within a few cycles of each other.
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

/// Run `trials` frames. In each, core A writes records `[0, split)` and core
/// B writes `[split, total)`, starting together. Returns the number of frames
/// in which some record ended up 0 although its core wrote it.
pub fn frames_with_lost_writes(total: usize, split: usize, trials: usize) -> usize {
    let nwords = total.div_ceil(RECORDS_PER_WORD);
    let words: Vec<AtomicU64> = (0 .. nwords).map(|_| AtomicU64::new(0)).collect();
    let bar = Spin {
        arrived: AtomicUsize::new(0),
        sense:   AtomicUsize::new(0),
    };
    let lost = AtomicUsize::new(0);
    thread::scope(|s| {
        let (words, bar, lost) = (&words, &bar, &lost);
        let a = s.spawn(move || {
            let mut sense = 0;
            for _ in 0 .. trials {
                bar.wait(&mut sense); // frame start
                write_window(words, 0, split);
                bar.wait(&mut sense); // frame end
                // A checks the frame and clears it for the next one.
                let mut ok = true;
                for r in 0 .. total {
                    let bit = words[r / RECORDS_PER_WORD].load(Ordering::Relaxed)
                        >> (r % RECORDS_PER_WORD)
                        & 1;
                    ok &= bit == 1;
                }
                if !ok {
                    lost.fetch_add(1, Ordering::Relaxed);
                }
                for w in words.iter() {
                    w.store(0, Ordering::Relaxed);
                }
                bar.wait(&mut sense); // cleared
            }
        });
        let b = s.spawn(move || {
            let mut sense = 0;
            for _ in 0 .. trials {
                bar.wait(&mut sense);
                write_window(words, split, total);
                bar.wait(&mut sense);
                bar.wait(&mut sense);
            }
        });
        a.join().unwrap();
        b.join().unwrap();
    });
    lost.load(Ordering::Relaxed)
}
