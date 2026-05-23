//! `particle_collision`: N-body integrator with collision detection
//! on the hilavitkutin engine.
//!
//! Topic 11 axis B of runtime megaround `202605101036` (user-swapped
//! from `particle_step`). Three parallel columns: `Column<Position>`,
//! `Column<Velocity>`, `Column<Mass>`. `IntegrateWu` advances
//! positions using arvo `UFixed` / `IFixed` fixed-point math.
//! `CollideWu` detects intersections and writes
//! `Column<CollisionEvent>`. Head + tail convergence variant per
//! Topic 3 axis E exercises irregular per-fiber load.
//!
//! Pass 7 ships this file as a structural example; the WU bodies
//! and `Scheduler::run` wiring land across subsequent rounds.

fn main() {
    // structural example demonstrating the parallel-columns +
    // per-WU access-set call-site shape. Real dispatch lands when
    // Scheduler::run's body fills in.
    //
    // The end-to-end shape consumers will write here:
    //
    //   let mut sched = Scheduler::builder()
    //       .with(Column::<Position>::new())
    //       .with(Column::<Velocity>::new())
    //       .with(Column::<Mass>::new())
    //       .with(Column::<CollisionEvent>::new())
    //       .with(IntegrateWu)
    //       .with(CollideWu)
    //       .build();
    //   let _: Outcome<(), ()> = sched.run();
    //
    // is the Pass-8-wiring target. The IntegrateWu / CollideWu
    // bodies use ctx.each() / ctx.batch() to traverse the Position
    // / Velocity / Mass / CollisionEvent columns with arvo-typed
    // fixed-point math, head+tail convergent so per-fiber work
    // diverges by collision-density tail.
}
