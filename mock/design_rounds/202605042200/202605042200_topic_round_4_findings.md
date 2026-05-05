**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** Synthesise the five-sketch evidence into a single ground-truth document. Each open item from topic 4 (S1-S5) maps to a passed empirical outcome and a concrete recommendation. The doc CL writes against this synthesis, not against topics 3 and 4 plus the FINDINGS files individually.
**Source topics:** Topic 1 (`202605042200_topic_builder_register_unification.md`, round 1+2 journey), Topic 2 (`202605042200_topic_markers_as_registrables_final.md`, round-3 REJECTED), Topic 3 (`202605042200_topic_kit_trait_split.md`, locked Kit shape), Topic 4 (`202605042200_topic_round_4_sketches.md`, sketch hypotheses S1-S5).

# Topic: round-4 sketch findings synthesis

## Why this topic exists

Topics 3 and 4 fixed the trait shape and named five empirical questions that had to settle before the doc CL could lock. The sketches ran; each produced a binary outcome and a concrete recommendation. The findings are spread across five `FINDINGS.md` files under `mock/research/sketches/`. This topic consolidates them into one place so the doc CL author can read a single document and know what the trait surface, the access-set encoding, the polarity choice, and the scoping axes look like as a coherent whole.

The sketches did not produce a pivot. Topic 3's locked Kit shape stands. Topic 4's recommendations all settled on the side that keeps the surface clean. The synthesis below is a green outcome: ready to write the doc CL.

## Sketch outcomes table

| Sketch | Open item | Outcome | Recommendation | Findings file |
|---|---|---|---|---|
| S1 | O1, deep stacking | WORKS | Typestate builder sustains at depth 4 with B-shape AccessSet | `mock/research/sketches/202605050530_deep_stacking/FINDINGS.md` |
| S2 | O2, arity solve (#333) | WORKS | B (recursive HList with `#[marker]`) | `mock/research/sketches/202605050503_accessset_arity/FINDINGS.md` |
| S3 | recursion limit | subsumed by S1 | Default `#![recursion_limit]` of 128 suffices at depth 4 | `mock/research/sketches/202605050530_deep_stacking/FINDINGS.md` |
| S4 | O3, Replaceable polarity | WORKS | Opt-in (`pub trait Replaceable {}` plus `impl Replaceable for X {}` per opted-in type) | `mock/research/sketches/202605050555_replaceable_polarity/FINDINGS.md` |
| S5 | O4, scoping axes completeness | WORKS | Two committed axes (visibility, replaceability) suffice for v1 | `mock/research/sketches/202605050615_kit_taxonomy/FINDINGS.md` |

## The complete round-4 surface, as one picture

Reading topics 3 and 4 in isolation leaves the reader assembling pieces. Here is the whole surface as a single description.

### Kit trait

```rust
pub trait Kit {
    type Units: WorkUnitBundle;
    type Owned: StoreBundle;
    // No `Required`. Derived mechanically from `Units::AccessSet \ Owned`.
}
```

`WorkUnitBundle` is a typed cons-list of WorkUnits with associated types `AccumRead`, `AccumWrite` that recursively concatenate each WU's Read and Write sets through the bundle. `StoreBundle` is a typed cons-list whose elements are any mix of `Resource<T>`, `Column<T>`, `Virtual<T>` markers. Both share the substrate's existing `Empty` and `Cons<H, T>` HList primitives.

### AccessSet shape

Recursive HList with `#[marker]` Contains:

```rust
#[marker]
#[diagnostic::on_unimplemented(/* ... */)]
pub trait Contains<X>: AccessSet {}

impl AccessSet for Empty {}
impl<H, T: AccessSet> AccessSet for Cons<H, T> {}

impl<X, T: AccessSet> Contains<X> for Cons<X, T> {}
impl<X, H, T: AccessSet> Contains<X> for Cons<H, T> where T: Contains<X> {}

#[marker]
pub trait ContainsAll<L>: AccessSet {}
impl<S: AccessSet> ContainsAll<Empty> for S {}
impl<S: AccessSet, H, T> ContainsAll<Cons<H, T>> for S where S: Contains<H> + ContainsAll<T> {}
```

The `#[marker]` attribute (gated by `feature(marker_trait_attr)`, already in use by v0.1) bypasses coherence overlap when `H = X`. The substrate's existing nightly footprint accepts this feature.

### Typestate builder

`SchedulerBuilder<Wus, Stores>` accumulates per `.add_kit::<K>()`:

```rust
impl<Wus, Stores> SchedulerBuilder<Wus, Stores> {
    pub fn add_kit<K: Kit>(self) -> SchedulerBuilder<
        <K::Units as Concat<Wus>>::Out,
        <K::Owned as Concat<Stores>>::Out,
    >
    where K::Units: Concat<Wus>, K::Owned: Concat<Stores> {
        // ZST construction
    }

    pub fn resource<T>(self) -> SchedulerBuilder<Wus, Cons<T, Stores>> { /* ... */ }
}

impl<Wus: WorkUnitBundle, Stores: AccessSet + StoreBundle> SchedulerBuilder<Wus, Stores>
where
    Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>,
{
    pub fn build(self) -> Scheduler<Wus, Stores> { /* ... */ }
}
```

The `.build()` static check is the proof that all WU accesses are satisfied by registered stores. Failure surfaces at the `.build()` call site with `#[diagnostic::on_unimplemented]` naming the missing marker.

### Replaceable surface (app-only)

```rust
pub trait Replaceable {}
// Authors opt in:
// impl Replaceable for LintConfig {}

impl<Wus, Stores> SchedulerBuilder<Wus, Stores> {
    pub fn replace_resource<T: Replaceable>(self, new: T) -> Self
    where Stores: Contains<T> {
        // construct
    }
}
```

Kits never call `.replace_resource`. App authors call it after the relevant kit's `.add_kit` has registered the resource. `T: Replaceable` is a static bound; non-replaceable types fail at compile time with an unsatisfied-trait-bound error pointing at the type.

### Owned access scoping (per-Owned-type, applied by kit author)

Two axes:

- **Visibility**, via standard Rust visibility: fully `pub`, `pub` type with sealed-supertrait operations, or `pub(crate)`.
- **Replaceability**, via the `Replaceable` opt-in marker.

Speculative axes (lifetime scope, build-order, init-source) are not committed; S5 found them resolving to either non-axes (scheduler-bound is the only v1 lifetime) or app-level concerns (init-source is `.resource::<T>(T::new(...))` at the app level, not a kit declaration).

### Diamond resolution

Topic 3's reframe stands: kits do not register shared instances. `InternerKit` declares `StringInterner` as a Required dependency (mechanically derived from its WUs' AccessSets); the app registers the instance via `.resource::<StringInterner>(StringInterner::new(...))` directly. Two kits requiring the same `StringInterner` see the same registered instance because the typestate's `Stores` cons-list deduplicates structurally (a `Contains<StringInterner>` proof matches the first occurrence).

