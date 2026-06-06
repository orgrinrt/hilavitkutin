//! Within-fiber linear fusion (domain 17, the deep-single-fiber rust-pipe).
//!
//! A fiber that is a straight read-after-write chain of opt-in per-record maps
//! (`RecordOp`, the api fusion contract) folds at the type level into one fused
//! work unit, `ChainWu`, whose body runs the maps as composed monomorphised
//! calls with every intermediate held in a register. Dead-store elimination
//! under fat LTO removes the intermediate-column memory traffic, so only the
//! chain's final output column is stored: the spec's "pure function pipeline
//! with locals, fiber-internal columns register-to-register". For a four-stage
//! chain this is 3 loads / 3 stores and zero indirect calls, matching a
//! hand-fused loop (proven by sketches `202606091200` + `202606091400`).
//!
//! The fold has three pieces. `OpChain` is the type-level composition of maps
//! (`run_chain` threads the value head to tail through concrete `apply` calls,
//! no function pointers, no `dyn`). `FuseCarrier` folds the scheduler's retained
//! `WuCons` carrier of `RecordOp` units into the matching `OpChain` (two
//! non-overlapping structural impls, so no E0119: single-chain folding does not
//! partition). `ChainWu<C>` wraps an `OpChain` as a normal `WorkUnit` reading
//! the chain's input column and writing its output column.
//!
//! `Scheduler::run_fused` is the dispatch entry: it folds the retained carrier
//! and walks the fused unit through the ordinary `RunFiber` path. The choice
//! between this and the per-WU walk is an explicit entry, not a transparent
//! `run()` auto-detection: the latter is not expressible on the toolchain (the
//! fused projection witness is an unconstrained specializing-impl parameter, and
//! `min_specialization` does not permit specializing on the `FuseCarrier` bound;
//! sketch `202606091800`). `run_fused`'s witness is a method generic inferred at
//! the call site, the same mechanism `run` uses.

use core::marker::PhantomData;

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::builder_input::{BuilderInput, UnitDispatch};
use hilavitkutin_api::column_value::ColumnValue;
use hilavitkutin_api::context::{
    ColumnReaderApi, ColumnWriterApi, EachApi, HasColumnReader, HasColumnWriter, HasEach,
};
use hilavitkutin_api::hint::{Atomic, Immediate, Normal};
use hilavitkutin_api::store::Column;
use hilavitkutin_api::work_unit::{Always, WorkUnit};
use hilavitkutin_api::work_unit_values::{WuCons, WuNil};
use hilavitkutin_api::RecordOp;

use crate::dispatch::engine_ctx::{ColPtrCons, ColPtrNil, EngineCtx, PtrNil};

/// A single-column read / single-column write access set.
type One<T> = Cons<Column<T>, Empty>;

/// A type-level composition of per-record maps: `run_chain` threads a value
/// from the chain's input type to its output type through each map's `apply`,
/// as concrete monomorphised calls.
///
/// `In` and `Out` are `ColumnValue` because the fused `ChainWu` reads the chain
/// input column and writes the chain output column; the links between maps are
/// held in registers, never columns.
pub trait OpChain {
    /// The value the chain consumes (the first map's input).
    type In: ColumnValue;
    /// The value the chain produces (the last map's output).
    type Out: ColumnValue;
    /// Run every map in order, threading the value head to tail.
    fn run_chain(&self, x: Self::In) -> Self::Out;
}

/// Chain terminator: the identity map on `T`. Closes a chain whose last real
/// map produced `T`.
pub struct OpNil<T>(PhantomData<T>);

impl<T> OpNil<T> {
    /// The identity terminator.
    #[inline(always)]
    pub fn new() -> Self {
        OpNil(PhantomData)
    }
}

impl<T> Default for OpNil<T> {
    #[inline(always)]
    fn default() -> Self {
        OpNil(PhantomData)
    }
}

impl<T: ColumnValue> OpChain for OpNil<T> {
    type In = T;
    type Out = T;
    #[inline(always)]
    fn run_chain(&self, x: T) -> T {
        x
    }
}

