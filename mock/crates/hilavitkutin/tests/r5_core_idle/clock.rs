//! The clock, the store and the hint every core-idle fixture shares.

use core::sync::atomic::{AtomicU64, Ordering};

use hilavitkutin_api::platform::{ClockApi, MemoryProviderApi, Nanos};
use hilavitkutin_providers::ArenaColumnStorage;

pub fn store<M: MemoryProviderApi>(provider: M) -> ArenaColumnStorage<M> {
    ArenaColumnStorage::new(provider)
}

/// Strictly increasing counter clock: every `now_ns` call returns a larger
/// value than the previous one. This makes any follower park record a positive
/// idle delta (the two `now()` calls bracket at least one increment), so the
/// idle measurement is deterministic rather than dependent on real wall-clock.
pub struct CounterClock {
    c: AtomicU64,
}

impl CounterClock {
    pub fn new() -> Self {
        Self {
            c: AtomicU64::new(0),
        }
    }
}

impl ClockApi for CounterClock {
    fn now_ns(&self) -> Nanos {
        // fetch_add returns the prior value; +1 keeps the first reading nonzero.
        Nanos::from_raw(self.c.fetch_add(1, Ordering::Relaxed) + 1)
    }
}

pub type HintT = (
    hilavitkutin_api::hint::Immediate,
    hilavitkutin_api::hint::Atomic,
    hilavitkutin_api::hint::Normal,
);
