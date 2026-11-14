//! The skip differential on every worker path, at every worker count.
//!
//! `tests/run_parallel_skip_differential.rs` holds `run_parallel` to the skip
//! on a four-unit fixture with at most two trunk roots in a phase and one
//! window of records, so it places every unit alike from two workers up and
//! walks one path. This file holds it on the wide fixture of `skip_wide`, six
//! roots in the first phase, built three ways: over several windows of records
//! (the windowed walk, `scheduler/worker.rs` past `:450`), over none (the
//! record-less walk, `:427-448`), and with an accumulator writer (the
//! unit-outer walk, `:124`). A unit whose inputs are all clean skips the pass
//! (the spec `:1418-1424`, row `a_unit_whose_inputs_are_all_clean_skips_the_pass`),
//! and the engine is one at every core count (row
//! `the_engine_is_one_and_the_core_count_is_configuration`).
//!
//! Each build is judged up to four ways. The skipping run holds the
//! every-store-dirty run's stores, and that run the model's, at 1, 2, 3, 4 and
//! `MAX_CORES` workers through `FixedWorkers`. `run` runs exactly the cone of
//! each step, as many times as it ran the unit in the cold frame.
//! `run_parallel` runs each unit in each frame as often as `run` says it
//! should: as often as `run` does on the windowed and record-less walks, where
//! a trunk has one owner walking the same windows, and on the unit-outer walk
//! once on every core whose slice of the N-way tiling is not empty, in a frame
//! where `run` ran it at all. And on the builds with records, a change made
//! without a mark leaves the skipping run behind; a record-less frame leaves
//! nothing a stale cone could show.
//!
//! The accumulator writer is held to its contents, never to the cone: what a
//! skipped accumulator writer does is asked of op
//! (`does_a_skipped_accumulator_writer_run_anyway_in_hilavitkutin`), and the
//! skipping run's accumulator must hold the every-store-dirty run's records,
//! in order, whatever he answers. The producer sharing its fiber is left out
//! of the cone with it.
//!
//! Today the workers ask no dirty mask, so the count and the unmarked-change
//! checks of `run_parallel` fail and are catalogued under the ticket that
//! tracks it, and `run` runs a record-less frame whole, so that build's cone
//! check fails under its own. While the workers skip nothing the store
//! comparison passes for a trivial reason, so its control is a runner known to
//! be wrong.

#[macro_use]
mod skip_wide;
mod common;

use common::FixedWorkers;
use hilavitkutin::thread::MAX_CORES;
use hilavitkutin_api::platform::ThreadPoolApi;
use notko::Outcome;
use skip_wide::{
    ACCUMULATOR,
    Difference,
    RECORDS,
    ROOTS,
    Serial,
    Trace,
    bytes_of,
    cone_trace,
    first_bytes_difference,
    first_count_difference,
    model_trace,
    serial,
};

/// Run a check at every worker count, giving each count beside its result.
macro_rules! at_every_count {
    (|$pool:ident, $n:ident| $body:expr) => {
        [
            (1, {
                let ($pool, $n) = (FixedWorkers::<1>, 1);
                $body
            }),
            (2, {
                let ($pool, $n) = (FixedWorkers::<2>, 2);
                $body
            }),
            (3, {
                let ($pool, $n) = (FixedWorkers::<3>, 3);
                $body
            }),
            (4, {
                let ($pool, $n) = (FixedWorkers::<4>, 4);
                $body
            }),
            (MAX_CORES, {
                let ($pool, $n) = (FixedWorkers::<MAX_CORES>, MAX_CORES);
                $body
            }),
        ]
    };
}

/// The worker counts whose result is not `want`, with the result.
fn counts_failing<T: PartialEq + Copy>(results: [(usize, T); 5], want: T) -> Vec<(usize, T)> {
    results
        .into_iter()
        .filter(|(_, got)| *got != want)
        .collect()
}

/// The cores that walk a unit on the unit-outer walk at `n` workers over
/// `records` records: those whose slice of the N-way tiling is not empty, each
/// slice `ceil(records / n)` long (`resource/bindings/per_core.rs:100-123`).
const fn tiled_cores(records: usize, n: usize) -> u32 {
    if records == 0 {
        return 1;
    }
    let per = records.div_ceil(n);
    records.div_ceil(per) as u32
}

