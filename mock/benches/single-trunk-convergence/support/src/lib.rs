//! Shared code for the single-trunk convergence bench.
//!
//! The question is how one commutative fiber on a single trunk should be
//! spread across cores. The engine today takes the N-way ceil slice: each of
//! `W` cores walks `ceil(n / W)` records forward and the per-core partials
//! merge in core sequence. The design also names head-and-tail convergence:
//! two walkers start at opposite ends of the same fiber, each with its own
//! stack-local accumulator, merged where they meet. The engine carries the
//! head-and-tail types (`RecordRange::{Head, Tail}`, `Convergence`,
//! `ConvergenceBuffer`) but never emits the head and tail ranges, so that
//! path cannot be driven through the scheduler without changing its source.
//! The bench therefore runs head-and-tail on its own crew of persistent
//! workers built from the engine's frame protocol, and runs the N-way slice
//! twice: through the engine, and on the same crew, so the split itself can be
//! compared on equal machinery.
//!
//! Every arm is a function of one [`ConvInput`]: a seed, a workload id and a
//! worker count. The seed builds the data, the workload picks the kernel and
//! the shape (fold or append), the worker count sizes the pool. Each arm's
//! state (data, scheduler or crew, output buffers) is built once per process
//! outside the timed region and reused across calls, the warm regime the
//! engine's schedule-once design runs in.

pub mod arms;
pub mod cache;
pub mod crew;
pub mod data;
pub mod engine;
pub mod kernel;
pub mod split;
pub mod std_par;
pub mod walk;
pub mod workload;

use mockspace_bench_core::Routine;

/// The input that crosses the harness boundary: what to build and how wide
/// to run it. The payload is built from `seed` inside the arm, so its size
/// is the arm's const `N` and never a stack array.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ConvInput {
    /// Seed of the record values.
    pub seed:    u64,
    /// Workload id, a row of [`workload::WORKLOADS`].
    pub kind:    u32,
    /// Worker count the arm runs at, where the arm takes one.
    pub workers: u32,
}

/// What every arm produces, compared byte for byte across arms.
///
/// For a fold, `value` is the wrapping sum of the kernel over every record
/// and `count` is the record count. For an append, `value` is the FNV-1a
/// hash of the appended values in their output sequence and `count` is how
/// many were appended, so a reordered or short output differs.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct ConvOutput {
    pub value: u64,
    pub count: u64,
}

/// The routine every arm runs: one fiber of `N` records.
pub struct Conv<const N: usize>;

impl<const N: usize> Routine for Conv<N> {
    type Input = ConvInput;
    type Output = ConvOutput;

    /// The default input: the first workload at one worker. The orchestrator
    /// replaces the builder per bench section, so this is what a direct
    /// caller of the trait gets.
    fn build_input(seed: u64) -> ConvInput {
        ConvInput {
            seed,
            kind: 0,
            workers: 1,
        }
    }

    fn validate_output(_input: &ConvInput, output: &ConvOutput) -> Result<(), &'static str> {
        if output.count != N as u64 {
            return Err("an arm covered a record count other than N");
        }
        Ok(())
    }

    fn ops_per_call(_input: &ConvInput) -> u64 {
        N as u64
    }
}

/// The input bytes for workload `K` at `W` workers: the builder the
/// orchestrator installs per bench section.
pub fn input_bytes<const K: u32, const W: u32>(seed: u64) -> Vec<u8> {
    let input = ConvInput {
        seed,
        kind: K,
        workers: W,
    };
    // SAFETY: `ConvInput` is `repr(C)`, `Copy`, and holds no padding (u64,
    // u32, u32), so its bytes are a valid image of the value.
    let bytes = unsafe {
        core::slice::from_raw_parts(
            (&input as *const ConvInput).cast::<u8>(),
            core::mem::size_of::<ConvInput>(),
        )
    };
    bytes.to_vec()
}
