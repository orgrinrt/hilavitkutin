**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** Adversarial audit of round-4's locked plan + sketch outcomes by an external senior reviewer. Captures every critical finding (C1-C4) and major concern (M1-M6) with specific remediation actions, file paths, and the order of work. Doc CL does not lock until C1-C4 plus M1-M6 are addressed.
**Source topics:** Topic 1, 2, 3 (kit trait split), 4 (sketch hypotheses), and 5 (synthesis at `202605042200_topic_round_4_findings.md`). Audit dispatched 2026-05-05 against the synthesis.

# Topic: round-4 audit findings and remediation plan

## Why this topic exists

The synthesis topic at `202605042200_topic_round_4_findings.md` declared the round green and ready for doc CL writeup. An external senior reviewer audited the full plan against the substrate's existing code and the workspace rules, and returned four critical issues plus six major concerns. The user directive: address all of them in full, do not lock the doc CL until they are.

This topic captures the audit's findings as a single document so post-compact sessions can resume work without re-reading the multi-thousand-word audit report. Each finding has a concrete remediation action, the file paths involved, and where it sits in the work order. The synthesis topic stays committed as audit trail, but it overstated the round's readiness; this topic supersedes the synthesis's "ready to write doc CL" claim.

## Critical issues (must address before doc CL)

### C1: S5 is paper, not empirical

The S5 directory `mock/research/sketches/202605050615_kit_taxonomy/` contains `FINDINGS.md` and `sketch_skeletons.md`. There is no `sketch.rs`, no compiler invocation, no `WORKS` outcome that means anything. The conclusion ("two committed axes suffice") is asserted in prose, not derived from a compiled experiment. This violates `cl-claim-sketch-discipline.md` which explicitly requires sketches to include "the actual code being tried, in fenced code blocks or as standalone .rs files inside a directory-shaped sketch."

**Remediation.** Write `mock/research/sketches/202605050615_kit_taxonomy/sketch.rs` that compiles the three kit shapes (MockspaceKit, BenchTracingKit, LintPackKit) under the two-axis annotation surface, exercising:

- `Replaceable` opt-in marker on a subset of the Owned types per kit.
- `pub(crate)` visibility wrapping the locked-down types.
- A cooperative-public path where one kit's Owned type is reachable from another kit's WU.
- A negative test: `replace_resource::<NotReplaceable>(...)` fails to compile with an unsatisfied `T: Replaceable` bound.

Update `FINDINGS.md` post-compile with the actual outcome (WORKS / FAILS / INCONCLUSIVE), citing the compile artefacts.

### C2: Cross-kit Owned reachability is enforced by Rust visibility, not by the typestate

The S1 missing-resource error trace contains `Cons<OuterA1, Cons<MidA1, Cons<LeafB1, Cons<LeafA1, Cons<LeafA2, ...>>>>>`. These are other kits' Owned types appearing in the accumulated read set. The typestate's `Stores: ContainsAll<Wus::AccumRead>` proof succeeds because every Owned-type accumulates into Stores via `K::Owned: Concat<Stores>`. The typestate has no notion of "kit-private."

Topic 3 line 71-73 framed `pub(crate)` visibility and sealed-supertrait wrapping as a substrate-enforced scoping axis. The substrate enforces nothing; Rust's standard visibility rules do all the work. A `pub(crate)` Owned type from kit A that lives in the same crate as kit B's WU declaration is reachable by name from kit B, and the typestate happily threads the proof.

**Remediation.** This is an honesty/framing issue, not a code change. Action items:

- This audit topic captures the corrected framing for the post-compact session.
- The doc CL author writes the visibility axis honestly: typestate sees all Owned types in the accumulated `Stores`; Rust's module-visibility rules govern which names a kit's WU declarations can reference. The two layers compose: typestate proves the type exists in `Stores`; Rust visibility proves the type's name is reachable at the WU's source location. A type that's not name-reachable cannot be referenced in a WU's `Read` or `Write` set, so its presence in `Stores` is harmless.
- The doc CL must not claim substrate-level enforcement of kit-private state.