/// The traces, checks and controls of one build of the fixture.
macro_rules! build_checks {
    ($name:ident, $kind:ident, $records:expr, parallel: $parallel:expr, unjudged: $unjudged:expr) => {
        mod $name {
            use super::*;

            /// How many times `run_parallel` at `n` workers runs a unit in a
            /// frame where `run` ran it `base` times.
            pub fn parallel_count(base: u32, n: usize) -> u32 {
                let count: fn(u32, usize) -> u32 = $parallel;
                count(base, n)
            }

            /// The skipping run and the every-store-dirty run through the
            /// history on `run_parallel` over `pool`. A frame `broken` names
            /// has record 0 of the sink altered after it runs, the wrong runner
            /// the control needs.
            pub fn traces<P: ThreadPoolApi>(
                s: &Serial,
                pool: &P,
                broken: Option<usize>,
            ) -> (Trace, Trace) {
                let skipping = wide_scheduler!(s, $records, $kind);
                let mut skipping = core::pin::pin!(skipping);
                let mut frame = 0;
                let skipped = trace!(
                    s, $kind, $records,
                    // SAFETY: the scheduler is not moved out of its pin, and
                    // every worker is parked between frames.
                    mut: unsafe { skipping.as_mut().get_unchecked_mut() },
                    read: skipping.as_ref().get_ref(),
                    frame: {
                        let outcome = skipping.as_mut().run_parallel(pool);
                        if broken == Some(frame) && $records > 0 {
                            let sink = skipping.as_ref().get_ref().__bindings().__ptr().as_ptr();
                            // SAFETY: the sink holds `$records` reserved
                            // records and every worker is parked; this is the
                            // deliberate corruption.
                            unsafe { (*sink).0 += 1 };
                        }
                        frame += 1;
                        outcome
                    },
                    every: false
                );
                let every = wide_scheduler!(s, $records, $kind);
                let mut every = core::pin::pin!(every);
                let all = trace!(
                    s, $kind, $records,
                    // SAFETY: as above.
                    mut: unsafe { every.as_mut().get_unchecked_mut() },
                    read: every.as_ref().get_ref(),
                    frame: every.as_mut().run_parallel(pool),
                    every: true
                );
                (skipped, all)
            }

            /// The skipping run through the history on `run`.
            pub fn run_trace(s: &Serial) -> Trace {
                let mut sched = wide_scheduler!(s, $records, $kind);
                trace!(
                    s, $kind, $records,
                    mut: sched,
                    read: sched,
                    frame: sched.run(),
                    every: false
                )
            }

            /// The first store difference of the skipping run against the
            /// every-store-dirty run, and of that run against the model.
            pub fn stores_check<P: ThreadPoolApi>(
                s: &Serial,
                pool: &P,
                broken: Option<usize>,
            ) -> Option<Difference> {
                let (skipped, all) = traces(s, pool, broken);
                let model = model_trace($records, stringify!($kind) == "accumulator");
                first_bytes_difference(&bytes_of(&all), &model)
                    .or_else(|| first_bytes_difference(&bytes_of(&skipped), &bytes_of(&all)))
            }

            /// The first count of the skipping `run_parallel` at `n` workers
            /// that is not what `run`'s count in that frame says it should be.
            pub fn count_check<P: ThreadPoolApi>(s: &Serial, pool: &P, n: usize) -> Option<Difference> {
                let reference = run_trace(s);
                let (skipped, _) = traces(s, pool, None);
                first_count_difference(&skipped, &reference, |_, _, base| parallel_count(base, n))
            }

            /// The first unit `run` runs a different number of times than the
            /// cone and its cold-frame count say, the unjudged units left out.
            pub fn cone_check(s: &Serial) -> Option<Difference> {
                let got = run_trace(s);
                let cone = cone_trace();
                let cold = got[0].counts;
                first_count_difference(&got, &got, |frame, unit, base| {
                    if $unjudged.contains(&unit) {
                        base
                    } else if cone[frame][unit] {
                        cold[unit]
                    } else {
                        0
                    }
                })
            }

            /// Whether a skipping `run_parallel` and an every-store-dirty one
            /// differ after the host changes every input without a mark.
            pub fn unmarked_change_diverges<P: ThreadPoolApi>(s: &Serial, pool: &P) -> bool {
                let skipping = wide_scheduler!(s, $records, $kind);
                let mut skipping = core::pin::pin!(skipping);
                let every = wide_scheduler!(s, $records, $kind);
                let mut every = core::pin::pin!(every);
                assert!(matches!(skipping.as_mut().run_parallel(pool), Outcome::Ok(())));
                assert!(matches!(every.as_mut().run_parallel(pool), Outcome::Ok(())));
                let changed = [7, 8, 9, 10, 11, 12];
                // SAFETY: neither scheduler is moved out of its pin, and every
                // worker is parked while the host writes through it.
                let parked = unsafe { skipping.as_mut().get_unchecked_mut() };
                set_inputs!(parked, changed);
                // SAFETY: as above.
                let parked = unsafe { every.as_mut().get_unchecked_mut() };
                set_inputs!(parked, changed);
                mark_every_store!(parked);
                assert!(matches!(skipping.as_mut().run_parallel(pool), Outcome::Ok(())));
                assert!(matches!(every.as_mut().run_parallel(pool), Outcome::Ok(())));
                snapshot!(skipping.as_ref().get_ref(), $records, $kind)
                    != snapshot!(every.as_ref().get_ref(), $records, $kind)
            }
        }
    };
}

