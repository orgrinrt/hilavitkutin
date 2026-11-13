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
//! in the engine crate shows a unit shape that runs. Ticket
//! `559df4cb0767` decides whether this becomes a runnable example or is
//! deleted.

fn main() {
    // structural sketch of the parallel-columns + per-WU access-set
    // call-site shape; it runs nothing.
    //
    // The end-to-end shape consumers will write here:
    //
    //   let mut sched = Scheduler::builder()
    //       .with(Column::<Position>::new())
    //       .with(Column::<Velocity>::new())
    //       .with(Column::<Mass>::new())
    //       .with(Column::<CollisionEvent>::new())
    //       .with(Integrate)
    //       .with(Collide)
    //       .build();
    //   let _: Outcome<(), ()> = sched.run();
    //
    // is the Pass-8-wiring target. The Integrate / Collide
    // bodies use ctx.each() / ctx.batch() to traverse the Position
    // / Velocity / Mass / CollisionEvent columns with arvo-typed
    // fixed-point math, head+tail convergent so per-fiber work
    // diverges by collision-density tail.
}
