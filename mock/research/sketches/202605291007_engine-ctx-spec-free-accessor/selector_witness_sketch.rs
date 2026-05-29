//! Op-gate sketch for #623: can the EngineCtx type-keyed accessor resolve
//! `T -> &T` WITHOUT `feature(specialization)`, by threading an inferred
//! `Selector` index that resolves at the concrete WU call site?
//!
//! Models the real shape: an AccessSet cons-list + `Contains` marker, the
//! existing spec-free `Selector<T, Index>` index-witness, a projected
//! `PtrCons`/`PtrNil` bundle, an `EngineCtx` that pins a CONCRETE bundle as
//! its `type Ctx`, the `HasResourceProvider -> resources() -> &Provider`
//! indirection, and a consumer "WU" body that calls `resource::<T>()`.
//!
//! The ONLY unstable feature is `marker_trait_attr` (WATCH-tier, the real
//! `Contains` already uses it). There is deliberately NO
//! `feature(specialization)` / `feature(min_specialization)`. If this
//! compiles and the call sites infer the index, the spec-free mechanism is
//! proven; if inference fails, that is the op-gate trip.

#![feature(marker_trait_attr)]
#![allow(dead_code)]

use core::marker::PhantomData;

// --- AccessSet cons-list + Contains marker (mirrors hilavitkutin-api) ---

struct Empty;
struct Cons<H, T>(PhantomData<(H, T)>);

trait AccessSet: 'static {}
impl AccessSet for Empty {}
impl<H: 'static, T: AccessSet> AccessSet for Cons<H, T> {}

struct Resource<T>(PhantomData<T>);

// Contains: #[marker] so the head-match + tail-recurse impls coexist, as in
// the real access.rs. Carries no index (marker traits forbid assoc items),
// which is exactly why the index must come from elsewhere.
#[marker]
trait Contains<S>: AccessSet {}
impl<H: 'static, T: AccessSet> Contains<Resource<H>> for Cons<Resource<H>, T> {}
impl<H: 'static, T: AccessSet, M: 'static> Contains<M> for Cons<H, T> where T: Contains<M> {}

// --- Index witnesses + Selector (the EXISTING spec-free machinery) ---

struct Here;
struct There<I>(PhantomData<I>);

struct PtrNil;
struct PtrCons<H, Tail> {
    head: *const H,
    tail: Tail,
}

trait Selector<T, Index> {
    fn get(&self) -> *const T;
}
impl<T, Tail> Selector<T, Here> for PtrCons<T, Tail> {
    fn get(&self) -> *const T {
        self.head
    }
}
impl<T, U, Tail, I> Selector<T, There<I>> for PtrCons<U, Tail>
where
    Tail: Selector<T, I>,
{
    fn get(&self) -> *const T {
        self.tail.get()
    }
}

// --- The accessor: fixed call shape `resource::<T>()`, index inferred ---
//
// `I` is an extra method generic. Partial turbofish (`resource::<T>()`)
// leaves it to inference. The `Self: ProvideResource<T, I>` bound is what
// the concrete call site discharges by resolving the unique index.

trait ProvideResource<T, I> {
    fn provide(&self) -> *const T;
}

struct EngineCtx<R, RBundle> {
    reads: RBundle,
    _r: PhantomData<R>,
}

// EngineCtx provides T at index I by delegating to its bundle's Selector.
impl<R, RBundle, T, I> ProvideResource<T, I> for EngineCtx<R, RBundle>
where
    RBundle: Selector<T, I>,
{
    fn provide(&self) -> *const T {
        self.reads.get()
    }
}

trait ResourceProviderApi<R: AccessSet> {
    fn resource<T: 'static, I>(&self) -> *const T
    where
        R: Contains<Resource<T>>,
        Self: ProvideResource<T, I>;
}

impl<R: AccessSet, RBundle> ResourceProviderApi<R> for EngineCtx<R, RBundle> {
    fn resource<T: 'static, I>(&self) -> *const T
    where
        R: Contains<Resource<T>>,
        Self: ProvideResource<T, I>,
    {
        <Self as ProvideResource<T, I>>::provide(self)
    }
}

// --- Provider indirection: HasResourceProvider -> resources() -> &Provider ---

trait HasResourceProvider<R: AccessSet> {
    type Provider: ResourceProviderApi<R>;
    fn resources(&self) -> &Self::Provider;
}

impl<R: AccessSet, RBundle> HasResourceProvider<R> for EngineCtx<R, RBundle> {
    type Provider = Self;
    fn resources(&self) -> &Self {
        self
    }
}

// --- Consumer "WU" bodies: pin a CONCRETE Ctx, call resource::<T>() ---

type R1 = Cons<Resource<u32>, Empty>;
type Ctx1 = EngineCtx<R1, PtrCons<u32, PtrNil>>;

// Shape 1: explicit T turbofish, index `_` inferred. `resource::<u32, _>()`.
fn wu_single_explicit(ctx: &Ctx1) -> *const u32 {
    ctx.resources().resource::<u32, _>()
}

// Shape 2: fully inferred. T flows from the binding annotation, I from the
// Selector bound. No turbofish at all. This is the cleanest call shape and
// the closest to the existing `resource::<T>()` ergonomics for sites where
// the type is otherwise constrained.
fn wu_single_inferred(ctx: &Ctx1) -> *const u32 {
    let p: *const u32 = ctx.resources().resource();
    p
}

type R2 = Cons<Resource<u64>, Cons<Resource<u32>, Empty>>;
type Ctx2 = EngineCtx<R2, PtrCons<u64, PtrCons<u32, PtrNil>>>;

// Deeper bundle: u32 is at index `There<Here>`, u64 at `Here`. Both infer.
fn wu_deep(ctx: &Ctx2) -> (*const u64, *const u32) {
    let a = ctx.resources().resource::<u64, _>();
    let b: *const u32 = ctx.resources().resource();
    (a, b)
}

fn main() {
    // Build a concrete Ctx1 over a stack value and exercise the accessor.
    let v: u32 = 7;
    let ctx = EngineCtx::<R1, _> {
        reads: PtrCons {
            head: &v as *const u32,
            tail: PtrNil,
        },
        _r: PhantomData,
    };
    let p = wu_single_explicit(&ctx);
    // SAFETY: p points at the live stack `v` for the duration of this fn.
    assert_eq!(unsafe { *p }, 7);
    let _ = wu_deep;
    println!("spec-free selector-witness accessor: OK");
}
