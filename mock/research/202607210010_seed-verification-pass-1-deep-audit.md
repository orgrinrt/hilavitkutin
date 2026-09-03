# Seed Verification Report (independent pass, 2026-07-19)

Audit of `mock/research/seed/` (MANIFEST.md + 10 chapters) against the
consolidation spec (`design_rounds/202604200055/202603181200`), the A2
precedence record (`pre_seed/202607193200`), the registered amendment chain
(unified-engine `202606061000`, A1 `202606111400`, storage addendum
`202606210600`, the GATE-2 chain, round-level amendments), and the
2026-07-19 evidence records `202607200800` through `202607202340`. All
chapters read in full; spec read in full; A1, A2, unified-engine, storage
addendum, deviation ledger headers, swap spec `202607200200`, dossier
`202607200300`, and all nine evidence records read.

## Findings

### 1. Completeness

**C-1 MAJOR. Shared-read-column strategy omitted; manifest claims it
drained.** Seed: `plan.md` (whole chapter) and `execution.md`; MANIFEST.md
line "spec domain 11 (... shared-read-column approaches ...)". Source: spec
domain 11, "Shared read columns between trunks" (T6 L3635-3670): Approach 1
(snapshot-to-local, 1.6 percent overhead) versus Approach 2 (aligned morsel
sync, zero copy), with the ARM-versus-NUMA recommendation. No seed chapter
states this mechanism (the only trace is "trunks may share read-only
columns"), yet the manifest's drain record claims domain 11's shared-read
approaches drained into plan.md, and A1-3 lists "shared-read-column
strategy" as a canon feature group of the completion arc. Either the
content lands in plan.md/execution.md or the manifest's losslessness claim
is corrected.

**C-2 MINOR. repr(C) enforcement on domain newtypes dropped.** Seed:
`data-model.md` "Column value types". Source: spec domain 05 ("repr(C) on
domain newtypes enforced by `#[derive(ColumnValue)]` macro or lint, not by
the trait itself"). The seed states the trait contract and the 16-byte cap
but not the repr(C) enforcement route for stored domain newtypes.

**C-3 MINOR. Inner morsel cube axes dropped.** Seed: `plan.md` "Morsel
model". Source: spec domain 12 (inner cube: x micro-morsel by y active
columns per op by z op index). The seed keeps the outer 3D window and
micro-morsel activation but drops the inner-cube axes; borderline
distillation, listed for the manifest's losslessness standard.

No other omissions found: the nine plan steps, devirt rules and approach
menu, const-eval grouping chain, fiber feasibility check, morsel formula,
virtual flag system, meta pipeline, version stamps, error model, R1-R9,
A1 rulings and all seven constraint notes, both A2 standards, the storage
addendum in full (numbers verified against the addendum), the fairness
correction, worker-side barrier, designated-core bands, clock slot, and
per-arm gate are all present and correctly stated.

### 2. Correctness (A2 precedence)

**K-1 MINOR. Governance enumerates the tier-2 chain out of the date order
its own sentence requires.** Seed: `governance.md` "Sources and precedence"
item 2 lists "r2 (`202606081500`), the unified-engine amendment
(`202606061000`), the GATE-2 rechart (`202606070100`) ... the fairness
correction (`202606081100`), r5 ...". Source: A2-1 and A2's registry state,
which order the chain by date (unified-engine 06-06; rechart 06-07; r3, r4
06-07; fairness 06-08 1100; r2 06-08 1500; r5 06-08 1600). The rule text
("in date order") is correct, but a reader applying the enumeration order
would apply r2 before amendments that postdate it loses to. Reorder the
enumeration.

**K-2 MINOR. "Eleven crates" is a miscount.** Seed: `identity.md` "Crate
structure". The chapter enumerates twelve crates (api, api-macros, engine,
build, ctx, persistence, str, kit, providers, linking, extensions,
extensions-macros); `mock/crates/` holds twelve. The count "Eleven" is
wrong (it mirrors the same stale count in the repo CLAUDE.md, whose own
table omits the engine).

No seed statement was found that contradicts the consolidation spec, a
registered amendment, or a dated superseding record on a design point,
with the currency exceptions in section 4 (which are staleness against
2026-07-19 records, not mis-transcription of canon).

### 3. Internal consistency

