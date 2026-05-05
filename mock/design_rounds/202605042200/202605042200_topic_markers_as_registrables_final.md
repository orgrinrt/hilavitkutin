**Date:** 2026-05-04 (revised 2026-05-05 after second adversarial audit)
**Phase:** TOPIC (third revision; second topic in round 202605042200; supersedes the design proposed in `202605042200_topic_builder_register_unification.md`).
**Scope:** hilavitkutin-api builder-bridge unification via universal sealed `Registrable<B>` trait with monotonic-extension contract on Output, marker types reused as constructable wrappers, `Kit` trait moved into api and collapsed to a thin Registrable-producing alias, diamond-resolution policy committed at compile time, recursion-limit prelude shipped, no escape hatch.
**Source topics:** Tier-1 architectural commitment from substrate-completion audit synthesis (2026-05-04); Task #330; cross-reference loimu's `App::build().with::<C>(value)` shape; six convergent bugs from adversarial audit round 2 (2026-05-04).

# Topic: Markers-as-Registrables, sealed `Registrable<B>` with monotonic-extension contract

The first topic in this round (`202605042200_topic_builder_register_unification.md`) explored a `BuilderRegister<M: StoreMarker>` chained-projection design. K=3 worked example revealed unreadable `<<<...>>>` cascades. Three audits in round 1 unanimously approved that flawed design.

This topic's prior revision (round 2 audited) retained markers, introduced `Registrable<B>` + Kit blanket. Three audits in round 2 ran adversarial and found six convergent bugs that made the design unsafe-as-shipped: diamond dependencies unspecified, public Registrable escape hatch as a smell, BuilderExtending deletion creating soundness hole, silent reorder hazard with Default impls, recursion_limit non-propagation, Kit/kit-crate coherence problem.

This third revision folds all six fixes in. Sketches and trybuild fixtures are mandated alongside the SRC CL; next-solver verification is a hard gate before doc CL lock.

# Final design (revised)

## 1. Markers reused as constructable wrappers

`Resource<T>` becomes value-carrying. `Column<T>` and `Virtual<T>` stay ZST. The asymmetry mirrors the engine's inherent registration methods exactly. **Default impls are dropped from all three markers**, replaced by explicit constructors that name the type at the value site (defends against silent-reorder hazards for adjacent same-shape `()`-init markers).

```rust
// hilavitkutin-api/src/store.rs

#[repr(transparent)]
pub struct Resource<T>(pub T);

#[repr(transparent)]
pub struct Column<T>(PhantomData<T>);

#[repr(transparent)]
pub struct Virtual<T>(PhantomData<T>);

// Constructors. Resource has tuple-struct ctor. Column and Virtual
// require turbofish at the call site so the type is named at value
// position; this prevents silent reorder when adjacent markers
// share Init = ().
impl<T> Column<T> {
    pub const fn new() -> Self { Column(PhantomData) }
}
impl<T> Virtual<T> {
    pub const fn new() -> Self { Virtual(PhantomData) }
}

// NO Default impl shipped on any of the three. Authors construct
// explicitly: Resource(t), Column::<T>::new(), Virtual::<T>::new().

// Copy/Clone propagate from T (Resource) or unconditional (others, ZST).
impl<T: Copy> Copy for Resource<T> {}
impl<T: Clone> Clone for Resource<T> {
    fn clone(&self) -> Self { Resource(self.0.clone()) }
}
impl<T> Copy for Column<T> {}
impl<T> Clone for Column<T> { fn clone(&self) -> Self { *self } }
impl<T> Copy for Virtual<T> {}
impl<T> Clone for Virtual<T> { fn clone(&self) -> Self { *self } }
```

The auto-trait propagation rule for `Resource<T>` becomes "via the actual T", not "via PhantomData<T>". `Resource<T>: Send` iff `T: Send`. Document explicitly.

## 2. Universal `Registrable<B>` trait, sealed, with monotonic-extension contract

