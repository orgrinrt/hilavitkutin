**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** Define the empirical sketches that gate round-4 doc CL lock. Each sketch maps to one open item from `202605042200_topic_kit_trait_split.md`, fixes a hypothesis, names the success criterion, and names the design pivot if the hypothesis fails. Sketches commit to `mock/research/sketches/` per `cl-claim-sketch-discipline.md` before the doc CL is written.
**Source topics:** `202605042200_topic_builder_register_unification.md` (Topic 1, round 1+2 journey), `202605042200_topic_markers_as_registrables_final.md` (Topic 2, REJECTED; round-3 NotIn proposal), `202605042200_topic_kit_trait_split.md` (Topic 3, locked round-4 trait shape with O1-O4 open items).

# Topic: round-4 sketches, S1 to S5

## Why this topic exists

Topic 3 fixed the round-4 trait shape (`Kit { type Units: WorkUnitBundle; type Owned: StoreBundle }`, no `Required`, compile-time check via typestate builder) and named four open items that block doc CL lock. The open items are not aesthetic preferences. They are empirical questions about whether the locked shape sustains under realistic load (deep nesting, accumulated arity), under nightly-feature constraints (`feature(negative_impls)` for opt-out replaceability), and across diverse consumer kit shapes.

`cl-claim-sketch-discipline.md` says sketches commit before the doc CL locks. This topic is the contract that each open item turns into. Each sketch below names a binary outcome (works / fails / inconclusive) and the design pivot the failure outcome forces. After all sketches run, the round either proceeds to doc CL or restarts a new topic for whichever pivot the failure named.

The round-3 sketch on `NotIn<H>` is the canonical precedent. It committed under `mock/research/sketches/registrable-not-in-202605051200/` with `sketch.rs` plus `FINDINGS.md`. The hypothesis was specific (compile-time diamond detection via `feature(negative_impls)` and a `NotIn<H>` constraint), the outcome was empirical (`FAILS WITH coherence-overlap`), and the pivot was named in the topic-3 reframe (drop marker-as-registrable; restructure `Kit` so kits stop registering shared instances). Each sketch below follows the same shape.

## Run order

Sketches are not independent. S2 chooses the AccessSet arity shape that S1 then tests against under depth. S4 depends on whether `feature(negative_impls)` is reachable for `impl !Replaceable for Foo {}` (round-3 evidence on `NotIn<H>` raises questions but does not transfer directly: `Replaceable` is a marker on a single type, not a relation across a heterogeneous list). S5 is taxonomy completeness and runs against whatever S2+S1 produced. S3 is light verification and slots in wherever it fits.

Order:

1. **S2 first.** Arity solve. Three candidates head-to-head. Output: chosen `AccessSet` shape.
2. **S1 next.** Deep-stacking typestate-builder, using S2's chosen shape. Output: pass or pivot.
3. **S4 in parallel.** Replaceable polarity, independent of S2 and S1. Output: opt-in, opt-out, or pivot.
4. **S3 conditional.** A `#![recursion_limit]` verification. May be subsumed by S1; runs only if S1 leaves it open.
5. **S5 last.** Taxonomy completeness across three diverse kit shapes. Output: two axes suffice, or one new axis is needed.

If S2 fails for all three candidates, the round restarts: arity is no longer a band-aid, it is a structural blocker. New topic, new design.

## Sketch format reminder

Per `cl-claim-sketch-discipline.md`, each sketch lives under `mock/research/sketches/<YYYYMMDDHHMM>_<topic>/` with at minimum:

- `FINDINGS.md` naming the hypothesis, the outcome (`WORKS`, `FAILS WITH <error>`, or `INCONCLUSIVE`, where the inconclusive case implies deeper investigation is needed), and the next step the outcome unblocks or blocks.
- One or more `*.rs` files, or a fenced code block inside `FINDINGS.md` if the sketch is a single short snippet.

