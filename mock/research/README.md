# hilavitkutin research

Source material backing the hilavitkutin designs. Research notes are not gated by the writing-style lint: files under this directory and its subdirectories are archival and are preserved verbatim from their origins where applicable. The directory is hidden in plain sight, in that the primary landing points stay DESIGN.md, PRINCIPLES.md and the per-crate DESIGN.md.tmpl, and research is consulted when a surface question reopens and the synthesis is not enough.

## What has which standing

`seed/` is the flat, self-contained expression of the effective canon: the founding consolidation spec (`202603181200`, round `202604200055`) with the full registered amendment chain (per addendum A2, `pre_seed/202607193200`) applied once, superseded material left out. While unfrozen it is under construction; when its manifest declares it frozen, all further canon changes happen in the registry (`mock/registry/`) rather than here, and a registry row outranks seed text on conflict.

`pre_seed/` is everything that came before: the amendment memos, roadmaps, audits, expert panels, sketches and notes that produced the seed. Read it as history rather than as canon; the surviving content is in `seed/` and, after the drain, in the registry.

`imported-from-polka-dots/` and `imported-from-saalis/` are verbatim copies of two other workspaces' research surfaces, indexed below, and they are neither seed nor pre-seed: they predate the split and are source material for both.

Everything new lands at this root, exactly as research always has: memos, research rounds and their deliverables, expert panels, third-party reading, prior art, citations, timestamped documents or timestamped subdirectories, and sketches under `sketches/`. What changes is its standing rather than its shape. Nothing here is ever mandatory reading: anything meaningful a piece of research produces lands as registry rows, with the research cited as provenance through `git::commit::<hash>` refs, and the research itself is the paper trail for how those rows came to be. Nothing here is referenced as design authority.

## `hilavitkutin-build-design-audit.md`

Synthesised audit of the full `hilavitkutin-build` design as inherited from polka-dots. Feature inventory, pragma table, profile defaults, PGO/BOLT pipeline, container workflow, gap analysis against the clause-dev seed. Read this first when touching anything build-related.

## `imported-from-polka-dots/`

Verbatim copy of the polka-dots research surface that the hilavitkutin design draws from. Covers the engine (plan, dispatch, thread, morsel, scheduler), the build-time toolchain (LLVM passes, PGO/BOLT, pragmas), and the hilavitkutin-wide 22-domain consolidation round.

Highlights:

- `design_rounds/202603181200_topic.hilavitkutin-design-consolidation.md`, the 22-domain consolidation (engine-wide canonical specification).
- `design_rounds/202603131230_topic.build-configuration-and-cfg-gating.md`, ~2450 lines, Q1-Q8 round that ratified the hilavitkutin-build shape.
- `design_rounds/202603171900_topic.execution-model.md`, execution-model decisions (plan, dispatch, fibre, trunk semantics).
- `design_rounds/202603151200_topic.hilavitkutin-api-surface.md`, api-surface round (hilavitkutin-api contracts lineage).
- `design_rounds/202603141800_topic.hilavitkutin-core-design.md`, pre-consolidation core design round.
- `design_rounds/202603141800_research.*.md`, 16 engine-focused research notes (morsel pipeline, scheduler algorithm, yielding, workunit trait, registration, unified store, columnar-ECS, DAG work-stealing, etc.). Each is a standalone deepdive on its named concern.
- `design_rounds/202603111529_topic.paradigm-and-architecture.md`, foundational paradigm (nightly stance, newtype mandate).
- `design_rounds/202603121310_topic.memory-layout-and-platform.md`, memory layout + platform assumptions.
- `design_rounds/202603121800_topic.dispatch-and-optimisation.md`, dispatch concerns that feed both engine and build-time choices.
- `design_rounds/202603131400_research.*`, `202603131500_research.*`, `202603131600_research.*`, `202603131700_research.*`, LLVM passes / Polly / per-crate backends / post-Polly passes (feed hilavitkutin-build).
- `design_rounds/202603171930_changelist.doc.deprecated.md`, `202603231200_changelist.doc.deprecated.md`, superseded changelists, kept for audit lineage.
- `design_rounds/2026-02-21/`, `202603110821/`, `202603111443/`, earlier architecture-signature and core-design-review rounds.
- `build-configuration-research.md`, `llvm-flags-and-experimental-optimisations.md`, `nightly-and-stable-optimisation-config.md`, `rust-nightly-features-for-type-constraints.md`, `execution-engines-duckdb-blis-polars.md`, top-level research notes.
- `bench/`, calling-convention benchmarks plus runtime benchmark .rs sources (`hilavitkutin_bench.rs`, `dispatch_optimization_test.rs`, `full_schedule_dispatch_test.rs`, `struct_field_devirt_test.rs`, `unchecked_pattern_test.rs`) and the calling-convention CSVs. Binaries and raw assembly dumps were intentionally omitted (large, platform-specific, and reproducible from the .rs sources).