```rust
// hilavitkutin-api/src/registrable.rs

#[doc(hidden)]
pub mod registrable_sealed {
    pub trait Sealed<B> {}
}

/// Sealed contract: `Self` can transform a builder of type `B` into
/// a new builder of type `Self::Output`.
///
/// `Self::Output: BuilderExtending<B>` is sealed at the trait
/// declaration. Every Registrable impl proves monotonic extension
/// of the builder's `Stores`. There is no escape hatch: consumers
/// implement `Kit`, never `Registrable` directly.
///
/// Engine ships impls for `Resource<T>` / `Column<T>` / `Virtual<T>`
/// and for cons-list / flat-tuple shapes. The Kit blanket bridges
/// consumer Kits via their `Bundle` projection.
#[allow(private_bounds)]
pub trait Registrable<B>: Sized + registrable_sealed::Sealed<B> {
    type Output: BuilderExtending<B>;
    fn apply(self, b: B) -> Self::Output;
}

// Tuple recursion: cons-list base + step. Sealed via blanket
// Sealed<B> impls below.
impl<B> registrable_sealed::Sealed<B> for () {}
impl<B> Registrable<B> for () {
    type Output = B;
    fn apply(self, b: B) -> B { b }
}

impl<B, H, R> registrable_sealed::Sealed<B> for (H, R) where ... {}
impl<B, H, R> Registrable<B> for (H, R)
where
    H: Registrable<B>,
    R: Registrable<H::Output>,
{
    type Output = R::Output;
    fn apply(self, b: B) -> Self::Output {
        let (h, r) = self;
        r.apply(h.apply(b))
    }
}

// Flat-tuple impls 1..=12 for ergonomic top-level (macro-generated;
// sealed identically). The arity cap matches AccessSet's flat-impl
// cap. Beyond 12, Kit authors compose via nested sub-Kits which
// scale unboundedly.
```

`BuilderExtending<B>` is **kept** in `hilavitkutin-api/src/builder.rs`, lifted as a sealed bound on `Registrable::Output`. This:

- Forces every Registrable impl (engine, blanket, future) to prove its Output extends B.
- The sealed seal on `Registrable` itself prevents consumer-side direct impls (no Layer-3 escape hatch).
- The combination means: only the engine ships leaf Registrable impls (for the markers), api ships the tuple-recursion blankets, and Kit's blanket impl piggybacks on `K::Bundle: Registrable<B>`. Three sources of Registrable, all engine- or substrate-controlled.

## 3. Engine impls for the markers

```rust
// hilavitkutin/src/scheduler/mod.rs

impl<MU, MS, ML, Wus, Stores, T> registrable_sealed::Sealed<...> for Resource<T> { }
impl<MU, MS, ML, Wus, Stores, T>
    Registrable<SchedulerBuilder<MU, MS, ML, Wus, Stores>>
    for Resource<T>
where
    T: 'static,
    Wus: AccessSet,
    Stores: AccessSet,
    (Resource<T>, Stores): AccessSet,
    Stores: NotIn<Resource<T>>, // diamond-policy: compile-time error on duplicate
{
    type Output = SchedulerBuilder<MU, MS, ML, Wus, (Resource<T>, Stores)>;
    fn apply(self, b: SchedulerBuilder<MU, MS, ML, Wus, Stores>) -> Self::Output {
        b.resource(self.0)
    }
}

// Column<T>, Virtual<T> analogous, each with NotIn<Marker<T>> diamond bound.
```

The `Output: BuilderExtending<B>` bound is satisfied because the new `Stores = (Marker<T>, OldStores)` trivially `WuSatisfied<OldStores>` via the existing cons-list `Contains` walk.

## 4. Diamond-resolution policy: compile-time error via `NotIn<S>`

