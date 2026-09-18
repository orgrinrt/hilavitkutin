#![feature(negative_impls)]
pub trait PlanAffecting {}
pub trait Replaceable {}
impl<T: PlanAffecting> !Replaceable for T {}
