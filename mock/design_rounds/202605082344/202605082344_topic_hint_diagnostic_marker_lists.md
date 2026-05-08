**Date:** 2026-05-08
**Phase:** TOPIC
**Scope:** hilavitkutin-api/src/hint.rs
**Source topics:** PR #68 reviewer findings, follow-up to round 202605082321

# Hint diagnostic notes: correct the implementor lists

## Background

Round 202605082321 (PR #68) added `#[diagnostic::on_unimplemented]` across the public trait surface. The four sealed-axis diagnostics in `hint.rs` (`UrgencyValue`, `DivisibilityValue`, `SignificanceValue`, `SchedulingHint`) listed wrong implementor sets in their `note` fields. The error originated in topic Decision 3 of round 202605082321 (line 120 of the topic file), was carried verbatim into the src CL, and into source. The src CL's `## CHANGE:` block verifications only checked attribute presence (`grep -c on_unimplemented`), not content correctness against the actual `impl UrgencyValue for X` lines elsewhere in the file. The PR reviewer caught it before merge.

This round corrects the four notes and ships nothing else.

## Workspace sweep — actual implementor sets

Verified by `grep -n "impl X for"` in `mock/crates/hilavitkutin-api/src/hint.rs`:

- `UrgencyValue`: `Immediate`, `Steady`, `Relaxed`, `Deferred` (4 markers)
- `DivisibilityValue`: `Atomic`, `Adaptive`, `Interruptible` (3 markers)
- `SignificanceValue`: `Critical`, `Important`, `Normal`, `Opportunistic`, `Optional` (5 markers)

Round 202605082321's notes had three classes of error:

1. `UrgencyValue` note named `Adaptive` and `Opportunistic` (which impl Divisibility and Significance respectively, not Urgency).
2. `DivisibilityValue` note named `Divisibility` (the type alias for the underlying `UFixed`, not a marker) and omitted `Adaptive`.
3. `SignificanceValue` note named `Significance` (same type-alias-not-marker error) and omitted `Opportunistic`.

The `SchedulingHint` aggregate note carried the same three errors verbatim.

## Decisions

### Decision 1: scope is the four hint.rs diagnostic note fields, nothing else

Source CL touches `mock/crates/hilavitkutin-api/src/hint.rs` only. Four `note = ...` strings are corrected. No other diagnostic, no other trait, no documentation edits. The DESIGN.md.tmpl `### Diagnostic coverage across the public surface` subsection from round 202605082321 stays unchanged: it does not enumerate the markers per axis, only that the axes carry diagnostics.

### Decision 2: corrected note text

`UrgencyValue`:
> Available markers: `Immediate`, `Steady`, `Relaxed`, `Deferred`. Sealed; consumer-defined markers are not supported.

`DivisibilityValue`:
> Available markers: `Atomic`, `Adaptive`, `Interruptible`. Sealed; consumer-defined markers are not supported.

`SignificanceValue`:
> Available markers: `Critical`, `Important`, `Normal`, `Opportunistic`, `Optional`. Sealed; consumer-defined markers are not supported.

`SchedulingHint`:
> SchedulingHint is implemented on the tuple `(U: UrgencyValue, D: DivisibilityValue, S: SignificanceValue)`. Use the substrate-provided ZST markers (`Immediate` / `Steady` / `Relaxed` / `Deferred` for U; `Atomic` / `Adaptive` / `Interruptible` for D; `Critical` / `Important` / `Normal` / `Opportunistic` / `Optional` for S).

### Decision 3: `## CHANGE:` verification grep extends beyond attribute presence

Per the lesson learned (workspace task #397), the src CL of this round uses verification commands that grep both for attribute presence AND for the corrected marker lists, with the result checked against the source's `impl X for Y` lines.

## Out of scope

- The mockspace lint extension to flag CHANGE-block claims that drift from source enumerations (workspace task #397). That's a separate refinement to the discipline rule and the future #318 lint; this round only fixes the immediate source.
- Any other diagnostic in the api crate. The prior round's `Replaceable`, `Contains`, `ContainsAll`, `Concat`, etc notes were verified by the reviewer and remain correct.

## Lock criteria

- The four `note = ...` strings in `hint.rs` carry the verbatim text from Decision 2.
- `cargo check --workspace` passes clean.
- `cargo test -p hilavitkutin-api --test access_set` still passes 7/7.
- No other source edits; no template edits.