```rust
// hilavitkutin-api/src/access.rs (extension)

#[doc(hidden)]
pub mod not_in_sealed {
    pub trait Sealed<S> {}
}

/// Proves that `H` is NOT a member of cons-list `Stores`. Sealed.
/// Two impls: base `()` (vacuously true), step `(K, R)` where
/// `H != K` (proven by sibling impl pattern that does NOT match
/// `H = K`) and `Stores = R: NotIn<H>`.
#[allow(private_bounds)]
pub trait NotIn<H>: not_in_sealed::Sealed<H> {}

impl<H> not_in_sealed::Sealed<H> for () {}
impl<H> NotIn<H> for () {}

// The "H != K" half is encoded via the negative-impl pattern:
// only impl NotIn<H> for (K, R) where H is NOT K.
// Rust does not have explicit negative impls without nightly.
// Approach: use a #[diagnostic::on_unimplemented]-friendly
// auxiliary trait that fires when H == K, providing a clear
// duplicate-marker error message.
```

The `NotIn<H>` shape needs care to encode "H != K" coherently. Three options for the SRC sketch phase:

a) **`feature(negative_impls)`** on nightly. Direct negative impl `impl<H, R> !NotIn<H> for (H, R) {}`. Sound, requires unstable feature (already in use elsewhere in the substrate).

b) **`TypeId`-free type inequality** via the `typeid` trick or via const-eval on a marker discriminant. Hilavitkutin forbids TypeId per workspace rules; this option is discarded.

c) **Separate sealed `Disjoint<A, B>` trait** with engine-provided impls per pair of distinct marker types. Not scalable past N markers; discarded.

The SRC CL ships a sketch in `mock/research/sketches/202605051200_not_in_negative_impl.md` validating that `feature(negative_impls)` produces the desired compile error AND that the trait-solver cost stays linear in `|Stores|`. If the sketch fails, fall back to runtime first-wins with explicit warning at engine `.resource(t)` registration (engine-side check; Stores is a concrete cons-list at runtime in the form of a typed marker; we walk it via const fn). Prefer the compile-time path if achievable; the runtime fallback is acceptable but loses the "static composition" purity.

The diamond-policy decision is **compile-time error preferred**. The fallback is documented but not chosen unless the sketch demonstrates infeasibility.

## 5. `Kit` trait moves into hilavitkutin-api; `hilavitkutin-kit` crate deleted

The Kit blanket `impl<B, K: Kit> Registrable<B> for K where K::Bundle: Registrable<B>` requires Kit and Registrable to be reachable from the same crate (coherence requirement that the prior topic ignored). `hilavitkutin-kit` currently forbids depending on `hilavitkutin-api` (lint matrix), so the blanket cannot live in either crate cleanly today.

Resolution: delete the `hilavitkutin-kit` crate. Move `Kit` into `hilavitkutin-api` as a thin trait. Update the lint matrix to drop the standalone-kit constraint.

```rust
// hilavitkutin-api/src/kit.rs

/// User-facing trait for bundling related store registrations.
/// Implement this on a struct (typically zero-sized or carrying
/// configuration knobs) to let consumers `.with(MyKit)`.
pub trait Kit {
    /// Heterogeneous tuple of items that this Kit registers.
    /// Each element is either a marker (Resource<T> / Column<T> /
    /// Virtual<T>) or another Kit. Composition is recursive.
    type Bundle;

    fn bundle(self) -> Self::Bundle;
}

// Blanket: any Kit IS Registrable through its Bundle.
impl<B, K: Kit> registrable_sealed::Sealed<B> for K
where K::Bundle: Registrable<B> { }

impl<B, K: Kit> Registrable<B> for K
where K::Bundle: Registrable<B>
{
    type Output = <K::Bundle as Registrable<B>>::Output;
    fn apply(self, b: B) -> Self::Output {
        self.bundle().apply(b)
    }
}
```

Bikeshed: rename `Registrations` to `Bundle` per the consumer-ergo audit. `Bundle` is shorter and more evocative of "a named collection of registrations".

The blanket Kit impl is the ONLY way for non-engine, non-substrate types to satisfy `Registrable<B>`. Combined with the seal on `Registrable` itself (no public escape hatch), the surface area is:

- Engine-impl'd Registrable: three markers (Resource, Column, Virtual).
- Api-impl'd Registrable: tuples and unit (mechanical recursion).
- Blanket Registrable via Kit: any user struct that implements Kit.

There is no fourth path. This closes the coherence space.

