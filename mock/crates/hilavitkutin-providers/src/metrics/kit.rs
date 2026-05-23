//! `MetricsKit`: Kit recipe bundling all nine per-axis metrics
//! Resources.
//!
//! Topic 5 axis F. Consumers register the kit via
//! `.add_kit(MetricsKit::default())` to wire every axis's metrics
//! Resource onto the scheduler in one step. Individual axes can also
//! be wired piecemeal via `.add_resource(...)`; the kit is a
//! convenience, not the only path.

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::BuilderInput;
use hilavitkutin_api::store::Resource;
use hilavitkutin_kit::{Kit, KitDispatch};

use super::{
    CacheResidencyMetrics, ChangeClassMetrics, CoreIdleTimeMetrics, FiberEmaMetrics,
    MemoryWatermarkMetrics, PassDurationMetrics, PhaseEmaMetrics, PredictiveParkingMetrics,
    ThroughputMetrics,
};

/// Bundles the nine per-axis metrics Resources as a Kit recipe.
pub struct MetricsKit;

impl Default for MetricsKit {
    fn default() -> Self {
        Self
    }
}

impl BuilderInput for MetricsKit {
    type Init = Self;
    type Dispatch = KitDispatch<Self>;
}

impl Kit for MetricsKit {
    type Units = Empty;
    type Owned = Cons<
        Resource<PassDurationMetrics>,
        Cons<
            Resource<PhaseEmaMetrics>,
            Cons<
                Resource<FiberEmaMetrics>,
                Cons<
                    Resource<ChangeClassMetrics>,
                    Cons<
                        Resource<CacheResidencyMetrics>,
                        Cons<
                            Resource<ThroughputMetrics>,
                            Cons<
                                Resource<PredictiveParkingMetrics>,
                                Cons<
                                    Resource<MemoryWatermarkMetrics>,
                                    Cons<Resource<CoreIdleTimeMetrics>, Empty>,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >;
}
