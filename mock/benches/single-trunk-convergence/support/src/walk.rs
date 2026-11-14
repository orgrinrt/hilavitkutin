//! The crew arms: a fiber walked by the bench's own workers under a split.
//!
//! A fold walker keeps a stack-local partial and stores it into its own slot
//! at the end of its walk; the caller merges the slots through the engine's
//! `ConvergenceBuffer` with wrapping addition, inside the frame, as the
//! engine merges inside `run_parallel`. An append walker appends into its own
//! region of one output buffer, starting at the region's first record, and
//! the caller compacts the regions in core sequence after the frame, the
//! scheme of the engine's threaded accumulator.

use std::cell::UnsafeCell;
use std::ops::Range;

use arvo::USize;
use evs_support::Crew;
use hilavitkutin::resource::ConvergenceBuffer;

use crate::ConvOutput;
use crate::engine::Op;
use crate::kernel::{Kernel, fold_backward, fold_forward};
use crate::split::{Ceil, Dir, HeadTailMid, Split, ceil_slice, head_tail_range};

/// The most walkers a crew arm runs: the largest worker count the bench
/// sweeps.
pub const MAX_WALKERS: usize = 8;

/// One walker's result, alone on its cache lines so walkers never share one.
#[repr(align(128))]
#[derive(Default)]
struct Slot {
    value: UnsafeCell<u64>,
    count: UnsafeCell<u64>,
}

// SAFETY: walker `c` writes only slot `c`, inside a frame; the caller reads
// the slots only after the frame's done barrier (Release then Acquire).
unsafe impl Sync for Slot {}

fn slots() -> Box<[Slot]> {
    (0 .. MAX_WALKERS).map(|_| Slot::default()).collect()
}

fn check_walkers(walkers: usize) -> usize {
    assert!(
        (1 ..= MAX_WALKERS).contains(&walkers),
        "a crew arm runs 1 to {MAX_WALKERS} walkers, asked for {walkers}"
    );
    walkers
}

/// The fold arm: `walkers` walkers under `split`.
pub fn fold<K: Kernel, S: Split + 'static>(
    data: Vec<u32>,
    split: S,
    walkers: usize,
) -> impl FnMut(Op) -> ConvOutput + 'static {
    let mut crew = Crew::new(check_walkers(walkers));
    let slots = slots();
    let n = data.len();
    let mut last = ConvOutput::default();
    move |op| {
        match op {
            Op::Frame => {
                split.begin(n);
                let w = crew.workers();
                let job = |core: usize| {
                    let mut acc = 0u64;
                    let mut seen = 0u64;
                    split.walk(core, w, n, &mut |r, dir| {
                        seen += r.len() as u64;
                        let xs = &data[r];
                        acc = match dir {
                            Dir::Forward => fold_forward::<K>(acc, xs),
                            Dir::Backward => fold_backward::<K>(acc, xs),
                        };
                    });
                    // SAFETY: slot `core` is this walker's alone (see `Slot`).
                    unsafe {
                        *slots[core].value.get() = acc;
                        *slots[core].count.get() = seen;
                    }
                };
                crew.frame(&job);
                // The merge at the meeting point, every worker parked.
                let mut values = ConvergenceBuffer::<u64, MAX_WALKERS>::new(0);
                let mut count = 0u64;
                for c in 0 .. w {
                    // SAFETY: the frame is over; no walker writes a slot.
                    let (v, k) = unsafe { (*slots[c].value.get(), *slots[c].count.get()) };
                    values.set(USize(c), v);
                    count += k;
                }
                last = ConvOutput {
                    value: values.combine(0, u64::wrapping_add),
                    count,
                };
                ConvOutput::default()
            },
            Op::Read => last,
        }
    }
}

/// A split that also names where each walker's append region starts.
pub trait Regions: Split {
    /// The records walker `core` of `walkers` may append, as a region of
    /// the output buffer: at most one value per record it walks.
    fn region(&self, core: usize, walkers: usize, n: usize) -> Range<usize>;
}

