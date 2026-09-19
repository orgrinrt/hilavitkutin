//! `EngineCtx` iteration loops (`EachApi` / `BatchApi` / `ReduceApi`) and
//! the eight `HasX` provider impls (the Context is its own provider for
//! every accessor).
//!
//! Split out of `dispatch/engine_ctx.rs` (file-size lint). No behaviour
//! change. Reads `self.morsel`, a still-private `EngineCtx` field: this
//! module is a descendant of `dispatch::engine_ctx`, where the struct is
//! defined, so it reaches it without any visibility widening.

use arvo::USize;
use arvo::strategy::{Additive, Identity};
use hilavitkutin_api::access::AccessSet;
use hilavitkutin_api::context::{
    BatchApi,
    EachApi,
    HasAccumWriter,
    HasBatch,
    HasColumnReader,
    HasColumnWriter,
    HasEach,
    HasReduce,
    HasResourceProvider,
    HasVirtualFirer,
    ReduceApi,
};

use super::EngineCtx;

// EachApi: per-record loop yielding a morsel-relative index `[0, len)`.
// `read` / `write` add `morsel.start` to recover the absolute column index,
// so the body works for any morsel start.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> EachApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    #[inline]
    fn run<F>(&self, mut f: F)
    where
        F: FnMut(USize),
    {
        let mut i = <USize as Identity<Additive>>::IDENTITY;
        let len = self.morsel.len;
        while i.0 < len.0 {
            f(i);
            i = USize(i.0 + 1);
        }
    }
}

// BatchApi: one call with the morsel-relative half-open range `[0, len)`.
// A body looping that range and calling `write(i)` lands at `morsel.start + i`.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> BatchApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    #[inline]
    fn run<F>(&self, mut f: F)
    where
        F: FnMut(USize, USize),
    {
        f(<USize as Identity<Additive>>::IDENTITY, self.morsel.len);
    }
}

// ReduceApi: fold yielding a morsel-relative index `[0, len)`, matching
// `EachApi`. `read` / `write` add `morsel.start` for the absolute index.

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> ReduceApi<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    #[inline]
    fn run<A, F>(&self, init: A, mut f: F) -> A
    where
        A: 'static,
        F: FnMut(A, USize) -> A,
    {
        let mut acc = init;
        let mut i = <USize as Identity<Additive>>::IDENTITY;
        let len = self.morsel.len;
        while i.0 < len.0 {
            acc = f(acc, i);
            i = USize(i.0 + 1);
        }
        acc
    }
}

// ---------------------------------------------------------------------
// HasX accessor impls: the Context is its own provider for every
// accessor (`type Provider = Self`). The seven `HasX` traits come from
// `hilavitkutin-api`'s `provider_generic!` / `provider_generic2!`
// macros; we satisfy them directly here.
// ---------------------------------------------------------------------

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasColumnReader<R> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;

    #[inline(always)]
    fn reader(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasColumnWriter<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;

    #[inline(always)]
    fn writer(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasResourceProvider<R> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;

    #[inline(always)]
    fn resources(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP>
    HasVirtualFirer<W> for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;

    #[inline(always)]
    fn virtuals(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> HasEach<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;

    #[inline(always)]
    fn each(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> HasBatch<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;

    #[inline(always)]
    fn batch(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> HasReduce<R, W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;

    #[inline(always)]
    fn reduce(&self) -> &Self::Provider {
        self
    }
}

impl<'frame, R: AccessSet, W: AccessSet, RBundle, RCols, WCols, WAccum, WVirt, MP> HasAccumWriter<W>
    for EngineCtx<'frame, R, W, RBundle, RCols, WCols, WAccum, WVirt, MP>
{
    type Provider = Self;

    #[inline(always)]
    fn accums(&self) -> &Self::Provider {
        self
    }
}
