# Canonical addendum: the resource storage model (handle + one-record blob)

**Date:** 2026-06-19, revised 2026-07-02 (bench-confirmed).
**Status:** canonical reading-correction of consolidation-spec R5, now with the layout fork
resolved by the six-variant bench (round 202606210600, both runs). The handle model is the
corrected READING of R5 (solid). The original version of this addendum ALSO asserted a
per-member decompose-to-shape-bound-columns layout; the bench refuted that (V2 decomposed and
V3 shape-bound lose axes B/C/E for no hot-loop win). The layout below is the bench-decided one:
a one-record blob (per-resource contiguous value), scalar-snapshotted, with live-streamed
collection members. See `mock/research/202606210600_storage-bench-findings.md` (the full findings
+ run-2 confirmation) and `mock/design_rounds/202606210600_topic.bench-run2-resolution.md`.
**Oracle:** consolidation spec R5 (`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md` lines 535-566, 1682-1710).

## The model, stated outright (bench-confirmed)

A `Resource<T>` is a **handle**, not an inline-value store.

1. **One-record blob value.** `T`'s value lives as a per-resource contiguous blob (one record),
   bumped from the arena, NOT decomposed into per-member columns. The bench decided this: the
   decomposed layout (V2, each member its own column) loses intra-resource locality (up to 3.1x
   at M=64) and crosses the `Dim<256>` column-count cap on realistic resource sets; the
   shape-bound shared layout (V3) carries a 3.4x cross-core false-sharing penalty plus a
   resize-invalidates-all-sharers hazard. Neither buys a hot-loop win. The `Decompose` trait
   (`hilavitkutin-api/src/storage.rs`) remains the per-member seam for the size fold and the
   collection ptr+len, but the value bytes stay contiguous.
2. **Scalar snapshot.** The small scalar `Field` members are snapshotted to a stack local before
   the morsel loop (spec 1714-1724). Wall-clock-neutral on M1 (the reload is L1-cheap), free
   insurance on any uarch where it is not, and the mechanism is real in codegen (V0 reloads,
   V1-V5 hoist; verbatim disasm in the findings). The shipped `DrainStores` blob lacks this
   snapshot; that absence is the drift, not the blob itself.
3. **Handle store, separate provenance.** The resource handle holds the pointer(s) to its value
   blob and to its collection columns, in a store keyed by resource id, with pointer provenance
   distinct from the value columns. This is the noalias substrate.
4. **`Seq`/`Map` are live-streamed, not copied.** A `Seq`/`Map` value is a pointer-to-first plus a
   length (count of strides), elements stored consecutively. The snapshot copies ONLY the ptr+len,
   never the elements: the collection is streamed live from its column inside the loop. The bench
   decided this (axis D): live-stream beats snapshot-copy by ~2.5x once the collection exceeds
   cache (64 MiB), parity below ~4 MiB, confirmed in both runs.
5. **Noalias invariant (architectural).** The handle store never aliases the value columns
   (separate provenance), and scalar members are snapshotted before the loop, so LLVM keeps them
   in registers across the morsel loop. The spec's 1.28-1.40x figure (1698) is a March-2026
   distillation; the bench did NOT reproduce it as a scalar wall-clock effect on M1 (scalars are
   L1-resident, so the hoist is wall-clock-nil there), but the mechanism is real and it is
   decisive for large collection members (axis D). Keep the invariant as an architectural
   guarantee, not a claimed scalar speedup.
6. **Erased static-shape addressing (op picked the hybrid, global-capable, 2026-07-02).** The
   value bytes stay the one-record contiguous blob (items 1-5), but are ADDRESSED through an
   erased static-shape descriptor (backcast on access) rather than a monomorphised concrete-type
   pointer, so a resource value can cross a cdylib/wasm plugin boundary and interoperate with
   builtin resources near-natively (loimu's model). The bench measured this addressing at parity
   with native monomorphised on every axis (v4/v1 within +-1.7% across both runs, ties on B/C/E),
   so it carries no in-process penalty. Op chose the hybrid citing future-proofing + parity, with
   the erasure complexity ruled out as an anti-axis. The per-resource-vs-global sub-question
   resolves to global-capable: every resource uses the erased addressing (any resource
   plugin-capable without a design change). See
   `mock/design_rounds/202606210600_topic.hybrid-decision.md`.

## Why the handle model is what R5 says (the reading correction)

- Spec **1689**: "All behind pointer indirection to external slab storage. NOT inline in
  the context struct." -> the resource holds pointers; values are external. This is item 1+3.
- Spec **1695-1704**: "Resource storage and column storage must have separate pointer
  provenance ... resource data lives in a stack-local region that LLVM can prove
  non-aliasing with column pointers." -> the **handle store** is the separate-provenance
  thing; the stack-local hoist is the noalias mechanism. This is item 3+5.
- Spec **537/1684**: "Resources consist of three field types `Field`/`Seq`/`Map`." -> the
  per-member composition read by the `Decompose` seam (size fold + collection ptr+len); the
  value bytes themselves stay contiguous in the blob (item 1), not one column per member.

## Retraction: the shape-bound-columns reading was wrong (bench)

The original addendum closed a "point R5 left light" by asserting a per-member
shape-bound-columns decomposition. The bench refuted it. R5 says "consist of three field types",
which is value-composition (what the `Decompose` seam reads for sizing), NOT "each member is its
own shape-bound column." Shape-bound sharing appears nowhere in R5 because it is not the design;
the six-variant bench confirmed it loses (V3: 3.4x false-sharing + resize hazard; V2: locality +
column-count). The one genuine R5 clarification that stands: **"separate arena for Seq/Map" (566)
attaches to the handle store, not to a resource-private value arena.** The value is a one-record
blob; only the handle store is separate-provenance.

## The shipped impl is drift only in lacking the snapshot

`DrainStores` (`mock/crates/hilavitkutin/src/resource/bindings.rs:22`) reserves a one-record blob
per `Resource<T>` behind a single `ResourcePtr<T>`. The bench confirms the blob is the CORRECT
per-record layout, so this is not the drift. The drift is that `DrainStores` reads the value live
through the pointer every iteration with no stack-snapshot of the scalar members and no
live-stream path for collection members. The fix is additive (add the snapshot + the live-stream),
not a decomposition rewrite.

## Status of dependent work

- A3b per-fiber morsel sizing rides downstream of the drift fix. The per-store size is the blob
  stride plus, for collection members, the `CollectionBytes` term; the fold is over blob strides,
  not a decomposed per-member column set.
- `CollectionBytes`/`ResourceFootprint` (#163/#164, merged) is the const size source for the
  `Seq`/`Map` collection-bytes term in that formula.
- The drift-fix arc (add scalar stack-snapshot + live-streamed collection access + the noalias
  invariant + erased static-shape addressing on the existing one-record blob) is the storage-model
  work. The layout is now fully settled: bench-decided (blob + snapshot + live-collections) plus
  op's hybrid call (erased addressing, global-capable, `202606210600_topic.hybrid-decision.md`).
  No open storage-model fork remains.
