//! `audio_mix`: offline-render audio mixer on the hilavitkutin
//! engine.
//!
//! Topic 11 axis B of runtime megaround `202605101036`. `Column<Sample>`
//! over fixed-point audio frames. Multiple `MixerWu` instances
//! composing in parallel (per-channel processing), one `MasterBusWu`
//! collapsing into the final mix bus. Predictive parking under
//! known phase wait times. Bit-exact reproducibility against the
//! Topic 3 S3 stnp+fence ordering protocol.
//!
//! Pass 7 ships this file as a structural example; the WU bodies
//! and `Scheduler::run` wiring land across subsequent rounds.

fn main() {
    // structural example demonstrating the per-channel parallel
    // mixer + bus-collapse call-site shape. Real dispatch lands
    // when Scheduler::run's body fills in.
    //
    // The end-to-end shape consumers will write here:
    //
    //   let mut sched = Scheduler::builder()
    //       .with(Column::<Sample>::new())
    //       .with(MixerWu::channel(0))
    //       .with(MixerWu::channel(1))
    //       .with(MixerWu::channel(2))
    //       .with(MixerWu::channel(3))
    //       .with(MasterBusWu)
    //       .build();
    //   let _: Outcome<(), ()> = sched.run();
    //
    // is the Pass-8-wiring target. The MixerWu bodies use
    // ctx.batch() over Column<Sample> with arvo `IFixed` fixed-
    // point gain math; MasterBusWu's commutative-write reduce
    // collapses the per-channel buses into the final mix.
}
