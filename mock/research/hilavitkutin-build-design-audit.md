# hilavitkutin-build — design audit against the polka-dots origin

**Date:** 2026-04-21
**Source workspace:** `~/Dev/polka-dots/` (authoritative). `~/Dev/saalis/` carries integration references only — no core design.
**Purpose:** Reconstitute the full designed surface of `hilavitkutin-build` inside clause-dev. The clause-dev seed inherited `DESIGN.md.tmpl` intact but the staged source skeleton hides how much of the vision is still deferred. This file names every designed capability, the research behind it, and the gaps between "designed" and "seeded here".

## 1. What hilavitkutin-build is

A build-dep-only crate whose single `configure().run()` call, invoked from a consumer's `build.rs`, drives every piece of build-time optimisation the hilavitkutin stack relies on. It generates a rustc workspace wrapper, a cargo profile config file, and one `cargo:rustc-cfg` emission for fast-math. Runtime binary never links the crate.

It is **not** no_std. Build-script crates always link `std`; hilavitkutin-build uses `std::env`, `std::fs`, `std::path`, `std::thread::available_parallelism()` in `bootstrap` and friends. Its pragma / profile / axis types are already no_std-safe (no heap, no `std::` imports); promoting them into a sub-module that advertises `#![no_std]` to external consumers is a BACKLOG item, not a current requirement.

## 2. Feature inventory

### 2.1 LLVM pass plugin (`polka-passes.so`)

Injected via `-Z llvm-plugins`, registers passes at two extension points. Pre-vectorize pipeline makes Rust IR Polly-friendly by first stripping bounds checks and panic branches; post-vectorize pipeline tunes cache behaviour.

**VectorizerStartEP:**

1. IRCE — Induction Range Check Elimination (drops bounds checks in loops)
2. LoopPredication — Guard hoisting (loop-invariant condition checks moved outside)
3. SimplifyCFG — Cleans up branches created by IRCE/Predication
4. LoopInterchange — Fixes nested loop order for sequential memory access
5. LoopDistribute — Breaks dependent loops to enable partial vectorization

**OptimizerLastEP:**

6. LoopDataPrefetch — Prefetch hints for vectorized access patterns
7. SeparateConstOffsetFromGEP — CSE of column base pointers
8. LoopFusion (conditional on LoopFusion pragma) — fuse adjacent loops

A separate `math-peephole.so` plugin carries the FastMath-gated float rewrites.

### 2.2 The 13 built-in pragmas

Pragmas are ZST types implementing a `Pragma` trait. Dependency enforcement uses trait combinators (`All<(A, B)>`, `Any<(A, B)>`); violations surface at `configure().run()` time.

| Pragma | Mechanism | Notes |
|--------|-----------|-------|
| LoopOptimization | rustc wrapper loads `polka-passes.so` | Core loop work |
| Polly | rustc wrapper | Polyhedral (tiling, fusion, GEMM). Polly-enabled LLVM required |
| MathPeephole | rustc wrapper | Requires FastMath |
| FastMath | rustc wrapper + `arvo_fast_math` cfg | unsafe-fp-math flag |
| ExpandedLto | generated config | fat LTO + codegen-units=1. Required for devirt (12.6x penalty otherwise) |
| Pgo | generated config | Consume profiles when present |
| Bolt | post-build hook | Linux ELF only; macOS falls back to `-split-machine-functions` |
| Profiling | post-build hook | Requires Any<(Pgo, Bolt)>. Runs benchmarks post-build |
| BuildStd | generated config | Rebuilds std with optimisation flags; LTO across std boundary. Not a default (build-time cost) |
| ParallelCodegen | rustc wrapper | `-Zthreads=N`. 0 = auto-detect |
| SharedGenerics | rustc wrapper | `-Zshare-generics=y`. Reduces debug link time |
| LoopFusion | rustc wrapper | EXPERIMENTAL. Not in any default set |
| MimallocAllocator | advisory only | No runtime code; recommends the consumer set `#[global_allocator]` |

### 2.3 Default pragma sets per profile

| Profile | Backend | Pragmas |
|---------|---------|---------|
| dev | Cranelift (workspace), LLVM (deps) | ParallelCodegen, SharedGenerics |
| dev-opt | LLVM | ParallelCodegen, SharedGenerics, LoopOptimization |
| release | LLVM | ParallelCodegen, LoopOptimization, MathPeephole, FastMath, ExpandedLto, Bolt, Pgo |
| profiling | LLVM | ParallelCodegen, LoopOptimization, MathPeephole, FastMath, Profiling, Pgo, Bolt |
| ci | LLVM | Same as release but thin LTO |

