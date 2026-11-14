//! Four shapes of a `run_parallel` worker parked between frames, built so Miri
//! can say which of them stays sound across what the engine does between
//! frames.
//!
//! Each arm is a miniature of the engine's scheduler: `!Unpin` and pinned by
//! the caller, a persistent pool of `CORES` workers spawned on the first frame
//! and parked on frame words between frames, a resource slot and an output
//! column in provider memory reached through a bindings struct, a plan-dirty
//! bit, and a per-frame band decision published to the workers. What the
//! engine does on the calling thread is kept: plain field writes during the
//! first frame's setup and after the spawn (`run_parallel.rs:176-213`), a
//! `&mut` reborrow of the bindings at every frame start (`reset_accumulators`,
//! `run_parallel.rs:223-225`), `replace_value` and `replace_resource` through
//! `&mut self` between frames (`scheduler/dirty.rs:49`, `:97`), and a `Drop`
//! that signals shutdown and waits every worker out (`scheduler/mod.rs:370`).
//!
//! The arms differ only in what a worker holds across the park and where what
//! it reads lives:
//!
//! - `today`: a shared reference to the scheduler, formed once and held across
//!   every park, as `worker.rs:101` does.
//! - `narrow`: a raw pointer to the scheduler held across the park, a shared
//!   reference formed per frame after the wake and dropped before the done
//!   arrival.
//! - `plane`: the frame words, the per-worker context and everything a worker
//!   reads moved to a plane the provider allocates; the worker holds a shared
//!   reference to the plane across the park, and the scheduler keeps only what
//!   the calling thread alone touches.
//! - `program`: each core gets its own program, allocated by the provider at
//!   the first frame and immutable after it, holding its record range, its
//!   bindings and a pointer to the shared sync words; the worker holds a shared
//!   reference to its program across the park.

use std::pin::Pin;

/// Workers in every arm.
pub const CORES: usize = 2;
/// Records in the output column; each core writes its half.
pub const RECORDS: usize = 4;

/// What every arm offers the drivers, in the engine's own signatures.
pub trait Arm {
    /// A built scheduler, before its first frame.
    fn build() -> Self;
    /// One frame on the pool: the output column and the band decision as the
    /// workers saw it.
    fn run_parallel(self: Pin<&mut Self>) -> Frame;
    /// Install a new resource value and mark its store dirty.
    fn replace_value(&mut self, v: u32);
    /// Install a new resource value and ask for the plan band.
    fn replace_resource(&mut self, v: u32);
}

/// What a frame left: the output column and how many workers saw the band.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Frame {
    pub out:  [u32; RECORDS],
    pub band: u32,
}

pub mod words;

pub mod on_sched;
pub mod plane;
pub mod program;

/// Today's shape: a shared reference to the scheduler held across every park.
pub type Today = on_sched::Sched<true>;
/// The narrower repair: a raw pointer held across the park, a reference per frame.
pub type Narrow = on_sched::Sched<false>;