/// Chain cons cell: a per-record map `head` followed by the rest of the chain
/// `tail`. The tail's input type is bound to the head's output type, so the
/// composition is the internal-column link held in a register.
pub struct OpCons<H, Tl> {
    /// The map at this position.
    pub head: H,
    /// The rest of the chain.
    pub tail: Tl,
}

impl<H, Tl> OpChain for OpCons<H, Tl>
where
    H: RecordOp,
    Tl: OpChain<In = <H as RecordOp>::Out>,
{
    type In = <H as RecordOp>::In;
    type Out = <Tl as OpChain>::Out;
    #[inline(always)]
    fn run_chain(&self, x: Self::In) -> Self::Out {
        self.tail.run_chain(self.head.apply(x))
    }
}

/// Fold a scheduler's retained `WuCons` carrier of `RecordOp` units into the
/// matching `OpChain`.
///
/// Two non-overlapping structural impls: a single-unit carrier folds to a
/// one-map chain terminated by identity, and a multi-unit carrier folds its head
/// onto the folded tail (the recursive link bound `tail Chain: OpChain<In =
/// Head::Out>` is the internal-column type link). Folding a single chain does not
/// partition, so this compiles with no E0119 (unlike type-level fiber grouping).
/// `fuse` takes `&self` (the carrier's unit instances are `Copy`) so the
/// scheduler folds the carrier it holds behind `&mut self`.
pub trait FuseCarrier {
    /// The `OpChain` this carrier folds to.
    type Chain: OpChain;
    /// Fold the carrier into its chain.
    fn fuse(&self) -> Self::Chain;
}

impl<H> FuseCarrier for WuCons<H, WuNil>
where
    H: RecordOp + Copy,
{
    type Chain = OpCons<H, OpNil<<H as RecordOp>::Out>>;
    #[inline(always)]
    fn fuse(&self) -> Self::Chain {
        OpCons { head: self.head, tail: OpNil::new() }
    }
}

impl<H, H2, T> FuseCarrier for WuCons<H, WuCons<H2, T>>
where
    H: RecordOp + Copy,
    WuCons<H2, T>: FuseCarrier,
    <WuCons<H2, T> as FuseCarrier>::Chain: OpChain<In = <H as RecordOp>::Out>,
{
    type Chain = OpCons<H, <WuCons<H2, T> as FuseCarrier>::Chain>;
    #[inline(always)]
    fn fuse(&self) -> Self::Chain {
        OpCons { head: self.head, tail: self.tail.fuse() }
    }
}

/// The fused work unit: a normal `WorkUnit` that reads the chain's input column,
/// runs the chain with intermediates in registers, and writes the chain's output
/// column. The internal columns of the original chain never appear in its access
/// set, so the dispatch projects only the input and output columns and DSE keeps
/// the links register-resident.
pub struct ChainWu<C> {
    chain: C,
}

impl<C> ChainWu<C> {
    /// Wrap a folded chain as a fused work unit.
    #[inline(always)]
    pub fn new(chain: C) -> Self {
        ChainWu { chain }
    }
}

impl<C: Send + Sync + 'static> BuilderInput for ChainWu<C> {
    type Init = Self;
    type Dispatch = UnitDispatch<Self>;
}

impl<C> WorkUnit<Always> for ChainWu<C>
where
    C: OpChain + Send + Sync + 'static,
{
    type Read = One<<C as OpChain>::In>;
    type Write = One<<C as OpChain>::Out>;
    type Hint = (Immediate, Atomic, Normal);
    type Ctx<'frame> = EngineCtx<
        'frame,
        One<<C as OpChain>::In>,
        One<<C as OpChain>::Out>,
        PtrNil,
        ColPtrCons<<C as OpChain>::In, ColPtrNil>,
        ColPtrCons<<C as OpChain>::Out, ColPtrNil>,
    >;
    #[inline]
    fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>) {
        ctx.each().run(|i| {
            // SAFETY: the chain input column is the unit's declared read column,
            // reserved for the record range; the output column is its declared
            // exclusive write column. The morsel covers only reserved records.
            let inp = unsafe { ctx.reader().read::<<C as OpChain>::In, _>(i) };
            let out = self.chain.run_chain(inp);
            unsafe { ctx.writer().write::<<C as OpChain>::Out, _>(i, out) };
        });
    }
}