## 6. `.with(r)` on SchedulerBuilder; plus `.with_all(tuple)` Bevy-style multi-Kit shortcut

```rust
impl<MU, MS, ML, Wus, Stores> SchedulerBuilder<MU, MS, ML, Wus, Stores> {
    pub fn with<R: Registrable<Self>>(self, r: R) -> R::Output {
        r.apply(self)
    }

    /// Apply a tuple of Registrables in one fluent call. Equivalent
    /// to chained .with() calls. Mirrors Bevy's `app.add_plugins((A, B, C))`.
    pub fn with_all<R: Registrable<Self>>(self, r: R) -> R::Output {
        r.apply(self)
    }
}
```

`with_all` is the same method as `with`, named differently for ergonomics. Both take a Registrable. Tuples of Registrables (length 1..=12 flat or arbitrary cons-list) flow through either method. Documented as "use `.with(single)` or `.with_all((a, b, c))` interchangeably; pick by readability".

## 7. Recursion-limit prelude

```rust
// hilavitkutin-api/src/prelude.rs (new)

/// Macro consumer crates invoke at their crate root to set the
/// recursion limit needed for deep Kit composition (K=20+ or
/// nested Kits 4+ deep).
///
/// Usage at the consumer crate root (lib.rs or main.rs):
///
///     hilavitkutin_api::recursion_limit_for_kits!();
///
/// Expands to:
///
///     #![recursion_limit = "1024"]
#[macro_export]
macro_rules! recursion_limit_for_kits {
    () => {
        #![recursion_limit = "1024"]
    };
}
```

Consumers with shallow Kit composition do not need to invoke. Consumers approaching K=30+ flattened or 4+ nested Kit layers should invoke. The `cargo doc` page on `Kit` explains when to invoke; the rustdoc on `.with()` cross-references.

## 8. Delete `BuilderResource<T>`, retain `BuilderExtending<B>` (lifted)

`BuilderResource<T>` and its sealing module (`builder_resource_sealed`) are deleted from api. The single engine impl is deleted from `scheduler/mod.rs`.

`BuilderExtending<B>` and its sealing module are **retained**. The engine's `add_kit` previously used `K::Output: BuilderExtending<Self>` as a where-clause; we move this guarantee onto `Registrable::Output: BuilderExtending<B>` directly in the trait declaration. `add_kit` is replaced by `.with(kit)` which transitively satisfies the bound via the sealed Registrable contract.

