//! Engine-side: a block unit registered with the real scheduler.
//!
//! `Blk<U>` is a `WorkUnit` whose context is the engine's computed `CtxFor`
//! over the unit's own lists, and whose `execute` derives the block form's
//! context from it and runs the unit's body. The wrapper exists because a
//! blanket `impl<U: Block> WorkUnit for U` may only be written in the crate
//! that owns `WorkUnit` (E0210, recorded in `check/ui/attempts`), and the api
//! as it stands is not edited by this sketch.
//!
//! `RI` and `WI` are the per-member witnesses the engine's context resolves
//! each column at. Nobody names them: they are inferred where the scheduler is
//! run, as the engine's own projection witnesses are.

#![no_std]

use core::marker::PhantomData;

use derived::{Block, ReadTable, WriteTable};
use hilavitkutin::dispatch::engine_ctx::{
    AccumBundleOf,
    ColBundleOf,
    CtxFor,
    ResourceBundleOf,
    VirtBundleOf,
};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::work_unit::WorkUnit;

/// A block unit, as the engine registers it.
pub struct Blk<U, RI = (), WI = ()>(pub U, PhantomData<fn() -> (RI, WI)>);

impl<U, RI, WI> Blk<U, RI, WI> {
    /// Wrap a block unit for registration.
    pub const fn new(unit: U) -> Self {
        Blk(unit, PhantomData)
    }
}

impl<U: Block, RI: 'static, WI: 'static> BuilderInput for Blk<U, RI, WI> {
    type Dispatch = UnitDispatch<Self>;
    type Init = Self;
}

impl<U, RI, WI> WorkUnit for Blk<U, RI, WI>
where
    U: Block,
    RI: 'static,
    WI: 'static,
    U::Read: ResourceBundleOf + ColBundleOf,
    U::Write: ColBundleOf + for<'f> AccumBundleOf<'f> + for<'f> VirtBundleOf<'f>,
    for<'f> U::Read: ReadTable<CtxFor<'f, U::Read, U::Write>, RI>,
    for<'f> U::Write: WriteTable<U::Write, CtxFor<'f, U::Read, U::Write>, WI>,
{
    type Ctx<'frame> = CtxFor<'frame, U::Read, U::Write>;
    type Hint = U::Hint;
    type Read = U::Read;
    type Write = U::Write;

    #[inline(always)]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        derived::enter::<_, U::Read, U::Write, RI, WI, _>(ctx, |cx| self.0.run(cx));
    }
}
