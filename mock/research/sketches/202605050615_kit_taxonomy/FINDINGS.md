# Findings: S5, Kit Owned-state taxonomy completeness

**Date:** 2026-05-05.
**Round:** 202605042200.
**Maps to:** Topic 4 sketch S5, open item O4. Audit C1 remediation.
**Outcome:** WORKS for the success path. WORKS for the Replaceable-bound negative test. Visibility axis: substantive revision; visibility cannot be a per-Owned-type substrate axis. See "Visibility finding" below.

This file supersedes the original paper-only S5 conclusions. Audit C1 required a real `sketch.rs` to ground the claims; the sketch revealed a finding the paper analysis had missed.

## What the sketch builds

`sketch.rs` defines the three diverse kit shapes named in topic 4 (MockspaceKit, BenchTracingKit, LintPackKit) under the kit-trait surface from Topic 3 (`Kit { type Units: WorkUnitBundle; type Owned: StoreBundle; }`) plus the Replaceable opt-in marker from S4 plus the typestate-builder substrate from S1 (recursive HList AccessSet, `#[marker]` Contains, ContainsAll proof at `.build()`).

Each kit has 2 to 3 Owned types. A subset of each kit's Owned types opt in to `Replaceable` so that `Scheduler::replace_resource::<T>(...)` works on them. One `WorkUnit` (`bench_tracing_kit::TracerFiber`) reads `lint_pack_kit::Diagnostic`, exercising the cooperative-public path: a sibling kit's pub Owned type appears in another kit's WU `Read` set.

The sketch culminates in `demo_success()` which builds a Scheduler with all three kits + two app-level resources (StringInterner, Clock), then calls `.replace_resource(...)` on each Replaceable type.

## Compile outcomes

```
$ rustup run nightly rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata
EXIT: 0
```

Success path compiles clean.

```
$ rustup run nightly rustc --crate-type=lib --edition=2024 \
    sketch.rs --emit=metadata --cfg 'feature="show_replace_bound_error"'
error[E0277]: the trait bound `AppPublicValueButNotReplaceable: Replaceable`
              is not satisfied
   --> sketch.rs:328:24
   ...
help: the trait `Replaceable` is not implemented for
      `AppPublicValueButNotReplaceable`
help: the following other types implement trait `Replaceable`:
        LintConfig, Diagnostic, Tracer
note: required by a bound in `Scheduler::<Wus, Stores>::replace_resource`
EXIT: 1
```

Negative test fails compile with E0277 and a high-quality diagnostic that names the offending type, lists the three types that DO implement `Replaceable`, and points at the bound in `replace_resource`. This error message is good enough to ship without a custom `#[diagnostic::on_unimplemented]` annotation, though one could land later for further polish.

## Visibility finding (substantive)

The sketch was authored expecting two compile attempts: success first, visibility-error negative second. It took three.

**First attempt.** Kit-internal Owned types declared as bare `struct` (module-private). 13 errors of form `error[E0446]: private type X in public interface`, fired by the `pub Kit` and `pub WorkUnit` impls whose associated types referenced the private types.

**Second attempt.** Same types declared as `pub(crate) struct`. 9 errors, same shape, this time `error[E0446]: crate-private type X in public interface`. The privacy rule does not relax for crate-private; the impl's pub-ness leaks the type at the same boundary.

**Third attempt (current sketch).** Same types declared as `pub struct`. 0 errors, success.

The implication: **visibility cannot be a per-Owned-type substrate axis under the current `pub Kit { type Owned: StoreBundle; }` shape.** Every Owned type must be at least as visible as the Kit impl. Topic 3's "pub(crate) wrapping of locked-down kit-internal state" does not compose with `pub Kit` impls.

The two workable kit-shape patterns are:

