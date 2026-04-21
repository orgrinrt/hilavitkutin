# hilavitkutin research

Source material backing the designs in `mock/crates/*/DESIGN.md.tmpl` and `mock/design_rounds/`. Research notes are NOT gated by the writing-style lint (they are the primary sources, preserved verbatim). They are allowed to have their own voice, their own rough edges, and their own archival quirks.

## `hilavitkutin-build-design-audit.md`

Synthesised audit of the full `hilavitkutin-build` design as inherited from polka-dots. Feature inventory, pragma table, profile defaults, PGO/BOLT pipeline, container workflow, gap analysis against the clause-dev seed. Read this first when touching anything build-related.

## `imported-from-polka-dots/`

Primary sources copied verbatim from `~/Dev/polka-dots/`, where the hilavitkutin-build design was originally researched and ratified. Files are unchanged from the source workspace; timestamps in filenames reflect the polka-dots round they belong to.

### `imported-from-polka-dots/*.md` — top-level research notes

- `build-configuration-research.md` — catalogue of build-time optimisation opportunities (PGO, BOLT, LTO, Polly, custom LLVM passes), written before the Q1-Q8 round resolved what to ship.
- `llvm-flags-and-experimental-optimisations.md` — deep survey of rustc / LLVM flag surface, including nightly-only options. Feeds the choice of which pragmas ship and which were dropped.
- `nightly-and-stable-optimisation-config.md` — maps each optimisation to the toolchain channel it requires. Grounds the nightly-by-default stance.
- `rust-nightly-features-for-type-constraints.md` — adjacent research on nightly features the broader stack depends on; relevant to build because the crate pins a nightly toolchain and must know which features are mature enough to require.

### `imported-from-polka-dots/design_rounds/` — ratified rounds

- `202603131230_topic.build-configuration-and-cfg-gating.md` — the authoritative Q1-Q8 round that resolved the hilavitkutin-build shape. Pragma model, requirement combinators, default profile sets, std/no_std stance (Q6), wrapper script generation (Q4), cfg emission (Q3), profile staleness handling (Q7), container workflow (Q8). ~2450 lines.
- `202603131400_research.rustc-llvm-optimization-flags.md` — supporting research feeding into the pragma flag surface.
- `202603131400_research.llvm-plugin-passes.md` — the custom LLVM pass design (IRCE, LoopPredication, SimplifyCFG, LoopInterchange, LoopDistribute, LoopDataPrefetch, SeparateConstOffsetFromGEP, LoopFusion).
- `202603131500_research.per-crate-backends-and-domain-optimizers.md` — dev-backend split (Cranelift for workspace, LLVM for deps) and per-crate pass selection.
- `202603131600_research.polly-llvm-rust-ir.md` — research on Polly's polyhedral optimiser and how to make Rust IR Polly-friendly (the pre-vectorise pass chain exists because of this).
- `202603131700_research.llvm-passes-beyond-polly.md` — survey of LLVM passes that are not Polly but are useful on their own.

### `imported-from-polka-dots/bench/` — calling-convention benchmarks

- `calling-convention-bench-results.md` — written summary of the calling-convention v4 run series (three runs plus a combined view).
- `202603261_calling-convention-v4-run{1,2,3}.csv` — raw benchmark data from three independent runs.
- `202603261_calling-convention-v4-combined.csv` — merged view.

Calling-convention work is not directly about `hilavitkutin-build`, but the crate's Cranelift-versus-LLVM backend split (dev-opt and release profiles) leans on the conclusions here — which pass selection and backend choice actually move the needle, and how much.

## Additional sources referenced but not copied

The polka-dots workspace has much more research (morsel pipeline, scheduler algorithm, unified store, yielding, workunit trait, etc.) that is relevant to other hilavitkutin crates (engine, api, ctx, persistence, str), not to `hilavitkutin-build` directly. Those will be ported when the matching consumer crate receives its audit/refresh round. Do not delete them from polka-dots in the meantime — the authoritative copy lives there until each audit ports it.

The large 2026-03-18 consolidation round (`202603181200_topic.hilavitkutin-design-consolidation.md`, 22 domains) is hilavitkutin-wide; it will be ported alongside the engine audit rather than here.
