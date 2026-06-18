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
use hilavitkutin_api::platform::PoolFrame;
use notko::Maybe;

use super::parking::{atomic_wait, atomic_wake_all};

/// Outcome of a single worker's barrier arrival.
///
/// `Last` means the caller observed the counter crossing the
/// `expected` threshold; the caller is responsible for resetting
/// the counter and waking parked workers via the platform atomic-
/// wait primitive (the parking module's `wake_all`). `Following`
/// means at least one worker has not yet arrived.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BarrierArrival {
    /// This caller was the last to arrive at the barrier.
    Last,
    /// Other workers are still outstanding.
    Following,
}

/// Worker-side barrier arrival. Release fetch_add on the centralised
/// counter, classify against `expected`.
///
/// The caller passes `expected` (the number of workers participating
/// in this phase). The `Release` ordering pairs with the `Acquire`
/// reset in `phase_barrier_reset`: every prior write by this worker
/// becomes visible to the worker that performs the reset, and via
/// transitivity to every worker the resetter wakes.
pub fn phase_barrier_arrive<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
    expected: USize,
) -> BarrierArrival {
    let prior = pool.phase_arrived.fetch_add(1, Ordering::Release); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic op takes u32 increment; tracked: #121
    if (prior as usize) + 1 == expected.0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: counter classified against USize threshold; tracked: #121
        BarrierArrival::Last
    } else {
        BarrierArrival::Following
    }
}

/// Reset the centralised barrier counter. Called by the worker that
/// observed `BarrierArrival::Last`. `Acquire` on the load to publish
/// every prior worker's writes; the store at zero is Release so the
/// next phase's arrivers see the fresh count.
pub fn phase_barrier_reset<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
) {
    pool.phase_arrived.store(0, Ordering::Release); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic reset constant; tracked: #121
}

/// Snapshot the current arrival count. Workers use this to check
/// progress without participating; the engine's adapt subsystem
/// reads it at `ScheduleEnd` for per-phase latency attribution.
/// `Acquire` so the observed count synchronises with the latest
/// arriver's Release.
pub fn phase_barrier_observe<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
) -> Maybe<USize> {
    let v = pool.phase_arrived.load(Ordering::Acquire);
    Maybe::Is(USize(v as usize)) // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic u32 to USize widening; tracked: #121
}

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
pub fn waist_barrier<'arena, const C: usize, const P: usize>( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    pool: &PoolFrame<'arena, C, P>,
    expected: USize,
) {
    let sense = pool.barrier_sense.load(Ordering::Acquire);
    let prior = pool.phase_arrived.fetch_add(1, Ordering::AcqRel); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic increment; tracked: #121
    if (prior as usize) + 1 == expected.0 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: counter classified against USize threshold; tracked: #121
        // Last arriver: open the next episode. Reset the count first (Relaxed;
        // the sense Release publishes it), then flip the sense and wake all.
        pool.phase_arrived.store(0, Ordering::Relaxed); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: atomic reset constant; tracked: #121
        pool.barrier_sense.store(sense.wrapping_add(1), Ordering::Release); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: generation bump; tracked: #121
        atomic_wake_all(&pool.barrier_sense);
    } else {
        // Follower: park until the sense flips (lost-wakeup-safe load-check-wait).
        loop {
            if pool.barrier_sense.load(Ordering::Acquire) != sense {
                break;
            }
            atomic_wait(&pool.barrier_sense, sense);
        }
    }
}
