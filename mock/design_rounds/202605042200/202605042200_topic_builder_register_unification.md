**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** hilavitkutin-api builder-bridge unification (`BuilderResource<T>` -> `BuilderRegister<M: StoreMarker>`) + hilavitkutin engine impl per marker + hilavitkutin-providers `InternerKit` consumer migration.
**Source topics:** Tier-1 architectural commitment from substrate-completion audit synthesis (2026-05-04, see workspace memory `project_substrate_completion_audit_synthesis_2026_05_04.md`); Task #330 (HILA-AUDIT-A1); cross-reference loimu's `AppBuilder::with::<C>(value)` shape (loimu `mock/crates/loimu-runtime/DESIGN.md.tmpl`, `mock/crates/loimu-registry/DESIGN.md.tmpl`).

# Topic: Unify BuilderResource into BuilderRegister&lt;M: StoreMarker&gt;

The api-level builder-bridge family currently ships `BuilderResource<T>` (round 202605041345, PR #49). The design comment on that trait already anticipates the unification question. Adding sibling traits later (`BuilderColumn<T>`, `BuilderVirtual<T>`) means three near-identical sealed traits, three sealing modules, three engine impls, and Kit bounds that name `B: BuilderResource<A> + BuilderColumn<C> + BuilderVirtual<V>` separately.

This round decides now: ship a single sealed `BuilderRegister<M: StoreMarker>` trait keyed off the existing `Resource<T>` / `Column<T>` / `Virtual<T>` marker types, with one engine impl per marker. The method is named `.with` (not `.with_registered`) per loimu's convention. Pre-1.0 is the only window where the migration is mechanical; `BuilderResource<T>` has exactly one consumer today (`InternerKit`).

## Current state

`hilavitkutin-api/src/builder.rs:125-132` defines `BuilderResource<T: 'static>` with a private supertrait `builder_resource_sealed::Sealed<T>`. The single legal impl lives in `hilavitkutin/src/scheduler/mod.rs:240-277`, forwarding to the inherent `SchedulerBuilder::resource()` method.

`hilavitkutin-providers/src/interner.rs:225-235` is the only consumer: `InternerKit<BYTES, ENTRIES>` impls `Kit<B>` where `B: BuilderResource<StringInterner<MemoryArena<BYTES, ENTRIES>>>`, and its `install()` calls `builder.with_resource(default_interner::<BYTES, ENTRIES>())`.

The store markers themselves live in `hilavitkutin-api/src/store.rs`: `Resource<T>`, `Column<T>`, `Virtual<T>` are the three `#[repr(transparent)]` `PhantomData<T>` zero-sized markers. The engine's inherent registration methods on `SchedulerBuilder` are `resource(t)` (takes a `T`), `column()` (takes nothing), `add_virtual()` (takes nothing).

## Why the markers cannot be collapsed

A natural question is whether to follow loimu's shape literally: drop the per-T markers, use a single category ZST `Resource` (no parameter), and let the runtime value's type carry the type identity. Loimu does this because their registries collect descriptors at runtime via the `inventory` crate; the type-level identity is irrelevant outside `TypeId`-keyed storage.

Hilavitkutin cannot. The architectural rule in `.claude/CLAUDE.md` is explicit: linker-magic registration (`inventory::`, `#[ctor]`, `#[distributed_slice]`) is banned in every hilavitkutin crate. Static composition is the rule; the engine's `SchedulerBuilder<MAX_*, Wus, Stores>` carries `Stores` as a type-level cons-list whose elements are exactly `Resource<T>` / `Column<T>` / `Virtual<T>` tokens. The compile-time proofs (`Buildable<Stores>` -> `WuSatisfied<Read>` -> `Contains<H>`) walk that cons-list. Erasing the markers would erase the proofs.

A second hilavitkutin-specific point: the same `T` is allowed to register as multiple distinct markers on the same builder. `Resource<Foo>` (latest singleton) and `Column<Foo>` (full history) are distinct entries in `Stores`, indexed independently by the proof system. Collapsing the marker into the runtime value's type would force one-marker-per-T and break that flexibility.

The right adaptation of loimu's shape is therefore: keep the markers, adopt the `.with(...)` method name, and unify the bridge into a single sealed trait keyed on the marker.

## Design

### StoreMarker sealed trait

Add a sealed `StoreMarker` trait in `hilavitkutin-api/src/store.rs` next to the marker types:

```
#[doc(hidden)]
pub mod store_marker_sealed {
    pub trait Sealed {}
}

/// Sealed marker trait identifying the three store shapes:
/// `Resource<T>`, `Column<T>`, `Virtual<T>`.
///
/// `Init` is the value type passed at registration time. `Resource<T>`
/// requires an initial `T`; `Column<T>` and `Virtual<T>` carry no
/// runtime payload at registration and use `()` as their init.
#[allow(private_bounds)]
pub trait StoreMarker: store_marker_sealed::Sealed {
    type Init;
}

impl<T: 'static> store_marker_sealed::Sealed for Resource<T> {}
impl<T: 'static> StoreMarker for Resource<T> {
    type Init = T;
}

impl<T: 'static> store_marker_sealed::Sealed for Column<T> {}
impl<T: 'static> StoreMarker for Column<T> {
    type Init = ();
}

impl<T: 'static> store_marker_sealed::Sealed for Virtual<T> {}
impl<T: 'static> StoreMarker for Virtual<T> {
    type Init = ();
}
```

Three impls, all in api. Consumers cannot add new store kinds because the sealing module is private.

### BuilderRegister bridge

Replace `BuilderResource<T>` in `hilavitkutin-api/src/builder.rs` with a single trait keyed off the marker. The method is named `.with` per loimu's `AppBuilder::with::<C>(value)` convention:

```
#[doc(hidden)]
pub mod builder_register_sealed {
    pub trait Sealed<M> {}
}

/// Lets a Kit declared in a crate that cannot import the engine
/// register a store marker on the builder via a trait bound.
///
/// `M` is a sealed `StoreMarker` (Resource&lt;T&gt;, Column&lt;T&gt;, Virtual&lt;T&gt;).
/// `WithRegistered` is the resulting builder type after the marker
/// is registered. The single legal impl lives in the engine's
/// `SchedulerBuilder`, forwarding to the inherent `.resource()` /
/// `.column()` / `.add_virtual()` methods per marker.
#[allow(private_bounds)]
pub trait BuilderRegister&lt;M: StoreMarker&gt;: builder_register_sealed::Sealed&lt;M&gt; {
    type WithRegistered;
    fn with(self, init: M::Init) -> Self::WithRegistered;
}
```

The `Init` associated type unifies the three registration shapes:

- `BuilderRegister<Resource<T>>::with(init: T)` registers `Resource<T>`.
- `BuilderRegister<Column<T>>::with(init: ())` registers `Column<T>`.
- `BuilderRegister<Virtual<T>>::with(init: ())` registers `Virtual<T>`.

The `()` init for Column/Virtual is mildly less ergonomic at the call site (`builder.with(())`) but the cost is paid only inside Kit `install()` bodies, which Kit authors write once. Direct-engine consumers keep using the inherent `.column::<T>()` / `.add_virtual::<T>()` methods that take no argument.

### Engine impls

`hilavitkutin/src/scheduler/mod.rs` ships three forwarding impls (replacing the single `BuilderResource<T>` impl):

```
impl&lt;..., T: 'static&gt; BuilderRegister&lt;Resource&lt;T&gt;&gt; for SchedulerBuilder&lt;..., Wus, Stores&gt;
where (Resource&lt;T&gt;, Stores): AccessSet, ...
{
    type WithRegistered = SchedulerBuilder&lt;..., Wus, (Resource&lt;T&gt;, Stores)&gt;;
    fn with(self, init: T) -> Self::WithRegistered { self.resource(init) }
}

impl&lt;..., T: 'static&gt; BuilderRegister&lt;Column&lt;T&gt;&gt; for SchedulerBuilder&lt;..., Wus, Stores&gt;
where (Column&lt;T&gt;, Stores): AccessSet, ...
{
    type WithRegistered = SchedulerBuilder&lt;..., Wus, (Column&lt;T&gt;, Stores)&gt;;
    fn with(self, _init: ()) -> Self::WithRegistered { self.column() }
}

impl&lt;..., T: 'static&gt; BuilderRegister&lt;Virtual&lt;T&gt;&gt; for SchedulerBuilder&lt;..., Wus, Stores&gt;
where (Virtual&lt;T&gt;, Stores): AccessSet, ...
{
    type WithRegistered = SchedulerBuilder&lt;..., Wus, (Virtual&lt;T&gt;, Stores)&gt;;
    fn with(self, _init: ()) -> Self::WithRegistered { self.add_virtual() }
}
```

Plus three sealing impls. All mechanical.

### Consumer migration

`hilavitkutin-providers/src/interner.rs:225-235`:

```
impl&lt;B, const BYTES: usize, const ENTRIES: usize&gt; Kit&lt;B&gt;
    for InternerKit&lt;BYTES, ENTRIES&gt;
where
    B: BuilderRegister&lt;Resource&lt;StringInterner&lt;MemoryArena&lt;BYTES, ENTRIES&gt;&gt;&gt;&gt;,
{
    type Output = B::WithRegistered;
    fn install(self, builder: B) -> Self::Output {
        builder.with(default_interner::&lt;BYTES, ENTRIES&gt;())
    }
}
```

One bound change, one method-name change, one associated-type rename.

### Multi-store Kit pattern

A Kit registering multiple store shapes in one `install()` call carries multiple bounds, expressed via `BuilderExtending` chains rather than a single multi-bound on `B`. Pattern (illustrative; Kit authors write this once per Kit):

```
impl&lt;B&gt; Kit&lt;B&gt; for MultiStoreKit
where
    B: BuilderRegister&lt;Resource&lt;Foo&gt;&gt;,
    &lt;B as BuilderRegister&lt;Resource&lt;Foo&gt;&gt;&gt;::WithRegistered:
        BuilderRegister&lt;Column&lt;Bar&gt;&gt;,
{
    type Output = &lt;&lt;B as BuilderRegister&lt;Resource&lt;Foo&gt;&gt;&gt;::WithRegistered
        as BuilderRegister&lt;Column&lt;Bar&gt;&gt;&gt;::WithRegistered;
    fn install(self, b: B) -> Self::Output {
        b.with(Foo::default()).with(())
    }
}
```

Verbose at the bound site (one chain link per registration) but the Kit's `install()` body is clean linear `.with(...).with(...).with(...)`. The verbosity is bounded to Kit authors and is the price of the type-state proof. Compare to the alternative (split traits): the bound site says `B: BuilderResource<Foo> + BuilderColumn<Bar>` but the engine's `Output` types lose the chain-tracking that BuilderExtending provides, opening a hole where multi-store Kits cannot prove they extend rather than replace.

A future ergonomic helper macro (BACKLOG, not this round) could mechanically generate the chain expression from a list of `(marker, init)` pairs. Out of scope for the round.

## Adapted shape compared to loimu

Loimu's `AppBuilder::with::<C>(value)` has the same surface keyword (`.with`), uses a sealed trait (`AppBuildingBlock<C>`), and treats the category as a turbofish parameter. Three things differ:

1. Loimu's `C` is a category ZST (no parameter); hilavitkutin's `M` is `Resource<T>` / `Column<T>` / `Virtual<T>` parameterised over the value type. Reason: hilavitkutin's `Stores` cons-list carries the parameterised marker as its load-bearing token.
2. Loimu's `value: impl AppBuildingBlock<C>` ties the registrability to the value type via a macro-generated impl. Hilavitkutin's `init: M::Init` decouples registrability (per marker) from the value (passed by the caller). Reason: hilavitkutin's same-T-as-multiple-markers requirement.
3. Loimu's registry uses inventory for runtime collection. Hilavitkutin static composition forbids inventory; the markers ARE the static-composition proof tokens.

The differences trace to substrate constraints that loimu does not share, not to design preference.

## Per-rule compliance

- `use-the-stack-not-reinvent.md`: change is purely structural rewiring of an existing api-level bridge; no new substrate primitives.
- `no-legacy-shims-pre-1.0.md`: deletes `BuilderResource<T>` cleanly. No deprecated alias, no re-export, no transition period. The single consumer migrates in the same round.
- `no-bare-primitives.md`: no new bare primitives introduced. `M::Init = T` for Resource and `M::Init = ()` for Column/Virtual; both are arvo-clean.
- `hilavitkutin-workunit-mental-model.md`: pure scheduler-builder shape change, no new ref-into-storage patterns introduced.
- `cl-claim-sketch-discipline.md`: no rustc-trait-solver risk identified. The lift from `BuilderResource<T>` to `BuilderRegister<M>` follows the same sealed-trait pattern already proven on `BuilderExtending<B>` and `BuilderResource<T>` itself. The `M::Init` associated-type indirection is standard rust trait shape; no GAT, no HRTB, no const-trait. No sketches required for this round.
- `writing-style.md`: doc comments use periods, commas, parentheses; no em-dashes; no hype words.

## Decision

Adopt the unified `BuilderRegister<M: StoreMarker>` shape with `Init` associated type. Method named `.with`. Delete `BuilderResource<T>` cleanly. Ship the three engine impls and migrate `InternerKit`. One round, three crates touched.

## Files

- `mock/crates/hilavitkutin-api/src/store.rs` (add `StoreMarker` sealed trait + three impls)
- `mock/crates/hilavitkutin-api/src/builder.rs` (delete `BuilderResource<T>` + sealing module; add `BuilderRegister<M>` + sealing module)
- `mock/crates/hilavitkutin-api/src/lib.rs` (re-export `StoreMarker`, `BuilderRegister`; remove `BuilderResource` re-export)
- `mock/crates/hilavitkutin-api/DESIGN.md.tmpl` (replace BuilderResource section with BuilderRegister section)
- `mock/crates/hilavitkutin/src/scheduler/mod.rs` (delete `BuilderResource<T>` impl + sealing impl; add three `BuilderRegister<M>` impls + three sealing impls; update imports)
- `mock/crates/hilavitkutin/DESIGN.md.tmpl` (mirror the api change in the engine's deep dive)
- `mock/crates/hilavitkutin-providers/src/interner.rs` (migrate `InternerKit` to `BuilderRegister<Resource<...>>` + `.with(...)` call)
- `mock/crates/hilavitkutin-providers/DESIGN.md.tmpl` (update the InternerKit subsection)
- `mock/crates/hilavitkutin-providers/tests/smoke.rs` (update prose comment that names `BuilderResource<T>`)

## Domain-expert audit pass

Three parallel audits dispatched on this topic before locking, per the audit-three-experts convention used previously on substrate-completion (audit synthesis 2026-05-04) and arvo Rounds 1-8 (#308).

### Type-system / trait-solver expert (verdict: ADOPT AS PROPOSED)

Soundness checks passed. `M::Init` indirection resolves bound-to-argument cleanly; no turbofish required at the standard Kit-author site. Sealed-trait pattern is coherence-safe (private supertrait module + disjoint marker keys + single Self family). Multi-store chain pattern under the trait solver costs O(1) per projection link; existing `recursion_limit = 512` declared on api/engine/kit crates absorbs the depth comfortably. `-Znext-solver=globally` compatibility: no GATs, no HRTBs, no const-trait, no negatives; clean. Alternative shapes (GAT, kind-polymorphism, fundamental tagging, three split traits) all evaluated and rejected as not-strictly-better.

One advisory note: the Column/Virtual `Init = ()` plus a Kit that holds two `()`-init bounds simultaneously without chain-projection would force UFCS turbofish (`<B as BuilderRegister<Column<Bar>>>::with(b, ())`). The chain pattern in the proposal sidesteps this case structurally, so no change to the trait shape is needed; document the pathological case in the rustdoc.

### Consumer-ergonomics expert (verdict: ADOPT WITH MODIFICATIONS)

Single-store Kit IDE inference is fine. Multi-store chain is "genuinely hard to read" but the split-trait alternative just relocates the verbosity; unifying concentrates it where a future macro can attack it. Compile-error noise from sealed bounds is the same shape as the current `BuilderResource<T>` pattern; not worse.

The `b.with(())` smell at Column/Virtual sites is the load-bearing finding. Three modifications recommended:

1. Add `.with_marker::<M>()` inherent helper gated to `M::Init = ()` so Column/Virtual Kits read `b.with_marker::<Column<Bar>>()` without the unit literal. Bare-unit `.with(())` stays available for macros.
2. Open a follow-up task **now** (do not defer indefinitely) for a `register![Resource<Foo> = foo, Column<Bar>, Virtual<Baz>]` macro that mechanises the multi-store chain. The chain verbosity is acceptable only with a credible plan to mechanise it before a third multi-store Kit ships.
3. Include a multi-store chain example in the BuilderRegister rustdoc, not just the single-store case; the chain is the part Kit authors will struggle with.

Loimu comparison: three wins (same-T-multi-marker, init decoupling, compile-time proof) versus three losses (`()` init readability, multi-store Output projection complexity, hand-written bounds vs macro-generated). Trade is acceptable because the wins are non-negotiable architectural requirements and the losses are bounded by the macro plan.

### Architectural-fit expert (verdict: ADOPT WITH MODIFICATIONS)

Type-state proof integration: clean. `BuilderRegister` is strictly compositional, not a parallel proof axis; `Buildable` / `WuSatisfied` / `BuilderExtending` / `Contains` / `Depth` continue to operate on `Stores` shape unchanged. `BuilderExtending` continues to discriminate "Kit must extend, not replace" via the chain-projection idiom.

Static-composition philosophy: sealing `StoreMarker` is correct today but closes two reasonable future doors (mutability-shaped Resource variants, cold-store-backed Column markers). Both are substrate-internal additions, mechanical to add inside api. Recommend documenting that `StoreMarker` is api-internal-expandable: substrate may add markers; consumers cannot.

Send/Sync propagation: the proposal is silent and that is correct for v0. The `M::Init` value is consumed at builder time (before threads exist), so `Init: Send` is not required for the registration call. Send applies to the stored value once threads run; that lives with #334's `Scheduler::run` signature, not on `BuilderRegister`. Recommend documenting the deferral explicitly.

Persistence-spine readiness: orthogonal. The persistence boundary attaches to `Column<T>` / `Resource<T>` storage, not to the registration bridge. No action needed.

#332 trait-diagnostics interaction: multi-store chain depth is K projections (linear, not K^2); the `do_not_recommend` attribute and prelude macro target leaf-bound failures cleanly. Recommend the #332 macro produce one canonical error per chain link, treating multi-store chains uniformly. No new noise the macro can't paper over.

Tier ordering: landing this **before** Tier 2 (#334, #338) is correct. `BuilderRegister` is build-time-shape commitment; the runtime-shape commitments design against the final builder shape, not a transient one.

"Markers cannot be collapsed" reasoning is sound. The const-generic-discriminator alternative (`Marker<T, const KIND: StoreKind>`) was considered and breaks because `Contains<H>` relies on type identity not const-value equality. Distinct types are load-bearing.

## Synthesis: revised proposal incorporates all three audits

Modifications adopted from the audit pass, all folded into the design without revising the trait core:

1. **Add `.with_marker<M>()` no-arg helper on `BuilderRegister`**, gated to `M::Init = ()` via a where-clause (or via a sibling sealed marker trait `EmptyInit`). The bare-unit `.with(())` form stays available for macro generation. Implementation detail: forward to `.with(())` internally.
2. **Document `StoreMarker` as api-internal-expandable**: doc comment states substrate may add markers in future api rounds; consumers cannot. Adds one paragraph in the rustdoc.
3. **Document Send/Sync deferral to #334**: doc comment on `BuilderRegister` notes that thread-safety constraints on the registered value land with the runtime-execution surface, not the build-time bridge.
4. **Document the `()`-init pathological-disambiguation case**: doc comment explains that the chain-projection idiom sidesteps it; the rare direct multi-`()`-bound case requires UFCS.
5. **Include multi-store chain example in `BuilderRegister` rustdoc**: a five-line example showing two-marker chain plus a one-liner pointing at the `register![]` macro task.
6. **Open follow-up task for `register![]` macro**: this round opens the task now (not deferred). The task creates a generative macro consuming `(marker, init)` pairs and emitting the `<<B as BuilderRegister<M1>>::WithRegistered as BuilderRegister<M2>>::WithRegistered` chain plus the linear `b.with(...).with(...)` install body. Lands in a future round; tracked from this round so the verbosity is bounded by a credible mechanisation plan.

The trait core is unchanged from the audit-input proposal: `BuilderRegister<M: StoreMarker>` with `type WithRegistered` and `fn with(self, init: M::Init)`. The modifications layer documentation, ergonomics helpers, and tracked follow-up; no impact on the type-system shape, the engine impl count, or the InternerKit migration.

## Decision (revised)

Adopt the unified `BuilderRegister<M: StoreMarker>` shape with `Init` associated type. Method named `.with`. Plus `.with_marker::<M>()` helper for `Init = ()` markers. Delete `BuilderResource<T>` cleanly. Ship the three engine impls and migrate `InternerKit`. Open the follow-up `register![]` macro task this round. One round, three crates touched, one new task opened.
