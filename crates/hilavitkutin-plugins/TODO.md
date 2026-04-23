# TODO — `hilavitkutin-plugins`

Status: planning scaffold only (no implementation yet)

## Phase 1 — Design

- [ ] Lock crate scope and boundaries
  - [ ] Confirm this crate is plugin-host orchestration built on extension loading machinery
  - [ ] Confirm non-goals (no domain-specific plugin contracts, no Viola-specific model logic)
  - [ ] Confirm relationship to `hilavitkutin-extensions` and downstream host crates
  - [ ] Confirm explicit, pull-based discovery model (no `inventory`/ctor linker magic)
- [ ] Define conceptual model (Loimu-inspired modules vs plugins split)
  - [ ] Extension = loadable binary unit
  - [ ] Plugin = contract-conforming feature unit loaded via extension
  - [ ] Host = orchestrator for plugin lifecycle and dispatch
- [ ] Freeze v1 compatibility policy
  - [ ] Host/plugin major version compatibility rule
  - [ ] Additive minor evolution policy
  - [ ] Rejection behavior for incompatible plugin contracts

## Phase 2 — API / Contracts

- [ ] Define plugin host abstraction interfaces
  - [ ] Plugin descriptor interface
  - [ ] Role/capability declaration interface
  - [ ] Lifecycle hooks interface (`initialize`, `invoke`, `shutdown`)
- [ ] Define registration and resolution contracts
  - [ ] Explicit C-ABI entry point signature (e.g., `__plugin_get_descriptor`)
  - [ ] Plugin identity uniqueness rules
  - [ ] Role-based lookup contract
  - [ ] Capability matching contract
- [ ] Define invocation and result contracts
  - [ ] Standard invocation request/response shape
  - [ ] Structured host/plugin error model
  - [ ] Retryable vs non-retryable failure classification
- [ ] Define execution policy contracts
  - [ ] Required vs optional plugin semantics
  - [ ] Fail-closed / fail-open policy hooks
  - [ ] Deterministic ordering guarantees for plugin execution/aggregation

## Phase 3 — Implementation

- [ ] Scaffold crate structure
  - [ ] `host/` (orchestration core)
  - [ ] `registry/` (registration + indexing)
  - [ ] `dispatch/` (role/capability invocation)
  - [ ] `lifecycle/` (init/invoke/shutdown management)
  - [ ] `errors/` (structured error types and codes)
- [ ] Implement plugin registry
  - [ ] Actively extract descriptors via explicit symbol calls (pull-based)
  - [ ] Register plugin descriptors
  - [ ] Validate descriptor completeness/compatibility
  - [ ] Resolve by role/capability
- [ ] Implement lifecycle orchestration
  - [ ] Initialize all selected plugins
  - [ ] Invoke role operations with policy controls
  - [ ] Shutdown in deterministic reverse-safe order
- [ ] Implement policy wiring
  - [ ] Required plugin load/initialize failure handling
  - [ ] Optional plugin policy handling
  - [ ] Invocation failure propagation and containment
- [ ] Integrate extension-loader dependency boundary
  - [ ] Consume loading results from `hilavitkutin-extensions`
  - [ ] Map low-level load/link errors into plugin-host error model

## Phase 4 — Tests

- [ ] Add contract validation tests
  - [ ] Duplicate plugin identity rejection
  - [ ] Missing lifecycle/role contract rejection
  - [ ] Version incompatibility rejection
- [ ] Add lifecycle tests
  - [ ] Happy path init/invoke/shutdown
  - [ ] Partial failure behavior with policy variants
  - [ ] Shutdown guarantees after failure
- [ ] Add dispatch tests
  - [ ] Role-based resolution correctness
  - [ ] Capability matching correctness
  - [ ] Deterministic invocation ordering rules
- [ ] Add error-model tests
  - [ ] Structured code/message/details mapping
  - [ ] Retryable flag behavior expectations
  - [ ] Low-level loader error translation coverage

## Phase 5 — Docs / Release

- [ ] Write crate README
  - [ ] Purpose and boundaries
  - [ ] Relationship to `hilavitkutin-extensions`
  - [ ] Loimu-inspired conceptual reference
  - [ ] Architectural rationale (why pull-based discovery and strict C-ABI boundaries)
  - [ ] Guidance for downstream hosts on macro-driven static monomorphization (Saalis-inspired)
- [ ] Add architecture notes
  - [ ] Plugin lifecycle state model
  - [ ] Host policy model
  - [ ] Compatibility/versioning policy summary
- [ ] Prepare first implementation milestone checklist
  - [ ] API review complete
  - [ ] Tests for v1 baseline complete
  - [ ] Downstream integration notes drafted