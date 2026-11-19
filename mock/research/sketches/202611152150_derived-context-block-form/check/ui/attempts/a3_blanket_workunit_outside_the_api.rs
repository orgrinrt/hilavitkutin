// Attempt 3: every block unit a `WorkUnit` by one blanket, written outside
// the crate that owns `WorkUnit`, so a block unit registers bare. The context
// assigned is the engine's `CtxFor`, with the bounds `bridge` gives it.

use derived::Block;
use hilavitkutin::dispatch::engine_ctx::{
    AccumBundleOf,
    ColBundleOf,
    CtxFor,
    ResourceBundleOf,
    VirtBundleOf,
};
use hilavitkutin_api::work_unit::WorkUnit;

impl<U> WorkUnit for U
where
    U: Block,
    U::Read: ResourceBundleOf + ColBundleOf,
    U::Write: ColBundleOf + for<'f> AccumBundleOf<'f> + for<'f> VirtBundleOf<'f>,
{
    type Ctx<'frame> = CtxFor<'frame, U::Read, U::Write>;
    type Hint = U::Hint;
    type Read = U::Read;
    type Write = U::Write;

    fn execute<'frame>(&self, _ctx: &Self::Ctx<'frame>) {}
}

fn main() {}
