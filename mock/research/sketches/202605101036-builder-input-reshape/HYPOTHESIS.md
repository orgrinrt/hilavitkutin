# BuilderInput trait reshape

**Date:** 2026-05-10
**Round:** 202605101036
**Topic:** Validate that `Provider` renamed to `BuilderInput` with `type Init = Self` (assoc-type default) and `type Dispatch`, plus sub-traits (`Resource`, `Column`, `Virtual`, `WorkUnit`, `Kit`, `MemoryProvider`, `ThreadPool`, `Clock`) supertrait-bound on `BuilderInput<Dispatch = ...Dispatch<Self>>`, compiles cleanly under nightly with `feature(associated_type_defaults, min_specialization, marker_trait_attr)`.

## Hypothesis

The 202605091700 sketch validated `.with(value)` plus three router structs (`UnitDispatch`, `StoreDispatch`, `PlatDispatch`) and library-side wrapper types (`Resource<T>`, `Column<T>`, `Virtual<T>`, `LinkedBin<T>`). Consumers had to construct the wrapper at the call site (`.with(Resource::new(state))`, `.with(Column::<Player>::new())`).

This reshape eliminates the wrapper structs for the sized cases. Consumer types implement `BuilderInput` plus one sub-trait directly. `.with(state)`, `.with(player_default)`, `.with(Tick)`, `.with(SpawnerWu)`, `.with(InputKit)`, `.with(MyClock)` all work bare. `dyn Trait` still needs a wrapper (`ExtensionSurface<T: ?Sized>`) because `dyn` is unsized; that arrives via a separate `.extend::<dyn Trait>()` builder method.

The supertrait bound (`Resource: BuilderInput<Dispatch = StoreDispatch<Self>>`) is the safety net: if a consumer impls `Resource` but their `BuilderInput::Dispatch` is `UnitDispatch<Self>`, the impl fails at trait-resolution time with the supertrait equality constraint unsatisfied.

## What this sketch proves

1. `BuilderInput` with `type Init = Self;` default and `type Dispatch` compiles.
2. Sub-traits with supertrait equality bounds (`Resource: BuilderInput<Dispatch = StoreDispatch<Self>>`) work and enforce the bound.
3. `SchedulerBuilder::with<P: BuilderInput>` routes to typestate accumulator updates via `P::Dispatch as Dispatch`.
4. `.extend::<dyn LinterApi>()` works as a separate method, wrapping `dyn Trait` in `ExtensionSurface<T: ?Sized>` internally.
5. AccessSet over consumer types directly (not wrapper types) continues to compose (`Cons<Interner, Cons<Player, Empty>>`).
6. Mismatched dispatch (`Resource` impl on a type whose `BuilderInput::Dispatch = UnitDispatch<Self>`) FAILS to compile.

## Outcome shape

Report whether the design compiles, whether the supertrait bound enforcement fires on mismatch, whether `.extend::<dyn _>()` separates cleanly, and any rough edges that surfaced.
