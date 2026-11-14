//! The skip differential through `run_parallel`, at every worker count.
//!
//! `tests/incremental_skip.rs` holds `run` to the skip: a skipping scheduler
//! and an every-store-dirty scheduler driven through one input history hold
//! equal column bytes after every frame, and the skipping one runs exactly the
//! cone of what each step marked. A unit whose inputs are all clean skips the
//! pass (the spec `:1418-1424`, row `a_unit_whose_inputs_are_all_clean_skips_the_pass`),
//! and the engine is one at every core count (row
//! `the_engine_is_one_and_the_core_count_is_configuration`), so `run_parallel`
//! is held to the same history, the same model and the same cone.
//!
//! The worker counts are 1, 2, 3, 4 and `MAX_CORES`, each through
//! `FixedWorkers`, never the host's count. The fixture has at most two trunk
//! roots in a phase and one window of records, so from two workers up every
//! count places its units alike and walks one path;
//! `tests/run_parallel_wide_differential.rs` holds the windowed, record-less
//! and unit-outer walks over six roots.
//!
//! Today the workers ask no dirty mask, so every unit runs every frame under
//! `run_parallel`. The executed-set test and the unmarked-change twin fail on
//! that and are catalogued under the ticket that tracks it. While the workers
//! skip nothing, the byte comparison passes for a trivial reason, so its
//! control here is a runner known to be wrong: a `run_parallel` whose column
//! is altered by one record after a frame, which the comparison must report.

#[macro_use]
mod skip_chain;
mod common;
mod fixed_workers;
mod stable_provider;

use fixed_workers::FixedWorkers;
use hilavitkutin::thread::MAX_CORES;
use hilavitkutin_api::platform::ThreadPoolApi;
use notko::Outcome;
use skip_chain::{
    Difference,
    Serial,
    Trace,
    bytes_of,
    cone_trace,
    first_bytes_difference,
    first_ran_difference,
    model_trace,
    serial,
};
use stable_provider::stable_provider;

/// Run the skipping scheduler and the every-store-dirty one through the
/// history on `run_parallel` over `pool`, returning both traces. A frame
/// `broken` names has record 0 of `Column<Cd>` altered after it runs, which
/// is the wrong runner the control test needs; `None` runs the engine as it is.
fn traces<P: ThreadPoolApi>(s: &Serial, pool: &P, broken: Option<usize>) -> (Trace, Trace) {
    let skipping = four_chain_scheduler!(s, stable_provider(32768, 0));
    let mut skipping = core::pin::pin!(skipping);
    let skipped = trace!(
        s,
        // SAFETY: the scheduler is not moved out of its pin, and every worker
        // is parked between frames while the step writes through it.
        mut: unsafe { skipping.as_mut().get_unchecked_mut() },
        read: skipping.as_ref().get_ref(),
        frame: skipping.as_mut().run_parallel(pool),
        every: false,
        after: |k| {
            if broken == Some(k) {
                let cd = skipping.as_ref().get_ref().__bindings().__ptr().as_ptr();
                // SAFETY: `Column<Cd>` holds RECORDS reserved records and every
                // worker is parked; this is the deliberate corruption.
                unsafe { (*cd).0 += 1 };
            }
        }
    );
    let every = four_chain_scheduler!(s, stable_provider(32768, 0));
    let mut every = core::pin::pin!(every);
    let all = trace!(
        s,
        // SAFETY: as above.
        mut: unsafe { every.as_mut().get_unchecked_mut() },
        read: every.as_ref().get_ref(),
        frame: every.as_mut().run_parallel(pool),
        every: true
    );
    (skipped, all)
}

/// The bytes the skipping run left against the every-store-dirty run's and
/// the every-store-dirty run's against the model, the first difference of each.
fn bytes_check<P: ThreadPoolApi>(
    s: &Serial,
    pool: &P,
    broken: Option<usize>,
) -> Option<Difference> {
    let (skipped, all) = traces(s, pool, broken);
    first_bytes_difference(&all, &model_trace())
        .or_else(|| first_bytes_difference(&skipped, &bytes_of(&all)))
}

