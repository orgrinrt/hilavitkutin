# Identity: What Hilavitkutin Is

hilavitkutin is a morsel-driven pipeline execution engine. Consumers declare
WorkUnits with typed read and write access sets; the engine analyses those
declarations into phases, trunks, and fibers, compiles per-core dispatch
programs, and runs them on a pre-allocated thread pool. Every crate is
`#![no_std]` with no `alloc`, no runtime spawn, and no dynamic dispatch:
monomorphisation is the dispatch, and the monomorphised function is the type
identity.

The engine is one thing. There is no separate single-core or multi-threaded
engine, run, or dispatch path anywhere (op ruling, 2026-06-06, canon-level
authority). Core count is configuration: the same plan pipeline computes the
single best sequence and parallelised per-core programs it can from all
statically available data, within the configured core count. At one core that
best sequence is serial as a natural degenerate of the same algorithms (every
fiber assigned to core 0 in the optimal order, phase sync points with one
arriver, trivial convergence, a one-worker pool). "Sequential" is a per-phase
strategy the plan may select, never a separate engine, and single-core mode is
never special-cased against multi-core mode in code.

## Scope

hilavitkutin owns DAG scheduling, WorkUnit dispatch, morsel chunking,
pipeline phase structure, progress tracking, and the optional work-stealing
extension point. It does not own column storage backing memory, the data
model or schema, query planning, UI, or IO. Consumers materialise data into
columns, provide platform implementations, and read results directly from
their own storage. DuckDB and the morsel-driven-parallelism literature are
architectural references, never dependencies; every mechanism is our own
implementation.

## Crate structure

Twelve crates, flat structure, five groupings.

**API (consumer-facing contracts).** `hilavitkutin-api` ships `WorkUnit`,
`AccessSet`, `Column<T>`, `Resource<T>`, `Virtual<T>`, `Context`, the
`Contains` family, and the platform contracts (`Clock`, `MemoryProvider`,
`ThreadPool`). `hilavitkutin-api-macros` is its proc-macro companion
(`#[derive(ResourceFootprint)]`, reporting a resource value type's
write-collection footprint for morsel sizing). The api crate never depends on
the engine: the api is the contract, the engine is one implementation of it.

**Engine.** `hilavitkutin` houses plan (phase, fiber, analysis, morsel,
loading), dispatch, resource, thread, strategy, adapt, scheduler, intrinsics,
and platform implementations.

**Build-time.** `hilavitkutin-build` holds LLVM passes, MIR manipulation, cfg
emission, PGO and BOLT workflows, and pragma-based build configuration. It is
a build-dependency only, with no runtime linkage; consumer `build.rs` uses
it, the runtime binary never links it.

**Ecosystem (standalone opt-in).** `hilavitkutin-ctx` (provider-gated context
framework), `hilavitkutin-persistence` (generic hot/cold storage bridge),
`hilavitkutin-str` (interned string system), `hilavitkutin-kit` (the `Kit`
preset trait: `type Units: WorkUnitBundle; type Owned: StoreBundle`), and
`hilavitkutin-providers` (default Kit implementations). The ecosystem crates
stay independently usable and do not depend on the engine.

**Plugin-host infrastructure.** `hilavitkutin-linking` (cross-platform
pull-based explicit-symbol dynamic library loading over dlopen and
LoadLibrary), `hilavitkutin-extensions` (contract-bound host orchestration:
`ExtensionDescriptor`, lifecycle with per-extension host-opaque context,
capability dispatch via stable `CapabilityId`, required-versus-optional
failure policy), and `hilavitkutin-extensions-macros` (the `#[repr(C)]`
descriptor and trampoline emitter). All three stay domain-agnostic; the
framework assumption is that any extension loads, runs, and drops at
arbitrary points independent of siblings, and no ABI surface may assume an
ecosystem-wide "all loaded before any invoked" gate.

The engine consumes the arvo substrate (`arvo`, `arvo-bitmask`, `arvo-graph`,
`arvo-sparse`, `arvo-spectral`, `arvo-comb`) for numeric and analysis
primitives; see [[foundations]]. arvo types are not re-exported through
`hilavitkutin-api`.

## Static composition, adaptive parameters

The governing split (spec resolution R6) that every other chapter leans on:

**Locked at compile time (the types).** The WU set, the dependency DAG, the
topological and RCM row order, the fiber/trunk/phase topology, and the
monomorphised dispatch functions. Never runtime-mutable. This is what makes
LLVM devirtualisation possible: the schedule is statically analysable.

**Recomputed between frames (the parameters).** Morsel sizes, per-phase
configs, core and lane affinity, record ranges, dirty masks. These feed into
the static per-core program as runtime values.

`meta::PlanStage` has two distinct cases. Parameter recompute (a record-count
change recomputes morsel ranges and re-selects configs over the same static
WU set) is the between-frames adaptive path. Structural change (a different
WU set) is a rebuild: a new static composition, a new monomorphisation, never
a runtime mutation of a running engine. No adaptive or replan trigger
anywhere in canon reorders dispatch or regroups fibers from a runtime signal;
this was mirror-verified exhaustively against the spec (r2 section 9).

Extensibility follows the same split. New data (records added to existing
columns) flows through the same static WUs, with record count as an adaptive
parameter: unbounded data extensibility with zero new dispatch. New code (a
runtime-loaded extension) never enters the engine's monomorphised dispatch;
it integrates through a statically registered facade WorkUnit (see
[[scheduler]]).

## Vocabulary

The execution hierarchy, coarsest to finest:

```
pipeline -> core -> phase <-> waist -> trunk -> fiber <-> branch <-> bridge -> morsel -> micro-morsel -> entry
```

| Term | Definition |
|---|---|
| record | One data point in a column. The global identity. A column has N records; a morsel windows into a range of records. Not "entity" (no ECS connotation), not "row" (no tabular connotation). |
| entry | One record times one op. Register level, the innermost unit. |
| micro-morsel | Inner tiling when peak live data exceeds L1. Sub-morsel record range, single-level by default, activates at ECS scale. |
| morsel | 3D window into a fiber's co-located arena: records by columns by temporal. L1-sized; multiple morsels per fiber. |
| fiber | Unit of column co-location and morsel windowing. WUs within a fiber share columns; morsels window into the fiber's arena. Whether its ops fuse into one loop body or run as separate passes is an execution-strategy decision, not a definitional property. |
| branch | Side path attached to a trunk; peer of fiber within a trunk. |
| bridge | Fan-in node reading from multiple trunks; runs after parent trunks reach the required record range. |
| trunk | Column-disjoint maximal path within a phase. Contains fibers, branches, bridges. Trunks share no write columns: zero sync. |
| waist | Narrow bottleneck where the DAG's concurrent path count drops to a minimum. Defines phase boundaries. |
| phase | Wide DAG section between waists. Phases execute with pipeline parallelism. |
| core | Physical CPU core as an execution unit; one pool thread per physical core; trunks are core-pinned. |
| pipeline | The entire DAG. |
| column | Independent typed store of records. No tabular joins. |

Dead terms, never used for engine concepts: `chain` (use fiber),
`chain group` (trunk), `partition` (phase), `archetype` (fiber), `entity` and
`row` (record), `order` as a WU field (WUs declare scheduling hints; the
scheduler derives ordering).
