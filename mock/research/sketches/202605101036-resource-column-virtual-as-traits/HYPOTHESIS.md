# Resource / Column / Virtual as traits, not wrapper structs

**Date:** 2026-05-10
**Round:** 202605101036
**Topic:** Flip the slot-kind discriminator from generic wrapper structs (`Resource<T>(PhantomData<T>)`, `Column<T>(...)`, `Virtual<T>(...)`) to traits implemented directly on consumer types, so consumers bare-pass values to `.with(...)`.

## Hypothesis

The 202605091700 design routes slot kinds via wrapper structs that the consumer constructs at the call site. Concretely: `.with(Resource::new(state))`, `.with(Column::<Player>::new())`, `.with(Virtual::<Tick>::new())`. The wrappers only exist to carry the `Kind` discriminator; the runtime never reads the wrapper itself.

A trait-flip removes the wrapper layer at the call site. The consumer's type itself implements `Resource`, `Column`, or `Virtual`. `.with(state)`, `.with(player_record)`, `.with(Tick)` all work without ceremony, while non-providers still fail with the `Provider` `on_unimplemented` diagnostic.

The flip is sound iff:

1. The blanket `Provider` impl can route to per-kind dispatch by detecting which sub-trait the consumer's type implements. This needs either `#[marker]` + specialization, plain specialization, or a non-overlapping discriminator (associated-type `Kind` carried directly in `Provider`).
2. `AccessSet`, `Contains`, and per-WU `Ctx` projections continue to work over consumer types directly (the cons-list members become consumer types instead of wrapper types).
3. `ExtensionSurface<dyn Trait>` (renamed from `LinkedBin<dyn Trait>` for clarity at the API surface) stays as a wrapper because `dyn Trait` is unsized; it's added through a separate `.extend::<dyn Trait>()` builder method, distinct from `.with(value)`.

## Variants to sketch

Three routing strategies, ordered by expected likelihood of success.

**Variant A.** `IntoBuildingBlock<T>` marker plus specialization. Three specialized `Provider` impls, one per slot trait, with a `default impl` covering "everything else implements `Resource` semantics by default" (or no default; pure dispatch on the three).

**Variant B.** Pure specialization without marker. `default impl<T: Resource> Provider for T`, then `impl<T: Column> Provider for T` and `impl<T: Virtual> Provider for T` as specialized versions. Tests whether specialization alone disambiguates without a marker scaffold.

**Variant C.** Associated-type discriminator. `Provider` carries `type Kind: ProviderKindTag`. No blanket impls; consumers implement `Provider` directly, optionally through a helper macro (`resource! { Interner }`, `column! { Player }`). No specialization, no overlap, no nightly feature beyond what's already on.

## Outcome shape

Report which variants compile, which produce the cleanest call-site UX, and which one the doc CL should commit to. Report the ergonomics of `.extend::<dyn Trait>()` separation alongside `.with(value)`.
