# TODO — `hilavitkutin-extensions`

Status: planning scaffold only (no implementation yet)

## Design

- [ ] Lock crate purpose and boundaries
  - [ ] Confirm this crate owns only dynamic extension loading/linking machinery
  - [ ] Confirm this crate does not own plugin contract semantics
  - [ ] Confirm this crate stays domain-agnostic (no Viola-specific logic)
  - [ ] Confirm strict `cdylib` / C-ABI optimization boundary
  - [ ] Confirm explicit ban on linker-magic discovery (no `inventory` / `.init_array` reliance)
- [ ] Define platform support policy
  - [ ] macOS (`.dylib`) support expectations
  - [ ] Linux (`.so`) support expectations
  - [ ] Windows (`.dll`) support expectations
- [ ] Define safety model
  - [ ] Loader lifecycle guarantees
  - [ ] Symbol resolution guarantees
  - [ ] Unload behavior and limitations
- [ ] Define compatibility posture
  - [ ] Extension-level version/compatibility check hooks
  - [ ] Error categories for incompatibility
  - [ ] Deterministic failure behavior

## API / Contracts

- [ ] Define public loader abstractions
  - [ ] Extension handle type(s)
  - [ ] Symbol handle type(s) (emphasize explicit, pull-based symbol extraction)
  - [ ] Loader options/config type(s)
- [ ] Define load/resolve lifecycle API
  - [ ] Open extension API
  - [ ] Resolve symbol API
  - [ ] Close/unload API (if supported)
- [ ] Define error model
  - [ ] Structured error enums/kinds
  - [ ] Source context attachment (path/symbol/platform)
  - [ ] Retryability semantics where applicable
- [ ] Define validation helpers
  - [ ] Symbol presence checks
  - [ ] Version guard helpers
  - [ ] Minimal contract-shape validation helpers

## Implementation

- [ ] Scaffold crate layout
  - [ ] `loader/` module
  - [ ] `symbols/` module
  - [ ] `errors/` module
  - [ ] `compat/` module
- [ ] Implement cross-platform loading backend
  - [ ] macOS backend
  - [ ] Linux backend
  - [ ] Windows backend
- [ ] Implement symbol resolution path
  - [ ] Required symbol resolve
  - [ ] Optional symbol resolve
  - [ ] Typed symbol wrapper helpers
- [ ] Implement error propagation and diagnostics
  - [ ] Path-not-found errors
  - [ ] Load/link failure errors
  - [ ] Missing-symbol errors
- [ ] Implement validation utilities
  - [ ] Required symbol set checks
  - [ ] Compatibility helper functions

## Tests

- [ ] Add unit tests for core APIs
  - [ ] Loader initialization/options tests
  - [ ] Symbol lookup success/failure tests
  - [ ] Error mapping tests
- [ ] Add platform-focused tests
  - [ ] Platform artifact extension handling tests
  - [ ] Platform-specific load failure behavior tests
- [ ] Add compatibility/validation tests
  - [ ] Missing required symbols
  - [ ] Incompatible version marker cases
  - [ ] Deterministic error outputs
- [ ] Add integration smoke tests
  - [ ] Load known-good test extension
  - [ ] Resolve known symbols
  - [ ] Verify graceful teardown path

## Docs / Release

- [ ] Add crate README
  - [ ] Scope and non-goals
  - [ ] Public API overview
  - [ ] Relationship to `hilavitkutin-plugins`
- [ ] Document architectural rationale
  - [ ] Extensions vs plugins boundary summary (Loimu-inspired)
  - [ ] Why `cdylib` and `dlsym` are mandated (Polka-dots benchmark optimization barriers)
  - [ ] Why `inventory`/linker-magic is banned (Saalis cross-platform fragility)
  - [ ] Reuse intent across downstream hosts
- [ ] Add usage guidance
  - [ ] Basic loading flow example
  - [ ] Required symbol validation flow example
- [ ] Prepare initial release checklist
  - [ ] API review complete
  - [ ] Tests passing across target platforms
  - [ ] Changelog/release notes draft