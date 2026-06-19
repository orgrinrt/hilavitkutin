# A3 re-chart: per-fiber morsel L1 budget vs the canonical R5 resource model

**Date:** 2026-06-20
**Status:** limited chart-the-path (op-requested). Re-grounds the A3 morsel-window
formula on the canonical R5 resource model after A3b drifted. Supersedes the A3b
approach (branch `feat/hilavitkutin-per-fiber-morsel-a3b`, parked, not merged) and
flags A3a's `StoreElemBytes` model as itself incomplete.
**Oracle:** consolidation spec `mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` R5 (lines 535-566, 1681-1690, 2428-2435) + the morsel formula (832-842, 548-555, 1078-1083).

## What the canonical design says (R5)

Resources consist of THREE field types ONLY:
- `Field<T>` where `T: ColumnValue` (<=16 bytes): scalar, stack-local, LLVM
  promotes to REGISTERS. Only accessed fields load.
- `Seq<T, const N: usize>`: const-sized array, separate arena. Seq/Map element
  types are NOT bound by the 16-byte ColumnValue limit; only `Field<T>` is.
- `Map<K, V, const N: usize>`: const-sized map, separate arena.

No dynamic collections; every size is compile-time const-generic N.

## The morsel-budget rule (the load-bearing formula)

`morsel_size = (L1_usable / Σ write_sizes).clamp(MIN_MORSEL, MAX_MORSEL) & !3`,
where `Σ write_sizes = Σ write_column_sizes + Σ write_resource_collection_sizes`.
Critically:
- **Write `Seq`/`Map` collections COUNT** toward the L1 morsel budget, at their
  static `N * size_of::<elem>()` footprint.
- **Read-only** resource collections (and read-only columns) do NOT: they ride the
  L2 prefetcher, off the L1 write budget.
- **`Field<T>` scalars do NOT count in the L1 morsel budget.** They are
  register-cached; their cost is in the REGISTER budget (line 1075: "write resource
  pointers (R5 Field<T>)"), a separate constraint, not the L1 morsel denominator.

## The drift (what A3b got wrong, and A3a too)

- **A3b** zeroed ALL resources in `StoreElemBytes`, silently dropping the
  `Σ write_resource_collection_sizes` term the formula requires. Write `Seq`/`Map`
  collections must contribute `N * elem`, not 0. (Discarded, never merged.)
- **A3a** (shipped, PR #160) modeled `StoreElemBytes for Resource<T: ColumnValue>`
  as `size_of::<T>()`. That is also wrong against R5: a `Resource<T>` is NOT a
  single `ColumnValue` scalar; `T` is a struct composed of `Field`/`Seq`/`Map`
  fields. Its L1-morsel footprint is `Σ` over its `Seq`/`Map` fields of `N*elem`
  (Field fields contribute 0 to L1). A3a's `store_sizes` is currently UNUSED (no
  caller), so the wrong model is latent, not yet exercised; the redo corrects it
  before any consumption.

## Corrected per-store L1-morsel-footprint model

The quantity the morsel formula sums is each WRITE store's L1-morsel footprint:
- `Column<T: ColumnValue>` -> `size_of::<T>()` (per-record column stride).
- `Resource<T>` -> `T`'s resource-collection footprint = `Σ` over `T`'s `Seq`/`Map`
  fields of `N * size_of::<elem>()`; `Field` fields contribute 0 (register budget).
- `Accum`, `Virtual` -> 0 for the L1 morsel budget (accum fibers dispatch
  unit-outer; Virtual is a fired marker).
- The READ-vs-WRITE split is already handled by iterating the fiber's WRITE mask
  (read-only stores never enter the sum).

## The unproven premise (what the sketch must confirm)

A3 was sketched only for COLUMN sizes (`ColumnValue::BIT_WIDTH`). The resource-
collection footprint is NOT yet modeled or proven. The crux: a `Resource<T>`'s
footprint depends on `T`'s `Seq`/`Map` FIELDS, which the engine cannot introspect
structurally. So `T` must report its own footprint via a trait, e.g.
`ResourceFootprint { const L1_BYTES: USize; }` summing its `Seq`/`Map` field sizes
(`Field` fields add 0), likely consumer-derived or hand-impl'd on the resource
value type. The sketch (next) must prove: (1) `Seq<T,N>` / `Map<K,V,N>` expose
`N * elem` at the type level (const-generic N + elem `size_of`/`ColumnValue`);
(2) a resource value type can report `Σ` of its `Seq`/`Map` field footprints with
`Field` fields contributing 0; (3) how this composes into the per-store fold so
`StoreElemBytes`/its successor returns the correct L1-morsel footprint per marker.

## Corrected A3 plan (supersedes the prior A3a/A3b split)

1. **A3-sketch-2** (next): prove the resource-collection-footprint surface above
   (`Seq`/`Map` N*elem extraction + a `ResourceFootprint`-style per-resource-type
   trait + `Field`->0). Leeway: some-shape for the footprint trait (derive vs
   manual vs a marker fold), exact for the N*elem arithmetic.
2. **A3-redo**: replace A3a's flat `StoreElemBytes` with the corrected per-store
   L1-morsel-footprint model (Column size_of; Resource -> ResourceFootprint;
   Accum/Virtual -> 0). Re-do the A3b wiring (RunCfg `L1_USABLE`/`MIN_MORSEL`/
   `MAX_MORSEL` consts; `compute_plan` per-fiber sum over the WRITE mask;
   `(L1/Σ).clamp & !3`). Flip `r6` green with a fixture exercising both a write
   column and a write `Seq` resource collection.
3. Then A2b-2 (run() consumes the now-correct `morsel_windows`).

## Note on A3a

A3a's `store_sizes` machinery (the per-store fold + entry) is structurally reusable;
only the per-marker `StoreElemBytes` VALUES were mis-modeled (Resource as a flat
ColumnValue size). The redo keeps the fold, corrects the per-marker footprint, and
adds the resource-collection term. The parked A3b branch is superseded by this
re-chart; its CLs were never locked/closed and its impl was discarded.