A. Kit declared `pub`, every Owned type `pub`. Visibility provides nothing the substrate enforces; only `Replaceable` distinguishes overridable from non-overridable. Convention (the kit author keeps non-Replaceable types out of the kit's public re-export surface) is the only signal kit-internal-vs-public.

B. Kit declared `pub(crate)`, every Owned type `pub(crate)`, kit lives in its own crate. Consumers register the kit via a pub helper-fn that consumes a builder and returns one without naming the kit. The kit-internal types are then genuinely kit-private at the crate boundary. The substrate cannot mix levels in a single Kit impl.

The sketch exercises pattern A. Pattern B's structural shape is documented for the doc CL but not built here (would require a multi-crate sketch).

## What this means for the round-4 plan

Topic 3 said the substrate would commit to two scoping axes: visibility and replaceability. The sketch shows the substrate gets one and a half:

- **Replaceable** is a real substrate axis. The opt-in marker plus the static `T: Replaceable` bound on `replace_resource` does the work, with a clean compile error when consumers try to override a non-Replaceable type.
- **Visibility** is *not* a substrate axis. It is a kit-shape convention applied at the level of "the whole kit is pub" or "the whole kit is pub(crate) and shipped from its own crate". The substrate cannot enforce per-Owned-type kit-internalness because Rust's E0446 forbids it under the chosen Kit trait shape.

The doc CL must reflect this. Two adjustments:

1. **Drop "two-axis annotation surface" framing.** The annotation surface has one axis (Replaceable) plus a structural choice (kit visibility = whole-kit-pub vs whole-kit-pub(crate)-with-helper-fn).
2. **Rename "visibility axis" to "kit visibility envelope".** The envelope is set by the kit author when shipping the kit crate. The substrate observes; it does not enforce per-type.

This does not invalidate any of the sound points the audit confirmed (G1-G7). The Kit trait shape (`Units` + `Owned`, no `Required`) stands. The Replaceable opt-in stands. Pre-1.0 churn licenses BuilderResource deletion. The compile-time-last framing is correctly applied.

This DOES tighten audit C2's framing: not only does the substrate not enforce kit-private state via the typestate, the substrate cannot enforce it via Rust visibility either (under this Kit trait shape). The honesty the doc CL must capture is even stronger than C2's original statement: visibility doesn't help at all at the per-type level; only at the per-kit-crate level.

## Speculative axes resolved

Topic 4's three speculative axes from topic 3, kept here for completeness:

| Speculative axis | Topic-3 hypothesis | S5 finding |
|---|---|---|
| Lifetime scope | needed when Owned state outlives scheduler | scheduler-bound is the only lifetime in v1; longer-lived state is constructed by the app and passed in via `.resource::<T>(initial)`. Not a substrate axis. |
| Build-order | needed when one kit's Owned must initialise before another's | app-level ordering of `.add_kit()` calls handles this; not a per-Owned-type axis. |
| Init source | needed when state comes from a non-default constructor | app-level `.resource::<T>(T::new(...))` handles this; not a per-Owned-type axis. |

Each speculative axis dissolves to a structural property of the substrate or an app-level concern outside the per-Owned-type taxonomy. No speculative axis lands in round-4.

## Recommendation (revised post-empirical)

Adopt **Replaceable as the sole substrate-enforced annotation** for round-4. The "kit visibility envelope" is documented in the doc CL as a per-kit shape choice, not a per-Owned-type axis.

If a future kit shape surfaces a genuine fourth concern (the substrate cannot anticipate every consumer), pre-1.0 churn licenses adding it then. Round-4 ships the minimum that holds.

## Cross-references

- `mock/research/sketches/202605050555_replaceable_polarity/`. S4: where the Replaceable opt-in polarity was decided.
- `mock/research/sketches/202605050530_deep_stacking/`. S1: typestate-builder substrate this sketch reuses.
- `mock/design_rounds/202605042200_topic_kit_trait_split.md`. Topic 3: the locked Kit shape.
- `mock/design_rounds/202605042200_topic_round_4_audit.md`. Audit topic: C1 remediation closed by this file.

## Notes on the now-stale `sketch_skeletons.md`

`sketch_skeletons.md` (committed alongside the original paper-only FINDINGS) describes the three kit shapes as conceptual skeletons and points at S4 as the empirical surface. It is preserved as audit trail. The `sketch.rs` in this directory is now the load-bearing artefact; the skeletons file is supplementary prose.