Never default: Polly (special LLVM build), BuildStd (build time), LoopFusion (experimental).

### 2.4 Four-part build configuration model

1. **`hilavitkutin-build` crate** — reads cargo env vars, emits `cargo:rustc-cfg` lines, writes the wrapper script and generated config. Runs in every build.rs.
2. **No custom cargo features** — ISA capabilities via `target_feature`; vendor via `CARGO_CFG_TARGET_VENDOR`; arch via `CARGO_CFG_TARGET_ARCH`. Cache and topology live in runtime code.
3. **`RUSTC_WORKSPACE_WRAPPER`** — shell script (can grow to hundreds of lines) that wraps every rustc invocation. Inspects three axes — Target (arch/vendor/features), Tier (opt-level), Passes (which LLVM passes apply) — and emits exactly the optimal flag set.
4. **Generated config file** — `target/hilavitkutin-build/hilavitkutin-config.toml` (gitignored) written by pragmas that require Cargo profile settings (ExpandedLto, Pgo, BuildStd).

### 2.5 PGO + BOLT tiered auto-optimisation

Pipeline runs entirely in the background. The consumer sets pragmas; tiers light up automatically as data accumulates.

**Release build 1 (no profiles yet):**

1. Cargo builds; binary produced.
2. Post-build hook: static BOLT immediately (seconds, Linux ELF only, static heuristics reorder blocks/functions for 3-5% code-layout gain).
3. Background: benchmarks run with PGO instrumentation and `perf record -b`, producing `.profraw` + `perf.data`. Profiles merged.

**Release build 2+ (profiles exist):**

1. `configure()` finds `merged.profdata`, adds `-C profile-use=...` and PGO-complementary flags.
2. Cargo recompiles with PGO (10-20% branch/inline gain).
3. Post-build hook: profile-guided BOLT (additional 5-10%).
4. Background: re-profiles with the now-optimised binary.

**Additive optimisation tiers (all automatic):**

| Tier | What | Gain |
|------|------|------|
| Stock | Nothing extra | Baseline |
| Plugins | LLVM pass plugins via wrapper | Loop opt, prefetch |
| Static BOLT | Post-build reorder (Linux ELF) | 3-5% layout |
| PGO | Profile-use from benchmarks | 10-20% branch/inline |
| Profile-guided BOLT | BOLT with profiling data | Additional 5-10% |
| Machine function split | Compile-time flag (all targets) | Partial BOLT for macOS |

### 2.6 Profile staleness handling

Profiles live in `target/hilavitkutin-build/pgo/` (gitignored). `cargo clean` deletes them. `configure()` records the git HEAD commit hash when profiles were generated; if HEAD has diverged more than 50 commits, emits a `cargo:warning=stale profiles` message. Never refuses to build; stale profiles degrade gracefully.

### 2.7 Container workflow (macOS release builds)

`-Z llvm-plugins` (dynamic LLVM) is Linux-nightly only. Dev builds on macOS use `-C passes=` and `-C llvm-args=` (built-in passes). Release builds on macOS run inside Apple Container (Apache 2.0, Virtualization.framework, boots in under a second), hosting a minimal Debian with rustup nightly. Native aarch64-linux on Apple Silicon — no VM overhead, no Rosetta.

### 2.8 cfg flags emitted

Only one custom cfg: `arvo_fast_math`, emitted when the FastMath pragma is active. arvo gates fast-math float semantics on this cfg. All ISA detection uses native cargo cfgs (`target_feature`, `CARGO_CFG_TARGET_FEATURE`); no bespoke hardware-capability flags.

### 2.9 Cargo files in a consumer

| File | Committed | Role |
|------|-----------|------|
| `rust-toolchain.toml` | yes | Nightly channel + components |
| `Cargo.toml` | yes | Profiles, features, workspace lints |
| `.cargo/config.toml` | yes | `[build] rustc-workspace-wrapper = "target/hilavitkutin-build/rustc-wrapper"` |
| `target/hilavitkutin-build/hilavitkutin-config.toml` | no (generated) | ExpandedLto, Pgo, BuildStd |

### 2.10 Consumer API

```rust
fn main() {
    hilavitkutin_build::configure()
        .profile("release", |p| p.enable::<(
            ExpandedLto, FastMath, LoopOptimization, Pgo, Bolt,
        )>())
        .run();
}
```

One call. Generates the wrapper script, writes the config file, emits cfgs. Auto-integration pattern mirroring mockspace's `bootstrap_from_buildscript()`.

## 3. Benchmarks and research

