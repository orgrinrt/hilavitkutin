pub trait AccessSet {} pub struct Empty; impl AccessSet for Empty {}
pub struct Always; pub struct On<T>(core::marker::PhantomData<T>);
// Schedulable carries the schedule, so S is constrained by the trait params.
pub trait Schedulable<Schedule = Always> { type Read: AccessSet; }
pub trait WorkUnit<Schedule = Always> { type Read: AccessSet; }
// ZERO-CHANGE blanket: every WorkUnit<S> is Schedulable<S>, any S.
impl<W: WorkUnit<S>, S> Schedulable<S> for W { type Read = <W as WorkUnit<S>>::Read; }
