# Findings: S5, Kit Owned-state taxonomy completeness

**Date:** 2026-05-05.
**Round:** 202605042200.
**Maps to:** Topic 4 sketch S5, open item O4.
**Outcome:** WORKS. The two committed axes (visibility + replaceability) cover the three diverse kit shapes named in topic 4. Each speculative axis (lifetime scope, build-order, init-source) resolves to either a non-axis (consequence of scheduler-bound lifetime by construction) or an app-level concern outside the substrate's per-Owned-type taxonomy.

## Setup

Topic 4's S5 named three diverse kit shapes and asked: do visibility + replaceability cover the realistic surface, or does at least one speculative axis (lifetime scope, build-order constraint, initialisation source) need to be committed for round-4?

Speculative axes from topic 3:

- **Lifetime scope.** Does the kit's Owned state outlive the scheduler's lifetime, or is it scheduler-bound?
- **Build-order constraints.** Does the kit's Owned state need to be initialised before/after another kit's?
- **Initialisation source.** Does the Owned state come from a default, a config-derived value, or a user-supplied builder?

## Three test shapes

### MockspaceKit (multi-marker owned, multi-shared required)

```rust
pub struct MockspaceKit;
impl Kit for MockspaceKit {
    type Units = (RoundProcessor, RoundLockChecker, ConfigLoader, ...);
    type Owned = (
        Resource<RoundState>,         // pub(crate), not Replaceable
        Column<DesignRound>,           // pub(crate), not Replaceable
        Resource<LintConfig>,          // pub for testability, Replaceable
    );
}
// Required (derived from Units): StringInterner, Clock.
```

Mapping to the two axes:

- Visibility: `RoundState`, `DesignRound` are `pub(crate)` (kit-internal). `LintConfig` is `pub` for testability.
- Replaceability: `RoundState`, `DesignRound` not `Replaceable`. `LintConfig` is `Replaceable` so test harnesses can swap in fixture configs.

Axes required: 2. No speculative axis needed.

### BenchTracingKit (FFI handle pre-scheduler init)

```rust
pub struct BenchTracingKit;
impl Kit for BenchTracingKit {
    type Units = (TracerFiber, SampleCollector);
    type Owned = (
        Resource<Tracer>,              // pub for cooperative use, Replaceable
        Column<TraceSample>,           // pub(crate), not Replaceable
    );
}
// Required: nothing.
```

The topic-4 hypothesis: `Tracer` setup requires an FFI handle initialised before the scheduler starts. Does this require a "lifetime scope" axis (Tracer outlives the scheduler)?

The mapping below shows it does not:

- Visibility: `Tracer` is `pub` for cooperative use (other kits may use a `Tracer` reference passed via context). `TraceSample` is `pub(crate)`.
- Replaceability: `Tracer` is `Replaceable` so the app can swap the FFI-bound implementation for a no-op or alternative. `TraceSample` not Replaceable.
- The FFI handle is set up at construction time, and `Tracer`'s lifetime is identical to the scheduler's by construction (the scheduler owns the resource). The "lifetime scope" speculative axis was a false flag: there is no v1 case where the Owned state needs to outlive the scheduler. If a future kit shape genuinely needs that, it is an app-level concern (the consumer constructs the long-lived state in main and passes a reference into the resource), not a per-Owned-type axis.
- The construction concern (how `Tracer::new(handle)` is wired) is the "init source" speculative axis; resolved as: app calls `.resource::<Tracer>(Tracer::new(handle))` at the app level. Not a substrate axis on the Owned declaration.

Axes required: 2. Lifetime-scope and init-source resolve to non-axes.

### LintPackKit (cooperative-public column shared with other kits)

```rust
pub struct LintPackKit;
impl Kit for LintPackKit {
    type Units = (LintEmitter, DiagnosticAggregator);
    type Owned = (
        Column<Diagnostic>,            // fully pub, Replaceable for test harnesses
        Resource<Statistics>,          // pub(crate), not Replaceable
    );
}
// Required: StringInterner.
```

The cooperative-public case: other kits' WUs may write to `Column<Diagnostic>` via a public path. This is the "fully public" visibility option from topic 3.

- Visibility: `Diagnostic` is fully `pub` (cooperative). `Statistics` is `pub(crate)`.
- Replaceability: `Diagnostic` is `Replaceable` so test harnesses can swap with a structured-output collector. `Statistics` not Replaceable.

Axes required: 2.

## Speculative axes resolved

| Speculative axis | Topic-3 hypothesis | S5 finding |
|---|---|---|
| Lifetime scope | needed when Owned state outlives scheduler | scheduler-bound is the only lifetime in v1; longer-lived state is constructed by the app and passed in |
| Build-order | needed when one kit's Owned must initialise before another's | app-level ordering of `.add_kit()` calls handles this; not a per-Owned-type axis |
| Init source | needed when state comes from a non-default constructor | app-level `.resource::<T>(T::new(...))` handles this; not a per-Owned-type axis |

Each speculative axis dissolves to either a structural property of the substrate (scheduler-bound lifetime) or an app-level concern outside the Owned-state taxonomy.

## Recommendation

Adopt the two committed axes for round-4 (visibility, replaceability). No speculative axis lands. Doc CL describes both axes per topic 3 with the mappings above as concrete kit-shape examples.

If a future kit shape surfaces a genuine fourth axis (the substrate cannot anticipate every consumer), the round-5 or later round adds it. Pre-1.0 churn is acceptable per `no-legacy-shims-pre-1.0.md`.

## Notes for round-4 doc CL

The MockspaceKit / BenchTracingKit / LintPackKit examples are illustrative only. The substrate doc CL should describe the two axes generically; consumer crates carry their own examples in their respective DESIGN.md.tmpl files.

The BenchTracingKit case (FFI handle init) deserves a doc-CL note explaining the pattern: when a substrate user has FFI or pre-scheduler-init concerns, the construction happens at the app level via `.resource::<T>(T::new(handle))`, not via a kit's `Owned` declaration. The kit declares `Resource<Tracer>` as Owned; the app provides the constructed instance.

## Cross-references

- `mock/research/sketches/202605050555_replaceable_polarity/`. S4's sketches use these same three kit shapes for the polarity test; the kits act as the test surface for both S4 and S5.
- `mock/design_rounds/202605042200_topic_kit_trait_split.md`. Topic 3 named the two committed axes and the three speculative ones.
