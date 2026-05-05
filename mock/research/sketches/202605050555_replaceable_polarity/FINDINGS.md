# Findings: S4, Replaceable polarity

**Date:** 2026-05-05.
**Round:** 202605042200.
**Maps to:** Topic 4 sketch S4, open item O3.
**Outcome:** WORKS (both polarities). Recommend opt-in. The author cost is identical in the worked example; opt-in avoids two nightly features (`auto_traits` + `negative_impls`), defaults safer (new types locked down by default), and matches the substrate's "tools not policy" stance.

## Setup

Topic 3 named `Replaceable` as a marker trait on Owned-state types, gating the app-only `.replace_resource::<T>(...)` API. Opt-in means the author writes `impl Replaceable for Foo {}` to enable replacement; opt-out means a default `auto trait Replaceable {}` with `impl !Replaceable for Foo {}` to disable. Topic 4 named the polarity decision as unsettled, with arguments on both sides and feature-gate questions hanging over opt-out (`feature(negative_impls)` had a coherence-shaped failure in round-3, in a different mechanism but adjacent enough to flag).

## What got built

Two parallel sketches with the same six-type test surface (`MockspaceKit` with RoundState + DesignRound + LintConfig, `BenchTracingKit` with Tracer + TraceSample, `LintPackKit` with Diagnostic + Statistics):

`a_opt_in/sketch.rs`. Plain `pub trait Replaceable {}`. Author opts in three types (LintConfig, Tracer, Diagnostic). Three types remain non-replaceable (RoundState, DesignRound, TraceSample, Statistics). Author writes 3 `impl Replaceable for X {}` lines.

`b_opt_out/sketch.rs`. `pub auto trait Replaceable {}` with `feature(auto_traits)` + `feature(negative_impls)`. Author opts out three types (RoundState, DesignRound, TraceSample, Statistics, count actually 4 from the same surface since opt-out flips the polarity). Author writes 4 `impl !Replaceable for X {}` lines.

## Compile results

Both sketches compile cleanly in the success path (replace replaceable types, lock unreplaceable types). Both produce reasonable errors for the missing-impl case.

Opt-in error:

```
error[E0277]: the trait bound `RoundState: Replaceable` is not satisfied
   |
22 | impl Replaceable for LintConfig {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `LintConfig`
28 | impl Replaceable for Tracer {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Tracer`
34 | impl Replaceable for Diagnostic {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Diagnostic`
note: required by a bound in `replace_resource`
```

Opt-out error:

```
error[E0277]: the trait bound `RoundState: Replaceable` is not satisfied
help: the trait `Replaceable` is not implemented for `RoundState`
note: required by a bound in `replace_resource`
```

Opt-out's message is cleaner (no list of positive impls because the auto trait covers everything implicitly). Opt-in's message lists the explicitly-replaceable types so the author can see at a glance what's available.

## Polarity comparison

| Axis | Opt-in (A) | Opt-out (B) |
|---|---|---|
| Stable Rust feature use | yes | no, requires `auto_traits` + `negative_impls` |
| Author lines (worked example) | 3 `impl Replaceable for X {}` | 4 `impl !Replaceable for X {}` |
| Default for new types | non-replaceable | replaceable |
| Coherence reliability | clean (single positive impl per type) | clean for single-type marker; round-3's NotIn issue was for type-list relations, not marker traits |
| Substrate principle alignment | "tools not policy" with default-deny matches | "ergonomic default" matches Rust philosophy |
| New-type cost | author must remember to opt in if they want replacement | author must remember to opt out if they want lock-down |

Both polarities work. The author-line cost is essentially equal in this example.

## Recommendation

Adopt **opt-in (A)** for round-4. Reasons:

1. **No nightly dependency for the polarity itself.** Replaceable as a plain trait is stable Rust. Opt-out adds `feature(auto_traits)` and `feature(negative_impls)` to the substrate's already-large nightly feature set. The substrate accepts nightly per `arvo-compile-time-last.md` when there's a runtime or correctness reason; opt-out's feature dependency would not buy a runtime or correctness gain over opt-in.
2. **Safer-by-default.** New Owned types default to non-replaceable, so an author who adds a new private store and forgets about replaceability does not accidentally expose it. Opt-out fails open: forgetting to `impl !Replaceable` makes the new type app-overridable.
3. **Matches "tools not policy" precisely.** The substrate's stance is that capabilities exist; consumers choose. Default-deny means the consumer must explicitly choose to expose; default-allow means the consumer must explicitly choose to lock down. Default-deny is the more conservative read of "tools not policy" because it does not pre-decide that things should be open.
4. **Round-3 negative-impls anxiety.** `feature(negative_impls)` worked here for single-type markers but had a coherence wall in round-3 for type-list relations. Adopting it for opt-out brings the substrate closer to that wall for any future use. Opt-in avoids it entirely.
5. **List of replaceable types is visible in the error.** The compile error in opt-in lists the types that are currently replaceable, which is mildly diagnostic. Opt-out cannot do this.

Argument against opt-in (the natural counter): Rust's general philosophy is "things are open unless stated otherwise." Replaceable could be read as following that pattern. The substrate stance overrides this in favour of explicit opt-in for capabilities.

## Path forward

S4 settled, recommendation A (opt-in). Doc CL specifies `pub trait Replaceable {}` (plain stable trait) on the public substrate API. Authors annotate their kit's Owned types with `impl Replaceable for Foo {}` per type they want app-overridable. The `.replace_resource::<T>` API carries `T: Replaceable` as a static bound; non-replaceable replacement attempts fail at compile time at the call site.

## Cross-references

- `mock/research/sketches/registrable-not-in-202605051200/FINDINGS.md`. Round-3's `feature(negative_impls)` coherence finding for type-list relations. Different shape than this sketch but informs the choice to avoid the feature where alternatives exist.
- `mock/design_rounds/202605042200_topic_kit_trait_split.md`. Topic 3 named the two polarities and flagged the decision as unsettled.
