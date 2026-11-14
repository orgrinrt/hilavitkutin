//! The input history the skip differentials drive, the trace of a run through
//! it, and the comparisons that judge a trace.

use super::{CONSUMER, PRODUCER, RECORDS, SECOND_CONSUMER, UNRELATED};

/// One step of the history, applied between two frames.
#[derive(Copy, Clone, Debug)]
pub enum Step {
    /// The new value of each input, written in place and marked, or `None` to
    /// leave it as the previous frame had it. `Set(None, None)` is a clean frame.
    Set(Option<u32>, Option<u32>),
    /// The producer's input replaced through `replace_value`.
    SwapValue(u32),
    /// The unrelated unit's input replaced through `replace_resource`.
    SwapPlan(u32),
}

/// The inputs the scheduler is built with.
pub const START: (u32, u32) = (100, 200);

/// The input history every differential drives, after a cold frame.
pub const HISTORY: [Step; 11] = [
    Step::Set(None, None),
    Step::Set(None, None),
    Step::Set(Some(7), None),
    Step::Set(None, Some(9)),
    Step::Set(Some(11), Some(13)),
    Step::Set(None, None),
    Step::Set(Some(11), None),
    Step::SwapValue(21),
    Step::Set(None, None),
    Step::SwapPlan(23),
    Step::Set(None, None),
];

/// Apply one step. The skipping run marks what the step changed (or lets the
/// swap mark it); the every-store-dirty run marks every store as well.
macro_rules! apply_step {
    ($sched:expr, $step:expr, $inputs:expr, every: $every:expr) => {{
        let inputs: &mut (u32, u32) = $inputs;
        match $step {
            $crate::skip_chain::Step::Set(na, nc) => {
                if let Some(v) = na {
                    inputs.0 = v;
                    $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_chain::InA>, _>();
                }
                if let Some(v) = nc {
                    inputs.1 = v;
                    $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_chain::InC>, _>();
                }
                set_inputs!($sched, inputs.0, inputs.1);
            }
            $crate::skip_chain::Step::SwapValue(v) => {
                inputs.0 = v;
                $sched.replace_value($crate::skip_chain::InA(v));
            }
            $crate::skip_chain::Step::SwapPlan(v) => {
                inputs.1 = v;
                $sched.replace_resource($crate::skip_chain::InC(v));
            }
        }
        if $every {
            mark_every_store!($sched);
        }
    }};
}

/// The four columns' values, one array of records per column, in the sequence
/// Ca, Cb, Cc, Cd.
pub type Bytes = [[u32; RECORDS]; 4];
/// Which units ran, indexed by [`PRODUCER`] and its siblings.
pub type Ran = [bool; 4];
/// The cold frame and one frame per step of [`HISTORY`].
pub const FRAMES: usize = HISTORY.len() + 1;

/// What one frame left: the column bytes and the units that ran.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Seen {
    pub bytes: Bytes,
    pub ran:   Ran,
}

impl Seen {
    /// A frame nothing has been recorded for yet.
    pub const NONE: Self = Self {
        bytes: [[0; RECORDS]; 4],
        ran:   [false; 4],
    };
}

/// Every frame of one run through [`HISTORY`].
pub type Trace = [Seen; FRAMES];

/// Run a scheduler through the cold frame and [`HISTORY`], recording what each
/// frame left. `mut` reaches the scheduler mutably between frames, `read`
/// shared, and `frame` runs one frame. `after: |k| ...` runs after frame `k`'s
/// outcome is checked and before its bytes are read.
macro_rules! trace {
    ($serial:expr, mut: $m:expr, read: $r:expr, frame: $f:expr, every: $every:expr) => {
        trace!($serial, mut: $m, read: $r, frame: $f, every: $every, after: |_k| ())
    };
    ($serial:expr, mut: $m:expr, read: $r:expr, frame: $f:expr, every: $every:expr, after: |$k:ident| $after:expr) => {{
        let serial: &$crate::skip_chain::Serial = &$serial;
        let mut inputs = $crate::skip_chain::START;
        let mut out = [$crate::skip_chain::Seen::NONE; $crate::skip_chain::FRAMES];
        for $k in 0 .. $crate::skip_chain::FRAMES {
            if $k > 0 {
                apply_step!($m, $crate::skip_chain::HISTORY[$k - 1], &mut inputs, every: $every);
            }
            let before = serial.counts();
            let outcome = $f;
            assert!(matches!(outcome, notko::Outcome::Ok(())), "frame {} did not complete", $k);
            $after;
            out[$k] = $crate::skip_chain::Seen {
                bytes: snapshot!($r),
                ran: serial.ran_since(before),
            };
        }
        out
    }};
}

