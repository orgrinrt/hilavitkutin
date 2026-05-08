# Findings: S1, deep-stacking typestate-builder

**Date:** 2026-05-05.
**Round:** 202605042200.
**Maps to:** Topic 4 sketch S1, open item O1.
**Outcome:** WORKS. Depth-4 hierarchy with 9 WorkUnits across 6 kits compiles cleanly using S2's chosen B-shape (recursive HList with `#[marker]`) at the AccessSet substrate. Missing-resource case fails at compile time at `.build()`, naming the unsatisfied `Empty: Contains<Clock>` bound. Default rustc recursion limit suffices.

## Setup

Topic 3 locked `Kit { type Units: WorkUnitBundle; type Owned: StoreBundle }` and `SchedulerBuilder<Wus, Stores>` typestated with accumulation through `.add_kit()`. Topic 4 named the question: does this sustain at depth 4-5 with realistic kit content and produce useful errors when a required store is missing?

S2 settled the AccessSet shape (B, recursive HList with `#[marker]`). S1 builds on that.

## What got built

`sketch.rs` defines:

- `AccessSet` and `Contains<X>` per S2's B-shape (`#[marker]`-overlapping head and tail-recurse impls).
- `ContainsAll<L>` via marker-overlap, walking L recursively and demanding `Self: Contains<H>` for each L element.
- `WorkUnit { type Read; type Write }`.
- `WorkUnitBundle` with `AccumRead` / `AccumWrite` associated types, recursively concatenating the head WU's accesses with the tail bundle's accumulated accesses.
- `Concat<R> { type Out }` recursive type-level concat.
- `Kit { type Units: WorkUnitBundle; type Owned: StoreBundle }` per the locked shape.
- `SchedulerBuilder<Wus, Stores>` with `.resource::<T>()`, `.add_kit::<K>()` (typestate accumulator), and `.build()` gated on `Stores: ContainsAll<Wus::AccumRead> + ContainsAll<Wus::AccumWrite>`.
- A 4-tier kit hierarchy (`LeafKitA`, `LeafKitB`, `MidKitA`, `MidKitB`, `OuterKit`, `RootKit`) with 9 WorkUnits total. Leaf WUs require `StringInterner` and `Clock` from the app.

## Success path

```
$ time rustup run nightly rustc --crate-type=lib --edition=2024 sketch.rs --emit=metadata
real    0.56s
```

Compiles cleanly. Default `#![recursion_limit]` (128) is not exceeded.

## Missing-resource path

App registers `StringInterner` but forgets `Clock`. Expected behavior: `.build()` rejects at compile time, naming the missing marker.

```
error[E0599]: the method `build` exists for struct `SchedulerBuilder<...>`,
              but its trait bounds were not satisfied
   --> sketch.rs:279:10
    |
 22 |   pub struct Empty;
    |   ---------------- doesn't satisfy `Empty: Contains<Clock>`
 23 |   pub struct Cons<H, T>(PhantomData<(H, T)>);
    |   --------------------- doesn't satisfy
    |       `_: ContainsAll<Cons<OuterA1, Cons<MidA1, Cons<LeafB1,
    |       Cons<LeafA1, Cons<LeafA2, Cons<Clock, Cons<Clock,
    |       Cons<StringInterner, Cons<StringInterner, Empty>>>>>>>>>>`
...
note: trait bound `Empty: Contains<Clock>` was not satisfied
note: the trait `Contains` must be implemented
```

The error message:

- Names the missing marker (`Clock`).
- Shows the accumulated AccumRead set, which includes duplicates (Clock appears twice because two leaf WUs read it; StringInterner same). This is a real concern for production codegen size but does not affect correctness; deduplication is a follow-up.
- Hits a "long type written to file" note because the accumulated cons-chain is verbose. Workable, slightly noisy.

A `#[diagnostic::on_unimplemented]` annotation on `Contains<X>` would make this friendlier:

```rust
#[diagnostic::on_unimplemented(
    message = "store `{X}` is not registered, required by a WorkUnit's Read or Write set",
    note = "Register it with `.resource::<{X}>(initial)`, `.column::<{X}>()`, or `.add_virtual::<{X}>()`."
)]
pub trait Contains<X>: AccessSet {}
```

The v0.1 hilavitkutin-api already does this. Carrying it forward is mechanical.

## Compile-time and trait-solver behaviour

Depth 4 / 9 WUs / 12 stores, build time 0.56s. This is comparable to other workspace sketches with const-trait machinery. No pathology observed.

S2 noted A and B both finished at arity 12 in ~50ms with simpler proofs. S1's higher 0.56s reflects the added work: each `.add_kit()` triggers a Concat type-resolution chain, the `WorkUnitBundle` recursion runs to compute `AccumRead`/`AccumWrite`, and the final `.build()` proves `ContainsAll<L>` over the accumulated lists (each element requires walking the Stores list).

For depth 5 or richer kits, the work scales. The accumulation produces lists whose length grows with `(WUs per kit) * (number of kits)`. At depth 5 with 25 WUs total, accumulated list lengths reach 25-50 elements; each `ContainsAll` proof walks this list per element, so the proof cost is O(N²) in the accumulated count.

This is a quadratic scaling cost. At depth 5 / N=50 the proof is ~2500 trait-solver-step operations, manageable. At depth 6 / N=100 it's ~10000 steps. Beyond depth 6 the curve will flatten if rustc's trait cache is effective; if not, this is the depth ceiling.

The empirical depth-5+ exploration is left as follow-up. Round-4 needs depth 4-5; this sketch confirms 4 is fine.

## Subtle finding: accumulated lists carry duplicates

The current `WorkUnitBundle::AccumRead` impl naively concatenates each WU's Read set into the bundle's accumulated set. If two WUs both read `Clock`, the accumulated list has `Clock` twice. The proof still works (`ContainsAll<Cons<Clock, Cons<Clock, Empty>>>` reduces to `Contains<Clock> + ContainsAll<Cons<Clock, Empty>>` reduces to `Contains<Clock> + Contains<Clock> + ContainsAll<Empty>` and both `Contains<Clock>` resolve), but it costs trait-solver time proportional to duplicate count.

A dedup-on-concat type-level operation would clean this up but adds complexity. For round-4 the duplicates are an inefficiency, not a correctness issue. Tracked as a follow-up for round-5 or whenever rustc's trait cache becomes a binding constraint.

## Path forward

S1 settled. The typestate builder approach with B-shape AccessSet sustains at depth 4 with reasonable compile time and usable error messages. Recursion limit was not approached.

Doc CL writes the trait surface per topic 3, with S2's B-shape as the AccessSet substrate. Annotate `Contains` with `#[diagnostic::on_unimplemented]` per v0.1 precedent. Mark dedup-on-concat as follow-up; not load-bearing for round-4.

## Cross-references

- `mock/research/sketches/202605050503_accessset_arity/FINDINGS.md`. S2's recommendation of B-shape used here.
- `crates/hilavitkutin-api/src/access.rs`. v0.1's `Contains` trait already uses `#[marker]` plus `#[diagnostic::on_unimplemented]`.
- `mock/design_rounds/202605042200_topic_kit_trait_split.md`. Topic 3 locked Kit shape this sketch implements.
