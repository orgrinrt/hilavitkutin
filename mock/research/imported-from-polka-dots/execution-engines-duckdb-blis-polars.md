# Execution Engine Research: DuckDB, BLIS, Polars, and Related Systems

**Date:** 2026-03-12
**Purpose:** Deep reference for polka-dots columnar execution design.
Covers execution models, scheduling strategies, cache optimisation,
parallelism approaches, and memory management across the systems we
evaluated when designing our fused-chain morsel-driven pipeline.

**Systems covered:**
- DuckDB (and the HyPer morsel-driven model it inherits)
- BLIS (BLAS-like Library Instantiation Software)
- Polars (streaming/new streaming engine)
- Apache Arrow DataFusion (briefly)
- fjall (briefly, for Rust-specific patterns)

---

## Table of Contents

1. [DuckDB — Vectorised Morsel-Driven Execution](#1-duckdb)
   1.1 Origins: The HyPer Morsel-Driven Model
   1.2 Push-Based Vectorised Execution
   1.3 Vectors and Morsels — Two-Level Granularity
   1.4 Pipeline Construction
   1.5 Pipeline Breakers
   1.6 Pipeline Dependencies and the Execution DAG
   1.7 Thread Scheduling and Dispatch
   1.8 NUMA Awareness
   1.9 Cache-Aware Sizing
   1.10 Memory Management
   1.11 Adaptive Execution
   1.12 Key Constants and Configuration
   1.13 Lessons for polka-dots

2. [BLIS — Cache-Oblivious Tiling for Linear Algebra](#2-blis)
   2.1 The Five-Loop Nest
   2.2 Blocking Parameters and Cache Mapping
   2.3 Data Packing
   2.4 The Micro-Kernel
   2.5 Hardware Detection and Sub-Configurations
   2.6 Hot-Swap Kernel Mechanism
   2.7 Thread Partitioning and Cache Topology
   2.8 Prefetching Strategy
   2.9 Configuration System
   2.10 Key Constants and Typical Values
   2.11 Lessons for polka-dots

3. [Polars — Streaming DAG Execution](#3-polars)
   3.1 Architecture Overview
   3.2 The Compute Node Graph
   3.3 Morsel Model
   3.4 Parallelism: Partition-Based
   3.5 Pipeline Blockers and Scheduling
   3.6 Operator Fusion (Structural)
   3.7 Backpressure and Flow Control
   3.8 Memory Management and Out-of-Core
   3.9 Morsel Ordering and Determinism
   3.10 Key Constants and Configuration
   3.11 Lessons for polka-dots

4. [Apache Arrow DataFusion — Brief Survey](#4-datafusion)
   4.1 Execution Model
   4.2 Batch Size
   4.3 Partitioning
   4.4 Lessons for polka-dots

5. [fjall — Rust-Specific Patterns](#5-fjall)
   5.1 Block Sizing
   5.2 ByteView Pattern
   5.3 Configuration Model
   5.4 Lessons for polka-dots

6. [Comparative Analysis](#6-comparative-analysis)
   6.1 Scheduling Models Compared
   6.2 Cache Strategies Compared
   6.3 Fusion Models Compared
   6.4 Parallelism Models Compared
   6.5 Memory Management Compared
   6.6 Configuration and Adaptation Compared

7. [Cross-Cutting Concerns](#7-cross-cutting-concerns)
   7.1 Error Handling Across Systems
   7.2 Cancellation and Early Termination
   7.3 Progress Reporting
   7.4 Resource Cleanup
   7.5 Determinism and Reproducibility
   7.6 Thread Pool Models

8. [Implications for polka-dots](#8-implications)
   8.1 What We Adopted
   8.2 What We Consciously Deferred
   8.3 Open Questions

9. [Appendix: Source References and Further Reading](#9-appendix)

---

## 1. DuckDB — Vectorised Morsel-Driven Execution

### 1.1 Origins: The HyPer Morsel-Driven Model

DuckDB's execution model derives from the morsel-driven parallelism
framework introduced by Leis et al. at SIGMOD 2014, originally designed
for the HyPer in-memory database. The core insight: rather than
statically partitioning data across threads (Volcano-style exchange
operators) or using a single-threaded pipeline with explicit
parallelism operators, the system divides input data into fixed-size
fragments called **morsels** and assigns them dynamically to worker
threads through a central dispatcher.

The morsel model solves three problems simultaneously:

**Elastic parallelism.** The degree of parallelism can change at morsel
boundaries. Threads can be reassigned to different queries mid-execution.
If a new high-priority query arrives, threads finishing their current
morsel can be redirected. This is impossible with static partitioning,
where thread assignments are fixed for the query's lifetime.

**Load balancing.** Fine-grained morsel assignment naturally handles
data skew. Fast threads (processing easy morsels) simply request more
work. Slow threads (processing skewed data or running on a loaded core)
process fewer morsels. No explicit load-balancing logic is needed —
the dispatcher is the load balancer.

**NUMA awareness.** The dispatcher preferentially assigns morsels to
threads running on the same NUMA socket where the data resides. Each
thread writes results to NUMA-local storage areas. This is critical
on multi-socket servers where cross-socket memory access has 2-3x
higher latency.

The original HyPer paper experimentally determined that a morsel size
of approximately 100,000 tuples yields a good trade-off between
scheduling overhead, load balancing granularity, and instant elasticity
adjustment. Performance is roughly flat for morsel sizes above ~10,000
tuples (Figure 6 in the paper), meaning the system is insensitive to
the exact morsel size within a wide range.

### 1.2 Push-Based Vectorised Execution

DuckDB uses a **push-based** vectorised execution model. DataChunks
(vectors of column data) are pushed through the operator pipeline from
source to sink. This contrasts with Volcano's pull-based model where
each operator calls `next()` on its child.

Push-based execution has several advantages for morsel-driven
scheduling:

- The source operator controls the iteration rate. It produces one
  vector at a time and pushes it through the pipeline. When the morsel
  is exhausted, it returns control to the scheduler.
- Pipeline fusion is natural: each operator's `Execute(input, output)`
  method processes one vector and passes the result to the next operator.
  No intermediate materialisation between operators within a pipeline.
- The push model maps cleanly to the morsel dispatcher: a thread
  receives a morsel, creates a pipeline executor, and pushes vectors
  through the pipeline until the morsel is done.

The execution is **vectorised**, not tuple-at-a-time. Each operator
processes a vector of tuples (default 2,048 rows) in a tight loop.
This amortises function call overhead, enables SIMD processing, and
keeps data in CPU cache. The Kersten et al. VLDB 2018 paper found
the sweet spot for vectorised execution is 1,024-4,096 tuples per
vector.

### 1.3 Vectors and Morsels — Two-Level Granularity

DuckDB maintains two distinct size constants for two distinct purposes:

**STANDARD_VECTOR_SIZE = 2,048 rows.** Defined in
`src/include/duckdb/common/vector_size.hpp`. This is the **processing
granularity** — the unit of work within a pipeline. Each operator
processes one vector at a time. The vector size is chosen so that a
vector of column data fits comfortably in L1 cache: at 8 bytes per
value, 2,048 × 8 = 16 KB, well within the typical 32-48 KB L1 data
cache.

The vector size must be a power of 2 (for efficient selection vector
operations and bitwise masking). It is a compile-time constant that
can be overridden at build time but cannot change at runtime.

**DEFAULT_ROW_GROUP_SIZE = 122,880 rows.** Defined in
`src/include/duckdb/storage/storage_info.hpp`. This is the **scheduling
granularity** — the unit of work assignment between the dispatcher and
worker threads. A row group contains exactly 60 vectors
(122,880 / 2,048 = 60).

The row group / morsel size is chosen to:
- Amortise scheduling overhead (one dispatch per 122,880 rows, not
  per 2,048)
- Provide fine enough granularity for load balancing (at 100 million
  rows, that's ~814 morsels to distribute across threads)
- Keep thread-local state (hash tables, aggregation buffers) within
  L2/L3 cache bounds

The relationship: a **morsel is the scheduling unit**, a **vector is
the cache-fitting unit**. A thread receives one morsel, then processes
it 60 vectors at a time through the fused pipeline. The two sizes
serve different purposes and are tuned independently.

This two-level granularity means scheduling overhead is incurred only
every 122,880 rows (once per morsel dispatch), while cache-fitting
overhead is managed every 2,048 rows (once per vector). The
scheduling-to-processing ratio is 60:1.

### 1.4 Pipeline Construction

A **pipeline** in DuckDB is a linear chain of operators:

```
Source → Operator₁ → Operator₂ → ... → Operatorₙ → Sink
```

**Source** operators produce data. The most common source is a table
scan, which reads row groups from storage. Other sources include the
probe side of a hash join (reading from the built hash table) or a
window function source (reading from materialised partitions).

**Operators** transform data. Each implements
`Execute(DataChunk &input, DataChunk &output)`. Operators include
filters, projections, hash probes, and expression evaluation. They
must be stateless with respect to the morsel — processing one vector
must not depend on having seen previous vectors (within the pipeline's
execution on a single thread, local state is allowed).

**Sinks** consume data, typically materialising it. A hash table build
side accumulates tuples into a shared hash table. A sort accumulates
tuples into a local buffer. An aggregation accumulates partial
aggregates.

Pipeline construction happens during physical plan optimisation. The
physical plan tree is traversed, and operators are assigned to
pipelines. Consecutive operators that can process data without
materialisation are fused into the same pipeline.

### 1.5 Pipeline Breakers

A **pipeline breaker** is an operator that must consume ALL input
before producing ANY output. It terminates one pipeline (as a sink)
and starts another (as a source). Pipeline breakers are the only
points where data is fully materialised between operators.

Known pipeline breakers in DuckDB:

**Hash join build side.** The entire build-side input must be consumed
to construct the hash table before the probe side can begin. The build
pipeline ends at the hash table sink. The probe pipeline starts with
the hash table as its source. The hash table itself is a shared data
structure built cooperatively by all threads processing build-side
morsels.

**Sort.** All input must be materialised before producing sorted
output. DuckDB uses a merge-sort approach: each thread sorts its
local morsels, then a merge phase combines sorted runs.

**Aggregation (hash-based).** Full aggregation requires seeing all
groups before emitting results. DuckDB uses thread-local hash tables
for pre-aggregation (reducing cardinality within each morsel), then
merges them. Pre-aggregation is a form of partial fusion — it reduces
the data volume that must be materialised.

**Window functions.** Require full partition data before computing
frame-based results. The partitioning step is a pipeline breaker;
the window computation itself may pipeline within partitions.

**Order-preserving operations.** Any operation that requires a global
order (e.g., LIMIT after ORDER BY) must materialise and sort first.

The key property: within a pipeline (between breakers), data flows
through operators without intermediate materialisation. Each vector
passes through the full operator chain before the next vector is
fetched from the source. This is where the cache efficiency comes
from — the vector stays in L1 as it passes through all operators.

### 1.6 Pipeline Dependencies and the Execution DAG

Pipelines form a **dependency DAG**. The probe pipeline depends on the
build pipeline completing. A union-all pipeline depends on all its
input pipelines. The `PipelineBuildState` tracks these dependencies.

The `Executor` manages the pipeline DAG:
1. Find all pipelines with no unsatisfied dependencies (root pipelines)
2. Launch those pipelines (assign morsels to threads)
3. When a pipeline completes, check if any dependent pipelines are now
   unblocked
4. Launch newly unblocked pipelines
5. Repeat until all pipelines complete

Independent pipelines (no shared dependencies) can run simultaneously.
The executor does not explicitly parallelise across pipelines — it
relies on the thread pool naturally assigning threads to available
work. If two pipelines are ready and there are enough threads, both
run in parallel.

Within a single pipeline, morsel-level parallelism provides the
concurrency: multiple threads process different morsels of the same
pipeline simultaneously.

### 1.7 Thread Scheduling and Dispatch

DuckDB's thread scheduling lives in `src/parallel/task_scheduler.cpp`.

**Thread pool.** DuckDB maintains a configurable thread pool. By
default, the pool size equals the number of logical CPU cores. Threads
are long-lived — they wait for tasks, execute them, and return to
waiting.

**Task queue.** Tasks are enqueued into a **lock-free concurrent queue**
(based on moodycamel's ConcurrentQueue). This is critical for
scalability — a mutex-based queue would become a bottleneck at high
thread counts.

**Task types.** Each task wraps a `PipelineTask` containing a
`PipelineExecutor`. The task's `Execute()` method processes vectors
through the pipeline until:
- `TASK_FINISHED` — morsel exhausted, task done
- `TASK_NOT_FINISHED` — reschedule immediately (more work in this
  morsel, voluntarily yielding to check for higher-priority work)
- `TASK_BLOCKED` — external dependency not ready, deschedule and
  wake later

**Morsel dispatch.** The source operator is responsible for morsel
assignment. When a PipelineTask starts, it calls
`source.GetData(chunk)` which returns the next vector from the
source's morsel. When the morsel is exhausted, the task finishes and
the thread picks up the next available task.

`LaunchScanTasks()` creates one task per available thread, bounded
by `source_state->MaxThreads()` and operator-specific limits. This
prevents over-subscription — if a source can only produce data from
3 files, only 3 tasks are created even if 16 threads are available.

**Thread pinning.** Automatic when `hardware_concurrency() > 64`.
On large machines, thread migration between cores causes L1/L2 cache
thrashing. Pinning threads to cores prevents this.

**The dispatcher is not a separate thread.** It is a lock-free data
structure executed inline by the requesting worker thread. When a
thread finishes its current task, it atomically dequeues the next
task from the queue. No scheduler thread, no contention on a central
lock, no cross-thread signaling for dispatch.

### 1.8 NUMA Awareness

DuckDB's NUMA awareness operates at multiple levels:

**Morsel assignment.** The dispatcher preferentially assigns morsels
from storage regions local to the thread's NUMA socket. This is
implemented in the scan operator: each thread has a preferred set of
row groups based on the NUMA topology.

**Thread-local storage.** Hash table build operations use per-thread
storage areas allocated on the local NUMA node. The merge phase
combines these local tables. This avoids cross-socket writes during
the hot build phase.

**Hash table layout.** The original HyPer paper describes using
interleaved memory allocation for the final hash table (after the
build phase), so that probe operations from any socket have roughly
equal access latency. DuckDB's approach may differ in implementation
details.

**Large pages.** The morsel paper mentions using 2 MB virtual memory
pages (huge pages) for both hash tables and tuple storage. Benefits:
the page table fits in L1 cache (fewer TLB entries needed for a given
memory range), and kernel page fault overhead is reduced during the
build phase (fewer faults, each mapping a larger region).

For polka-dots, NUMA is largely irrelevant — dotfiles workloads run on
consumer hardware (laptops, desktops) with a single memory controller.
Multi-socket servers with NUMA are not the target deployment.

### 1.9 Cache-Aware Sizing

DuckDB's cache awareness is implicit rather than explicit:

**Vector size (2,048).** Chosen so that a vector of column data fits
in L1/L2 cache. For 8-byte integer columns, 2,048 × 8 = 16 KB. For
16-byte columns (e.g., HUGEINT, DECIMAL), 2,048 × 16 = 32 KB. Both
fit in the typical 32-48 KB L1 data cache with room for processing
state.

The vector size is a **compile-time constant**. DuckDB does NOT detect
L1 cache size at runtime. The constant was chosen once to be safe on
all target platforms. This is the "DuckDB approach" that polka-dots
explicitly rejected in favor of runtime detection.

**Row group size (122,880).** Chosen so that thread-local hash tables
during pre-aggregation stay within L2/L3 cache bounds. If a thread
processes a 122,880-row morsel with moderate cardinality reduction, the
local hash table fits in L2 (typically 256 KB - 1 MB).

**CachingPhysicalOperator.** A special operator that buffers small
chunks (threshold: 64 rows) to prevent sending tiny DataChunks through
the pipeline. If a filter is very selective (e.g., 1 in 100 rows
pass), the output chunks would be very small, and the per-vector
overhead would dominate. The caching operator accumulates rows until
the threshold is met, then pushes a full-sized chunk.

**No runtime L1 detection.** DuckDB does not probe CPUID, sysctl, or
sysfs for cache sizes. The 2,048 constant was chosen conservatively
and works across platforms. This simplifies the code but leaves
performance on the table on platforms with larger L1 caches (Apple
Silicon with 128-192 KB L1 could efficiently process ~8,000-12,000
rows per vector at 16 bytes/entry).

### 1.10 Memory Management

**Per-thread memory budget.** DuckDB computes a per-operator-per-thread
memory budget: `(max_memory / num_threads) / 4`. This prevents any
single operator on any single thread from consuming all available
memory. The factor of 4 accounts for multiple active operators and
temporary allocations.

**Buffer manager.** DuckDB has a buffer manager that tracks memory
usage and can evict pages to disk when the memory limit is reached.
This is primarily for the storage layer (row groups on disk), not for
intermediate pipeline results.

**Temporary directory.** When memory pressure is high, DuckDB can
spill intermediate results (e.g., partially-built hash tables, sort
runs) to a temporary directory on disk. This enables processing
datasets larger than available RAM, at the cost of I/O.

**Memory allocator.** DuckDB uses jemalloc on Linux for reduced
fragmentation and better multi-threaded allocation performance. On
other platforms, the system allocator is used.

### 1.11 Adaptive Execution

DuckDB includes several adaptive execution features:

**Adaptive joins.** The join implementation can switch between hash
join and nested-loop join based on estimated cardinalities and runtime
statistics.

**Adaptive aggregation.** Pre-aggregation in each thread's local hash
table reduces the data volume before the global merge. If the
aggregation has low cardinality (few groups), pre-aggregation is very
effective. If cardinality is high (many groups), the local hash tables
grow large and pre-aggregation provides less benefit.

**Parallelism limits.** Source operators can limit the number of
parallel tasks based on the data layout. A scan over 3 Parquet files
creates at most 3 parallel tasks, even if 16 threads are available.
This prevents over-subscription and unnecessary synchronisation.

### 1.12 Key Constants and Configuration

| Constant | Value | Purpose | Configurable |
|----------|-------|---------|-------------|
| STANDARD_VECTOR_SIZE | 2,048 | Processing granularity (L1 fitting) | Compile-time only |
| DEFAULT_ROW_GROUP_SIZE | 122,880 | Scheduling granularity (morsel) | Yes |
| CachingOperator threshold | 64 rows | Minimum chunk size to push | No |
| Memory budget factor | 1/4 | Per-operator-per-thread share | Via max_memory setting |
| Thread pool size | logical core count | Default parallelism | SET threads = N |
| Thread pinning threshold | > 64 cores | Auto-pin on large machines | No |

### 1.13 Lessons for polka-dots

**What we adopted:**
- Morsel-driven scheduling (dynamic assignment, not static partitioning)
- Pipeline/chain fusion (operators fused within a pipeline)
- Pipeline breakers force materialisation boundaries
- Pipeline dependency DAG with progress tracking

**What we adapted:**
- Single-level granularity (morsel = both scheduling and cache-fitting)
  instead of DuckDB's two-level (morsel for scheduling, vector for cache).
  Justified by our small data volumes.
- Runtime L1 detection instead of fixed constants. DuckDB's 2,048
  constant is conservative for Apple Silicon's 128-192 KB L1.
- Variable morsel sizes per chain instead of one global constant.

**What we deferred:**
- NUMA awareness (not relevant for consumer hardware)
- Spill to disk (data fits in memory)
- Lock-free dispatch (low thread count, contention negligible)
- Adaptive execution (static workloads)
- Per-thread memory budgets (small data volumes)

---

## 2. BLIS — Cache-Oblivious Tiling for Linear Algebra

### 2.1 The Five-Loop Nest

BLIS implements general matrix multiplication (GEMM: C += A × B) using
a five-loop nest around a micro-kernel. This is the canonical
high-performance linear algebra implementation strategy, documented in
the BLIS TOMS (Transactions on Mathematical Software) paper by Van Zee
and van de Geijn.

The five loops, from outermost to innermost:

```
5th loop (JC): partitions n dimension — nc columns of B, C
  4th loop (PC/KC): partitions k dimension — kc rows of A, B
    3rd loop (IC): partitions m dimension — mc rows of A, C
      2nd loop (JR): iterates over nr-wide micropanels of packed B
        1st loop (IR): iterates over mr-tall micropanels of packed A
          MICRO-KERNEL: computes mr × nr tile of C
```

Each loop level corresponds to a level of the cache hierarchy. The
blocking parameters (mc, nc, kc, mr, nr) are chosen so that the data
accessed by the inner loops fits in the target cache level.

The genius of this structure is that it separates concerns completely:
- The outer loops handle data movement (cache tiling)
- The micro-kernel handles computation (register-level)
- The packing routines handle data layout (contiguous, aligned)
- Each can be optimised independently

### 2.2 Blocking Parameters and Cache Mapping

The blocking parameters are constrained by cache sizes:

| Loop | Blocks | Data resident | Target cache |
|------|--------|---------------|-------------|
| 5th (JC) | nc columns | kc × nc panel of B | L3 |
| 4th (PC) | kc rows | kc × nc panel of B packed | L3 |
| 3rd (IC) | mc rows | mc × kc block of A packed | L2 |
| 2nd (JR) | nr columns | kc × nr micropanel of B | L1 |
| 1st (IR) | mr rows | kc × mr micropanel of A | Registers |

The constraints:
- **kc × nr** (one micropanel of B) must fit in L1.
  Example: kc = 256, nr = 8, double precision (8 bytes).
  256 × 8 × 8 = 16 KB. Fits in 32 KB L1 with room for A micropanel.

- **mc × kc** (one packed block of A) must fit in L2.
  Example: mc = 128, kc = 256, double precision.
  128 × 256 × 8 = 256 KB. Fits in 256 KB L2 (or uses most of it).

- **kc × nc** (one packed panel of B) must fit in L3.
  Example: kc = 256, nc = 4096, double precision.
  256 × 4096 × 8 = 8 MB. Fits in a typical 8-20 MB L3 slice.

The general constraint formula:
```
(mc × kc + kc × nc) × sizeof(type) ≤ cache_size × utilization_factor
```

The utilization factor is typically 0.5-0.8, leaving room for other
cache residents (stack, instruction cache, OS data).

### 2.3 Data Packing

Before the micro-kernel runs, BLIS **packs** the relevant portions of
A and B into contiguous, aligned buffers. This is a critical step that
transforms arbitrary-stride source data into the exact layout the
micro-kernel expects.

**A packing (column-panel-wise):**
- The mc × kc block of A is copied into a contiguous buffer.
- Data is laid out in mr × kc panels, where each panel is stored
  column-by-column (for efficient register loading).
- Leading dimension = PACKMR (typically = mr).
- First panel aligned to 4096 bytes (page boundary); subsequent
  panels aligned to PACKMR × sizeof(type).

**B packing (row-panel-wise):**
- The kc × nc panel of B is copied into a contiguous buffer.
- Data is laid out in kc × nr panels, where each panel is stored
  row-by-row.
- Leading dimension = PACKNR (typically = nr).
- Alignment same as A.

**Why packing matters:**
- Eliminates TLB misses: packed data is contiguous, so page table
  entries are sequential.
- Enables vector loads: the micro-kernel can use aligned SIMD load
  instructions (e.g., `vmovapd` on x86) instead of gather operations.
- Removes stride arithmetic: the micro-kernel doesn't need to compute
  memory offsets for non-contiguous data.
- Converts one complex access pattern (arbitrary source strides) into
  the simplest possible pattern (sequential read).

The packing overhead is O(mc × kc + kc × nc) — proportional to the
data copied. For large matrices, this is amortised over the O(mc × nc
× kc) computation in the micro-kernel. The compute-to-pack ratio is
roughly nc/kc for A packing and mc/kc for B packing, both typically
> 10.

For polka-dots, our column data is ALREADY in contiguous arrays with
uniform stride (16 bytes per entry). This is analogous to pre-packed
data — no packing step is needed. The hardware prefetcher can stream
our columns directly into cache.

### 2.4 The Micro-Kernel

The micro-kernel is the innermost computation, operating on an mr × nr
tile of C. It performs:

```
C[0:mr, 0:nr] += A[0:mr, 0:kc] × B[0:kc, 0:nr]
```

This is a rank-kc update to an mr × nr register block. The
micro-kernel iterates over the k dimension (kc iterations), loading
one column of A (mr elements) and one row of B (nr elements), computing
their outer product, and accumulating into the C register block.

**Register allocation:** The mr × nr C values stay in registers
throughout the kc iterations. On x86 with 16 YMM registers (256-bit,
4 doubles each): mr = 6, nr = 8 requires 6 × 8 / 4 = 12 registers
for C, leaving 4 for A, B, and temporaries.

**The micro-kernel is the only architecture-specific code.** It is
typically written in assembly (or compiler intrinsics) for each target
microarchitecture. BLIS provides reference C micro-kernels as
fallbacks, but performance-critical deployments use hand-tuned assembly.

Typical micro-kernel sizes:

| Architecture | mr | nr | Registers used | Throughput |
|-------------|----|----|---------------|-----------|
| Haswell (AVX2) | 6 | 8 | 12 YMM + 4 temp | ~90% peak |
| Zen (AVX2) | 6 | 8 | 12 YMM + 4 temp | ~85% peak |
| SkylakeX (AVX-512) | 14 | 8 | 14 ZMM + 2 temp | ~90% peak |
| ARM NEON | 8 | 4 | 8 Q-reg + 4 temp | ~80% peak |
| Apple M1 (NEON) | 8 | 12 | architecture-specific | ~85% peak |

The micro-kernel achieves 80-95% of theoretical peak FLOPS. This is
remarkable — most code achieves 5-20% of peak. The entire BLIS
framework exists to ensure the micro-kernel always operates on
cache-resident, aligned, contiguous data.

### 2.5 Hardware Detection and Sub-Configurations

BLIS uses a **sub-configuration** system that maps microarchitectures
to kernel sets and blocking parameters.

**Detection flow:**
1. At library initialisation, `bli_cpuid_query_id()` runs CPUID
   (x86), `/proc/cpuinfo` parsing (ARM), or equivalent.
2. The detected microarchitecture maps to an `arch_t` enum value
   (e.g., `BLIS_ARCH_HASWELL`, `BLIS_ARCH_ZEN3`, `BLIS_ARCH_FIRESTORM`).
3. The `arch_t` selects a **context** (`cntx_t`) containing:
   - Function pointers to optimised micro-kernels
   - Architecture-specific blocking parameters (mc, nc, kc, mr, nr)
   - Alignment and memory allocation preferences
   - Level-1 and level-1f kernel pointers (for BLAS level-1 operations)

**Umbrella configurations.** A single BLIS binary can contain kernels
for multiple architectures (e.g., Haswell + Zen3 + SkylakeX). At
runtime, detection selects the appropriate sub-configuration. This
is the "umbrella family" concept — one binary, many kernels.

**Fallback.** If detection fails or the hardware is unrecognised, BLIS
falls back to reference C kernels with generic blocking parameters.
Performance degrades (~30-50% of optimised) but correctness is
preserved.

### 2.6 Hot-Swap Kernel Mechanism

BLIS supports changing the active kernel configuration at runtime
through function pointer indirection.

The mechanism:
- All kernel calls go through the context: `bli_cntx_get_l3_nat_ukr_dt()`
  returns the active micro-kernel for a given datatype.
- Changing the context changes ALL kernel pointers simultaneously.
- The portable API functions (e.g., `bli_dgemm()`) internally query
  the current context for the appropriate kernel.

This enables:
- **Runtime architecture selection** on heterogeneous hardware (e.g.,
  big.LITTLE, where different cores have different optimal kernels).
- **Testing** with reference kernels even when optimised kernels are
  available.
- **Dynamic tuning** where an application can select between kernels
  optimised for different workload sizes.

The overhead is one pointer indirection per micro-kernel invocation.
Since the micro-kernel processes an mr × nr × kc block (hundreds to
thousands of FLOPs), the overhead is negligible.

### 2.7 Thread Partitioning and Cache Topology

BLIS parallelises **four of the five loops**, with the thread
decomposition mapped directly to the cache topology:

| Loop | Env var | What it parallelises | Cache mapping |
|------|---------|---------------------|---------------|
| 5th (JC) | BLIS_JC_NT | n dimension columns of B | Different L3 regions (multi-socket) |
| 3rd (IC) | BLIS_IC_NT | m dimension rows of A | Shared L3 for B, private L2 for A blocks |
| 2nd (JR) | BLIS_JR_NT | n dimension micropanels | Shared L2 for A, private L1 for B micropanels |
| 1st (IR) | BLIS_IR_NT | m dimension micropanels | Typically 1 (rarely parallelised) |

The 4th loop (PC/KC) is NOT parallelised because it updates the shared
output matrix C. Parallelising it would require synchronisation
(atomics or locks) on every C update.

**Total threads = JC_NT × IC_NT × JR_NT × IR_NT.** This is a
hierarchical factorisation that mirrors the hardware topology:

- **Multi-socket (2+ sockets):** Parallelise JC across sockets. Each
  socket gets independent B panels → independent L3 working sets. No
  cross-socket data sharing in the outer loop.
- **Cores within a socket:** Parallelise IC. Threads share the packed B
  panel in L3 (read-only, no contention) and pack different A blocks
  into their private L2.
- **Hyperthreads or paired cores sharing L2:** Parallelise JR. Threads
  share the packed A block in L2 (read-only) and stream different B
  micropanels through their private L1.

**Thread binding:** BLIS recommends `GOMP_CPU_AFFINITY` or
`OMP_PROC_BIND=close` with `OMP_PLACES=cores`. The principle: "Fill
up a socket with one thread per core before moving to the next
socket." This maximises cache sharing at the appropriate levels.

**Configuration precedence:** Three levels, highest first:
1. Per-call `rntm_t` objects (passed to individual BLIS calls)
2. Global runtime API (`bli_thread_set_ways()`)
3. Environment variables (read once at init)

### 2.8 Prefetching Strategy

The micro-kernel receives an `auxinfo_t` structure containing
**prefetch hints**: pointers to the next A and B micropanels. This
enables the kernel to prefetch data for the next iteration while
computing the current one.

Software prefetching in BLIS:
- **A prefetch:** The current micro-kernel iteration prefetches the
  next mr-element column of A (for the next k iteration within the
  current micropanel) or the first column of the next micropanel.
- **B prefetch:** Similar, for the next nr-element row of B.
- **C prefetch:** Before the micro-kernel begins, C values are
  prefetched from memory into registers.

Hardware prefetching alone is insufficient for BLIS because the access
pattern within the micro-kernel (alternating between A and B with
non-trivial strides) is too complex for the hardware prefetcher to
detect reliably. Explicit software prefetch instructions
(`_mm_prefetch` on x86, `__builtin_prefetch` on GCC) guide the
hardware.

For polka-dots, hardware prefetching IS sufficient because our access
pattern (sequential scan through contiguous column arrays) is exactly
what hardware prefetchers are designed for. No software prefetching
needed.

### 2.9 Configuration System

BLIS configurations are stored in `config/` subdirectories, one per
architecture family. Each contains:

- `bli_cntx_init_<arch>.c` — context initialisation (kernel selection,
  blocking parameters)
- `bli_family_<family>.h` — family-level definitions
- Kernel source files (assembly or C intrinsics)

The build system (`configure` script) selects which configurations to
include. An umbrella build includes multiple configurations; a targeted
build includes only one.

Key configuration parameters per architecture:

```c
// Example: Haswell configuration
bli_cntx_set_blkszs(
    BLIS_NC, 4080,     // L3 blocking (n dimension)
    BLIS_KC, 256,      // L1/L2 boundary (k dimension)
    BLIS_MC, 144,      // L2 blocking (m dimension)
    BLIS_NR, 8,        // L1/register boundary (n micro)
    BLIS_MR, 6,        // Register blocking (m micro)
);
```

### 2.10 Key Constants and Typical Values

| Parameter | Haswell | Zen 3 | Apple M1 | Purpose |
|-----------|---------|-------|----------|---------|
| mr | 6 | 6 | 8 | Micro-kernel m dimension (register) |
| nr | 8 | 8 | 12 | Micro-kernel n dimension (register) |
| kc | 256 | 256 | 512 | L1/L2 blocking (k dimension) |
| mc | 144 | 144 | 240 | L2 blocking (m dimension) |
| nc | 4080 | 4080 | 2040 | L3 blocking (n dimension) |
| A block (mc×kc×8B) | 288 KB | 288 KB | 960 KB | L2 resident |
| B panel (kc×nr×8B) | 16 KB | 16 KB | 48 KB | L1 resident |

Apple M1 values are notably different: larger kc and mc (exploiting
the 128 KB L1 and larger L2), wider nr (exploiting the wider register
file), and smaller nc (smaller shared L3 per cluster).

### 2.11 Lessons for polka-dots

**What we adopted:**
- Runtime hardware detection at startup (BLIS-style CPUID/sysctl)
- Cache-aware sizing derived from detected parameters
- Fallback to safe defaults when detection fails

**What we adapted:**
- Single-level tiling (L1 only) instead of BLIS's three-level
  (L1/L2/L3). Our data volumes don't exceed L2.
- No data packing — our columnar data is already contiguous and
  uniform-stride. BLIS needs packing because source matrices have
  arbitrary strides.
- No hand-tuned assembly micro-kernels — our inner loop is simple
  enough for compiler autovectorisation.

**What we learned:**
- The hierarchical cache mapping (different loops → different cache
  levels) is elegant but overkill for our scale.
- The utilisation factor (not using 100% of cache) is important — BLIS
  uses 0.5-0.8, we use 0.75.
- Thread binding to physical cores matters for cache stability.
- The umbrella/fallback pattern (multiple code paths, runtime
  selection) is robust and worth emulating for platform detection.

---

## 3. Polars — Streaming DAG Execution

### 3.1 Architecture Overview

Polars' execution engine has evolved significantly. The current
streaming engine lives in the `polars-stream` crate and uses a
graph-based execution model. It replaces the earlier streaming engine
with a more flexible DAG-based approach.

The engine processes data as a stream of **morsels** (DataFrame
chunks) flowing through a graph of **compute nodes** connected by
**logical pipes**. This is structurally similar to a dataflow system
but with several Polars-specific adaptations for memory management
and ordering.

### 3.2 The Compute Node Graph

Computation is organised as a **DAG of compute nodes**. Each node
implements the `ComputeNode` trait with two key methods:

**`update_state()`** — A state machine that transitions the node
through its lifecycle. Port states follow a three-state machine:
`Blocked → Ready → Done` (no backwards transitions from `Done`).
The `update_all_states()` method propagates state changes until a
fixed point is reached, determining which subgraphs are executable.

**`spawn()`** — Creates async tasks for processing. Each invocation
produces one or more tasks that process morsels through the node's
logic.

The engine identifies executable subgraphs through topological sort,
starting from sinks and working backward. This ensures receive ports
initialise before send ports — a node can't produce data until its
downstream consumer is ready to receive.

### 3.3 Morsel Model

A `Morsel` in Polars contains:

**DataFrame** — the actual data payload. A morsel is a chunk of rows
from a larger dataset.

**MorselSeq** — a monotonically non-decreasing sequence number used
for ordering. Internally, the sequence number is doubled (shifted left
by 1) to reserve the LSB for future "final morsel" marking. This
ensures morsels can be reordered back into their original sequence
after parallel processing.

**SourceToken** — identifies the origin source. Used for flow control:
when a source needs to stop producing (e.g., LIMIT reached), the
source token enables targeted stop signaling rather than draining the
entire pipeline.

**WaitToken (optional)** — backpressure notification. When present,
the producer should wait for the token to be consumed before producing
more morsels.

**Default morsel size: 100,000 rows.** Defined as
`DEFAULT_IDEAL_MORSEL_SIZE = 100_000` in `crates/polars-config/src/lib.rs`.
This is configurable via the `POLARS_IDEAL_MORSEL_SIZE` environment
variable (with backwards-compatible alias `POLARS_STREAMING_CHUNK_SIZE`).

The morsel size is empirically chosen — it matches the morsel-driven
literature (HyPer's ~100K) but is not derived from cache size. Polars
does not perform runtime cache detection.

### 3.4 Parallelism: Partition-Based

Polars uses **partition-based parallelism**, which is fundamentally
different from DuckDB's morsel-driven dispatch:

**Static partitioning.** The number of parallel pipeline instances
equals the rayon thread pool's active thread count. Data is
pre-partitioned across these instances at pipeline construction time.

**One task per partition.** For each compute node, `spawn()` creates
one async task per pipeline partition (one per receiver-sender pair).
Within each partition, morsels flow sequentially through the node's
logic.

**No central dispatcher.** There is no dynamic morsel assignment. Each
partition processes its own morsel stream independently. Load
balancing is implicit: if partitions are evenly sized, threads finish
at roughly the same time. Skew is handled by the underlying rayon
work-stealing scheduler.

**Distributors and linearizers.** To connect the partitioned execution:
- A `Distributor` fans out morsels from a single source across N
  pipeline partitions (round-robin or hash-based).
- A `MorselLinearizer` reorders morsels back into sequence order
  (using a priority queue on MorselSeq) when downstream operations
  require ordered input.

This approach is simpler than DuckDB's dynamic dispatch but less
elastic — the parallelism degree is fixed at pipeline construction and
cannot change mid-execution. For Polars' batch analytics workloads,
this is acceptable.

### 3.5 Pipeline Blockers and Scheduling

The streaming scheduler uses **memory-aware scheduling**:

**Pipeline blockers.** Nodes declare themselves as pipeline blockers
via `is_memory_intensive_pipeline_blocker()`. The scheduler
distinguishes:
- **Expensive blockers** (memory-intensive): hash join build, group-by
  aggregation, sort. These accumulate large amounts of state.
- **Cheap operations** (streaming): filter, map, select, projection,
  slice. These process morsels in-place without accumulation.

**Scheduling strategy.** The scheduler preferentially executes blockers
whose outputs are ready for consumption by downstream nodes. This
minimises peak memory usage — rather than building all hash tables
simultaneously, the scheduler completes one join before starting the
next (if possible).

Known pipeline-blocking nodes:
- `in_memory_map`, `in_memory_source`, `in_memory_sink`
- Hash join (build side)
- `dynamic_group_by`, `rolling_group_by`
- `merge_sorted`

Known streaming (non-blocking) nodes:
- `filter`, `map`, `select`, `simple_projection`
- `with_row_index`
- `ordered_union`
- `streaming_slice`

**Execution loop.** The engine loops:
1. Update all node states (propagate `Blocked → Ready → Done`)
2. Find runnable subgraphs (nodes with all inputs ready)
3. Execute via spawned async tasks
4. Wait for completion
5. Repeat until all nodes are `Done`

### 3.6 Operator Fusion (Structural)

Polars does NOT fuse operators in the compiled/JIT sense (like HyPer's
code generation or Spark's Tungsten). Instead, fusion is **structural**:

Streaming operators pass morsels through without full materialisation.
The `update_state` method in streaming nodes like `map` and `filter`
performs `recv.swap_with_slice(send)`, directly wiring input port state
to output port state. The node processes morsels in-place as they flow
through, without buffering.

Within a single node, operations apply sequential processing within
the morsel: `df.filter_seq()` rather than `df.filter()`. This avoids
redundant intra-morsel parallelism — the inter-morsel parallelism
across pipeline partitions is sufficient.

The practical effect is similar to pipeline fusion: data flows through
multiple operators without intermediate materialisation to disk or to
a separate memory region. But the operators are still separate objects
with separate function calls — there's no code generation that merges
them into a single tight loop.

### 3.7 Backpressure and Flow Control

Polars has explicit backpressure mechanisms:

**Consume tokens.** A pipe's consume token is dropped only after a
send succeeds (for linearizers) or before entering the distributor.
This prevents downstream congestion — if the receiver is slow, the
sender blocks on the consume token.

**Buffer sizes.** Pipes have configurable buffer sizes:
- `DEFAULT_LINEARIZER_BUFFER_SIZE` — buffering capacity for the
  reordering queue
- `DEFAULT_DISTRIBUTOR_BUFFER_SIZE` — buffering capacity for the
  fan-out queue

**Source stop signaling.** When a LIMIT is reached, the engine signals
the source via the `SourceToken` to stop producing morsels. This
avoids processing unnecessary data.

**PipeMetrics.** Each pipe tracks `morsels_sent`, `rows_sent`, and
`largest_morsel_sent` for monitoring and debugging.

### 3.8 Memory Management and Out-of-Core

Polars supports out-of-core execution for datasets larger than
available RAM:

**Morsel spilling.** Morsels can be stored via the memory manager
(`into_token()`) and retrieved later. The memory manager decides when
to spill based on memory pressure.

**Spill policy.** Configurable via `POLARS_OOC_SPILL_POLICY`
environment variable. Options include eager spilling (spill early to
keep memory low) and lazy spilling (only spill when memory pressure
is high).

**Spill format.** Configurable via `POLARS_OOC_SPILL_FORMAT` (default:
IPC/Arrow format). Morsels are serialised to disk and deserialised
when needed.

### 3.9 Morsel Ordering and Determinism

Polars maintains morsel ordering through several mechanisms:

**MorselSeq.** Each morsel carries a monotonically non-decreasing
sequence number. Morsels from the same source maintain their relative
order.

**MorselLinearizer.** A priority queue that reorders morsels back into
sequence order. Placed at points where ordered output is required
(e.g., before ORDER BY, before output).

**Ordered operations.** Some operations (`ordered_union`,
`merge_sorted`) explicitly maintain order invariants.

For unordered operations (hash join probe, filter), the engine does
not guarantee morsel order within a partition. The linearizer restores
order when needed.

### 3.10 Key Constants and Configuration

| Constant | Value | Purpose | Configurable |
|----------|-------|---------|-------------|
| DEFAULT_IDEAL_MORSEL_SIZE | 100,000 | Morsel row count | POLARS_IDEAL_MORSEL_SIZE env |
| Pipeline partitions | rayon thread count | Parallelism degree | RAYON_NUM_THREADS env |
| Linearizer buffer | default varies | Reordering queue capacity | Code-level |
| Distributor buffer | default varies | Fan-out queue capacity | Code-level |
| Spill format | IPC | Out-of-core serialisation | POLARS_OOC_SPILL_FORMAT env |

### 3.11 Lessons for polka-dots

**What we adopted:**
- DAG-based scheduling (our Schedule constructs a dependency DAG)
- Pipeline blocker concept (our pipeline breakers)
- Priority-based scheduling (their "execute blockers whose outputs
  are ready" ≈ our "unblocks most" priority)

**What we rejected:**
- Partition-based parallelism in favor of morsel-driven dispatch
  (more flexible, better load balancing)
- No cache-aware sizing — we do runtime L1 detection instead

**What is open:**
- Backpressure (Polars has it, we haven't decided)
- Morsel ordering (Polars needs it for SQL semantics, we may need it
  for determinism)
- Out-of-core (Polars has it, probably unnecessary for us)
- Memory-aware blocker scheduling (interesting, not yet evaluated)

---

## 4. Apache Arrow DataFusion — Extended Survey

### 4.1 Execution Model

DataFusion is Apache Arrow's query execution framework, written in
Rust. It uses a **pull-based** execution model (Volcano-style) where
each operator implements `ExecutionPlan::execute()` returning a
`SendableRecordBatchStream` — a stream of Arrow RecordBatches.

The execution is **partition-aware**: each ExecutionPlan declares how
many output partitions it produces. Downstream operators can request
specific partitions. Repartitioning operators (hash repartition,
round-robin) redistribute data across partitions.

**The `ExecutionPlan` trait** is the central abstraction:

```rust
pub trait ExecutionPlan: Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn schema(&self) -> SchemaRef;
    fn output_partitioning(&self) -> Partitioning;
    fn output_ordering(&self) -> Option<&[PhysicalSortExpr]>;
    fn children(&self) -> Vec<Arc<dyn ExecutionPlan>>;
    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream>;
    fn statistics(&self) -> Statistics;
    fn required_input_distribution(&self) -> Vec<Distribution>;
    fn required_input_ordering(&self) -> Vec<Option<Vec<PhysicalSortRequirement>>>;
}
```

Several details are relevant to polka-dots:

**Partition-awareness is declared, not computed.** Each operator
declares its output partitioning (Hash, RoundRobin, UnknownPartitioning).
The optimizer uses this to decide whether repartitioning is needed. This
is analogous to our Schedule computing chain assignments from the DAG
topology — the operator declares its data distribution requirements,
the framework handles placement.

**The `execute()` call is per-partition.** The caller passes a partition
index, and the operator returns a stream for that specific partition.
This means parallelism is driven by the caller requesting multiple
partitions, not by the operator internally spawning threads.

**Statistics propagation.** Each operator can propagate row count
estimates, column statistics, and selectivity information upward. The
optimizer uses these for cost-based decisions. polka-dots doesn't need
this (our data volumes are known at Schedule construction time), but the
pattern of propagating metadata through the plan tree is similar to our
column count propagation through chain partitioning.

### 4.2 Batch Size and Memory Management

Default batch size: **8,192 rows.** This is the number of rows per
RecordBatch in the output stream. Configurable via
`SessionConfig::with_batch_size()`.

The batch size is NOT cache-aware — it's a configuration parameter
chosen for reasonable default performance. Users tuning for specific
workloads can adjust it.

DataFusion does not detect hardware parameters or adapt batch sizes
to cache hierarchies.

**Memory management** is handled through a `MemoryPool` abstraction:

```rust
pub trait MemoryPool: Send + Sync {
    fn register(&self, consumer: &MemoryConsumer);
    fn unregister(&self, consumer: &MemoryConsumer);
    fn grow(&self, reservation: &MemoryReservation, additional: usize);
    fn shrink(&self, reservation: &MemoryReservation, shrink: usize);
    fn try_grow(
        &self,
        reservation: &MemoryReservation,
        additional: usize,
    ) -> Result<()>;
}
```

Two built-in implementations:
- **`UnboundedMemoryPool`** — no limits, always succeeds. The default.
- **`GreedyMemoryPool`** — tracks total usage, returns error when
  limit is exceeded. First-come-first-served with no fairness.
- **`FairSpillPool`** — reserves a fraction of memory for each
  consumer, with spill-capable operators given less guaranteed memory
  (because they can spill). Non-spillable operators get guaranteed
  allocations up to `total / num_consumers`.

The memory pool is passed through `TaskContext`, which is available to
every operator during execution. Operators that accumulate state (hash
joins, sorts, aggregations) call `try_grow()` before allocating memory.
If the pool rejects the allocation, the operator must spill to disk.

This is more sophisticated than DuckDB's fixed formula
`(max_memory / threads) / 4` — it's dynamic and accounts for the
actual number of active operators and their spill capability.

For polka-dots, the `ColumnStorage` trait serves a similar role: it
abstracts memory allocation so the consumer controls the strategy.
DataFusion's FairSpillPool pattern is informative for a potential
future memory budget implementation.

### 4.3 Partitioning and Repartitioning

DataFusion's parallelism is partition-based:
- Each ExecutionPlan node declares its output partitioning
- The optimizer inserts repartitioning operators where needed
- Each partition is processed by a separate tokio task
- No morsel-driven dynamic dispatch

**Partitioning types:**
- `Partitioning::RoundRobinBatch(n)` — rows distributed round-robin
- `Partitioning::Hash(exprs, n)` — rows hash-partitioned on expressions
- `Partitioning::UnknownPartitioning(n)` — n partitions, no guarantees

**Repartitioning operators** insert when:
- A hash join requires both inputs hash-partitioned on the join key
- An aggregate needs data grouped by key on the same partition
- The configured `target_partitions` differs from the current count

The `RepartitionExec` operator reads from all input partitions and
redistributes to output partitions. It uses tokio channels for
cross-partition data transfer, with configurable sort-preserving
behavior.

**Target partitions.** The `SessionConfig::target_partitions` setting
(default: number of CPU cores) determines how many partitions the
optimizer targets. This is the primary parallelism knob — increasing
it increases parallelism but also increases repartitioning overhead and
memory usage.

### 4.4 Operator State and Lifecycle

DataFusion operators follow a particular lifecycle pattern:

**Stateless operators** (filter, projection) process each RecordBatch
independently. They implement `execute()` by wrapping the child stream
with a transformation closure.

**Stateful operators** (hash join, sort, aggregate) must accumulate
input before producing output. The pattern:

1. `execute()` creates an async stream
2. The stream's first poll drives the operator to consume all input
   from the child stream
3. Once input is exhausted, the operator transitions to producing output
4. Subsequent polls yield output RecordBatches

This is the async equivalent of DuckDB's pipeline breakers: the
operator blocks the pull chain while accumulating state, then becomes
a source for the next phase.

**The `RecordBatch` as data unit.** All data flows as Arrow
RecordBatches — columnar data with a shared schema. This is fixed-size
in row count (batch_size) but variable in byte size (depends on column
types and null density). This is different from polka-dots where our
stride is fixed-size in bytes (16 bytes per entry), making memory
consumption predictable per morsel.

### 4.5 The Optimizer Pipeline

DataFusion's optimizer is worth noting for its structure, even though
polka-dots doesn't have a query optimizer:

1. **Logical plan** — abstract expression tree (SQL → LogicalPlan)
2. **Analyzer** — resolves types, validates schemas
3. **Optimizer** — rewrites logical plan (predicate pushdown, projection
   pruning, common subexpression elimination)
4. **Physical planner** — converts logical plan to ExecutionPlan tree
5. **Physical optimizer** — inserts repartitioning, sorts, and
   coalescing operators based on physical requirements

The physical optimizer's "ensure requirements" pass is structurally
similar to our chain partitioning: both walk a DAG and insert boundaries
(repartitioning / chain breaks) based on resource constraints.

### 4.6 Lessons for polka-dots

**Configuration vs detection.** DataFusion uses configuration-based
sizing (batch_size, target_partitions) while we use runtime detection.
DataFusion serves database developers who understand their workloads;
we serve dotfiles users who should not need to tune parameters.

**Memory pool abstraction.** DataFusion's `MemoryPool` trait with
FairSpillPool is a good model for future memory budgeting. The pattern
of separating spillable from non-spillable consumers is insightful —
in our model, this could mean treating loader chains (which control
input size) differently from transform chains (which have bounded
intermediate state).

**Partition-awareness as metadata.** The pattern of operators declaring
their partitioning requirements (rather than the framework computing
them) simplifies the planner. In our model, WorkUnits declare their
Read/Write column sets, and the Schedule computes chain partitioning —
similar separation of declaration from placement.

**The RecordBatch model.** Arrow's RecordBatch is variable-size in
bytes, which complicates memory budgeting. Our fixed 16-byte stride
makes memory consumption perfectly predictable: `rows × columns × 16`
bytes per morsel. This is a significant advantage for cache fitting.

---

## 5. fjall — Rust-Specific Patterns

### 5.1 Block Sizing

fjall is an LSM-tree storage engine in Rust. Its block sizes are
fixed and configurable:
- Default block size: 4 KiB
- Configurable range: 4-64 KiB
- Set via `Config::block_size()`

No runtime cache detection. No page size detection. The block size
is a configuration parameter that the user sets based on their
workload (point lookups prefer smaller blocks, range scans prefer
larger blocks).

fjall relies on the OS kernel for page mapping — it does not use
`mmap` or custom memory management for block I/O. The block cache
is a simple LRU with configurable capacity via `Config::cache_size()`.

### 5.2 ByteView Pattern

The most relevant pattern from fjall for polka-dots is **ByteView**:

```rust
struct ByteView {
    // 24 bytes total
    len: u32,
    inline_or_ptr: [u8; 20], // if len <= 20: inline data
                               // if len > 20: pointer + metadata
}
```

For values ≤ 20 bytes, the data is stored inline in the struct.
For values > 20 bytes, the struct contains a pointer to heap-allocated
data plus metadata (offset, length).

This is conceptually identical to our 128-bit stride with 15-byte
inline payload. The key insight is the same: most values in practice
are small, and inline storage eliminates pointer chasing for the
common case.

fjall's ByteView is 24 bytes (larger than our 16-byte stride) because
it stores the length explicitly and supports variable-length values
up to 20 bytes inline. Our design uses a fixed payload size (15 bytes)
with the type information in the column metadata (not per-entry),
which allows a smaller stride.

**Comparison of inline value representations:**

| System | Stride | Inline capacity | Length stored | Type info |
|--------|--------|----------------|---------------|-----------|
| fjall ByteView | 24 bytes | 20 bytes | Per-entry (u32) | Per-entry (implicit in mode) |
| Arrow StringView | 16 bytes | 12 bytes | Per-entry (u32) | Per-column (schema) |
| polka-dots Stride | 16 bytes | 15 bytes | Per-column (fixed) | Per-column (Column metadata) |
| DuckDB string_t | 16 bytes | 12 bytes | Per-entry (u32) | Per-column (LogicalType) |

Arrow's StringView (used in arrow-rs and DuckDB) is particularly
relevant. It uses 16 bytes: 4-byte length + 4-byte prefix (for
comparison shortcuts) + either 8 bytes inline data OR 4-byte buffer
index + 4-byte offset. This is closer to our design in total size.

The key design difference: Arrow and fjall store the length per entry
because their values have variable lengths within a column. Our columns
have uniform entry sizes (determined by the column type), so we store
the type once in the column metadata and use the full 15 bytes for
payload. This is possible because our domain doesn't have variable-
length strings as a primary data type — our values are names,
versions, booleans, small integers, and enum variants, all ≤ 15 bytes.

### 5.3 Configuration Model

fjall uses a **builder pattern** for configuration:

```rust
let config = Config::new()
    .block_size(16_384)
    .cache_size(128 * 1024 * 1024)
    .compaction_strategy(CompactionStrategy::Leveled);
```

All parameters have sensible defaults. No runtime detection. The user
tunes for their workload if needed.

### 5.4 Compaction and Background Work

fjall uses background compaction threads to merge and compact SST files.
This is structurally similar to pipeline-parallel execution: the main
thread handles writes while compaction threads process accumulated data
in the background.

Key patterns:
- **Write amplification awareness.** Level-based compaction has known
  write amplification ratios. fjall exposes these as configuration
  parameters (`level_ratio`, `l0_threshold`). For polka-dots, the
  analogous concern is "how many times is each data entry touched
  across chains?" — ideally once per chain (no redundant copies).

- **Monotonic sequence numbers.** Each write operation gets a
  monotonically increasing sequence number for ordering. This is
  similar to our AtomicUsize progress counters — both use monotonic
  integers for ordering without locks.

- **Manifest tracking.** fjall tracks the current set of SST files
  and their metadata in a manifest file. Our BuildManifest serves a
  similar purpose — tracking what exists, what changed, and what needs
  reprocessing.

### 5.5 Lessons for polka-dots

**ByteView pattern validation.** Our 128-bit stride with 15-byte
inline payload is a well-known pattern independently discovered by
fjall (and Arrow). This gives confidence that the approach is sound.

**Configuration vs detection.** fjall uses configuration because its
users (database developers) understand their workloads. polka-dots
uses detection because its users (dotfiles managers) should not need
to tune cache parameters.

**Monotonic counters as coordination primitive.** Both fjall
(sequence numbers) and our design (AtomicUsize progress) use the
same lightweight coordination pattern: monotonically increasing
integers that can be compared without locks.

---

## 6. Comparative Analysis

### 6.1 Scheduling Models Compared

| System | Model | Dispatch | Elasticity |
|--------|-------|----------|-----------|
| DuckDB | Morsel-driven | Lock-free inline dispatcher | Full: threads reassigned at morsel boundaries |
| BLIS | Static loop | Compile/run-time loop decomposition | None: static per-call |
| Polars | Partition-based | Pre-partitioned streams | Limited: fixed partition count |
| DataFusion | Partition-based | Tokio task per partition | Limited: fixed partition count |
| polka-dots | Chain-driven | Thread pool, one chain per core | Limited: fixed chain assignment |

DuckDB has the most flexible scheduling — threads can be reassigned
between queries and between morsels within a query. BLIS is the most
rigid — parallelism is determined at call time and cannot change.

polka-dots falls between: chains are assigned to cores at dispatch
time, but within a chain, morsels are processed sequentially. The
parallelism is between chains, not within chains. This is simpler
than DuckDB's fully dynamic morsel dispatch but provides the
parallelism we need (independent DAG branches on separate cores).

### 6.2 Cache Strategies Compared

| System | Detection | Levels | Sizing |
|--------|-----------|--------|--------|
| DuckDB | None | L1 implicit (vector size) | Fixed compile-time constant |
| BLIS | CPUID + sysctl + sysfs | L1, L2, L3 | Per-architecture blocking parameters |
| Polars | None | None | Empirical morsel size |
| DataFusion | None | None | Configurable batch size |
| polka-dots | CPUID + sysctl + sysfs | L1 | Runtime formula from detected L1 |

BLIS is the gold standard for cache awareness — three cache levels,
per-architecture parameters, measured utilisation factors. DuckDB and
Polars don't detect cache at all — they use fixed constants that were
chosen once and work well enough.

polka-dots sits between: runtime detection like BLIS, but only one
cache level (L1). This is justified by our data volumes — our working
sets fit in L1, so L2/L3 tiling provides no benefit.

### 6.3 Fusion Models Compared

| System | Fusion type | Boundary |
|--------|------------|----------|
| DuckDB | Push-based pipeline | Pipeline breakers (hash build, sort, aggregate) |
| BLIS | Loop fusion in micro-kernel | Fixed five-loop structure |
| Polars | Structural (port wiring) | Pipeline blockers (memory-intensive ops) |
| polka-dots | Fused chains (ops back-to-back per morsel) | Fan-in, max_cols, pipeline breakers |

DuckDB's fusion is the most aggressive for databases — operators
within a pipeline share no intermediate materialisation, and the
vector stays in L1 through the entire chain. HyPer goes further with
code generation (JIT-compiling the pipeline into a single function),
but DuckDB uses interpreted vectorised execution instead.

polka-dots' fusion is structurally similar to DuckDB's: ops within
a chain run back-to-back on the same morsel, with no intermediate
materialisation. The main difference is our additional break condition
(max_cols) which DuckDB doesn't need because its vector size is fixed
and small enough that many columns fit.

### 6.4 Parallelism Models Compared

| System | Between pipelines | Within a pipeline | Granularity |
|--------|------------------|------------------|-------------|
| DuckDB | Dependency DAG | Morsel-driven (dynamic) | ~122K rows |
| BLIS | Not applicable | Hierarchical loop factorisation | Cache-level blocks |
| Polars | DAG-based | Partition-based (static) | ~100K rows |
| polka-dots | Chain dependency DAG | One chain per core (static) | Variable morsel size |

DuckDB has the finest-grained parallelism: multiple threads process
different morsels of the same pipeline simultaneously, with dynamic
load balancing. Polars has static parallelism within a pipeline: N
partitions, one per thread, no rebalancing.

polka-dots has no parallelism WITHIN a chain — each chain runs on
one core, processing morsels sequentially. Parallelism is only
BETWEEN chains (independent branches or pipeline-parallel dependent
chains). This is the simplest model and is correct for our data
volumes, where the total row count per chain may be only a few
thousand rows.

### 6.5 Memory Management Compared

| System | Budget | Spill | Allocator |
|--------|--------|-------|-----------|
| DuckDB | (max_mem / threads) / 4 per op | Yes (temp directory) | jemalloc (Linux) |
| BLIS | Explicit pack buffer allocation | No | malloc with alignment |
| Polars | Memory-aware scheduler | Yes (IPC format) | System allocator |
| polka-dots | Unspecified | Unspecified | ColumnStorage trait |

DuckDB and Polars both have memory management strategies because they
process arbitrarily large datasets. BLIS allocates fixed-size pack
buffers proportional to cache sizes — predictable and bounded.

polka-dots has not yet addressed memory management. The ColumnStorage
trait allows the consumer to implement any strategy, but the library
doesn't enforce limits or provide spill support. Given our data
volumes (KB to low MB), memory management is unlikely to be critical,
but a configurable ceiling is good practice.

### 6.6 Configuration and Adaptation Compared

| System | Adaptation approach | User configuration |
|--------|-------------------|-------------------|
| DuckDB | Compile-time constant + runtime adaptive join/agg | SET threads, SET memory_limit |
| BLIS | Full runtime detection per architecture | Env vars, API, per-call overrides |
| Polars | Env vars for morsel size | POLARS_IDEAL_MORSEL_SIZE, RAYON_NUM_THREADS |
| DataFusion | SessionConfig builder | batch_size, target_partitions |
| polka-dots | Runtime detection, library handles all | ColumnStorage trait, Executor trait override |

BLIS has the most sophisticated adaptation: per-microarchitecture
kernel selection, per-cache-level blocking, and three configuration
levels. DuckDB and Polars are less adaptive — they use fixed constants
with optional user overrides.

polka-dots aims for BLIS-level detection with DuckDB-level user
simplicity: detect hardware automatically, derive optimal parameters,
present a single `Pipeline::run()` entry point. Advanced users can
override via the Executor trait.

---

## 7. Cross-Cutting Concerns

### 7.1 Error Handling Across Systems

Each system handles errors differently during execution, and the
strategies reveal fundamental design trade-offs:

**DuckDB — Fail-fast with thread coordination.**
When an operator encounters an error during pipeline execution, the
error is propagated upward through the task return value. The executor
marks the pipeline as failed and prevents further morsel dispatch. All
active threads processing morsels for the failed pipeline see the
failure on their next morsel request and wind down.

Key detail: partially-processed morsels are discarded. DuckDB does
not attempt to recover partial results — the entire query fails. This
is correct for database semantics (transactions are atomic), but might
not be what polka-dots wants (a build failure on one app shouldn't
necessarily fail the entire pipeline).

**BLIS — Error returns at API boundary.**
BLIS functions return `void` (they don't fail) or `err_t` error codes.
Internal failures (allocation failure, invalid parameters) are caught
at the API boundary. Once execution begins, the inner loops cannot
fail — all memory is pre-allocated, data is pre-packed, and the
micro-kernel operates on guaranteed-valid data.

This is the most robust model: eliminate error conditions before the
hot path starts. polka-dots can partially emulate this — ColumnStorage
allocation and validation happen before chain execution begins.

**Polars — Async error propagation.**
Errors in Polars' streaming engine propagate through the async task
system. A failed task's error surfaces when the engine joins all tasks
at a synchronisation point. The error type is `PolarsError` which
includes context about which operation and which data triggered the
failure.

Polars uses `anyhow`-style error chaining: inner errors are wrapped
with context as they propagate. This is similar to polka-dots' use of
`anyhow`.

**DataFusion — Result<RecordBatch> streams.**
Every RecordBatch in the stream is wrapped in `Result<>`. An error in
any RecordBatch terminates the stream. Downstream operators that pull
from the stream see the error on the next poll and propagate it upward.

This is the simplest model and works well with Rust's `?` operator.
But it means error detection is lazy — a downstream operator might
process several successful batches before encountering the error from
an earlier upstream batch that propagated slowly.

**Implications for polka-dots:**

Our chain model has a natural error boundary: the chain. Options:

1. **Fail chain, continue siblings.** If LoadCatalogue fails, the
   MergeSources chain (which doesn't depend on it) can still complete.
   Only chains that transitively depend on the failed chain are skipped.
   This is the most useful for build pipelines — partial results are
   often better than no results.

2. **Fail chain, fail all downstream.** The failed chain poisons its
   progress counter (e.g., store `usize::MAX`). Dependent chains see
   this sentinel and skip execution. Unrelated chains complete normally.

3. **Fail chain, fail pipeline.** DuckDB's approach. Any chain failure
   aborts everything. Simplest to implement, least useful for builds.

This is Q10 in the dispatch-and-optimisation topic.

### 7.2 Cancellation and Early Termination

**DuckDB — Interrupt support.**
DuckDB supports query cancellation via `duckdb_interrupt()`. The
interrupt flag is checked at morsel boundaries — when a thread finishes
a morsel, it checks the interrupt flag before requesting the next one.
This provides bounded cancellation latency: at most one morsel's
processing time.

DuckDB also supports LIMIT pushdown: when a LIMIT is reached, the
source operator stops producing morsels. This is cooperative — the
source checks whether the sink has received enough rows.

**Polars — SourceToken stop signaling.**
Polars' `SourceToken` enables targeted source shutdown. When a LIMIT
node has accumulated enough rows, it signals the source to stop via
the token. The source sees this signal on the next morsel produce
attempt and returns `Done`.

This is more targeted than DuckDB's global interrupt — only the
specific source is stopped, other sources in the same graph continue.

**BLIS — No cancellation.**
BLIS operations run to completion. There is no mechanism to cancel a
GEMM mid-computation. For long-running operations on large matrices,
this means the caller must wait.

**DataFusion — Stream drop.**
Cancellation in DataFusion happens implicitly when the stream consumer
drops the stream. Tokio tasks are cancelled via their JoinHandle.
However, in-flight batches in progress may not be interrupted — the
task runs to the next await point and then sees the cancellation.

**Implications for polka-dots:**

For dotfiles builds, cancellation is useful (user presses Ctrl-C).
The morsel-boundary check approach (DuckDB) maps cleanly to our model:
check a shared `cancelled: AtomicBool` at each morsel boundary.
Bounded latency: one morsel's processing time (microseconds to low
milliseconds at our data volumes).

### 7.3 Progress Reporting

**DuckDB — QueryProgress.**
DuckDB exposes `duckdb_query_progress()` which returns a percentage.
Internally, this is computed from the ratio of processed morsels to
total morsels in the pipeline source. For complex DAGs with multiple
pipelines, the progress is weighted by estimated cost.

**Polars — PipeMetrics.**
Each pipe in the streaming engine tracks `morsels_sent`, `rows_sent`,
and `largest_morsel_sent`. These are available for monitoring but not
exposed as a user-facing progress API.

**BLIS — No progress.**
BLIS does not report progress. The operation completes or it doesn't.

**DataFusion — No built-in progress.**
DataFusion has no progress reporting API. The caller can count
RecordBatches as they arrive from the stream, but this requires
knowing the total count upfront.

**Implications for polka-dots:**

Our AtomicUsize progress counters already contain enough information
for progress reporting: `sum(progress[chain]) / sum(total_rows[chain])`
gives an aggregate progress percentage. The dispatch loop has this
information naturally. Exposing it to the CLI layer for a progress
bar is straightforward — no additional bookkeeping needed.

### 7.4 Resource Cleanup

**DuckDB — RAII with explicit finalize.**
Pipeline state (hash tables, sort buffers) is owned by the pipeline
executor and dropped when the pipeline completes. The `Finalize()`
method on sink operators performs any cleanup that must happen after
all morsels are processed (e.g., combining thread-local hash tables
into a single table).

**Polars — Async task completion.**
Resources are dropped when async tasks complete. The `ComputeNode`'s
state is owned by the task closure and dropped when the task finishes.

**BLIS — Explicit buffer free.**
Pack buffers allocated by `bli_packm_alloc()` are freed by
`bli_packm_release_cached()`. The lifecycle is managed by the
framework loop, not by individual kernels.

**DataFusion — Arc-based ownership.**
Operators and their state are wrapped in `Arc<>` and shared across
partition tasks. The state is dropped when the last Arc reference is
released (when all partition tasks complete).

**Implications for polka-dots:**

Column data is owned by the ColumnStorage implementation, not by the
executor or the chains. This means cleanup is the consumer's
responsibility — when `Pipeline::run()` returns, the consumer can
read output columns and then drop the ColumnStorage. The library
never allocates, so it never needs to free.

For intermediate state (if any chain maintains local state across
morsel boundaries), Rust's ownership model handles cleanup
automatically — the chain's local state is dropped when the chain
closure completes.

### 7.5 Determinism and Reproducibility

**DuckDB — Non-deterministic by default.**
Morsel assignment to threads is non-deterministic. Different runs may
process morsels in different orders on different threads. For queries
with ORDER BY, the sort operator restores deterministic output. For
queries without ORDER BY, result order is non-deterministic.

Hash table operations (hash join, hash aggregate) use a fixed seed,
so the same data produces the same hash table layout — but the order
in which threads insert into the hash table is non-deterministic.

**Polars — Deterministic via MorselSeq.**
Polars maintains deterministic output ordering through MorselSeq
numbers and the linearizer. Even though morsels are processed in
parallel across partitions, the linearizer reassembles them in
sequence order before output.

This determinism has a cost: the linearizer introduces a
synchronisation point (priority queue) and may cause head-of-line
blocking (morsel 5 can't be output until morsel 4 arrives, even if
5 is already computed).

**BLIS — Fully deterministic.**
For a given input and blocking configuration, BLIS produces
bit-identical results across runs. The loop nest is deterministic,
and floating-point operations within the micro-kernel follow a fixed
accumulation order.

Note: results may differ between configurations (different
blocking → different accumulation order → different floating-point
rounding). But within one configuration, results are reproducible.

**DataFusion — Non-deterministic partition assignment.**
Like DuckDB, partition assignment and processing order are non-
deterministic. The tokio runtime schedules tasks based on availability.

**Implications for polka-dots:**

Our model is deterministic by construction: each chain runs on one
core, processing morsels sequentially. Two runs with the same input
and same Schedule produce identical output. This is a desirable
property for a build tool — deterministic builds are easier to debug
and reason about.

If we ever support intra-chain parallelism (multiple cores processing
different morsels of the same chain), we'd need Polars-style morsel
sequencing. The MorselSeq + linearizer pattern is well-understood
but adds complexity and latency. This is Q6 in the dispatch topic.

### 7.6 Thread Pool Models

**DuckDB — Dedicated thread pool.**
DuckDB manages its own thread pool (`TaskScheduler::threads_`). Threads
are created at startup and persist for the lifetime of the database.
The pool size defaults to hardware concurrency but is configurable.

Worker threads wait on a condition variable when no tasks are
available. Task submission signals sleeping threads. This is a
classic producer-consumer pattern.

**Polars — Rayon thread pool.**
Polars delegates to rayon's global thread pool for CPU work and uses
tokio for async I/O coordination. The streaming engine spawns async
tasks that internally dispatch to rayon for compute-heavy work.

This dual-runtime approach is common in Rust data processing:
- rayon: deterministic parallelism, work-stealing, compute tasks
- tokio: async I/O, event-driven scheduling, network/disk tasks

**BLIS — OpenMP/pthreads.**
BLIS uses OpenMP for parallelism by default (configurable to pthreads
at build time). The parallelism is expressed as `#pragma omp parallel`
around loop nests. OpenMP handles thread pool management, barrier
synchronisation, and work distribution.

The overhead model is well-understood: OpenMP parallel regions have
~1-5 µs overhead on modern hardware. This is amortised across the
kc × nc × mc computation in each parallel region.

**DataFusion — Tokio runtime.**
DataFusion runs entirely on the tokio async runtime. CPU-bound work
(filter evaluation, hash computation) runs on tokio worker threads.
This can be suboptimal — tokio is designed for I/O-bound workloads,
and long-running CPU tasks can starve other tasks.

DataFusion mitigates this by yielding cooperatively at batch
boundaries. The `RecordBatchStream` implementation calls `yield_now()`
periodically to give other tasks a chance to run.

**Implications for polka-dots:**

We should NOT require a specific runtime. The Executor trait lets
the consumer provide their own threading model. The default Executor
could use a simple `std::thread::scope()` with one thread per physical
core. No rayon, no tokio, no OpenMP — just OS threads with the
simplest possible coordination (AtomicUsize progress + the dispatch
loop).

For consumers who want rayon or tokio integration, a custom Executor
can bridge to their runtime. The key contract is: the Executor calls
the provided chain closures, one per chain, respecting the dependency
order given by the Schedule.

---

## 8. Implications for polka-dots

### 8.1 What We Adopted

| Feature | Source | How we adapted |
|---------|--------|---------------|
| Morsel-driven batching | DuckDB/HyPer | Variable morsel size per chain (not fixed constant) |
| Operator fusion into pipelines | DuckDB | Fused chains with back-to-back ops per morsel |
| Pipeline dependency DAG | DuckDB | AtomicUsize per chain for progress tracking |
| Runtime cache detection | BLIS | L1 only (not L2/L3), min across core types |
| Cache-aware blocking | BLIS | Morsel sized to fit union columns in L1 |
| Hardware fallback | BLIS | 32 KB L1, 1 core as safe minimum |
| DAG-based scheduling | Polars | Two-step: DAG construction → chain partitioning |
| Pipeline breakers | DuckDB + Polars | Fan-in, max_cols, pipeline breakers force chain breaks |
| Priority scheduling | Polars (memory-aware) | "Unblocks most" pre-computed from chain graph |
| Inline value pattern | fjall (ByteView) | 128-bit stride with 15-byte inline payload |

### 8.2 What We Consciously Deferred

These are documented in the dispatch-and-optimisation topic as open
questions, NOT as confirmed decisions:

| Feature | Why deferred | Reference system |
|---------|-------------|-----------------|
| NUMA awareness | Target hardware is single-socket consumer | DuckDB |
| Out-of-core / spill | Data fits in memory (KB-MB) | Polars |
| Per-architecture assembly | Compiler autovectorisation sufficient | BLIS |
| Two-level granularity | Data volumes too small to benefit | DuckDB |
| Lock-free dispatch | Low thread count, negligible contention | DuckDB |
| Memory budget per thread | Small data volumes | DuckDB |
| Backpressure | All data fits in memory | Polars |
| Morsel ordering | Single core per chain preserves order | Polars |
| Adaptive execution | Static workloads | DuckDB |

### 8.3 Open Questions

The following require explicit decisions and are tracked in
`202603121800_topic.dispatch-and-optimisation.md`:

1. Should the design be NUMA-compatible even if we don't implement
   NUMA awareness now?
2. Should ColumnStorage have a capacity limit or spill interface?
3. Should we benchmark compiler autovectorisation of our inner loop?
4. Should we confirm morsel-driven (DuckDB) vs partition-based (Polars)
   parallelism?
5. Is single-level morsel granularity the right choice, or should we
   separate scheduling and cache-fitting units?
6. How do we ensure deterministic output if chains ever span multiple
   cores?
7. Is backpressure needed in any scenario?
8. Should dispatch be lock-free, mutex-based, or unspecified?
9. Should the library enforce a memory ceiling?
10. What happens when process_batch fails mid-chain?
11. How does the consumer extract typed results from output columns?

---

## 9. Appendix: Source References and Further Reading

### 9.1 Academic Papers

**Morsel-Driven Parallelism: A NUMA-Aware Query Evaluation Framework
for the Many-Core Age.**
Viktor Leis, Peter Boncz, Alfons Kemper, Thomas Neumann.
SIGMOD 2014.
The foundational paper for DuckDB's execution model. Introduces morsel
dispatch, NUMA-aware scheduling, and elastic parallelism. Experimental
results show near-linear scalability to 64 cores on TPC-H.

**Everything You Always Wanted to Know About Compiled and Vectorized
Queries But Were Afraid to Ask.**
Timo Kersten, Viktor Leis, Alfons Kemper, Thomas Neumann, Andrew Pavlo,
Peter Boncz.
PVLDB 2018 (VLDB).
Compares compiled (HyPer) vs vectorized (DuckDB) execution. Finds that
vectorized execution with 1,024-4,096 element vectors achieves
comparable performance to compiled execution for most workloads, with
simpler implementation.

**DuckDB: an Embeddable Analytical Database.**
Mark Raasveldt, Hannes Mühleisen.
SIGMOD 2019 (demo paper).
Introduces DuckDB and its vectorised execution engine. Describes the
pipeline-based execution model and the embedded database design.

**BLIS: A Framework for Rapidly Instantiating BLAS Functionality.**
Field G. Van Zee, Robert A. van de Geijn.
ACM Transactions on Mathematical Software (TOMS), 2015.
Describes the BLIS framework, five-loop nest, micro-kernel abstraction,
and blocking parameter derivation. The canonical reference for
understanding cache-aware matrix multiplication.

**Anatomy of High-Performance Matrix Multiplication.**
Kazushige Goto, Robert A. van de Geijn.
ACM Transactions on Mathematical Software (TOMS), 2008.
The predecessor to BLIS. Introduces the packing strategy, cache-aware
blocking, and the concept of keeping one operand in L2 while streaming
the other through L1. BLIS generalizes this approach.

**The Analytical Engine: On Vectorized Execution, Column Stores,
and Compression.**
Peter Boncz, Marcin Zukowski, Niels Nes.
Various publications (summarized in Boncz's tutorial at VLDB 2005).
Foundational work on column-store execution strategies, vectorized
processing, and the performance characteristics of cache-conscious
data processing.

### 9.2 Source Code References

**DuckDB source code:**
- `src/include/duckdb/common/vector_size.hpp` — STANDARD_VECTOR_SIZE
- `src/include/duckdb/storage/storage_info.hpp` — DEFAULT_ROW_GROUP_SIZE
- `src/parallel/task_scheduler.cpp` — thread pool and task dispatch
- `src/parallel/pipeline.cpp` — pipeline construction and execution
- `src/execution/physical_operator.cpp` — operator base class
- `src/execution/operator/scan/` — table scan with morsel dispatch

**BLIS source code:**
- `frame/include/bli_type_defs.h` — blocking parameters struct
- `frame/base/bli_cpuid.c` — hardware detection (CPUID)
- `frame/3/bli_l3_blocksize.c` — blocking parameter computation
- `config/haswell/bli_cntx_init_haswell.c` — Haswell sub-configuration
- `kernels/haswell/3/bli_dgemm_haswell_asm_6x8.c` — AVX2 micro-kernel
- `frame/thread/bli_thread.c` — thread partitioning logic

**Polars source code:**
- `crates/polars-stream/src/morsel.rs` — Morsel, MorselSeq
- `crates/polars-stream/src/nodes/` — compute node implementations
- `crates/polars-stream/src/graph.rs` — DAG construction
- `crates/polars-stream/src/execute/` — execution loop
- `crates/polars-config/src/lib.rs` — DEFAULT_IDEAL_MORSEL_SIZE

**DataFusion source code:**
- `datafusion/physical-plan/src/lib.rs` — ExecutionPlan trait
- `datafusion/execution/src/memory_pool/` — MemoryPool implementations
- `datafusion/physical-plan/src/repartition/` — RepartitionExec

**fjall source code:**
- `src/config.rs` — Config builder pattern
- `src/compaction/` — compaction strategies
- `src/blob_tree/` — value storage with inline optimization

### 9.3 Concrete Numbers for Mental Models

Rough order-of-magnitude numbers useful when reasoning about polka-dots
execution performance:

**Cache latencies (Apple M-series, approximate):**
- L1 data cache: ~1 ns (3-4 cycles)
- L2 cache: ~4-5 ns (12-15 cycles)
- L3 cache (SLC): ~12-15 ns (35-45 cycles)
- DRAM: ~80-120 ns (250-350 cycles)

**Cache sizes (consumer hardware range):**
- L1 data: 32 KB (x86 mainstream), 48 KB (x86 recent), 64 KB
  (Apple M1 E-core), 128 KB (Apple M1 P-core), 192 KB (Apple M2 P-core)
- L2: 256 KB (older), 512 KB (M1 E-core), 1 MB (x86 recent),
  4 MB (M1 P-core cluster), 16 MB (M2 P-core cluster)
- L3/SLC: 8 MB (x86 mainstream), 12-16 MB (M1), 24 MB (M2 Pro/Max)

**Atomic operation costs (x86, approximate):**
- Relaxed load/store: 0 extra cycles (same as regular)
- Acquire load: 0 extra cycles (x86 loads are acquire by default)
- Release store: 0 extra cycles (x86 stores are release by default)
- SeqCst load/store: ~5-20 cycles (full memory barrier)
- fetch_add (CAS loop): ~10-40 cycles (depending on contention)
- Compare-exchange: ~10-40 cycles

This is why we chose plain store(Release) over fetch_add — on x86,
Release store is literally a regular `mov` instruction. fetch_add is a
`lock xadd` which is 10-40x more expensive.

**ARM (Apple Silicon) atomic costs differ:**
- Release store: `stlr` instruction (~5-10 cycles, NOT free like x86)
- Acquire load: `ldar` instruction (~5-10 cycles, NOT free like x86)
- fetch_add: `ldxr/stxr` loop (~10-30 cycles)
- dmb ish (full barrier): ~10-30 cycles

On ARM, atomics are relatively more expensive than on x86. But at
morsel granularity (one atomic per morsel per chain), even 30 cycles
per atomic is negligible compared to processing thousands of rows.

**Thread creation/joining costs:**
- `std::thread::spawn()`: ~20-50 µs (involves OS syscall)
- `std::thread::scope()` overhead: ~1-2 µs (reuses thread pool)
- rayon task spawn: ~0.1-0.5 µs (work-stealing queue)
- Morsel processing time (our scale): ~10-100 µs

This is why the thread pool should be pre-created and reused. Spawning
a thread per morsel would add 20-50% overhead at our data volumes.

**polka-dots typical data volumes:**
- Registry entries: 50-500 apps
- Columns per chain: 3-15
- Rows per chain: 50-5,000
- Morsels per chain: 1-10
- Total data per pipeline: 10 KB - 2 MB
- Pipeline execution time (estimated): 1-50 ms

At these volumes, the absolute overhead of scheduling is more relevant
than asymptotic efficiency. A single mutex acquisition (~50 ns) vs
lock-free CAS (~20 ns) saves 30 ns per morsel — with 10 morsels, that's
300 ns total. Irrelevant. The right choice is whichever is simplest to
implement correctly.

### 9.4 Glossary

| Term | Definition | Used by |
|------|-----------|---------|
| **Morsel** | Fixed-size batch of rows processed as a unit | DuckDB, Polars |
| **Vector** | Small batch of rows fitting in L1 cache | DuckDB |
| **RecordBatch** | Arrow columnar batch with schema | DataFusion |
| **Pipeline** | Fused chain of operators between breakers | DuckDB |
| **Pipeline breaker** | Operator requiring full input before output | DuckDB |
| **Compute node** | DAG node in streaming execution | Polars |
| **Micro-kernel** | Register-level computation, hand-tuned | BLIS |
| **Packing** | Copying data to contiguous, aligned layout | BLIS |
| **Blocking parameter** | Cache-level tile size (mc, nc, kc) | BLIS |
| **Sub-configuration** | Architecture-specific kernel + params set | BLIS |
| **Partition** | Static data subset assigned to one thread | Polars, DataFusion |
| **Stride** | Fixed 16-byte column entry | polka-dots |
| **Chain** | Fused sequence of WorkUnits | polka-dots |
| **Schedule** | DAG of WorkUnits lowered to chains with indices | polka-dots |
| **ColumnStorage** | Consumer-provided backing memory | polka-dots |
| **Progress counter** | AtomicUsize tracking rows processed per chain | polka-dots |