/// Where two records of a run first part.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Difference {
    /// One record of one column, after one frame.
    Bytes {
        frame:  usize,
        column: usize,
        record: usize,
    },
    /// Whether one unit ran, in one frame.
    Ran {
        frame: usize,
        unit:  usize,
    },
}

/// The first record whose bytes in `got` differ from `want`, frame by frame.
pub fn first_bytes_difference(got: &Trace, want: &[Bytes; FRAMES]) -> Option<Difference> {
    got.iter()
        .zip(want)
        .enumerate()
        .find_map(|(frame, (seen, want))| {
            seen.bytes
                .iter()
                .zip(want)
                .enumerate()
                .find_map(|(column, (got, want))| {
                    let record = got.iter().zip(want).position(|(g, w)| g != w)?;
                    Some(Difference::Bytes {
                        frame,
                        column,
                        record,
                    })
                })
        })
}

/// The first unit whose running in `got` differs from `want`, frame by frame.
pub fn first_ran_difference(got: &Trace, want: &[Ran; FRAMES]) -> Option<Difference> {
    got.iter()
        .zip(want)
        .enumerate()
        .find_map(|(frame, (seen, want))| {
            let unit = seen.ran.iter().zip(want).position(|(g, w)| g != w)?;
            Some(Difference::Ran {
                frame,
                unit,
            })
        })
}

/// The bytes of every frame of a trace.
pub fn bytes_of(trace: &Trace) -> [Bytes; FRAMES] {
    trace.map(|seen| seen.bytes)
}

/// What the four units compute from the two inputs, independent of the engine.
pub fn model(a: u32, c: u32) -> Bytes {
    let from = |base: u32| core::array::from_fn(|i| base + i as u32);
    [from(a), from(a), from(c), from(a)]
}

/// The model's bytes after every frame of [`HISTORY`].
pub fn model_trace() -> [Bytes; FRAMES] {
    let mut inputs = START;
    let mut out = [model(inputs.0, inputs.1); FRAMES];
    for (k, step) in HISTORY.iter().enumerate() {
        match *step {
            Step::Set(na, nc) => {
                inputs.0 = na.unwrap_or(inputs.0);
                inputs.1 = nc.unwrap_or(inputs.1);
            },
            Step::SwapValue(v) => inputs.0 = v,
            Step::SwapPlan(v) => inputs.1 = v,
        }
        out[k + 1] = model(inputs.0, inputs.1);
    }
    out
}

/// The units the skip rule runs in every frame of [`HISTORY`]: all four in
/// the cold frame, then the cone of whatever the step marked. A unit whose
/// inputs are all clean skips the pass (the spec `:1418-1424`, row
/// `a_unit_whose_inputs_are_all_clean_skips_the_pass`), and a unit downstream
/// of a dirty one is dirty.
pub fn cone_trace() -> [Ran; FRAMES] {
    let a_cone = {
        let mut r = [false; 4];
        r[PRODUCER] = true;
        r[CONSUMER] = true;
        r[SECOND_CONSUMER] = true;
        r
    };
    let mut out = [[true; 4]; FRAMES];
    for (k, step) in HISTORY.iter().enumerate() {
        let (a, c) = match *step {
            Step::Set(na, nc) => (na.is_some(), nc.is_some()),
            Step::SwapValue(_) => (true, false),
            Step::SwapPlan(_) => (false, true),
        };
        let mut ran = [false; 4];
        if a {
            ran = a_cone;
        }
        ran[UNRELATED] = c;
        out[k + 1] = ran;
    }
    out
}
