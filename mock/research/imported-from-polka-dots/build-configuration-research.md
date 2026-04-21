# Build Configuration Research

**Date:** 2026-03-13
**Supports:** 202603131230_topic.build-configuration-and-cfg-gating.md

Consolidated findings from four parallel research efforts on SIMD,
cache detection, heterogeneity detection, and Cargo configuration
mechanisms.

---

## 1. SIMD and auto-vectorisation

**portable_simd is unsuitable.** API still breaking on nightly (broke
Polars in Jan 2026 when LaneCount/SupportedLaneCount were removed).
No stabilisation timeline after 4+ years. Not suitable as a hard
dependency.

**Auto-vectorisation beats explicit SIMD.** DataFusion/arrow-rs
removed hand-written SIMD kernels because auto-vectorised code was
40% faster (PR #1221). Their strategy: columnar layout + tight loops
+ `chunks_exact` + `-C target-cpu=native`.

**`#[target_feature(enable = "avx2")]`** enables auto-vectorisation
with wider registers for the entire function body. The function
becomes unsafe to call (caller must verify via
`is_x86_feature_detected!`). This is the standard pattern.

**Morsel size does NOT need to be const for vectorisation.** LLVM's
Loop Vectorizer supports unknown trip counts. It vectorises the bulk
and handles remainder as scalar. The inner processing loop structure
matters, not the trip count being const. Use `chunks_exact(LANE_WIDTH)`
where LANE_WIDTH is const for the inner vectorised loop. Morsel size
and vector width are orthogonal concerns.

**`multiversion` crate** for shipping multi-ISA binaries with runtime
dispatch. Proven, maintained, works on stable. Compiles the same
function for N ISA levels, dispatches at runtime.

**Recommended pattern:**
```rust
#[target_feature(enable = "avx2")]
unsafe fn process_morsel(data: &[f32]) {
    let morsel_size = *MORSEL_SIZE; // LazyLock, one atomic load
    for chunk in data[..morsel_size].chunks_exact(8) {
        // LLVM auto-vectorises this with AVX2 (8xf32)
    }
}
```

Sources: DataFusion arrow-rs PR #1221, Shnatsel "State of SIMD in
Rust 2025", Nick Wilcox auto-vectorisation blog series, LLVM
Vectorizers documentation.

---

## 2. L1 cache size detection

**Cannot derive from target features.** CARGO_CFG_TARGET_FEATURE only
contains ISA flags (avx2, neon, etc.), never cache geometry. LLVM has
internal cache models (getCacheSize()) but they are not exposed to
Rust and are coarse (one value for all Intel Penryn through Kabylake).

**No CARGO_CFG var for cache info.** No cache line size, no L1 size,
no cache levels. There is also no CARGO_CFG_TARGET_CPU (the CPU name
from -C target-cpu is not passed to build scripts).

**Every major project uses hardcoded constants:**
- DuckDB: 2048 tuples (compile-time, overridable via CMake)
- DataFusion: 8192 rows (runtime, configurable via SessionConfig)
- Polars: 100,000 rows (runtime, env var POLARS_IDEAL_MORSEL_SIZE)
- None detect cache sizes automatically.

**build.rs CAN detect host cache:**
- macOS: `sysctl -n hw.perflevel0.l1dcachesize` returns 131072 (128KB)
  for M-series P-cores, 65536 (64KB) for E-cores
- Linux: `/sys/devices/system/cpu/cpu0/cache/index0/size` returns "32K"
- Breaks on cross-compilation (gives host values, not target)

**Apple Silicon cache line is 64 bytes, not 128.** `sysctl
hw.cachelinesize` reports 128, but that is the SoC-level DMA
coherency granularity. The actual CPU cache line (ARM CTR_EL0) is
64 bytes. LLVM correctly uses 64 for Apple CPUs. Crossbeam uses 128
pessimistically for false-sharing prevention (different concern).

**Existing crates:**
- `raw-cpuid`: runtime CPUID on x86 only, no compile-time info
- `cache-size`: thin wrapper around raw-cpuid, returns None on ARM
- `cache-padded`/crossbeam: compile-time constants per target_arch
  (128B for x86-64 and aarch64, pessimistic)
- No crate maps CPU model to L1/L2 sizes at compile time

Sources: LLVM X86TargetTransformInfo.cpp, AArch64Subtarget.cpp,
DuckDB discussion #16963, DataFusion ARROW-12136.

---

## 3. Core heterogeneity detection

**macOS (Apple Silicon):**

`sysctlbyname()` provides complete P/E topology:
```
hw.nperflevels                  = 2
hw.perflevel0.physicalcpu       = 8   (P-cores)
hw.perflevel0.l1dcachesize      = 131072 (128KB)
hw.perflevel0.l2cachesize       = 12582912 (12MB)
hw.perflevel1.physicalcpu       = 2   (E-cores)
hw.perflevel1.l1dcachesize      = 65536 (64KB)
hw.perflevel1.l2cachesize       = 4194304 (4MB)
```

Callable from Rust via `libc::sysctlbyname()`. Works in build.rs for
host builds. Convention: perflevel0 = performance, perflevel1 =
efficiency.

**Linux (Intel hybrid, kernel 6.2+):**

```
/sys/devices/system/cpu/types/intel_core_0/cpulist   -> "4-7"
/sys/devices/system/cpu/types/intel_atom_0/cpulist   -> "0-3"
```

If `/sys/devices/system/cpu/types/` does not exist, the system is
homogeneous.

**Compile-time heuristic:**
- `cfg(target_vendor = "apple", target_arch = "aarch64")` = always
  heterogeneous (all Apple Silicon has P/E). Cannot determine the
  exact P/E ratio at compile time (M1 = 4P+4E, M1 Pro = 8P+2E).
- Intel hybrid cannot be detected at compile time (no target feature
  distinguishes 12th gen+ from earlier).

**Rust crates:**
- `hwlocality` (wraps hwloc 2.x): cpukinds API with `hwloc-2_4_0`
  feature. Unified cross-platform P/E detection. Requires hwloc C
  library as system dependency.
- `num_cpus`, `sysinfo`, `core_affinity`: no P/E awareness.
- `raw-cpuid`: does not implement CPUID leaf 0x1A (hybrid detection).

**LazyLock for morsel sizing:**

Post-initialisation fast path: one atomic load (Acquire ordering).
On x86_64: compiles to plain `mov` (TSO provides acquire semantics
for free). On aarch64: `ldar` instruction, sub-nanosecond overhead.

Pattern: dereference once into local `usize` at function entry:
```rust
static MORSEL_SIZE: LazyLock<usize> = LazyLock::new(|| ...);

fn process(data: &[u8]) {
    let morsel_size = *MORSEL_SIZE; // one atomic load
    for chunk in data.chunks(morsel_size) {
        // morsel_size is a plain usize in a register
    }
}
```

LLVM cannot constant-propagate through LazyLock, but this barely
matters: the loop body structure drives vectorisation, not the trip
count.

**No major database engine implements core-type-aware scheduling.**
DuckDB, Polars, DataFusion all use work-stealing for implicit load
balancing. SiliconDB (VLDB 2019) showed 2x gains with explicit P/E
awareness but this hasn't been adopted in production systems.

Sources: Apple Developer Forums thread 692671, hwloc issue #454,
Phoronix Intel hybrid sysfs patches, SiliconDB VLDB 2019.

---

## 4. Cargo configuration mechanisms

**`cargo --config`:** Highest precedence. Supports all config keys.
Multiple flags merge left-to-right. Can pass TOML strings or file
paths. Fully stable.

**`.cargo/config.toml` `include`:** Stable since Cargo 1.93 (Dec
2025). No conditional includes (no cfg-gated inclusion). Paths must
be relative and end with `.toml`.

**`[target.<cfg>]` rustflags:** Supports target predicates
(target_arch, target_os, target_vendor, target_feature) but NOT
`cfg(feature = "...")`. Cannot distinguish Intel from AMD within
x86_64 (same triple).

**`rustc-workspace-wrapper`:** Wraps every rustc invocation for
workspace members only. Has access to all CARGO_CFG_* env vars. Can
inspect target arch/vendor and inject arbitrary -C flags. Stable.
This is the strongest mechanism for conditional codegen flags.

Example:
```bash
#!/bin/bash
RUSTC="$1"; shift
if [ "$CARGO_CFG_TARGET_VENDOR" = "apple" ]; then
    exec "$RUSTC" "$@" -C target-cpu=apple-m1
elif [ "$CARGO_CFG_TARGET_ARCH" = "x86_64" ]; then
    exec "$RUSTC" "$@" -C target-cpu=x86-64-v3
fi
exec "$RUSTC" "$@"
```

**Build script limitations:** `cargo:rustc-flags` only supports -l
and -L. Cannot emit -C flags (deliberate Cargo design decision,
cargo#1293). Build scripts can emit cfg flags and env vars but not
codegen options.

**Profile-rustflags:** Still nightly-only. Has known bugs (not
propagated to CARGO_ENCODED_RUSTFLAGS, doesn't trigger recompile
properly, incompatible with build-std). No stabilisation target.

**`[env]` table:** Cannot set RUSTFLAGS or CARGO_ENCODED_RUSTFLAGS
(blocked by cargo#9579). Can set other env vars visible to build
scripts and rustc.

**How real projects handle this:**
- Polars: `RUSTFLAGS="-C target-cpu=native"` env var
- ring: build.rs compiles different assembly per CARGO_CFG_TARGET_ARCH
- Firefox: custom build system constructs RUSTFLAGS programmatically
- Nobody solves vendor-specific flags purely within Cargo config

**Key finding:** The four rustflag sources are mutually exclusive
(first match wins): CARGO_ENCODED_RUSTFLAGS > RUSTFLAGS >
target.<cfg>.rustflags > build.rustflags. If any target.<cfg> matches,
build.rustflags is completely ignored.

Sources: Cargo Reference (config, build-scripts, profiles, unstable),
cargo#10271, cargo#12862, cargo#14306, cargo#16284.

---

## 5. Synthesis: what the build model should look like

**Custom cargo features: almost none needed.**

- SIMD: use target_feature + target-cpu. No custom features.
- L1/cache: runtime detection, LazyLock. No custom features.
- Core topology: runtime detection, LazyLock. No custom features.
- Heterogeneity: compile-time cfg for Apple Silicon code paths
  (target_vendor + target_arch), runtime for Intel hybrid.
- Vendor identity: not a feature. target-cpu handles LLVM tuning.

**Profiles: tier-only.**

- dev (LLVM default, accurate)
- dev-fast (Cranelift, fast iteration)
- bench (LLVM, release-level opt)
- release (LLVM, max opt, LTO, strip)
- profiling (release + debug info)

No vendor axis. No profile explosion.

**LLVM vendor flags: three options, ranked.**

1. `RUSTFLAGS="-C target-cpu=native"` for host builds (simplest)
2. `[target.'cfg(target_arch)']` rustflags in config.toml for
   architecture-level defaults
3. `RUSTC_WORKSPACE_WRAPPER` script for fine-grained vendor dispatch

For most development: option 1. For CI/distribution: option 2 or 3.

**Runtime hardware detection: LazyLock<HardwareTopology>.**

Computed once at first access:
- macOS: sysctlbyname for P/E counts, L1/L2 per core type
- Linux: sysfs for topology, getconf for cache sizes
- Fallback: num_cpus for total count, architecture defaults for cache

Morsel sizes derived from detected topology. Functions copy to local
usize at entry. Zero measurable overhead.

**polka-meta-build: minimal role.**

- compile_error! guards for conflicting feature combinations
  (if any custom features remain)
- For host builds: could run sysctl/sysfs detection and emit
  cargo:rustc-cfg flags as a convenience. But runtime detection
  is more robust and works for all build scenarios.

The crate may end up nearly empty. That is fine.
