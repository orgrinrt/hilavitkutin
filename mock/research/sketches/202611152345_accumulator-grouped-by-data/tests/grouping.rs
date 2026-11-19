//! A fold split across workers, grouped by worker and grouped by the data, on
//! the shipped engine and on the copy whose accumulator slices fall on group
//! boundaries.
//!
//! Two units, each the one writer of an accumulator of partials, each reading a
//! column of values under signed saturating addition:
//!
//! - `PerExec` folds the whole range one `execute` is handed and appends one
//!   partial. The range is the worker's slice, so this is the grouping by
//!   worker, and it is the control.
//! - `PerGroup` folds every `G` records counted from the start of its range and
//!   appends one partial per group, the last one short where the range ends
//!   inside a group. Where every range starts on a multiple of `G`, its groups
//!   are the data's groups, fixed from record zero.
//!
//! After the frame the merged accumulator is read back in its kept sequence,
//! and its partials are folded left to right into the frame's one result. A
//! frame runs on `run_parallel` with 1, 2, 3, 4 and `MAX_CORES` workers, and on
//! `run`, the one-thread entry, twice each so the per-frame reset is crossed.

use accumulator_grouped_by_data::{
    Part,
    Val,
    Workers,
    all_equal,
    bytes_of,
    bytes_of_one,
    combine,
    inputs,
    model_grouped,
    poison,
    zero,
};

/// Records per frame where every worker count's slice is a whole number of
/// groups of eight on the shipped engine: `6144 = 8 * 768`, and 768 is a
/// multiple of 1, 2, 3, 4 and 256.
const ALIGNED_RECORDS: usize = 6144;
/// Records per frame where it is not: 1001 over 2, 3, 4 or 256 workers puts a
/// slice boundary inside a group, and the last group is one record long.
const CUT_RECORDS: usize = 1001;
/// The group width the units of the shipped suite count by. The copy's suite
/// counts by the copy's own constant.
const GROUP: usize = 8;

