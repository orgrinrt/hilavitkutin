//! Adapt-axis Kit recipes.
//!
//! Topic 5 axis A. `StandardAdaptKit` bundles the default-on
//! combination of axes (pass duration + phase EMA + fiber EMA +
//! change class + predictive parking + core idle time); the three
//! axes that are NOT in the default-on set (cache residency,
//! throughput, memory watermark) ship as opt-in `.with(...)`
//! registrations because their sampling cost dominates the
//! information they provide on the typical pipeline shape.
//!
//! `OffAdaptKit` registers nothing; consumers use it to declare
//! "no adapt subsystem" explicitly. Equivalent to omitting any axis
//! `.with(...)` call but composes inside Kit chains that opt out a
//! default-on bundle as a unit.
//!
//! Per Topic 5 axis A "individuals primary, Kits as recipes": the
//! axes are registrable individually via
//! `.with(PassDurationAxis::default())`; the kits exist as
//! ergonomic shortcuts, not as the only path.

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::adapt::{
    ChangeClassAxis, CoreIdleTimeAxis, FiberEmaAxis, PassDurationAxis, PhaseEmaAxis,
    PredictiveParkingAxis,
};
use hilavitkutin_api::builder_input::BuilderInput;
use hilavitkutin_kit::{Kit, KitDispatch};

/// Default-on bundle of adapt axes. Topic 5 axis A.
pub struct StandardAdaptKit;

impl Default for StandardAdaptKit {
    fn default() -> Self {
        Self
    }
}

impl BuilderInput for StandardAdaptKit {
    type Init = Self;
    type Dispatch = KitDispatch<Self>;
}

impl Kit for StandardAdaptKit {
    type Units = Empty;
    type Owned = Cons<
        PassDurationAxis,
        Cons<
            PhaseEmaAxis,
            Cons<
                FiberEmaAxis,
                Cons<
                    ChangeClassAxis,
                    Cons<PredictiveParkingAxis, Cons<CoreIdleTimeAxis, Empty>>,
                >,
            >,
        >,
    >;
}

/// Explicitly empty adapt-axis bundle. Topic 5 axis A.
pub struct OffAdaptKit;

impl Default for OffAdaptKit {
    fn default() -> Self {
        Self
    }
}

impl BuilderInput for OffAdaptKit {
    type Init = Self;
    type Dispatch = KitDispatch<Self>;
}

impl Kit for OffAdaptKit {
    type Units = Empty;
    type Owned = Empty;
}
