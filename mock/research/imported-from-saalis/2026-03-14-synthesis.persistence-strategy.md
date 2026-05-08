# Persistence Strategy Synthesis: Hot Store, Cold Store, and the Layer Between

**Date:** 2026-03-14
**Status:** Synthesis / Design Proposal
**Inputs:** `2026-03-14-research.hot-cold-storage-strategies.md`, `2026-03-14-research.sqlite-alternatives.md`
**Context:** Saalis uses an in-memory columnar hot store (hilavitkutin pipeline engine) with SQLite as cold persistent storage. The hot store handles queries and transforms; SQLite handles durability. This document synthesises findings from both research docs into a unified persistence strategy and proposes novel combinations that neither doc covers alone.

---

## 1. SIEVE Eviction Meets SQLite WAL: Two Clocks, One Heartbeat

Both research documents independently converge on the same two mechanisms: SIEVE for hot-store eviction and SQLite WAL for durable writes. What neither document explores is how these two systems interact temporally and how their coordination can be exploited.

### The Interaction Model

SIEVE maintains a "hand" pointer that sweeps through entries in FIFO insertion order, clearing visited bits and evicting unvisited entries. SQLite WAL maintains a write cursor that appends modified pages sequentially, with periodic checkpoints that transfer WAL contents to the main database file. Both are sweep-based, append-oriented mechanisms. The key insight is that **SIEVE eviction events and SQLite WAL checkpoints should be phase-locked**.

When SIEVE evicts a dirty segment from the hot store, that segment must first be flushed to SQLite. If WAL checkpoint is happening concurrently, the flush competes with checkpoint I/O. Worse, if the segment being flushed modifies pages currently being checkpointed, SQLite must handle this contention internally (it does, correctly, but with added latency). The solution is to treat SIEVE eviction sweeps and WAL checkpoints as phases of the same cycle:

1. **Eviction phase:** SIEVE sweeps and identifies candidates. Dirty candidates are written to SQLite (appended to the WAL). Clean candidates are simply dropped from the hot store.
2. **Quiesce phase:** Brief pause — no new dirty flushes.
3. **Checkpoint phase:** SQLite checkpoints the WAL to the main database file.
4. **Resume phase:** Normal operation resumes.

