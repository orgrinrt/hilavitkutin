**Date:** 2026-05-08
**Phase:** TOPIC
**Scope:** hilavitkutin (engine), hilavitkutin-providers, hilavitkutin-api docs, hilavitkutin-kit docs
**Source topics:** task #381 (HILA-AUDIT-A5)

# SchedulerBuilder method renames for naming consistency

## Background

The `SchedulerBuilder` ships with a mixed verb-prefix surface that does not communicate the principle behind the split. Three groups coexist with three different shapes:

- `.add::<W>()` — register a WorkUnit. Bare-verb form.
- `.resource::<T>(initial)`, `.resource_default::<T>()`, `.column::<T>()` — register typed stores. No verb prefix.
- `.add_virtual::<T>()`, `.add_kit::<K>()` — register Virtual / Kit. `.add_` prefix.
- `.memory(provider)`, `.threads(pool)`, `.clock(clock)` — set platform contracts. No verb prefix.

A consumer reading the surface for the first time sees three different conventions and has no way to know which form a method takes without looking each up. The audit (#381) flags this as a substrate-completion ergonomic issue: every consumer-facing builder needs one principled convention.

This round picks the convention and applies it across source, doc comments, and DESIGN.md.tmpl content.

## Principle

The builder has two semantic axes:

1. **Registration**: methods that grow the typestate (extend the `Wus` or `Stores` cons-list at the type level). Six methods: WorkUnit, Resource (with-initial and default-init), Column, Virtual, Kit.
2. **Configuration**: methods that set a platform contract (Memory provider, Thread pool, Clock). Three methods. They do not grow the typestate; they bind a generic parameter.

Each axis gets one verb prefix. The split is principled (different runtime semantics) and idiomatic Rust (`add_*` for typestate growth, `with_*` for configurator setters in builder patterns).

## Decisions

### Decision 1: registration methods all use `.add_*` prefix

| Current | Renamed | Notes |
|---|---|---|
| `.add::<W>()` | `.add_unit::<W>()` | Clarifies what's being added. Single bare-verb `add` is too ambiguous in a method-rich builder. |
| `.resource::<T>(initial)` | `.add_resource::<T>(initial)` | |
| `.resource_default::<T>()` | `.add_resource_default::<T>()` | Retained as ergonomic for `T: Default`. |
| `.column::<T>()` | `.add_column::<T>()` | |
| `.add_virtual::<T>()` | `.add_virtual::<T>()` | UNCHANGED (already correct prefix). |
| `.add_kit::<K>()` | `.add_kit::<K>()` | UNCHANGED (already correct prefix). |

Rationale: every `.add_*` call extends the typestate. Reading a builder chain `.add_kit::<X>().add_resource::<Y>(...).add_unit::<Z>()` reads as "register X, then Y, then Z" with no ambiguity about what each verb does.

### Decision 2: configuration methods all use `.with_*` prefix

| Current | Renamed | Notes |
|---|---|---|
| `.memory(provider)` | `.with_memory(provider)` | |
| `.threads(pool)` | `.with_threads(pool)` | |
| `.clock(clock)` | `.with_clock(clock)` | |

Rationale: `.with_*` is the idiomatic Rust builder verb for "configure with this value", commonly seen across the ecosystem (`PathBuf::with_extension`, `String::with_capacity`-shape patterns at type level, etc). Distinguishes platform-contract bindings from typestate-growing registrations.

The rename also disambiguates from the `Context<P>::memory()` accessor (returning `&Self::Provider`); the builder's setter is `.with_memory(provider)` while the runtime accessor stays as `.memory()` on the context. Different namespaces, but the explicit prefix on the builder side reads more clearly.

### Decision 3: no aliases, no deprecated shims

Per `no-legacy-shims-pre-1.0.md`. Direct rename. The seven changed methods (one was bare `add`; three platform setters; three typed-store registrations) get their new names; the old names are deleted from the inherent impl. Every call site in source AND every doc-comment / diagnostic-note reference updates to the new names.

### Decision 4: `Scheduler::builder()`, `.build()`, `.replace_resource()` unchanged

`Scheduler::builder()` is a static constructor, not a builder method. Stays.

`.build()` is the finalizer. Stays.

`.replace_resource()` is a runtime method on `Scheduler` (not the builder), so the rename rules do not apply. Stays.

### Decision 5: documentation surface synchronised

Every place in the workspace that names a SchedulerBuilder method updates:

- `mock/crates/hilavitkutin-api/src/access.rs` — `Contains` and `ContainsAll` diagnostic notes mention `.resource::<T>(initial)`, `.column::<T>()`, `.add_virtual::<T>()`. Renamed to `.add_resource::<T>(initial)`, `.add_column::<T>()` (`.add_virtual` stays).
- `mock/crates/hilavitkutin-api/src/work_unit.rs` — `WorkUnitBundle` diagnostic note mentions `.add::<W>()`. Renamed to `.add_unit::<W>()`.
- `mock/crates/hilavitkutin-api/src/store.rs` — `StoreBundle` diagnostic note mentions `.resource::<T>(initial)`, `.column::<T>()`, `.add_virtual::<T>()`. Renamed.
- `mock/crates/hilavitkutin-kit/src/lib.rs` — `Kit` diagnostic note + module doc mention `.add_kit::<K>()` (unchanged).
- `mock/crates/hilavitkutin-providers/src/lib.rs`, `src/interner.rs` — doc comments reference `builder.resource(...)`. Renamed to `builder.add_resource(...)`.
- `mock/crates/hilavitkutin-providers/tests/smoke.rs` — uses `.add_kit::<...>()` (unchanged).
- `mock/crates/hilavitkutin/tests/scheduler_builder.rs` — extensive call sites for `.add::<W>()`, `.resource(...)`, `.column::<T>()`, `.add_kit::<K>()`. Renamed accordingly.
- `mock/crates/hilavitkutin-api/DESIGN.md.tmpl` — any prose example calling the methods updates.
- `mock/crates/hilavitkutin/DESIGN.md.tmpl` — same.
- `mock/crates/hilavitkutin-kit/DESIGN.md.tmpl` — same (Kit-side examples).
- `README.md` (repo root) — same if any concrete builder example exists.

### Decision 6: workspace consumers (vehje, viola) — no edits required

Workspace sweep confirmed zero call sites in vehje or viola. Cross-repo consumers will pick up the new names when they integrate.

## Out of scope

- Folding `.add_resource_default` into `.add_resource` via `T: Default` overload (task #299). Different round, different mechanism.
- `register![]` macro for multi-store chains (task #350). Different round, requires Decision 1 first.
- Builder typestate redesign or trait-shape changes. This round is a pure rename.
- Reverting any names. Decision 3 is direct rename, no aliases.

## Lock criteria

- All seven methods renamed in `mock/crates/hilavitkutin/src/scheduler/mod.rs`.
- Every call site across the workspace updated to the new names.
- Every doc comment / diagnostic note that names a builder method uses the new name.
- `cargo check --workspace` passes clean.
- `cargo test --workspace` passes the same set as before (no test-shape changes; just identifier renames).
- No `lint:allow` additions, no severity downgrades.
- Workspace sweep grep `\.add::|\.resource\(|\.resource_default\(|\.column::|\.memory\(|\.threads\(|\.clock\(` against `mock/crates/hilavitkutin*/src/*.rs` and `mock/crates/hilavitkutin*/tests/*.rs` returns zero hits in builder-method position (preserving `ctx.memory()` / `ctx.threads()` / `ctx.clock()` accessor calls — those are different traits with different namespaces).
