//! How the records of one fiber are divided among walkers.
//!
//! A split hands each walker the record ranges it owns and the direction it
//! walks them in. Three splits are measured:
//!
//! 1. [`Ceil`] is the engine's N-way slice, the same formula as
//!    `worker_accum_unit_outer` and the morsel-local path: `per = ceil(n /
//!    W)`, `lo = min(c * per, n)`, `hi = min(lo + per, n)`, walked forward.
//! 2. [`HeadTailMid`] is head-and-tail as the engine's plan types spell it:
//!    `RecordRange::Head { mid_slot }` walked forward from the first record
//!    and `RecordRange::Tail { mid_slot }` walked backward from the last, the
//!    meeting point fixed at plan time. The `Convergence` record keeps it.
//! 3. [`HeadTailDyn`] lets the meeting point float: the two walkers claim
//!    morsel-sized blocks from their own end of one shared cursor until the
//!    cursor's ends cross, so a walker that runs faster takes more of the
//!    fiber. This is the protocol of the head-and-tail sketch
//!    (`202606062200_e4b-head-tail-convergence`), coarsened from one claim per
//!    record to one per morsel.
//!
//! A head-and-tail split runs two walkers by design. With one walker it
//! degenerates to `RecordRange::Full`, walked forward.

use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};

use arvo::USize;
use hilavitkutin::thread::{Convergence, ThreadHandle};
use hilavitkutin_api::dispatch_codegen::RecordRange;
use hilavitkutin_api::run_cfg::{DefaultRunCfg, RunCfg};

/// The engine's morsel size, the granularity of a dynamic claim.
pub fn morsel() -> usize {
    <DefaultRunCfg as RunCfg>::MORSEL_SIZE.0
}

/// The direction a walker visits a range in.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Dir {
    Forward,
    Backward,
}

/// A division of `n` records among `walkers` walkers.
pub trait Split: Sync {
    /// Called by the caller between frames, every worker parked, before a
    /// frame over `n` records.
    fn begin(&self, _n: usize) {}

    /// Hand walker `core` of `walkers` each range it owns, in the sequence
    /// it walks them, with the direction it walks each.
    fn walk(
        &self,
        core: usize,
        walkers: usize,
        n: usize,
        visit: &mut impl FnMut(Range<usize>, Dir),
    );
}

/// The engine's N-way ceil slice.
#[derive(Copy, Clone, Debug, Default)]
pub struct Ceil;

/// The ceil slice of walker `core` of `walkers` over `n` records.
pub fn ceil_slice(core: usize, walkers: usize, n: usize) -> Range<usize> {
    let per = n.div_ceil(walkers.max(1));
    let lo = (core * per).min(n);
    let hi = (lo + per).min(n);
    lo .. hi
}

impl Split for Ceil {
    fn walk(
        &self,
        core: usize,
        walkers: usize,
        n: usize,
        visit: &mut impl FnMut(Range<usize>, Dir),
    ) {
        let r = ceil_slice(core, walkers, n);
        if !r.is_empty() {
            visit(r, Dir::Forward);
        }
    }
}

/// The record range walker `core` takes under head-and-tail with a fixed
/// meeting point.
pub fn head_tail_range(core: usize, walkers: usize, n: usize) -> RecordRange {
    let mid_slot = USize(n / 2);
    match (walkers, core) {
        (0 | 1, _) => RecordRange::Full,
        (_, 0) => {
            RecordRange::Head {
                mid_slot,
            }
        },
        (_, 1) => {
            RecordRange::Tail {
                mid_slot,
            }
        },
        // Head-and-tail runs two walkers; a third has no range.
        _ => {
            RecordRange::Head {
                mid_slot: USize(0),
            }
        },
    }
}

/// Head-and-tail with the meeting point fixed at `n / 2`.
pub struct HeadTailMid {
    conv: Convergence,
}

impl HeadTailMid {
    pub const fn new() -> Self {
        Self {
            conv: Convergence::new(ThreadHandle(USize(0)), ThreadHandle(USize(1))),
        }
    }

