# Sketch findings: unified bindings as resource + column source (Shape A)

**Date:** 2026-06-04
**Round:** column-data-plane (feat/hilavitkutin-column-data-plane)
**Hypothesis:** a single interleaved bindings cons-list type can implement both `Selector<T, I>` (resource lookup) and `ColSelector<T, I>` (column lookup), with pass-through `There<I>` recursion on all node kinds, and the dual blanket `Project` / `ColProject` impls resolve their independent index lists unambiguously at a concrete call site.

## Verdict: WORKS

`sketch.rs` compiles clean (one dead-code warning on the unexercised `VBind`) under `rustc 1.98.0-nightly` (the workspace pin) and runs:

```
WORKS: dual Selector+ColSelector over interleaved bindings resolves unambiguously (R2=12, C1=21, C2=22)
```

The test list interleaves `RBind<R1> -> CBind<C1> -> RBind<R2> -> CBind<C2>`. Read set `[Resource<R2>, Column<C1>]`, write set `[Column<C2>]`. R2 sits behind a column node (exercises `Selector` pass-through over `CBind`); C1 sits behind a resource node (exercises `ColSelector` pass-through over `RBind`). All three pointers resolve to the correct tags, indices fully inferred.

## Why it resolves without ambiguity

- `Here` and `There<I>` are disjoint types, so `Selector<T, Here> for RBind<T>` never overlaps `Selector<T, There<I>> for RBind<U>` (different trait instances). Same for `ColSelector`. No coherence conflict from co-implementing both selector traits on the same node types.
- Each store type appears once in a well-formed access set, so exactly one node matches at `Here` and the index resolves uniquely; the blanket `Project`/`ColProject` impls thread the parallel index cons-list, fixing each per-member index by position.
- `Selector` and `ColSelector` are independent traits with independent index parameters; co-implementing both on one type does not cross-constrain their inference.

## Two gotchas (both already handled by the real code)

1. **Projection must use the fully-qualified call**, not method syntax. `a.col_project()` is ambiguous between the read-set and write-set `ColProject` projections (E0283/E0284); `<A as ColProject<R, RCIdx>>::col_project(a)` is required. The real `EngineCtx::project` (engine_ctx.rs:473-475) already does exactly this, so the lifted `fiber_shim` must follow the same form.
2. **The column/resource pointer wrappers must impl `Copy` unconditionally** (no implicit `T: Copy`). The real `ResourcePtr<T>` / `ColumnPtr<T>` are `repr(transparent)` `NonNull` wrappers with manual `Copy` impls, so this is already satisfied; only a `#[derive(Copy)]` would add the spurious bound.

## What this unblocks

Shape A of the column data plane: the bindings cons-list serves as BOTH the resource source and the column source. The source changes required:

- New `ColSelector<T, Here>` on `ColumnBinding<T, Tail>`; `ColSelector<T, There<I>>` pass-through on `ColumnBinding<U>`, `ResourceBinding<U>`, `VirtualBinding<U>`.
- New `Selector<T, There<I>>` pass-through on `ColumnBinding<U>` and `VirtualBinding<U>` (closes the pre-existing resource-after-column traversal gap: today `Selector` only traverses `ResourceBinding` nodes, so a resource declared after a column is unreachable).
- `run()` passes the bindings as the column source; `fiber_shim`/`CollectFiber` drop the `ColPtrNil`-only bound for `A: ColProject<Read, RCIdx> + ColProject<Write, WCIdx>`, using the fully-qualified projection.

Reservation timing (Timing X, separate decision): columns reserve at `build()` to capacity, stable pointer for the scheduler lifetime; per-frame live record count is a metadata field, no per-frame allocation. The `ColumnBinding` Column drain arm stops recording a dangling placeholder and records the real `column_ptr_mut` after a `reserve` sized by the record count.
