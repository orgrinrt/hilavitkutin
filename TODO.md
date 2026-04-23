# TODO — hilavitkutin extensions/plugins split (Loimu-aligned)

Status: planning only (docs-first, no implementation yet)

## Phase 1 — Design

- [ ] Lock architecture split
  - [ ] Confirm `extensions` = low-level dynamic loading/linking only
  - [ ] Confirm `plugins` = host-level plugin contract orchestration
  - [ ] Confirm no domain-specific logic in shared crates
  - [ ] Confirm strict `cdylib` / C-ABI optimization boundary
  - [ ] Confirm explicit ban on linker-magic discovery (no `inventory` / `.init_array` reliance)
- [ ] Lock Loimu-aligned terminology and boundaries
  - [ ] Extension (binary loading unit) vs Plugin (contract-bound feature unit)
  - [ ] Ownership boundaries between loader and host layers
  - [ ] Error ownership boundaries (link/load errors vs plugin contract errors)
- [ ] Define downstream assumptions
  - [ ] Target reuse by Viola host/plugin stack
  - [ ] Keep crates general-purpose (not Viola-specific)
  - [ ] Confirm expected integration direction with `viola-plugin-abi`

## Phase 2 — Crate scaffolding

- [ ] Create `crates/hilavitkutin-extensions/`
  - [ ] Add crate README with scope/non-goals
  - [ ] Add crate TODO checklist
  - [ ] Add placeholder module layout
- [ ] Create `crates/hilavitkutin-plugins/`
  - [ ] Add crate README with scope/non-goals
  - [ ] Add crate TODO checklist
  - [ ] Add placeholder module layout
- [ ] Add workspace metadata (when implementation starts)
  - [ ] Workspace members
  - [ ] Shared lint/format settings
  - [ ] Basic CI checks for new crates

## Phase 3 — API/Contract definitions

- [ ] Define `hilavitkutin-extensions` API surface
  - [ ] Library load/unload abstraction
  - [ ] Symbol resolution abstraction (emphasize explicit, pull-based symbol extraction)
  - [ ] Platform artifact handling strategy
  - [ ] Structured link/load error model
- [ ] Define `hilavitkutin-plugins` API surface
  - [ ] Explicit C-ABI entry point signature (e.g., `__plugin_get_descriptor`)
  - [ ] Plugin descriptor registration/validation
  - [ ] Lifecycle orchestration contract (`init/invoke/shutdown`)
  - [ ] Role/capability mapping abstractions
  - [ ] Host-side plugin failure policy hooks
- [ ] Define compatibility/version policy
  - [ ] Extension boundary version checks
  - [ ] Plugin contract version checks
  - [ ] Mismatch classification and reporting rules

## Phase 4 — Implementation

- [ ] Implement `hilavitkutin-extensions` core
  - [ ] Cross-platform library loading backend
  - [ ] Symbol resolution backend
  - [ ] Safe wrappers and guardrails
  - [ ] Structured error propagation
- [ ] Implement `hilavitkutin-plugins` core
  - [ ] Plugin registry and validation
  - [ ] Lifecycle dispatcher
  - [ ] Role/capability dispatch plumbing
  - [ ] Deterministic host execution helpers
- [ ] Implement integration seams for downstream hosts
  - [ ] Loader-to-plugin-host handoff
  - [ ] Contract mismatch rejection behavior
  - [ ] Optional hooks for host-level policy decisions

## Phase 5 — Tests

- [ ] Add `hilavitkutin-extensions` tests
  - [ ] Load success cases
  - [ ] Missing symbol cases
  - [ ] Invalid artifact cases
  - [ ] Cross-platform behavior sanity checks
- [ ] Add `hilavitkutin-plugins` tests
  - [ ] Descriptor validation cases
  - [ ] Lifecycle ordering (`init -> invoke -> shutdown`)
  - [ ] Capability mismatch rejection
  - [ ] Deterministic invocation ordering (where applicable)
- [ ] Add integration tests between crates
  - [ ] End-to-end plugin load + lifecycle
  - [ ] Error mapping consistency
  - [ ] Failure policy path coverage

## Phase 6 — Docs and rollout

- [ ] Add crate-level architecture docs
  - [ ] Responsibilities and non-goals
  - [ ] Public API intent
  - [ ] Versioning/compatibility policy summary
  - [ ] Architectural rationale (why `cdylib` boundaries and pull-based discovery)
- [ ] Add Loimu references
  - [ ] Link relevant Loimu docs/code in README files
  - [ ] Note what is adopted vs intentionally different
- [ ] Prepare first milestone checklist
  - [ ] “Loader usable” acceptance criteria
  - [ ] “Plugin host usable” acceptance criteria
  - [ ] Downstream consumer trial plan (Viola-focused)

## Tracking

- [x] Root TODO normalized into phased checklist format
- [ ] Crate skeletons created
- [ ] Crate READMEs aligned with this plan
- [ ] API contract drafts written
- [ ] Initial implementation started