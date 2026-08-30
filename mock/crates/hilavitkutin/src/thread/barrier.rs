//! Phase-barrier counter (Topic 6 axis I).
//!
//! Centralised counter shape: workers `fetch_add(1)` against
//! `PoolFrame.phase_arrived` with Release on phase exit. Last
//! arriver is the worker whose `fetch_add` returns `expected - 1`
//! (it observes the count crossing the threshold); it resets the
//! counter and wakes all parked workers.
//!
//! Tree-barrier deferred. The dense atomic shape is sound up to the
//! point cacheline ping-pong becomes measurable on 32+ core
//! workloads; the BACKLOG note in the api crate's `PoolFrame` doc
//! tracks the upgrade trigger.

use core::sync::atomic::Ordering;

use arvo::USize;
use hilavitkutin_api::platform::{Nanos, PoolFrame};

use super::parking::{atomic_wait, atomic_wake_all};

/// Worker-side sense-reversing waist barrier. Blocks until all `expected`
/// workers have arrived, then releases all; reusable across the many waists of
/// one frame with no reset race.
///
/// The reversing design (spec Topic 6 axis I, the deferred-E3 fix): a worker
/// snapshots `barrier_sense` on entry, then `fetch_add`s `phase_arrived`. The
/// last arriver (`prior + 1 == expected`) resets `phase_arrived` to zero and
/// bumps `barrier_sense` (Release), then futex-wakes everyone; followers park on
/// `barrier_sense` and wake once it differs from their snapshot. A fast worker
/// that loops straight into the next episode waits for the NEXT flip, never a
/// stale count, so the naive arrive+reset race cannot occur. Single-core
/// (`expected == 1`) takes the last-arriver path immediately with no parking.
///
/// `Release` on the sense bump publishes every prior write by every worker to
/// the followers it wakes; the `Acquire` snapshot + reload pair with it.
///
/// The follower branch times its own park into `idle_accumulator[core]` (the
/// core-idle adapt signal, domain 22). A worker that finishes its trunks for the
/// phase early arrives first and waits here while slower cores grind; that wait
/// is the per-core idle. The last arriver never parks, so a bottleneck core
/// records no idle, which is correct (it was the cause, not the starved core).
/// Measuring inside the primitive captures every waist a worker crosses,
/// independent of which dispatch path the frame took. `now` is the caller's
/// monotonic clock read (`impl Fn`, monomorphised, no dyn); the caller holds the
/// concrete clock and the frame does not carry one.
pub fn waist_barrier<'arena, const C: usize, const P: usize>(
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
    core: USize,
    expected: USize,
    now: impl Fn() -> Nanos,
) {
    let sense = pool.barrier_sense.load(Ordering::Acquire);
    let prior = pool.phase_arrived.fetch_add(1, Ordering::AcqRel); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic increment; tracked: #121
    if (prior as usize) + 1 == expected.0 {
        // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: counter classified against USize threshold; tracked: #121
        // Last arriver: open the next episode. Reset the count first (Relaxed;
        // the sense Release publishes it), then flip the sense and wake all.
        pool.phase_arrived.store(0, Ordering::Relaxed); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic reset constant; tracked: #121
        pool.barrier_sense
            .store(sense.wrapping_add(1), Ordering::Release); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: generation bump; tracked: #121
        atomic_wake_all(&pool.barrier_sense);
    } else {
        // Follower: park until the sense flips (lost-wakeup-safe load-check-wait),
        // timing the wait into this core's idle accumulator.
        let entered = now().to_raw(); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: raw nanos for atomic accumulator math; tracked: #121
        loop {
            if pool.barrier_sense.load(Ordering::Acquire) != sense {
                break;
            }
            atomic_wait(&pool.barrier_sense, sense);
        }
        let waited = now().to_raw().saturating_sub(entered); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: monotonic park delta in raw nanos; tracked: #121
        let slot = core.0;
        // Bounds guard: the worker count can exceed MAX_CORES, so the core id is
        // not assumed in range. An out-of-range core simply does not record idle.
        if slot < pool.idle_accumulator.len() {
            pool.idle_accumulator[slot].fetch_add(waited, Ordering::Release);
        }
    }
}
