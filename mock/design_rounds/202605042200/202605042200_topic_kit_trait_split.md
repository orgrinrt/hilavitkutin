**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** Kit trait redesign: split contributions into `Units: WorkUnitBundle` + `Owned: StoreBundle`. Drop `Required` (derived from Units' AccessSets minus Owned). Compile-time check via typestate builder. Cascading dissolution of #330's original six decision items.
**Source topics:** Topic 1 (`202605042200_topic_builder_register_unification.md`) framed `BuilderResource -> BuilderRegister<M: StoreMarker>`; explored audit reports + round-1+2 journey. Topic 2 (`202605042200_topic_markers_as_registrables_final.md`) proposed marker-as-registrable with compile-time diamond detection via `feature(negative_impls)`; that proposal was REJECTED after the empirical sketch (`mock/research/sketches/registrable-not-in-202605051200/`) showed `NotIn<H>` cannot be soundly encoded under coherence. This topic supersedes both: the diamond problem, the Output transitivity gap, the silent-reorder hazard, and the persistence-spine churn all dissolve when kits stop registering shared instances and the trait is restructured to make the kit-private vs shared distinction explicit.

# Topic: Kit trait split — `{ Units, Owned: StoreBundle }`, `Required` derived

## Why the previous frames failed

Topics 1 and 2 both took the original `BuilderResource<T>` shape as a fixed starting point and asked "how do we extend it". Topic 1 unified it across markers (`Resource` / `Column` / `Virtual`) into `BuilderRegister<M: StoreMarker>`. Topic 2 then asked how to make duplicate registrations of the same `M` a compile error via marker-as-registrable. The compile-time encoding turned out to be empirically infeasible (the sketch in `mock/research/sketches/registrable-not-in-202605051200/` is the audit-trail evidence).

The deeper issue: both frames assumed kits ship concrete *instances* of shared infrastructure. That assumption is the source of the diamond problem. An interner is not a piece of `InternerKit`; an interner is a piece of *the app*. `InternerKit` only happens to need one. The audit-trail rounds confirmed by exhaustion that no policy on top of "kits register shared instances" produces a clean substrate. We change the policy.

## The reframe

Kits contribute three categories of thing, and the categories have *different diamond profiles*:

1. **WorkUnits** — always distinct (each WU is its own type). No diamond by construction.

2. **Owned state** — resources, columns, and virtuals the kit *internally uses*. `BenchKit` has a private `BenchAccumulator` that only its own WorkUnits touch. `LintKit` has a private `Column<Diagnostic>` (where `Diagnostic` is a kit-internal type) that only its lint WUs write to and its emit WU drains. Two kits both shipping their own internal `Tracer` is two distinct `Tracer` types from distinct modules — different positions in the store table, no conflict.

3. **Required state** — resources/columns/virtuals the kit *needs but does not own*. `BenchKit` needs a `Clock`. `LintKit` needs a `StringInterner`. Multiple kits might need the same `StringInterner`. **This is the only category with diamond risk** — and it dissolves when kits don't ship instances of it. The kit *declares* the dependency; the *app* registers the instance.

The trait split makes this typology a load-bearing feature of the public API:

```rust
trait Kit {
    type Units: WorkUnitBundle;
    type Owned: StoreBundle;
    // No `Required`. Derived from Units::AccessSet \ Owned.
}
```

That is the locked shape. Everything below is the rationale, the open items, and the path forward.

## Why `Owned: StoreBundle`, not `ResourceBundle`

The original Topic 1 shape spoke of resources only. The substrate has three store markers (`Resource<T>`, `Column<T>`, `Virtual<T>`); a kit can plausibly own private state in any of them. A bench kit might have private `Column<BenchSample>` that no other kit reads. A debug kit might own `Virtual<DebugTick>` that signals only into its own WUs.

`StoreBundle` is the union: a typed cons-list whose elements are any mix of `Resource<T>`, `Column<T>`, `Virtual<T>` markers. The existing `Buildable<Stores>` proof system (`hilavitkutin-api/src/store.rs`) already operates on `Stores` cons-lists with mixed marker kinds — `StoreBundle` is the surface name for that, exposed at the kit boundary.

This unifies with the engine's existing internal model rather than fragmenting into per-marker bundle types.

## Why `Required` is dropped

The naive design would add `type Required: StoreBundle` next to `type Owned`. The author writes "I need `Resource<StringInterner>` and `Column<Diagnostic>`" and the build-time check verifies the app provided them.

This is redundant. WorkUnits already carry `type Read: AccessSet` and `type Write: AccessSet`. `Units::AccessSet` (the union of all member WUs' Read+Write) is exactly what the kit's Required set *should* be — and it cannot drift, because it is mechanically derived. Author edits a WU's access set; the kit's effective requirement updates automatically.

Letting the author also write `type Required = (...)` introduces two failure modes: (a) the declared list lags the WUs' actual accesses (false-positive ok); (b) the declared list claims something the WUs don't actually access (false-positive register-but-unused). Both are bug shapes.

The clean answer: drop `Required` from the trait. The build-time check uses `Units::AccessSet \ Owned` directly. For documentation purposes, a `Kit::requirements_doc()` helper or rustdoc-emit can derive the requirement list from the same source at compile time, keeping the doc and the check in sync by construction.

## Compile-time check via typestate builder

Per the workspace rule `arvo-compile-time-last.md` (rewritten 2026-05-05; see Cross-references), compile time is paid once and runtime is paid forever. The rule's title was previously misread; the corrected title — "Compile time is paid once; runtime is paid forever" — clarifies that we *spend* compile-time budget freely when doing so buys runtime or correctness. A static check that catches a missing resource at the `.build()` call site at compile time is exactly the trade the rule licenses.

Shape: `SchedulerBuilder<MAX_*, Wus, Stores>` already typestates `Wus` and `Stores`. `.add_kit::<K>(...)` returns a new typed builder where `Wus = (K::Units, OldWus)` and `Stores = (K::Owned, OldStores)` (or appropriately concatenated). `.build()` carries `where Self::AccumulatedAccesses: SatisfiedBy<Self::AllStores>` as a bound. Missing resource → compile error at the `.build()` line, naming the offending marker type via `#[diagnostic::on_unimplemented]`.

The runtime alternative — accumulate Vec'd accesses, panic at `.build()` time if missing — is rejected on this round per the corrected rule. (If the static check turns out empirically infeasible the way Topic 2's `NotIn<H>` did, that is a pivot point and a sketch result; we do not concede compile-time-statics on aesthetic grounds alone.)

## Owned access scoping — axes

A kit's `Owned` types may need access scoping. Two independent axes are identified now; more may surface as the surface evolves. The kit author turns these knobs per-Owned-type, not per-kit.

### Axis 1: Visibility / usability

How much of the type can other kits' code touch?

- **Fully public (`pub` type, `pub` methods).** Anyone can name and operate on it. Use when the kit *intends* to expose the type for cooperative use even though it owns the storage. Example: a `Tracer` type the kit owns but whose API is a public service.
- **Pub type, sealed-supertrait operations.** Type appears in signatures (so it can be named in `where Self::Owned: ...` bounds), but useful operations on it are gated by a sealed supertrait the kit's crate is the only impl-er of. Example: an internal handle that other kits' WUs need to *receive* but not *manipulate*.
- **`pub(crate)` type.** Type cannot be named outside the kit's crate. Strongest scoping. Example: a kit-internal accumulator that is genuinely an implementation detail.

These are existing Rust idioms — no novel mechanism needed.

### Axis 2: Replaceability

Can the app substitute a different instance via `.replace_resource::<T>(...)`?

- **Replaceable** (impls a marker trait, e.g. `pub trait Replaceable {}`). App may swap the instance.
- **Non-replaceable** (does not impl the marker). `.replace_resource::<T>` fails at compile time with `T: Replaceable` unsatisfied.

The polarity (opt-in vs opt-out) is **unsettled** and listed below as an open item. Both shapes have arguments; a sketch will clarify.

### Future axes (not committed, listed for completeness)

- **Lifetime scope** — does the kit's Owned state outlive the scheduler's lifetime, or is it scheduler-bound? Today everything is scheduler-bound; future kit shapes that want pre-scheduler init (e.g. for FFI handles) might want this axis.
- **Build-order constraints** — does the kit's Owned state need to be initialised before/after another kit's? Today there is no order; if future cases demand it, this becomes an axis.
- **Initialisation source** — does the Owned state come from a default, a config-derived value, or a user-supplied builder? This is an ergonomics axis, not a soundness axis; mostly app-level.

These are noted to make the documentation comprehensive when this lands. The two axes that are committed for round-4 are visibility and replaceability.

## Nested-kit propagation

Kits compose. `MockspaceKit` may internally pull `LintKit` and `BenchKit`. The trait's typestate-builder shape needs to accumulate Units and Owned across the recursion.

Two approaches:

1. **Recursive registration.** `MockspaceKit::register(b)` calls `b.add_kit(LintKit)` and `b.add_kit(BenchKit)` directly. The builder's typestate accumulates each call's contributions through standard `.add_kit()` chaining. No type-level union at the parent level — propagation is mechanical via the registration sequence.

2. **Type-level associated-type union.** `MockspaceKit::Units = (LintKit::Units, BenchKit::Units, OwnUnits)`. Author maintains the union; rustc proves at compile time that the parent's accesses satisfy through the union. More explicit, more rigid, more type-system burden.

Approach 1 is preferred (less rustc strain, mechanical, mirrors how WUs are added today). The compile-time AccessSet proof at `.build()` runs against the *accumulated* state at that point, which is the union by construction.

The hard question is whether approach 1 sustains under deep stacking. With many nested kits, the accumulated `Stores` cons-list grows linearly with depth, and AccessSet's arity (currently capped at 12 per #333) becomes the bottleneck. This is why **#333 must be solved as part of round-4**; deferring it leaves the design unstable. Sketch S2 (below) characterises behaviour and selects a solution.

## Patch / replace as a parallel app-only API

The "swap interner for a faster one" use case stays explicit and app-level. It does *not* go through a kit:

```rust
let scheduler = Scheduler::builder()
    .add_resource(StringInterner::with_capacity(64 * 1024))
    .add_kit(InternerKit)         // declares Required: StringInterner; satisfied by above
    .add_kit(LintKit)             // also requires StringInterner; same instance, no conflict
    .replace_resource::<StringInterner>(BetterInterner::new(...))  // app deliberately overrides
    .build()?;
```

`replace_resource::<T>` carries `T: Replaceable` as a static bound. Kits never call it. If a kit author wants to declare their Owned state non-replaceable, they simply don't impl `Replaceable` for it; the bound fails at compile time with a clear diagnostic.

There is no "kit replaces another kit's resource" path. By construction, kits can't reach into other kits' Owned (per axis-1 visibility) and don't carry shared instances (per the trait split). The patch concept is exclusively an app-author tool.

## Cascading dissolution of original six decisions

The original six items in #330 (per `project_resume_330_blocked_on_user_2026_05_05.md`) all dissolve under the new shape:

| # | Original item | Status |
|---|---|---|
| 1 | Diamond policy: shadow / panic / `.replace::<M>` / defer | Dissolved. No diamond — kits don't ship shared instances. |
| 2 | `Output: BuilderExtending<B>` placement | Dissolved. No `Required` to thread; per-call `BuilderExtending` becomes per-kit-add bound, transitive by typestate accumulation. |
| 3 | Silent-reorder defense | Dissolved. No registration ordering hazard (Owned is namespaced per kit; Required goes to the app's single registration site). |
| 4 | Recursion-limit propagation via macro | Reframed as "verify with sketch S3 or document one-line consumer-side `#![recursion_limit]`". Not a design decision; a verification task. |
| 5 | Strict-error vs ecosystem composability | Dissolved. No strict-error policy needed because there is no diamond. |
| 6 | Persistence-spine sibling-trait churn | Dissolved. Sibling-trait-per-marker was a workaround for the per-marker registration interface; that interface is gone. |

The compile-time-paid-once correction (a separate but parallel issue surfaced this round) was applied to the workspace rule `arvo-compile-time-last.md` and to the `feedback_arvo_design_principles.md` memory description. See Cross-references.

## Open items requiring sketches before lock

These are deliberate open items. Per `cl-claim-sketch-discipline.md`, sketches commit before the doc CL locks. Each open item below maps to a sketch in the next topic file.

### O1. AccessSet propagation under deep stacking

**Question.** Does the typestate-builder approach sustain under realistic kit nesting depth (3-5 levels, 20+ WorkUnits, 10+ stores)?

**Why it matters.** If rustc trait-solver behaviour degrades catastrophically at 4+ levels deep, or compile times balloon nonlinearly, or error messages become unreadable, the static-check approach has a practical ceiling. Sketch defines that ceiling empirically.

**Sketch.** Build a synthetic kit hierarchy 4 levels deep with 25 WUs total, register against a typestate builder, exercise both the success path and a deliberately-missing-resource failure path. Measure compile time, error message, monomorphisation table size.

**Maps to S1 in the next topic.**

### O2. AccessSet arity solve (#333)

**Question.** The current AccessSet flat-impl arity-12 cap is band-aid. Linear stacking through nested kits will hit it. What's the right structural answer — bigger N (32, 64), recursive HList, const-generic length, or a novel mechanism?

**Why it matters.** Round-4 doc CL cannot lock without a stable answer. Lifting to N=32 buys time but leaves the same problem at depth N+1. Need empirical comparison.

**Sketch.** Implement three candidates head-to-head: (a) macro-generated flat-impls up to N=64; (b) recursive HList AccessSet with terminal `()`; (c) const-generic-length with associated-type-machinery. Measure compile-time cost (rustc self-profile), error-message readability when something is missing, and operator simplicity (union, intersection, difference).

**Maps to S2 in the next topic.**

### O3. Replaceable polarity — opt-in vs opt-out

**Question.** Should `Replaceable` be opt-in (kit author impls it for things they're willing to let app override) or opt-out (default-impl on every store, kit author writes `impl !Replaceable for Foo {}` to lock it down)?

Arguments for opt-in: explicit, conservative, minor authorial cost is fine because most things are kit-internal anyway. Default-deny matches the substrate's "no policy" stance — the author chooses what's open.

Arguments for opt-out: ergonomic — most apps want most things replaceable for testing, debugging, and config; opt-out matches the principle of least surprise. Default-allow matches Rust's general philosophy ("things are public-replaceable unless stated otherwise"). And requires nightly `feature(negative_impls)` again, which had the rough sketch failure last round — though for a different mechanism.

**Sketch.** Write both shapes with a small kit example. Inspect ergonomics in real-app shape: how often does the author have to write the `Replaceable` impl in opt-in mode? How often the `!Replaceable` impl in opt-out mode? Which gives clearer error messages?

**Maps to S4 in the next topic.**

### O4. Owned access-scoping taxonomy completeness

**Question.** Are there axes beyond visibility and replaceability that load-bearing kit shapes will need? The "future axes" listed above (lifetime scope, build-order, init-source) are speculative. Concrete kit designs (mockspace, viola, vehje) should reveal whether any are actually needed for v1.

**Why it matters.** If a kit-shape needs an axis we haven't thought of, the round-4 trait surface will need extension. Better to find that now than after lock.

**Sketch.** Write skeletons for three diverse kit shapes — `MockspaceKit` (owns multiple internal stores, requires interner+clock), `BenchTracingKit` (owns a tracer with FFI handle that needs pre-scheduler init), `LintPackKit` (owns Column<Diagnostic> shared with other kits via the cooperative-public path). Inspect whether the two committed axes suffice.

**Maps to S5 in the next topic (if added).**

## Path forward

1. **This topic file** commits to `feat/builder-register-unification` branch alongside the existing two topic files. (Topic 1 = round 1+2 journey; topic 2 = round-3 REJECTED proposal; this is topic 3.)
2. **Sketch topic** comes next: `202605042200_topic_round_4_sketches.md`. Defines S1-S5 (or S1-S4) concretely with hypothesis, scope, success criteria.
3. **Run sketches**: each sketch lives under `mock/research/sketches/<YYYYMMDDHHMM>_<topic>/` with sketch.rs (or directory) plus FINDINGS.md per `cl-claim-sketch-discipline.md` format.
4. **Pivot or confirm.** If a sketch reveals a load-bearing infeasibility (as round-3's NotIn did), restart with a new topic. If sketches confirm, proceed.
5. **Doc CL.** Mechanical, per-crate, per-file: hilavitkutin-api (Kit trait, StoreBundle, AccessSet propagation contracts, Replaceable surface), hilavitkutin-providers (kits become {Units, Owned} shape, BuilderResource removed/renamed), hilavitkutin engine (typestate builder, compile-time AccessSet proof at .build()), hilavitkutin-extensions (if affected). Lock with `cargo mock lock` when stable.
6. **SRC CL.** Structured CHANGE: blocks per `cl-claim-sketch-discipline.md`. Execute, validate, lock, close.

## Cross-references

- `mock/design_rounds/202605042200_topic_builder_register_unification.md` — Topic 1: round 1+2 journey, audit reports, original `BuilderResource -> BuilderRegister<M>` framing.
- `mock/design_rounds/202605042200_topic_markers_as_registrables_final.md` — Topic 2: round-3 REJECTED proposal (marker-as-registrable + `NotIn<H>` compile-time diamond detection via `feature(negative_impls)`).
- `mock/research/sketches/registrable-not-in-202605051200/FINDINGS.md` — empirical evidence the round-3 mechanism cannot work under coherence.
- `~/Dev/clause-dev/.claude/rules/arvo-compile-time-last.md` — workspace rule, rewritten 2026-05-05 (title now "Compile time is paid once; runtime is paid forever") to remove the misreading that allowed agents (and humans) to read it as "prefer runtime checks". Direction of "last" clarified: it's the cost we minimise least, the bucket we pour into, not the place we avoid.
- `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md` — sketch-before-lock requirement.
- `~/Dev/clause-dev/.claude/rules/no-legacy-shims-pre-1.0.md` — pre-1.0 churn is acceptable; v0.1's `BuilderResource` / `InternerKit + BuilderResource<T>` shape can be replaced cleanly without compatibility shims.
- `~/Dev/clause-dev/.claude/rules/hilavitkutin-workunit-mental-model.md` — apps are WorkUnits + scheduler-owned data; kits-bring-WUs is consistent with this.
- Tasks: #330 (umbrella, reframed), #333 (arity solve, now load-bearing), #361 (this topic file), #362 (sketch topic), #363 (run sketches), #364 (doc CL), #365 (SRC CL + close).
