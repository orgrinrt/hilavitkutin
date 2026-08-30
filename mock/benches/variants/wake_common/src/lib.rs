//! Shared carrier for the wake_policy bench pair (deviation-3 evidence,
//! round 202607202310).
//!
//! A persistent pool of real std threads parks on a real `PoolFrame`
//! through the shipped `thread::frame_*` helpers, exactly the engine's
//! frame protocol. One timed frame is the full round trip: the main
//! thread publishes, every worker wakes, folds a bounded slice of
//! per-record work into its private accumulator, arrives at the done
//! barrier, and the main thread wakes from `frame_await_done`. The two
//! arms differ ONLY in the `spin_budget` passed to `frame_await` /
//! `frame_await_done`: 0 is the shipped park-immediately baseline,
//! 128 the canonical Topic 6 axis K middle tier. Identical work means
//! identical outputs, so cross-variant validation stays strict.
//!
//! The size axis `n` is the per-core record count of the in-frame
//! work: small n keeps the waits in the wake-latency-dominated regime
//! the spin tier targets; large n shifts toward compute-dominated
//! frames where the parking cost amortises.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use arvo::USize;
use hilavitkutin::thread::{
    atomic_wait, frame_await, frame_await_done, frame_done_arrive, frame_exit_arrive, frame_publish,
};
use hilavitkutin_api::platform::PoolFrame;

/// Wait-policy axis for the round-2 arms: how the bounded pre-roll
/// iterates before the (identical) lost-wakeup-safe park. `Shipped`
/// routes through the engine's `frame_await` / `frame_await_done`
/// (whose spin iteration is `core::hint::spin_loop`, an ISB on
/// aarch64 at roughly 10 to 20 ns per iteration); `Local` uses the
/// same atomics and the same `atomic_wait` park with the spin
/// iteration parameterised (plain re-load, optionally hinted), so the
/// per-iteration cost axis is measurable independently of the budget.
#[derive(Copy, Clone)]
pub enum WaitPolicy {
    Shipped { budget: usize },
    Local { budget: usize, hint: bool },
}

fn local_await_seq(
    pool: &PoolFrame<'static, 8, 2>,
    last: usize,
    budget: usize,
    hint: bool,
) -> usize {
    let mut spins = 0usize;
    loop {
        let cur = pool.seq.load(std::sync::atomic::Ordering::Acquire);
        if cur as usize != last {
            return cur as usize;
        }
        if spins < budget {
            spins += 1;
            if hint {
                core::hint::spin_loop();
            }
            continue;
        }
        atomic_wait(&pool.seq, cur);
    }
}

fn local_await_done(pool: &PoolFrame<'static, 8, 2>, expected: usize, budget: usize, hint: bool) {
    let mut spins = 0usize;
    loop {
        let d = pool.done.load(std::sync::atomic::Ordering::Acquire);
        if d as usize == expected {
            return;
        }
        if spins < budget {
            spins += 1;
            if hint {
                core::hint::spin_loop();
            }
            continue;
        }
        atomic_wait(&pool.done, d);
    }
}

pub use rcm_common::fnv1a_u32_slice;

const NCORES: usize = 3;

/// Per-worker shared cells the frame protocol reads and writes.
struct Shared {
    pool: PoolFrame<'static, 8, 2>,
    /// Per-core record count for the next frame, written before publish.
    work: AtomicUsize,
    /// Seed folded into the per-record mix, written before publish.
    seed: AtomicU64,
    /// Per-worker accumulators, gathered after the done barrier.
    acc: [AtomicU64; NCORES],
    /// Worker-side wait policy (budget encoding: bit 63 set = local
    /// no-hint, bit 62 set = local hinted; else shipped).
    policy_bits: AtomicUsize,
}

const LOCAL_NOHINT: usize = 1 << 63;
const LOCAL_HINT: usize = 1 << 62;

fn encode(policy: WaitPolicy) -> usize {
    match policy {
        WaitPolicy::Shipped { budget } => budget,
        WaitPolicy::Local {
            budget,
            hint: false,
        } => budget | LOCAL_NOHINT,
        WaitPolicy::Local { budget, hint: true } => budget | LOCAL_HINT,
    }
}

fn decode(bits: usize) -> WaitPolicy {
    if bits & LOCAL_NOHINT != 0 {
        WaitPolicy::Local {
            budget: bits & !LOCAL_NOHINT,
            hint: false,
        }
    } else if bits & LOCAL_HINT != 0 {
        WaitPolicy::Local {
            budget: bits & !LOCAL_HINT,
            hint: true,
        }
    } else {
        WaitPolicy::Shipped { budget: bits }
    }
}