Built-in benchmarks ship with hilavitkutin-build and exercise hot paths of the polka framework (morsel iteration, column access, scheduler dispatch, pipeline stage execution). They read the consumer's `polka.toml` with fixed synthetic input. Run by default on release builds; additive with consumer benchmarks because more coverage gives better profiles.

Concrete gain figures that drove the design:

- Static BOLT: 3-5% code layout improvement
- PGO: 10-20% branch/inline improvement
- Profile-guided BOLT: additional 5-10%
- Staleness threshold: warning fires at >50 commits of divergence

Research documents in polka-dots (referenced in the Q2/Q3 resolutions of the original round):

- `mock/research/llvm-flags-and-experimental-optimisations.md`
- `mock/research/nightly-and-stable-optimisation-config.md`
- `mock/research/rust-nightly-features-for-type-constraints.md`
- `mock/design_rounds/202603131400_research.rustc-llvm-optimization-flags.md`
- `mock/design_rounds/202603131400_research.llvm-plugin-passes.md`
- `mock/design_rounds/202603131600_research.polly-llvm-rust-ir.md`
- `mock/design_rounds/202603131700_research.llvm-passes-beyond-polly.md`
- `mock/research/build-configuration-research.md`
- `mock/research/calling-convention-bench-results.md`

Key decision D2 from the original round: "no unilateral decisions on what to pursue." Polly, PGO, BOLT, AutoFDO were catalogued; adoption required explicit human choice. AutoFDO was dropped because PGO is strictly better when benchmarks are under our control. The same discipline applies when re-implementing capabilities in clause-dev: list, then let the user pick.

## 4. Relationships and contracts

- **Consumers:** every hilavitkutin crate's build.rs (api, build, ctx, persistence, str) plus external projects (polka-dots, saalis, loimu).
- **arvo integration:** emits `arvo_fast_math` cfg for FastMath; arvo itself handles target_feature detection inside its own build.rs.
- **notko integration:** no direct dependency in the current design. The `#[optimize_for]` macro in `notko-macros` and the cross-crate optimiser accumulation in `notko-build` (task #99) are separate concerns; a future round might connect them.
- **Runtime linkage:** none. Build-dep only.
- **Upstream deps:** nothing. Standalone.

## 5. Primary source files in polka-dots

- `mock/design_rounds/202603131230_topic.build-configuration-and-cfg-gating.md` — Q1-Q8, ~2450 lines, resolves all major scoping decisions
- `docs/HILAVITKUTIN_BUILD_OVERVIEW.md` — generated overview
- `mock/crates/hilavitkutin-build/DESIGN.md.tmpl` — source template (identical to the one in clause-dev)
- `mock/design_rounds/202603181200_topic.hilavitkutin-design-consolidation.md` — cross-domain context (22 domains)

Dependencies recorded in 202603131230:
- 202603111529_topic.paradigm-and-architecture.md (nightly stance, newtype mandate)
- 202603121800_topic.dispatch-and-optimisation.md (morsel sizing, core-type dispatch, hardware detection)

## 6. Gaps in the clause-dev seed

What the current `~/Dev/clause-dev/hilavitkutin/mock/crates/hilavitkutin-build/` contains:

- `DESIGN.md.tmpl` — complete, matches polka-dots
- Source skeleton — `lib.rs`, `pragma.rs`, `profile.rs`, `axis.rs`, `bootstrap.rs`, `config.rs`, `requirements.rs`, `guards.rs`
- Pragma enum + PragmaSet (13 variants, u16 bitmask)

What is designed but intentionally not yet implemented (tracked in BACKLOG per mockspace discipline):

- Rustc workspace wrapper script generation (bash and PowerShell emitter)
- LLVM pass plugin C++ sources (`polka-passes.cpp`, `math-peephole.cpp`) and pre-built dylib distribution
- PGO/BOLT tier integration (post-build hook, profile staleness tracking, benchmark running)
- `hilavitkutin-config.toml` schema + reader
- Per-crate consumer build.rs migration

`BACKLOG.md.tmpl` lists these with rationale. The DESIGN is not lost; the implementation is staged.

## 7. Known mockspace.toml drift

The current `mock/mockspace.toml` `forbidden-imports` rule blocks `std::*` for all `hilavitkutin*` crates. This is wrong for `hilavitkutin-build`: the design says it drops `#![no_std]` and uses `std` in `bootstrap`, `config`, and `pragma`. The rule should carve `hilavitkutin-build` out (or grant build-only crates a std amnesty). See `mock/design_rounds/202603131230_topic.build-configuration-and-cfg-gating.md` Q6.