macro_rules! suite {
    ($name:ident, $eng:ident, $group:expr) => {
        pub mod $name {
            use super::*;
            use arvo::USize;
            use hilavitkutin_api::access::{Cons, Empty};
            use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
            use hilavitkutin_api::context::{
                AccumWriterApi,
                ColumnReaderApi,
                EachApi,
                HasAccumWriter,
                HasColumnReader,
                HasEach,
            };
            use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
            use hilavitkutin_api::store::{Accum, Column};
            use hilavitkutin_api::work_unit::{Always, WorkUnit};
            use hilavitkutin_providers::{ArenaColumnStorage, StaticBumpMemory};
            use notko::Outcome;
            use $eng::dispatch::engine_ctx::{
                AccPtrCons,
                AccPtrNil,
                ColPtrCons,
                ColPtrNil,
                EngineCtx,
                SnapNil,
            };
            use $eng::scheduler::Scheduler;

            pub const MAX_CORES: usize = $eng::thread::MAX_CORES;
            pub const G: usize = $group;

            type In = Cons<Column<Val>, Empty>;
            type Out = Cons<Accum<Part>, Empty>;
            type Hints = (Immediate, Atomic, Normal);
            type Ctx<'frame> = EngineCtx<
                'frame,
                In,
                Out,
                SnapNil,
                ColPtrCons<Val, ColPtrNil>,
                ColPtrNil,
                AccPtrCons<'frame, Part, AccPtrNil>,
            >;

            /// Folds the range one `execute` is handed into one partial.
            pub struct PerExec;
            impl BuilderInput for PerExec {
                type Dispatch = UnitDispatch<Self>;
                type Init = Self;
            }
            impl WorkUnit<Always> for PerExec {
                type Ctx<'frame> = Ctx<'frame>;
                type Hint = Hints;
                type Read = In;
                type Write = Out;

                fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                    let mut acc = zero();
                    let mut seen = false;
                    ctx.each().run(|i| {
                        // SAFETY: the column is seeded for every record, and the
                        // reader maps the range-relative index into it.
                        let v = unsafe { ctx.reader().read::<Val, _>(i) };
                        acc = acc + v.0;
                        seen = true;
                    });
                    if seen {
                        ctx.accums().append::<Part, _>(Part(acc));
                    }
                }
            }

            /// Folds every `G` records from the start of its range into one
            /// partial, and a short last group where the range ends inside one.
            pub struct PerGroup;
            impl BuilderInput for PerGroup {
                type Dispatch = UnitDispatch<Self>;
                type Init = Self;
            }
            impl WorkUnit<Always> for PerGroup {
                type Ctx<'frame> = Ctx<'frame>;
                type Hint = Hints;
                type Read = In;
                type Write = Out;

                fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
                    let mut acc = zero();
                    let mut held = 0;
                    ctx.each().run(|i| {
                        // SAFETY: as in `PerExec`.
                        let v = unsafe { ctx.reader().read::<Val, _>(i) };
                        acc = acc + v.0;
                        held += 1;
                        if held == G {
                            ctx.accums().append::<Part, _>(Part(acc));
                            acc = zero();
                            held = 0;
                        }
                    });
                    if held != 0 {
                        ctx.accums().append::<Part, _>(Part(acc));
                    }
                }
            }

            fn store(n: usize) -> ArenaColumnStorage<StaticBumpMemory> {
                let bytes = 1 << 20;
                let buf = std::vec![core::mem::MaybeUninit::new(0xA5u8); bytes + n * 8].leak();
                ArenaColumnStorage::new(StaticBumpMemory::new(buf))
            }

            // Build a scheduler over `$v` with `$unit`, seed the column with
            // `$v` and fill the accumulator's buffer with the poison value. A
            // macro so the scheduler's type stays inferred.
            macro_rules! build {
                ($unit:expr, $v:expr) => {{
                    let v: &[Val] = $v;
                    let n = v.len();
                    let s = Scheduler::builder()
                        .with(Accum::<Part>::new())
                        .with(Column::<Val>::start_from(Val(zero())))
                        .with($unit)
                        .build(store(n), USize(n))
                        .unwrap_or_else(|_| panic!("build should succeed"));
                    {
                        // Bindings head: the column; its tail: the accumulator.
                        let b = s.__bindings();
                        let col = b.__ptr().as_ptr();
                        let acc = b.__tail().__ptr().as_ptr();
                        for (i, x) in v.iter().enumerate() {
                            // SAFETY: both reserve `n` records; no frame runs.
                            unsafe {
                                col.add(i).write(*x);
                                acc.add(i).write(Part(poison()));
                            }
                        }
                    }
                    s
                }};
            }

            // The kept partials of the accumulator, read from the bindings `$b`.
            macro_rules! kept {
                ($b:expr) => {{
                    let a = $b.__tail();
                    let live = a.__len_cell().get().0;
                    let base = a.__ptr().as_ptr();
                    // SAFETY: the first `live` records are the kept partials.
                    (0 .. live).map(|k| unsafe { base.add(k).read() }).collect::<Vec<Part>>()
                }};
            }

            macro_rules! on_workers {
                ($unit:expr, $v:expr, $w:expr) => {{
                    let s = build!($unit, $v);
                    let mut s = core::pin::pin!(s);
                    let pool = Workers::<{ $w }>;
                    let mut frames: Vec<Vec<Part>> = Vec::new();
                    for _ in 0 .. 2 {
                        let out = s.as_mut().run_parallel(&pool);
                        assert!(matches!(out, Outcome::Ok(())), "run_parallel at {} refused", $w);
                        frames.push(kept!(s.as_ref().get_ref().__bindings()));
                    }
                    assert_eq!(frames[0], frames[1], "two frames at {} workers", $w);
                    frames.pop().unwrap_or_default()
                }};
            }

            macro_rules! on_run {
                ($unit:expr, $v:expr) => {{
                    let mut s = build!($unit, $v);
                    let mut frames: Vec<Vec<Part>> = Vec::new();
                    for _ in 0 .. 2 {
                        assert!(matches!(s.run(), Outcome::Ok(())), "run refused");
                        frames.push(kept!(s.__bindings()));
                    }
                    assert_eq!(frames[0], frames[1], "two frames on run");
                    frames.pop().unwrap_or_default()
                }};
            }

            // One scheduler per call, each in a frame of its own: a scheduler
            // sized for `MAX_CORES` is large, and several in one frame overflow
            // a test thread's stack in a debug build.
            #[inline(never)]
            fn per_exec_at<const W: usize>(v: &[Val]) -> Vec<Part> {
                on_workers!(PerExec, v, W)
            }

            #[inline(never)]
            fn per_group_at<const W: usize>(v: &[Val]) -> Vec<Part> {
                on_workers!(PerGroup, v, W)
            }

            /// The kept partials at 1, 2, 3, 4 and `MAX_CORES` workers.
            pub fn per_exec_counts(v: &[Val]) -> Vec<(usize, Vec<Part>)> {
                vec![
                    (1, per_exec_at::<1>(v)),
                    (2, per_exec_at::<2>(v)),
                    (3, per_exec_at::<3>(v)),
                    (4, per_exec_at::<4>(v)),
                    (MAX_CORES, per_exec_at::<MAX_CORES>(v)),
                ]
            }

            /// The same for the grouping unit.
            pub fn per_group_counts(v: &[Val]) -> Vec<(usize, Vec<Part>)> {
                vec![
                    (1, per_group_at::<1>(v)),
                    (2, per_group_at::<2>(v)),
                    (3, per_group_at::<3>(v)),
                    (4, per_group_at::<4>(v)),
                    (MAX_CORES, per_group_at::<MAX_CORES>(v)),
                ]
            }

            /// The kept partials of the grouping unit on `run`.
            #[inline(never)]
            pub fn per_group_run(v: &[Val]) -> Vec<Part> {
                on_run!(PerGroup, v)
            }

            /// The kept partials of the per-range unit on `run`.
            #[inline(never)]
            pub fn per_exec_run(v: &[Val]) -> Vec<Part> {
                on_run!(PerExec, v)
            }
        }
    };
}

