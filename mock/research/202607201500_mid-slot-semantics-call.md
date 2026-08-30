# Design Call: `mid_slot` Is a Record Boundary

**Date:** 2026-07-19
**Status:** the item-7 design call of the seed pre-freeze batch
(governance: "record boundary versus arena slot; the convergence design
decides and records it; a design call in the batch, not an op gate").
Made and recorded here per the batch's standing instruction.

## The call

`RecordRange::Head { mid_slot }` / `RecordRange::Tail { mid_slot }`
(`hilavitkutin-api/src/dispatch_codegen.rs`) carry a RECORD boundary: an
index into `0..record_count`, the same unit as `RecordRange::Full` and
every other range in the dispatch surface. Head processes records
`0..mid`, tail processes `mid..record_count`. The value is
morsel-aligned by construction at plan time (the morsel formula already
aligns to multiples of four), which is a constraint on the value, not on
the unit.

## Why record, not arena slot

Three converging reasons. Uniformity: the enum is a RECORD range;
`Full` is record-indexed, and a mixed-unit enum would make every
consumer of the variant re-derive which unit it holds. Layout
independence: an arena-slot unit would couple the range to the column
layout (per-column strides differ), so one mid could not describe the
split across a fiber's columns; the record index describes it once and
the record-to-address mapping happens at dispatch exactly as it does
for morsel windows. Protocol fit: the proven head+tail convergence
protocol (the packed low/high cursor CAS, sketch-proven exactly-once
claiming) operates in record-index space; a slot-unit mid would need
translating back before initialising the cursors.

## Two notes recorded with the call

The field NAME `mid_slot` misleads (slot reads as arena vocabulary);
the rename to `mid_record` is owed and lands with the G2-Nd convergence
build round that wires these variants into dispatch, per the pre-1.0
no-shims rule (delete-and-replace in one round, no alias).

Relationship to the dynamic protocol: the proven convergence protocol
converges by CAS from both ends and needs no static midpoint; the
static `Head`/`Tail` split is the codegen-time SEED of the two cursor
starting halves (head cursor starts ascending at 0 bounded by the
claim frontier, tail descending from `record_count`), not a fixed wall.
`mid_slot` therefore names the initial balance point the plan
estimates; the protocol may converge elsewhere. The convergence build
round encodes this in the variant docs when it wires the path.

## Seed effect

Governance item 7 closes as a made call (no op gate was claimed for
it). The dispatch chapter's convergence wording needs no change; the
record-unit statement above drains into the registry as the item's row
when the seed freezes.
