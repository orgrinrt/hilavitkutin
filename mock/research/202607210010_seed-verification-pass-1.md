# Seed Verification Pass 1: Four Independent Audits and the Fix Batch

**Date:** 2026-07-19
**Status:** the first of the two verification passes the seed lifecycle
requires (governance.md: verification precedes freeze; the second pass
runs on the candidate registry after drain).

## The pass

Four independent audits ran in parallel against `mock/research/seed/`:
one deep canon audit (top-tier agent; seed vs the consolidation spec, the
A2 precedence chain, and all nine pre-freeze evidence records, everything
read in full) and three focused checks (cross-reference and manifest
integrity; governance-ledger claims vs their cited records; seed source
claims vs the shipped tree). All prompts were neutral; none saw the
others' output.

## Results

The ledger check verified all eight governance items against their cited
records with zero discrepancies (every figure quoted matches its record).
The cross-reference check found all 58 chapter links and all cited paths
and round identifiers resolving (one pre-freeze-expected note: the
registry directory is created at drain). The deep audit's verdict: FIT
AFTER LISTED FIXES, no blockers; three MAJOR findings (shared-read-column
strategy omitted while the manifest claimed it drained; storage.md's
replacement section stale against the shipped swap spec, including an
inverted marker pairing; execution.md deviation 5 asserting the soundness
claim the 202607201600 audit refuted) plus five MINOR (crate miscount,
tier-2 enumeration order, morsel-cube axes, repr(C) enforcement note,
deviation-set numbering mapping). The source spot-check confirmed twelve
seed claims against the tree and surfaced the drift the deep audit's
storage/execution findings overlap, plus the founding-spec-shaped
`WorkUnit` block, the pre-`Api`-suffix platform trait spellings, bare
primitive spellings in two code blocks, the `SchedulerMetrics` designed
field set stated as current, and `PipelineResult` labelled committed while
unshipped.

## The fix batch (applied 2026-07-19, same session)

Every finding above was fixed in place: storage.md's replacement section
rewritten to the shipped S1-S7 state with the correct marker pairing and
the S4 exclusivity; execution.md deviation 5 rewritten to the confirmed
hole and soundness-required relocation, deviation 3 to the wake-policy
bench ruling, the deviation-set numbering note added, and the metrics
sentence split into designed vs currently-shipped fields; plan.md gained
the domain-11 shared-read strategies and the inner-cube axes; contracts.md's
`WorkUnit` block updated to the locked-round canonical shape (no `NAME`,
GAT `Ctx<'frame>`, arvo `Bool`); foundations.md's platform traits updated
to the shipped `*Api` names and arvo-typed signatures; data-model.md
gained the repr(C) enforcement note and the `USize`-typed `BIT_WIDTH`;
identity.md's crate count corrected to twelve; governance.md's tier-2
enumeration reordered by date and the registry path annotated.

## Standing

Pass 1 is complete: the seed as committed reflects every finding. The
freeze remains gated on the op rulings governance item 5 and items 1/3
await; the second verification pass runs on the candidate registry.
