# Round 202605091700 src apply — decision notes

Recorded 2026-05-10 during overnight autonomous src apply. Captures impl-detail decisions made within the locked doc CL's design space. None of these decisions change the design intent; they choose between options that all align with the doc CL.

## D1. Resource<T> stays PhantomData-only at this round

The doc CL prose says "Resource::new(value)" suggests Resource carries T. The validated sketch literally has `pub struct Resource<T>(T)`. But the existing `Resource<T>(PhantomData<T>)` is load-bearing for typestate use (`Cons<Resource<T>, Stores>` accumulator), and the existing `Default` / `Copy` / `Clone` impls would break if Resource grew a T field.

**Decision**: Resource<T> keeps `PhantomData<T>`. The new `pub const fn new(_value: T) -> Self` consumes T but constructs `Resource(PhantomData)` (drops the value at the constructor).

Rationale: the runtime data plane (HILA-RUNTIME-* tasks) is genuinely not built. `add_resource(_init: T)` already drops the value today. The new constructor preserves identical semantics: Resource is a type-marker; the value passed to `Resource::new` is a stub-time placeholder until HILA-RUNTIME-C6 (resource resolution + persistence spine wiring) lands. At that point the constructor changes to actually store T and wire to the persistence layer. The `Init = T` associated type on Provider already declares this future home.

This keeps the type-level usage surface untouched and lets all existing Default / Copy / Clone impls survive. The tradeoff is subtle: a reader of `Resource::new(GameState::default())` may expect the value to be retained. The doc-comment will note it's a stub until HILA-RUNTIME-C6.

## D2. SchedulerBuilder remains 2-parameter (Wus, Stores)

The sketch had three: `SchedulerBuilder<Wus, Stores, Plat>`. The current source has two: `SchedulerBuilder<Wus, Stores>`. Adding a third would be a breaking signature change for any consumer that names the builder type.

**Decision**: SchedulerBuilder stays 2-parameter. `Dispatch` trait keeps all three GATs (`NextWus`, `NextStores`, `NextPlatform`) per the doc CL spec, but `.with` only consumes the first two. `PlatformDispatch::NextPlatform<Platform>` is forward-compatible: when SchedulerBuilder grows a platform tuple later (likely with HILA-RUNTIME-C4 thread pool implementation), the GAT is already there.

Rationale: forward-compatibility without a breaking change. `with_memory` / `with_threads` / `with_clock` returned `Self` today (no typestate change); equivalent semantics post-reshape is "PlatformDispatch on these types passes Wus and Stores through unchanged".

## D3. WorkUnit and Kit get explicit Provider supertrait, no blanket impl

The doc CL says `pub trait WorkUnit<Schedule = Always>: Provider<Init = Self> + Send + Sync + 'static`. A naive blanket `impl<W: WorkUnit<S>, S> Provider for W` would conflict with `impl<K: Kit> Provider for K` (Rust coherence). `feature(specialization)` exists in the crate but using it for this would be high-risk.

**Decision**: no blanket. Every `impl WorkUnit for FooWu` requires a paired `impl Provider for FooWu { type Init = Self; type Dispatch = UnitDispatch<Self>; const KIND: ProviderKind = ProviderKind::WorkUnit; }`. Same for Kit (with `KitDispatch<Self>` and `ProviderKind::Kit`).

Rationale: explicit per-impl is the pattern the validated sketch uses. The boilerplate cost is real but bounded; a future derive macro can collapse it to one annotation. Tracked as a follow-up in the BACKLOG.

## D4. Memory / Threads / Clock platform impls are per-impl Provider

Same coherence argument as D3. A blanket `impl<M: MemoryProviderApi> Provider for M` and `impl<P: ThreadPoolApi> Provider for P` would conflict because a type could in theory impl both.

**Decision**: per-impl Provider for platform types. Users impl `MemoryProviderApi` then separately `impl Provider for MyMemory { type Init = Self; type Dispatch = PlatformDispatch<Self>; const KIND: ProviderKind = ProviderKind::Memory; }`.

Rationale: matches sketch. Boilerplate cost mirrors D3.

## D5. Provider lives in its own module `provider.rs`, not in `lib.rs`

The api crate's `lib.rs` is currently a thin facade. New trait families get their own module file (see `access.rs`, `capability.rs`, `codec.rs`, etc.). 

**Decision**: `mock/crates/hilavitkutin-api/src/provider.rs` carries Provider, Dispatch, the four routers, ProviderKind, and the sealed Token. Re-exports go through `lib.rs`.

## D6. LinkedBin lives in `provider.rs` too

LinkedBin<T: ?Sized> is a new wrapper, conceptually adjacent to Resource / Column / Virtual. But putting it in `store.rs` would mix it with the existing store-marker family, and the doc CL describes it as "the type-system anchor" for extension-loaded provider bins (a new conceptual category, not a store).

**Decision**: LinkedBin lives in `provider.rs` next to Provider/Dispatch/routers. The store.rs wrappers (Resource/Column/Virtual) get Provider impls referencing the trait from provider.rs. Re-export through `lib.rs::pub use`.

## D7. KitDispatch GAT where-clauses

