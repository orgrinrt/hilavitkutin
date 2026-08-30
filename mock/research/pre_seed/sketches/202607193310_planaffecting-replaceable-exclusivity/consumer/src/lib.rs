use sketch_api::{PlanAffecting, Replaceable};

pub struct AppState;
impl Replaceable for AppState {}

pub struct RunKnobs;
impl PlanAffecting for RunKnobs {}

#[cfg(feature = "violate")]
mod violation {
    use super::*;
    pub struct Both;
    impl PlanAffecting for Both {}
    // EXPECT: rejected cross-crate against the upstream blanket negative impl.
    impl Replaceable for Both {}
}
