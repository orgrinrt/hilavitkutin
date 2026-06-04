# Sketch findings: accumulator append surface over the unified bindings

**Date:** 2026-06-04
**Round:** accumulator-columns (feat/hilavitkutin-accumulator-columns)
**Hypothesis:** the scheduler bindings cons-list can implement a THIRD type-keyed selector, `AccumSelector` (for `Accum<T>` members), alongside the landed `Selector` (resources) and `ColSelector` (columns), with pass-through `There<I>` recursion on all node kinds, resolving all three unambiguously at a concrete call site; AND the accumulator projection can retain a `'frame` borrow of the binding (the live-length `Cell`) so the `&self` `append` accessor advances it, composing with the copy-pointer resource/column projections from one `&'frame bindings`.

## Verdict: WORKS

`sketch.rs` compiles (only dead-code warnings on the unexercised `VBind` and the bundle tails) under the workspace toolchain and runs:

```
WORKS: 3-way Selector+ColSelector+AccumSelector resolves; the 'frame-borrowed
accum projection composes with copy-pointer projections; append advances the
live-length under &self (A1 len=2 [100,101], A2 len=1 [200])
```

The test list interleaves `RBind<R1> -> CBind<C1> -> ABind<A1> -> ABind<A2>`. Read set `[Resource<R1>, Column<C1>]`, write set `[Accum<A1>, Accum<A2>]`. A1 sits behind a resource and a column node (exercises `AccumSelector` pass-through over `RBind` + `CBind`); A2 sits behind A1 (pass-through over `ABind`). All three selectors resolve to the correct tags; appending to A1 then A2 advances each independent live-length and lands the values in order.

## Why it resolves and is sound

- The three selectors (`Selector`, `ColSelector`, `AccumSelector`) co-implemented on the same node types never overlap: `Here` and `There<I>` are disjoint, and each store type appears once in a well-formed access set, so exactly one index resolves per member. Adding the third selector does not perturb the round-1 dual-selector resolution.
- The accumulator projection is lifetime-tied where the column/resource projections are not. `AccumProject` is a GAT trait (`type Out<'s> where Self: 's`); `AccumSelector::get(&'s self) -> AccPtr<'s, T>` borrows the binding for the projection lifetime, so the projected bundle holds `&'frame Cell<USize>` nodes. Projecting all bundles from one `&'frame bindings` (the resource source, already `'frame`-tied in `EngineCtx::project`) gives the accumulator bundle its borrow while the column source can stay a shorter borrow (it copies `Copy` pointers). This is the architect's Fork-C resolution.
- `append` under `&self` is sound because the live-length is a `Cell<USize>` (interior mutability); `Cell::get` / `Cell::set` take `&Cell`. Single-core now; `AtomicUSize` is the drop-in for multi-core later.

## Two gotchas (both already handled by the real code patterns)

1. **Projection must use the fully-qualified call**, not method syntax, exactly as the landed `EngineCtx::project` does (the read-set and write-set projections are otherwise ambiguous).
2. **`AccPtr` impls `Copy` unconditionally** (a shared ref + a raw base are both `Copy`), so the projected bundle materialises without moving out of the binding, mirroring the real `ColumnPtr` / `ResourcePtr` `Copy` wrappers.

## What this unblocks

The accumulator-columns round (round 2 of op's input-vs-accumulator model). Source shape, corroborated by the `feature-dev:code-architect` read (agent ac4b8a73015e46b8d):

- New `Accum<T>` marker in `store.rs` (sibling of `Column<T>`; `Init = ()`, `Dispatch = StoreDispatch<Self>`, `HasTrivialCtor`).
- New `AccumBinding<T, Tail>` in `bindings.rs` holding the reserved buffer `ColumnPtr<T>` + a `Cell<USize>` live-length; the `Sv<Accum<T>>` drain arm reserves to `record_count` (capacity == record_count this round, per Fork D) and records the real pointer; one new `BindingsFor` arm.
- New `AccumSelector<T, I>` on the binding nodes + `Selector` / `ColSelector` / `AccumSelector` `There` pass-through over the new `AccumBinding` node (and `AccumSelector` pass-through over the existing nodes); new `AccumProject<Set, Idx>` GAT trait mirroring `ColProject`; a fourth `EngineCtx` bundle (the write-set accumulator bundle) projected from the `'frame` bindings; one new `project` generic + bound.
- New `AccumWriterApi<W>` + `ResolveAccumAppend<T, I>` bridge + `HasAccumWriter` accessor in `hilavitkutin-api/src/context.rs`, with `append<T, I>(&self, v: T)` bounded `W: Contains<Accum<T>>` and a `len<T, I>()` reader.
- `CollectFiber` / `fiber_shim` lift to project the accumulator bundle (one more index in the witness tuple); the column-only WorkUnit infers an empty accumulator bundle and dispatches unchanged.

Out of scope (later rounds): the PassEnd persistence drain that reads `[0, live_length)` (#344 / #134); a distinct per-accumulator capacity dimension (Fork D defers it); multi-core atomic live-length.
