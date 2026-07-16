# Self analysis (main agent): the resource storage model, pressure-tested

**Date:** 2026-06-19
**Role:** one of the pressure-test deliverables (the main agent's own expert take), alongside
the three dispatched domain experts. Durable artifact for the synthesis.
**Inputs:** consolidation spec R5 (535-566, 1682-1710); the canonical addendum
`202606210600_resource-storage-model-canonical-addendum.md`; loimu's unified-storage
discussion; the shipped impl (`resource/bindings.rs`, `hilavitkutin-providers/src/storage.rs`,
`hilavitkutin-api/src/store.rs`).

## Position summary

The handle model (Resource = handle; values external; handle store separate-provenance) is
correct and canonical (R5:1689). The shipped one-record-blob is drift. But the part that
deserves real scrutiny is the **per-member shape-bound-column** decomposition: I think it is
right for collection members and questionable, possibly wrong, for scalar members. My lean is
a **hybrid**, stated below, and I want the experts and bench to confirm or refute it.

## The central tension: columns are a MANY-record tool; a resource is ONE instance

The columnar/SoA model earns its keep on **record data**: many records, morsel-windowed,
strided access vectorizes, per-record locality matters. A `Resource` is a **singleton**, one
instance accessed roughly once per pass, not morsel-windowed. Applying the record-column model
to a singleton is not obviously a win:

- **Scalar `Field` members.** Decomposing a resource's scalar fields into separate shape-bound
  columns SCATTERS one resource's value across many columns (one per distinct member shape),
  each entry at the resource's slot index. Accessing the whole resource then touches many
  cache lines in unrelated columns. A contiguous resource record (all its scalars together)
  has better locality for "read this resource's state." The shape-bound sharing
  (`Field<u32>` across resources in one column) gives the provenance-separation noalias win,
  but the scatter is a real cost the spec never weighed.
- **`Seq`/`Map` collection members.** These ARE record-like: N elements, strided, accessed in
  bulk. A column is the natural home; ptr+len over consecutive strides is right. No dispute.

So the uniform "every member is a shape-bound column" rule conflates two cases. The collection
case wants columns; the scalar case may want co-location.

## My lean: hybrid (scalars co-located in the handle region, collections as columns)

- A resource's **scalar `Field` members** live co-located in the resource's **handle/record
  region** (a stack-local, separate-provenance region per the spec's own "resource data lives
  in a stack-local region" wording, 1703). Contiguous; one provenance; the registers/noalias
  win; good locality for whole-resource reads. This is closer to a literal reading of R5 than
  the scatter-into-shared-columns model, and may be what R5 actually intended.
- A resource's **`Seq`/`Map` members** are columns in the unified store; the handle holds their
  ptr+len. This is where decomposition genuinely pays.

Under this hybrid the morsel-size question is unchanged from the column-only resolution: the
per-fiber write-byte sum is over record-column strides; scalar resources contribute 0 (singleton,
co-located, not morsel-scaled); written `Seq`/`Map` resource collections contribute their
`N*elem` (the R5 term, sourced from `CollectionBytes`/`ResourceFootprint` #163/#164). So the
hybrid does NOT reintroduce the heterogeneous type-fold dissonance.

## Why shape-bound matters where it applies, and its risk

Shape-bound (share a column by stride) is the right answer to the column-count-explosion that
type-unique-per-`T` causes. But it only matters for the column-resident members (collections,
and scalars IF we keep them as columns). If scalars co-locate (my lean), shape-bound applies
only to the collection columns, which are fewer and where dedup-by-shape is cleaner. Risk to
flag for the bench: a shape-bound column shared across many resources interleaves unrelated
resources' data; updates from different resources hit the same column (false-sharing across
cores if two cores own different resources sharing a shape-bound column). For singleton
resources written by the meta/plan band this is likely negligible, but it is unbenched.

## Impl-path notes (if hybrid or pure-column is chosen)

- Member enumeration: a resource value type must expose its `Field`/`Seq`/`Map` members at the
  type level. The `ResourceFootprint` derive (#164) already walks exactly these members
  syntactically; it is the natural seed for "list this resource's members + their shapes."
- `DrainStores` changes from "reserve one blob column per Resource<T>" to "for each member:
  collection -> reserve/locate a shape-bound column; scalar -> place in the resource region
  (hybrid) or a shape-bound scalar column (pure)." The handle records the member column
  ids/ptrs.
- `ColumnStorage` likely needs shape-keyed reservation (reserve-or-get by stride) for the
  shape-bound sharing, vs the current id-keyed reserve.
- ctx accessor: `resource::<T>()` resolves the handle, then member access reads the member
  column / co-located scalar.

## Open questions for the synthesis + op

1. Pure per-member shape-bound columns for ALL members, or the hybrid (scalars co-located,
   collections columns)? This is the crux; bench-decidable (whole-resource read locality;
   noalias holds either way).
2. Shape-bound sharing across resources: confirm no false-sharing / soundness issue for the
   write paths that touch shared columns.
3. The morsel-window-internal member fetch vs stack-local cache fork: for singletons the
   stack-local cache is the natural fit (they are not morsel-windowed); morsel-internal-column
   fetch only makes sense for collection members aligned to the record window, which singleton
   resources are not. My lean: stack-local cache for the handle; collection members read via
   their column ptr.

These are hypotheses; the experts' deliverables and a bench decide. I do not treat the hybrid
as settled, only as the strongest candidate I see.