suite!(on_shipped, shipped, GROUP);
suite!(on_aligned, aligned, aligned::ACCUM_GROUP.0);

/// Each count's combined result, as bytes.
fn combined(results: &[(usize, Vec<Part>)]) -> Vec<(usize, Vec<u8>)> {
    results
        .iter()
        .map(|(c, p)| (*c, bytes_of_one(combine(p))))
        .collect()
}

/// Each count's kept partials, as bytes.
fn partials(results: &[(usize, Vec<Part>)]) -> Vec<(usize, Vec<u8>)> {
    results.iter().map(|(c, p)| (*c, bytes_of(p))).collect()
}

fn report(label: &str, results: &[(usize, Vec<Part>)]) {
    for (c, p) in results {
        println!(
            "{label}: workers {c}: {} partials, combined {}",
            p.len(),
            combine(p).to_raw()
        );
    }
}

#[test]
fn the_copy_counts_the_same_group_as_the_shipped_suite() {
    assert_eq!(on_aligned::G, GROUP);
    assert_eq!(on_shipped::MAX_CORES, on_aligned::MAX_CORES);
}

/// The control. Folding per worker and combining gives different bytes at two
/// worker counts on the inputs the grouping by the data is tested on, so an
/// equality there is a property of the grouping and not of the inputs.
#[test]
fn control_grouping_by_worker_differs_across_counts_on_the_shipped_engine() {
    let v = inputs(ALIGNED_RECORDS);
    let r = on_shipped::per_exec_counts(&v);
    report("shipped per worker, 6144", &r);
    let verdict = all_equal(&combined(&r));
    println!("shipped per worker, 6144: {verdict:?}");
    assert!(
        verdict.is_err(),
        "the per-worker grouping must give different bytes at two counts"
    );
    // One worker walks the whole range in one `execute`, as `run` does.
    assert_eq!(r[0].1, on_shipped::per_exec_run(&v));
}

#[test]
fn control_grouping_by_worker_differs_across_counts_on_the_copy() {
    let v = inputs(CUT_RECORDS);
    let r = on_aligned::per_exec_counts(&v);
    report("aligned per worker, 1001", &r);
    let verdict = all_equal(&combined(&r));
    println!("aligned per worker, 1001: {verdict:?}");
    assert!(
        verdict.is_err(),
        "the per-worker grouping must give different bytes at two counts"
    );
}

/// On the shipped engine, where every slice is a whole number of groups, the
/// grouping by the data keeps the same partials at every count, equal to `run`
/// and to the hand model.
#[test]
fn grouping_by_data_is_byte_equal_on_the_shipped_engine_when_slices_fall_on_groups() {
    let v = inputs(ALIGNED_RECORDS);
    let r = on_shipped::per_group_counts(&v);
    report("shipped per group, 6144", &r);
    let model = model_grouped(&v, GROUP);
    assert_eq!(all_equal(&partials(&r)), Ok(()));
    assert_eq!(all_equal(&combined(&r)), Ok(()));
    assert_eq!(r[0].1, model, "the partials are the data's groups");
    assert_eq!(
        on_shipped::per_group_run(&v),
        model,
        "run keeps the same partials"
    );
}

/// On the shipped engine, where a slice boundary falls inside a group, the same
/// unit splits that group in two and the bytes differ between counts.
#[test]
fn grouping_by_data_differs_on_the_shipped_engine_when_a_slice_cuts_a_group() {
    let v = inputs(CUT_RECORDS);
    let r = on_shipped::per_group_counts(&v);
    report("shipped per group, 1001", &r);
    let model = model_grouped(&v, GROUP);
    assert_eq!(r[0].1, model, "one worker keeps the data's groups");
    let verdict = all_equal(&combined(&r));
    println!("shipped per group, 1001: {verdict:?}");
    assert!(
        verdict.is_err(),
        "a slice boundary inside a group must change the bytes"
    );
}

/// On the copy, whose slices are whole groups at every count, the grouping by
/// the data keeps the same partials at every count for record counts that do
/// and do not divide evenly, equal to `run` and to the hand model.
#[test]
fn grouping_by_data_is_byte_equal_on_the_copy_at_every_record_count_tried() {
    for n in [1, 3, 8, 9, 255, 1001, 2049, ALIGNED_RECORDS] {
        let v = inputs(n);
        let r = on_aligned::per_group_counts(&v);
        let model = model_grouped(&v, GROUP);
        if n == CUT_RECORDS || n == ALIGNED_RECORDS {
            report(&format!("aligned per group, {n}"), &r);
        }
        assert_eq!(all_equal(&partials(&r)), Ok(()), "records {n}");
        assert_eq!(all_equal(&combined(&r)), Ok(()), "records {n}");
        assert_eq!(
            r[0].1, model,
            "records {n}: the partials are the data's groups"
        );
        assert_eq!(
            on_aligned::per_group_run(&v),
            model,
            "records {n}: run keeps the same partials"
        );
    }
}