fn make_pool() -> PoolFrame<'static, 8, 2> {
    use core::marker::PhantomData;
    use core::ptr::NonNull;
    use core::sync::atomic::AtomicBool;
    use std::sync::atomic::AtomicU32;
    PoolFrame {
        shutdown: AtomicBool::new(false),
        phase_arrived: AtomicU32::new(0),
        barrier_sense: AtomicU32::new(0),
        seq: AtomicU32::new(0),
        done: AtomicU32::new(0),
        exited: AtomicU32::new(0),
        predicted_wait_ns: core::array::from_fn(|_| AtomicU32::new(0)),
        idle_accumulator: core::array::from_fn(|_| AtomicU64::new(0)),
        park_count: core::array::from_fn(|_| AtomicU64::new(0)),
        // The wake bench never reads progress slots; the dangling
        // non-null is never dereferenced.
        progress_slots: NonNull::dangling(),
        progress_slot_count: USize(0),
        _arena: PhantomData,
    }
}

/// A prepared wake carrier: persistent workers parked on the frame word.
pub struct WakeCarrier {
    shared: &'static Shared,
}

impl WakeCarrier {
    /// Spawn the persistent worker pool with the given wait policy.
    /// Leaked allocations are intentional: the pool lives for the bench
    /// worker process's lifetime, matching the engine's spawn-once shape.
    pub fn new_with(policy: WaitPolicy) -> Self {
        let shared: &'static Shared = Box::leak(Box::new(Shared {
            pool: make_pool(),
            work: AtomicUsize::new(0),
            seed: AtomicU64::new(0),
            acc: core::array::from_fn(|_| AtomicU64::new(0)),
            policy_bits: AtomicUsize::new(encode(policy)),
        }));
        for c in 0..NCORES {
            std::thread::spawn(move || {
                let s = shared;
                let mut last = USize(0);
                loop {
                    last = match decode(s.policy_bits.load(Ordering::Relaxed)) {
                        WaitPolicy::Shipped { budget } => frame_await(&s.pool, last, USize(budget)),
                        WaitPolicy::Local { budget, hint } => {
                            USize(local_await_seq(&s.pool, last.0, budget, hint))
                        }
                    };
                    if s.pool.shutdown.load(Ordering::Relaxed) {
                        frame_exit_arrive(&s.pool, USize(NCORES));
                        return;
                    }
                    let n = s.work.load(Ordering::Relaxed);
                    let seed = s.seed.load(Ordering::Relaxed);
                    // Bounded per-record fold: a cheap multiply-xor mix
                    // dependent on the record index and the seed only
                    // (never the frame seq: arms and warmups run
                    // different frame counts, and validation demands a
                    // pure function of input and size).
                    let mut acc: u64 = seed;
                    let base = c as u64;
                    let mut i: u64 = 0;
                    while i < n as u64 {
                        acc = (acc ^ (base.wrapping_mul(1_000_003).wrapping_add(i)))
                            .wrapping_mul(0x100000001B3);
                        i += 1;
                    }
                    s.acc[c].store(acc, Ordering::Relaxed);
                    frame_done_arrive(&s.pool, USize(NCORES));
                }
            });
        }
        Self { shared }
    }

    /// Run one timed frame: publish, wait for all workers, gather.
    pub fn frame(&self, per_core_records: usize, seed: u64) -> u64 {
        let s = self.shared;
        s.work.store(per_core_records, Ordering::Relaxed);
        s.seed.store(seed, Ordering::Relaxed);
        let policy = decode(s.policy_bits.load(Ordering::Relaxed));
        frame_publish(&s.pool);
        match policy {
            WaitPolicy::Shipped { budget } => {
                frame_await_done(&s.pool, USize(NCORES), USize(budget));
            }
            WaitPolicy::Local { budget, hint } => {
                local_await_done(&s.pool, NCORES, budget, hint);
            }
        }
        let mut out: u64 = 0;
        for a in s.acc.iter() {
            out = out.wrapping_mul(0x9E3779B97F4A7C15) ^ a.load(Ordering::Relaxed);
        }
        out
    }
}

/// FNV-1a of the raw input bytes, the shared seed derivation.
pub fn seed_from_input(input: &[u8]) -> u64 {
    let mut acc: u64 = 0xcbf29ce484222325;
    for b in input.iter() {
        acc = (acc ^ (*b as u64)).wrapping_mul(0x100000001b3);
    }
    acc
}
