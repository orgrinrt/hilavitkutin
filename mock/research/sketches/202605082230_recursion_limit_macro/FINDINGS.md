**Date:** 2026-05-08
**Sketch:** S1, recursion-limit macro probe
**Outcome:** FAILS WITH `error: an inner attribute is not permitted in this context`

# Findings: macro-expansion to crate-level inner attribute

## Hypothesis tested

Round 202605042200's locked topic file proposed shipping a `recursion_limit_for_kits!()` macro that consumers invoke at their crate root, expanding to `#![recursion_limit = "1024"]`. The macro shape:

```rust
#[macro_export]
macro_rules! recursion_limit_for_kits {
    () => { #![recursion_limit = "1024"] };
}
```

Consumer usage:

```rust
// lib.rs
hilavitkutin_api::recursion_limit_for_kits!();
```

The hypothesis was that current rustc accepts macros that expand to crate-level inner attributes when invoked at the very top of the crate root.

## Result

Compilation fails with `error: an inner attribute is not permitted in this context` pointing at the `#![recursion_limit = "1024"]` token inside the macro body. rustc's diagnostic note: "inner attributes ... annotate the item enclosing them, and are usually found at the beginning of source files".

The reason: `macro_rules!` invocation positions are item-positions, and macro expansion produces items. An inner attribute is bound to a containing item (the crate root for `#![...]`), and the rule is that inner attributes must appear textually before items, not as items themselves. Even when the macro invocation is the very first token after module-level doc comments and outer attributes, the expansion does not happen "before" the item position; it happens AT the item position, and items can't be inner attributes.

This is a stable rustc behaviour. Tracking issue rust-lang/rust#73933 (and related) discusses the gap; no plan to resolve has converged. The pattern simply does not work in rustc 2026-05-08 (nightly toolchain in use here).

## Decision

Drop the `recursion_limit_for_kits!()` macro from the public substrate. The round's scope refocuses on:

1. Documenting `#![recursion_limit = "1024"]` as a manual consumer-side directive in DESIGN.md.tmpl with thorough guidance on when to bump.
2. Wiring the trait diagnostics (`#[diagnostic::on_unimplemented]`) to mention the attribute in the relevant `note` field so consumers hitting "overflow evaluating the requirement" land on a message that names the fix verbatim.
3. Filing a follow-up workspace task to revisit if rustc gains crate-root inner-attribute-from-macro support.

## Why a non-functional macro is worse than no macro

Shipping a `recursion_limit_for_kits!()` macro that does NOT expand to the attribute (because it cannot) would be a worse outcome than documenting the directive plainly. Consumers would assume the macro DID something, fail to set the limit when needed, and hit the same overflow they would have without any substrate work. The guide-by-diagnostic approach is the honest substitute that actually solves the problem.

## What this round still ships

- `prelude` module with re-exports (no macro).
- `#[diagnostic::on_unimplemented]` coverage across the load-bearing public trait surface in hilavitkutin-api and hilavitkutin-kit.
- DESIGN.md.tmpl prose documenting the recursion-limit issue, the manual-attribute fix, and the depth thresholds where bumping becomes necessary.
- Diagnostic messages on `Contains<S>` / `ContainsAll<L>` / `Kit` / `WorkUnit` mention the recursion-limit directive in their notes when relevant.

## Sketch artefacts

`sketch.rs` in this directory carries the failing reproducer. Compile with `rustc --edition 2024 sketch.rs` to reproduce.

## Cross-references

- Round 202605042200 topic `markers_as_registrables_final.md` lines 254-269 (the original macro design).
- `~/Dev/clause-dev/.claude/rules/cl-claim-sketch-discipline.md` (the discipline that mandated this sketch).
- Workspace task #332 (HILA-AUDIT-A3) — this round.
- New follow-up task TBD (revisit when rustc supports inner-attribute-from-macro at crate root).
