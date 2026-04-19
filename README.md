# hilavitkutin

Morsel-driven pipeline execution engine. `#![no_std]`, no alloc.

Consumers declare typed WorkUnits (Read/Write access sets + scheduling hints). The engine analyses the resulting DAG, forms phases/trunks/fibers, compiles per-core dispatch programs, and runs them on a pre-allocated thread pool.

## Crates

| Crate | Role |
|-------|------|
| `hilavitkutin-api` | Consumer-facing types + traits + platform contracts |
| `hilavitkutin` | Engine (plan, dispatch, resource, thread, strategy, adapt, scheduler) |
| `hilavitkutin-build` | Build-time optimisations (LLVM, MIR, cfg, PGO/BOLT) |
| `hilavitkutin-ctx` | Provider-gated context framework (standalone ext) |
| `hilavitkutin-persistence` | Generic hot-cold storage bridge (standalone ext) |
| `hilavitkutin-str` | Interned string system (standalone ext) |

## Substrate

Depends on the [`arvo`](https://github.com/orgrinrt/arvo) numeric + analysis substrate:
- `arvo-graph` for DAG topology
- `arvo-bitmask` for AccessMasks
- `arvo-sparse` for RCM reordering
- `arvo-spectral` for trunk formation
- `arvo-comb` for fiber grouping

## Status

**Design mature, pre-implementation.** The authoritative spec lives in `mock/design_rounds/202603181200_topic.hilavitkutin-design-consolidation.md` (2480 lines, 22 domains, 9 resolutions). Latest changelist landed 2026-03-24.

Next step: implement `hilavitkutin-api` contracts in the mockspace, then the core engine bottom-up.

## Consumers

Known consumers driving the design:

- **loimu** — signal/DAG framework (consumer research under `mock/design_rounds/`)
- **clause** — language runtime (macros, scratch, generative pipeline)
- **polka-dots**, **saalis** — misc pipeline consumers

## Contributing

Design conversations happen in `mock/design_rounds/`. New topics are timestamped + prefixed (`topic.*`, `research.*`, `changelist.*`). Implementation PRs land in `mock/crates/` against the mockspace validation gate.
