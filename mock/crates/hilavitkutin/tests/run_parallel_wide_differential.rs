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
//! Each build is judged by what it can show. The skipping run holds the
//! every-store-dirty run's stores, and that run the model's, at 1, 2, 3, 4 and
//! `MAX_CORES` workers through `FixedWorkers`, on the builds with records. `run`
//! and `run_parallel` each run exactly the cone of each step, a unit of it as
//! often as `run` ran it in the cold frame: the engine is one at every core
//! count, and no unit of the fixture declares itself commutative, so no fiber
//! is split across cores (row `ordering_is_guaranteed_by_construction`, the
//! spec `:604-607`). And on the builds with records, a change made without a
//! mark leaves the skipping run behind.
//!
//! The accumulator writer is held to its contents, never to the cone: what a
//! skipped accumulator writer does is asked of op
//! (`does_a_skipped_accumulator_writer_run_anyway_in_hilavitkutin`), and the
//! skipping run's accumulator must hold the every-store-dirty run's records,
//! in order, whatever he answers. The producer sharing its fiber is left out
//! of the counts with it.
//!
//! Today the workers ask no dirty mask, the unit-outer walk tiles a carrier
//! none of whose fibers is commutative, and `run` runs a record-less frame
//! whole, so the count and unmarked-change checks of `run_parallel` and the
//! record-less cone check fail and are catalogued under the tickets that track
//! them. While the workers skip nothing the store comparison passes for a
//! trivial reason, so its controls are runners known to be wrong, one altering
//! the sink and one dropping an accumulator record.

#[macro_use]
mod skip_wide;
mod common;

use common::FixedWorkers;
use hilavitkutin::thread::MAX_CORES;
use hilavitkutin_api::platform::ThreadPoolApi;
use notko::Outcome;
use skip_wide::{
    ACCUMULATOR,
    Counts,
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

/// What a runner known to be wrong alters after the frame it names, for the
/// controls of the store comparison.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Alter {
    /// Record 0 of the sink, one more than the frame wrote.
    Sink,
    /// The accumulator's last live record, dropped.
    LastKept,
}

/// Apply an [`Alter`] to a scheduler whose workers are parked.
macro_rules! alter {
    ($sched:expr, plain, $alter:expr) => {
        match $alter {
            Alter::Sink => alter!(@sink $sched),
            Alter::LastKept => panic!("a plain build has no accumulator"),
        }
    };
    ($sched:expr, accumulator, $alter:expr) => {
        match $alter {
            Alter::Sink => alter!(@sink $sched),
            Alter::LastKept => {
                let b = $sched.__bindings();
                let acc = at!(b; t t t t t t t t t t t t t t);
                let live = acc.__len_cell().get().0;
                acc.__len_cell().set(arvo::USize(live - 1));
            },
        }
    };
    (@sink $sched:expr) => {{
        let sink = $sched.__bindings().__ptr().as_ptr();
        // SAFETY: the sink holds its reserved records and every worker is
        // parked; this is the deliberate corruption.
        unsafe { (*sink).0 += 1 };
    }};
}