This phasing eliminates the pathological case where eviction-driven flushes and checkpoint I/O overlap and contend for disk bandwidth. On SD card / eMMC storage (Batocera's typical medium), sequential I/O is dramatically faster than random I/O, and phase-locking keeps the I/O pattern sequential within each phase.

### Checkpoint Trigger Policy

Rather than relying on SQLite's default `wal_autocheckpoint` (1000 pages), saalis should trigger checkpoints at natural boundaries:

- **After a scheduler epoch completes** (all work units in the batch are done).
- **When SIEVE eviction has flushed more than N dirty segments** since the last checkpoint (backpressure-driven).
- **On idle timeout** (no user interaction for T seconds — flush everything while the system is quiet).
- **On clean shutdown** (flush all dirty segments, checkpoint, then close).

This gives us `PRAGMA wal_autocheckpoint=0` (disable automatic) with application-controlled `sqlite3_wal_checkpoint_v2(SQLITE_CHECKPOINT_PASSIVE)` calls. Passive checkpoints do not block readers, which preserves the hot store's ability to serve demand-loaded reads during checkpoint.

---

## 2. The Sync Layer: Epoch-Batched Dirty Flush with Prioritised Startup Warming

The sync layer is the bridge between the hot store and SQLite. It has two directions: downward (hot-to-cold flush) and upward (cold-to-hot warming). Both research documents treat these independently. Here they are unified.

### Downward: Epoch-Batched Dirty Flush

The hot-cold research recommends epoch-batched dirty tracking, and the SQLite research confirms that burst-write patterns are well-suited to WAL mode. The sync layer combines these:

**Epoch lifecycle:**
1. A scheduler work unit (catalogue scan, enrichment cycle, user edit) begins. An epoch counter increments.
2. All mutations within this work unit mark their column segments dirty and tag them with the current epoch number.
3. When the work unit completes, the sync layer collects all segments tagged with that epoch.
4. A single SQLite transaction wraps the entire flush: `BEGIN IMMEDIATE` → write all dirty segments → `COMMIT`.
5. Dirty bits are cleared. The epoch number advances.

**Why `BEGIN IMMEDIATE`?** SQLite WAL mode allows concurrent reads during `BEGIN IMMEDIATE` transactions. This is critical: the UI can continue browsing (reading from the hot store, or demand-loading from SQLite) while the flush transaction is writing. A `BEGIN EXCLUSIVE` would block readers unnecessarily. Since we have a single writer (the scheduler), there is no write contention to worry about, only read/write concurrency.

**Epoch size tuning:** The epoch boundary should align with scheduler work-unit boundaries, not wall-clock time. A catalogue scan that discovers 500 new entities produces one epoch of 500 inserts. An enrichment cycle that updates metadata for 200 entities produces one epoch of 200 updates. The SQLite transaction size directly corresponds to the logical unit of work, making crash recovery semantically clean: either the whole enrichment batch is persisted, or none of it is.

**Crash-loss window:** If saalis crashes mid-epoch, the mutations in that epoch are lost. The hot store is gone (it was in memory), and the transaction was never committed to SQLite. The maximum data loss is one epoch — one work unit's worth of mutations. For catalogue scans, this means re-scanning; for enrichment, re-enriching. Both are idempotent operations. This is an acceptable trade-off for the performance gain of batched writes.

### Upward: Prioritised Startup Warming

The hot-cold research recommends persisting a manifest of hot segments on shutdown and reloading them on startup. The sync layer adds priority ordering based on data classification:

**Priority tiers for startup warming:**
1. **Tier 0 — Schema metadata.** Entity type descriptors, registered metadata table definitions, column type mappings. These are tiny (kilobytes) and required before any other data can be interpreted. Load synchronously before the UI starts.
2. **Tier 1 — Navigation indices.** Entity names, sort keys, and filter columns (genre, platform, year). These power the library browse view. Load in a background work unit immediately after startup. The UI shows a loading indicator until this tier is warm.
3. **Tier 2 — Last-session working set.** The manifest records which segments were hot at shutdown. Load these next, still in the background. By the time the user navigates to a specific entity, the data they were looking at in their last session is likely already warm.
4. **Tier 3 — Frequency-ranked fill.** If memory budget permits after tiers 0-2, load the most frequently accessed segments (tracked by access counters persisted alongside the manifest). This opportunistic fill runs at lowest scheduler priority and yields to any user-initiated work.

**Manifest structure:** A SQLite table (`_hot_manifest`) rather than a separate file. This ensures the manifest is transactionally consistent with the data it describes and benefits from the same backup/restore mechanism (copy the `.db` file). Columns: `segment_id TEXT`, `access_count INTEGER`, `last_access_epoch INTEGER`, `tier INTEGER`, `size_bytes INTEGER`. On shutdown, truncate and re-insert. On startup, `SELECT * FROM _hot_manifest ORDER BY tier ASC, access_count DESC`.

---

## 3. DuckDB as a Hot-Cold Bridge: The Case Against and the Narrow Case For

The SQLite alternatives research concluded that DuckDB is not recommended as a cold-store replacement because our architecture already pushes analytical work to the hot store. But neither document asks the more interesting question: could DuckDB serve as the **sync layer itself** — the bridge between hot and cold — rather than replacing either tier?

### The Theoretical Appeal

DuckDB's columnar storage format is structurally similar to what the hot store uses. Column segments in DuckDB are stored as compressed, typed arrays with dictionary encoding, bitpacking, and run-length encoding. If the hot store's internal representation is also columnar (which it is — hilavitkutin is a columnar pipeline engine), then DuckDB could serve as a "warm" tier that speaks both languages:

- **Hot → DuckDB:** Evicted segments are written to DuckDB in their columnar form, preserving the encoding. No row-wise serialisation needed.
- **DuckDB → Hot:** Warming reads column segments directly from DuckDB's storage, again without row-wise deserialisation.
- **DuckDB → SQLite:** DuckDB can read from and write to SQLite via its `sqlite_scanner` extension, enabling bulk synchronisation.

### Why This Does Not Work for Saalis

Despite the theoretical elegance, this architecture introduces more problems than it solves:

1. **Three tiers instead of two.** Adding DuckDB as a warm tier means managing eviction between hot→warm and warm→cold, consistency between three representations of the same data, and crash recovery across three systems. The complexity cost is enormous.
2. **Binary size.** DuckDB adds ~5MB to the binary. SQLite adds ~600KB. Running both is a 10x increase in persistence-layer footprint.
3. **DuckDB's OLTP weakness.** Point writes (updating a single entity's metadata) are slow in DuckDB. The hot store's dirty-flush workload is a mix of bulk inserts (new entities from catalogue scan) and point updates (enrichment results for specific entities). DuckDB handles the first well but not the second.
4. **SQLite cannot be eliminated.** We need FTS5 for full-text search, foreign key constraints for referential integrity, and the single-file backup format. DuckDB provides none of these. So DuckDB would be an addition, not a replacement, and the complexity of maintaining two disk databases outweighs the serialisation savings.

