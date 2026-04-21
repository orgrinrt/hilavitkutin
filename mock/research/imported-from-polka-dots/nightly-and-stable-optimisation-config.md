# Optimisation Configuration: Nightly Features & Stable Tuning

**Date:** 2026-03-13
**Purpose:** Comprehensive reference for all compiler, cargo, and
linker optimisation options available to us — both nightly-only
unstable features and stable flags/settings that should be enabled
regardless. Covers build speed, runtime performance, binary size,
diagnostics, and validation.

**Scope:** polka-dots (and shared foundations with saalis / ECS engine).
macOS aarch64 (Apple Silicon) is the primary dev target; Linux x86_64
is the deployment/CI target.

---

## Table of Contents

1. [Nightly Feature Gates](#1-nightly-feature-gates)
2. [Nightly Compiler Flags](#2-nightly-compiler-flags)
3. [Nightly Cargo Features](#3-nightly-cargo-features)
4. [Cranelift Backend](#4-cranelift-backend)
5. [Stable Codegen Options](#5-stable-codegen-options)
6. [Stable Cargo Profiles](#6-stable-cargo-profiles)
7. [Linker Configuration](#7-linker-configuration)
8. [Stable Diagnostics & Lints](#8-stable-diagnostics--lints)
9. [Target-Specific Tuning](#9-target-specific-tuning)
10. [Build Caching & CI](#10-build-caching--ci)
11. [Profile-Guided Optimisation (PGO)](#11-profile-guided-optimisation-pgo)
12. [Recommended Configurations](#12-recommended-configurations)

---

## 1. Nightly Feature Gates

These require `#![feature(...)]` in crate roots. See the companion
document `rust-nightly-features-for-type-constraints.md` for full
rationale, ICE analysis, and adoption rules.

### 1.1 Enable in every crate

| Feature | Gate | Purpose | Risk | Legwork |
|---------|------|---------|------|---------|
| `min_specialization` | `#![feature(min_specialization)]` | Blanket impls with concrete overrides. Layered trait defaults. | Very low — stdlib uses it | None |
| `min_generic_const_args` | `#![feature(min_generic_const_args)]` | Const exprs in generic positions. `Check<{ size_of::<Self>() <= 15 }>: IsTrue` | Medium — 11/18 tasks done | Low — our patterns are simple |
| `adt_const_params` | `#![feature(adt_const_params)]` | Struct/enum types as const generics. `ColumnLayout` as const param. | Medium — ICEs in combos | Medium — follow avoidance rules |
| `const_trait_impl` | `#![feature(const_trait_impl)]` | `const trait Foo`, `~const` bounds. Compile-time trait methods. | Medium-high — just rewritten | Low — syntax may change |

**`adt_const_params` avoidance rules** (must follow to dodge ICEs):
- Only use types deriving `ConstParamTy` with integer/bool/char fields
- No raw pointers or function pointers as const params
- No dependent const param types (`const A: Foo<N>`)
- No combining with `generic_const_exprs`, `unsized_const_params`,
  or `generic_const_parameter_types`
- No async fn in const-param-bearing contexts
- No `-Zmir-opt-level=5`

### 1.2 Enable where useful

| Feature | Gate | Purpose | Where |
|---------|------|---------|-------|
| `explicit_tail_calls` | `#![feature(explicit_tail_calls)]` | `become func()` guaranteed TCE | Recursive AST traversal in DSL |
| `loop_match` | `#![feature(loop_match)]` | State machine → jump table dispatch | Fused chain inner loops, pipeline stages |
| `never_type` | `#![feature(never_type)]` | `!` type, `Result<T, !>` for infallible | Error handling in infallible pipelines |
| `macro_metavar_expr` | `#![feature(macro_metavar_expr)]` | `${count()}`, `${index()}` in `macro_rules!` | DSL macros in polka-lang-polka |

---

## 2. Nightly Compiler Flags

These are `-Z` flags passed via `RUSTFLAGS` or `.cargo/config.toml`.
No code changes required.

### 2.1 Always on

| Flag | Purpose | Risk | Fallback |
|------|---------|------|----------|
| `-Zthreads=8` | Parallel rustc front end — parallelises parsing, name resolution, macro expansion, type checking | None | `-Zthreads=1` |

### 2.2 Always on (cargo-level)

| Config | Purpose | Risk | Fallback |
|--------|---------|------|----------|
| `build-std = ["core", "alloc", "std"]` | Rebuild std from source. LTO across std boundary, target-CPU features in std, dead code elimination. | Low (longer first build) | Remove from config |

### 2.3 Always on (dev builds)

| Flag | Purpose | Risk | Fallback |
|------|---------|------|----------|
| `-Zshare-generics=y` | Share monomorphised generics across crates instead of duplicating. Crate A and crate B both instantiate `Vec<String>` → only one copy is generated. | Very low | Remove flag |

This is **particularly impactful for our architecture** — we have
extensive trait generics (`Column<In<IR>, As<T>>`, `WorkUnit`,
`ColumnSlices`) that get monomorphised in every crate that uses them.
Without `share-generics`, each of our 17 crates independently
generates the same generic instantiations. With it, they share.

Bevy uses this (their config template enables it for all platforms).
On Windows, can hit the 65k symbol limit — not relevant for us
(macOS + Linux targets).

Note: this flag is **on by default for dev profiles on some
platforms** but not all. Explicitly setting it ensures consistency.

### 2.4 Dev profile only (Cranelift)

See §4 for full Cranelift configuration. The flag form:

| Flag | Purpose | Risk | Fallback |
|------|---------|------|----------|
| `-Zcodegen-backend=cranelift` | ~30% faster debug compilation, worse runtime perf | Low | Use LLVM (default) |

Requires `rustc-codegen-cranelift-preview` component. The Cargo.toml
profile form (§4) is preferred over the flag form.

### 2.5 CI / testing only

| Flag | Purpose | When |
|------|---------|------|
| `-Zsanitizer=address` | AddressSanitizer — detects use-after-free, buffer overflows, stack overflows | Test unsafe column storage code |
| `-Zsanitizer=thread` | ThreadSanitizer — detects data races | Test morsel dispatch concurrency |
| `-Znext-solver` | Test compatibility with the next-gen trait solver | Periodic CI check |
| `-Zpolonius` | Test with next-gen borrow checker | Periodic CI check |

---

## 3. Nightly Cargo Features

### 3.1 Public/private dependencies

```toml
# Cargo.toml — nightly cargo only
[dependencies]
polka-sdk = { path = "crates/polka-sdk", public = true }
toml = { version = "0.8", public = false }
```

Prevents leaking private dependency types into the public API.
Cargo warns when a public API exposes types from private deps.

### 3.2 cargo-semver-checks

Not nightly-specific but pairs well with the pub/priv dependency
model. Install separately:

```sh
cargo install cargo-semver-checks
cargo semver-checks  # before publishing SDK crates
```

---

## 4. Cranelift Backend

Cranelift is an alternative code generator to LLVM. It produces
lower-quality machine code but compiles significantly faster. The
trade-off is clear: **dev builds compile faster, run slower.**

### 4.1 How it works

LLVM (the default backend) runs ~100 optimisation passes. Cranelift
runs a simpler pipeline — it lowers IR to machine code with basic
register allocation and instruction selection, skipping expensive
optimisations. The result:

| Metric | LLVM (opt-level=0) | Cranelift | Difference |
|--------|-------------------|-----------|------------|
| Compile time | Baseline | ~30% faster | Significant for large projects |
| Runtime perf | Baseline (unoptimised) | ~10-30% slower | Noticeable in benchmarks, irrelevant for most dev testing |
| Debug info | Full support | Full support | No difference |

### 4.2 Configuration (Cargo.toml — preferred)

The Cargo.toml profile form gives finer control than the `-Z` flag:

```toml
# Cargo.toml (workspace root)

# Requires nightly
[unstable]
codegen-backend = true

# Use Cranelift for our code in dev builds
[profile.dev]
codegen-backend = "cranelift"

# Keep LLVM for dependencies — they benefit from optimisation
# and their compile time is cached anyway
[profile.dev.package."*"]
codegen-backend = "llvm"
opt-level = 2
```

This is the Bevy pattern: your code (which changes frequently) gets
fast Cranelift compilation, while dependencies (which change rarely
and benefit from optimisation) stick with LLVM. The deps are cached
by cargo, so their LLVM compile cost is paid once.

### 4.3 Installation

```sh
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

### 4.4 Known limitations

- **macOS:** Bevy explicitly warns that macOS builds can crash with
  Cranelift. This is Cranelift's AArch64 backend being less mature
  than x86_64. Test on your actual hardware.
- **No optimisation:** `opt-level` is ignored when using Cranelift —
  it always produces unoptimised code. This is by design.
- **Some intrinsics missing:** Rare SIMD intrinsics may not be
  implemented. Unlikely to affect us (our SIMD is via
  autovectorisation, not intrinsics).
- **Incompatible with LTO:** Cannot use `lto = "thin"` or `"fat"`
  with Cranelift. Not relevant since Cranelift is dev-only.

### 4.5 When to use vs not

| Scenario | Use Cranelift? | Why |
|----------|---------------|-----|
| `cargo check` | No | Check doesn't codegen — Cranelift irrelevant |
| `cargo test` | **Yes** | Tests need compilation but rarely need runtime perf |
| `cargo run` (dev iteration) | **Yes** | Faster compile loop, acceptable runtime |
| `cargo bench` | **No** | Benchmarks need LLVM optimisation |
| `cargo build --release` | **No** | Release needs maximum codegen quality |
| Profiling | **No** | Cranelift codegen doesn't represent release perf |

### 4.6 Interaction with other optimisations

- **`-Zshare-generics=y`:** Works with Cranelift. Complementary —
  share-generics reduces duplicate work, Cranelift makes the remaining
  work faster.
- **`-Zthreads=8`:** Works with Cranelift. Complementary.
- **`-Zbuild-std`:** Works with Cranelift but rebuilds std with
  Cranelift too — std code will be slower. Consider the
  `[profile.dev.package."*"] codegen-backend = "llvm"` pattern
  to keep std on LLVM (std counts as a non-workspace dependency).
- **Sanitisers:** Not compatible with Cranelift. Use LLVM for
  sanitiser runs.

---

## 5. Stable Codegen Options

These work on any Rust toolchain. Configure via `-C` flags in
`RUSTFLAGS` or `[build] rustflags` in `.cargo/config.toml`, or
via `Cargo.toml` profile settings.

### 5.1 Optimisation level

| Value | Purpose | When |
|-------|---------|------|
| `0` | No optimisation | Dev default — fastest compile |
| `1` | Basic optimisation | Rarely used |
| `2` | Most optimisations | Good balance |
| `3` | All optimisations + aggressive inlining | Release default |
| `"s"` | Optimise for binary size | Embedded, WASM |
| `"z"` | Size + disable loop vectorisation | Smallest binary |

### 5.2 Link-time optimisation (LTO)

| Value | Purpose | Compile cost | Perf gain |
|-------|---------|-------------|-----------|
| `false` | Thin-local (within each crate) | Baseline | Baseline |
| `"thin"` | Cross-crate thin LTO | Moderate | Good (nearly as good as fat) |
| `"fat"` / `true` | Full cross-crate LTO | High | Best |
| `"off"` | No LTO at all | Fastest | Worst |

**Recommendation:** `"thin"` for release. The compile cost of `"fat"`
is rarely worth the marginal gain over `"thin"`.

### 5.3 Codegen units

Higher = faster compile, worse optimisation (less cross-function
inlining). Dev default: 256. Release default: 16.

**Set to `1` for maximum release performance.** Pairs with LTO —
with `lto = "thin"` and `codegen-units = 1`, LLVM has maximum
visibility for inlining and dead code elimination.

### 5.4 Panic strategy

| Value | Binary size | Perf | Trade-off |
|-------|------------|------|-----------|
| `"unwind"` | Larger (unwind tables) | Baseline | `catch_unwind` works |
| `"abort"` | Smaller | Better (no unwind overhead) | No `catch_unwind`, no backtraces on panic |

**Recommendation:** `"abort"` for release binaries. Tests always use
`"unwind"` regardless of profile setting.

### 5.5 Strip

| Value | Effect |
|-------|--------|
| `"none"` | Keep everything (default) |
| `"debuginfo"` | Strip debug info, keep symbols |
| `"symbols"` | Strip everything — smallest binary |

### 5.6 Debug info

| Value | Info level | Use case |
|-------|-----------|----------|
| `0` / `false` | None | Release distribution |
| `"line-tables-only"` | Line numbers only | Profiling (perf, instruments) |
| `1` / `"limited"` | Types + lines | Crash debugging |
| `2` / `true` | Full | Dev debugging |

**Recommendation:** `"line-tables-only"` for release builds you want
to profile. The overhead is small (~5% binary size) and it enables
meaningful flamegraphs.

### 5.7 Target CPU

| Value | Effect |
|-------|--------|
| `"native"` | Detect host CPU, use all its features |
| `"generic"` | Conservative baseline |
| Specific name | e.g., `"apple-m1"`, `"skylake"`, `"znver3"` |

**`target-cpu=native` is free performance for non-distributed
binaries.** On x86_64 the gap is large (SSE2 baseline → AVX2+FMA);
on aarch64-apple-darwin the gap is small (baseline already includes
most M1 features).

**Warning:** The standard library is NOT recompiled with your target
features (unless using `-Zbuild-std`). With `build-std`, this warning
disappears — std is rebuilt with the same features as your code.

### 5.8 Commonly overlooked stable flags

| Flag | Purpose | Notes |
|------|---------|-------|
| `-C overflow-checks=yes` | Runtime integer overflow detection | On in dev, off in release. Consider enabling in release for safety-critical code. |
| `-C symbol-mangling-version=v0` | New deterministic symbol mangling | Cleaner symbols for profiling and debugging. No perf impact. |
| `-C force-frame-pointers=yes` | Preserve frame pointers | Enables profiling with perf/instruments. ~1-2% perf cost. Enable for profiling builds. |
| `-C profile-generate` / `-C profile-use` | PGO (Profile-Guided Optimisation) | Stable. 10-20% gains. See §11. |

---

## 6. Stable Cargo Profiles

### 6.1 Complete profile key reference

| Key | Dev default | Release default | Notes |
|-----|-------------|-----------------|-------|
| `opt-level` | `0` | `3` | `0`,`1`,`2`,`3`,`"s"`,`"z"` |
| `debug` | `true` (full) | `false` | `0`/`false`, `"line-tables-only"`, `1`/`"limited"`, `2`/`true` |
| `split-debuginfo` | platform | platform | `"off"`, `"packed"`, `"unpacked"` |
| `strip` | `"none"` | `"none"` | `"none"`, `"debuginfo"`, `"symbols"` |
| `debug-assertions` | `true` | `false` | Enables `cfg(debug_assertions)` |
| `overflow-checks` | `true` | `false` | Runtime integer overflow detection |
| `lto` | `false` | `false` | `true`/`"fat"`, `"thin"`, `false`, `"off"` |
| `panic` | `"unwind"` | `"unwind"` | `"unwind"`, `"abort"` |
| `incremental` | `true` | `false` | Workspace members + path deps only |
| `codegen-units` | `256` | `16` | Integer > 0 |
| `rpath` | `false` | `false` | Set rpath in binary |

### 6.2 Profile inheritance

- `test` inherits from `dev`
- `bench` inherits from `release`
- Custom profiles use `inherits = "..."`:

```toml
[profile.release-lto]
inherits = "release"
lto = "thin"
codegen-units = 1
strip = "symbols"
panic = "abort"
```

Use with: `cargo build --profile release-lto`

### 6.3 Per-package optimisation overrides

The single most impactful stable optimisation for dev build speed:

```toml
# Optimise ALL non-workspace deps even in dev builds
[profile.dev.package."*"]
opt-level = 2

# Or optimise specific heavy crates
[profile.dev.package.regex]
opt-level = 3
[profile.dev.package.serde_json]
opt-level = 3
```

Your code stays at `opt-level = 0` (fast compile, full debug info)
while dependencies are optimised (fast runtime). This is particularly
impactful when tests exercise code paths through heavy deps.

**Restriction:** `panic`, `lto`, and `rpath` cannot be overridden
per-package.

### 6.4 Build script / proc macro override

```toml
[profile.release.build-override]
opt-level = 0
codegen-units = 256
```

Build scripts and proc macros use this profile instead of the main
one. Defaults to unoptimised even in release — since they only run
at build time, optimising them wastes compile time.

---

## 7. Linker Configuration

### 7.1 Linker comparison

| Linker | Speed (vs GNU ld) | Platform | Cross-compile? | LTO? | Install | Maturity |
|--------|-------------------|----------|----------------|------|---------|----------|
| **GNU ld** (bfd) | 1x (baseline) | Linux | No | Yes | Built-in | Decades |
| **GNU gold** | 2-3x | Linux (ELF only) | No | Yes | Built-in | Mature |
| **LLD** | 5-8x | Linux, macOS, Windows, WASM | No | Yes | `brew install llvm` / `apt install lld` | Production (LLVM project) |
| **mold** | 15-20x | Linux (best), macOS (sold fork, less mature) | No | Yes | `brew install mold` / `apt install mold` | Production (v2.x) |
| **Apple ld** (ld64) | ~5-8x (macOS only) | macOS | No | Yes | Built-in (Xcode) | Decades |
| **Zig** (via cargo-zigbuild) | ~5-8x (uses LLD internally) | Linux + macOS targets | **Yes — turnkey** | Yes (with workarounds) | `brew install zig && cargo install cargo-zigbuild` | Viable but pre-1.0 |

Real-world link times (from mold benchmarks, 16c/32t Linux):

| Binary | GNU ld | LLD | mold |
|--------|--------|-----|------|
| MySQL 8.3 (0.47 GiB) | 10.84s | 1.64s | 0.46s |
| Clang 19 (1.56 GiB) | 42.07s | 5.20s | 1.35s |
| Chromium 124 (1.35 GiB) | N/A | 6.10s | 1.52s |

### 7.2 Platform recommendations

**macOS (aarch64-apple-darwin):**

Bevy's finding (confirmed by our research): **the default Apple ld64
linker is faster than LLD on macOS.** Newer Xcode versions have
significantly improved ld64. The sold (macOS mold fork) is
unmaintained and about the same speed as ld64.

**Recommendation: use the default Apple linker.** No configuration
needed.

```toml
# .cargo/config.toml — macOS: no linker override needed
[target.aarch64-apple-darwin]
rustflags = [
    "-Zthreads=8",
    "-Zshare-generics=y",
    "-C", "target-cpu=native",
]
```

If you want to test lld anyway (some projects see gains):
```toml
# Optional — test and compare
rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/opt/llvm/bin/ld64.lld"]
```

**Linux (x86_64-unknown-linux-gnu):**

LLD is now the default Rust linker on Linux (no config needed). For
maximum link speed, use mold:

```toml
# .cargo/config.toml — Linux: mold for fastest linking
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = [
    "-Zthreads=8",
    "-Zshare-generics=y",
    "-C", "target-cpu=x86-64-v3",
    "-C", "link-arg=-fuse-ld=mold",
]
```

### 7.3 Zig as linker — the cross-compilation play

**What zig is:** A complete C/C++ toolchain based on Clang/LLVM that
bundles headers and implementations for ~97 libc variants. When used
as a Rust linker, it provides turnkey cross-compilation without
Docker, sysroots, or external toolchains.

**What zig is NOT:** A speed improvement over LLD for same-platform
builds. It uses LLD internally but adds ~75ms overhead per
invocation from the wrapper layer.

#### When to use zig

| Scenario | Use zig? | Why |
|----------|---------|-----|
| Local dev (same platform) | **No** | Overhead without benefit. Default linker or mold is faster. |
| Local dev (cross-compile) | **Yes** | `cargo zigbuild --target x86_64-unknown-linux-gnu` from macOS — no Docker needed |
| CI (building for same platform) | **No** | Use mold (Linux) or default (macOS) |
| CI (building Linux from macOS) | **Yes** | Avoids cross-toolchain setup entirely |
| Release builds | **No** | Use the platform-native linker + LTO for maximum optimisation |
| Targeting old glibc | **Yes** | `--target x86_64-unknown-linux-gnu.2.17` — no other tool does this easily |

#### Setup

```sh
brew install zig
cargo install cargo-zigbuild
```

Usage:
```sh
# Cross-compile from macOS to Linux
cargo zigbuild --target x86_64-unknown-linux-gnu

# Target specific glibc version
cargo zigbuild --target x86_64-unknown-linux-gnu.2.17

# Release cross-build
cargo zigbuild --release --target x86_64-unknown-linux-gnu
```

#### Limitations

- ~75ms overhead per compiler invocation (wrapper layer)
- Some `-sys` crates need `CC`/`CXX` pointed at `zig cc`
- Static glibc linking not supported
- Zig is pre-1.0 — breaking changes between versions
- LTO: standard `lto = "thin"` / `"fat"` works. Cross-language LTO
  (`-Clinker-plugin-lto`) had issues but is fixed in cargo-zigbuild
  v0.22+.
- macOS-to-macOS cross (Intel↔ARM): needs `SDKROOT` set

#### Our recommendation

**Don't use zig for local development.** The default Apple linker on
macOS and mold on Linux are faster for same-platform builds.

**Do use zig for cross-compilation in CI** if we need to build Linux
binaries from macOS CI runners. The alternative is Docker or a full
cross-compilation sysroot, both of which are heavier.

### 7.4 LTO interaction with linkers

| LTO type | All linkers | Notes |
|----------|-------------|-------|
| `lto = false` (thin-local) | Works everywhere | Default |
| `lto = "thin"` | Works everywhere | Recommended for release |
| `lto = "fat"` | Works everywhere | Maximum optimisation, slow compile |
| Cross-language LTO (`-Clinker-plugin-lto`) | LLD or zig only | Rust + C compiled with clang. Advanced. |

### 7.5 Debug vs release linker strategy

There's no reason to use different linkers for debug vs release on
the same platform. The linker doesn't optimise code — it resolves
symbols and lays out sections. The same linker works for both.

The one exception is **cross-compilation for release**: if you
cross-compile release builds via zig, you might want to also
cross-compile debug builds via zig for consistency. But for
same-platform work, the linker choice is profile-independent.

---

## 8. Stable Diagnostics & Lints

### 8.1 Workspace-level lint configuration

```toml
# Cargo.toml (workspace root)
[workspace.lints.rust]
# Safety
unsafe_op_in_unsafe_fn = "deny"          # Require unsafe {} inside unsafe fn
# Our architecture already bans unwrap in non-test code (CLAUDE.md),
# but this catches cases the rule misses at the language level.

# Code quality
unreachable_pub = "warn"                 # Catch pub items that should be pub(crate)
missing_debug_implementations = "warn"   # All public types need Debug
unused_crate_dependencies = "warn"       # Catch unused Cargo.toml deps
redundant_imports = "warn"               # Remove duplicate imports
let_underscore_drop = "warn"             # Catch accidental immediate drops

# Bug prevention
trivial_casts = "warn"                   # Unnecessary `as` casts
trivial_numeric_casts = "warn"           # Unnecessary numeric `as` casts
explicit_outlives_requirements = "warn"  # Remove redundant lifetime bounds

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
# Override specific noisy pedantic lints
module_name_repetitions = "allow"        # polka_data::DataMap is fine
must_use_candidate = "allow"             # Too many false positives
missing_errors_doc = "allow"             # We use anyhow, not typed errors
```

Per-crate opt-in:
```toml
# Each crate's Cargo.toml
[lints]
workspace = true
```

### 8.2 CI enforcement

```sh
# Treat all warnings as errors in CI
RUSTFLAGS="-D warnings" cargo check --all-targets
RUSTFLAGS="-D warnings" cargo clippy --all-targets
```

### 8.3 Notable individual lints

**For our architecture specifically:**

| Lint | Level | Why |
|------|-------|-----|
| `unsafe_op_in_unsafe_fn` | deny | Column storage has unsafe regions — every unsafe op must be in an explicit unsafe block |
| `unreachable_pub` | warn | SDK crate boundary enforcement — catch accidentally public items |
| `unused_crate_dependencies` | warn | Keep dependency graph clean (architecture rule: deps flow downward) |
| `trivial_casts` | warn | Our newtype-heavy architecture means lots of conversions — catch unnecessary ones |

---

## 9. Target-Specific Tuning

### 9.1 Apple Silicon (aarch64-apple-darwin)

The macOS aarch64 baseline already includes 35+ features:
`neon` (SIMD), `aes`/`sha2`/`sha3` (crypto), `crc` (hardware CRC),
`lse` (better atomics), `fp16` (half-precision), `dotprod` (integer
dot product).

**The gain from `target-cpu=native` is small on Apple Silicon**
because the baseline is already high. Still worth enabling for:
- Apple M2/M3/M4 specific features not in the M1 baseline
- Ensuring LLVM knows the exact microarchitecture for scheduling

### 9.2 x86_64 (Linux deployment)

The x86_64 baseline is SSE2 only (year 2003). **Massive gains from
`target-cpu=native` or a higher baseline.**

| Level | Features | Covers | Flag |
|-------|----------|--------|------|
| Baseline | SSE2 | All x86_64 | (default) |
| v2 | SSE4.2, POPCNT, SSSE3 | Sandy Bridge+ (2011) | `-C target-cpu=x86-64-v2` |
| v3 | AVX2, FMA, BMI1/2 | Haswell+ (2013) | `-C target-cpu=x86-64-v3` |
| v4 | AVX-512 | Ice Lake+ (2019, server) | `-C target-cpu=x86-64-v4` |

**For our morsel processing inner loops**, AVX2 (`v3`) enables 256-bit
SIMD operations on column data. With our 16-byte stride and contiguous
arrays, LLVM can autovectorise to process 2 entries per SIMD operation
(256 / 128 = 2). With AVX-512 (`v4`), that doubles to 4 entries.

**Recommendation:** Target `x86-64-v3` as minimum for deployment.
Use `native` for benchmarking.

### 9.3 Cross-platform config

```toml
# .cargo/config.toml
[target.aarch64-apple-darwin]
rustflags = ["-C", "target-cpu=native"]

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = [
    "-C", "target-cpu=x86-64-v3",
    "-C", "link-arg=-fuse-ld=mold",
]
```

---

## 10. Build Caching & CI

### 10.1 sccache

Caches compiled artifacts across builds. Major speedup for clean
builds and CI.

```sh
cargo install sccache
export RUSTC_WRAPPER=sccache
```

Works with local disk cache by default. Can also use S3, GCS, or
Redis for shared CI cache.

### 10.2 Environment variables

| Variable | Purpose | Value |
|----------|---------|-------|
| `RUSTC_WRAPPER` | Wrap rustc calls | `sccache` |
| `CARGO_INCREMENTAL` | Force incremental on/off | `0` (CI), `1` (dev) |
| `CARGO_BUILD_JOBS` | Parallel compilation jobs | Integer (default: logical CPUs) |
| `CARGO_PROFILE_RELEASE_LTO` | Override profile via env | `"thin"` |
| `CARGO_PROFILE_RELEASE_CODEGEN_UNITS` | Override via env | `1` |

### 10.3 CI-specific considerations

- **Disable incremental in CI:** `CARGO_INCREMENTAL=0`. Incremental
  artifacts bloat the cache and are rarely useful in CI (each run
  starts from a different state).
- **Use `sccache` with shared storage:** S3/GCS-backed sccache turns
  clean CI builds into near-incremental speed.
- **Separate check and test steps:**
  `cargo check --all-targets` is much faster than `cargo test` and
  catches most errors. Run it first, fail fast.

---

## 11. Profile-Guided Optimisation (PGO)

PGO is **fully stable** and can yield 10-20% performance improvement
for CPU-bound workloads. It works by:

1. Building an instrumented binary that records branch frequencies
2. Running it against representative workloads
3. Rebuilding with the profile data, so LLVM optimises hot paths

### 11.1 Steps

```sh
# Step 1: Build with instrumentation
RUSTFLAGS="-C profile-generate=/tmp/pgo-data" \
    cargo build --release --target aarch64-apple-darwin

# Step 2: Run representative workloads
./target/aarch64-apple-darwin/release/polka build
./target/aarch64-apple-darwin/release/polka audit
# ... exercise all important code paths

# Step 3: Merge profile data
# (requires llvm-profdata — comes with brew install llvm)
/opt/homebrew/opt/llvm/bin/llvm-profdata merge \
    -o /tmp/pgo-data/merged.profdata /tmp/pgo-data/

# Step 4: Rebuild with profile data
RUSTFLAGS="-C profile-use=/tmp/pgo-data/merged.profdata" \
    cargo build --release --target aarch64-apple-darwin
```

### 11.2 When to use

- **Release builds** where runtime performance matters
- **After the architecture is stable** — PGO profiles are invalidated
  by code changes
- **In CI** for producing optimised release binaries

### 11.3 Interaction with other flags

PGO works with LTO, `codegen-units=1`, and `target-cpu=native`. The
combination of all four is the maximum optimisation stack:

```
PGO + LTO=thin + codegen-units=1 + target-cpu=native
```

---

## 12. Recommended Configurations

### 12.1 Development (fast iteration)

```toml
# rust-toolchain.toml
[toolchain]
channel = "nightly"
components = ["rust-src", "rustc-codegen-cranelift-preview"]
```

```toml
# Cargo.toml (workspace root)

# Cranelift for our code — fast compile, unoptimised runtime
[unstable]
codegen-backend = true

[profile.dev]
opt-level = 0
debug = 1                  # line-tables-only — significant macOS gains (Bevy tip)
incremental = true
codegen-backend = "cranelift"

# LLVM + optimisation for dependencies — cached, so the LLVM
# cost is paid once. Runtime perf matters for test throughput.
[profile.dev.package."*"]
codegen-backend = "llvm"
opt-level = 2
```

```toml
# .cargo/config.toml

[target.aarch64-apple-darwin]
# macOS: default Apple linker is fastest (Bevy finding, confirmed)
rustflags = [
    "-Zthreads=8",
    "-Zshare-generics=y",
    "-C", "target-cpu=native",
]

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = [
    "-Zthreads=8",
    "-Zshare-generics=y",
    "-C", "target-cpu=native",
    "-C", "link-arg=-fuse-ld=mold",
]
```

Plus: `export RUSTC_WRAPPER=sccache`

### 12.2 Release (maximum performance)

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"
strip = "symbols"
# Cranelift is NOT used for release — LLVM is the default
# and produces far better code.
```

```toml
# .cargo/config.toml
[unstable]
build-std = ["core", "alloc", "std"]

[target.aarch64-apple-darwin]
rustflags = [
    "-Zthreads=8",
    "-C", "target-cpu=native",
    "-C", "symbol-mangling-version=v0",
]

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = [
    "-Zthreads=8",
    "-C", "target-cpu=x86-64-v3",
    "-C", "link-arg=-fuse-ld=mold",
    "-C", "symbol-mangling-version=v0",
]
```

For cross-compiled release builds (macOS → Linux):
```sh
cargo zigbuild --release --target x86_64-unknown-linux-gnu
```

### 12.3 Profiling (release + debug info)

```toml
# Cargo.toml
[profile.profiling]
inherits = "release"
debug = "line-tables-only"
strip = "none"

[profile.profiling.build-override]
opt-level = 0
codegen-units = 256
```

Use with: `cargo build --profile profiling`

Enables flamegraphs with `cargo flamegraph` or Instruments on macOS,
while keeping full release optimisations.

### 12.4 CI

```toml
# Cargo.toml
[profile.ci]
inherits = "release"
lto = "thin"
codegen-units = 1
```

```sh
# CI script
export CARGO_INCREMENTAL=0
export RUSTC_WRAPPER=sccache

# Fast fail on type errors
RUSTFLAGS="-D warnings -Zthreads=8" cargo check --all-targets

# Lint
RUSTFLAGS="-D warnings -Zthreads=8" cargo clippy --all-targets

# Test
cargo test --all-targets

# Solver compatibility check (periodic)
RUSTFLAGS="-Znext-solver -Zthreads=8" cargo check --all-targets

# Sanitiser checks (periodic / nightly CI only)
RUSTFLAGS="-Zsanitizer=address -Zthreads=8" \
    cargo test -Zbuild-std --target aarch64-apple-darwin

# Semver check (before SDK releases)
cargo semver-checks
```

### 12.5 Feature gates (all crate roots)

```rust
// Always enabled
#![feature(min_specialization)]
#![feature(min_generic_const_args)]
#![feature(adt_const_params)]
#![feature(const_trait_impl)]

// Where useful
#![feature(explicit_tail_calls)]       // recursive AST traversal
#![feature(loop_match)]               // state machine inner loops
#![feature(never_type)]               // infallible pipelines
#![feature(macro_metavar_expr)]        // DSL macro improvements
```

### 12.6 Workspace lints

```toml
# Cargo.toml (workspace root)
[workspace.lints.rust]
unsafe_op_in_unsafe_fn = "deny"
unreachable_pub = "warn"
missing_debug_implementations = "warn"
unused_crate_dependencies = "warn"
redundant_imports = "warn"
let_underscore_drop = "warn"
trivial_casts = "warn"
trivial_numeric_casts = "warn"
explicit_outlives_requirements = "warn"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
module_name_repetitions = "allow"
must_use_candidate = "allow"
missing_errors_doc = "allow"
```

---

## Appendix: The full optimisation stack

From least to most aggressive, the layers that compound:

```
Layer 1 — Free (no trade-offs):
  -Zthreads=8                    parallel compilation
  -Zshare-generics=y             deduplicate monomorphised generics
  target-cpu=native              use all CPU features
  sccache                        cache compiled artifacts
  debug = 1 (macOS)              line-tables-only (Bevy tip — significant gains)
  [profile.dev.package."*"]      optimise deps in dev
    opt-level = 2

Layer 2 — Nightly build infrastructure (no code changes):
  cranelift backend (dev)        ~30% faster debug compilation
    codegen-backend = "cranelift" (own code)
    codegen-backend = "llvm"     (deps — cached anyway)
  -Zbuild-std                    LTO across std boundary
  mold linker (Linux)            15-20x faster linking vs GNU ld
  default linker (macOS)         Apple ld64 is already fast

Layer 3 — Release defaults:
  opt-level = 3                  full optimisation
  lto = "thin"                   cross-crate inlining
  codegen-units = 1              maximum inlining scope
  panic = "abort"                no unwind overhead
  strip = "symbols"              smallest binary
  symbol-mangling-version = v0   cleaner symbols for profiling

Layer 4 — Nightly feature gates:
  min_specialization             specialised hot paths
  min_generic_const_args         trait-level layout guarantees
  adt_const_params               structured const metadata
  const_trait_impl               compile-time trait methods
  loop_match                     jump-table state machines
  explicit_tail_calls            guaranteed TCE
  macro_metavar_expr             DSL macro improvements

Layer 5 — Cross-compilation:
  cargo-zigbuild                 turnkey macOS → Linux builds
  glibc version targeting        --target x86_64-linux-gnu.2.17

Layer 6 — Advanced (when architecture is stable):
  PGO                            profile-guided branch layout (10-20%)
  -Zsanitizer=address/thread     validation (not for release)
  -Znext-solver                  CI compatibility check
  -Zpolonius                     CI borrow checker check

Each layer compounds on the previous. Layer 1 is free. Layer 2
requires nightly but no code changes. Layer 3 is standard release
practice. Layer 4 requires feature gates in crate roots. Layer 5
is for CI/deployment. Layer 6 requires workflow changes.
```
