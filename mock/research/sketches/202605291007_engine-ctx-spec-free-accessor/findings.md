# Findings: spec-free EngineCtx type-keyed accessor (#623 op-gate)

**Date:** 2026-05-29
**Task:** #623 (remove forbidden full `feature(specialization)` from the engine)
**Sketch:** `selector_witness_sketch.rs` (compiles + runs on `nightly-2026-05-28`)
**Verdict:** op-gate PASSES. The spec-free mechanism is feasible. Proceed with the rewrite, no op pause.

## The problem

`feature(specialization)` is forbidden (unsound by design, never stabilises). The engine
uses it in exactly one place: `dispatch/engine_ctx.rs` `TryHeadResource<T>` /
`TryHeadColumn<T>` (`default fn` plus a type-equality specialising impl), which back
`ResourceBundle::fetch<T>` / `ColumnBundle::fetch<T>`.

The reason it was reached for: the api accessor methods are fixed-shape and key the lookup
on `T` alone, with no selector index:

```rust
fn resource<T: 'static>(&self) -> &T where R: Contains<Resource<T>>;
```

`Contains<S>` is a `#[marker]` trait, so it carries no position (marker traits forbid
associated items). A keyed-on-`T`-alone lookup over a *generic* heterogeneous cons-list
needs either type-equality specialization or an externally supplied index, and the fixed
signature supplies neither. Inside `impl<H, Tail> ResourceBundle for PtrCons<H, Tail>`, the
head type `H` is a generic parameter, so deciding `H == T` is undecidable until
monomorphization, which is exactly what `default fn` specialization papers over.

## The spec-free mechanism

Move the head-vs-tail decision from a generic impl body to the *call site*. The file already
ships the spec-free `Selector<T, Index>` index-witness (`Here` / `There<I>`, distinct index
types so the two impls never overlap). Thread that index as an inferred method generic on the
accessor, bounded so the concrete call site discharges it:

```rust
trait ResourceProviderApi<R: AccessSet> {
    fn resource<T: 'static, I>(&self) -> &T
    where
        R: Contains<Resource<T>>,
        Self: ProvideResource<T, I>;   // EngineCtx delegates to RBundle: Selector<T, I>
}
```

At a WU call site the Ctx is concrete (a WU pins `type Ctx<'frame> = EngineCtx<'frame, R, W,
PtrCons<...>, ColPtrCons<...>>`, see `engine_ctx.rs` tests), so the bundle is a concrete
cons-list and exactly one `Selector<T, I>` impl applies. The trait solver resolves `I`
uniquely:

- single-element bundle: `I = Here`
- `T` at depth `k`: `I = There<...There<Here>>` (`k` levels)

The sketch confirms this resolves through the `HasResourceProvider -> resources() ->
&Provider` indirection and for multi-element bundles.

## Call-shape consequence (minor, test-only blast radius)

A method with two type generics cannot be partially turbofished (`resource::<T>()` triggers
E0107, "expected 2 generic arguments"). So an explicit-`T` call site changes:

- `ctx.resource::<u32>()`  becomes  `ctx.resource::<u32, _>()`, or
- `let v: &u32 = ctx.resource();`  (full inference, no turbofish) when the binding already pins `T`.

Both shapes compile and infer correctly (verified in the sketch: `wu_single_explicit` and
`wu_single_inferred`). The repo-wide blast radius is tiny and entirely engine-internal: 6
`resource::<>`, 1 `read::<>`, 1 `write::<>`, 0 `fire::<>`, all in `engine_ctx.rs` tests where
the binding annotation already pins `T`, so they rewrite to the full-inference form with zero
consumer-visible change. Two doc references in `*.md.tmpl` update alongside.

## Unstable features used

Only `marker_trait_attr` (WATCH-tier, already used by the real `Contains`). No
`specialization`, no `min_specialization`. `min_specialization` was already confirmed
infeasible for the `TryHead` shape (`error: specializing impl repeats parameter T`).

## Rewrite plan (engine-only blast radius)

1. `hilavitkutin-api/src/context.rs`: add the inferred index generic + the `ProvideResource`
   / `ProvideColumnRead` / `ProvideColumnWrite` bridge-trait bounds to the four keyed
   accessors (`resource`, `read`, `write`; `fire` is a B3 no-op and needs no lookup).
2. `hilavitkutin/src/dispatch/engine_ctx.rs`: delete `TryHeadResource` / `TryHeadColumn` and
   the `default fn` impls; reimplement `ResourceBundle::fetch` / `ColumnBundle::fetch` (or
   fold them away) through `Selector` / `ColSelector` with the threaded index; impl the
   bridge traits for `EngineCtx`.
3. Remove `#![feature(specialization)]` from `hilavitkutin/src/lib.rs` and the vestigial gate
   in `hilavitkutin-api/src/lib.rs` (confirm `column_value.rs` builds under
   `min_specialization` alone, per #625).
4. Update the `engine_ctx.rs` test call sites to the full-inference shape.
5. Full `cargo +nightly-2026-05-28 test` across the engine.

The `Project` / `ColProject` construction path already builds the bundle spec-free, so only
the fetch side changes.
