//! The pieces every arm shares: the frame words, provider memory, and the
//! worker's per-frame body.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::{CORES, RECORDS};

/// The pool's frame words, as `PoolFrame` carries them.
#[derive(Default)]
pub struct Words {
    pub seq:      AtomicU32,
    pub done:     AtomicU32,
    pub exited:   AtomicU32,
    pub shutdown: AtomicBool,
    /// The band decision for the frame in flight, published with `seq`.
    pub band:     AtomicBool,
}

impl Words {
    /// Worker side: wait for a `seq` past `last`.
    pub fn await_frame(&self, last: u32) -> u32 {
        loop {
            let s = self.seq.load(Ordering::Acquire);
            if s != last {
                return s;
            }
            std::thread::yield_now();
        }
    }

    /// Calling thread: publish a frame and wait every worker through it.
    pub fn publish_and_await(&self, band: bool) {
        self.band.store(band, Ordering::Relaxed);
        let before = self.done.load(Ordering::Relaxed);
        self.seq.fetch_add(1, Ordering::Release);
        while self.done.load(Ordering::Acquire) != before + CORES as u32 {
            std::thread::yield_now();
        }
    }

    /// Calling thread: shut the pool down and wait every worker out.
    pub fn shut_down(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
        self.seq.fetch_add(1, Ordering::Release);
        while self.exited.load(Ordering::Acquire) != CORES as u32 {
            std::thread::yield_now();
        }
    }
}

/// Provider memory: one block, leaked, never inside the scheduler.
pub fn provider_block<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}

/// Where the resource slot and the output column are, as the engine's
/// bindings carry pointers into the provider's storage.
#[derive(Copy, Clone)]
pub struct Bindings {
    pub slot: *mut u32,
    pub out:  *mut u32,
}

impl Default for Bindings {
    fn default() -> Self {
        Self::new()
    }
}

impl Bindings {
    pub fn new() -> Self {
        Self {
            slot: provider_block(5u32),
            out:  provider_block([0u32; RECORDS]).cast(),
        }
    }

    /// Stands for `reset_accumulators`: takes the bindings exclusively at a
    /// frame start, as the engine does, and changes nothing a driver reads.
    pub fn reset(&mut self) {
        let _ = &mut *self;
    }

    /// The calling thread's read of the output column between frames.
    pub fn read_out(&self) -> [u32; RECORDS] {
        // SAFETY: the column holds RECORDS records and every worker is parked.
        unsafe { *self.out.cast::<[u32; RECORDS]>() }
    }
}

/// The record range core `core` owns: its half of the column.
pub const fn range_of(core: usize) -> (usize, usize) {
    let half = RECORDS / CORES;
    (core * half, (core + 1) * half)
}

/// A unit's body over one core's range: each record gets the resource value
/// times ten, plus one where the band ran.
pub fn unit_body(range: (usize, usize), bindings: &Bindings, band: bool) {
    // SAFETY: the slot is a live provider block; nothing writes it in a frame.
    let v = unsafe { *bindings.slot };
    for r in range.0 .. range.1 {
        // SAFETY: each core writes only its own half of the column.
        unsafe { bindings.out.add(r).write(v * 10 + band as u32) };
    }
}

/// A pointer made `Send` for the spawn, as the engine's `SendCtxPtr`.
pub struct SendPtr<T>(pub *const T);
// SAFETY: the pointee outlives every worker and is read under the frame
// words' happens-before.
unsafe impl<T> Send for SendPtr<T> {}
