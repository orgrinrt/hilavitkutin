# Phase B concrete design: data plane and per-WorkUnit Context

This note is the implementation guide for Phase B of the engine-dispatch arc. It builds on the spine in `202605282100_engine-dispatch-build-plan.md` (read that first for the whole-arc map) and pins the concrete shapes for the data plane plus the per-WorkUnit Context. It is written to survive a context compaction: an implementer should be able to land rounds B1 through B5 from this note plus the cited source.

Phase A (the `ResourceSnapshot` disambiguation, #335) is already merged (PR #90). The placeholder `ResourceSnapshot<const N>` and `ResourceCache` are gone; the typed `ResourceSnapshot<'phase, R>` is the surviving concept and ships here in round B4.

## The single binding decision (op directive, 2026-05-28)

The per-WorkUnit Context is scoped to that WU's declared Read/Write, and the scoping is enforced PHYSICALLY, not only by type-gated where-clauses. A WU's Context holds only the projected pointers for the stores in `Read` union `Write`. A WU cannot reach an undeclared store because its Context does not physically carry that pointer, not merely because a `Contains<...>` bound would reject the call. The engine Context is its own distinct per-WU type; it is not a shared or ambient object and does not reuse `hilavitkutin-ctx::Context<P>`. The term `Ctx` means exactly "this WU's declared access surface" and is not overloaded for any other purpose.

This overrides the earlier whole-arena draft where a single `EngineCtx` held `&'frame WholeArena` and relied on `Contains` bounds alone. The cost of the physical projection is a cheap per-WU pointer copy at dispatch; that is principled, not a shortcut.

## Current contracts (verified 2026-05-28; build against these)

- Store markers (`hilavitkutin-api/src/store.rs`), all `#[repr(transparent)] PhantomData<T>`: `Resource<T>` (`impl BuilderInput { type Init = T; type Dispatch = StoreDispatch<Self>; }`, `Resource::new(_value: T)` currently DROPS); `Column<T>` and `Virtual<T>` (`Init = ()`, `StoreDispatch<Self>`, `HasTrivialCtor`). Also `Field<T: ColumnValue>`, `Seq<T, const N: Cap>`, `Map<K,V,const N: Cap>`, `StoreBundle`, `Replaceable`.
- Access machinery (`hilavitkutin-api/src/access.rs`): `Empty`, `Cons<H,T>`; `AccessSet: Sealed + 'static { const LEN: USize; }`; `#[marker] Contains<S>`; `#[marker] ContainsAll<L>`; `Concat<L> { type Out; }`.
- Accessor contract (`hilavitkutin-api/src/context.rs`, `work_unit.rs`): raw API traits `ColumnReaderApi<R> { unsafe fn read<T: ColumnValue>(&self, i: USize) -> T where R: Contains<Column<T>>; }`, `ColumnWriterApi<W> { unsafe fn write<T: ColumnValue>(&self, i, v) where W: Contains<Column<T>>; }`, `ResourceProviderApi<R> { fn resource<T: 'static>(&self) -> &T where R: Contains<Resource<T>>; }`, `VirtualFirerApi<W> { fn fire<V>(&self) where W: Contains<Virtual<V>>; }`, `EachApi<R,W> { fn run<F: FnMut(USize)>(&self, f: F); }`, `BatchApi<R,W> { fn run<F: FnMut(USize,USize)>(&self,f); }`, `ReduceApi<R,W> { fn run<A,F: FnMut(A,USize)->A>(&self, init: A, f: F) -> A; }`. The seven `HasX` accessor traits (`HasColumnReader<R> { type Provider: ColumnReaderApi<R>; fn reader(&self) -> &Self::Provider; }` etc.) come from `provider_generic!`/`provider_generic2!` and emit no `Context<P>` delegation. `WorkUnit { type Read/Write: AccessSet; type Hint; type Ctx<'frame>: <the seven HasX over Read/Write>; fn execute<'frame>(&self, ctx: &Self::Ctx<'frame>); }`.
- Platform (`hilavitkutin-api/src/platform.rs`): `MemoryProviderApi: Send+Sync+'static { unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8; unsafe fn deallocate(&self, ptr: *mut u8, len: USize); unsafe fn protect(...); }`. Plus `ThreadPoolApi`, `ClockApi`, and `HasMemoryProvider/HasThreadPool/HasClock`.
- Builder (`hilavitkutin/src/scheduler/mod.rs`): `SchedulerBuilder<Wus, Stores>(PhantomData)`; `.with<P: BuilderInput>(self, _provider: P)` DISCARDS the value; `build()` requires `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>` and returns `Scheduler::default()`. `BuilderInput { type Init = Self; type Dispatch; }`, `Dispatch<Wus,Stores,Platform> { type NextWus/NextStores/NextPlatform; }`, routers `UnitDispatch`/`StoreDispatch`/`PlatformDispatch` (NextPlatform currently UNUSED by the 2-param builder)/`RunCfgDispatch`.
- Provenance (`hilavitkutin/src/resource/provenance.rs`): `#[repr(transparent)] ResourcePtr<T>(NonNull<T>)`, `ColumnPtr<T>(NonNull<T>)`, both Copy, `unsafe fn new_unchecked(*mut T)`, `const fn as_ptr(self)`, Send/Sync when T is. Distinct newtypes per store kind give LLVM distinct provenance classes.
- Morsel (`hilavitkutin/src/dispatch/morsel.rs`): `MorselRange { start: USize, len: USize }` with `new/end/is_empty`; `iter_morsel<F,S>(range, body, sync_probe)`. `PlanInputs { ..., record_count: USize }` (`plan/inputs.rs`).
- ColumnValue (`hilavitkutin-api/src/column_value.rs`): `BIT_WIDTH` specialisation, sub-byte UFixed widths. `ceiling_div.rs` exists for byte sizing.

## Round sequence (dependency-ordered; each lands real unit tests)

The whole of Phase B lands on branch `feat/engine-data-plane`, multiple sequential mockspace v1 rounds, one PR when a coherent slice is reviewable (likely split B1+B2 in one PR, B3 in another, B4+B5 in a third; judge by reviewability).

### B1: StageList + builder value retention (stops the drop)

Smallest coherent slice that makes `Resource::new` stop dropping its value.

- Add a sealed value-carrying cons-list distinct from the typestate `AccessSet`: `StageEmpty` (leaf) and `Stage<H, T: StageList>` carrying one real `H` value at the head. Lives in `hilavitkutin/src/scheduler/stage.rs`.
- `Resource::new(value: T)` changes from dropping to returning a value-carrying carrier `StagedResource<T>(T)` that impls `BuilderInput<Init = T, Dispatch = StoreDispatch<Resource<T>>>`. The ZST marker `Resource<T>` lands in the `Stores` typestate via `StoreDispatch<Resource<T>>`; the value `T` rides in the carrier. (`Column<T>`/`Virtual<T>` keep `Init = ()`, no value to carry; WUs carry their `Init = Self` instance.)
- No method is added to `BuilderInput` (it stays `Init` + `Dispatch`): adding a required value-extraction method would break all 24 existing `impl BuilderInput` sites and force boilerplate on every consumer WorkUnit. Instead `.with` retains the whole registered value (the carrier) on the staged list, and `StagedResource<T>` exposes `into_inner(self) -> T`. Value-extraction from the carrier into the arena is a sealed trait introduced in B2, keyed on the carrier types, not a `BuilderInput` method. AS SHIPPED in B1 (PR #91).
- Reshape `SchedulerBuilder<Wus, Stores>` to `SchedulerBuilder<Wus, Stores, Platform, Staged: StageList>` (thread the currently-unused `Platform` accumulator; `Staged` holds the values). `.with<P>(self, provider: P)` advances all four typestates and moves the whole `provider` onto the front of `Staged` (the inner value is extracted later, at the B2 arena drain). `builder()` returns `SchedulerBuilder<Empty, Empty, Empty, StageEmpty>`. Pre-1.0 clean delete-replace: consumers using `let b = Scheduler::builder()` + inference still compile; any explicit `SchedulerBuilder<Wus, Stores>` namer updates (the builder is ephemeral, so this is rare).
- The MemoryProvider is registered via `.with(provider)` like everything else (`PlatformDispatch` routes it to the `Platform` accumulator); extracted at `build()` via a sealed accessor on the `Platform` cons-list (`HasMemoryProvider`-shaped over the tuple).

Tests: `stage_list_carries_value` (a `Stage<u32, StageEmpty>` holds and yields its value); `builder_with_retains_resource_value` (`.with(Resource::new(42u32))`, extract the staged value from the builder, assert 42, proving the drop is gone).

### B2: ArenaFor + ResourceArena allocation

DESIGN VALIDATED 2026-05-29 (architect pass + a compiled sketch). Two premises in the earlier draft were WRONG and are corrected here:

- **`Staged` and `Stores` are NOT 1:1 aligned, so there is no lockstep walk of `Staged`.** `Stores` gains a member only for store inputs; `Staged` (B1) held every input (WUs, platform, kits, stores). Searching the heterogeneous `Staged` for a typed carrier needs a recursive extraction method whose head-match and tail-recurse impls overlap, incoherent without specialization. FIX: replace B1's all-values `Staged` param with a `Stores`-aligned store-VALUE list `StoreValues` populated at `.with` time via a `RouterKind` tag on the dispatch routers plus a `Place<P>` GAT keyed on the tag-as-`Self` (three disjoint `Self` types `StoreKind`/`UnitKind`/`PlatformKind`, non-overlapping, no specialization, no extra method). The single unified `.with` verb is PRESERVED (the architect's `.with`/`.store` split is REJECTED; `DESIGN.md` documents one verb). `StoreKind::Place` prepends the carrier onto `StoreValues`; `UnitKind`/`PlatformKind` drop the value (their TYPE still tracked in `Wus`/`Platform`; WU/platform value retention is a later-round dispatch concern, not the data plane). Mechanism validated + recorded in `mock/research/sketches/202605290002_builder-kind-dispatch.md` (compiles on stable). Update B1's `builder_retains_registered_value` test to read off `StoreValues`.
- **The MemoryProvider is a `build(memory_provider: M)` ARGUMENT, not list-extracted.** The `Platform` accumulator is the same ZST `Cons` (type-level only); provider VALUES were on `Staged`. Rather than walk a list for a maybe-present provider (needs negative bounds, not in stable Rust), `build<M: MemoryProviderApi>(self, memory_provider: M)` takes it explicitly: missing provider is a call-site arity error, duplicate impossible. `Scheduler` retains `M` in a field for `Drop`-time `deallocate`.

Then:

- `resource/arena.rs` (engine crate; `ResourcePtr`/`ColumnPtr` are engine-side): sealed `ArenaFor<S: AccessSet> { type Arena; }` maps `Empty -> ArenaTail`, `Cons<Resource<T>, R> -> ArenaResourceNode<T, ArenaFor<R>::Arena>` (holds `ResourcePtr<T>`), `Cons<Column<T>, R> -> ArenaColumnNode<T, ...>` (holds `ColumnPtr<T>` + `USize` count; uninit in B2a), `Cons<Virtual<T>, R> -> ArenaVirtualNode<T, ...>` (no pointer).
- `DrainStores`: a sealed recursive trait walking `Stores` and the `StoreValues` list in lockstep (same order + length, no search). Per `Resource<T>`: `mp.allocate(size_of::<T>, align_of::<T>)`, null-check, `core::ptr::write` the staged `T` (from the carrier's `into_inner()`) in, record `ResourcePtr::new_unchecked`. Column stride `ceiling_div(T::BIT_WIDTH, 8) * record_count` (B2b; `record_count` runtime, no `generic_const_exprs` wall). On allocation failure, drop the prefix already built and return `Outcome::Err(BuildError::AllocationFailed)`, so `build` returns `Outcome<Scheduler<...>, BuildError>`.
- `Scheduler<Cfg, Stores, M>` gains `arena: <Stores as ArenaFor>::Arena` + `memory_provider: M` fields (struct bound `Stores: AccessSet + ArenaFor`), and a `Drop` impl: a sealed `DropArena` walk runs `core::ptr::drop_in_place` on each moved-in `T` THEN `mp.deallocate` its block (destructor before free; arena owned by value so no double-free). Adding `Stores`/`M` is a pre-1.0 clean change; inference carries them from `build()`. Provide `type BuiltScheduler<Stores, M> = Scheduler<DefaultRunCfg, Stores, M>` for explicit namers.
- SPLIT: **B2a** ships Resources + `Drop` (Column/Virtual nodes present as no-alloc placeholders). **B2a DONE (PR #92, dev `0dc3ec5`).** **B2b is DISSOLVED:** column buffers are sized by `record_count`, which varies per run (a pipeline lints N files, N changes per invocation), so columns are NOT allocated at `build()` like resources; column allocation belongs to the run-loop / plan phase (C/E) where the per-frame `record_count` (via `RunCfg: HasRecordCount`) is known. There is no standalone build-time column round; B3's column accessors operate on per-frame column buffers passed into the Context at construction.

Tests: `arena_resource_round_trip` (register `Resource<u32>=99`, build with a stack-backed test `MemoryProvider`, deref the recorded `ResourcePtr<u32>` to 99); `arena_column_allocation` (register `Column<u8>` with 16 records, `ColumnPtr` non-null, slab length correct); `arena_drop_deallocates` (counting provider, drop scheduler, every allocate paired with a deallocate).

### B3: per-WU projected Context (the op-directive round)

MECHANISM VALIDATED 2026-05-29 (compiled sketch `mock/research/sketches/202605290101_ctx-projection-selector.md`). The type-keyed lookup and the per-WU projection both compile on stable Rust via a frunk-style index witness: `Selector<T, Index>` with `Here` / `There<I>` types (disjoint indices, non-overlapping, no specialization), used over both the arena nodes and the small projected bundle; and `Project<R, Indices>` carrying a parallel `Indices` cons-list (each element index a trait type param, dodging E0207), with a free `project_reads::<R, _, _>(arena)` helper that pins `R` by turbofish and infers the index list. Reuse this exact shape; the `FindResourcePtr<T>`-style "navigation" referenced below IS this `Selector`/`Project` machinery.

- `dispatch/engine_ctx.rs`: `EngineCtx<'frame, R: AccessSet, W: AccessSet>` carrying a per-WU PROJECTED pointer bundle, not the whole arena. Shape: a read bundle (the `ResourcePtr`/`ColumnPtr` for stores in `R`) and a write bundle (for stores in `W`), plus `morsel: MorselRange`. The projection is built per-WU at the monomorphised `invoke_wu_in_fiber::<W>` call site by walking `R` and `W` and pulling the matching pointers out of the scheduler arena into the bundle. A WU physically cannot reach an undeclared store: its `EngineCtx` does not hold that pointer.
- Implement the seven `*Api` traits on `EngineCtx` over the projected bundle: `resource<T>()` finds the `ResourcePtr<T>` in the read bundle (sealed `FindResourcePtr<T>` navigation over the projected list); `read<T>(i)`/`write<T>(i,v)` find the `ColumnPtr<T>` and do bit-offset math at `(morsel.start + i) * BIT_WIDTH`; `each/batch/reduce` iterate the morsel. All accessors take `&self` (interior mutability in the pointer math, never `&mut self`) so LLVM does not reorder writes across fused WUs. The unsafe read/write aliasing obligation is the scheduler's (plan-time DAG analysis proves no concurrent write-overlap); WU bodies do not re-check.
- Implement the seven `HasX` traits on `EngineCtx` with `type Provider = Self` (the Context is its own provider for every accessor). A WU's `type Ctx<'frame>` is not named as `EngineCtx` in any public position; the WU declares the `HasX` bounds (already its shape), and the engine instantiates `EngineCtx<'frame, W::Read, W::Write>` at the private monomorphised call site, which satisfies the bounds.

Tests: `context_resolves_resource` (hand-built projected bundle, `ctx.resources().resource::<u32>()` returns the value); `context_column_read_after_write` (write then read back via the ctx); `context_each_covers_morsel` (morsel start 5 len 3 yields indices 5,6,7); `context_batch_full_range` (start 5, len 3). Negative coverage: a trybuild fixture proving a WU whose `Read` lacks `Resource<T>` cannot call `resource::<T>()` (the `Contains` bound rejects) and, by construction, the projected bundle would not carry it either.

### B4: typed ResourceSnapshot<'phase, R>

Ship the real `ResourceSnapshot<'phase, R: AccessSet>` (the Phase A placeholder is already deleted; the api DESIGN carries the contract-in-prose with a "lands with C6" note to promote back now). It is a read-only view holding the resource pointers for `R`, captured at a phase barrier, lifetime-invariant on `'phase` so it cannot escape across a barrier. Constructed from `&'phase Arena`. Used by `AdaptWu` at `ScheduleEnd`. Promote the contract from prose back to a concrete signature in `hilavitkutin-api/DESIGN.md.tmpl` reconciled against the real `BuilderInput` (no `ResourceDispatch`/`Owned`).

Tests: `snapshot_read_matches_arena`; a trybuild negative proving `'phase` invariance prevents lift-out past the source lifetime.

### B5: persistence drain WU plumbing

A `PassEnd`-fired WU drains accumulator columns into `hilavitkutin-persistence` via a `Resource<P: PersistenceProvider>` (the engine never depends on persistence types directly; the consumer wires the provider). Establishes the plumbing contract; full `PersistenceContext` usage is driven by example apps later.

Tests: `pass_end_wu_fires_at_frame_end` (minimal scheduler + a PassEnd WU, run one frame, assert the WU's execute ran via an atomic flag).

## Files

Create: `scheduler/stage.rs`, `resource/arena.rs`, `resource/snapshot.rs`, `dispatch/engine_ctx.rs`.
Modify: `scheduler/mod.rs` (builder reshape, `Scheduler<Cfg, Stores>`, arena/provider fields, Drop), `hilavitkutin-api/src/store.rs` (`Resource::new` returns `StagedResource<T>` carrier with `into_inner`; the carrier and its `BuilderInput` impl live here, NOT in `builder_input.rs`; no `BuilderInput` method added), `dispatch/wu_fn.rs` (construct + pass the per-WU Ctx), `resource/mod.rs` + `dispatch/mod.rs` (exports).

## rustc-limit notes

`generic_const_exprs` field-access on const-generic params is rejected (why `Scheduler.plan_dirty` and `MAX_PLAN_AFFECTING_RESOURCES` are hardcoded 256, #345). The cons-list arena sidesteps it: one concrete pointer per node, node types known from the `Stores` cons-list at compile time. Column sizing is runtime (`record_count` is runtime), so no const wall there. Keep the 256 hardcode + lint-allow at the L0 root (risk R7).

## Test infrastructure

Arena/ctx tests use a test `MemoryProvider` backed by a fixed `[MaybeUninit<u8>; N]` buffer passed by reference, defined in the test module, counting allocate/deallocate pairs. No `std::alloc`; stays `#![no_std]`.
