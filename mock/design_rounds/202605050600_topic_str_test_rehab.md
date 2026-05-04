**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** hilavitkutin-str test rehab. Fix three classes of test-compilation regression introduced by arvo Round D adoption: trait-not-in-scope on `from_narrowed`, missing `feature(const_trait_impl)` in test crates, and `Maybe<&str>` vs `Option<&str>` mismatch in interner test assertions. Partial #291 (str only).
**Source topics:** Task #291 hilavitkutin test rehab (post-Round-D API debt). Discovered during overnight pivot from #330 parking.

# Topic: hilavitkutin-str test rehab (partial #291)

Three failure modes block hilavitkutin-str test compilation post-arvo-Round-D:

1. `Bits::<28>::from_narrowed(...)` resolves only when the `BitsRefitCtor` trait is in scope. The `str_const!` macro emits this call. Fix: macro brings the trait into scope via `use ::arvo_bits::BitsRefitCtor as _;` at the start of its expansion block.
2. `from_narrowed` is on a `pub const trait`. Calling it inside a `static` initialiser (which `str_const!` does) requires the consumer crate to enable `feature(const_trait_impl)`. Test files lack the feature flag. Fix: add `#![feature(const_trait_impl)]` to each of the three str test files using `str_const!`.
3. `StringInterner::resolve` returns `Maybe<&str>`, tests assert with `Some(...)`. Fix: import `notko::Maybe` and replace `Some(...)` with `Maybe::Is(...)` in interner.rs test assertions.

The remaining hilavitkutin engine test failures (mismatched types in dispatch / accumulator / thread pool, missing Display on UFixed, missing `Not` for Bool) are deeper and tracked under #291 as continuing scope. This round closes the str-only subset.

## Decision

Apply the three fixes. Land as a contained PR addressing approximately one third of #291's surface (the str-test subset). Update #291 description with the breakdown post-merge.