## `imported-from-saalis/`

Verbatim copy of the saalis research surface. Saalis is the workspace where `hilavitkutin-ctx`, `hilavitkutin-persistence`, and `hilavitkutin-str` were originally designed; its research covers persistence strategies, hot/cold tiering, content-addressable storage, sieve eviction, observability tracing, proc-macro patterns, async-sync bridging, error handling, and a large swath of deepdives on adjacent Rust ecosystem patterns.

Highlights:

- `design_rounds/202603161400/`, mesh/crate-taxonomy round (hilavitkutin-arvo integration settling).
- `design_rounds/202603162300/`, implementation roadmap.
- `design_rounds/202603170810/`, primitives design (also ported to arvo/research; relevant to both).
- `design_rounds/202603170905/`, contracts-crate split (hilavitkutin-api lineage).
- `design_rounds/202603241200_topic.mesh-architecture-and-crates.md`, `...-mesh-data-and-operations.md`, mesh-layer architecture decisions.
- `2026-03-14-research.*.md`, `2026-03-14-synthesis.*.md`, foundational research and synthesis documents: appliance-first design, columnar query engine, connector architecture, content-acquisition pipeline, ecosystem contributions, flexible schema strategy, persistence strategy, progressive UX strategy.
- `2026-03-15-deepdive.*.md`, topic-specific deepdives (accessibility, async-sync bridging, axum, backup/restore, batocera, concurrency primitives, content-addressable storage, cross-compilation CI, error-handling, full-text search, game-frontend integration, htmx/SSE, i18n, image handling, inventory/dlopen plugins, limbo/sqlite, maud, min-specialization, observability, proc-macro patterns, retro-gaming metadata, rom-verification, seaorm, serde, sieve eviction, testing, ureq, and more).
- `2026-03-19-research.headless-server-client-architecture.md`, `2026-03-19-research.host-crate-apis-from-unfinished-consolidation.md`, late-stage architecture reviews.
- `2026-03-28-*`, taxonomy review material (reference corpus CSV + run outputs + decomposition reviews).

Some saalis material is about products outside clause-dev's scope (ROM management, retro-gaming metadata, batocera internals). It's preserved because the underlying patterns (sieve eviction, async-sync bridging, proc-macro mechanics) are reusable and worth knowing about; the domain framing is just saalis-specific.

## `imported-from-polka-dots/` and `imported-from-saalis/` conventions

Files are copied unchanged from the source workspaces. Filename timestamps reflect the source round. Never edit these files. If corrections are needed, do them in a new research note under this `mock/research/` root, at top level, referencing the source. Port is snapshot-at-a-time; the authoritative copies continue to live in the source workspaces.

## Additional sources not copied

- `~/Dev/polka-dots/` has runtime executables and assembly dumps (100s of KB per file) from the benchmark suite. Not portable to other machines; the `.rs` sources here can regenerate them.
- `~/Dev/saalis/` has a non-Rust research branch (UX mocks, Figma exports, etc.) that is out of scope for clause-dev.