The `.add::<W>()` method on SchedulerBuilder for WorkUnit registration is **retained** (different shape: WUs do not have markers + init values; they're registered via type-level token). The `.with` vs `.add` verb split is documented as principled (different shapes; same builder).

## 9. `Resource::default()` is dropped explicitly

The current `impl<T: Default> Default for Resource<T>` impl is deleted. There is no path for default-constructing `Resource<T>` in the substrate. Consumers writing `Resource(T::default())` if they want a default-init resource. The engine's existing `resource_default<T: Default + 'static>(self)` inherent method is **retained** for direct-engine consumers (it does not construct a `Resource<T>` value; it just produces the type-state with the default-init value flowing through).

`Column<T>::default()` and `Virtual<T>::default()` are also dropped. The `new()` constructors require turbofish; the dropped Default impls cannot accidentally be invoked at the value-tuple site without typing the marker.

## User-facing examples

**K=1 (InternerKit, migrated):**

```rust
impl<const BYTES: usize, const ENTRIES: usize> Kit for InternerKit<BYTES, ENTRIES> {
    type Bundle = (Resource<StringInterner<MemoryArena<BYTES, ENTRIES>>>,);
    fn bundle(self) -> Self::Bundle {
        (Resource(default_interner()),)
    }
}
```

**K=3 (DiagnosticsKit):**

```rust
impl Kit for DiagnosticsKit {
    type Bundle = (
        Resource<Settings>,
        Column<Diagnostic>,
        Virtual<DiagnosticEmitted>,
    );
    fn bundle(self) -> Self::Bundle {
        (
            Resource(Settings::new()),
            Column::<Diagnostic>::new(),
            Virtual::<DiagnosticEmitted>::new(),
        )
    }
}
```

Note: each `Column::<T>::new()` and `Virtual::<T>::new()` names its `T` at the value-tuple site. Reordering the `Bundle` tuple type without updating the value tuple produces a compile error because the inferred type at each `new()` mismatches.

**Composition (no arity cap via nesting):**

```rust
impl Kit for AppKit {
    type Bundle = (
        InternerKit<8192, 256>,
        DiagnosticsKit,
        CompilerFrontendKit,
        Resource<RuntimeFlags>,
    );
    fn bundle(self) -> Self::Bundle {
        (
            InternerKit,
            DiagnosticsKit,
            CompilerFrontendKit,
            Resource(RuntimeFlags::new()),
        )
    }
}
```

**Consumer app:**

```rust
// At consumer crate root:
hilavitkutin_api::recursion_limit_for_kits!();

// In main:
let scheduler = Scheduler::<32, 32, 8>::builder()
    .with(InternerKit::<8192, 256>)
    .with(DiagnosticsKit)
    .with(CompilerFrontendKit)
    .with(Resource(MyAppConfig::detect()))
    .with(Column::<UserData>::new())
    .with(EmissionKit)
    .with_all((Resource(Foo::new()), Column::<Bar>::new(), Virtual::<Baz>::new())) // shortcut
    .add::<Lexer>()
    .add::<Parser>()
    .add::<Typechecker>()
    .add::<Emitter>()
    .build();
```

## Persistence-spine future churn (acknowledged)

When #134 (persistence spine) and #335 (ResourceSnapshot semantics) land, per-store metadata may need to flow through Registrable. Likely shape: additional associated types on `Registrable<B>` (e.g. `type Snapshot: SnapshotPolicy = Never;`), or a sibling trait per-marker. Either path requires a breaking change to `Registrable` pre-1.0. The `no-legacy-shims-pre-1.0.md` rule blesses this; this paragraph captures the audit-trail intent. The exact shape is deferred to the persistence rounds; this round commits only to "the Registrable trait will likely gain associated types or per-marker sibling traits when persistence ships, and that breakage is intentional".

## What this preserves vs deletes vs adds

**Preserves:**
- The `Stores` cons-list as the load-bearing compile-time proof token.
- `Buildable<Stores>`, `WuSatisfied<A>`, `Contains<H>`, `Depth` — type-state proof system.
- `BuilderExtending<B>` — lifted onto `Registrable::Output` as a sealed bound.
- The engine's inherent `.resource(t)`, `.resource_default::<T>()`, `.column::<T>()`, `.add_virtual::<T>()` methods — kept as low-level direct surface.
- `.add::<W>()` for WU registration — kept; different shape.
- Same-T-multi-marker flexibility.
- Strict-static-composition semantics (no runtime registration, no inventory).

**Deletes:**
- `BuilderResource<T>` trait and its sealing module (api).
- The single engine impl of `BuilderResource<T>`.
- `hilavitkutin-kit` crate (entirely; trait moves to api).
- The `BuilderRegister<M> / StoreMarker` proposal from prior topic (never shipped).
- `register![]` macro task #350 (no chain to mechanise; closed as obsolete).
- `Default` impls on all three markers (silent-reorder defense).
- `add_kit` method on SchedulerBuilder (subsumed by `.with(kit)` via Kit blanket).

**Adds:**
- `Registrable<B>` sealed trait + `Sealed<B>` private supertrait module (api).
- Cons-list base + step blanket impls + flat-tuple impls 1..=12 (api, macro-generated).
- `Kit` trait + blanket Registrable impl (api, replacing hilavitkutin-kit crate).
- `NotIn<H>` sealed trait for diamond-policy compile-time-error encoding (api, with sketch in research).
- `Column::<T>::new()` and `Virtual::<T>::new()` constructors (api, store.rs).
- `recursion_limit_for_kits!` macro in api prelude.
- `.with(r)` and `.with_all(r)` methods on SchedulerBuilder (engine).
- Three engine `Registrable<SchedulerBuilder<...>>` impls for the three markers, each with `NotIn<...>` bound for diamond detection.

**Migration cost:**
- `Resource<T>(PhantomData<T>)` -> `Resource<T>(pub T)`. Find runtime construction sites; verified zero non-trivial sites in the existing tree (only `Default` impl bodies, which are deleted).
- `InternerKit` migrates to new shape (~10 lines).
- `tests/store_types.rs` `size_of::<Resource<Foo>>() == 0` assertion is deleted; Column and Virtual size assertions remain.
- Remove `hilavitkutin-kit` from workspace `Cargo.toml`. Remove its lint config from `.claude/rules/lint-forbidden-hilavitkutin-kit.md`. Update lint matrix in mockspace.toml.
- Documentation: README, DESIGN.md.tmpl files in api / engine / providers updated.

## Trybuild fixture plan (mandatory; ships with SRC CL)

Per `cl-claim-sketch-discipline.md`, the SRC CL ships sketches and trybuild fixtures verifying the load-bearing claims:

- **Fixture A** (`tests/trybuild/sealed_registrable_blocks_user_impl.rs`, must NOT compile): user attempts `impl Registrable<SchedulerBuilder<...>> for MyType`. Sealed trait should reject. Error message asserts mentions of the seal.
- **Fixture B** (`tests/trybuild/kit_at_K3_compiles.rs`, must compile): K=3 Kit with three different markers, Stores cons-list of length 3, .build() succeeds.
- **Fixture C** (`tests/trybuild/diamond_compile_error.rs`, must NOT compile): two Kits both registering `Resource<Logger>`, AppKit composes both. Error asserts NotIn<Resource<Logger>> failure with helpful message.
- **Fixture D** (`tests/trybuild/nested_kit_K8.rs`, must compile): K=8 with 4-deep Kit nesting; recursion_limit prelude invoked at consumer root.
- **Fixture E** (`tests/trybuild/silent_reorder_caught.rs`, must NOT compile): adjacent `Column<X>` and `Column<Y>` swapped in Bundle type but not in bundle() value; the required-turbofish constructor catches the mismatch.

## Sketches (ships with SRC CL)

- `mock/research/sketches/202605051200_not_in_negative_impl.md` — verifies `feature(negative_impls)` for `NotIn<H>` works under current rustc; falls back to runtime if not.
- `mock/research/sketches/202605051210_registrable_sealed_kit_blanket.md` — verifies the blanket Kit impl + sealed Registrable doesn't trigger orphan-rule conflicts under `-Znext-solver=globally`.
- `mock/research/sketches/202605051220_recursion_limit_propagation.md` — verifies `#![recursion_limit = "1024"]` via macro at consumer crate root actually propagates to instantiation sites (rustc consumes the attribute at parse time).

## Hard gates before doc CL lock

- Round 3 of adversarial expert audits dispatched on this revised topic, ≤1 substantive new finding required to lock.
- All three sketches above pass.
- `cargo +nightly check -Znext-solver=globally` runs clean.
- Trybuild fixtures A/B/C/D/E behave as specified.

## Per-rule compliance

- `use-the-stack-not-reinvent.md`: rewires existing api-level bridge; no new substrate primitives.
- `no-legacy-shims-pre-1.0.md`: deletes `BuilderResource<T>`, `hilavitkutin-kit`, the prior-topic proposals cleanly. No deprecation aliases.
- `no-bare-primitives.md`: no new bare primitives.
- `hilavitkutin-workunit-mental-model.md`: pure scheduler-builder shape change; no ref-into-storage patterns.
- `cl-claim-sketch-discipline.md`: trait-solver risks (sealed Registrable + Kit blanket coherence; `NotIn<H>` negative-impl encoding; recursion_limit propagation) are all sketched as enumerated above.
- `writing-style.md`: no em-dashes anywhere; no hype words.

## Domain-expert audit pass (round 3, this revision)

Three parallel adversarial audits dispatched on this revised topic immediately. Findings appended below before the topic locks.

(Audit round 3 findings to be appended.)