    /// The meeting record the last frame stored.
    pub fn meeting(&self) -> usize {
        self.conv.meeting_record.load().0
    }
}

impl Default for HeadTailMid {
    fn default() -> Self {
        Self::new()
    }
}

impl Split for HeadTailMid {
    fn walk(
        &self,
        core: usize,
        walkers: usize,
        n: usize,
        visit: &mut impl FnMut(Range<usize>, Dir),
    ) {
        match head_tail_range(core, walkers, n) {
            RecordRange::Head {
                mid_slot,
            } => {
                if mid_slot.0 > 0 {
                    visit(0 .. mid_slot.0, Dir::Forward);
                }
                if core == 0 {
                    self.conv.meeting_record.store(mid_slot);
                }
            },
            RecordRange::Tail {
                mid_slot,
            } => {
                if mid_slot.0 < n {
                    visit(mid_slot.0 .. n, Dir::Backward);
                }
            },
            _ => {
                if n > 0 {
                    visit(0 .. n, Dir::Forward);
                }
            },
        }
    }
}

/// Head-and-tail with a floating meeting point: morsel-sized blocks claimed
/// from both ends of one cursor.
///
/// The cursor packs the next head block in its high half and one past the
/// last unclaimed tail block in its low half. A claim is one compare and swap
/// on it; the walkers meet when the halves cross. No claim moves the cursor
/// after that, so each walker, once its own claim fails, stores the same
/// meeting record from it.
pub struct HeadTailDyn {
    cursor: AtomicU64,
    conv:   Convergence,
}

impl HeadTailDyn {
    pub const fn new() -> Self {
        Self {
            cursor: AtomicU64::new(0),
            conv:   Convergence::new(ThreadHandle(USize(0)), ThreadHandle(USize(1))),
        }
    }

    /// The meeting record the last frame stored.
    pub fn meeting(&self) -> usize {
        self.conv.meeting_record.load().0
    }

    /// Claim the next block from the head (`from_tail == false`) or the tail.
    fn claim(&self, from_tail: bool) -> Option<u64> {
        let mut cur = self.cursor.load(Ordering::Relaxed);
        loop {
            let lo = cur >> 32;
            let hi = cur & 0xFFFF_FFFF;
            if lo >= hi {
                return None;
            }
            let (next, block) = if from_tail {
                ((lo << 32) | (hi - 1), hi - 1)
            } else {
                (((lo + 1) << 32) | hi, lo)
            };
            match self
                .cursor
                .compare_exchange_weak(cur, next, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => return Some(block),
                Err(seen) => cur = seen,
            }
        }
    }
}

impl Default for HeadTailDyn {
    fn default() -> Self {
        Self::new()
    }
}

impl Split for HeadTailDyn {
    fn begin(&self, n: usize) {
        let blocks = n.div_ceil(morsel()) as u64;
        assert!(
            blocks <= u32::MAX as u64,
            "{n} records exceed the cursor's 32-bit block halves"
        );
        self.cursor.store(blocks, Ordering::Relaxed);
    }

    fn walk(
        &self,
        core: usize,
        walkers: usize,
        n: usize,
        visit: &mut impl FnMut(Range<usize>, Dir),
    ) {
        if walkers < 2 {
            if n > 0 {
                visit(0 .. n, Dir::Forward);
            }
            return;
        }
        if core > 1 {
            return;
        }
        let from_tail = core == 1;
        let m = morsel();
        while let Some(block) = self.claim(from_tail) {
            let lo = block as usize * m;
            let hi = (lo + m).min(n);
            visit(
                lo .. hi,
                if from_tail { Dir::Backward } else { Dir::Forward },
            );
        }
        // Both ends crossed: the head's next block is the meeting record.
        let lo = (self.cursor.load(Ordering::Relaxed) >> 32) as usize;
        self.conv.meeting_record.store(USize((lo * m).min(n)));
    }
}