### The Narrow Exception

There is one scenario where DuckDB earns its place: **offline analytical exports**. If a user wants to run complex queries across their entire library — "show me all RPGs released between 1990-2000, grouped by platform, with average completion time" — this is exactly what DuckDB excels at. Rather than making DuckDB part of the persistence architecture, it could be a **read-only analytical view** materialised from SQLite on demand:

```
User requests analytical report →
  Export relevant SQLite tables to DuckDB (in-memory or temp file) →
  Run analytical query in DuckDB →
  Return results →
  Discard DuckDB instance
```

This keeps DuckDB out of the critical persistence path while leveraging its analytical strengths. The cost is the DuckDB library size in the binary, which may not be justified unless analytical features are a confirmed requirement. **For now, defer this decision.**

---

## 4. Rusqlite Over SeaORM: Implications for the Sync Layer

The SQLite alternatives research recommends evaluating rusqlite (sync) over SeaORM (async) for cold-store access. This recommendation has deep implications for the sync layer design.

### The Performance Argument

SeaORM wraps SQLite access in an async runtime (tokio). Since SQLite has no native async I/O, every database call is dispatched to a blocking thread pool via `spawn_blocking`. This adds:

- Thread pool dispatch overhead per operation.
- Unnecessary `Future` state machine compilation for what is fundamentally a synchronous call.
- Waker/poll overhead that provides no benefit when the underlying I/O is blocking.

Benchmarks cited in the SQLite research show sync SQLite access (via rusqlite) performing considerably better than async wrappers. For saalis's sync-core architecture, where the only async boundary is the axum web layer, wrapping sync SQLite calls in async machinery is pure overhead.

### Sync Layer Design with Rusqlite

Using rusqlite directly aligns perfectly with the epoch-batched flush model:

```
// Epoch flush — all sync, no async overhead
fn flush_epoch(conn: &rusqlite::Connection, dirty_segments: &[DirtySegment]) -> Result<()> {
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
    for seg in dirty_segments {
        seg.write_to(&tx)?;
    }
    tx.commit()
}
```

No `async fn`, no `.await`, no `spawn_blocking`. The scheduler thread calls `flush_epoch` directly. This is simpler, faster, and easier to reason about.

### What We Lose

SeaORM provides:
- **Schema migration management.** Rusqlite does not. We would need a lightweight migration system (e.g., `refinery` crate, or hand-rolled `user_version`-based migrations).
- **Type-safe query building.** SeaORM generates Rust types from the schema. With rusqlite, queries are raw SQL strings with manual parameter binding. However, since the hot store handles most queries, the cold store's query surface is small: bulk inserts, bulk updates, point lookups by entity ID, and manifest reads. These are simple enough that raw SQL is not a maintenance burden.
- **Relationship traversal.** SeaORM models foreign key relationships and provides eager/lazy loading. With rusqlite, JOIN queries are manual SQL. Again, the hot store handles relationship-heavy queries; the cold store primarily does bulk I/O.

### The Middle Path: Rusqlite with Typed Wrappers

Rather than choosing between full ORM and raw SQL, the sync layer can provide a thin typed wrapper over rusqlite:

- **A `ColdStore` struct** that owns a `rusqlite::Connection` and exposes domain-specific methods: `flush_segments()`, `load_segments()`, `warm_manifest()`, `checkpoint()`.
- **Compile-time SQL validation** via the `include_str!` pattern — SQL statements live in `.sql` files, loaded at compile time, and executed with typed parameter structs.
- **Migration via `PRAGMA user_version`** — each schema version is a numbered `.sql` file, applied in order on startup if the database is behind.

This gives us the performance of rusqlite, the type safety of domain-specific methods, and the maintainability of separated SQL files, without the overhead of a full async ORM.

### Connection Management

With rusqlite (sync), connection management is straightforward:

