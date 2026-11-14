//! Incremental-skip integration test (domain 16, canonical Step 9, E7).
//!
//! `Scheduler::run` re-runs only the units whose input cone changed since
//! the prior frame; the rest are skipped, producing identical output to
//! running them. The skip is seeded per store: `mark_dirty::<S>()` flags a
//! store the consumer mutated, every unit reading that store is seeded
//! dirty, and the dirty flag propagates forward over the predecessor masks
//! in carrier (topological) sequence so a unit downstream of a dirty unit
//! also re-runs.
//!
//! The units are the four of `skip_chain`: a producer reading `Resource<InA>`,
//! a consumer and a second consumer downstream of it over RAW edges, and an
//! unrelated unit reading `Resource<InC>`. Each counts its own executions, so
//! the count is the direct skip observable.
//!
//! Three frames pin the contract. The cold first frame runs every unit
//! (`first_frame` seeds all dirty). A second frame with no `mark_dirty` finds
//! an empty change seed, so every unit is clean and skipped. A third frame
//! marks only `Resource<InA>`: the producer is seeded, its readers inherit the
//! dirt over their predecessor masks, and the unrelated `InC` unit stays
//! clean.
//!
//! Red first: against a `run` that does not gate (every unit runs every
//! frame), the second frame's counts would move and both post-cold assertions
//! fail. The gate is what makes the clean units skip.
//!
//! The counts alone cannot tell a skip that is correct from one that is stale,
//! so a differential drives a skipping scheduler and an every-store-dirty
//! scheduler through the fixture's history and asserts equal column bytes at
//! every frame, with the every-store-dirty run also checked against a model of
//! the units and the skipping run's executed units against the cone the skip
//! rule gives. A twin changes an input without `mark_dirty` and asserts the two
//! runs differ. `tests/run_parallel_skip_differential.rs` drives the same
//! history through `run_parallel`.
//!
//! Lives under `tests/` so the bare numeric record values and counters do
//! not trip the src-tree primitive lints.

#[macro_use]
mod skip_chain;
mod stable_provider;

use arvo::USize;
use hilavitkutin::scheduler::Scheduler;
use hilavitkutin_api::store::{Column, Resource, StartsFromDefault};
use notko::Outcome;
use skip_chain::{
    Ca,
    Cb,
    Cc,
    ConsumerWu,
    InA,
    InC,
    ProducerWu,
    RECORDS,
    UnrelatedWu,
    bytes_of,
    cone_trace,
    first_bytes_difference,
    first_ran_difference,
    model_trace,
    serial,
    store,
};
use stable_provider::stable_provider;

#[test]
fn incremental_skip_runs_only_the_changed_cone() {
    let s = serial();
    let provider = stable_provider(32768, 0);
    let mut scheduler = Scheduler::builder()
        .with(Resource::new(InA(100)))
        .with(Resource::new(InC(200)))
        .with(Column::<Ca>::start())
        .with(Column::<Cb>::start())
        .with(Column::<Cc>::start())
        .with(ProducerWu)
        .with(ConsumerWu)
        .with(UnrelatedWu)
        .build(store(provider), USize(RECORDS))
        .unwrap_or_else(|_| panic!("build should succeed"));

    // Cold frame: `first_frame` seeds every unit dirty, so all three run. The
    // fourth slot (SecondConsumer) is unused here; this test registers only
    // three units, so it stays zero throughout.
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        s.counts(),
        [1, 1, 1, 0],
        "the cold first frame runs every unit (all seeded dirty)"
    );

    // Clean frame: no `mark_dirty`, so the change seed is empty and every unit
    // is skipped. A non-gating `run` would re-run all three here ([2, 2, 2]).
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        s.counts(),
        [1, 1, 1, 0],
        "a frame with no marked store skips every unit; the counts do not move"
    );

    // Mark the producer's input only. The producer reads InA so it is seeded;
    // the consumer inherits the producer's dirt over its predecessor mask; the
    // unrelated InC unit stays clean and is skipped.
    scheduler.mark_dirty::<Resource<InA>, _>();
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        s.counts(),
        [2, 2, 1, 0],
        "marking InA re-runs the producer and its dependent consumer; the \
         unrelated InC unit stays clean and skipped"
    );
}

