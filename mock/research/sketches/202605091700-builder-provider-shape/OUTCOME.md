# Outcome — builder-provider-shape sketch

**Status: WORKS**

The unified `.with(value)` builder shape compiles and resolves
correctly under nightly 1.96.0 with the features
`adt_const_params + const_trait_impl + generic_const_exprs`.

## What the sketch proves

1. **One method, one verb.** `SchedulerBuilder::with<P: Provider>(p: P)`
   accepts WUs, Resources, Columns, Virtuals, LinkedBins, Kits, and
   platform providers in one signature. No `add_*` / `with_*` split.
2. **Per-kind typestate update via `Provider::Dispatch`.** Each
   provider declares its `Dispatch` associated type as one of three
   routers: `UnitDispatch<Self>` (WUs and Kits land on the `Wus`
   list), `StoreDispatch<Self>` (Resources / Columns / Virtuals /
   LinkedBins land on `Stores`), or `PlatDispatch<Self>` (memory,
   threads, clock land on `Plat`). The per-kind list grows correctly
   under the single blanket `.with` method.
3. **`pub const trait Marker: Provider<Init = ()>`** with `fn new() -> Self`
   compiles with the `pub const trait` keyword form. Library-side
   wrappers (`Column<T>`, `Virtual<T>`, `LinkedBin<T: ?Sized>`) impl
   `Marker` to surface a `const fn new()` for call-site uniformity.
4. **`LinkedBin<dyn TraitFamily>` works with `?Sized` bounds**.
   The `dyn LinterApi` parameter survives the typestate accumulation.
5. **End-to-end ergonomics matches the design.** Call site reads as
   ten lines of `.with(value)`, value-form per kind:

   ```rust
   SchedulerBuilder::new()
       .with(MyMemory::new(arena))
       .with(MyThreadPool::new(8))
       .with(MyClock)
       .with(Resource::new(GameState::default()))
       .with(Column::<Player>::new())
       .with(Virtual::<Tick>::new())
       .with(LinkedBin::<dyn LinterApi>::new())
       .with(InputKit)
       .with(SpawnerWu)
       .with(PhysicsWu)
       .build();
   ```

6. **Diagnostic UX is correct.** Passing a non-Provider type (`u32`)
   produces `error[E0277]: u32 is not a Provider; pass a registered
   provider value to .with(...)` with the on_unimplemented note
   pointing at constructors and listing all `Provider`-impl types.
   The error surfaces at the call site (the `.with(42u32)` token)
   and points at the missing `Provider` bound, not at any incidental
   per-kind blanket.

## Why the diagnostic shape mattered

Initial sketch had multiple per-kind blanket impls of a `Registers`
helper trait. Trait solver picked one (the `WorkUnit`-blanket) as
the closest match for `u32` and reported `u32: WorkUnit not satisfied`,
which misled the reader about the real missing bound. Restructuring
dispatch through a single `impl<P: Provider> Registers for P` (then
collapsing `Registers` entirely into `.with`'s where-clause) fixed
the diagnostic. The error now points at `Provider` directly, which
is the load-bearing trait.

The lesson: when the goal is "one trait gates the whole surface",
the dispatch-via-associated-type pattern (`Provider::Dispatch`)
beats per-kind blanket-trait stacks because the trait solver
fails on the right bound at the right level.

## Open questions for the doc CL

1. **Naming the dispatch struct vs. type alias.** Sketch uses
   `UnitDispatch<W>`, `StoreDispatch<S>`, `PlatDispatch<P>` as
   PhantomData-carrying tuple structs. Production code may prefer
   them as type aliases or sealed-marker enums. Bikeshed.

2. **Kit dispatch placement.** Sketch routes Kits to `UnitDispatch`
   (same list as WUs). The actual production typestate already has
   Kit's `Units` and `Owned` fields appended via the round-4
   `Concat` machinery. Production `Kit` dispatch needs a
   fourth router that does `Concat<K::Units, Wus>` and
   `Concat<K::Owned, Stores>` instead of single-cons. Mechanical
   to add; out of sketch scope.

3. **`Marker` trait scope.** Sketch impls `Marker` for the four
   library-side wrappers. Should we also impl `Marker` for unit-
   struct WUs/Kits to enable a `Type::new()` form alongside the
   bare-name form? Probably no — the bare name is shorter and
   already idiomatic for unit structs. `Marker` exists for
   PhantomData-carrying generic wrappers; unit structs don't need
   it.

4. **Build-side `Stores: ContainsAll<Wus::AccumRead>` constraint.**
   Sketch's `.build()` is signature-only; production carries the
   typestate `ContainsAll` proof. The proof is orthogonal to the
   builder shape; it stays untouched by this round's reshape.

## What this round's doc CL specifies

Based on the sketch:

- New trait `pub trait Provider: Sized` in `hilavitkutin-api` with
  `type Init`, `const KIND: Kind`, `type Dispatch: Dispatch`.
- New trait `pub const trait Marker: Provider<Init = ()>` with
  `fn new() -> Self`.
- New trait `pub trait Dispatch` with `type NextWus<Wus>`,
  `type NextStores<Stores>`, `type NextPlat<Plat>`.
- Three dispatch markers: `UnitDispatch`, `StoreDispatch`,
  `PlatDispatch` (and a fourth `KitDispatch` doing Concat).
- Single `.with<P: Provider>(p: P)` on `SchedulerBuilder`.
- All current `add_*` and `with_*` methods retire.
- Library-side `Provider` impls on `Resource<T>`, `Column<T>`,
  `Virtual<T>`, `LinkedBin<T: ?Sized>`, plus blanket impls for
  `MemoryProviderApi` / `ThreadPoolApi` / `ClockApi`-implementing
  types.
- Existing `WorkUnit` and `Kit` traits supertrait `Provider`.

## Recorded

2026-05-09 sketch round 202605091700, validating the topic-2 builder
shape decision before drafting the doc CL.
