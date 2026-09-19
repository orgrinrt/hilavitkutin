# Foundations: Substrate, Platform, Build

The engine sits on three foundations: the arvo primitive family for every
numeric and analysis need, a platform tier model that keeps the engine free
of OS assumptions, and a build-time optimisation crate that makes the
devirtualised dispatch actually devirtualise.

## The arvo substrate

All mathematical and analysis primitives the engine uses live in the arvo
crate family, defined once with the same `#![no_std]`, no-alloc,
const-generic-sizing, LLVM-optimised discipline, and reusable by any
consumer. hilavitkutin consumes them; it never reimplements them, and when a
substrate limitation surfaces the fix goes upstream into arvo, never into a
parallel local implementation.

| Crate | Provides |
|---|---|
| arvo | Fixed-point numerics (`UFixed`/`IFixed`), strategy markers (Hot/Precise/Warm), float wrappers (FastFloat/StrictFloat), composable trait family, semantic aliases |
| arvo-bitmask | Fixed-width masks, bit-matrix adjacency, set ops, popcount and scan, dirty propagation |
| arvo-sparse | CSR sparse matrices, RCM reordering, block diagonal detection, Dulmage-Mendelsohn decomposition |
| arvo-graph | Kahn's topological sort, connected components, weighted paths, upward and downward rank, critical path, waist detection, node renumbering |
| arvo-spectral | Laplacian construction, power iteration, Fiedler vector, spectral bisection and k-way partitioning |
| arvo-comb | Matrix-chain ordering DP, greedy constrained interval grouping, two-level greedy bin-packing |

How the engine maps onto them: arvo-bitmask carries `AccessMask`, DAG
adjacency, and virtual-flag storage; arvo-graph does topo sort, waist
detection, rank, and components; arvo-sparse does the access matrix, RCM,
block diagonal, and Dulmage-Mendelsohn; arvo-spectral does spectral
partitioning in the plan; arvo-comb does fiber-grouping DP and bin-packing.
Column data types come from wherever the consumer likes: hilavitkutin does
not care what is in the columns, only the read/write dependency structure.
Boolean columns are `UFixed<1,0>` with 8x density under the BitPacked layout.

Internal numerics follow the arvo-types-only discipline (weights and costs
are arvo fixed-point newtypes, not raw integers; they compile to the same
machine instructions). Graph and analysis capacities follow the
`Capacity`/`Dim<N>` pattern (arvo's plan-dimension axis: a `Capacity` trait
implemented by `Dim<N>` marker types, so every capacity is a type parameter
with a tunable default) rather than hardcoded const generics, per the
caps-are-defaults rule in [[constraints]].

arvo detects hardware capabilities in its own build script via target
features, not via custom cfg flags from hilavitkutin-build; the single cfg it
accepts from the build crate is the fast-math signal when that pragma is
active.

## Platform tiers

Two tiers, os and no_os. The std tier the founding spec sketched is deferred
indefinitely (op decision, A1 constraint note 6).

| Tier | Target | Memory | Threads | Clock | Panics |
|---|---|---|---|---|---|
| os (default) | Linux, macOS | raw mmap | raw pthread/clone | raw clock_gettime | abort |
| no_os | bare metal, WASM | consumer provides | consumer provides | consumer provides | abort |

The no_os tier is trait contracts only; the consumer provides every platform
implementation. The `alloc` crate is technically available under no_std but
hilavitkutin does not use it: no Vec, no Box, no String anywhere. All dynamic
storage goes through consumer-provided backing memory or stack arrays;
consumers may use alloc in their own code.

The platform contracts live in hilavitkutin-api:

```rust
pub trait MemoryProviderApi: Send + Sync {
    unsafe fn allocate(&self, len: USize, align: USize) -> *mut u8;
    unsafe fn deallocate(&self, ptr: *mut u8, len: USize, align: USize);
    unsafe fn protect(&self, ptr: *mut u8, len: USize, read: Bool, write: Bool);
}
pub trait ThreadPoolApi: Send + Sync {
    fn spawn<F: FnOnce() + Send + 'static>(&self, f: F);
    fn worker_count(&self) -> USize;
}
pub trait ClockApi: Send + Sync {
    fn now_ns(&self) -> Nanos;
}
```

(The shipped names carry the `Api` suffix and arvo-typed signatures; the
founding spec's bare `MemoryProvider` / `ThreadPool` / `Clock` spellings
refer to the same three contracts.)

Each tier ships defaults; the consumer overrides through the scheduler
builder. The Scheduler is generic over the platform types, never `dyn`
(storing `dyn MemoryProvider` would violate the dyn ban); platform traits
monomorphise per call site. The executor is generic over the thread-pool
contract, so the same engine drives any conforming pool.

The clock is a builder-slot provider (`SchedulerBuilder::clock`), defaulting
to the os-tier monotonic clock under the default feature and to a null clock
on no_os until dependency injection supplies one. Platform inputs routed
through a generic `.with(...)` drop their values; value-carrying providers
get dedicated builder slots (A1 constraint note 7).

Hardware detection (L1 size, core count and classes) runs once at startup:
CPUID on x86, sysctl on macOS ARM, sysfs on Linux ARM, with a 32 KB L1 and
one core as the fallback.

## Build-time optimisation

hilavitkutin-build is a shared build-dependency crate, not an xtask. Every
crate's `build.rs` calls `hilavitkutin_build::configure().run()`. It is
independent of the runtime: it optimises how code is compiled.

The four-part configuration model: cfg-flag emission from the build crate; no
custom cargo features (the compiler's target features and `CARGO_CFG_*`
suffice); a `RUSTC_WORKSPACE_WRAPPER` carrying all LLVM codegen flags per
target and tier; and `compile_error!` guards for mutually exclusive options.

The pragma system is a builder API
(`configure().profile("release", |p| p.enable::<(...)>()).run()`) over
built-in pragmas: LoopOptimization, Polly, MathPeephole, FastMath,
ExpandedLto, Bolt, Pgo, Profiling, BuildStd, ParallelCodegen, SharedGenerics,
LoopFusion, MimallocAllocator. A `Pragma` trait with a `Requires` associated
type enforces dependencies. Pragmas take effect through three mechanisms:
generated config (ExpandedLto, Pgo, BuildStd), the rustc wrapper
(LoopOptimization, Polly, MathPeephole, FastMath), and post-build hooks
(Bolt, Profiling).

LLVM pass work registers at the vectoriser-start and optimiser-last extension
points, with IR cleanup ahead of Polly SCoP detection. PGO and BOLT share
benchmark runs (one run produces both the .profraw and the perf data); BOLT
is Linux ELF only, with machine-function splitting as the partial macOS
alternative. The optimisation ladder is Stock, Plugins, Static BOLT, PGO,
Profile-guided BOLT.

Five profiles: dev (Cranelift for the workspace, LLVM for deps), dev-opt,
release (full pragmas), profiling (release plus debug info), ci (release with
thin LTO for caching).

**The load-bearing fact:** the ExpandedLto pragma (fat LTO plus one codegen
unit) is required for LLVM to devirtualise the monomorphised dispatch.
Without it, struct-field function-pointer arrays do not devirtualise and
dispatch is 12.6x slower. The dispatch design in [[dispatch]] assumes
ExpandedLto.
