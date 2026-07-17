extern crate api2; use api2::*;
pub struct ScheduleEnd;
pub struct AdaptWu;                                  // non-Always WorkUnit
impl WorkUnit<On<ScheduleEnd>> for AdaptWu { type Read = Empty; }
pub struct DepthPrepass;                             // GPU pass, not a WorkUnit
impl Schedulable for DepthPrepass { type Read = Empty; }
// the planner takes both, without knowing which is which
fn plan<T: Schedulable<S>, S>() -> core::marker::PhantomData<T::Read> { core::marker::PhantomData }
fn main() {
    let _ = plan::<AdaptWu, On<ScheduleEnd>>();  // via blanket, implementor unchanged
    let _ = plan::<DepthPrepass, Always>();      // hand-written, GPU
}
