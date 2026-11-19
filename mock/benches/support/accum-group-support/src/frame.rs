//! One frame of a scheme on the bench's crew.
//!
//! The state is built once per process and reused across frames: the column,
//! the crew, the region buffer and the per-core lengths. A frame has each
//! walker fold its slice into partials written into its own region (the
//! region starts at the slice's first record and holds at most one partial
//! per record), then, every walker parked, compacts the regions in core
//! sequence and folds the partials left to right. The compaction and the
//! final fold are the merge, and they are inside the timed frame, since the
//! engine's merge is part of the frame too.

use std::cell::UnsafeCell;
use std::marker::PhantomData;

use evs_support::Crew;
use stc_support::engine::Op;

use crate::AccOutput;
use crate::law::{Sat, bytes_of, left_fold, zero};
use crate::scheme::Scheme;

/// One walker's partial count, alone on its cache lines.
#[repr(align(128))]
struct Len(UnsafeCell<usize>);

// SAFETY: walker `c` writes only `lens[c]`, inside a frame; the caller reads
// them only after the frame's done barrier.
unsafe impl Sync for Len {}

/// The region buffer's base, shared with the walkers.
#[derive(Copy, Clone)]
struct Regions(*mut Sat);

// SAFETY: walkers write disjoint regions (each inside its own record slice,
// and the slices of one scheme are disjoint), and the caller reads the buffer
// only after the frame's done barrier.
unsafe impl Send for Regions {}
unsafe impl Sync for Regions {}

impl Regions {
    /// The element `i` places past the base. A method, so a closure captures
    /// the whole value rather than its raw pointer field.
    fn at(self, i: usize) -> *mut Sat {
        // SAFETY: callers stay inside the buffer (see `Frame::run`).
        unsafe { self.0.add(i) }
    }
}

/// A scheme's state between frames.
pub struct Frame<S: Scheme> {
    data:    Vec<Sat>,
    crew:    Crew,
    regions: Vec<Sat>,
    lens:    Box<[Len]>,
    parts:   usize,
    _scheme: PhantomData<S>,
}

impl<S: Scheme> Frame<S> {
    /// The state for `data` at `workers` workers.
    pub fn new(data: Vec<Sat>, workers: usize) -> Self {
        let crew = Crew::new(S::walkers(workers));
        let lens = (0 .. crew.workers())
            .map(|_| Len(UnsafeCell::new(0)))
            .collect();
        let regions = vec![zero(); data.len()];
        Frame {
            data,
            crew,
            regions,
            lens,
            parts: 0,
            _scheme: PhantomData,
        }
    }

    /// The walkers the crew runs, which the crew may have clamped.
    pub fn walkers(&self) -> usize {
        self.crew.workers()
    }

    /// Run one frame: the fold on every walker, then the merge.
    pub fn run(&mut self) -> Sat {
        let n = self.data.len();
        let w = self.crew.workers();
        let base = Regions(self.regions.as_mut_ptr());
        let data = &self.data[..];
        let lens = &self.lens[..];
        let job = |core: usize| {
            let slice = S::slice(core, w, n);
            let mut len = 0usize;
            for run in data[slice.clone()].chunks(S::GROUP) {
                // SAFETY: `len` counts runs of at least one record each, so
                // `slice.start + len` stays inside this walker's slice, which no
                // other walker writes.
                unsafe { *base.at(slice.start + len) = left_fold(run) };
                len += 1;
            }
            // SAFETY: `lens[core]` is this walker's alone (see `Len`).
            unsafe { *lens[core].0.get() = len };
        };
        self.crew.frame(&job);
        // The merge, every walker parked.
        let mut at = 0usize;
        for c in 0 .. w {
            let start = S::slice(c, w, n).start;
            // SAFETY: the frame is over; no walker writes a length.
            let len = unsafe { *self.lens[c].0.get() };
            if start != at {
                self.regions.copy_within(start .. start + len, at);
            }
            at += len;
        }
        self.parts = at;
        left_fold(&self.regions[.. at])
    }

    /// The partials the last frame kept, in sequence.
    pub fn partials(&self) -> &[Sat] {
        &self.regions[.. self.parts]
    }

    /// What the last frame produced, as the routine compares it.
    pub fn output(&self, workers: usize) -> AccOutput {
        AccOutput::of::<S>(self.partials(), workers)
    }

    /// The crew's frame protocol, for the harness's warm regime.
    pub fn op(&mut self, op: Op, workers: usize) -> AccOutput {
        match op {
            Op::Frame => {
                core::hint::black_box(self.run());
                AccOutput::default()
            },
            Op::Read => self.output(workers),
        }
    }
}

/// FNV-1a over `bytes`.
pub fn digest(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xCBF2_9CE4_8422_2325u64, |h, &b| {
        (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01B3)
    })
}

/// The digest of a sequence of partials.
pub fn digest_of(partials: &[Sat]) -> u64 {
    digest(bytes_of(partials))
}
