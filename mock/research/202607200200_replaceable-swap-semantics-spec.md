# The Replaceable Swap Specification

**Date:** 2026-07-19
**Status:** DRAFT pending op ratification (the seed pre-freeze resolution
batch). On ratification this document is the ruling source for the swap
rulings' registry rows and the seed storage chapter's replacement section.
**Commissioned by:** A2-5 (`pre_seed/202607193200`), which found swap
semantics unspecified and the prior grounding a mis-citation.
**Inputs:** the expert panel (`pre_seed/202607193300_swap-semantics-panel/`,
memos 01 soundness, 02 cost, 03 synthesis; the draft below lifts memo 03's
audited clauses), the exclusivity sketch
(`pre_seed/sketches/202607193310_planaffecting-replaceable-exclusivity/`,
WORKS both arms), the storage addendum (`pre_seed/202606210600_resource-storage-model-canonical-addendum.md`),
A2-3 (PlanAffecting open marker), and shipped source at the cited lines.

The prior presentation of this spec was declined not on content but on the
durability of canon's medium; that objection is resolved by the seed and
registry regime. The clauses below are unchanged in substance from the
audited synthesis, restated as the normative spec with the evidence pointers
inline.

## S1: replace_value mechanism

`Scheduler::replace_value<V: Replaceable>(&mut self, new: V)` installs `new`
as the whole one-record blob value: a `ptr::write` of `V` through the same
`Selector<V, Index>` witness (the binding's backcast pointer) that the drain
and the projection read use. No pointer derived by any other route (the
noalias provenance argument depends on it). No drop of the old value
(`V: ColumnValue` is `Copy`; the bytes are overwritten). The call
additionally sets the store-dirty bit for `Resource<V>` via `mark_dirty` and
never touches `plan_dirty`.

The call is legal only between frames. `&mut self` plus the
parked-between-frames invariant (named as load-bearing, because worker raw
pointers sit outside the borrow checker's jurisdiction) means no swap can
race a live-streamed read or an in-flight snapshot. The swap is observable
starting at the next `run()` and at no earlier point; of multiple swaps
before a `run()`, only the last is ever observed.

## S2: replace_resource mechanism

`Scheduler::replace_resource<T: PlanAffecting>(&mut self, new: T)` performs
the identical S1 write and store-dirty mark, and additionally sets the
`plan_dirty` bit for `T`'s `PlanAffectingId` in the scheduler's plan-dirty
bit array. The very next `run()` consumes that array: the band gate is
`first_frame OR any plan_dirty bit`, a plan-dirty frame enters the leading
plan band at phase 0, and the consumed bits clear.

Signature obligation riding #696: `PlanAffecting` becomes an open marker on
resource value types (the `BuilderInput` supertrait and the seal drop, per
A2-3), and the `Locate` target unifies with S1 to
`Stores: Locate<Resource<T>, Index>`. The shipped `Locate<T, Index>` shape
is the pre-A2-3 form; the implementing round corrects it. The shipped
plan-dirty wiring gap (the band gate computed from `first_frame` only;
`replace_resource` setting no bit) is the implementing round's first
obligation.

## S3: collection members

Normative now: (a) a swap never changes the base pointer of a `Seq`/`Map`
member's ptr+len view; columns are sized at plan time and never resized, and
a foreign base pointer breaks provenance. (b) A swap never writes past the
member's `Cap` bound N; the new value shares the old value's exact
`Seq<T, N>` type, so the bound holds by construction. (c) Element bytes are
written into the member's existing collection-column region, up to N'
elements, then the length updates; the blob carries only the ptr+len view.

Contingent on the #344 write-side wiring: the concrete accessor performing
(c). Until that lands, a collection-bearing swap installs the scalar members
and the ptr+len view only; the element-write half is unimplementable, not
underspecified. The same resolution answers the founding spec's open
"resource collection accessor shape" item: collection access, read side and
write side alike, is the ptr+len view over the member's collection-column
region behind the erased descriptor; the concrete consumer-facing API lands
with #344, shaped by this clause.

## S4: trait relationship (resolved by sketch)

hilavitkutin-api ships `impl<T: PlanAffecting> !Replaceable for T` under
`#![feature(negative_impls)]` (WATCH tier, allowed; the gate carries the
standard vetting comment). `PlanAffecting` is dominant: a plan-affecting
type can never take the cheap `replace_value` path, structurally, with E0751
at the consumer's impl site, cross-crate. Evidence: the exclusivity sketch,
both arms. The supertrait alternatives fail on the merits
(`PlanAffecting: Replaceable` reopens the silent-skip hole;
`Replaceable: PlanAffecting` routes every app-state swap through the plan
band); a lint is advisory where a structural wall exists.

## S5: cost asymmetry (caller-facing contract)

`replace_value` costs one blob write plus, next frame, the store's dirty
cone: O(sizeof(V)) then O(cone). `replace_resource` costs one blob write
plus, next frame, the entire leading plan band: O(sizeof(T)) then O(plan
band). `replace_value` is the supported every-frame path. `replace_resource`
swaps must be rare relative to frame rate; an every-frame `PlanAffecting`
swap is misuse regardless of correctness. The sound path and the fast path
coincide here by structure: once the storage addendum killed per-member
decomposition there was exactly one candidate write shape.

## S6: test surface

The catalogued red at `tests/resource_bindings.rs:353` maps to S1; its
assertions are already the spec's, only its ignore reason is refuted (the
implementing round rewrites the reason to cite this spec, then removes the
ignore when the install lands). New tests: blob-address byte-identity across
a swap; double-swap-before-run, last value wins; ZST swap (dirty set, no
byte write, no UB); swap of an unread resource (dirty set and cleared, no
unit re-runs); `replace_resource` causes plan-band entry on a non-first
frame (catalogued red until the S2 wiring); collection swap asserting
base-pointer stability and length update (catalogued red on #344);
compile-fail double impl rejected with E0751.

## S7: bench

Swap-D (cold-write-then-stream; N swept 16/64/256, element sizes 4/16/64
bytes, baseline versus swap-at-frame-end): commissioned with the trigger
"when #344's write side lands". It decides whether the
`write_resource_collection_sizes` morsel term absorbs the swap's
write-then-read traffic or a separate budget line is owed. No accessor-shape
arm (one write per swap, between frames; the stakes do not justify one).

## Artifacts the implementing round corrects

All three carry the refuted member-by-member language A2-5 traced to the
tier-4 input memo; each contradicts S1: the engine DESIGN template's swap
paragraph, the `replace_value` FIXME's citation, and the catalogued test's
ignore reason. The S2 signature correction additionally touches
`replace_resource`'s bounds and the `PlanAffecting` seal alongside #696.
