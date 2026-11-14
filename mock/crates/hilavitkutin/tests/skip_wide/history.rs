//! The input history the wide differential drives, the trace of a run through
//! it, the model it is held to, and the comparisons that judge a trace.

use super::{CONSUMER, Counts, ROOTS, UNITS, seed};

/// The inputs the scheduler is built with.
pub const START: [u32; ROOTS] = [100, 201, 300, 400, 500, 600];

/// One step of the history, applied between two frames: the new value of each
/// input, written in place and marked, or `None` to leave it as it was. A step
/// of `None` everywhere is a clean frame.
pub type Step = [Option<u32>; ROOTS];

const N: Option<u32> = None;

/// The history every wide differential drives, after a cold frame. It changes
/// one root, the root the consumer reads, two roots at once (one of them the
/// root the accumulator writer reads), the same value again under a mark, and
/// every root, with clean frames between.
pub const HISTORY: [Step; 8] = [
    [N, N, N, N, N, N],
    [N, N, N, Some(7), N, N],
    [Some(9), N, N, N, N, N],
    [N, Some(11), N, N, N, Some(13)],
    [N, N, N, N, N, N],
    [Some(9), N, N, N, N, N],
    [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
    [N, N, N, N, N, N],
];

/// The cold frame and one frame per step of [`HISTORY`].
pub const FRAMES: usize = HISTORY.len() + 1;

/// Apply one step. The skipping run marks what the step changed; the
/// every-store-dirty run marks every store as well.
macro_rules! apply_step {
    ($sched:expr, $step:expr, $inputs:expr, every: $every:expr) => {{
        let inputs: &mut [u32; $crate::skip_wide::ROOTS] = $inputs;
        let step: $crate::skip_wide::Step = $step;
        for (k, v) in step.iter().enumerate() {
            if let Some(v) = *v {
                inputs[k] = v;
            }
        }
        if step[0].is_some() {
            $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<0>>, _>();
        }
        if step[1].is_some() {
            $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<1>>, _>();
        }
        if step[2].is_some() {
            $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<2>>, _>();
        }
        if step[3].is_some() {
            $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<3>>, _>();
        }
        if step[4].is_some() {
            $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<4>>, _>();
        }
        if step[5].is_some() {
            $sched.mark_dirty::<hilavitkutin_api::store::Resource<$crate::skip_wide::In<5>>, _>();
        }
        set_inputs!($sched, *inputs);
        if $every {
            mark_every_store!($sched);
        }
    }};
}

/// What one frame left in the stores: the seven columns, `Out<0>` to `Out<5>`
/// then the sink, and the accumulator's live records, empty in a plain build.
#[derive(Clone, Debug, PartialEq)]
pub struct Bytes {
    pub cols:  Vec<Vec<u32>>,
    pub accum: Vec<u32>,
}

/// What one frame left: the stores and how many times each unit ran.
#[derive(Clone, Debug, PartialEq)]
pub struct Seen {
    pub bytes:  Bytes,
    pub counts: Counts,
}

/// Every frame of one run through [`HISTORY`].
pub type Trace = Vec<Seen>;

/// Run a scheduler through the cold frame and [`HISTORY`]. `mut` reaches the
/// scheduler mutably between frames, `read` shared, and `frame` runs one
/// frame; `$kind` and `$records` say what the snapshot reads.
macro_rules! trace {
    ($serial:expr, $kind:ident, $records:expr, mut: $m:expr, read: $r:expr, frame: $f:expr, every: $every:expr) => {{
        let serial: &$crate::skip_wide::Serial = &$serial;
        let mut inputs = $crate::skip_wide::START;
        let mut out = Vec::with_capacity($crate::skip_wide::FRAMES);
        for k in 0 .. $crate::skip_wide::FRAMES {
            if k > 0 {
                apply_step!($m, $crate::skip_wide::HISTORY[k - 1], &mut inputs, every: $every);
            }
            let before = serial.counts();
            let outcome = $f;
            assert!(matches!(outcome, notko::Outcome::Ok(())), "frame {k} did not complete");
            out.push($crate::skip_wide::Seen {
                bytes:  snapshot!($r, $records, $kind),
                counts: serial.counts_since(before),
            });
        }
        out
    }};
}

/// What the units compute from the inputs over `records` records, independent
/// of the engine, with the accumulator's records when `accumulator` is set.
pub fn model(inputs: [u32; ROOTS], records: usize, accumulator: bool) -> Bytes {
    let from = |base: u32| (0 .. records).map(|i| base + seed(i)).collect::<Vec<_>>();
    let mut cols: Vec<Vec<u32>> = inputs.iter().map(|&v| from(v)).collect();
    cols.push(from(inputs[0]));
    let accum = if accumulator {
        from(inputs[1]).into_iter().filter(|v| v % 2 == 1).collect()
    } else {
        Vec::new()
    };
    Bytes {
        cols,
        accum,
    }
}

/// The model's stores after every frame of [`HISTORY`].
pub fn model_trace(records: usize, accumulator: bool) -> Vec<Bytes> {
    let mut inputs = START;
    let mut out = vec![model(inputs, records, accumulator)];
    for step in HISTORY {
        for (k, v) in step.iter().enumerate() {
            if let Some(v) = *v {
                inputs[k] = v;
            }
        }
        out.push(model(inputs, records, accumulator));
    }
    out
}

/// Whether each unit is in the cone of every frame of [`HISTORY`]: every unit
/// in the cold frame, then each producer whose input the step marked, and the
/// consumer with the first producer. A unit whose inputs are all clean skips
/// the pass (the spec `:1418-1424`, row
/// `a_unit_whose_inputs_are_all_clean_skips_the_pass`). The accumulator writer
/// is left out: what a skipped accumulator writer does is asked of op
/// (`does_a_skipped_accumulator_writer_run_anyway_in_hilavitkutin`), and its
/// contents are held to the model instead.
pub fn cone_trace() -> Vec<[bool; UNITS]> {
    let mut out = vec![[true; UNITS]];
    for step in HISTORY {
        let mut cone = [false; UNITS];
        for (k, v) in step.iter().enumerate() {
            cone[k] = v.is_some();
        }
        cone[CONSUMER] = cone[0];
        out.push(cone);
    }
    out
}

/// Where two traces first part.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Difference {
    /// One record of one column, after one frame; column 7 is the accumulator.
    Bytes {
        frame:  usize,
        column: usize,
        record: usize,
    },
    /// How many times one unit ran, in one frame.
    Count {
        frame: usize,
        unit:  usize,
        got:   u32,
        want:  u32,
    },
}

