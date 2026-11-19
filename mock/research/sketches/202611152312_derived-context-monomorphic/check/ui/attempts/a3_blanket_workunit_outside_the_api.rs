// Attempt 3 of the first sketch, against this sketch's api and engine copies:
// every block unit a `WorkUnit` by one blanket, written outside the crate that
// owns `WorkUnit`, so a block unit registers bare. The context assigned is the
// engine's `CtxFor`, with the bounds the engine copy's `Blk` gives it.

use hilavitkutin::dispatch::engine_ctx::{
    AccumBundleOf,
    ColBundleOf,
    CtxFor,
    ResourceBundleOf,
    VirtBundleOf,
};
use hilavitkutin_api::block::{Block, ColList, ColSlots};
use hilavitkutin_api::work_unit::WorkUnit;

impl<U> WorkUnit for U
where
    U: Block,
    U::Read: ColList + ResourceBundleOf + ColBundleOf,
    U::Write: ColList + ColBundleOf + for<'f> AccumBundleOf<'f> + for<'f> VirtBundleOf<'f>,
    <U::Read as ColBundleOf>::Out: ColSlots<U::Read>,
    <U::Write as ColBundleOf>::Out: ColSlots<U::Write>,
{
    type Ctx<'frame> = CtxFor<'frame, U::Read, U::Write>;
    type Hint = U::Hint;
    type Read = U::Read;
    type Write = U::Write;

    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        self.run(ctx);
    }
}

fn main() {}