Topic 3 itself stays committed (frozen). The corrected framing lands in the doc CL.

### C3: Depth 5 was Topic 3's design target but only depth 4 was tested

Topic 3 line 147: "Question. Does the typestate-builder approach sustain under realistic kit nesting depth (3-5 levels, 20+ WorkUnits, 10+ stores)?" S1 covered 4 levels, 9 WUs, 11 stores. The S1 FINDINGS rationalised this as "round-4 needs depth 4-5; this sketch confirms 4 is fine." That is sleight of hand: confirming 4 does not confirm 5.

The synthesis projects `O(N²)` cost out to N=100 at depth 6 from a single 0.56s data point at N=30. With `#[marker]` Contains, the trait solver explores multiple matching impls when `H = X` is possible, so the search is non-deterministic and the constant factor in the projection is not stable. Single-data-point projections are unsound.

**Remediation.** Extend S1's sketch (or create S1b) to depth 5 with 25 WUs and 12-15 stores. Concretely, add a fifth tier above RootKit (call it MetaKit) with 4 to 6 additional WUs that read from RootA and write to a new MetaA marker. Test:

- Success path compile time, comparing depth 4 to depth 5.
- Missing-resource error message readability and length at depth 5.
- Bundle accumulator's duplicate count if any leaf-tier resource bubbles up through the new tier.

Capture the data in S1's FINDINGS.md (or an S1b FINDINGS.md). If depth 5 compiles in under 5 seconds with usable error messages, the round-4 plan stands. If compile time crosses 30 seconds or the error messages become unreadable (no marker name visible in the noise), pivot per Topic 4's bitset fallback or accept depth 4 as the realistic ceiling and document it.

### C4: `feature(marker_trait_attr)` is the substrate keystone, and it is a 9-year-old unstable feature with disputed semantics

The synthesis treated `marker_trait_attr` as equivalent to `const_trait_impl` in stability story. The audit observes that `const_trait_impl` is on a stabilisation track with substantial recent activity, while `marker_trait_attr` (`rust-lang/rust#29864`, dating to 2017) has had no stabilisation movement and periodic relitigation about its coherence semantics. If a future rustc tightens or reshapes marker-trait coherence rules, the entire AccessSet substrate has to be redesigned.

**Remediation.** Two parts:

- The doc CL explicitly captures `feature(marker_trait_attr)` as a single point of failure for the AccessSet substrate, alongside the same risk capture format used for other nightly features the substrate accepts (per `arvo-compile-time-last.md`). State the risk plainly: AccessSet works because marker traits permit overlapping impls; a coherence-rules change to marker traits forces a substrate redesign.
- The Topic 4 bitset fallback path is named in the doc CL as the planned response if the keystone fails. Concrete shape: encode AccessSet as a compile-time bitset over a registered store-table indexed by `ConstParamTy` markers. This is **not** built in round-4; it is the documented escape hatch. A short research note (under `mock/research/`) capturing the fallback's general shape lands as part of the doc CL writeup.

The substrate's other nightly-feature dependencies (`adt_const_params`, `generic_const_exprs`, etc.) have similar risk profiles; the doc CL's nightly-feature risk capture should treat them as a class, not single out marker_trait_attr.

## Major concerns (must address before doc CL)

### M1: Factual errors in S2 FINDINGS

S2 FINDINGS line 113 says "v0.1's access.rs hand-codes ~1500 lines of macro-generated impls. B's recursive impl is dozens of lines." The actual file is 188 lines. v0.1 already ships **both** flat-arity-2-through-12 impls **and** the recursive `(H, R)` Cons-style impl on line 188, with `#[marker]` already applied at line 32. The "candidate A vs candidate B" framing was misleading: v0.1 is already A+B in coexistence, and round-4's actual change is to delete the flat impls and rename the existing primitives.

**Remediation.** Edit `mock/research/sketches/202605050503_accessset_arity/FINDINGS.md`:

- Replace "1500 lines" with the verified count (188 at audit time).
- Add a "## Correction (post-audit)" section noting that v0.1 already ships both shapes; the round-4 change is to delete the flat impls and rename the primitives, not to switch between alternatives.
- The recommendation B itself stands; the framing context updates.

