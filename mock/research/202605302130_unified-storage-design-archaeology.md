# Unified storage design archaeology and synthesis

**Date:** 2026-05-30
**For:** task #654 (unified columnar storage contract) and the TOPIC `mock/design_rounds/202605302100_topic.unified-columnar-storage.md`.
**Purpose:** catalogue the storage-relevant design corpus timestamp-ordered so the override chronology is legible, and synthesise the heritage detail worth mirroring. The engine accreted disjoint bespoke arenas (resource arena, adapt arena, the 11 MB plan const-array tree, dispatch buffers); the prior designs worked out a single columnar store that all of these should route through. This note is the reference for that arc.

The pre-extraction designs were imported wholesale into this repo under `mock/research/imported-from-polka-dots/` and `mock/research/imported-from-saalis/` during the extraction to a standalone repo. The polka-dots copies are identical to the upstream source. So the archaeology is in-repo; no need to leave the repo to mine it.

## Corpus catalogue (timestamp-ordered)

Heritage research, pre-extraction (polka-dots, 2026-03-14), all under `research/imported-from-polka-dots/design_rounds/`:

- `202603141800_research.unified-store-deep-dive.md` (also seeded into `design_rounds/202604200055/`): the full `Store` / `Resource` / `Column` / `Virtual` model with concrete Rust signatures. `Cardinality { Single, Many, Virtual }`. The T2 proposal (`SlabColumn<T>`, `VersionedStore<S>`, slice-returning access) that T6 later corrected.
- `202603141800_research.loimu-storage.md`: the `SlabColumn` atom (mmap reserve + demand-commit), SoA views, `ErasedColumn` / `Shape` / `HasShape` plugin-type access, swap buffers, operation fusion, seven access modes. Marked by the heritage itself as "what hilavitkutin should simplify."
- `202603141800_research.columnar-storage-ecs.md`: the singleton-vs-collection unification rationale, the `ColumnStorage` trait proposal (with the now-superseded `as_slice`).
- `202603141800_research.loimu-synthesis.md`: the polka-dots synthesis naming what transfers directly vs what to simplify.
- `202603181200_topic.hilavitkutin-design-consolidation.md`: the authoritative post-synthesis consolidation. Domain 05 (R4 `ColumnValue`), domain 07 (R5 resource field model + R6 the aliasing-critical raw-pointer `ColumnStorage` contract + R2 evict/inject), domain 12 (generation counters), domain 19 (resource stack-local caching + pointer provenance), domain 09 (meta-pipeline plan-as-resource).

Heritage persistence/columnar research, pre-extraction (saalis, 2026-03), under `research/imported-from-saalis/` (NOT YET mined; next pass): `2026-03-14-synthesis.columnar-query-engine.md`, `synthesis.persistence-strategy.md`, `research.hot-cold-storage-strategies.md`, `synthesis.flexible-schema-strategy.md`, `research.universal-entity-model-gotchas.md`, `2026-03-15-deepdive.sieve-eviction.md`. These are the cold-store / eviction / schema side that `hilavitkutin-persistence` descends from; relevant to the `evict`/`inject` boundary.

Bench evidence (NOT YET mined; next pass): `benches/cache_layout_n{64,256,1024,4096,16384}_findings.md`, `benches/variants/layout_soa/`, `benches/NIGHT_FINDINGS_2026-05-11.md`. Empirical grounding for the SoA layout and the aliasing/pointer-provenance findings.

Post-extraction in-repo rounds touching storage (the chronology that may override the heritage):

