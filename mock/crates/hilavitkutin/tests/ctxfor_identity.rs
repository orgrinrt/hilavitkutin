//! P0.2: `CtxFor<'frame, R, W, S>` computes the same nine-parameter `EngineCtx`
//! type a consumer would hand-spell. Type-identity assertions: `IsSame<T>` has a
//! single blanket impl `impl<T> IsSame<T> for T`, so `assert_same::<A, B>()`
//! type-checks only when `A` and `B` are literally the same type. Each case
//! pins `CtxFor` output against the hand-spelled `EngineCtx` form across the
//! store kinds and the three schedule kinds (`Always` / `On<V>` keying the meta
//! pointer to `MetaNil`, `OnMeta<V>` to `MetaRef<'frame>`). Compiling is the
//! whole test: the fold traits and the alias are pure type functions.

#![allow(dead_code)]

use arvo::{Bool, USize};
use hilavitkutin::dispatch::engine_ctx::{
    AccPtrCons, AccPtrNil, ColPtrCons, ColPtrNil, CtxFor, EngineCtx, MetaNil, MetaRef, SnapCons,
    SnapNil, VirtCons, VirtNil,
};
use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::store::{Accum, Column, Resource, Virtual};
use hilavitkutin_api::{Always, On, OnMeta, ScheduleEnd};

// `Tick` is a plain virtual marker for the `On<V>` / write-virtual cases.
struct Tick;

trait IsSame<T: ?Sized> {}
impl<T: ?Sized> IsSame<T> for T {}
fn assert_same<A: ?Sized + IsSame<B>, B: ?Sized>() {}

// Case 1: column read + column write, `Always`. No resource, no accum, no virtual.
type R1 = Cons<Column<USize>, Empty>;
type W1 = Cons<Column<Bool>, Empty>;
type Hand1<'f> = EngineCtx<
    'f,
    R1,
    W1,
    SnapNil,
    ColPtrCons<USize, ColPtrNil>,
    ColPtrCons<Bool, ColPtrNil>,
    AccPtrNil,
    VirtNil,
    MetaNil,
>;

// Case 2: resource + column read, accumulator write, `Always`.
type R2 = Cons<Resource<USize>, Cons<Column<Bool>, Empty>>;
type W2 = Cons<Accum<USize>, Empty>;
type Hand2<'f> = EngineCtx<
    'f,
    R2,
    W2,
    SnapCons<USize, SnapNil>,
    ColPtrCons<Bool, ColPtrNil>,
    ColPtrNil,
    AccPtrCons<'f, USize, AccPtrNil>,
    VirtNil,
    MetaNil,
>;

// Case 3: virtual write, `On<Tick>` schedule. MP stays `MetaNil` for `On<V>`.
type R3 = Empty;
type W3 = Cons<Virtual<Tick>, Empty>;
type Hand3<'f> = EngineCtx<
    'f,
    R3,
    W3,
    SnapNil,
    ColPtrNil,
    ColPtrNil,
    AccPtrNil,
    VirtCons<'f, Tick, VirtNil>,
    MetaNil,
>;

// Case 4: `OnMeta<ScheduleEnd>` keys the meta pointer to `MetaRef<'frame>`.
type Hand4<'f> = EngineCtx<
    'f,
    Empty,
    Empty,
    SnapNil,
    ColPtrNil,
    ColPtrNil,
    AccPtrNil,
    VirtNil,
    MetaRef<'f>,
>;

fn _identity<'f>() {
    assert_same::<CtxFor<'f, R1, W1>, Hand1<'f>>();
    assert_same::<CtxFor<'f, R2, W2>, Hand2<'f>>();
    assert_same::<CtxFor<'f, R3, W3, On<Tick>>, Hand3<'f>>();
    assert_same::<CtxFor<'f, Empty, Empty, OnMeta<ScheduleEnd>>, Hand4<'f>>();
}

#[test]
fn ctxfor_equals_hand_spelled() {
    // The assertions are compile-time; reaching here means every `CtxFor`
    // instantiation resolved to the hand-spelled `EngineCtx` type.
    _identity();
}