The only "shared instance" register path is the app's `.resource::<T>(...)` direct call. Kits cannot reach into other kits' Owned via Visibility axis 1 (`pub(crate)` or sealed-supertrait gate the cross-kit access path).

### Nested kits

A composite kit (`MockspaceKit` containing `LintKit` and `BenchKit`) registers its constituents recursively at `.add_kit` time. The typestate accumulator handles propagation mechanically; no type-level union at the parent level is needed.

```rust
impl Kit for MockspaceKit {
    type Units = (MockspaceWUs, LintKit::Units, BenchKit::Units, /* concat */);
    type Owned = (MockspaceOwned, LintKit::Owned, BenchKit::Owned, /* concat */);
}

// Or, equivalently and preferred:
// MockspaceKit::register(b) -> b.add_kit(LintKit).add_kit(BenchKit).add_kit(MockspaceCore)
```

The latter pattern (recursive registration) is preferred per topic 3 because it mirrors how WUs are added today and keeps rustc's trait-solver work proportional to the actual `.add_kit` call sequence, not to a parent-level type-level union.

## Cross-cutting concerns

### Compile-time cost

S1's depth 4, 9 WU, 6 kit example compiles in 0.55s on metadata-only. The bundle accumulator's recursive Concat traverses the AccessSet lists on every `.add_kit` call; the `.build()` proof walks `Stores: ContainsAll<L>` over the accumulated lists.

The asymptotic cost is `O(N²)` in the accumulated list length where N is the total stores plus the total accessed markers (per-WU summed). At depth 4 with 12 stores and 9 WUs each carrying 1-2 markers, N is roughly 30; the proof is roughly 900 trait-solver steps. At depth 5 with N=50, roughly 2500. At depth 6 / N=100, 10000. Beyond that the curve depends on rustc's trait cache effectiveness.

The substrate accepts this cost per `arvo-compile-time-last.md`: compile time is paid once, runtime is paid forever. The static check at `.build()` catches a class of bugs that runtime checks would let propagate, and that trade is the rule's licensed direction.

### Bundle accumulator carries duplicates

S1 noted that the WorkUnitBundle's recursive Concat does not deduplicate. If two WUs both read `Clock`, the accumulated read set contains `Clock` twice. The proof works (both `Contains<Clock>` calls resolve to the same impl), but each duplicate doubles the trait-solver work for that proof.

This is a cosmetic concern at depth 4. At deeper nesting it compounds. A type-level set-dedup operation (`Concat` plus a `RemoveDup` step) is a tractable round-5 follow-up. It is not a round-4 blocker.

### Error-message hygiene