// A trunk has one owner on the windowed and record-less walks, and the owner
// walks the windows `run` walks.
build_checks!(windowed, plain, RECORDS, parallel: |base, _| base, unjudged: [ACCUMULATOR]);
build_checks!(record_less, plain, 0, parallel: |base, _| base, unjudged: [ACCUMULATOR]);
// The producer of `Out<1>` shares the accumulator writer's fiber, so `run`
// runs it with that fiber in every frame (row
// `a_mixed_carrier_keeps_incremental_skip_per_fiber_and_accumulator_fibers_always_run_the_whole_range`,
// provisional), and it is left unjudged with the writer. On the unit-outer walk
// every core with a slice runs a unit once over it, where `run` runs it once a
// window.
build_checks!(
    unit_outer,
    accumulator,
    RECORDS,
    parallel: |base, n| if base > 0 { tiled_cores(RECORDS, n) } else { 0 },
    unjudged: [ACCUMULATOR, 1]
);

/// One test of one build, by its short name: `stores`, `cone`, `count` or
/// `unmarked`, with whatever attributes the build gives it.
macro_rules! build_test {
    ($name:ident, $(#[$attr:meta])* stores) => {
        #[test]
        $(#[$attr])*
        fn the_skipping_run_holds_the_every_store_dirty_stores_at_every_worker_count() {
            let s = serial();
            let red = counts_failing(
                at_every_count!(|pool, _n| super::super::$name::stores_check(&s, &pool, None)),
                None,
            );
            assert!(
                red.is_empty(),
                "the skipping run holds the every-store-dirty run's stores, and that run \
                 the model's, at every worker count; not at (workers, first difference) {red:?}"
            );
        }
    };
    ($name:ident, $(#[$attr:meta])* cone) => {
        #[test]
        $(#[$attr])*
        fn run_runs_exactly_the_cone_of_each_step() {
            let s = serial();
            let red = super::super::$name::cone_check(&s);
            assert_eq!(red, None, "run runs each unit of the cone as often as in the cold frame and no other");
        }
    };
    ($name:ident, $(#[$attr:meta])* count) => {
        #[test]
        $(#[$attr])*
        fn run_parallel_runs_each_unit_as_often_as_run_does_at_every_worker_count() {
            let s = serial();
            let red = counts_failing(
                at_every_count!(|pool, n| super::super::$name::count_check(&s, &pool, n)),
                None,
            );
            assert!(
                red.is_empty(),
                "run_parallel runs each unit as often as run says it should on its walk, \
                 at every worker count; not at (workers, first difference) {red:?}"
            );
        }
    };
    ($name:ident, $(#[$attr:meta])* unmarked) => {
        #[test]
        $(#[$attr])*
        fn the_skipping_run_diverges_when_a_mutated_store_is_not_marked() {
            let s = serial();
            let red = counts_failing(
                at_every_count!(|pool, _n| super::super::$name::unmarked_change_diverges(&s, &pool)),
                true,
            );
            assert!(
                red.is_empty(),
                "a skipping run keeps the stale cone of an unmarked change at every worker \
                 count; not at (workers, diverged) {red:?}"
            );
        }
    };
}

/// The tests of one build, each named by its short name.
macro_rules! build_tests {
    ($name:ident { $($(#[$attr:meta])* $test:ident),* $(,)? }) => {
        mod $name {
            use super::*;

            $(build_test!($name, $(#[$attr])* $test);)*
        }
    };
}

mod tests {
    use super::*;

    // The workers ask no dirty mask, so `run_parallel` runs units `run`
    // skips, and an unmarked change reaches it as it reaches the
    // every-store-dirty run.
    build_tests!(windowed {
        stores,
        cone,
        #[ignore = "catalogue: run_parallel runs every unit every pass and never asks dirty_units, so it runs units run skips; needs the dirty mask published with the frame and gated on by every worker walk; tracked #run_parallel_dispatches_every_unit_each_pass_and_never_asks"]
        count,
        #[ignore = "catalogue: run_parallel runs every unit every pass and never asks dirty_units, so an input changed without a mark reaches the skipping run as it reaches the every-store-dirty one; needs the dirty mask published with the frame and gated on by every worker walk; tracked #run_parallel_dispatches_every_unit_each_pass_and_never_asks"]
        unmarked,
    });

    // A record-less frame leaves no record a stale cone could show, so it has
    // no unmarked-change test. `run` runs it whole whatever the mask says, and
    // `run_parallel` does the same, so their counts agree today.
    build_tests!(record_less {
        stores,
        #[ignore = "catalogue: run runs the record-less frame whole and never asks dirty_units, so it runs units whose inputs are all clean; needs the record-less walk gated on the dirty mask as the windowed one is; tracked #hilavitkutin_run_skips_nothing_on_a_record_less_frame"]
        cone,
        count,
    });

    build_tests!(unit_outer {
        stores,
        cone,
        #[ignore = "catalogue: run_parallel runs every unit every pass and never asks dirty_units, so it runs units run skips; needs the dirty mask published with the frame and gated on by every worker walk; tracked #run_parallel_dispatches_every_unit_each_pass_and_never_asks"]
        count,
        #[ignore = "catalogue: run_parallel runs every unit every pass and never asks dirty_units, so an input changed without a mark reaches the skipping run as it reaches the every-store-dirty one; needs the dirty mask published with the frame and gated on by every worker walk; tracked #run_parallel_dispatches_every_unit_each_pass_and_never_asks"]
        unmarked,
    });
}

#[test]
fn the_comparison_reports_a_runner_that_altered_one_record() {
    let s = serial();
    // The frame after the first marked change of the consumer's root, so the
    // sink's record differs from what the every-store-dirty run left whichever
    // units ran.
    let frame = 3;
    let red = counts_failing(
        at_every_count!(|pool, _n| windowed::stores_check(&s, &pool, Some(frame))),
        Some(Difference::Bytes {
            frame,
            column: ROOTS,
            record: 0,
        }),
    );
    assert!(
        red.is_empty(),
        "the comparison reports the altered record of the sink at every worker count; \
         not at (workers, what it reported) {red:?}"
    );
}

#[test]
fn the_windowed_build_walks_several_windows_over_six_roots() {
    let s = serial();
    let cold = windowed::run_trace(&s)[0].counts;
    assert!(
        cold[.. ROOTS].iter().all(|&c| c >= 2),
        "every producer runs more than once in the cold frame, one run per window; ran {cold:?}"
    );
    let sched = wide_scheduler!(s, RECORDS, plain);
    assert!(
        sched.__plan_handle().trunk_count().0 == ROOTS,
        "the six producers are six trunks, the consumer in the first one's"
    );
}

#[test]
fn the_unit_outer_tiling_counts_the_cores_with_a_slice() {
    // 1000 records over 256 cores is 4 records a core, so 250 cores walk it.
    assert_eq!(tiled_cores(1000, 256), 250);
    assert_eq!(tiled_cores(1000, 3), 3);
    assert_eq!(tiled_cores(0, 4), 1);
    assert_eq!(record_less::parallel_count(1, 4), 1);
    // The fixture's 13000 records over 256 cores is 51 a core, so 255 cores
    // walk it and one has none.
    assert_eq!(tiled_cores(RECORDS, 256), 255);
    assert_eq!(unit_outer::parallel_count(5, MAX_CORES), 255);
    assert_eq!(unit_outer::parallel_count(0, MAX_CORES), 0);
}
