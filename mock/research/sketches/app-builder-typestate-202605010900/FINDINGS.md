# Findings: app-builder type-state sketch (202605010900)

## Update 2026-05-01 round 202605011500 (final design after third reviewer)

Five substantive fixes validated. The third reviewer caught three issues neither the first two reviewers nor the author saw. All landed in the sketch.

### 1. Buildable Wus uncap (recursive impl)

Original per-arity macro expansion `impl_buildable!(W0..W11)` capped registered WUs at 12. Replacement: one base case + one recursive impl. No cap.

```rust
impl build_sealed::Sealed for () {}
impl<Stores: AccessSet> Buildable<Stores> for () {}

impl<H, R> build_sealed::Sealed for (H, R) {}
impl<H, R, Stores> Buildable<Stores> for (H, R)
where
    H: WorkUnit,
    R: Buildable<Stores>,
    Stores: AccessSet + WuSatisfied<H::Read> + WuSatisfied<H::Write>,
{}
```

Verified by `smoke_fifty_wus` (50 WUs in one builder chain). Coherence: `()` and `(H, R)` are disjoint, no `#[marker]` needed.

### 2. WuSatisfied uncap via cons-list shape

The original per-arity 0..=12 cap on `WuSatisfied<A>` bites real consumers (vehje typecheck legitimately reads from 16+ stores: Interner, TypeEnv, ConstEnv, ImportTable, ScopeStack, SealedRegistry, OrphanRules, Definitions, Bodies, Mir, Spans, ResolvedTypes, NameTables, Macros, Diagnostic, Errors).

Fix: ship `read!` / `write!` macros that convert flat-tuple syntax to cons-list shape. `WuSatisfied<A>` becomes recursive over the cons-list, no cap.

```rust
#[macro_export]
macro_rules! read {
    () => { () };
    ($T:ty $(,)?) => { ($T, ()) };
    ($T:ty, $($rest:ty),+ $(,)?) => { ($T, $crate::read!($($rest),+)) };
}

impl<S: AccessSet> WuSatisfied<()> for S {}
impl<S, H: 'static, R> WuSatisfied<(H, R)> for S
where S: Contains<H> + AccessSet + WuSatisfied<R>, R: AccessSet,
{}
```

Consumer migrates from `type Read = (Resource<X>, Column<Y>);` to `type Read = read![Resource<X>, Column<Y>];`. Verified by `smoke_wu_with_sixteen_stores` (a single WU declaring 16 stores in Read).

### 3. Kit::Output structurally constrained via BuilderExtending<B>

A buggy Kit could previously declare `type Output = SchedulerBuilder<(), ()>` and silently wipe registrations. Fix: new sealed trait `BuilderExtending<B>` proves the Kit's output preserves the input's WUs and contains every store the input had.

```rust
pub trait BuilderExtending<B>: extending_sealed::Sealed<B> {}

impl<Wus, Stores, NewStores> BuilderExtending<SchedulerBuilder<Wus, Stores>>
    for SchedulerBuilder<Wus, NewStores>
where
    Wus: AccessSet,
    Stores: AccessSet,
    NewStores: AccessSet + WuSatisfied<Stores>,  // proves no store wiped
{}

// Engine method:
pub fn add_kit<K>(self, k: K) -> K::Output
where
    K: Kit<Self>,
    K::Output: BuilderExtending<Self>,
{ k.install(self) }
```

The `Wus` parameter is identical between input and output (Kit can only add stores, not WUs). The `NewStores: WuSatisfied<Stores>` clause proves nothing was dropped. Verified by `smoke_buggy_kit_rejected` (commented out compile-fail case).

### 4. Depth trait for plan-stage code

`AccessSet::LEN` reports immediate-tuple-arity (always 2 on cons-list cells). Plan-stage code needs total cons-list depth. Ship a sealed `Depth` trait scoped to cons-list shapes only.

```rust
mod depth_sealed { pub trait Sealed {} }

#[allow(private_bounds)]
pub trait Depth: depth_sealed::Sealed {
    const D: USize;
}

impl depth_sealed::Sealed for () {}
impl<H, R: Depth> depth_sealed::Sealed for (H, R) {}

impl Depth for () { const D: USize = 0; }
impl<H, R: Depth> Depth for (H, R) { const D: USize = R::D + 1; }
```

Two impls. No specialization. Disjoint shapes. Compile-time assertion `<FiftyWusType as Depth>::D == 50` passes.