impl Regions for Ceil {
    fn region(&self, core: usize, walkers: usize, n: usize) -> Range<usize> {
        ceil_slice(core, walkers, n)
    }
}

/// No arm the bench times appends under this split: head-and-tail takes the
/// sequential fallback on the append (`arms::HeadTail`). It is the split
/// without that fallback, which `tests/correctness.rs` runs to show the
/// reversed tail the fallback exists to avoid.
impl Regions for HeadTailMid {
    fn region(&self, core: usize, walkers: usize, n: usize) -> Range<usize> {
        use hilavitkutin_api::dispatch_codegen::RecordRange;
        match head_tail_range(core, walkers, n) {
            RecordRange::Head {
                mid_slot,
            } => 0 .. mid_slot.0,
            RecordRange::Tail {
                mid_slot,
            } => mid_slot.0 .. n,
            _ => 0 .. n,
        }
    }
}

/// The output buffer's base, shared with the walkers.
#[derive(Copy, Clone)]
struct OutPtr(*mut u32);

// SAFETY: walkers write disjoint regions of the buffer (see `Regions`), and
// the caller reads it only after the frame's done barrier.
unsafe impl Send for OutPtr {}
unsafe impl Sync for OutPtr {}

impl OutPtr {
    /// The element `i` places past the base. A method, so a closure captures
    /// the whole `OutPtr` rather than its raw pointer field.
    fn at(self, i: usize) -> *mut u32 {
        // SAFETY: callers stay inside the buffer (see `append`).
        unsafe { self.0.add(i) }
    }
}

/// The append arm: `walkers` walkers under `split`, each appending into its
/// own region in the sequence it walks its records.
pub fn append<K: Kernel, S: Regions + 'static>(
    data: Vec<u32>,
    split: S,
    walkers: usize,
) -> impl FnMut(Op) -> ConvOutput + 'static {
    let mut crew = Crew::new(check_walkers(walkers));
    let lens = slots();
    let n = data.len();
    let mut out = vec![0u32; n];
    let mut total = 0usize;
    move |op| {
        match op {
            Op::Frame => {
                split.begin(n);
                let w = crew.workers();
                let base = OutPtr(out.as_mut_ptr());
                let job = |core: usize| {
                    let region = split.region(core, w, n);
                    let mut len = 0usize;
                    split.walk(core, w, n, &mut |r, dir| {
                        assert!(
                            len + r.len() <= region.len(),
                            "a walker overran its append region"
                        );
                        // SAFETY: [region.start + len, + r.len()) lies inside this
                        // walker's region (asserted), which no other walker writes.
                        let dst = unsafe {
                            std::slice::from_raw_parts_mut(base.at(region.start + len), r.len())
                        };
                        let xs = &data[r];
                        match dir {
                            Dir::Forward => {
                                for (d, &x) in dst.iter_mut().zip(xs) {
                                    *d = K::apply(x);
                                }
                            },
                            Dir::Backward => {
                                for (d, &x) in dst.iter_mut().zip(xs.iter().rev()) {
                                    *d = K::apply(x);
                                }
                            },
                        }
                        len += xs.len();
                    });
                    // SAFETY: slot `core` is this walker's alone.
                    unsafe { *lens[core].count.get() = len as u64 };
                };
                crew.frame(&job);
                // Compact the regions in core sequence, every worker parked.
                let mut at = 0usize;
                for c in 0 .. w {
                    let region = split.region(c, w, n);
                    // SAFETY: the frame is over; no walker writes a slot.
                    let len = unsafe { *lens[c].count.get() } as usize;
                    if region.start != at {
                        out.copy_within(region.start .. region.start + len, at);
                    }
                    at += len;
                }
                total = at;
                ConvOutput::default()
            },
            Op::Read => {
                ConvOutput {
                    value: crate::data::hash(&out[.. total]),
                    count: total as u64,
                }
            },
        }
    }
}
