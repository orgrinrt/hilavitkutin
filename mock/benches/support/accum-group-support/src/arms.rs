//! The arms: each a scheme at the section's group width, timed one frame per
//! call on state built once per process.

use std::marker::PhantomData;

use mockspace_bench_core::{FfiBenchCall, timed_calibrated};
use stc_support::cache;
use stc_support::engine::Op;

use crate::frame::Frame;
use crate::scheme::{Grouped, GroupedOnCeil, PerCore, Scheme, Sequential};
use crate::{AccInput, AccOutput, GroupVisitor, data, visit_group};

/// An arm: the scheme it runs at a group width.
pub trait Arm: 'static {
    type At<const G: usize>: Scheme;
}

/// One walker over every record.
pub struct SequentialArm;
impl Arm for SequentialArm {
    type At<const G: usize> = Sequential;
}

/// The engine's ceil slice, one partial per core.
pub struct PerCoreArm;
impl Arm for PerCoreArm {
    type At<const G: usize> = PerCore;
}

/// Option 3, grouped by the data on slices of whole groups.
pub struct GroupedArm;
impl Arm for GroupedArm {
    type At<const G: usize> = Grouped<G>;
}

/// Groups from each slice's start on the unrounded ceil slice. The control
/// the tests hold to failing; no bench section times it.
pub struct GroupedOnCeilArm;
impl Arm for GroupedOnCeilArm {
    type At<const G: usize> = GroupedOnCeil<G>;
}

/// What identifies an arm's state: everything its construction reads.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Key {
    arm:   &'static str,
    input: AccInput,
    n:     usize,
}

/// Build (once per process) and time one frame of arm `A` on `input` over
/// `n` records, writing the frame's result to `out`.
pub fn call<A: Arm>(input: &AccInput, n: usize, out: &mut AccOutput) -> FfiBenchCall {
    visit_group(input.group, Entry::<A> {
        input: *input,
        n,
        out,
        _arm: PhantomData,
    })
}

/// Build arm `A`'s state for `input` and run one frame, untimed: the tests'
/// entry, through the same state and frame the timed call uses.
pub fn run_once<A: Arm>(input: &AccInput, n: usize) -> AccOutput {
    struct Once<A> {
        input: AccInput,
        n:     usize,
        _arm:  PhantomData<A>,
    }
    impl<A: Arm> GroupVisitor for Once<A> {
        type Out = AccOutput;

        fn visit<const G: usize>(self) -> AccOutput {
            let w = self.input.workers as usize;
            let mut frame = Frame::<A::At<G>>::new(data::column(self.input.seed, self.n), w);
            frame.op(Op::Frame);
            frame.op(Op::Read)
        }
    }
    visit_group(input.group, Once::<A> {
        input: *input,
        n,
        _arm: PhantomData,
    })
}

struct Entry<'a, A> {
    input: AccInput,
    n:     usize,
    out:   &'a mut AccOutput,
    _arm:  PhantomData<A>,
}

impl<A: Arm> GroupVisitor for Entry<'_, A> {
    type Out = FfiBenchCall;

    fn visit<const G: usize>(self) -> FfiBenchCall {
        let key = Key {
            arm:   <A::At<G> as Scheme>::NAME,
            input: self.input,
            n:     self.n,
        };
        let (seed, n, w) = (self.input.seed, self.n, self.input.workers as usize);
        let out = self.out;
        cache::with(
            key,
            move || Frame::<A::At<G>>::new(data::column(seed, n), w),
            |frame| {
                let r = timed_calibrated! {
                    run {
                        core::hint::black_box(frame.op(Op::Frame));
                    }
                };
                *out = frame.op(Op::Read);
                r
            },
        )
    }
}