**I-1 MINOR. The execution deviation set does not match A2-4's six-entry
enumeration or the manifest's drain record.** Seed: `execution.md`
"Recorded deviations awaiting evidence-then-bless" (deviations 1-6) versus
`MANIFEST.md` (execution drains ledger "sections 2, 4-and-its-supersession,
5, 6, 7, 10") and `governance.md` item 5. A2-4's six agent-call entries are
ledger sections {2 PoolFrame, 4 barrier, 5 park, 6 spawn, 7 aliasing,
10 scratch}. The chapter's six substitute the superseded barrier (section
4, correctly narrated instead in the frame-protocol paragraph) with the
already-op-blessed ownership mask (ledger section 1, which A2-4 and the
dossier place outside the batch). The substitution is defensible (the mask
still carries a standing bench-gate obligation; the barrier carries
nothing), but chapter, manifest, and A2-4 currently name three different
"six"-sets. State the mapping explicitly in execution.md or the manifest.

**I-2 covered under U-2/U-3.** Governance item 5 records the deviation-5
soundness hole and the deviation-3 bench ruling; the execution chapter
still states the pre-evidence position for both. Chapter-versus-governance
contradiction, filed under currency below.

No other cross-chapter contradictions found; vocabulary, thresholds,
formulas, and cross-references are mutually consistent.

### 4. Currency (2026-07-19 evidence records not yet reflected)

Governance.md is fully current (all nine records reflected, item by item).
The chapters lag it in three places:

**U-1 MAJOR. storage.md's replacement section is stale against the shipped,
benched swap spec.** Seed: `storage.md` "Replaceability" + "Replacement
semantics remain unspecified". Sources: `202607200200` (S1-S7) and
`202607201100` (both changelists locked, S6 green, S7 benched; op
ratification pending). Three concrete staleness points: (a) "the
`Replaceable` marker gates `replace_resource`" contradicts the shipped S2
(`replace_resource` is keyed to `PlanAffecting`; `replace_value` to
`Replaceable`); (b) the S4 exclusivity (`impl<T: PlanAffecting>
!Replaceable for T`, shipped with a compile-fail doctest) is absent, while
the chapter's "mirroring the Replaceable opt-in" phrasing implies the
markers coexist; (c) "`replace_value` marks dirty without installing" and
"the specifying design round is commissioned" describe the pre-round state,
not the implemented-and-benched-awaiting-ratification state governance
item 1 records. Pending ratification the open-question stance is correct in
principle, but the chapter must at minimum point at the spec under
ratification as governance does, and (a) must not state a gating claim the
locked implementation contradicts.

**U-2 MAJOR. execution.md deviation 5 asserts a soundness claim a dated
audit refuted.** Seed: `execution.md` deviation 5 ("made sound by the
parked-between-frames invariant plus column disjointness"). Source:
`202607201600` (the #689 audit): the claim does NOT hold as shipped; the
between-frames `&mut` surface aliases a parked worker's live `&Scheduler`
(miscompilation-class), and the deviation 1+6 plane relocation is upgraded
to soundness-required. Governance item 5 records the hole; the chapter
states the refuted claim as fact.

**U-3 MINOR. execution.md deviation 3 and the parking-tier paragraph
predate the wake-policy ruling.** Seed: `execution.md` deviation 3
("blocked on the telemetry widening") and "Frame protocol and barriers"
(predictive spin/spin-backoff/park tiers stated without qualification).
Source: `202607202340`: the middle tier was built (`RunCfg::
WAKE_SPIN_BUDGET`, budget-sweep test), benched, and lost at every size;
park-immediately is the blessed default (0), machinery ships tunable, and
any future `pick_tier` selection must beat it. The chapter should carry the
resolution, not the blocked-on-telemetry framing.

No currency gap found for the RCM record (plan.md Step 5 already cites
`202607200800` and carries the retraction of the 1.10x chain claim), the
spectral record (plan.md deviation note matches "awaits the op bless"), the
mid_slot call (dispatch wording needs no change, per the record itself),
LIGHT_THRESHOLD (execution.md "named and deliberately unvalued" matches the
registration), or deviations 2 and 4 (the chapter's inline dispositions
match the records).

## Verdict

**FIT AFTER LISTED FIXES.** The seed is a faithful, near-lossless
consolidation: chapter after chapter matches the spec and the registered
chain verbatim in substance, the A2 precedence regime is applied correctly,
and governance.md tracks all nine pre-freeze evidence records accurately.
Nothing rises to BLOCKER: no seed statement misstates settled canon in a
way the precedence order cannot untangle, and the seed's own lifecycle
already forbids freezing before the open-question batch lands in the
chapters. The three MAJOR items are exactly the freeze-path work: one real
omission the manifest wrongly claims drained (shared-read-column strategy),
and two chapters (storage, execution) whose in-place open-item text now
contradicts dated evidence records that governance already reflects, one
of them a refuted soundness claim that must not survive into a frozen text.
The MINOR items (deviation-set mapping, chain enumeration order, crate
count, two distillation drops) are one editing pass. With those fixes plus
the pending op rulings the governance lifecycle already gates on, the seed
is fit to freeze.