- `202604201319_topic.hilavitkutin-persistence-sweep.md`, `202604420000_topic.persistence-sweep.md`: persistence crate design.
- `202605282319_phase-b-data-plane-design.md` + `202605282319_topic.builder-value-retention.md`: the data-plane build-out.
- `202605290018_topic.resource-arena.md`: the bespoke resource arena (`ArenaFor` / `DrainStores`). This is the deviation this arc corrects: a separate mechanism instead of routing resources through the one columnar store.
- `202605301730` (#652): the `PlanDims` / `Capacity`-as-type adoption, which supersedes every `const N: Cap` shape in the heritage.
- `202605302100_topic.unified-columnar-storage.md`: this arc.

## Heritage synthesis (the design worth mirroring)

### ColumnStorage contract (consolidation domain 07, R5 + R6)

Settled shape: raw pointers, not slices. The original T2 proposal returned `&[T]` / `&mut [T]`; the T6 bench found that a slice borrow plus a resource pointer from the same `&self` context defeats LLVM's `noalias`, forcing a reload every iteration (1.28-1.40x overhead). Resolution R6: the context exposes `unsafe fn read<T>(&self, i) -> T` / `unsafe fn write<T>(&self, i, v)` over raw pointers. Type-native stride (`size_of::<T>()` per column, no universal stride). 64-byte alignment on column base addresses. Consumer provides backing via `MemoryProvider`; the library never calls an allocator directly. `release(column)` advisory for column lifetime with a reader-count model (decrement on fiber completion, release at zero). `evict`/`dump` + `inject`/`import` (R2) are the persistence handoff: the consumer moves column data to/from external storage at pass boundaries, keyed by per-morsel generation counters so an unchanged root column skips its whole transitive DAG. No alloc. `HasShape` / `ErasedColumn` for plugin (runtime-known) types.

### Decomposition to scalar columns (the heart)

Three layers, all bottoming out in columns of column-sized scalars:

- **Resource field decomposition (R5).** A `Resource` is not a monolithic struct. It decomposes into exactly three field shapes: `Field<T>` where `T: ColumnValue` (scalar, ≤16 bytes, fiber-dispatcher stack-cached, LLVM register-promoted, only-accessed-fields loaded); `Seq<T, N>` (const-sized array in a separate arena); `Map<K, V, N>` (const-sized map, same pattern). No dynamic collections. `Seq`/`Map` elements are not themselves constrained to 16 bytes; only `Field<T>` carries the `ColumnValue` limit.
- **ColumnValue as the scalar base case (R4).** Every column-stored type goes through `trait ColumnValue: Copy + 'static { const BIT_WIDTH: usize; }` with a blanket impl (`size_of * 8`) and sub-byte specialisations for arvo types (`UFixed<1,0>` -> 1 bit). `BIT_WIDTH` drives bitpacked column layout. The type is the storage: no slot-size intermediary, no pack/unpack step. 16-byte max enforced at compile time.
- **Access-set SoA layout.** Within a fiber, columns are physical struct-of-arrays: each column a separate base+stride, co-located in one arena allocation; the morsel context hands one `arena_base` and each column lives at `arena_base + col_offset + i*stride`. RCM reordering sets the column offsets to maximise cache-line co-access. For plugin types, `Shape { fields: [ShapeField { offset, size, field_type }] }` via `HasShape` (`offset_of!`) encodes per-field offsets so the same `SlabColumn` access works for host and plugin records after one load-time validation.

`Seq`/`Map` elements are NOT recursively decomposed to per-field columns; they stay whole-T in their arena. SoA decomposition applies at the Resource/Column boundary.

### Resource vs Column (intent, not mechanism)

Both back onto `SlabColumn` storage; a `Resource` is a length-1 `SlabColumn` (one page committed; the ~4 KB-vs-64-byte overhead accepted for one code path). The scheduler tracks both as `StoreId`; the DAG-edge algorithm makes no cardinality distinction. The distinction is intent and access pattern: `Resource` = persistent / cold / infrequently-written singleton-or-more (config, providers, and the plan), accessed `ctx.resource::<T>() -> &T`; `Column` = hot per-morsel records, accessed morsel-chunked via raw `read`/`write`. `Virtual<T>` = no backing, DAG edge only. Critical invariant (domain 19): resource and column pointers must have separate provenance, or LLVM reloads resource values every iteration; the fiber dispatcher stages resource data stack-local before the morsel loop. The plan is a meta-`Resource<ExecutionPlan>`, a scheduler-owned singleton the meta-pipeline WorkUnits access; making it genuinely store-backed (live-count-sized) is what dissolves the 11 MB.

### Names worth mirroring

`StoreId`, `Cardinality{Single,Many,Virtual}`, `Resource`/`Column`/`Virtual`, `ColumnValue` (`const BIT_WIDTH`), `Field<T>` / `Seq<T,N>` / `Map<K,V,N>`, `SlabColumn`, `ErasedColumn` / `Shape` / `ShapeField` / `HasShape`, `evict`/`inject`. Marker traits `SingleStore` / `ManyStore` / `VirtualStore`.

## Override chronology (what supersedes what)

1. T2 (`202603141800` deep-dive): `SlabColumn` + `VersionedStore` + slice-returning access + `StoreId(u64)` hash + `HashMap<StoreId, ErasedStore>`.
2. T6 bench correction: slices cause aliasing UB -> raw pointers (R6). `StoreId` becomes a dense plan-time `u16` index, not a hash. `HashMap` ruled out by no-alloc -> const-array + plan-time index assignment.
3. Consolidation (`202603181200`): R4 `ColumnValue`, R5 resource field model, R2 evict/inject, R6 raw pointers. Authoritative.
4. Post-extraction Capacity-as-type (#650/#651/#652): every `const N: Cap` / `[T; cap_size(N)]` shape in the heritage is superseded by `C: Capacity` + `Dim<const N: usize>` + `C::Array<T>`. Any heritage signature with `const N` is read through this lens.
5. The bespoke resource arena (`202605290018`): the deviation. This arc returns resources to the one columnar store.

## Falls-in-nicely vs stale

Falls in nicely: the unified `Store`/`StoreId` DAG model regardless of cardinality; `Resource` as a length-1 column on the same code path; `Field<T>` scalar base case with register promotion; `ColumnValue` blanket + sub-byte `BIT_WIDTH`; `evict`/`inject` hot/cold boundary; `HasShape`/`ErasedColumn` plugin decomposition; `Virtual<T>` zero-alloc DAG edge; a single `StoreId`-keyed store map with erased pointers (no `dyn`).

Stale / superseded: `const N: Cap` / `const N: usize` on storage types (-> `Capacity`); `VersionedStore` threading a const-generic `N` (-> thread `Capacity`); slice-returning `as_slice`/`as_mut_slice` (-> raw pointers, R6); `StoreId` hash identity with `LazyLock` (-> dense plan-time `u16`); `HashMap`-backed `StoreMap` (-> const-array + plan-time index); loimu's dynamic view affinity / hysteresis / restructuring (-> static plan-time co-located fiber arenas); loimu's seven access modes + three fusion tiers (-> Read/Write + commutativity); temporal storage (loimu-specific); per-morsel `SnapshotBuffer` double-buffering (-> generation counters).

## Next pass (pending mining)

- `research/imported-from-saalis/` persistence/columnar set (cold-store, eviction, schema): fold into the `evict`/`inject` + persistence-boundary part of the contract.
- `benches/cache_layout_*` + `benches/variants/layout_soa/`: the empirical grounding for the SoA layout and the aliasing/pointer-provenance choices; cite in the DOC CL where the contract makes the raw-pointer / SoA-layout commitments.