- **One write connection** owned by the scheduler thread, used for epoch flushes and checkpoints. Opened with `PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL; PRAGMA cache_size=-8000;` (8MB page cache).
- **One or more read connections** used for demand-loading cold data into the hot store. These can be opened with `PRAGMA query_only=1` for safety. On a single-core appliance, one read connection suffices; on multi-core, a small pool (2-4) allows concurrent demand loads.
- **Connection lifetime:** Connections are opened at startup and held for the process lifetime. No connection pooling overhead.

---

## 5. Limbo as Future Migration Path

The SQLite research identifies Turso's Limbo (SQLite rewritten in Rust with native async I/O via `io_uring`) as the most promising future option. How does this interact with the sync layer design?

### What Limbo Changes

If Limbo reaches stability and API compatibility with SQLite, the migration path is:

1. Replace `rusqlite` dependency with Limbo's Rust API.
2. The sync layer's `ColdStore` struct swaps its connection type. Method signatures remain the same (Limbo aims for SQLite API compatibility).
3. On Linux (Batocera), `io_uring` eliminates the blocking I/O model entirely. Epoch flushes become truly asynchronous at the kernel level, meaning the scheduler thread is not blocked during disk writes.

### What Limbo Does Not Change

- The SIEVE eviction strategy is hot-store-internal and independent of the cold store implementation.
- The epoch-batched flush model remains the same — batching is a logical concern, not an I/O concern.
- The startup warming strategy reads from whatever SQLite-compatible store exists.
- The manifest table, migration system, and connection management patterns transfer directly.

### Design for Migration

To prepare for a future Limbo migration without coupling to it today:

- **Abstract the connection behind a `ColdStore` trait** with methods like `flush_segments()`, `load_segments()`, `checkpoint()`. The current implementation uses rusqlite; a future implementation uses Limbo.
- **Keep SQL statements dialect-neutral.** Limbo targets SQLite compatibility, so standard SQLite SQL will work unchanged.
- **Do not depend on SeaORM's code generation.** SeaORM would need to add a Limbo backend, which may not happen or may lag. Rusqlite-style raw connections are easier to swap.
- **Isolate `PRAGMA` configuration.** Limbo may support different pragmas or different defaults. Keep all pragma configuration in a single `configure_connection()` function.

The key architectural decision that enables this migration is the one already made: sync core with the cold store accessed through a narrow, domain-specific interface rather than a general-purpose ORM spread throughout the codebase.

---

## 6. Novel Combination: SQLite as Write-Only Store (No Read Path)

Neither research document proposes this, but the architecture makes it possible: **what if SQLite never serves reads during normal operation?**

### The Idea

In the current design, SQLite serves two roles:
1. **Durability** — persist data so it survives crashes and restarts.
2. **Demand loading** — when the hot store has a cache miss, fetch the data from SQLite.

Role 2 is only needed when the hot store does not contain the requested data. If we could guarantee the hot store always contains everything the user needs, SQLite becomes a pure write target — an append-only durability log that is only read at startup (for warming) and never during normal operation.

### When This Works

This works when the **entire active working set fits in the hot store's memory budget.** For a library of 10,000 entities with, say, 20 metadata columns averaging 100 bytes each, the raw data is ~20MB. With columnar encoding overhead, call it 50MB. Even on a 256MB memory budget, this is trivially resident. The hot store can hold the entire library.

For larger libraries (100,000+ entities) or richer metadata (cover art thumbnails cached as blobs, lengthy descriptions), the working set may exceed the memory budget. In that case, demand loading from SQLite is unavoidable, and SQLite must serve reads.

### What This Simplifies

If SQLite is write-only during normal operation:

1. **No read connection needed.** A single write connection suffices. Connection management is trivial.
2. **No cache-miss latency.** Every query is served from the hot store at in-memory speed. The user experience is uniformly fast.
3. **SQLite page cache can be minimised.** Since SQLite is not serving reads, its page cache (`PRAGMA cache_size`) can be set to the minimum (10 pages / 40KB). This frees memory for the hot store.
4. **Checkpoint timing is unconstrained.** With no readers on the SQLite file, checkpoints can use `SQLITE_CHECKPOINT_TRUNCATE` (the most aggressive mode) without blocking anyone. WAL file size stays bounded.
5. **SIEVE eviction simplifies.** Evicted segments do not need to be "available from cold storage" because they will not be re-read. They simply need to have been flushed to SQLite before eviction for durability. Clean eviction requires no I/O at all — just drop the segment.

### The Hybrid Approach

