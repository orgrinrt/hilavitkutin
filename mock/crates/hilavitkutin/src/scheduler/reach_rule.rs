//! The reach rule, held at `build`.

use core::marker::PhantomData;

use crate::plan::grouping::{BundleMasks, reach_reads_write_nothing};
use crate::plan::{DefaultPlanDims, PlanDims};

/// A unit that reads a plain column with a reach does not write that column.
///
/// A reach read sees the records to either side of the one the unit stands
/// on. Were the unit to write the same column, it would read its own writes of
/// the pass at some offsets and the values of the pass before at others, by how
/// its morsels fall. A buffered column is not held to this, since a reach read
/// of one sees the side the last boundary froze.
///
/// Forcing `HOLDS` fails the build at monomorphisation when a unit of the
/// bundle breaks the rule, and `build` forces it. The failure is a
/// monomorphisation error, so `cargo check` does not report it and
/// `cargo build` does. A unit that smooths a column in place is refused:
///
/// ```compile_fail,E0080
/// use arvo::{Cap, USize};
/// use hilavitkutin::dispatch::engine_ctx::{AccPtrNil, ColPtrCons, ColPtrNil, EngineCtx, SnapNil};
/// use hilavitkutin::scheduler::{NullColumnStorage, Scheduler};
/// use hilavitkutin_api::Reach;
/// use hilavitkutin_api::access::{Cons, Empty};
/// use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
/// use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
/// use hilavitkutin_api::store::Column;
/// use hilavitkutin_api::work_unit::{Always, WorkUnit};
///
/// const ONE: Cap = arvo_tensor::cap(1);
///
/// #[derive(Copy, Clone, PartialEq)]
/// struct Height(u32);
///
/// type Around = Cons<Reach<Column<Height>, ONE>, Empty>;
/// type Same = Cons<Column<Height>, Empty>;
///
/// #[derive(Copy, Clone)]
/// struct Smooth;
///
/// impl BuilderInput for Smooth {
///     type Dispatch = UnitDispatch<Self>;
///     type Init = Self;
/// }
///
/// impl WorkUnit<Always> for Smooth {
///     type Ctx<'frame> = EngineCtx<
///         'frame,
///         Around,
///         Same,
///         SnapNil,
///         ColPtrCons<Height, ColPtrNil>,
///         ColPtrCons<Height, ColPtrNil>,
///         AccPtrNil,
///     >;
///     type Hint = (Immediate, Atomic, Normal);
///     type Read = Around;
///     type Write = Same;
///
///     fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
/// }
///
/// let _ = Scheduler::builder()
///     .with(Column::<Height>::new())
///     .with(Smooth)
///     .build(NullColumnStorage, USize(4));
/// ```
///
/// The same unit writing its result to a second column builds:
///
/// ```
/// use arvo::{Cap, USize};
/// use hilavitkutin::dispatch::engine_ctx::{
///     AccPtrNil,
///     ColPtrCons,
///     ColPtrNil,
///     EngineCtx,
///     SnapNil,
/// };
/// use hilavitkutin::scheduler::{NullColumnStorage, Scheduler};
/// use hilavitkutin_api::Reach;
/// use hilavitkutin_api::access::{Cons, Empty};
/// use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
/// use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
/// use hilavitkutin_api::store::Column;
/// use hilavitkutin_api::work_unit::{Always, WorkUnit};
///
/// const ONE: Cap = arvo_tensor::cap(1);
///
/// #[derive(Copy, Clone, PartialEq)]
/// struct Height(u32);
///
/// #[derive(Copy, Clone, PartialEq)]
/// struct Smoothed(u32);
///
/// type Around = Cons<Reach<Column<Height>, ONE>, Empty>;
/// type Other = Cons<Column<Smoothed>, Empty>;
///
/// #[derive(Copy, Clone)]
/// struct Smooth;
///
/// impl BuilderInput for Smooth {
///     type Dispatch = UnitDispatch<Self>;
///     type Init = Self;
/// }
///
/// impl WorkUnit<Always> for Smooth {
///     type Ctx<'frame> = EngineCtx<
///         'frame,
///         Around,
///         Other,
///         SnapNil,
///         ColPtrCons<Height, ColPtrNil>,
///         ColPtrCons<Smoothed, ColPtrNil>,
///         AccPtrNil,
///     >;
///     type Hint = (Immediate, Atomic, Normal);
///     type Read = Around;
///     type Write = Other;
///
///     fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
/// }
///
/// let _ = Scheduler::builder()
///     .with(Column::<Height>::new())
///     .with(Column::<Smoothed>::new())
///     .with(Smooth)
///     .build(NullColumnStorage, USize(4));
/// ```
pub(super) struct ReachRule<Wus, Stores, Witnesses>(PhantomData<(Wus, Stores, Witnesses)>);

impl<Wus, Stores, Witnesses> ReachRule<Wus, Stores, Witnesses>
where
    Wus: const BundleMasks<Stores, Witnesses, <DefaultPlanDims as PlanDims>::Stores>,
{
    /// Evaluates only when no unit of `Wus` writes a plain column it reads
    /// with a reach.
    pub(super) const HOLDS: () = assert!(
        reach_reads_write_nothing::<
            Wus,
            Stores,
            Witnesses,
            <DefaultPlanDims as PlanDims>::Units,
            <DefaultPlanDims as PlanDims>::Stores,
        >()
        .0,
        "a unit writes a plain column it reads with a reach. Write the result to another column, or make the column a `Buffered<T, M>`, whose reach read sees the values the last pass left."
    );
}
