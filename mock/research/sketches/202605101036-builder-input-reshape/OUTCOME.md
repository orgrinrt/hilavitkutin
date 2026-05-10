# Outcome: BuilderInput trait reshape sketch

**Date:** 2026-05-10
**Round:** 202605101036
**Status:** WORKS. Sketch compiles cleanly under `rustc 1.96.0-nightly (fda6d37bb 2026-03-27)`.

## Compile result

`cargo +nightly check` on the sketch (lib crate, `edition = "2024"`):

```
Checking sketch_validate_builder_input v0.1.0 (/private/tmp/sketch-validate-builder-input)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
```

No errors, no warnings. One iteration was needed during sketch development (see "Iteration notes" below).

## Verified properties

1. **`BuilderInput` with assoc-type default works.** `type Init = Self;` as the default, `type Dispatch;` mandatory. Resource impls omit `Init` (defaults to `Self`); Column / Virtual / `ExtensionSurface` override `Init = ()`.

2. **Sub-traits with supertrait equality bounds compile.** `Resource: BuilderInput<Dispatch = StoreDispatch<Self>> + Sized + 'static` and the rest of the family are accepted. Consumer types impl both `BuilderInput` and the sub-trait directly; the sub-trait impl is a one-liner that asserts membership.

3. **Supertrait equality enforcement fires on mismatch.** Verified by injecting a `WrongDispatch` type with `BuilderInput::Dispatch = UnitDispatch<Self>` and `impl Resource for WrongDispatch`. Compile error:

   ```
   error[E0271]: type mismatch resolving
     `<WrongDispatch as BuilderInput>::Dispatch == StoreDispatch<WrongDispatch>`
   note: required by a bound in `Resource`
       pub trait Resource: BuilderInput<Dispatch = StoreDispatch<Self>> + Sized + 'static {}
                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Resource`
   ```

   The diagnostic points at exactly the supertrait equality clause and names both the expected and the found dispatch struct. Production-grade.

4. **`SchedulerBuilder::with<P: BuilderInput>` routes correctly.** Single method body returns `SchedulerBuilder<<P::Dispatch as Dispatch>::NextWus<Wus>, NextStores<Stores>, NextPlat<Plat>>`. The call site `.with(Interner { count: 0 }).with(Player { hp: 100 }).with(Tick).with(MyClock).with(SpawnerWu).with(InputKit)` chains fine.

5. **`.extend::<dyn LinterApi>()` works.** Separate method, takes a `?Sized + 'static` type parameter, internally references `ExtensionSurface<T>` to compute the typestate update. The call site is `.extend::<dyn LinterApi>()` with no explicit construction.

6. **AccessSet over bare consumer types composes.** `type ReadSet = Cons<Interner, Cons<Player, Empty>>;` plus `Contains` checks via `check_contains_interner::<ReadSet>()` and `check_contains_player::<ReadSet>()` resolve. Note: `Contains` had to be marked `#[marker]` (see iteration notes); this is the same scaffold the existing AccessSet machinery uses.

## Iteration notes

One compile error surfaced during sketch development:

```
error[E0119]: conflicting implementations of trait `Contains<_>` for type `Cons<_, _>`
```

The two `Contains<X>` impls (head match + tail recursion) overlap on `Cons<X, X>`-shaped inputs. The fix is `#[marker] pub trait Contains<X> {}`, which is sound because the trait carries no items (empty body), so picking either branch is observationally identical. This matches the pattern already used in production-grade type-level cons-list code and is one of the listed nightly features (`marker_trait_attr`) anyway.

No other iterations were needed. The supertrait equality bound, the assoc-type default, the WU full signature with `Schedule = Always` default, the GAT-based `Dispatch` trait with three accumulator slots, and the four router structs (`UnitDispatch`, `StoreDispatch`, `KitDispatch`, `PlatformDispatch`) all compose without surprises.

## Observations

- The assoc-type default on `Init = Self` is a real ergonomic win for Resource impls. The default removes one line per Resource. Column/Virtual override to `()` and that override is the only ceremony at the impl site beyond naming the dispatch.
- The supertrait equality bound (`Dispatch = StoreDispatch<Self>`) gives the cleanest possible error message for the WrongDispatch case. The consumer sees exactly which line on the sub-trait declares the constraint and which side mismatches. No specialization-induced ambiguity, no "trait X is not implemented" red herrings.
- `WorkUnit` with `Schedule = Always` default plus `Read / Write / Hint / Ctx + execute` reads cleanly. `min_specialization` and `marker_trait_attr` are listed in the feature flags but only `marker_trait_attr` is load-bearing for the sketch (used by `Contains` and the bundle markers); `min_specialization` is not exercised here. The doc CL can keep both flags in the feature list per the original spec, but should note that `min_specialization` is not consumed by the BuilderInput design itself.
- `ExtensionSurface<T: ?Sized>` carrying a `BuilderInput` impl with `type Init = ()` and `Dispatch = StoreDispatch<Self>` keeps it on the same routing path as Column/Virtual without special-casing the builder. The dyn-Trait family lands on the Stores accumulator just like any other store, which means the engine treats extensions uniformly for resource resolution.

## Recommendation for the doc CL

**Ship as designed.** The reshape compiles cleanly, the supertrait bound enforces the dispatch contract with production-grade diagnostics, and `.extend::<dyn _>()` separates cleanly from `.with(value)`. Two notes for the doc CL:

1. Mention that `Contains<X>` is `#[marker]` (it already is in the production code, but the rationale is worth a sentence: head-match plus tail-recurse impls overlap on `Cons<X, X>`-shaped inputs; marker semantics make this sound).
2. Note that `min_specialization` is listed for forward compatibility / hedging, but the BuilderInput design as specified does not actually consume it. If a future variant needs specialization (for a fallback `default impl Resource for T: !Column + !Virtual`-style blanket), the feature flag is already on; otherwise it's free.

No tweaks to the named API surface. Resource / Column / Virtual / WorkUnit / Kit / MemoryProvider / ThreadPool / Clock all compile under the supertrait-bounded design, the wrapper structs go away for the sized cases, and ExtensionSurface stays as the dyn-Trait carrier with `.extend` as its dedicated builder method.
