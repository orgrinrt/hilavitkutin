//! Per-frame wake-word protocol (GATE-2 R4c).
//!
//! The persistent pool spawns its workers once and parks them between frames.
//! This module is the orchestration over `PoolFrame`'s frame words (`seq`,
//! `done`, `exited`), the sibling of `barrier.rs` (which handles the
//! within-frame waist barrier `phase_arrived`). The scheduler drives the main
//! side; each worker drives the worker side.
//!
//! Every park is lost-wakeup-safe: the caller loads the word, checks the
//! condition, and only `atomic_wait(word, observed)`. The platform primitive
//! sleeps only while `*word == observed`, so a bump racing in between the load
//! and the wait returns immediately instead of sleeping through the wakeup. A
//! spin loop would sidestep this handoff, which is exactly where lost-wakeup
//! races live; the protocol uses the real futex / __ulock / WaitOnAddress
//! primitive via `parking::atomic_wait` / `atomic_wake_all`. The atomic words
//! are `u32` (the platform-wait ABI); `USize` is the surface type.
//!
//! Mechanism proven by sketches `202606071600` and `202606071700`.

use core::sync::atomic::Ordering;

use arvo::USize;
use hilavitkutin_api::platform::PoolFrame;

use super::parking::{atomic_wait, atomic_wake_all};

/// Scheduler side: publish a new frame. Reset the completion counter, bump the
/// sequence (Release, so the slot writes the scheduler made before this become
/// visible to the waking workers), and wake every parked worker.
pub fn frame_publish<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
) {
    pool.done.store(0, Ordering::Relaxed); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic reset constant; tracked: #121
    pool.seq.fetch_add(1, Ordering::Release); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic op takes u32 increment; tracked: #121
    atomic_wake_all(&pool.seq);
}

/// Worker side: park until a new frame is published past `last_seen`. Returns
/// the new sequence value (the caller threads it back as the next `last_seen`).
/// Lost-wakeup-safe: only sleeps while `seq` still equals the observed value.
pub fn frame_await<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
    last_seen: USize,
) -> USize {
    loop {
        let cur = pool.seq.load(Ordering::Acquire);
        if cur as usize != last_seen.0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic u32 widened to compare against USize; tracked: #121
            return USize(cur as usize); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic u32 widened to USize surface; tracked: #121
        }
        atomic_wait(&pool.seq, cur);
    }
}

/// Worker side: arrive at the frame-completion barrier. The last of `expected`
/// arrivers futex-wakes the scheduler parked on `done`. Release pairs with the
/// scheduler's Acquire in `frame_await_done`, publishing this worker's output
/// writes before the scheduler reads them.
pub fn frame_done_arrive<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
    expected: USize,
) {
    let prior = pool.done.fetch_add(1, Ordering::Release); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic op takes u32 increment; tracked: #121
    if (prior as usize) + 1 == expected.0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: counter classified against USize threshold; tracked: #121
        atomic_wake_all(&pool.done);
    }
}

/// Scheduler side: park until every one of `expected` workers has finished the
/// frame. Acquire pairs with the workers' Release, publishing their output.
pub fn frame_await_done<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
    expected: USize,
) {
    loop {
        let d = pool.done.load(Ordering::Acquire);
        if (d as usize) == expected.0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic u32 widened to compare against USize; tracked: #121
            return;
        }
        atomic_wait(&pool.done, d);
    }
}

/// Scheduler side: request shutdown. Set the flag, then bump `seq` and wake so
/// every parked worker observes the flag on its next `frame_await` return.
pub fn request_shutdown<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
) {
    pool.shutdown.store(true, Ordering::Relaxed);
    pool.seq.fetch_add(1, Ordering::Release); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic op takes u32 increment; tracked: #121
    atomic_wake_all(&pool.seq);
}

/// Worker side: record departure on observing shutdown and wake the scheduler.
/// The last of `expected` arrivers futex-wakes `exited`.
pub fn frame_exit_arrive<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
    expected: USize,
) {
    let prior = pool.exited.fetch_add(1, Ordering::Release); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic op takes u32 increment; tracked: #121
    if (prior as usize) + 1 == expected.0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: counter classified against USize threshold; tracked: #121
        atomic_wake_all(&pool.exited);
    }
}

/// Scheduler side: park until every one of `expected` workers has left its
/// mainloop. After this returns, the `PoolFrame` and the carrier the workers
/// read are safe to drop. This is the shutdown join (no thread join needed).
pub fn await_exit<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
    expected: USize,
) {
    loop {
        let e = pool.exited.load(Ordering::Acquire);
        if (e as usize) == expected.0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic u32 widened to compare against USize; tracked: #121
            return;
        }
        atomic_wait(&pool.exited, e);
    }
}
