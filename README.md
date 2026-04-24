# `hilavitkutin`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/hilavitkutin.svg)](https://github.com/orgrinrt/hilavitkutin/stargazers)
[![Crates.io](https://img.shields.io/crates/v/hilavitkutin)](https://crates.io/crates/hilavitkutin)
[![docs.rs](https://img.shields.io/docsrs/hilavitkutin)](https://docs.rs/hilavitkutin)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/hilavitkutin.svg)](https://github.com/orgrinrt/hilavitkutin/issues)
![License](https://img.shields.io/github/license/orgrinrt/hilavitkutin?color=%23009689)

> Morsel-driven pipeline execution engine. Typed WorkUnits, DAG analysis, pre-allocated thread pool, static composition. `#![no_std]`, no alloc, no runtime spawn.

</div>

## What it is

`hilavitkutin` is a pipeline execution engine. Consumers declare typed WorkUnits with read/write access sets and scheduling hints; the engine analyses the resulting DAG into phases, trunks, and fibers, compiles per-core dispatch programs, and runs them on a pre-allocated thread pool. Every crate in the stack is `#![no_std]`, no `alloc`, no runtime spawn, no dynamic dispatch. Monomorphisation is the dispatch.

Composition is static. All WorkUnits register at compile time via the scheduler builder. Plan parameters adapt at runtime (morsel size, fiber assignment, trunk fusion), but the set of WorkUnits does not. Linker-magic registration patterns (`inventory::`, `#[ctor]`, `#[distributed_slice]`, `.init_array`) are banned across every crate in the stack, engine and plugin-host layer alike.

Plan and schedule happen once at pipeline construction, then reuse across frames. Recompute is rare and triggered only by structural change. A plugin-host layer (`hilavitkutin-linking` / `hilavitkutin-extensions` / `hilavitkutin-extensions-macros`) provides pull-based dynamic loading for downstream consumer hosts; it is not runtime WorkUnit discovery, and the engine itself never loads WorkUnits at runtime.

## Contents

| Crate | Role |
|---|---|
| `hilavitkutin-api` | Consumer-facing `WorkUnit` / `AccessSet` / `Column` / `Resource` / `Virtual` / `Context` / platform contracts. |
| `hilavitkutin` | Engine: plan, dispatch, resource, thread, strategy, adapt, scheduler, intrinsics, platform. |
| `hilavitkutin-build` | Build-time optimisations: LLVM passes, MIR manipulation, cfg emission, PGO / BOLT, ExpandedLto pragmas. |
| `hilavitkutin-ctx` | Provider-gated context framework (standalone extension). |
| `hilavitkutin-persistence` | Generic hot-cold storage bridge (standalone extension). |
| `hilavitkutin-str` | Interned string system (standalone extension). |
| `hilavitkutin-linking` | Cross-platform dynamic library loader primitive; `no_std`, no allocator, pull-based symbol resolution. |
| `hilavitkutin-extensions` | Contract-bound host orchestration over `hilavitkutin-linking`: descriptor shape, lifecycle, capability dispatch. |
| `hilavitkutin-extensions-macros` | Proc-macro companion: emits `#[repr(C)]` descriptors and capability trampolines via `#[export_extension]`. |

## Status

**Design mature, pre-implementation.** The authoritative spec lives in `mock/design_rounds/202603181200_topic.hilavitkutin-design-consolidation.md` (22 domains, 9 cross-domain resolutions). The plugin-host layer (linking + extensions + macros) is implemented in the mockspace; the engine and the standalone extensions are next.

## Vocabulary

The engine's canonical hierarchy, coarsest to finest:

```
pipeline → core → phase ↔ waist → trunk → fiber ↔ branch ↔ bridge
  → morsel → micro-morsel → entry
```

A `record` is one data point in a column, never a `row`, never an `entity`. A `morsel` windows into a range of records. Columns are independent; there is no tabular join. Dead terms (`chain`, `chain_group`, `partition`, `archetype`, `entity`, `row`, `order`) are scanned for by the vocabulary lint and must not be reintroduced.

## Static composition + pre-allocated pool

Threads are spawned once at pipeline construction, park between frames, and wake to consume morsels. No `thread::spawn`, `tokio::spawn`, or `async_std::spawn` runs at frame time. Morsel assignment is deterministic by default; consumers that want work stealing implement the `Executor` trait to opt in.

The scheduler schedules itself via meta WorkUnits (`PlanStage → ScheduleReady → PassStart → ScheduleEnd`). Engine invariants are enforced by the same machinery that enforces consumer WorkUnits.

## Plugin-host layer

Three crates form a reusable loader layer for downstream consumers that want to load extensions at arbitrary points in their own lifecycle:

1. **`hilavitkutin-linking`** — dlopen / LoadLibrary wrapper, pull-based explicit symbol lookup, `no_std`, no allocator.
2. **`hilavitkutin-extensions`** — contract-bound orchestration: `ExtensionDescriptor` with `MaybeNull<fn>` at FFI slots, host-opaque context pointer per extension, capability dispatch by stable `CapabilityId`, required-versus-optional failure policy.
3. **`hilavitkutin-extensions-macros`** — the sole proc-macro crate in the stack. Emits the descriptor, exports `__hilavitkutin_extension_descriptor`, and emits capability trampolines via `#[export_extension]`.

Downstream consumers (viola, etc.) layer their own contract crates on top. The layer itself stays domain-agnostic. Any extension must tolerate loading, running, and dropping at arbitrary points independent of its siblings; no "all loaded before any invoked" gate exists.

## Installation

```bash
cargo add hilavitkutin
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
hilavitkutin = "0.1"
```

Consumers that want only the contract surface (to define WorkUnits without linking the engine) add `hilavitkutin-api` instead. Build-time features use `hilavitkutin-build` as a `build-dependencies` entry; it never links into the runtime binary.

## Positioning

`hilavitkutin` depends on [`notko`](https://github.com/orgrinrt/notko) (foundation primitives: `Just` / `Maybe` / `Outcome` / `MaybeNull`) and [`arvo`](https://github.com/orgrinrt/arvo) (numeric and analysis substrate). The engine uses `arvo-graph` for DAG topology, `arvo-bitmask` for access masks, `arvo-sparse` for RCM reordering, `arvo-spectral` for trunk formation, and `arvo-comb` for fiber grouping. arvo types do not leak through `hilavitkutin-api`; consumers that need arvo depend on it directly.

Downstream consumers include [`clause`](https://github.com/orgrinrt/clause) (language runtime: macros, scratch, generative pipelines) and external projects using the plugin-host layer as a dlopen substrate.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/hilavitkutin/blob/dev/LICENSE)