Sketches commit to the `feat/builder-register-unification` branch in chronological subdirectories. They are not deleted after the round; the audit trail is permanent. If a later sketch supersedes an earlier hypothesis, the earlier sketch's `FINDINGS.md` is renamed to `FINDINGS.deprecated.md` (or the whole directory is renamed) and the supersession is named in the new sketch.

## S2. AccessSet arity, three candidates head-to-head

`hilavitkutin-api`'s current `AccessSet` flat impls cap at arity 12 (per task #333). Round-4's typestate builder accumulates `Stores` and accumulated WU access through nested `.add_kit()` calls. With `MockspaceKit` pulling in `LintKit` and `BenchKit`, each carrying 5-8 WUs over 3-5 stores, arity 12 is hit at the second level of nesting. The flat-N cap is not a design; it is band-aid scaffolding that worked at v0.1's single-kit shape.

**Hypothesis.** One of three candidate shapes scales to depth 4-5 and arity 32+ without rustc trait-solver pathology, with readable error messages, and with mechanically-derivable union / intersection / difference operators (the latter is what `Units::AccessSet \ Owned` evaluates to at `.build()`):

- **A. Macro-flat impls up to N=64.** Same shape as today, just bigger. Macro generates `impl AccessSet for (T0, T1, ..., TN-1)` and the union/intersection/difference impls per N. Compile-time cost scales with N.
- **B. Recursive HList AccessSet.** `AccessSet for ()` and `AccessSet for (Head, Tail) where Tail: AccessSet`. Operations are recursive: `union(self, other) = (self.head, self.tail.union(other))` etc. Cleanest shape, deepest trait-solver path.
- **C. Const-generic-length over a typed array.** `AccessSet<const N: usize>` with associated `type Items: ItemList<N>` and TypeId-free disambiguation via const generics. Most exotic, may not be reachable on stable feature gates.

**Scope.** Implement each candidate in a fresh sketch crate that depends only on `hilavitkutin-api`'s relevant traits (or stub equivalents inside the sketch). For each, exercise:

- Construction: build an AccessSet of size 4, 12, 24, 48 with the candidate's surface.
- Union / intersection / difference at each size.
- Missing-element error: deliberately ask for an element not in the set and capture the rustc error verbatim.
- Compile time: `cargo +nightly rustc -- -Zself-profile` and capture the trait-solver-time line.
- Monomorphisation: `cargo +nightly rustc -- -Zprint-type-sizes` (or `--emit=llvm-ir` size proxy) and capture the largest type-size and total IR size.

**Success criterion.** A candidate passes when all four measurements are within 2x of the arity-12 baseline at arity 24, and within 5x at arity 48. The qualitative criterion: error messages from the missing-element case must name the offending element and the receiving operator, not bottom out as "the trait `AccessSet` is not implemented for ..." with no further context. The chosen candidate is the one that passes; if multiple pass, prefer the one with cleanest operator semantics (B is preferred on this axis if performance-comparable).

**Failure modes that change the design.**