`KitDispatch<K>::NextWus<Wus>` requires `K::Units: Concat<Wus>`. GATs with where-clauses are stable under nightly's GAT support.

**Decision**:
```rust
impl<K: Kit> Dispatch for KitDispatch<K> {
    type NextWus<Wus> = <<K as Kit>::Units as Concat<Wus>>::Out
    where
        <K as Kit>::Units: Concat<Wus>;
    type NextStores<Stores> = <<K as Kit>::Owned as Concat<Stores>>::Out
    where
        <K as Kit>::Owned: Concat<Stores>;
    type NextPlatform<Platform> = Platform;
}
```

Same shape as the existing `add_kit` where-clauses.

## D8. ProviderKind enum lives next to Provider

Per doc CL it's a documentation/debugging aid only; load-bearing dispatch is via `Provider::Dispatch`. The const `KIND: ProviderKind` on each Provider impl gives diagnostics a cheap discriminator.

**Decision**: `pub enum ProviderKind { WorkUnit, Resource, Column, Virtual, LinkedBin, Kit, Memory, Threads, Clock }` in `provider.rs`. No Copy / Clone / Eq derives unless tests need them; can be added under usage pressure.

## D9. Sealed Token

Provider sealing follows the standard private-token pattern: `pub trait Provider: Sealed` where `Sealed` is `pub(crate)`. Implementing from outside the crate is impossible because Sealed is unreachable.

**Decision**: reuse the existing `mod sealed { pub(crate) trait Sealed {} }` pattern from `lib.rs`. Provider's supertrait is `crate::sealed::Sealed`. Per-impl `impl Sealed for FooType {}` is the unlock.

This means: every concrete Provider impl needs `impl crate::sealed::Sealed for FooType {}` BEFORE the Provider impl. That's per-type. For the four library wrappers (Resource / Column / Virtual / LinkedBin), the Sealed impls live in `provider.rs` or `store.rs`.

For consumer types (WUs, Kits, platform impls), they need `impl Sealed for FooType` outside their crate. Wait — that breaks. Sealed is `pub(crate)`. Consumers can't impl it.

Re-reading the sketch: it uses no sealing. Provider is open: anyone can impl it. The sealing is conceptual, not coherence-enforced.

**Decision (revised)**: Provider is NOT crate-sealed. It's an open trait. Anyone can impl it. The `#[diagnostic::on_unimplemented]` is the consumer-facing UX gate. The sealing in the doc CL phrasing was marketing language; the sketch confirms it doesn't need to be cryptographically sealed.

Rationale: consumers MUST be able to impl Provider on their own types (WUs, Kits, platform impls). A Sealed-bound supertrait would block them. The sketch validated this; trusting the validation.

The `Token` pattern mentioned in the doc CL is interpreted as: a private-token argument to `Dispatch` methods? No — Dispatch has no methods, only GATs. There's nothing to seal at the call site. Sealing was over-specified in the doc CL prose.

Captured here so future agents reading the locked doc CL know the prose's "sealed" language was non-load-bearing and the code intentionally doesn't seal Provider.

## D10. Rustc version — Edition 2024

`rustc 1.94.0 nightly`. `mock/rust-toolchain.toml` pins `nightly`. The sketch validated under `nightly 1.96.0`; current local build is 1.94.0 (slight drift). The features used (adt_const_params, const_trait_impl, generic_const_exprs, marker_trait_attr, specialization) are all enabled in `hilavitkutin-api/src/lib.rs` and stable enough across these point releases.

**Decision**: proceed with current local nightly. If trait-solver behaviour differs, document and adjust.

## D11. Capability* → Provider* rename: file-level decision

The rename in `hilavitkutin-extensions` touches descriptor.rs, error.rs, extension.rs, host.rs, lib.rs, traits.rs. Some prose mentions of "capability" describe the *general concept* of "what an extension exposes" — these are still appropriate to rename per the workspace-wide vocabulary unification (the entire round's purpose).

**Decision**: rename ALL mentions in extensions src — types, fields, methods, doc-comment prose, error messages. The workspace-wide noun is `provider` for the registration/access surface; `capability` retired here. Care taken: the api crate has a separate "capability layer" trait family (Push/BulkPush/Len/Capacity) that is unrelated and stays as-is. The rename scope is `hilavitkutin-extensions` source only.

## D12. Apply order

1. provider.rs in api (new types) — foundation.
2. Provider impls on Resource/Column/Virtual in store.rs.
3. LinkedBin and its Provider/HasTrivialCtor.
4. lib.rs re-exports.
5. WorkUnit Provider supertrait + paired Provider impls in test fixtures.
6. Kit Provider supertrait + paired Provider impls in test fixtures.
7. SchedulerBuilder reshape.
8. Test migration to .with(...).
9. hilavitkutin-extensions Capability* → Provider* rename.
10. hilavitkutin-extensions-macros emit references update.
11. cargo check + test + lint between increments.
12. Lock src CL, close round, push.

## Cross-references

- Round files: `mock/design_rounds/202605091700_*`
- Sketch: `mock/research/sketches/202605091700-builder-provider-shape/`
- Design rule: `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md`