Sketch FINDINGS files are not topic files; per `cl-claim-sketch-discipline.md`, factual corrections via append-and-supersede are appropriate. The deprecated framing is preserved as audit trail; the correction is the new ground truth.

### M2: Bundle accumulator duplicates compound the depth-5 risk; deferring to round-5 is wrong defensive ordering

The S1 sketch's `WorkUnitBundle::AccumRead` recursively concatenates each WU's Read set into the bundle's accumulated set. If two WUs both read `Clock`, the accumulated list contains `Clock` twice. The proof works, but each duplicate doubles the trait-solver's per-element work for that proof.

The synthesis's O(N²) projection ignored this multiplier. At depth 5 with 25 WUs and a realistic share-rate (every leaf reading the interner, every mid layer reading a clock), the duplicate count grows roughly N times share-fanout. A 25-WU bundle with 5x share might produce a 125-element accumulated list. The cheap fix (type-level dedup-on-Concat) should land with the expensive validation (depth-5 sketch from C3), not after.

**Remediation.** Implement a type-level `Difference<L, R>` or `RemoveDup<L>` on the cons-list substrate before the doc CL. Two candidate shapes to sketch:

- Type-level set-difference walking L for each element of R, removing matches. Recursive, structurally similar to ContainsAll.
- Type-level `Concat-dedup` that during concatenation skips an element of the right operand if the left already contains it.

Sketch under `mock/research/sketches/<TS>_dedup_concat/`. Verify the trait-solver's behavior on the resulting deduplicated lists is consistent with the depth-5 success path from C3. If the dedup operator itself has trait-solver pathology, the round-4 plan accepts the duplicate cost and the doc CL captures the inefficiency as a cost.

### M3: Error-message readability at depth 5+ is unverified

The S1 sketch at depth 4 already emits a 9-element nested `Cons<...>` chain in the missing-resource error and the rustc "long type written to file" note fires. The synthesis dismissed this as "Workable. Not a blocker." That is defensible at depth 4. At depth 5 with N=50 the cons-chain in the error is roughly 50 entries; at depth 6, 100 entries.

**Remediation.** The C3 depth-5 sketch captures the missing-resource error message verbatim. Compare to the depth-4 error from S1. Decision criterion: the offending marker's name must appear in the inline error text (not buried in the long-type file note) for the error to count as readable. If at depth 5 the marker name is buried, add a `trybuild` fixture that asserts the inline text contains the marker name; the doc CL ships a richer `#[diagnostic::on_unimplemented]` annotation that surfaces the marker name at the top of the message.

The trybuild fixture itself is a round-5 candidate, not a round-4 blocker, but the depth-5 verification is.

### M4: Cross-kit reachability framing dishonest

Subsumed by C2. The doc CL must describe the layered enforcement honestly. No additional remediation beyond C2.

### M5: Topic 3 named depth 3 to 5; sketch confirmed only 4

Subsumed by C3. The depth-5 extension closes this gap.

### M6: `Required derived from Units::AccessSet \ Owned` is rhetorical, not type-level computed

Topic 3 sells `Required` as mechanically derivable from `Units::AccessSet \ Owned`, with a hint at a `Kit::requirements_doc()` helper that emits the requirement list at compile time. The S1 sketch does not compute the set difference; it propagates `AccumRead`/`AccumWrite` and proves `Stores: ContainsAll<...>` against the indistinguishable union of Owned and app-Resource entries.

To compute the difference, the substrate needs a type-level `Difference<L, R>` or `NotIn<X, L>`. Round-3's NotIn proof already showed this cannot be soundly encoded under coherence (`mock/research/sketches/registrable-not-in-202605051200/FINDINGS.md`). The "Required is derivable" claim therefore rests on a mechanism that does not exist.

**Remediation.** Two options for the doc CL:

- **Drop the `requirements_doc()` claim entirely.** The substrate proves correctness via the `ContainsAll<AccumRead>` plus `ContainsAll<AccumWrite>` checks at `.build()`. Documentation of "what a kit requires" lives in prose (the kit author lists the required Resources in the Kit's rustdoc), not in compile-time-derivable form.
- **Build the difference operator.** This depends on the M2 dedup work. If `Difference<L, R>` becomes feasible as part of M2's exploration, the `requirements_doc()` claim survives. If not, fall back to option 1.

Recommend option 1 unless M2 shows the difference operator is cheap. Option 2 reopens the round-3 NotIn problem at a different angle.

## Items the audit confirmed as sound

For completeness, the audit identified seven points where the plan stands as proposed:

- Dropping `Required` from the trait definition (G1; M6 is about the secondary `requirements_doc()` claim, not the trait shape).
- The two-axis scoping conclusion appears correct contingent on C1's verification (G2).
- Pre-1.0 churn licensing BuilderResource deletion (G3).
- Replaceable opt-in is sound and well-defended (G4).
- Compile-time-last framing correctly applied (G5).
- Coherence with `hilavitkutin-workunit-mental-model` holds (G6).
- Primitive-vocabulary discipline preserved (G7).

These do not require remediation. They are listed so the post-compact session does not relitigate them.

## Work order for the post-compact session

The audit's recommended sequence, refined for the workspace's actual constraints:

1. **C1 first.** Write the real S5 sketch. Lowest-risk piece, validates the two-axis conclusion empirically, and grounds the rest of the work in the round's actual surface.
2. **C3 next.** Extend S1 to depth 5. Captures depth-5 compile time and error messages in one shot.
3. **M1.** Edit S2 FINDINGS with the factual correction. Append `## Correction (post-audit)` section per the M1 remediation note. Mechanical, fast.
4. **M2 + M3 together.** Sketch the dedup-on-Concat operator under `mock/research/sketches/<TS>_dedup_concat/`. Verify against C3's depth-5 baseline. M3's error-message work is captured during C3's depth-5 run.
5. **M6.** Decide drop-or-build for `requirements_doc()`. If M2's Difference operator is cheap, build. Otherwise drop.
6. **C4.** Doc CL captures marker_trait_attr risk + bitset fallback shape. Mechanical, written during the doc CL itself.
7. **C2 / M4.** Doc CL captures the layered-enforcement honesty. Mechanical, written during the doc CL itself.

After all seven steps land: write doc CL, lock, then SRC CL per the standard flow.

## Branch and commit state at audit close

- Branch: `feat/builder-register-unification`.
- Latest commit: `85abfe8` (synthesis topic).
- Topics committed: 1-3 at `72055dc`, 4 (sketch hypotheses) at `9d48bf2`, 5 (synthesis) at `85abfe8`.
- Sketches committed: S2 at `93363eb`, S1 at `41af131`, S4 at `2d5c917`, S5 (paper-only, see C1) at `efb80ef`.
- Round still in TOPIC phase. No doc CL exists.

## Cross-references

- `mock/design_rounds/202605042200_topic_kit_trait_split.md`. Topic 3 (locked Kit shape).
- `mock/design_rounds/202605042200_topic_round_4_sketches.md`. Topic 4 (sketch hypotheses).
- `mock/design_rounds/202605042200_topic_round_4_findings.md`. Synthesis (the audit's primary input).
- `mock/research/sketches/202605050503_accessset_arity/FINDINGS.md`. S2 (target of M1 correction).
- `mock/research/sketches/202605050530_deep_stacking/FINDINGS.md`. S1 (target of C3 extension).
- `mock/research/sketches/202605050615_kit_taxonomy/FINDINGS.md`. S5 (target of C1 sketch authoring).
- `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md`. The discipline C1 violates.
- `~/Dev/clause-dev/.claude/rules/arvo-compile-time-last.md`. Frames C4's keystone risk.
- `~/Dev/clause-dev/.claude/rules/local-pr-review-flow.md`. Authorises this audit pattern.
- Tasks: #330 (umbrella), #333 (load-bearing for C3), #361-#363 done, #364 (doc CL, blocked by audit work), #365 (SRC CL plus close). New task IDs assigned for each audit action item; see post-compact session task list.