`Depth` is only impl'd on cons-list shapes. Flat tuples beyond `()` and `(T,)` don't get `Depth`. This is correct: plan-stage code consumes the engine's `Wus` / `Stores` accumulators (always cons-list), and consumer-declared `Read` / `Write` use the macros (cons-list). If a code path ever needs Depth on a flat tuple, that's an architectural smell.

### 5. Recursion limit

Set `#![recursion_limit = "512"]` at the api crate level. A 50-WU consumer with 50 stores per app produces ~1M trait-solver sub-obligations during `.build()`. Default 128 won't cut it. Engine, kit, and consumer crates may also need to declare it; document in api lib.rs preamble.

### Decisions vs alternatives

- **No specialization.** Tried `min_specialization` for `Depth`; rejected (can't specialize on trait bounds). Full `specialization` is unsound and won't stabilize. Sealed two-impl pattern works without it.
- **No `#[marker]` on `Depth`** (markers can't have associated consts).
- **No new lint cops.** All sealing via private supertrait + `#[allow(private_bounds)]`.
- **No structural-coincidence reliance.** The cons-list pattern is now explicit at every level: `Buildable`, `WuSatisfied`, `Depth` all have explicit cons-list impls. `AccessSet` is documented as shape-based (LEN reports immediate-tuple-arity); cons-list code paths use `Depth` instead.



**Date:** 2026-05-01
**Backs:** Round `202605010900` (#255). The DOC changelist relies on these findings for concrete signatures.
**Sketch:** `sketch.rs`. Self-contained, `no_std` plus `feature(marker_trait_attr)`. Compiles clean on nightly with three positive smoke tests; the negative test (registered WU with no matching `Resource<T>`) is verified to fail compilation with a pointed error.

## What was validated

The mechanism for `SchedulerBuilder` to evolve from a no-op skeleton into a phantom-tuple type-state shape that proves WU `Ctx` bounds at `.build()`, plus the Bevy-style `Kit` trait composing onto whatever the builder gets.

Compiles. Negative case rejected with the right error. The decisions locked in TOPIC are sound.

## Decisions confirmed

1. **Phantom-tuple accumulation works.** `SchedulerBuilder<Wus, Stores>` with two slots is sufficient (not four). `Wus` accumulates registered WU types. `Stores` unifies registered `Resource<T>`, `Column<T>`, and `Virtual<T>` markers; this matches how `WorkUnit::Read: AccessSet` is shaped in `hilavitkutin-api/src/work_unit.rs` (single tuple of mixed marker kinds).

2. **Method-only Bevy-style Kit composes cleanly.** A Kit's `install` body is just chained builder method calls. The Kit's signature pins input and output types via `impl<Wus, Stores> Kit<SchedulerBuilder<Wus, Stores>> for MyKit`. The `Kit::Output` associated type carries the resulting builder type forward. No type-level Kit-specific machinery needed; type-state evolves via the builder's existing methods.

3. **`.add_kit(k)` returns `K::Output`.** Single line: `k.install(self)`. Because Kit is parameterised over the builder type, the call site infers correctly.

4. **`.build()` proof works via two-tier sealed traits.** `Wus: Buildable<Stores>` per Wus arity reduces to `Stores: WuSatisfied<Wn::Read> + WuSatisfied<Wn::Write>` for every Wn. `WuSatisfied<A>` per-arity reduces to `Stores: Contains<T>` for every T in A.

5. **Compile errors point at the right thing.** When a WU declares `Read = (Resource<Interner>,)` and no `Resource<Interner>` is registered, rustc says: "trait bound `(): Contains<Resource<Interner>>` was not satisfied". The consumer reads "I'm missing `Resource<Interner>`" directly.

## What changed from the topic file

One subtle correction:

**Builder shape is `SchedulerBuilder<Wus, Stores>`, not `SchedulerBuilder<Wus, Resources, Columns, Providers>` as the topic file initially sketched.**

The topic file mentioned "four tail type parameters carrying phantom tuples: registered WU types, registered Resource value types, registered Column value types, registered accessor-projection markers from Kits". The first sketch implemented exactly that. It didn't compile, because splitting Resources from Columns at the type level requires per-member filtering of the WU's mixed Read/Write tuple. Plus the existing `AccessSet` design in `hilavitkutin-api` does not split by store kind (Read is `(Resource<X>, Column<Y>, Virtual<Z>)` mixed).

Single unified `Stores` slot matches the api shape exactly. Resource / Column / Virtual distinction is encoded in the marker types themselves; the registry just holds them all. Every concern the four-slot version was meant to address (kind safety, Kit projections) collapses into membership in the single `Stores` tuple.

The "Providers" slot from the topic file becomes unnecessary too: a Kit's "accessor projection" is just additional `Resource<T>` registrations that the WU's `Ctx` bound already references. Nothing separate to track.

The topic file says "Concrete `.build()` where-clause shape, AccessSet arity ceiling for accumulator tuples, and exact composition of 'providers' are settled in DOC phase with code sketches." This is that sketch. The four-slot scheme is replaced by the two-slot scheme; DOC CL writes against two slots.

## Cons-list shape vs flat tuples

The other shape detail: `Stores` accumulates as a **cons-list** `(Head, Rest)` because each builder method wraps the previous Stores as the second tuple element. After `.resource::<A>(_).column::<B>().resource::<C>(_)`, Stores is `(Resource<C>, (Column<B>, (Resource<A>, ())))` (4 levels deep, arity-2 at every level).

The existing `Contains<T>` impls in `hilavitkutin-api/src/access.rs` are for **flat tuples** at each arity (`(T0,)`, `(T0, T1)`, `(T0, T1, T2)`, up to 12). They cover head matches but do NOT recurse through the cons-list tail.

The fix is **one extra `Contains` impl** that recurses through the tail:

```rust
impl<H: 'static, R: 'static, T: 'static> Contains<T> for (H, R)
where R: Contains<T> {}
```

This says: `(H, R)` contains T if R contains T. Combined with the existing arity-2 impl `Contains<T0> for (T0, T1)` (head match), membership chains correctly down arbitrarily deep cons-lists.

The two impls overlap; this works because `Contains` is `#[marker]`, which permits overlap. No coherence conflict. The sketch validates this.

The existing flat-tuple impls stay. They're how `WorkUnit::Read = (Resource<X>, Column<Y>)` declarations are checked. The recursion impl is additive and only fires for cons-list shapes.

This implies a small change to `hilavitkutin-api/src/access.rs`: add the recursion impl alongside the existing flat impls. Trivial line; one line of new code, conceptually orthogonal.

## AccessSet arity ceiling: not the binding constraint

Topic file flagged the existing `AccessSet` arity-12 cap as a possible concern (accumulator tuples may exceed 12). With the cons-list shape, this is moot: `(H, R)` is always arity-2 at every cons cell, regardless of how many stores have been registered. The cons-list bottoms out at `()` (arity-0 AccessSet impl).

`AccessSet` impl coverage needs to extend to handle the cons cell shape. The single recursive `AccessSet for (T0, T1) where T1: AccessSet` impl plus the arity-0 base case `AccessSet for ()` covers cons-lists at any depth.

Sketch uses arities 0..=4 of the existing flat-tuple `AccessSet` impls (cons cells happen to also be arity-2 tuples, so the existing arity-2 `AccessSet for (T0, T1)` impl satisfies them). The DOC CL can either:

- Keep the existing flat-tuple `AccessSet` impls 0..=12 unchanged (they cover cons cells incidentally because every cons cell is arity 2).
- Extend the recursion impl on `Contains` as described above.
- Confirm the depth limit is rustc's recursion limit (default 128, raisable via `#![recursion_limit]`), well above any plausible store count.

No need to extend AccessSet to higher arities. The arity-12 cap continues to govern WU `Read`/`Write` declarations (which are flat tuples), not the builder accumulator (cons-list).

## What the DOC CL writes

Concrete signatures the DOC CL pins:

```rust
// hilavitkutin/src/scheduler/mod.rs

pub struct SchedulerBuilder<Wus, Stores> {
    // ... plus the existing MAX_UNITS / MAX_STORES / MAX_LANES const generics
    _phantom: PhantomData<(Wus, Stores)>,
}

impl SchedulerBuilder<(), ()> {
    pub const fn new() -> Self { /* ... */ }
}

impl<Wus, Stores> SchedulerBuilder<Wus, Stores>
where
    Wus: AccessSet,
    Stores: AccessSet,
{
    pub fn add<W: WorkUnit>(self) -> SchedulerBuilder<(W, Wus), Stores>
    where (W, Wus): AccessSet { /* ... */ }

    pub fn resource<T: 'static>(self, _init: T)
        -> SchedulerBuilder<Wus, (Resource<T>, Stores)>
    where (Resource<T>, Stores): AccessSet { /* ... */ }

    pub fn column<T: 'static>(self) -> SchedulerBuilder<Wus, (Column<T>, Stores)>
    where (Column<T>, Stores): AccessSet { /* ... */ }

    pub fn virtual_<T: 'static>(self) -> SchedulerBuilder<Wus, (Virtual<T>, Stores)>
    where (Virtual<T>, Stores): AccessSet { /* ... */ }

    pub fn add_kit<K: Kit<Self>>(self, k: K) -> K::Output { k.install(self) }
}

impl<Wus, Stores> SchedulerBuilder<Wus, Stores>
where
    Wus: Buildable<Stores>,
    Stores: AccessSet,
{
    pub fn build(self) -> Scheduler<MAX_UNITS, MAX_STORES, MAX_LANES> { /* ... */ }
}
```

```rust
// hilavitkutin-kit/src/lib.rs

pub trait Kit<B> {
    type Output;
    fn install(self, builder: B) -> Self::Output;
}
```

```rust
// hilavitkutin-api/src/access.rs (adds one impl)

impl<H: 'static, R: 'static, T: 'static> Contains<T> for (H, R)
where R: Contains<T> {}
```

The `Buildable<Stores>` and `WuSatisfied<A>` sealed traits live in `hilavitkutin-api` (alongside `AccessSet`). They're api-shape contracts, not engine-internal.

## Open questions for DOC, now answered

The topic file listed five open questions for DOC phase. Sketch answers four:

1. **AccessSet arity ceiling.** Not a concern. Cons-list shape sidesteps it. Existing arity-12 flat-tuple cap is fine for WU `Read`/`Write` declarations.
2. **`.build()` where-clause shape.** Two-tier sealed: `Wus: Buildable<Stores>`, `Buildable` per arity reduces to `Stores: WuSatisfied<Wn::Read> + WuSatisfied<Wn::Write>`, `WuSatisfied<A>` per arity reduces to `Stores: Contains<T>` for every T in A.
3. **Provider categorisation.** Single unified `Stores` tuple. No separate "providers" slot. Kit projections are just additional `Resource<T>` registrations within the same tuple.
4. **Compile-fail test infrastructure.** `trybuild` is the standard tool. DOC CL adds it as a dev-dep. Sketch confirmed manually that the negative case fires with a clear error; trybuild captures that for CI.
5. **Crate layering verification.** No cycles in the layout `notko` -> `arvo*` -> `hilavitkutin-api` -> `hilavitkutin-ctx` -> `hilavitkutin-kit` -> `hilavitkutin (engine)`. The `Kit<B>` trait and the `Buildable` / `WuSatisfied` sealed-trait machinery split cleanly: contracts (`Buildable`, `WuSatisfied`) in `hilavitkutin-api`; preset trait (`Kit`) in `hilavitkutin-kit`; type-state usage in the engine.

One minor question (5) was about where the sealed traits live. Sketch puts them in `hilavitkutin-api` because they're contract-shape artefacts referenced in `WorkUnit`'s `Ctx` bound, not engine internals. `hilavitkutin-kit` only needs the `Kit<B>` trait itself.

## Caveats

- The sketch elides the real `Ctx` bound conjunction (seven `HasX<...>` traits in `hilavitkutin-api/src/work_unit.rs`). The conjunction reduces to `Stores: WuSatisfied<W::Read> + WuSatisfied<W::Write>` once the seven `HasX` projections are erased to per-store membership. Sketch validated the membership half; the projection-erasure half is mechanical and routine.
- The sketch uses `Buildable` / `WuSatisfied` impls at arities 0..=3 (Wus) and 0..=2 (Read/Write). The DOC CL extends to the existing arity-12 cap (or higher if needed). Mechanical macro work; no novel design call.
- `feature(marker_trait_attr)` is the one new nightly feature this round adds. hilavitkutin already runs nightly; per `arvo-compile-time-last.md`, nightly features are accepted when they unlock substrate goals. The recursive `Contains` impl needs `#[marker]` to overlap with the existing flat-tuple impls; without it, coherence rejects the addition.

## What was learned about the design-round flow

Prototyping in `mock/research/sketches/` before writing the DOC CL caught the four-slot-vs-two-slot bug that would have surfaced in SRC phase. Cost: about 60 minutes. Without the prototype, SRC would have written four slots, hit the compile error, then back-tracked to two slots, at minimum a deprecate-and-rewrite of the doc CL.

This is the model for substrate-shaping rounds going forward: where the load-bearing detail is type-level engineering, prototype before DOC. Where the round is purely mechanical (sweep, rename, primitive substitution), DOC straight from TOPIC is fine.