The `#[diagnostic::on_unimplemented]` annotation on `Contains<X>` keeps the missing-resource error friendly when the trait-solver chain works. Without the annotation, the failure surfaces as a verbose recursion-walk (S1 captured this). With the annotation, the user sees a single clear sentence.

The doc CL must annotate `Contains`, `ContainsAll`, `Replaceable` (with text suggesting the `impl Replaceable for X {}` line), and the `Kit` trait itself (with text suggesting kits be added via `.add_kit::<K>()`).

The "long type written to file" rustc behavior at depth 4-5 is a known cost of deep typestate. Workable. Not a blocker.

### Pre-1.0 churn

Per `no-legacy-shims-pre-1.0.md`, the v0.1 `BuilderResource<T>` shape and its `InternerKit + BuilderResource<T>` pattern get deleted, not deprecated. The doc CL writes the new shape; the SRC CL deletes the old one and updates every call site. No transition period.

## Open follow-ups for round-5 or beyond

Not blocking. Capture for later attention.

1. Bundle accumulator dedup: type-level `RemoveDup` on Concat output, eliminates the trait-solver work for repeated marker accesses.
2. `set!` macro: a friendly `set![M0, M1, ..., MN]` that emits the `Cons<M0, Cons<M1, ..., Empty>>` type alias at the call site, so authors do not write the cons-chain by hand.
3. `kit!` macro: a similar friendly authoring surface that reduces `impl Kit for Foo { type Units = ...; type Owned = ...; }` boilerplate.
4. Empirical exploration of arity 24, 48, 64 with the chosen B-shape if a future kit shape demands it. Not a v1 concern.
5. AccessSet bitmask alternative as the topic-4 pivot if depth 5+ kit nesting reveals trait-solver pathology. Not a v1 concern but worth keeping in BACKLOG.
6. Lifetime-scope axis if a future kit shape genuinely needs Owned-state outliving the scheduler. Currently no such case; the substrate stays simple.

## Path to doc CL

1. **This topic file** commits to `feat/builder-register-unification` as the synthesis. Topics 1 through 5 plus all five FINDINGS files are the input; this topic is the one-page summary the doc CL writes against.
2. **Doc CL**: `mock/design_rounds/202605042200_changelist.doc.md`. Per-crate, per-file, mechanical:
   - `hilavitkutin-api`: `Kit` trait, `WorkUnitBundle`, `StoreBundle`, `AccessSet`, `Contains`, `ContainsAll`, `Concat`, `Replaceable`, `Empty`, `Cons<H, T>`. All annotated with `#[diagnostic::on_unimplemented]`. The existing v0.1 `Buildable` and `Contains` shapes get rewritten or removed; v0.1's flat-tuple `Contains` impls go.
   - `hilavitkutin`: `SchedulerBuilder<Wus, Stores>` typestate, `.add_kit::<K>()`, `.resource::<T>()`, `.replace_resource::<T>()` with the `T: Replaceable` bound, `.build()` with `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>` bound.
   - `hilavitkutin-providers`: convert `InternerKit` (and any other v0.1 kits) from `BuilderResource<T>` shape to `{Units, Owned}` shape. Remove `BuilderResource<T>`.
   - `hilavitkutin-extensions`: only if any contract surface is affected. Audit during doc-CL writeup.
3. **Lock doc CL** via `cargo mock lock`.
4. **SRC CL** with structured `## CHANGE:` blocks per `cl-claim-sketch-discipline.md`.
5. **Execute, validate, lock SRC CL, close round.**

## Cross-references

- `mock/design_rounds/202605042200_topic_builder_register_unification.md`. Topic 1, round 1+2 journey.
- `mock/design_rounds/202605042200_topic_markers_as_registrables_final.md`. Topic 2, round-3 REJECTED.
- `mock/design_rounds/202605042200_topic_kit_trait_split.md`. Topic 3, locked Kit shape.
- `mock/design_rounds/202605042200_topic_round_4_sketches.md`. Topic 4, sketch hypotheses.
- `mock/research/sketches/202605050503_accessset_arity/FINDINGS.md`. S2 findings.
- `mock/research/sketches/202605050530_deep_stacking/FINDINGS.md`. S1 findings (subsumes S3).
- `mock/research/sketches/202605050555_replaceable_polarity/FINDINGS.md`. S4 findings.
- `mock/research/sketches/202605050615_kit_taxonomy/FINDINGS.md`. S5 findings.
- `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md`. Sketch discipline; `## CHANGE:` block format for SRC CL.
- `~/Dev/clause-dev/.claude/rules/arvo-compile-time-last.md`. Compile-time-paid-once principle licensing the static-check trade.
- `~/Dev/clause-dev/.claude/rules/no-legacy-shims-pre-1.0.md`. Permitting clean replacement of v0.1 shapes.
- Tasks: #330 (umbrella), #333 (load-bearing for #330), #361-#363 done, #364 (next, doc CL), #365 (SRC CL plus close).