#[test]
fn incremental_skip_propagates_two_hops() {
    let s = serial();
    let provider = stable_provider(32768, 0);
    // Producer -> Consumer -> SecondConsumer is a depth-two RAW sequence
    // (InA -> Ca -> Cb -> Cd); the unrelated unit reads InC -> Cc.
    // Registration is topological (each writer before its reader), which
    // `build` requires.
    let mut scheduler = four_chain_scheduler!(s, provider);

    // Cold frame: all four run.
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        s.counts(),
        [1, 1, 1, 1],
        "the cold first frame runs every unit (all seeded dirty)"
    );

    // Clean frame: every unit is skipped.
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        s.counts(),
        [1, 1, 1, 1],
        "a frame with no marked store skips every unit; the counts do not move"
    );

    // Mark only the root input InA. The producer is seeded (reads InA); the
    // consumer inherits over its Ca predecessor mask; the second consumer
    // inherits over its Cb predecessor mask, two hops from the marked root.
    // The unrelated InC unit stays clean. A propagation that stopped at depth
    // one would leave the second consumer at 1 here ([2, 2, 1, 1]); transitive
    // propagation re-runs it ([2, 2, 1, 2]).
    scheduler.mark_dirty::<Resource<InA>, _>();
    assert!(matches!(scheduler.run(), Outcome::Ok(())));
    assert_eq!(
        s.counts(),
        [2, 2, 1, 2],
        "marking InA propagates dirt two hops: producer, consumer, and second \
         consumer all re-run; the unrelated InC unit stays clean and skipped"
    );
}

// Differential on the single-core entry (`run`): a skipping scheduler and an
// every-store-dirty scheduler through the fixture's history, every column
// holding equal bytes after every frame, the every-store-dirty run equal to
// the model, and the skipping run running exactly the cone of each step.
#[test]
fn skipping_run_matches_every_store_dirty_run_at_every_frame() {
    let s = serial();
    let mut skipping = four_chain_scheduler!(s, stable_provider(32768, 0));
    let mut every = four_chain_scheduler!(s, stable_provider(32768, 0));
    let skipped = trace!(s, mut: skipping, read: skipping, frame: skipping.run(), every: false);
    let all = trace!(s, mut: every, read: every, frame: every.run(), every: true);
    assert_eq!(
        first_bytes_difference(&skipped, &bytes_of(&all)),
        None,
        "skipping and every-store-dirty runs must hold equal column bytes"
    );
    // The every-store-dirty run is itself checked against a model of the
    // units, so two runs that are equally stale cannot pass.
    assert_eq!(first_bytes_difference(&all, &model_trace()), None, "model");
    assert_eq!(
        first_ran_difference(&skipped, &cone_trace()),
        None,
        "the skipping run runs the cone of each step and nothing else"
    );
}

#[test]
fn skipping_run_diverges_when_a_mutated_store_is_not_marked() {
    let s = serial();
    let mut skipping = four_chain_scheduler!(s, stable_provider(32768, 0));
    let mut every = four_chain_scheduler!(s, stable_provider(32768, 0));
    assert!(matches!(skipping.run(), Outcome::Ok(())));
    assert!(matches!(every.run(), Outcome::Ok(())));

    // The host changes InA but forgets `mark_dirty`: the skipping run keeps the
    // stale cone, which is what the differential above must be able to see.
    set_inputs!(skipping, 7, 200);
    set_inputs!(every, 7, 200);
    mark_every_store!(every);
    assert!(matches!(skipping.run(), Outcome::Ok(())));
    assert!(matches!(every.run(), Outcome::Ok(())));
    assert_ne!(snapshot!(skipping), snapshot!(every));
}