/// Run a check at every worker count, giving each count beside its result, so
/// a test reports every count it fails at and not only the first.
macro_rules! at_every_count {
    (|$pool:ident| $body:expr) => {
        [
            (1, {
                let $pool = FixedWorkers::<1>;
                $body
            }),
            (2, {
                let $pool = FixedWorkers::<2>;
                $body
            }),
            (3, {
                let $pool = FixedWorkers::<3>;
                $body
            }),
            (4, {
                let $pool = FixedWorkers::<4>;
                $body
            }),
            (MAX_CORES, {
                let $pool = FixedWorkers::<MAX_CORES>;
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

#[test]
fn skipping_run_parallel_matches_every_store_dirty_at_every_worker_count() {
    let s = serial();
    let red = counts_failing(at_every_count!(|pool| bytes_check(&s, &pool, None)), None);
    assert!(
        red.is_empty(),
        "the skipping run holds the every-store-dirty run's bytes, and that run the \
         model's, at every worker count; not at (workers, first difference) {red:?}"
    );
}

#[test]
#[ignore = "catalogue: run_parallel runs every unit every pass and never asks dirty_units, so it runs more than the cone of a step; needs the dirty mask published with the frame and gated on by both worker paths; tracked #run_parallel_dispatches_every_unit_each_pass_and_never_asks"]
fn run_parallel_runs_only_the_changed_cone_at_every_worker_count() {
    let s = serial();
    let red = counts_failing(
        at_every_count!(|pool| first_ran_difference(&traces(&s, &pool, None).0, &cone_trace())),
        None,
    );
    assert!(
        red.is_empty(),
        "the skipping run runs the cone of each step and nothing else at every worker \
         count; not at (workers, first difference) {red:?}"
    );
}

#[test]
#[ignore = "catalogue: run_parallel runs every unit every pass and never asks dirty_units, so an input changed without a mark reaches the skipping run as it reaches the every-store-dirty one; needs the dirty mask published with the frame and gated on by both worker paths; tracked #run_parallel_dispatches_every_unit_each_pass_and_never_asks"]
fn skipping_run_parallel_diverges_when_a_mutated_store_is_not_marked() {
    let s = serial();
    let red = counts_failing(
        at_every_count!(|pool| unmarked_change_diverges(&s, &pool)),
        true,
    );
    assert!(
        red.is_empty(),
        "a skipping run keeps the stale cone of an unmarked change at every worker \
         count; not at (workers, diverged) {red:?}"
    );
}

/// Whether a skipping `run_parallel` and an every-store-dirty one differ after
/// the host changes an input without a mark.
fn unmarked_change_diverges<P: ThreadPoolApi>(s: &Serial, pool: &P) -> bool {
    let skipping = four_chain_scheduler!(s, stable_provider(32768, 0));
    let mut skipping = core::pin::pin!(skipping);
    let every = four_chain_scheduler!(s, stable_provider(32768, 0));
    let mut every = core::pin::pin!(every);
    assert!(matches!(
        skipping.as_mut().run_parallel(pool),
        Outcome::Ok(())
    ));
    assert!(matches!(every.as_mut().run_parallel(pool), Outcome::Ok(())));
    // SAFETY: neither scheduler is moved out of its pin, and every worker is
    // parked between frames while the host writes through it.
    let parked = unsafe { skipping.as_mut().get_unchecked_mut() };
    set_inputs!(parked, 7, 200);
    // SAFETY: as above.
    let parked = unsafe { every.as_mut().get_unchecked_mut() };
    set_inputs!(parked, 7, 200);
    mark_every_store!(parked);
    assert!(matches!(
        skipping.as_mut().run_parallel(pool),
        Outcome::Ok(())
    ));
    assert!(matches!(every.as_mut().run_parallel(pool), Outcome::Ok(())));
    snapshot!(skipping.as_ref().get_ref()) != snapshot!(every.as_ref().get_ref())
}

#[test]
fn the_comparison_reports_a_runner_that_altered_one_record() {
    let s = serial();
    // The frame after the first marked change of InA, so the record differs
    // from what the every-store-dirty run left whichever units ran.
    let frame = 3;
    let red = counts_failing(
        at_every_count!(|pool| bytes_check(&s, &pool, Some(frame))),
        Some(Difference::Bytes {
            frame,
            column: 3,
            record: 0,
        }),
    );
    assert!(
        red.is_empty(),
        "the comparison reports the altered record of Column<Cd> at every worker \
         count; not at (workers, what it reported) {red:?}"
    );
}
