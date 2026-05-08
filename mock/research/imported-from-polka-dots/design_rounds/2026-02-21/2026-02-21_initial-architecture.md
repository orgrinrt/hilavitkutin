# Design Round: Initial Architecture

**Date:** 2026-02-21
**Status:** proposed

## Current state (before this round)

polka-dots is a single-crate Rust binary (~9,300 LOC, 108 tests) with the following
architectural problems identified through deep analysis:

1. **Zero traits** — everything is concrete types with no abstraction boundaries
2. **`toml::Value` leaked everywhere** — raw TOML values are the universal data currency,
   leaking from project loading through DSL expansion to command output
3. **Schema types are raw data bags** — serde structs with no behavioral API
4. **I/O baked into business logic** — `std::fs`, `Command` calls in 6 non-command modules
5. **`&Ui` bleeds into pipeline** — scheduler, trash, and other non-command code takes `&Ui`
6. **Commands bypass `project::load()`** — 5 commands independently deserialize polka.toml
7. **Cross-command coupling** — deploy <-> deploy_ssh bidirectional, stage -> merge, sync -> deploy
8. **Stringly-typed dispatch** — `Machine.platform`, `ServiceEntry.source`, `SecretStep.action`
9. **Scattered platform code** — no guards on audit/services brew/launchctl calls

One true layer violation exists: `update_check.rs` (L3) calls `commands::self_update::check_latest()` (L4).

## Changes proposed in this round

### Decompose into 13 crates across 4 layers

**Layer 0 — Core (no polka deps):**
- `polka-platform` — Platform enum, detection, conditional execution
- `polka-schema` — All TOML schema types with validation methods
- `polka-data` — Typed data model replacing `toml::Value`

**Layer 1 — Domain (depends on Layer 0 only):**
- `polka-registry` — Single entry point for registry loading/querying
- `polka-keybinds` — Keybind resolution with inventory formats
- `polka-dsl` — Parser, expander, builtins, transforms (no I/O)
- `polka-resolve` — File glob resolution

**Layer 2 — Pipeline (depends on Layers 0-1):**
- `polka-project` — THE single entry point for project loading
- `polka-manifest` — Build/deploy state tracking
- `polka-pipeline` — Build/stage/deploy as pure functions returning results

**Layer 3 — Commands:**
- `polka-cli` — CLI definitions, command dispatch, all I/O

**Supporting:**
- `polka-audit` — Audit subsystem returning structured results
- `polka-scheduler` — Pure scheduling logic

### Key design decisions

1. **Traits over generics, no dynamic dispatch** — `impl Trait` and monomorphization.
   `dyn` only where inventory bridging requires it.
2. **Structured results** — pipeline stages return result structs, CLI renders them.
   No `&Ui` below CLI layer.
3. **`DataStore` trait** — replaces `toml::Value` as data access mechanism. Concrete
   types behind a formalized API, not a universal value type.
4. **Keep `inventory`** — the compile-time registration pattern for keybind formats
   and extension points stays. Add trait contracts alongside.
5. **Strangler pattern** — design in mock workspace, validate it compiles, then port
   existing code into the new structure incrementally.

### Execution order

1. Validate mock workspace compiles with stub crates
2. Replace stubs with real trait signatures and struct definitions
3. Validate the signatures compile and the dependency graph holds
4. Port existing implementation code into the new crate structure
5. Delete old monolithic source files as each domain is fully ported

## What this does NOT change

- The .polka DSL syntax and semantics
- The polka.toml configuration format
- CLI command names and arguments
- The inventory registration pattern
- Skip propagation in the DSL
