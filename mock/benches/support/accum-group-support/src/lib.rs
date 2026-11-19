//! Shared code for the `accumulator_grouped` bench.
//!
//! The question is what it costs to split a fold that is not associative
//! across cores so that its result is the same bytes at every worker count.
//! Option 3 of the fold split groups the accumulator by the data: the records
//! cut into groups of `G` from record zero, one partial per group, partials
//! kept in group sequence and folded left to right at the merge. Each core's
//! slice is rounded up to whole groups so no slice cuts a group. The engine
//! as shipped takes one partial per core over the ceil slice, whose result
//! moves with the worker count, and that is the baseline the price is read
//! against.
//!
//! This is a model of the split inside the bench tree, on the bench's own
//! crew of parked workers, and changes nothing shipped. The engine-side
//! evidence is the sketch `mock/research/sketches/202611152345_accumulator-
//! grouped-by-data/`, where the same grouping on a copy of the engine with
//! `accum_per` rounded to whole groups gave the same bytes at 1, 2, 3, 4 and
//! 256 workers.
//!
//! Every arm is a function of one [`AccInput`]: a seed, a worker count and a
//! group width. Every output carries its scheme, so validation holds each arm
//! to its own hand model. The arms' results differ from each other by design.

pub mod arms;
pub mod data;
pub mod frame;
pub mod law;
pub mod scheme;
pub mod section;

use mockspace_bench_core::Routine;

use crate::frame::digest_of;
use crate::law::{Sat, left_fold};
use crate::scheme::{
    Grouped,
    GroupedOnCeil,
    ID_GROUPED,
    ID_GROUPED_ON_CEIL,
    ID_PER_CORE,
    ID_SEQUENTIAL,
    PerCore,
    Scheme,
    Sequential,
};

/// The input that crosses the harness boundary.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AccInput {
    /// Seed of the record values.
    pub seed:    u64,
    /// Worker count the arm runs at.
    pub workers: u32,
    /// Records per group, where the scheme groups.
    pub group:   u32,
}

/// What an arm produces: its partials as a digest and a count, and the
/// frame's one result.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct AccOutput {
    /// FNV-1a over the bytes of the partials in their kept sequence.
    pub digest:   u64,
    /// How many partials the frame kept.
    pub parts:    u64,
    /// The raw value of the partials folded left to right.
    pub combined: i64,
    /// The scheme's id, so validation checks the right model.
    pub scheme:   u32,
    /// The worker count the arm ran at.
    pub workers:  u32,
}

impl AccOutput {
    /// The output for `partials` kept by scheme `S` at `workers` workers.
    pub fn of<S: Scheme>(partials: &[Sat], workers: usize) -> Self {
        AccOutput {
            digest:   digest_of(partials),
            parts:    partials.len() as u64,
            combined: i64::from(left_fold(partials).to_raw()),
            scheme:   S::ID,
            workers:  workers as u32,
        }
    }
}

/// The group widths the bench sweeps. A width is a const parameter of the
/// scheme, so a section's width is resolved to its monomorphised path once,
/// outside the timed region.
pub const GROUPS: [u32; 3] = [8, 64, 1024];

/// A visitor over the group width as a type.
pub trait GroupVisitor {
    type Out;
    fn visit<const G: usize>(self) -> Self::Out;
}

/// Resolve `group` to its const width and call the visitor on it.
///
/// # Panics
///
/// On a width [`GROUPS`] does not hold: the hooks build every input from a
/// section of the bench, so an unknown width is a broken section.
pub fn visit_group<V: GroupVisitor>(group: u32, v: V) -> V::Out {
    match group {
        8 => v.visit::<8>(),
        64 => v.visit::<64>(),
        1024 => v.visit::<1024>(),
        other => panic!("group width {other} is not one of {GROUPS:?}"),
    }
}

/// The output scheme `id` must give on `data` at `workers` and `group`, from
/// the scheme's hand model.
pub fn expected(id: u32, data: &[Sat], workers: usize, group: u32) -> Option<AccOutput> {
    struct Model<'a> {
        id:      u32,
        data:    &'a [Sat],
        workers: usize,
    }
    impl GroupVisitor for Model<'_> {
        type Out = Option<AccOutput>;

        fn visit<const G: usize>(self) -> Option<AccOutput> {
            fn out<S: Scheme>(data: &[Sat], workers: usize) -> AccOutput {
                AccOutput::of::<S>(&S::model(data, workers), workers)
            }
            let (d, w) = (self.data, self.workers);
            Some(match self.id {
                ID_SEQUENTIAL => out::<Sequential>(d, w),
                ID_PER_CORE => out::<PerCore>(d, w),
                ID_GROUPED => out::<Grouped<G>>(d, w),
                ID_GROUPED_ON_CEIL => out::<GroupedOnCeil<G>>(d, w),
                _ => return None,
            })
        }
    }
    visit_group(group, Model {
        id,
        data,
        workers,
    })
}

/// The routine every arm runs: one fold over `N` records.
pub struct Acc<const N: usize>;

impl<const N: usize> Routine for Acc<N> {
    type Input = AccInput;
    type Output = AccOutput;

    /// The default input: one worker, the smallest group. The hooks replace
    /// the builder per section, so this is what a direct caller gets.
    fn build_input(seed: u64) -> AccInput {
        AccInput {
            seed,
            workers: 1,
            group: GROUPS[0],
        }
    }

    /// The arm's output against its scheme's hand model, rebuilt from the
    /// seed. The hand model of `grouped` names no worker count, so a pass at
    /// every section's count is the equality across counts.
    fn validate_output(input: &AccInput, output: &AccOutput) -> Result<(), &'static str> {
        if output.workers != input.workers {
            return Err("an arm ran at a worker count other than its section's");
        }
        let data = data::column(input.seed, N);
        match expected(output.scheme, &data, input.workers as usize, input.group) {
            None => Err("an output names no scheme of the bench"),
            Some(model) if model == *output => Ok(()),
            Some(_) => Err("an arm's partials differ from its scheme's hand model"),
        }
    }

    fn ops_per_call(_input: &AccInput) -> u64 {
        N as u64
    }
}

/// The input bytes at `W` workers and groups of `G`: the builder the hooks
/// install per section.
pub fn input_bytes<const W: u32, const G: u32>(seed: u64) -> Vec<u8> {
    let input = AccInput {
        seed,
        workers: W,
        group: G,
    };
    // SAFETY: `AccInput` is `repr(C)`, `Copy`, and holds no padding (u64, u32,
    // u32), so its bytes are a valid image of the value.
    let bytes = unsafe {
        core::slice::from_raw_parts(
            (&input as *const AccInput).cast::<u8>(),
            core::mem::size_of::<AccInput>(),
        )
    };
    bytes.to_vec()
}

/// The input builder for a section at `workers` workers and groups of
/// `group`, or `None` for a pair no section of the bench declares.
pub fn builder(workers: u32, group: u32) -> Option<fn(u64) -> Vec<u8>> {
    macro_rules! by_w {
        ($g:literal) => {
            match workers {
                1 => Some(input_bytes::<1, $g> as fn(u64) -> Vec<u8>),
                2 => Some(input_bytes::<2, $g>),
                3 => Some(input_bytes::<3, $g>),
                4 => Some(input_bytes::<4, $g>),
                _ => None,
            }
        };
    }
    match group {
        8 => by_w!(8),
        64 => by_w!(64),
        1024 => by_w!(1024),
        _ => None,
    }
}
