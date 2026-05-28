# Sketch — per-WU Context projection via Selector + Project index witnesses

**Hypothesis:** the per-WorkUnit Context (B3) can resolve `ctx.resource::<T>() -> &T` by a type-keyed lookup over a heterogeneous arena cons-list, and can hold ONLY the pointers the WU declares (op's physical-projection directive) by projecting them out of the full arena at construction, all on stable `no_std` Rust with no specialization and no overlapping-impl errors.

**Outcome: WORKS.** Validated by a standalone stable-Rust crate (edition 2024, no feature gates, no heap, no specialization). The positive test (`ctx.resource::<u32>() == &99`, `ctx.resource::<u8>() == &3`) passes; the negative case (`ctx.resource::<u16>()` when `u16` was not in the WU's Read set) is a genuine compile error (`Selector<u16, _>` unsatisfied after the projected bundle is exhausted), which is exactly the physical-scoping guarantee op asked for.

## The mechanism (frunk-style index witness)

Type-keyed lookup over a heterogeneous cons-list overlaps (head-match vs tail-recurse) without specialization. The escape is a type-level index witness `Here` / `There<I>` carried as a second `Selector<T, Index>` parameter: the two impls are keyed on distinct `Index` types, so they never overlap, and the index infers at the call site. The same `Selector` works over the arena nodes and over the small projected bundle.

The projection (build a bundle of only the Read set's pointers) hit `E0207` when the per-element Selector index was a free impl type parameter (a bare `where A: Selector<T, I>` does not constrain `I`, since multiple `I` satisfy it). The fix: `Project<R, Indices>` carries a parallel `Indices` cons-list, so each element index is a *trait* type parameter (constrained by "this trait is implemented"), and a free `project_reads::<R, _, _>(arena)` helper pins `R` by turbofish while inference fills the whole `Indices` list.

## Validated source (the B3 implementer reuses this shape)

```rust
struct Here;
struct There<I>(PhantomData<I>);

trait Selector<T, Index> { fn get(&self) -> ResourcePtr<T>; }
impl<T, Tail> Selector<T, Here> for ArenaResourceNode<T, Tail> {
    fn get(&self) -> ResourcePtr<T> { self.ptr }
}
impl<T, U, Tail, I> Selector<T, There<I>> for ArenaResourceNode<U, Tail>
where Tail: Selector<T, I> {
    fn get(&self) -> ResourcePtr<T> { self.tail.get() }
}
// (same two impls over the projected bundle `PtrCons<H, Tail>` / `PtrNil`)

trait Project<R, Indices> { type Out; fn project(&self) -> Self::Out; }
impl<A> Project<Empty, Empty> for A { type Out = PtrNil; fn project(&self) -> PtrNil { PtrNil } }
impl<A, T, I, RTail, ITail> Project<Cons<Resource<T>, RTail>, Cons<I, ITail>> for A
where A: Selector<T, I>, A: Project<RTail, ITail> {
    type Out = PtrCons<T, <A as Project<RTail, ITail>>::Out>;
    fn project(&self) -> Self::Out {
        PtrCons { head: <A as Selector<T, I>>::get(self), tail: <A as Project<RTail, ITail>>::project(self) }
    }
}

fn project_reads<R, A, Indices>(arena: &A) -> <A as Project<R, Indices>>::Out
where A: Project<R, Indices> { arena.project() }

struct Ctx<RBundle> { reads: RBundle }
impl<RBundle> Ctx<RBundle> {
    fn resource<T, Idx>(&self) -> &T where RBundle: Selector<T, Idx> {
        unsafe { self.reads.get().deref::<'_>() }
    }
}
```

## How this maps onto B3 (per-WU Context, the op-directive round)

- The engine `EngineCtx<'frame, R, W>` is constructed per WU at dispatch: project the scheduler arena down to `R` (and `W`) via `project_reads`, holding only the declared `ResourcePtr`/`ColumnPtr` bundle. A WU physically cannot reach an undeclared store (no `Selector` path in its bundle). This is op's directive enforced structurally, not by type-gating a whole-arena ref.
- `resource::<T>()` resolves via `Selector` over the projected read bundle. `read::<T>(i)` / `write::<T>(i, v)` over the projected column bundle index `base + (morsel.start + i)`. `each`/`batch`/`reduce` iterate the morsel. All accessors `&self` (interior mutability in the pointer math).
- Implement the seven `HasX` traits on `EngineCtx` with `Provider = Self` (the Ctx is its own provider). The WU declares the `HasX` bounds on `type Ctx<'frame>`; the engine instantiates `EngineCtx<'frame, W::Read, W::Write>` at the monomorphised `invoke_wu_in_fiber::<W>` call site.
- The arena nodes use the same `Selector` for lookup; reuse the index-witness types `Here`/`There`.

## Columns lifetime reframing (corrects the earlier "B2b at build()" framing)

Resource buffers are sized once at `build()` (singleton, scheduler lifetime; shipped in B2a). Column buffers are sized by `record_count`, which varies per run (a real pipeline lints N files where N changes per invocation), so columns are NOT allocated at `build()`; they belong to the run-loop / plan phase (C/E) where the per-frame `record_count` (via `RunCfg: HasRecordCount`) is known. There is no standalone "B2b columns at build()" round; column buffer allocation folds into the run-loop. B3's column accessors operate on the per-frame column buffers passed into the Context at construction. B3 is testable now with hand-provided column buffers, independent of the run-loop.

## Status

Validated mechanism, not yet shipped. Lands in B3 (per-WU Context). The arena (resources) is shipped (B2a); B3 projects it per WU and adds the accessors.
