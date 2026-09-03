# Canonical Design Addendum A2: Precedence, Rulings, and a Citation Correction

**Date:** 2026-07-19
**Status:** locked (op rulings recorded live, 2026-07-19; the precedence ruling op-delegated to
a provenance trace, evidence cited inline)
**Amends:** the consolidation spec
(`mock/design_rounds/202604200055/202603181200_topic.hilavitkutin-design-consolidation.md`) as
modified by the chain registered in A1 (`202606111400`), extended below.

## A2-1: Document precedence (traced ruling)

Op delegated this ruling to a provenance trace of the record (2026-07-19): "Trace the timestamps
and content, it's all inferrable in context." The trace and its conclusion:

The standalone spec (`202606111800_canonical-spec-standalone.md`) claims in its header that
"where this document and any older one disagree, this document is the design." That claim
exceeds its commission. The commissioning text is A1 "Sequence from here" item 3, op-blessed
live 2026-06-11: the standalone is written "fully self-contained, readable with no prior
project knowledge ... superseding the need to cross-read the chain registered above." The
commission grants the role of a faithful consolidation that spares the reader cross-reading. It
does not grant conflict-winning authority, and a faithful consolidation cannot legitimately win
conflicts against its sources: any disagreement with them is by definition a transcription
defect in the consolidation. Provenance agrees: A1 is op-locked; the standalone entered the
repo unsigned inside the GATE-2 feature squash (`ff147c1a`, 2026-06-19), never through its own
reviewed round.

The precedence order is therefore:

1. The consolidation spec (`202603181200`), the founding canon.
2. Op-ruled and bench-decided amendments, in date order, each modifying canon where it
   explicitly speaks: the chain registered in A1, plus the unified-engine amendment
   (`202606061000`, op explicit ruling 2026-06-06, registered below), plus A1 itself, plus the
   storage addendum (`202606210600_resource-storage-model-canonical-addendum.md`, bench-decided
   with op's hybrid call 2026-07-02), plus this addendum, plus later dated rulings. A later
   ruling wins over an earlier one on the same point. Where an amendment is silent or
   ambiguous, the consolidation spec's wording is the tiebreak.
3. The standalone spec: a commissioned reading aid with no independent design authority. Where
   it conflicts with tiers 1 or 2, the conflict is a transcription defect in the standalone, to
   be fixed there. Its header is corrected to state this (same commit as this file).
4. Everything else (roadmaps, audits, expert memos, sketches, DESIGN.md.tmpl, agent memory):
   intermediate artifacts, never canon.

This subsumes the informally reported 2026-07-19 ruling that r7 carried ("neither the
standalone nor the unified-engine amendment blanket-wins"), with one refinement the trace
grounds: the unified-engine amendment DOES carry op-granted authority over intermediate
artifacts (its own status line, op explicit), while the standalone carries none.

## A2-2: Registry addition

The unified-engine amendment (`202606061000_canon-amendment-unified-engine.md`, op explicit
ruling 2026-06-06) is missing from A1's amendment-chain registry despite its canon status. It
is hereby registered as part of the chain, positioned by its date (before the rechart
`202606070100`).

## A2-3: PlanAffecting ranges over an open marker trait (op, 2026-07-19)

Canon's plan-dirty trigger (a swapped plan-affecting resource marks the plan for recompute) had
no consumer-reachable carrier: `PlanAffecting` shipped sealed with one engine-internal
implementor. Op ruled: unseal it. Consumers implement `PlanAffecting` on resource value types
whose swap invalidates the plan, mirroring the existing `Replaceable` opt-in marker. The
property is intrinsic to the type, not to one app's registration. Implementation is tracked
under #696 and rides the swap-semantics round (A2-5), since the two surfaces interact.

## A2-4: Disposition of the GATE-2 agent-call deviations (op, 2026-07-19)

The six ledger entries tagged `[agent-call] pending op review` in `202606072100` (PoolFrame
inline + Pin, main-orchestrated waist barrier, park-immediately with no spin tier, pointer-size
spawn + exit-counter join, discipline-sound raw scheduler aliasing, inline GATE-2 scratch)
follow the evidence-then-bless standard: for each, the canon shape is built or sketched, benched
against the shipped shape where the ledger names a bench trigger, and presented to op with
evidence for a bless-or-rebuild ruling. None is blessed by default; none is rebuilt by default.
The same standard governs the spectral role deviation (canon step 7 trunk formation), per op's
2026-07-19 ruling on it.

## A2-5: The 202606210600 citation correction and the commissioned swap-semantics round

Two shipped rounds (the D1 install round and the 2026-07-19 revert round `202607193110`) cited
"`202606210600`" for the claim that a `Replaceable` swap is "a member-by-member copy of each
leaf into its column slot ... not an in-place blob memcpy." That sentence is from
`202606210600_expert-architect-storage-model.md`, an expert-panel INPUT memo which in the same
passage states "Swap semantics need explicit spec." It is tier-4 material under A2-1. The
round's canonical output, the storage addendum, decided the opposite layout (one-record blob;
per-member decomposition bench-refuted; `Decompose` scoped to size fold and collection
ptr+len) and never specified swap semantics. Both rounds were therefore grounded on a
mis-citation; the revert round's DESIGN.md.tmpl sentence, the `replace_value` FIXME, and the
catalogued test's ignore reason encode refuted-layout language.

Op's direction (2026-07-19): the missing swap spec is researched and designed explicitly, not
picked from options. A design round is commissioned to produce the explicit `Replaceable` swap
specification under the bench-decided layout (blob record, erased addressing, collection
elements in collection columns), covering `replace_value`, `replace_resource`, the
`PlanAffecting` interplay (A2-3), Seq/Map element ownership, dirty propagation, and quiescence.
The same round corrects the mis-grounded artifacts. Until it lands, `replace_value` stays
mark-dirty-only and the catalogued test stays red.

## Registry state after A2

Consolidation spec `202603181200`; unified-engine `202606061000` (A2-2); rechart `202606070100`;
r3 `202606070200`; r4 `202606070700`; fairness `202606081100`; r2 `202606081500`; r5
`202606081600`; round-level amendments per A1 item 6; A1 `202606111400`; storage addendum
`202606210600` (canonical-addendum file only; sibling memos are input material); A2 (this file).
The standalone spec `202606111800` is a reading aid outside the chain.
