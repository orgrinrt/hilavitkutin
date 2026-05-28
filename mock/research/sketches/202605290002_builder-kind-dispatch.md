# Sketch — single-`.with` store-value routing via RouterKind dispatch

**Hypothesis:** the scheduler builder can keep its single unified `.with(value)` verb (per `DESIGN.md`) while retaining ONLY store-registration values on a `Stores`-aligned value list, routing store-vs-nonstore at the type level without the overlapping-impl / coherence error that a heterogeneous-list search (`FindCarrier`) hits, and without specialization, on stable `no_std` Rust.

**Outcome: WORKS.** Validated by a standalone stable-Rust crate (no `#![feature]`, no alloc, no specialization). The test builds `Builder::new().with(StagedResource(42u32)).with(MyWu).with(StagedResource(7u32)).with(MyMem)` and the resulting store-value list holds exactly the two store values in registration order; the workunit and platform values are dropped. Compile-time length assertion confirms the list shape.

This unblocks B2: it lets the value drain walk a `Stores`-aligned store-value list in lockstep (no search), so the arena build is coherence-clean. It also means the architect's recommended `.with`/`.store` method split (and its fifth builder parameter justification) is rejected: the unified verb survives.

## Why the naive approach fails

Matching a typed carrier (`StagedResource<T>`) on the heterogeneous staged list at drain time needs a recursive extraction method: a head-match impl (`for Stage<StagedResource<T>, Tail>`) and a tail-recurse impl (`for Stage<H, Tail> where Tail: Find`). These overlap when `H = StagedResource<T>` and are incoherent without specialization. Type-level membership (`Contains`) escapes this via `#[marker]` (no methods), but extraction needs a method, so `#[marker]` does not apply.

## The mechanism that works

Each `Dispatch` router carries an associated kind tag; the kind-conditional next-list computation lives on a `Place<P>` GAT keyed on the tag as `Self` (three disjoint `Self` types, so coherence sees no overlap). The builder's single `.with` return type projects through it.

```rust
mod sealed { pub trait Sealed {} }
pub trait StoreValues: sealed::Sealed {}
pub struct SvEmpty;
pub struct Sv<H, T: StoreValues> { head: H, tail: T }
impl sealed::Sealed for SvEmpty {}
impl<H, T: StoreValues> sealed::Sealed for Sv<H, T> {}
impl StoreValues for SvEmpty {}
impl<H, T: StoreValues> StoreValues for Sv<H, T> {}

pub struct StoreKind; pub struct UnitKind; pub struct PlatformKind;

pub struct StoreDispatch<S>(core::marker::PhantomData<S>);
pub struct UnitDispatch<W>(core::marker::PhantomData<W>);
pub struct PlatformDispatch<P>(core::marker::PhantomData<P>);

pub trait RouterKind { type Kind; }
impl<S> RouterKind for StoreDispatch<S> { type Kind = StoreKind; }
impl<W> RouterKind for UnitDispatch<W> { type Kind = UnitKind; }
impl<P> RouterKind for PlatformDispatch<P> { type Kind = PlatformKind; }

pub trait BuilderInput { type Dispatch: RouterKind; }

// Non-overlapping impls keyed on the kind tag (Self); P is the provider value.
pub trait Place<P> {
    type Next<L: StoreValues>: StoreValues;
    fn place<L: StoreValues>(provider: P, sv: L) -> Self::Next<L>;
}
impl<P> Place<P> for StoreKind {
    type Next<L: StoreValues> = Sv<P, L>;
    fn place<L: StoreValues>(provider: P, sv: L) -> Self::Next<L> { Sv { head: provider, tail: sv } }
}
impl<P> Place<P> for UnitKind {
    type Next<L: StoreValues> = L;
    fn place<L: StoreValues>(_p: P, sv: L) -> Self::Next<L> { sv }
}
impl<P> Place<P> for PlatformKind {
    type Next<L: StoreValues> = L;
    fn place<L: StoreValues>(_p: P, sv: L) -> Self::Next<L> { sv }
}

type KindOf<P> = <<P as BuilderInput>::Dispatch as RouterKind>::Kind;

pub struct Builder<L: StoreValues> { sv: L }
impl Builder<SvEmpty> { pub fn new() -> Self { Builder { sv: SvEmpty } } }
impl<Cur: StoreValues> Builder<Cur> {
    pub fn with<P>(self, provider: P) -> Builder<<KindOf<P> as Place<P>>::Next<Cur>>
    where P: BuilderInput, KindOf<P>: Place<P> {
        Builder { sv: <KindOf<P> as Place<P>>::place(provider, self.sv) }
    }
}
```

The only adjustment from the first draft was renaming the GAT parameter off `Sv` (it shadowed the `Sv<H, T>` struct); call it `L`. No structural change. GATs on stable handled `Next<L>` without issue.

## How this maps onto B2

- The real `BuilderInput` already names `type Dispatch`; add `RouterKind` impls for the existing `StoreDispatch` / `UnitDispatch` / `PlatformDispatch` / `RunCfgDispatch` / `KitDispatch` routers (Kit routes its `Owned` stores; settle Kit's kind when Kit value-retention is needed, likely UnitKind-like drop for B2 since Kit-owned resource values come from `HasTrivialCtor`, not a carrier).
- The B1 `Staged: StageList` builder parameter is replaced by a `StoreValues` parameter populated via this `Place` dispatch (store carriers retained in `Stores` order; workunit and platform values dropped, their TYPE still tracked in `Wus`/`Platform`). B1's retention goal is preserved and sharpened (the list is now `Stores`-aligned). Update B1's `builder_retains_registered_value` test to read off the store-value list.
- The MemoryProvider is a `build(mp)` argument (not list-retained), so its value never needs to ride the list; `PlatformKind` dropping platform values is fine for B2.
- The drain walks `Stores` and the `StoreValues` list in lockstep (same order, same length) producing `<Stores as ArenaFor>::Arena`. No `FindCarrier`, no coherence problem.

## Status

Validated mechanism, not yet shipped. Lands in B2a (resource arena) per `202605282319_phase-b-data-plane-design.md`.
