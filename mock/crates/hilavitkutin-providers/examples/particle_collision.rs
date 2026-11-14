//! `particle_collision`: N-body integrator with collision detection
//! on the hilavitkutin engine.
//!
//! Topic 11 axis B of runtime megaround `202605101036` (user-swapped
//! from `particle_step`). Three parallel columns: `Column<Position>`,
//! `Column<Velocity>`, `Column<Mass>`. `Integrate` advances
//! positions using arvo `UFixed` / `IFixed` fixed-point math.
//! `Collide` detects intersections and writes
//! `Column<CollisionEvent>`. Head + tail convergence variant per
//! Topic 3 axis E exercises irregular per-fiber load.
//!
//! This file is a structural sketch with an empty `main`: the unit
//! bodies and the run wiring are not written. `Scheduler::run`,
//! `run_fused` and `run_parallel` exist, and `tests/incremental_skip.rs`
//! in the engine crate shows a unit shape that runs.

fn main() {
    // FIXME(placeholders_tagged_559df4cb0767): structural sketch of the parallel-columns +
    // per-WU access-set call-site shape; it runs nothing. Either it becomes
    // a runnable example or it is deleted.
    //
    // The end-to-end shape consumers will write here:
    //
    //   let mut sched = Scheduler::builder()
    //       .with(Column::<Position>::start())
    //       .with(Column::<Velocity>::start())
    //       .with(Column::<Mass>::start())
    //       .with(Column::<CollisionEvent>::start())
    //       .with(Integrate)
    //       .with(Collide)
    //       .build();
    //   let _: Outcome<(), AccumRefused> = sched.run();
    //
    // is the Pass-8-wiring target. The Integrate / Collide
    // bodies use ctx.each() / ctx.batch() to traverse the Position
    // / Velocity / Mass / CollisionEvent columns with arvo-typed
    // fixed-point math, head+tail convergent so per-fiber work
    // diverges by collision-density tail.
}