/// The first record whose value in `got` differs from `want`, frame by frame.
/// A column of another length differs at the first record past the shorter.
pub fn first_bytes_difference(got: &[Bytes], want: &[Bytes]) -> Option<Difference> {
    got.iter()
        .zip(want)
        .enumerate()
        .find_map(|(frame, (got, want))| {
            let columns = got
                .cols
                .iter()
                .chain([&got.accum])
                .zip(want.cols.iter().chain([&want.accum]));
            columns.enumerate().find_map(|(column, (g, w))| {
                let record = g
                    .iter()
                    .zip(w)
                    .position(|(g, w)| g != w)
                    .or((g.len() != w.len()).then(|| g.len().min(w.len())))?;
                Some(Difference::Bytes {
                    frame,
                    column,
                    record,
                })
            })
        })
}

/// The first unit and frame whose count in `got` is not `want`, which gives
/// the count it should be from the frame, the unit and the count `base` saw.
pub fn first_count_difference(
    got: &Trace,
    base: &Trace,
    want: impl Fn(usize, usize, u32) -> u32,
) -> Option<Difference> {
    got.iter()
        .zip(base)
        .enumerate()
        .find_map(|(frame, (seen, base))| {
            (0 .. UNITS).find_map(|unit| {
                let want = want(frame, unit, base.counts[unit]);
                let got = seen.counts[unit];
                (got != want).then_some(Difference::Count {
                    frame,
                    unit,
                    got,
                    want,
                })
            })
        })
}

/// The stores of every frame of a trace.
pub fn bytes_of(trace: &Trace) -> Vec<Bytes> {
    trace.iter().map(|seen| seen.bytes.clone()).collect()
}