Rather than choosing write-only or read-write statically, the sync layer can adapt:

- **At startup,** attempt to load the entire library into the hot store (tier-0 through tier-3 warming).
- **If the entire library fits,** set a `cold_reads_disabled` flag. SQLite becomes write-only. SIEVE eviction is replaced with simple LRU for managing query working memory (temporary buffers), since all entity data is always resident.
- **If the library exceeds the memory budget,** operate in the standard two-tier mode with SIEVE eviction and demand loading.
- **Monitor memory pressure at runtime.** If the library grows (new entities catalogued) and approaches the budget, transition from write-only to two-tier mode gracefully.

This adaptive approach captures the simplicity of write-only mode for small-to-medium libraries (the common case for a personal game collection) while gracefully degrading to full two-tier operation for large collections.

### Write-Only as WAL-Only

Taking this further: if SQLite is write-only, we can configure it for maximum write throughput with no concern for read performance:

- `PRAGMA synchronous=NORMAL` — fsync only on checkpoint, not on every WAL append. Safe because saalis tolerates losing one epoch on crash.
- `PRAGMA journal_mode=WAL` — sequential appends, no page overwrites during writes.
- `PRAGMA wal_autocheckpoint=0` — manual checkpoints only, at epoch boundaries or on idle.
- `PRAGMA cache_size=-64` — minimal 64KB page cache (just enough for the write path).
- `PRAGMA temp_store=MEMORY` — temporary tables in memory (avoids temp file I/O).
- `PRAGMA locking_mode=EXCLUSIVE` — since there are no readers, take an exclusive lock at open and hold it forever. This eliminates per-transaction lock acquisition overhead.

This configuration makes SQLite behave essentially as a structured, transactional append-only log — which is exactly what a write-only cold store is.

---

## 7. Memory Budget Management: Splitting the Pie

On a Batocera/loisto appliance with 2-8GB total RAM, saalis must share memory with the OS, Batocera's emulator processes, and other system services. A realistic memory budget for saalis is **256MB to 1GB**, depending on the device and whether an emulator is concurrently running.

### Budget Partitioning

The hot-cold research recommends DuckDB's unified-pool approach. Applied to saalis with the additional constraint of SQLite's page cache:

**Total saalis memory budget (M):**

| Component | Write-Only Mode | Two-Tier Mode | Rationale |
|---|---|---|---|
| **Hot store: entity data** | 70% of M | 55% of M | Cached column segments. Managed by SIEVE. |
| **Hot store: query working memory** | 20% of M | 20% of M | Transient buffers for sort, filter, aggregation during pipeline execution. |
| **SQLite page cache** | 1% of M (minimum) | 15% of M | In write-only mode, almost nothing; in two-tier mode, supports demand-load reads. |
| **Sync layer buffers** | 4% of M | 5% of M | Serialisation buffers for epoch flush, manifest data, segment metadata. |
| **Headroom** | 5% of M | 5% of M | Safety margin for allocation spikes, stack space, misc. |

**Concrete example — 512MB budget:**

| Component | Write-Only | Two-Tier |
|---|---|---|
| Entity data cache | 358 MB | 281 MB |
| Query working memory | 102 MB | 102 MB |
| SQLite page cache | 5 MB | 77 MB |
| Sync layer buffers | 20 MB | 26 MB |
| Headroom | 26 MB | 26 MB |

### Unified Pool with Soft Boundaries

Rather than hard-partitioning memory, adopt DataFusion's reservation model with soft boundaries:

1. **A single `MemoryPool`** tracks all allocations across entity cache, query buffers, and sync layer buffers.
2. **Each component holds a `MemoryReservation`** that tracks its current usage.
3. **Soft limits** define the target partition (e.g., entity cache aims for 55% of M). Components can exceed their soft limit if other components are under-utilising their share.
4. **Hard limit** is the total budget M. When total usage approaches M, the pool triggers SIEVE eviction in the entity cache first (largest component, most evictable data), then spills query working memory to disk (temporary files), then shrinks sync layer buffers.
5. **SQLite page cache is managed separately** via `PRAGMA cache_size` because SQLite manages its own memory internally. The budget for SQLite is set at connection open time and not dynamically adjusted. However, the `sqlite3_soft_heap_limit64()` function can impose a process-wide soft limit on SQLite's heap usage, providing a safety valve.

### Dynamic Adjustment

The memory pool should adapt to runtime conditions:

- **During startup warming,** entity cache gets priority (up to 80% of M) since query working memory is idle.
- **During active browsing,** entity cache and query memory share normally.
- **During catalogue/enrichment bursts,** sync layer buffers grow temporarily (serialising large batches) at the expense of entity cache (evicting less-frequently-used data).
- **During idle,** entity cache can expand into query working memory's share (no active queries) to cache more data for the next active session.

### SQLite Page Cache Sizing

The interaction between SQLite's page cache and the hot store deserves specific attention:

- In **two-tier mode**, SQLite's page cache accelerates demand-load reads. A larger page cache means repeated demand loads for the same cold data are served from SQLite's cache rather than from disk. This is a second-level cache beneath the hot store — a cache of a cache.
- The optimal SQLite cache size depends on the "churn rate" of demand loads. If the hot store's SIEVE eviction is stable (low churn), most demand loads are for truly cold data that is unlikely to be re-requested soon, and a large SQLite page cache provides little benefit. If SIEVE thrashes (memory budget is severely constrained), the SQLite page cache catches re-requests and acts as a buffer.
- **Recommendation:** Start with a modest SQLite page cache (8MB, `PRAGMA cache_size=-8000`) and monitor demand-load hit rates. If the SQLite page cache hit rate is below 10%, shrink it and give the memory to the hot store. If above 50%, consider growing it.

---

