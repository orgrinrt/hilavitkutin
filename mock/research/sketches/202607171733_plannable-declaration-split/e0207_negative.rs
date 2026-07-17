#![allow(dead_code)]
pub trait AccessSet {} pub struct Empty; impl AccessSet for Empty {}
pub trait Schedulable { type Read: AccessSet; }
pub struct Always;
pub trait WorkUnit<Schedule = Always> { type Read: AccessSet; }
// ATTEMPT: keep Read on WorkUnit and blanket-derive Schedulable, so
// implementors change nothing at all. Does the unconstrained S bite?
impl<W: WorkUnit<S>, S> Schedulable for W { type Read = <W as WorkUnit<S>>::Read; }
fn main() {}