/// The traces, checks and controls of one build of the fixture. The units in
/// `unjudged` are held to nothing by the count checks.
macro_rules! build_checks {
    ($name:ident, $kind:ident, $records:expr, unjudged: $unjudged:expr) => {
        mod $name {
            use super::*;

            /// The skipping run and the every-store-dirty run through the
            /// history on `run_parallel` over `pool`. The skipping run is
            /// altered after the frame `broken` names, the wrong runner the
            /// controls need.
            pub fn traces<P: ThreadPoolApi>(
                s: &Serial,
                pool: &P,
                broken: Option<(usize, Alter)>,
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
                        if let Some((at, how)) = broken {
                            if at == frame {
                                alter!(skipping.as_ref().get_ref(), $kind, how);
                            }
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

            /// The first store difference of the every-store-dirty run against
            /// the model, and of the skipping run against it.
            pub fn stores_check<P: ThreadPoolApi>(
                s: &Serial,
                pool: &P,
                broken: Option<(usize, Alter)>,
            ) -> Option<Difference> {
                let (skipped, all) = traces(s, pool, broken);
                let model = model_trace($records, stringify!($kind) == "accumulator");
                first_bytes_difference(&bytes_of(&all), &model)
                    .or_else(|| first_bytes_difference(&bytes_of(&skipped), &bytes_of(&all)))
            }

            /// The first unit and frame of `trace` whose count is not the skip
            /// rule's: a unit in the cone of the step runs as often as in the
            /// cold frame of `run`, `cold`, and a unit outside it not at all.
            fn against_the_cone(trace: &Trace, cold: Counts) -> Option<Difference> {
                let cone = cone_trace();
                first_count_difference(trace, trace, |frame, unit, got| {
                    if $unjudged.contains(&unit) {
                        got
                    } else if cone[frame][unit] {
                        cold[unit]
                    } else {
                        0
                    }
                })
            }

            /// The first count of `run` the skip rule does not give.
            pub fn cone_check(s: &Serial) -> Option<Difference> {
                let got = run_trace(s);
                against_the_cone(&got, got[0].counts)
            }

            /// The first count of the skipping `run_parallel` over `pool` the
            /// skip rule does not give, a unit in the cone running as often as
            /// `run` runs it cold: the engine is one at every core count, and
            /// no fiber of the fixture declares itself commutative, so none is
            /// split across cores.
            pub fn count_check<P: ThreadPoolApi>(s: &Serial, pool: &P) -> Option<Difference> {
                let cold = run_trace(s)[0].counts;
                let (skipped, _) = traces(s, pool, None);
                against_the_cone(&skipped, cold)
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

build_checks!(windowed, plain, RECORDS, unjudged: [ACCUMULATOR]);
build_checks!(record_less, plain, 0, unjudged: [ACCUMULATOR]);
// The producer of `Out<1>` shares the accumulator writer's fiber, so `run`
// runs it with that fiber in every frame (row
// `a_mixed_carrier_keeps_incremental_skip_per_fiber_and_accumulator_fibers_always_run_the_whole_range`,
// provisional), and it is left unjudged with the writer.
build_checks!(unit_outer, accumulator, RECORDS, unjudged: [ACCUMULATOR, 1]);

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
        fn run_parallel_runs_exactly_the_cone_of_each_step_at_every_worker_count() {
            let s = serial();
            let red = counts_failing(
                at_every_count!(|pool, _n| super::super::$name::count_check(&s, &pool)),
                None,
            );
            assert!(
                red.is_empty(),
                "run_parallel runs each unit of the cone as often as run does in the cold \
                 frame and no other, at every worker count; not at (workers, first \
                 difference) {red:?}"
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

    // A record-less frame has no record, so neither a store comparison nor a
    // stale cone has anything to show; the build is judged by its counts.
    build_tests!(record_less {
        #[ignore = "catalogue: run runs the record-less frame whole and never asks dirty_units, so it runs units whose inputs are all clean; needs the record-less walk gated on the dirty mask as the windowed one is; tracked #hilavitkutin_run_skips_nothing_on_a_record_less_frame"]
        cone,
        #[ignore = "catalogue: run_parallel runs every unit every pass and never asks dirty_units, so it runs units whose inputs are all clean; needs the dirty mask published with the frame and gated on by every worker walk; tracked #run_parallel_dispatches_every_unit_each_pass_and_never_asks"]
        count,
    });

    // The accumulator writer puts the carrier on the unit-outer walk, which
    // tiles the whole carrier across the cores although no fiber of it is
    // commutative, and at one worker walks a fiber once over the whole range
    // where `run` walks it once a window (ticket 115,
    // `hilavitkutin_ships_three_run_entries_where_the_unified_engin`); and it
    // skips nothing (157).
    build_tests!(unit_outer {
        stores,
        cone,
        #[ignore = "catalogue: run_parallel tiles a carrier none of whose fibers is commutative across every core, where a non-commutative fiber runs on one core, and walks it over a core's whole slice where run walks it once a window, and asks no dirty mask; needs the tiling gated on commutativity, the one frame body, and the mask on the walk; tracked #the_ungated_n_way_record_split_ships_where_its_own_round_del"]
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
        at_every_count!(|pool, _n| windowed::stores_check(&s, &pool, Some((frame, Alter::Sink)))),
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
fn the_comparison_reports_a_runner_that_dropped_one_accumulator_record() {
    let s = serial();
    let frame = 3;
    // The accumulator is the column past the sink, and a dropped record shows
    // where the shorter one ends.
    let kept = model_trace(RECORDS, true)[frame].accum.len();
    assert!(kept > 0, "the accumulator holds records in frame {frame}");
    let red = counts_failing(
        at_every_count!(|pool, _n| {
            unit_outer::stores_check(&s, &pool, Some((frame, Alter::LastKept)))
        }),
        Some(Difference::Bytes {
            frame,
            column: ROOTS + 1,
            record: kept - 1,
        }),
    );
    assert!(
        red.is_empty(),
        "the comparison reports the dropped record of the accumulator at every worker \
         count; not at (workers, what it reported) {red:?}"
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