## 8. Putting It All Together: The Layered Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Web UI (axum)                     │  ← async boundary
├─────────────────────────────────────────────────────┤
│               hilavitkutin pipeline engine           │  ← queries, transforms
│            ┌─────────────────────────────┐           │
│            │     Hot Store (columnar)     │           │
│            │   SIEVE eviction policy      │           │
│            │   unified memory pool        │           │
│            └──────────┬──────────────────┘           │
├───────────────────────┼─────────────────────────────┤
│                 Sync Layer                           │
│   ┌───────────────────┼───────────────────────┐     │
│   │  Downward:        │  Upward:              │     │
│   │  epoch-batched    │  demand load          │     │
│   │  dirty flush      │  + startup warming    │     │
│   │  (scheduler       │  (prioritised tiers)  │     │
│   │   work units)     │                       │     │
│   └───────────────────┼───────────────────────┘     │
├───────────────────────┼─────────────────────────────┤
│              SQLite (rusqlite, WAL mode)             │  ← cold store
│         write conn (scheduler) + read conn(s)        │
│         application-controlled checkpoints           │
│         _hot_manifest table for warming              │
└─────────────────────────────────────────────────────┘
```

### Data Flow: Write Path

1. User action or scheduler work unit mutates entity data in the hot store.
2. Affected column segments are marked dirty with the current epoch number.
3. Work unit completes. The sync layer collects all dirty segments for the epoch.
4. Sync layer opens a `BEGIN IMMEDIATE` transaction on the write connection.
5. Each dirty segment is serialised and written to SQLite (INSERT or UPDATE).
6. Transaction commits. Dirty bits are cleared.
7. If accumulated WAL size exceeds threshold, trigger a passive checkpoint.

### Data Flow: Read Path (Two-Tier Mode)

1. Pipeline query requests a column segment not present in the hot store.
2. Hot store signals a cache miss to the sync layer.
3. Sync layer issues a SELECT on a read connection, deserialises the segment.
4. Segment is inserted into the hot store, subject to SIEVE eviction.
5. If SIEVE needs to evict to make room, evicted clean segments are dropped; evicted dirty segments are flushed first.

### Data Flow: Read Path (Write-Only Mode)

1. Pipeline query requests a column segment. It is always present (entire library fits in memory).
2. Query executes at in-memory speed. No sync layer involvement.

### Data Flow: Startup

1. Open SQLite connection. Run migrations via `PRAGMA user_version`.
2. Configure pragmas (WAL mode, synchronous, cache size).
3. Load tier-0 (schema metadata) synchronously.
4. Start axum web server (UI becomes responsive with loading state).
5. Background work unit loads tier-1 (navigation indices).
6. Background work unit loads tier-2 (last-session working set from `_hot_manifest`).
7. Background work unit loads tier-3 (frequency-ranked fill, budget permitting).
8. If entire library loaded, switch to write-only mode.

### Data Flow: Shutdown

1. Flush all dirty segments (final epoch).
2. Truncate and repopulate `_hot_manifest` with current hot-store contents and access counters.
3. Run `SQLITE_CHECKPOINT_TRUNCATE` (clean WAL).
4. Close connections.

---

## 9. Decision Summary

| Decision | Choice | Rationale |
|---|---|---|
| **Cold store engine** | SQLite (keep) | Relational model, FTS5, single-file backup, ARM support, maturity |
| **Cold store access** | rusqlite (sync) | Sync core architecture; no async overhead; narrow cold-store query surface |
| **Eviction algorithm** | SIEVE | Patent-free, parameter-free, outperforms ARC, trivial to implement |
| **Eviction granularity** | Column segment | Matches columnar layout, cheap dirty tracking, single I/O per operation |
| **Write-back strategy** | Epoch-batched dirty flush | Aligns with scheduler work units, bounded crash-loss window, single SQLite transaction per epoch |
| **WAL checkpoint** | Application-controlled, phase-locked with eviction | Eliminates I/O contention on flash storage, maximises sequential write patterns |
| **Startup warming** | 4-tier prioritised partial preload from `_hot_manifest` table | Near-instant UI responsiveness with progressive warming |
| **Memory management** | Unified pool with soft boundaries, reservation-based allocation | Avoids fragmentation, adapts to runtime conditions |
| **SQLite page cache** | 8MB default, monitor and adjust | Minimal in write-only mode, moderate in two-tier mode |
| **Write-only optimisation** | Adaptive — enabled when entire library fits in memory | Simplifies read path, checkpoint strategy, and connection management for common case |
| **DuckDB** | Not included in persistence path; possible future analytical export tool | Complexity of three-tier outweighs columnar bridge benefits |
| **Limbo migration** | Prepared for but not adopted yet | Abstract cold store behind `ColdStore` trait; swap implementation when Limbo stabilises |
| **SeaORM** | Replaced by rusqlite with typed wrappers for cold store access | Async overhead eliminated; domain-specific interface over general ORM |

---

## 10. Open Questions for Future Work

1. **Segment serialisation format.** How are column segments serialised for SQLite storage? Options: raw bytes (fastest), MessagePack (compact + schema-flexible), bincode (Rust-native), or SQLite's native column types (one SQLite column per hot-store column, most query-friendly from cold). This interacts with whether DuckDB-style columnar encoding is worth implementing for the on-disk representation.

2. **Crash-recovery testing.** The epoch-batched model tolerates losing one epoch. But what if the epoch is very large (initial catalogue scan of 50,000 entities)? Should large epochs be sub-batched with intermediate commits to bound the crash-loss window?

3. **Compaction and vacuum.** SQLite databases can fragment over time with many updates. When should `VACUUM` run? How does this interact with the write-only mode (no readers to block) versus two-tier mode (readers would be blocked)?

4. **Memory pool implementation.** Should the unified pool be a custom implementation or should we adopt DataFusion's `MemoryPool` trait and `GreedyMemoryPool`? DataFusion is a large dependency to pull in just for memory management, but implementing reservation-based cooperative memory management correctly is non-trivial.

5. **Flash wear monitoring.** On SD card / eMMC appliances, write amplification matters for device longevity. Should the sync layer track total bytes written and warn the user if write rates suggest premature storage wear?

---

## Sources

Synthesised from:
- [Hot/Cold Storage Strategies Research](2026-03-14-research.hot-cold-storage-strategies.md) — SIEVE, eviction algorithms, dirty tracking, startup warming, memory management
- [SQLite Alternatives Research](2026-03-14-research.sqlite-alternatives.md) — SQLite WAL, DuckDB evaluation, Limbo, rusqlite vs SeaORM, ARM compatibility

Additional references cited in source documents:
- [SIEVE: Simpler than LRU (NSDI '24)](https://www.usenix.org/conference/nsdi24/presentation/zhang-yazhuo)
- [DuckDB Memory Management (2024)](https://duckdb.org/2024/07/09/memory-management)
- [DataFusion SIGMOD 2024 Paper](https://andrew.nerdnetworks.org/pdf/SIGMOD-2024-lamb.pdf)
- [SQLite WAL Documentation](https://sqlite.org/wal.html)
- [VoltDB Anti-Caching (VLDB 2013)](https://www.vldb.org/pvldb/vol6/p1942-debrabant.pdf)
- [Rust ORMs in 2026](https://aarambhdevhub.medium.com/rust-orms-in-2026-diesel-vs-sqlx-vs-seaorm-vs-rusqlite-which-one-should-you-actually-use-706d0fe912f3)
