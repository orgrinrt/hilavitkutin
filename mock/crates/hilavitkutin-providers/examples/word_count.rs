//! `word_count`: map-reduce demonstration on the hilavitkutin
//! engine.
//!
//! Topic 11 axis B of runtime megaround `202605101036`. Tokenises a
//! corpus into a `Column<Word>` via the substrate interner, runs a
//! `Map` to count per-document occurrences, and a `Reduce` to
//! aggregate across all documents. Exercises:
//!
//! - morsel parallelism over the `Column<Word>`
//! - phase barrier between Map and Reduce
//! - `Resource<Interner>` shared across both WUs
//!
//! Pass 7 ships this file as a structural example demonstrating the
//! call-site shape; Pass 8 + follow-up rounds wire the
//! `Scheduler::run` body that drives the dispatch through to
//! completion.

fn main() {
    // structural example: demonstrates the builder-driven scheduler
    // composition shape. Real dispatch lands when Scheduler::run's
    // body fills in across subsequent rounds.
    //
    // The end-to-end shape consumers will write here:
    //
    //   use hilavitkutin::scheduler::Scheduler;
    //   use hilavitkutin_providers::{InternerKit, AdaptWu};
    //
    //   let mut sched = Scheduler::builder()
    //       .with(InternerKit)
    //       .with(AdaptWu::default())
    //       .with(Map)
    //       .with(Reduce)
    //       .build();
    //   let _: Outcome<(), ()> = sched.run();
    //
    // is the Pass-8-wiring target. The Map and Reduce WorkUnit
    // declarations live in the example app's own scope; here they
    // would be `impl WorkUnit<Always> for Map { type Read = ...;
    // type Write = ...; type Hint = ...; type Ctx<'frame> = ...;
    // fn execute(&self, ctx: &Ctx) { ctx.each().run(|i| ...); } }`.
}
