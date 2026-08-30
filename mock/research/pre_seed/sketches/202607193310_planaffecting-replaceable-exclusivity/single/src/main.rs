//! Sketch: can `negative_impls` enforce cross-trait mutual exclusivity
//! between two marker traits on nightly-2026-05-28?
//!
//! Hypothesis: a blanket negative impl `impl<T: PlanAffecting> !Replaceable for T`
//! compiles, and a type implementing both traits is then rejected as an
//! overlapping/conflicting impl. If so, the exclusivity fork from panel memo 01
//! has a structural (type-system) resolution candidate; if the coherence gap
//! (#133556) or a blanket-negative-impl restriction blocks it, the remaining
//! candidates are a supertrait bound or a lint.
#![feature(negative_impls)]

pub trait PlanAffecting {}
pub trait Replaceable {}

// The exclusivity wall under test.
impl<T: PlanAffecting> !Replaceable for T {}

struct OnlyReplaceable;
impl Replaceable for OnlyReplaceable {}

struct OnlyPlanAffecting;
impl PlanAffecting for OnlyPlanAffecting {}

#[cfg(feature = "violate")]
mod violation {
    use super::*;
    struct Both;
    impl PlanAffecting for Both {}
    // EXPECT: rejected (conflicts with the blanket negative impl).
    impl Replaceable for Both {}
}

fn main() {}
