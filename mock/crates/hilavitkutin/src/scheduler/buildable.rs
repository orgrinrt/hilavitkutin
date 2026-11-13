//! What `build` asks of a registered bundle, under one inferred witness.
//!
//! `build` needs two witness lists: the per-unit projection indices that place
//! each unit's access sets in `Stores`, and the per-column indices that give
//! each column binding its position there. The one impl below takes them as a
//! pair, so the build entries keep a single witness parameter and both halves
//! infer at the call site.

use arvo::USize;

use super::BuildError;
use super::plan_handle::compute_plan;
use super::reach_rule::ReachRule;
use crate::plan::grouping::BundleMasks;
use crate::plan::project::{AccumStoresMask, BundleProject, StoreSizes};
use crate::plan::{DefaultPlanDims, ExecutionPlan, MorselBudget, PlanDims};
use crate::resource::bindings::PlaceIn;

/// A registered bundle `build` can finalise over `Stores` and `Bindings`.
///
/// Implemented once, for every bundle whose units project into `Stores` and
/// whose column bindings find their position there. `Witness` is the pair of
/// witness lists that proves both; a caller leaves it to inference.
pub trait Buildable<Stores, Bindings, Witness> {
    /// Evaluates only when the bundle holds the build-time rules: no unit
    /// writes a plain column it reads with a reach.
    const RULES: ();

    /// Compute the execution plan of the bundle.
    fn plan(
        record_count: USize,
        budget: MorselBudget,
    ) -> notko::Outcome<ExecutionPlan<DefaultPlanDims>, BuildError>;

    /// Give each column binding its position in `Stores`.
    fn place(bindings: &mut Bindings);
}

impl<Wus, Stores, Bindings, UnitWit, PlaceWit> Buildable<Stores, Bindings, (UnitWit, PlaceWit)>
    for Wus
where
    Wus: BundleProject<
            Stores,
            UnitWit,
            <DefaultPlanDims as PlanDims>::Units,
            <DefaultPlanDims as PlanDims>::Stores,
        > + const BundleMasks<Stores, UnitWit, <DefaultPlanDims as PlanDims>::Stores>,
    Stores: AccumStoresMask<<DefaultPlanDims as PlanDims>::Stores>
        + StoreSizes<<DefaultPlanDims as PlanDims>::Stores>,
    Bindings: PlaceIn<Stores, PlaceWit>,
{
    const RULES: () = ReachRule::<Wus, Stores, UnitWit>::HOLDS;

    #[inline]
    fn plan(
        record_count: USize,
        budget: MorselBudget,
    ) -> notko::Outcome<ExecutionPlan<DefaultPlanDims>, BuildError> {
        compute_plan::<Wus, Stores, UnitWit>(record_count, budget)
    }

    #[inline]
    fn place(bindings: &mut Bindings) {
        PlaceIn::<Stores, PlaceWit>::place_in(bindings);
    }
}
