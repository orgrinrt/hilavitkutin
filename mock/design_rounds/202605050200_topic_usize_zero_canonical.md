**Date:** 2026-05-05
**Phase:** TOPIC
**Scope:** hilavitkutin-api: replace literal `USize(0)` with `USize::ZERO` for const-canonical clarity. Three sites in api crate (id.rs, sink.rs, access.rs).
**Source topics:** Task #328 (hilavitkutin-api: StoreId Default uses USize(0); migrate to USize::ZERO).

# Topic: USize(0) → USize::ZERO canonicalisation in hilavitkutin-api

arvo's `USize::ZERO` constant has shipped (#306, arvo Round 8) and is in widespread use across the workspace. The `hilavitkutin-api` crate has three sites that still use the literal `USize(0)` form, which obscures intent and breaks symmetry with adjacent code that uses `USize::ZERO`.

## Sites

1. `mock/crates/hilavitkutin-api/src/id.rs:20` — `StoreId::default()` returns `StoreId(USize(0))`. Per task #328.
2. `mock/crates/hilavitkutin-api/src/sink.rs:74` — `CountingSink::new()` const fn body initialises `count: USize(0)`.
3. `mock/crates/hilavitkutin-api/src/access.rs:42` — `AccessSet::LEN` for `()` is `USize(0)`.

All three are pure value sites (not type tokens). All three replace cleanly with `USize::ZERO` once `arvo::strategy::Identity` is in scope.

## Decision

Replace the three literals. Add the `Identity` import to each module that gains a `USize::ZERO` reference.

## Per-rule compliance

- `no-bare-primitives.md`: `USize::ZERO` is canonically arvo-typed. No bare primitives introduced; one bare `0` literal is removed.
- `writing-style.md`: trivial mechanical change; doc comments unchanged.
- `cl-claim-sketch-discipline.md`: structured `## CHANGE:` blocks belong in the SRC CL.
