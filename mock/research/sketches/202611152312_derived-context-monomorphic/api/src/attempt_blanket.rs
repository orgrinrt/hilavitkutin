//! Attempts at a blanket `WorkUnit` for every block unit, written where
//! `WorkUnit` is defined. Sketch additions behind features no build turns on
//! by default; each is built once and its compiler output kept under
//! `diagnostics/`.
//!
//! The wall these probe is not the blanket itself but what it assigns to
//! `Ctx`: the api cannot name the engine's context, so the context has to come
//! from a type function the api declares and a runner supplies.

use crate::access::AccessSet;
use crate::block::{Block, Records};
use crate::builder_input::BuilderInput;
use crate::context::{
    HasBatch,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
    HasReduce,
    HasResourceProvider,
    HasVirtualFirer,
};
use crate::work_unit::WorkUnit;

/// Attempt b1: the context of the lists `(Self, W)`, as a type function the
/// api declares on the read list and a runner is to implement.
#[cfg(feature = "attempt-blanket")]
pub trait CtxOf<'frame, W: AccessSet>: AccessSet + Sized {
    /// The context a runner hands a unit over these lists.
    type Ctx: Records<Self, W>
        + HasColumnReader<Self>
        + HasColumnWriter<W>
        + HasResourceProvider<Self>
        + HasVirtualFirer<W>
        + HasEach<Self, W>
        + HasBatch<Self, W>
        + HasReduce<Self, W>;
}

#[cfg(feature = "attempt-blanket")]
impl<U> WorkUnit for U
where
    U: Block + BuilderInput<Init = U>,
    U::Read: for<'f> CtxOf<'f, U::Write>,
{
    type Ctx<'frame> = <U::Read as CtxOf<'frame, U::Write>>::Ctx;
    type Hint = U::Hint;
    type Read = U::Read;
    type Write = U::Write;

    #[inline(always)]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        self.run(ctx);
    }
}

/// Attempt b2: the same type function carrying a witness list `I`, as the
/// first sketch's bridge carried `RI` and `WI`.
#[cfg(feature = "attempt-blanket-witness")]
pub trait CtxOfAt<'frame, W: AccessSet, I>: AccessSet + Sized {
    /// The context a runner hands a unit over these lists.
    type Ctx: Records<Self, W>
        + HasColumnReader<Self>
        + HasColumnWriter<W>
        + HasResourceProvider<Self>
        + HasVirtualFirer<W>
        + HasEach<Self, W>
        + HasBatch<Self, W>
        + HasReduce<Self, W>;
}

#[cfg(feature = "attempt-blanket-witness")]
impl<U, I> WorkUnit for U
where
    U: Block + BuilderInput<Init = U>,
    U::Read: for<'f> CtxOfAt<'f, U::Write, I>,
{
    type Ctx<'frame> = <U::Read as CtxOfAt<'frame, U::Write, I>>::Ctx;
    type Hint = U::Hint;
    type Read = U::Read;
    type Write = U::Write;

    #[inline(always)]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        self.run(ctx);
    }
}