- All three fail or pathological: the trait-solver-driven AccessSet shape is the wrong primitive. Pivot: AccessSet becomes a `const fn`-derived bitset over a registered store-table indexed at compile time (more like arvo's `Mask64` approach) and the typestate builder ratchets the bitset through `.add_kit()` calls. New topic.
- A passes but B and C fail: arity-N flat impls are accepted as the long-term shape; #333 closes by macro-generating up to N=64 (or whatever the bench shows is safe). The "band-aid" framing was wrong; flat-N is fine if N is generous.
- B passes alone: HList recursive shape is the long-term answer. The macro-N infrastructure can be retired. `Units::AccessSet \ Owned` becomes a recursive set-difference operator on HLists.

**Sketch directory.** `mock/research/sketches/<TS>_accessset_arity/` where `<TS>` is the launch timestamp. Three subdirectories `a_macro_flat/`, `b_hlist/`, `c_const_generic/`, each a tiny crate. `FINDINGS.md` at the top compares the three with a table of measurements.

**Maps to O2 (#333). Load-bearing.**

## S1. Deep-stacking typestate-builder

Topic 3's locked shape says `.add_kit::<K>()` returns a new typed `SchedulerBuilder` with `Wus` and `Stores` accumulated. `.build()` carries `where Self::AccumulatedAccesses: SatisfiedBy<Self::AllStores>` as a static bound. The `#[diagnostic::on_unimplemented]` message names the missing marker.

This is plausible at depth 1. The empirical question is whether it sustains at depth 4-5 with the AccessSet shape S2 chose, with realistic kit content (multiple WUs per kit, multiple stores per kit), and with both the success path and a deliberately-broken path.

**Hypothesis.** A 4-level kit hierarchy with 25 WorkUnits total and 12-15 stores total compiles in under 60 seconds incremental (under 180 seconds clean), produces readable error messages on the deliberately-broken path, and does not blow rustc's `#![recursion_limit]` from default (currently 128).

**Scope.** Build a synthetic kit hierarchy in a sketch crate:

- Level 4 (innermost): `LeafA`, `LeafB`, each with 4 WUs and 2 owned stores, requiring 1 shared store from app.
- Level 3: `MidA(LeafA, LeafB, OwnA)`, `MidB(LeafB, OwnB)`, each adding 3 WUs and 2 owned stores.
- Level 2: `OuterA(MidA, MidB, OwnOuter)` adding 4 WUs and 1 owned store.
- Level 1: `RootKit(OuterA, OwnRoot)` adding 2 WUs and 1 owned store.
- App: provides the 1 shared store the leaves required and calls `.build()`.

Two scenarios:

- Success: shared store provided. Expected: compiles, the typestate proves all WUs' accesses are covered.
- Failure: shared store not provided. Expected: compile error at the `.build()` line naming the leaf-level marker that has no provider.

Measure: compile time (clean and incremental), monomorphisation size (`-Zprint-type-sizes` cap), error-message readability (capture verbatim), and `#![recursion_limit]` requirement (does the consumer need to bump it?).

**Success criterion.** Both scenarios behave as expected, compile time stays under the targets above, error message names the offending marker and the leaf kit asking for it without burying it under five layers of associated-type machinery, and the default recursion limit suffices. If the recursion limit needs a bump, the bump is a single `#![recursion_limit = "256"]` at the consumer's crate root and the doc CL documents that as the consumer-side cost.

**Failure modes that change the design.**

- Compile time scales nonlinearly past depth 3 (e.g. depth 4 takes 5+ minutes clean): the typestate approach has a practical depth ceiling. Pivot: the engine accepts kits flat at the app level, kits do not nest, and `MockspaceKit` becomes a documentation pattern rather than a kit-of-kits. Topic 3's nested-kit-via-registration approach gets retired in favour of flat composition.
- Error messages are unreadable (deeply-nested associated-type chains, no concrete marker name): the `#[diagnostic::on_unimplemented]` story does not work at this depth. Pivot: the `.build()` static check is replaced by a const-fn check that produces a synthetic error name at compile time, or replaced by a `.build()`-time runtime panic with a clear message. Per the corrected `arvo-compile-time-last.md`, the runtime alternative is a fallback only if the compile-time path is empirically infeasible, not a default.
- Recursion limit explodes (consumer needs `#![recursion_limit = "1024"]` or higher): mark this in doc CL but not a pivot unless it crosses 1024 (above which rustc itself starts to misbehave).

**Sketch directory.** `mock/research/sketches/<TS>_deep_stacking/` with `kits/` directory mirroring the hierarchy and a top-level `FINDINGS.md`.

**Maps to O1.**

## S3. Recursion-limit verification

Light verification. May be folded into S1 if S1 already exercises the recursion-limit boundary.

**Hypothesis.** Deeply-nested kit registration does not require the consumer to set `#![recursion_limit]` above the rustc default of 128.

**Scope.** Take S1's depth-4 hierarchy, build it without any consumer-side `#![recursion_limit]` directive, capture the result. If rustc complains, find the smallest limit that compiles. If the depth-4 case passes at default, push to depth 5, depth 6 to find the depth at which the default fails.

**Success criterion.** The default recursion limit suffices for the realistic depth (3-5). If a bump is needed, it is single-digit-multiple of the default (256 or 512, not 4096).

**Failure modes that change the design.** Same as S1's recursion-limit failure mode. Documented as a consumer-side cost in the doc CL.

**Sketch directory.** Folded into S1's directory with a `RECURSION_LIMIT.md` annex. If S1 leaves the question open, a separate `mock/research/sketches/<TS>_recursion_limit/` is created.

## S4. Replaceable polarity, opt-in vs opt-out

Topic 3 named two polarities for the `Replaceable` marker: opt-in (kit author writes `impl Replaceable for Foo {}` for things the app may swap) or opt-out (default impl on every store, kit author writes `impl !Replaceable for Foo {}` to lock down). The polarity decision is unsettled because both shapes have arguments and one of them (opt-out) requires `feature(negative_impls)` which had a coherence-shaped failure in round-3 (different mechanism, but the feature gate's overall reliability is in question).

**Hypothesis.** Exactly one polarity is empirically usable. If both compile cleanly, the ergonomic comparison decides. If only opt-in compiles cleanly (because `feature(negative_impls)` produces coherence overlaps for `Replaceable` similar to `NotIn<H>` in round-3), opt-in wins by feature-availability.

**Scope.** Two parallel sketches in subdirectories of the same parent:

- **a_opt_in/.** Define `pub trait Replaceable {}`. Three example kits each with 3-4 owned stores. Author manually writes `impl Replaceable for Foo {}` on stores intended to be app-overridable. Count manual impls. Try `.replace_resource::<NonReplaceable>(...)` on a store that did not opt in; capture the compile error.
- **b_opt_out/.** Define `pub auto trait Replaceable {}` (or equivalent default-impl shape) and rely on `feature(negative_impls)` for `impl !Replaceable for Foo {}`. Same three kits; author writes `impl !Replaceable for Foo {}` on stores intended to lock. Count manual impls. Try `.replace_resource::<LockedDown>(...)`; capture the compile error. Critically: try multiple `impl !Replaceable for ...` declarations across the same crate and confirm coherence does not overlap (round-3's failure mode).

**Success criterion.** Both compile and produce comparable error messages: the polarity with fewer manual annotations across the three kits wins. If only one compiles, that one wins. Tie-break by error-message clarity.

**Failure modes that change the design.**

- Opt-out fails to compile (coherence overlap on `feature(negative_impls)`-based `!Replaceable`): opt-in is the locked answer, no further investigation needed.
- Both fail: the marker-trait approach is wrong. Pivot: `Replaceable` becomes a method-on-trait or a separate const-generic bound, not a marker. Topic 3's "two committed axes" shrinks to one (visibility only) and replaceability becomes a runtime decision. Doc CL must reflect.
- Both pass and the count of manual impls is identical or a wash: lock opt-in (more conservative, no nightly dependency, matches the substrate's "tools not policy" rule).

**Sketch directory.** `mock/research/sketches/<TS>_replaceable_polarity/` with `a_opt_in/` and `b_opt_out/` subdirectories.

**Maps to O3.**

## S5. Owned access-scoping taxonomy completeness

Topic 3 committed two axes for `Owned` access scoping: visibility (pub / pub(crate) / sealed-supertrait) and replaceability (Replaceable marker, polarity per S4). Topic 3 also listed three speculative axes (lifetime scope, build-order, init-source) marked as not committed. The empirical question is whether v1's three concrete kit shapes need any of the speculative axes or are covered by the two committed ones.

**Hypothesis.** Three diverse kit shapes (`MockspaceKit`, `BenchTracingKit`, `LintPackKit`) map cleanly to the two committed axes without needing any speculative axis.

**Scope.** Write skeletons (not full implementations) for three kit shapes:

- **MockspaceKit.** Owns multiple internal stores: a `Resource<RoundState>`, a `Column<DesignRound>`, and a `Resource<LintConfig>`. Requires `Resource<StringInterner>` and `Resource<Clock>` from app. Tests the common-case kit: multi-marker owned, multi-shared required.
- **BenchTracingKit.** Owns a `Resource<Tracer>` whose construction needs FFI handle setup before the scheduler exists (tests the speculative "lifetime scope" axis: does the kit need its Owned to outlive the scheduler?). Owns also a `Column<TraceSample>`. Requires nothing.
- **LintPackKit.** Owns a `Column<Diagnostic>` that other kits' WUs need to write to via a cooperative-public path (tests the "fully public" visibility option, axis 1). Requires `Resource<StringInterner>`.

For each, name how the two committed axes apply and whether any speculative axis is genuinely needed (not just nice-to-have).

**Success criterion.** All three kits map without invoking a speculative axis. Sketch findings document the mapping per kit.

**Failure modes that change the design.**

- One or more kit needs a speculative axis: that axis is committed to round-4. Topic 3's "two committed axes" expands to include the surfaced one. Doc CL widens to cover it.
- Two committed axes cover all three kits but feel restrictive (e.g. they cover but with unnatural patterns): this is not a failure; it is a documentation finding. Note in doc CL as "future axis candidates".

**Sketch directory.** `mock/research/sketches/<TS>_kit_taxonomy/` with three subdirectories per kit and a top-level `FINDINGS.md`.

**Maps to O4.**

## After all sketches run

If every sketch's hypothesis holds: proceed to doc CL (`202605042200_changelist.doc.md`). The doc CL is per-crate, per-file, mechanical, covering `hilavitkutin-api` (Kit trait, `StoreBundle`, AccessSet propagation contracts at the chosen arity shape, Replaceable surface), `hilavitkutin-providers` (kits become `{Units, Owned}` shape, `BuilderResource` removed/renamed), `hilavitkutin` engine (typestate builder, compile-time AccessSet proof at `.build()`), and `hilavitkutin-extensions` if any contract surface is affected.

If any sketch's hypothesis fails in the way the failure-modes section names: the round restarts with a new topic file naming the pivot. Topic 3 stays committed (frozen as audit trail), the sketch findings stay committed (audit trail), and the new topic supersedes both.

If a sketch reveals an issue not listed in its failure-modes section: stop. Add a finding to the sketch's `FINDINGS.md`, raise the issue, and decide between a focused new sketch and a pivot. Do not patch the topic file; topic 3 is frozen.

## Cross-references

- `mock/design_rounds/202605042200_topic_kit_trait_split.md`. Topic 3, the locked round-4 trait shape. The four open items O1 through O4 map to this topic's S1, S2, S4, and S5.
- `mock/design_rounds/202605042200_topic_markers_as_registrables_final.md`. Topic 2, REJECTED. The round-3 sketch precedent for the format used here.
- `mock/research/sketches/registrable-not-in-202605051200/FINDINGS.md`. Round-3 sketch evidence and the format precedent for FINDINGS.md.
- `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md`. Sketches commit before doc CL locks. Defines the FINDINGS.md format.
- `~/Dev/clause-dev/.claude/rules/arvo-compile-time-last.md`. The compile-time-paid-once principle. Licenses heavy trait-solver work in S1 and S2 if it buys runtime or correctness wins.
- `~/Dev/clause-dev/.claude/rules/no-legacy-shims-pre-1.0.md`. Pre-1.0 churn is acceptable, so sketch outcomes that force a pivot are normal.
- Tasks: #330 (umbrella), #333 (load-bearing for S2), #361 (Topic 3, done), #362 (this topic), #363 (run sketches), #364 (doc CL), #365 (SRC CL plus close).
