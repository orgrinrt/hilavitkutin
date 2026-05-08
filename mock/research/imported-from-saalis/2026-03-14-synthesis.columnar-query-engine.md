# Synthesis: The Columnar Hot Store as a Universal Query Engine

**Date:** 2026-03-14
**Context:** Saalis content library manager integrates hilavitkutin (a morsel-driven columnar pipeline engine from polka-dots) as its hot store. This document argues that hilavitkutin is not merely a cache sitting in front of SQLite -- it is a full query engine that can deliver analytical-database-grade performance for every DataView interaction in the UI.

---

## 1. Why This Combination Is Novel

No existing content library -- Playnite, LaunchBox, Jellyfin, Plex, Steam -- uses a columnar analytical engine for its UI queries. They all follow the same pattern: SQLite or PostgreSQL stores entities, the application issues row-oriented SQL queries, and the UI renders the result set. Filtering, sorting, and pagination happen inside the relational engine, constrained by its row-at-a-time execution model.

Analytical databases -- DuckDB, Polars, DataFusion, ClickHouse -- operate on columnar data with vectorized execution. They process thousands of values per CPU instruction using SIMD, achieve cache-optimal memory access patterns, and can filter-sort-aggregate datasets of millions of rows in single-digit milliseconds. But these engines are designed for batch analytics: ad-hoc SQL queries over Parquet files, data science notebooks, ETL pipelines. Nobody has pointed one at a content library's browse-filter-sort UI.

hilavitkutin occupies an unusual position: it is a morsel-driven columnar pipeline engine designed for real-time interactive use, not batch analytics. Its morsels are sized to fit L1 cache. Its DAG scheduler dispatches work units based on column read/write dependencies. Its execution model is stateless -- `fn process_batch(read: Self::Read, write: Self::Write) -> Result<()>` -- with no query planning, no SQL parsing, no cost-based optimization. The pipeline is compiled at build time through Rust's type system.

This means saalis can offer Notion-like flexible views -- table, gallery, board, list, with arbitrary filters, sorts, and groupings -- backed by the same query performance characteristics as DuckDB, but without the overhead of a SQL layer, query planner, or external process. The DataView system and hilavitkutin's pipeline engine are two sides of the same coin: DataViews declare what the user wants to see, and the pipeline engine executes it on columnar data in sub-millisecond time.

---

## 2. Morsel-Driven Execution and Sub-Millisecond UI Response

### 2.1 What a Morsel Is

A morsel is a batch of column values sized to fit in L1 cache (typically 32-64 KB). For a library of 100,000 game entities, a column of 64-bit integers (entity IDs, ratings, release years) occupies 800 KB -- roughly 12-25 morsels depending on L1 cache size. A column of interned string handles (u32, 4 bytes each) for titles occupies 400 KB -- about 6-12 morsels.

The critical property of morsel-driven execution is that each morsel's data fits entirely in L1 cache while being processed. This eliminates cache misses during the inner loop -- the dominant performance bottleneck in row-oriented databases where chasing pointers through row structures causes constant L1/L2 cache misses.

### 2.2 Filter-Sort-Paginate in One Pipeline

Consider the most common DataView operation: "show me 50 games sorted by rating, filtered by platform = SNES." In a row-oriented model, this requires:

1. Scan all rows, evaluate the platform predicate per row
2. Collect matching rows into a temporary buffer
3. Sort the buffer by rating
4. Take the first 50

Each step materializes intermediate results. In hilavitkutin's pipeline model, these stages fuse into a single pass:

1. The filter stage reads the `platform` column morsel-by-morsel, producing a selection vector (a bitmask of which rows pass the predicate)
2. The selection vector feeds directly into a partial sort stage that maintains a top-50 heap, reading the `rating` column only for selected rows
3. The final 50 entity IDs feed into a projection stage that reads only the columns needed for display (title, cover_art_url, rating, platform)

No intermediate materialization. No temporary buffers larger than a morsel. The filter reads one column, the sort reads one column, the projection reads a few columns. At 100k entities, this completes in hundreds of microseconds -- well under 1 ms. The user perceives it as instantaneous.

### 2.3 Comparison to Traditional Database Execution

SQLite processes queries row-by-row through a virtual machine (VDBE). Even with covering indexes, a filter-sort-paginate query on 100k rows involves:

- B-tree traversal to locate matching rows (log N per lookup, or a range scan)
- Row reconstruction from the page cache (row-oriented layout, pulling entire rows even if only 3 of 15 columns are needed)
- Sorting via a temporary B-tree or external sort
- Page cache contention if the working set exceeds the cache size

For 100k entities with proper indexes, SQLite achieves 5-50 ms depending on query complexity and cache state. This is fast enough for most applications -- but it is 10-100x slower than columnar execution on the same data. More importantly, SQLite's performance degrades unpredictably with complex multi-column filters, missing indexes, or cold cache states. The columnar model has deterministic, data-size-proportional performance regardless of filter complexity.

---

## 3. The Columnar Layout Advantage for DataView Projections

### 3.1 Only Touch Columns You Display

A game entity might have 30 metadata fields: title, developer, publisher, release_year, genre, platform, region, languages, file_size, checksum, download_status, play_status, rating, cover_art_url, screenshot_urls, description, series, sequel_to, rom_format, emulator_compatibility, last_played, play_time, completion_percentage, user_tags, notes, source_connector, source_url, created_at, updated_at, verified_at.

A Gallery view displays 5 of these: cover_art_url, title, platform, rating, download_status. A Table view might display 10. A Board view grouped by download_status displays 4.

In row-oriented storage, every query reads entire rows -- all 30 fields -- even if only 5 are needed. The wasted bandwidth is proportional to the number of unused columns. With 100k entities and 30 fields averaging 50 bytes each, a row-oriented scan reads 150 MB. A columnar scan reading 5 columns reads 25 MB. At the L1 cache level, this difference is the difference between fitting in cache and not.

hilavitkutin's `type Read: ColumnSlices` associated type on WorkUnit enforces this at compile time. A projection work unit for Gallery view declares `type Read = (CoverArtUrl, Title, Platform, Rating, DownloadStatus)`. The pipeline physically cannot read other columns -- the type system prevents it. This is projection pushdown enforced by the Rust compiler, not by a query optimizer that might get it wrong.

### 3.2 Compression and Cache Density

Columnar data compresses dramatically because values within a column share type and distribution. A `platform` column with 100k entries drawn from ~50 distinct platforms compresses via dictionary encoding to under 1 KB for the dictionary plus 1-2 bytes per entry (50k-200k total). A `release_year` column spanning 1980-2025 compresses via frame-of-reference encoding to 6 bits per entry. A `download_status` column with 5 possible states compresses to 3 bits per entry.

This compression serves double duty: it reduces memory footprint (critical for saalis's 256-512 MB budget on Batocera appliances) and increases effective cache bandwidth. More values fit in each cache line, so each morsel processes more rows.

---

## 4. Performance at Our Scale: Comparison to DuckDB and Polars

### 4.1 The Scale Question

Saalis targets 10k-100k entities. This is tiny by analytical database standards -- DuckDB and Polars are designed for millions to billions of rows. But the performance characteristics that make analytical engines fast at scale also make them absurdly fast at small scale.

DuckDB benchmarks show single-threaded scan-filter-aggregate on 10M rows completing in ~100 ms. Linear extrapolation to 100k rows gives ~1 ms. Polars lazy execution on similar workloads at 100k rows completes in sub-millisecond time. hilavitkutin, being a purpose-built morsel pipeline without SQL parsing, query planning, or memory management overhead, should match or exceed these numbers for equivalent operations.

### 4.2 Where We Win

DuckDB and Polars have overhead that saalis does not need:

- **SQL parsing and planning:** DuckDB parses SQL, builds a logical plan, optimizes it, and generates a physical plan. For saalis, the "query" is a DataView configuration compiled at build time into a typed pipeline. There is zero planning overhead per query.
- **Memory management:** DuckDB's buffer manager handles larger-than-memory datasets with spill-to-disk. Saalis's hot store fits in memory by design (SIEVE eviction ensures this). No buffer management overhead.
- **Type dispatch:** Polars uses dynamic dispatch for column operations across arbitrary types. hilavitkutin's column types are monomorphized at compile time -- each morsel operation is a concrete function, not a trait object call.

### 4.3 Where They Win

- **Complex joins:** DuckDB's hash join and sort-merge join are highly optimized. hilavitkutin does not implement joins -- cross-entity queries must be expressed as multi-stage pipelines with explicit intermediate columns.
- **Adaptive execution:** DuckDB's Morsel-Driven Parallelism paper (Leis et al., SIGMOD 2014) describes adaptive morsel sizing and work-stealing across cores. hilavitkutin inherits these concepts but saalis runs sync-core, limiting parallelism to background work units.
- **Window functions and complex aggregations:** DuckDB implements the full SQL window function specification. hilavitkutin provides the primitives (sorted column access, running accumulators) but complex windowed analytics would need to be composed from simpler stages.

### 4.4 The Verdict

For saalis's query patterns -- filter, sort, paginate, aggregate, group-by, search across a single entity type with occasional cross-type analytics -- hilavitkutin provides performance equivalent to DuckDB/Polars without the overhead of a general-purpose query engine. The specialization is the advantage.

---

## 5. Notion-Like Flexible Views with Analytical Query Performance

### 5.1 The Notion Performance Problem

Notion databases degrade at 5,000 rows and become painful at 10,000-20,000. This is not because Notion uses a bad database (PostgreSQL, sharded) -- it is because Notion's block-per-row model requires reconstructing rows from scattered blocks, applying filters in application code, and transferring filtered results over the network to the client.

### 5.2 Why saalis Can Do Better

Saalis's architecture eliminates every bottleneck in Notion's pipeline:

1. **No network transfer for queries.** Saalis is a single binary; the hot store, query engine, and web server are in the same process. A DataView query is a function call, not an HTTP round-trip to a database service.
2. **No row reconstruction.** Data is already in columnar form in the hot store. A projection reads exactly the columns needed.
3. **No application-layer filtering.** Filters execute as morsel operations on columnar data, not as predicate evaluation over deserialized JSON objects.
4. **No client-side sorting.** Sort runs on the hot store's columnar data and produces a pre-sorted result. The client receives an already-ordered HTML fragment via HTMX.

The result: a user with 100,000 games can switch from Gallery view to Table view, apply a compound filter (platform = SNES AND rating > 8 AND status = downloaded), sort by release year descending, and see the first page of results -- all in under 5 ms end-to-end. This is Notion's view flexibility with 100x the entity count and 100x faster response.

### 5.3 View Switching as Column Selection

View switching (Gallery, Table, Board, List) maps directly to changing which columns are projected:

| View | Projected columns |
|------|-------------------|
| Gallery | cover_art_url, title, platform, rating, status (5) |
| Table | all visible columns selected by user (5-15) |
| Board | title, status (grouped), assignee (3) |
| List | title, platform, rating (3) |

Switching views does not re-query the hot store -- it re-runs the projection stage of the pipeline with a different column set. The filter and sort stages are shared. The morsel pipeline supports this natively: the projection is the last stage, and replacing it is a constant-time operation.

---

## 6. Chain Partitioning: Fusing Filter-Sort-Paginate

### 6.1 The Algorithm

Chain partitioning is a technique from streaming query engines where consecutive pipeline stages that operate on the same morsel are fused into a single function call, eliminating intermediate materialization. For DataView queries, the chain is:

```
Filter(predicate) → Sort(column, direction) → Paginate(offset, limit) → Project(columns)
```

In a naive implementation, each stage produces an intermediate result:
- Filter produces a `Vec<usize>` of matching row indices
- Sort reorders the index vector
- Paginate slices the vector
- Project reads column values for the surviving indices

With chain partitioning, the filter produces a selection bitmask that the sort reads directly (no index materialization). The sort uses a partial sort (selection algorithm) that finds only the top-K elements needed for the requested page, avoiding a full sort of the entire filtered dataset. The projection reads only the K surviving row positions.

### 6.2 Partial Sort Optimization

For pagination (page 1 of 50 results from 100k filtered entities), a full sort is O(N log N) = O(100k * 17) = 1.7M comparisons. A partial sort to find the top 50 is O(N + K log K) = O(100k + 50 * 6) = 100.3k comparisons -- a 17x reduction. For subsequent pages, the partial sort with offset (find elements 51-100) is only marginally more expensive.

This optimization is why DuckDB's top-N operator outperforms PostgreSQL's `ORDER BY ... LIMIT` for large tables. hilavitkutin can implement the same optimization as a pipeline stage: a `TopK` work unit that maintains a K-element heap while scanning the filtered selection vector.

### 6.3 Compound Filters as Bitmask Algebra

Multiple filters (platform = SNES AND rating > 8 AND year > 1994) produce independent bitmasks that are AND-ed together. Bitmask AND is a single SIMD instruction per 256 bits (32 entities). For 100k entities, that is ~3,125 instructions -- nanoseconds. Adding or removing a filter pill in the UI re-evaluates one bitmask and re-ANDs, achieving effectively zero latency for compound filter changes.

OR filters and NOT filters compose identically through bitmask OR and XOR/ANDN. The filter builder UI (drop-down property, operator, value) maps 1:1 to bitmask generation on the hot store.

---

## 7. Real-Time Aggregation for Dashboard Widgets

### 7.1 The Use Case

Dashboard widgets need aggregations: total games, games per platform, games per genre, total storage used, download progress summary, recently added count. In a traditional architecture, these are SQL queries that scan the database on every page load, or cached values with staleness windows.

### 7.2 Columnar Aggregation Performance

On columnar data, aggregation is embarrassingly fast:

- **COUNT:** Number of set bits in a selection bitmask. On 100k entities, this is a `popcnt` loop over ~12 KB of bitmask data -- sub-microsecond.
- **SUM:** Iterate a numeric column, accumulating. With SIMD, 100k 64-bit values sum in ~12 microseconds (8 values per 256-bit SIMD lane, ~12.5k iterations at ~1 ns each).
- **GROUP BY + COUNT:** Hash-aggregate over a dictionary-encoded column. With 50 distinct platforms, the hash table fits in L1 cache. 100k lookups + increments complete in ~50 microseconds.
- **MIN/MAX:** Single pass through a numeric column. Sub-microsecond for 100k values.

These numbers mean that dashboard aggregations can be computed on demand, on every page load, without caching. There is no staleness. The user always sees current numbers.

### 7.3 Aggregation as Pipeline Stages

Each dashboard widget is a pipeline stage:

```
SelectAll → Aggregate(count) → "Total Games"
SelectAll → GroupBy(platform) → Count → "Games per Platform (chart data)"
Filter(status = downloaded) → Sum(file_size) → "Storage Used"
Filter(added_after = 7_days_ago) → Count → "Added This Week"
```

These pipelines share the `SelectAll` source and can be fused: a single pass through the data feeds multiple aggregation accumulators. This is the vectorized aggregation technique that DuckDB uses for multi-aggregate queries.

---

## 8. Novel Ideas

### 8.1 Vectorized String Matching for Search

Full-text search in content libraries is typically handled by dedicated engines (FTS5 in SQLite, Tantivy, Meilisearch). These build inverted indexes and support ranked retrieval. But for saalis's primary use case -- fuzzy matching against entity titles -- the columnar hot store offers an alternative.

The title column, stored as interned string handles (u32), can be searched by:

1. **Prefix matching:** Build a sorted array of (string_hash, handle) pairs. Binary search for the prefix hash, then verify against the actual string. This is O(log N) per search query.
2. **Substring matching:** Use SIMD-accelerated string scanning (`memchr` crate techniques) over the interned string pool. At 100k titles averaging 30 bytes each, the total string data is ~3 MB -- small enough to scan in under 1 ms with AVX2.
3. **Fuzzy matching:** Compute edit distance or Jaro-Winkler similarity between the query and each title. Naive per-character comparison at 100k * 30 characters = 3M character comparisons. With SIMD byte comparison (32 bytes per instruction), this completes in ~100k instructions. For finer similarity scoring, a pre-computed trigram index (stored as an auxiliary column in the hot store) enables sub-millisecond fuzzy lookup.

The key insight: at 100k entities, brute-force SIMD scanning of the string pool is competitive with indexed search, and it requires zero index maintenance. Every entity mutation is instantly searchable because there is no secondary index to update.

### 8.2 Incremental View Maintenance

When underlying data changes (a new game is catalogued, a rating is updated, a download completes), every active DataView that depends on the changed columns must update. Traditional approaches either invalidate the cache (forcing a full re-query) or use materialized view maintenance (tracking deltas and applying them incrementally).

hilavitkutin's column dependency tracking enables precise incremental maintenance:

1. Each DataView records which columns its pipeline reads (known at compile time from `type Read: ColumnSlices`).
2. When a write work unit modifies a column, the hot store identifies which DataViews depend on that column.
3. For each affected DataView, the engine re-evaluates only the changed rows against the pipeline. If a new entity passes the filter and belongs in the current page's sort window, it is inserted. If an existing entity no longer passes, it is removed.
4. The delta (inserted/removed/updated rows) is pushed to the UI via an HTMX out-of-band swap or a WebSocket event.

This is real-time view maintenance -- the UI shows the current state of the library at all times without polling or manual refresh. When a download completes and the `download_status` column updates, the Board view's "Downloaded" column gains a card and the "Downloading" column loses one, with no page reload.

The cost is proportional to the number of changed rows (typically 1-10 per event), not the total dataset size. Even with 100k entities, updating a DataView for a single row change is sub-microsecond.

### 8.3 Computed Columns as Pipeline Stages

Some DataView columns are not stored -- they are derived from other columns:

- **"Completion %"** = files_downloaded / files_total * 100
- **"Age"** = current_date - release_year
- **"Match Score"** = weighted combination of rating, popularity, and user preference
- **"Storage Estimate"** = file_count * average_file_size (for games not yet downloaded)

In a SQL database, these are either computed at query time (expensive for complex formulas, recomputed on every query) or stored as materialized columns (stale, requires triggers).

In hilavitkutin, a computed column is a pipeline stage that reads source columns and writes a derived column:

```rust
struct ComputionPercent;

impl WorkUnit for CompletionPercent {
    type Read = (FilesDownloaded, FilesTotal);
    type Write = (CompletionPct,);

    fn process_batch(read: Self::Read, write: Self::Write) -> Result<()> {
        let (downloaded, total) = read;
        let (pct,) = write;
        for i in 0..downloaded.len() {
            pct[i] = if total[i] > 0 {
                (downloaded[i] as f64 / total[i] as f64) * 100.0
            } else {
                0.0
            };
        }
        Ok(())
    }
}
```

The computed column is a first-class column in the hot store. It can be filtered, sorted, and aggregated like any other column. The pipeline scheduler knows that `CompletionPercent` reads `FilesDownloaded` and `FilesTotal` and writes `CompletionPct` -- so it automatically re-runs the computation when either source column changes. The dependency is declared in the type system, not in application logic.

This enables user-defined "formula columns" (analogous to Notion formulas) that run at columnar speed. A user creates a custom computed column; the system generates a pipeline stage; the stage runs as part of every DataView query that projects that column. At 100k entities, even complex multi-column computations complete in microseconds.

### 8.4 Cross-Entity-Type Analytics

The research on universal entity models identifies polymorphic queries as a key challenge: "show me all entities sorted by name" requires knowing which metadata table stores the name for each entity type. In row-oriented storage, this means N queries or a UNION ALL.

In the columnar hot store, cross-type analytics become tractable through virtual columns. A `display_name` virtual column is computed from the type-specific name columns:

```
For EntityType::Game  → game_metadata.title
For EntityType::Movie → movie_metadata.title
For EntityType::User  → user_metadata.username
```

The virtual column materializes lazily: when a cross-type DataView requests `display_name`, the pipeline reads the `entity_type` column and the relevant type-specific columns, producing a unified `display_name` column. This is a scatter-gather operation on columnar data -- efficient because each branch reads a contiguous column slice.

Cross-type analytics unlock powerful views:

- **"All content" dashboard:** Games + movies + music in one table, sortable by any shared property (name, added date, size, status).
- **"Storage analysis":** Aggregate file sizes across all entity types, grouped by type, showing which content category uses the most disk space.
- **"Activity timeline":** Interleave game play sessions, movie watches, and music listens in a unified timeline, sorted by date.
- **"User impact":** For multi-user scenarios, show which user's content library is largest, most active, or most diverse.

These queries are impossible in Notion (limited to single-database views), difficult in Fibery (cross-space queries are limited), and natural in a columnar engine where all entity types share the same physical column store.

---

## 9. Applicable Query Patterns from DataFusion and Polars

### 9.1 Predicate Pushdown

DataFusion pushes filter predicates as close to the data source as possible. In saalis, predicate pushdown means evaluating filters at the morsel level before any other processing. The filter stage produces a selection bitmask that all subsequent stages respect. This is the default behavior of hilavitkutin's pipeline model -- no additional optimization pass needed.

### 9.2 Projection Pushdown

DataFusion eliminates unused columns from scans. hilavitkutin enforces this at compile time through the `type Read` associated type. A DataView that displays 5 columns physically cannot read the other 25. This is stronger than DataFusion's optimizer-based projection pushdown because it is guaranteed by the type system, not by an optimization pass that might fail.

### 9.3 Late Materialization

Polars uses late materialization: instead of materializing intermediate results as full column arrays, it passes row indices through the pipeline and only reads column values at the final projection stage. This minimizes memory bandwidth for selective queries (where filters eliminate most rows).

hilavitkutin's selection bitmask serves the same purpose. After filtering, only the sort stage and projection stage access column data, and only for rows that passed the filter. For a DataView with a selective filter (e.g., "platform = Neo Geo" matching 200 out of 100k entities), late materialization reduces column reads by 500x.

### 9.4 Adaptive Morsel Sizing

The original Morsel-Driven Parallelism paper (Leis et al., SIGMOD 2014) describes adaptive morsel sizing based on the number of active threads and the pipeline's computational intensity. For saalis's sync-core architecture, adaptive sizing takes a different form: morsel size adapts to L1 cache occupancy based on the number of columns being processed. A filter reading 1 column uses large morsels (more rows per batch); a projection reading 10 columns uses smaller morsels (fewer rows, but each row spans more column data). The total bytes per morsel remains constant at L1 cache size.

### 9.5 Pipeline Fusing from Polars

Polars fuses consecutive map operations into a single pass. In hilavitkutin, consecutive pipeline stages that have no data dependency between them (e.g., computing two independent derived columns) can be fused into a single morsel pass. The DAG scheduler identifies independent stages through the `Read`/`Write` type declarations and schedules them within the same morsel iteration.

### 9.6 Hash Aggregation from DuckDB

DuckDB's external aggregation (2024) uses a two-phase approach: partition the input by hash into thread-local hash tables, then merge. For saalis's single-threaded hot path, the simpler approach works: a single hash table aggregation over the morsel stream, exploiting the fact that group-by cardinality is low (50 platforms, 20 genres, 5 download states). The hash table fits in L1 cache, so the aggregation is memory-bound at cache speed.

### 9.7 Reservoir Sampling for Approximate Aggregations

For dashboard widgets that show approximate statistics ("~47,000 games"), reservoir sampling over the morsel stream provides O(1)-space approximate counts, averages, and percentiles. This is relevant for very large libraries where even a full columnar scan might be unnecessary for a dashboard widget.

---

## 10. Architectural Implications

### 10.1 The Hot Store IS the Query Engine

The central thesis: hilavitkutin is not a cache with a query layer bolted on. It is the query engine, and caching (keeping data in columnar form in memory) is a side effect of its execution model. The SIEVE eviction policy manages which column segments are resident. The morsel pipeline processes whatever is resident. SQLite is the durable backing store, not the query engine.

This inverts the traditional architecture. Instead of "SQLite is the database, the hot store is a cache," it becomes "the hot store is the database for reads, SQLite is the persistence layer for writes." CQRS is not an optimization -- it is the fundamental architecture.

### 10.2 DataView as Compiled Query Plan

Each DataView configuration (view type, filters, sorts, visible columns, group-by) corresponds to a compiled pipeline. The DataView builder API (§5 of the DataView Builder API design doc) already expresses this: `DataViewBuilder<R: Row>` carries the row type through the entire construction process, and `.build()` erases the type only at the boundary with the template layer.

With hilavitkutin integration, the pipeline stages are:

1. **Source:** Column segments from the hot store, identified by the DataView's entity type
2. **Filter stages:** One per active filter, each producing a selection bitmask
3. **Bitmask merge:** AND/OR the bitmasks according to the filter builder's logic
4. **Sort stage:** TopK partial sort on the designated sort column, respecting the merged bitmask
5. **Computed columns:** Any derived columns needed by the projection
6. **Projection:** Read only the columns needed for the current view type
7. **Materialization:** Convert columnar output to `DataViewItem` for template rendering

Steps 1-6 operate entirely within the columnar hot store. Step 7 is the only row-oriented operation, and it runs on at most `page_size` (typically 50) items.

### 10.3 Memory Budget Integration

DuckDB's unified memory pool concept applies directly. The hot store's memory budget covers both:

- **Cached column segments:** The resident data, managed by SIEVE eviction
- **Pipeline working memory:** Selection bitmasks, sort heaps, aggregation hash tables

At 100k entities, the pipeline working memory is negligible: a selection bitmask is 12.5 KB, a top-50 sort heap is ~400 bytes, an aggregation hash table for 50 groups is ~800 bytes. The vast majority of the memory budget goes to cached column segments.

### 10.4 Write Path: SQLite First, Hot Store Second

Writes flow through SQLite (source of truth) first, then propagate to the hot store:

1. Scheduler work unit writes to SQLite via `ctx.write()`
2. SQLite WAL accepts the write
3. The hot store's column invalidation listener marks affected segments dirty
4. Dirty segments are refreshed from SQLite (or updated in-place if the delta is known)
5. Incremental view maintenance pushes updates to active DataViews

This ensures durability (SQLite WAL) while keeping the hot store fresh. The staleness window is bounded by the time to process steps 3-5, which is sub-millisecond for single-row changes.

---

## 11. What This Means for Users

The user does not know or care about morsel-driven columnar execution. What they experience:

- **Instant filter response.** Clicking a filter chip updates the view before their finger lifts from the mouse button. Not "fast" -- instant. There is no loading spinner, no skeleton screen, no perceptible delay.
- **Instant sort response.** Clicking a column header re-sorts 100,000 entities and shows the result immediately.
- **Instant view switching.** Switching from Gallery to Table to Board rearranges the same data without re-querying.
- **Live dashboards.** The "total games" counter, the "games per platform" chart, and the "storage used" bar update in real time as downloads complete and new games are catalogued.
- **Search that works.** Typing in the search box produces fuzzy-matched results as fast as the user can type, with no indexing delay for newly added entities.
- **No scale wall.** A library of 500 games and a library of 100,000 games feel identical. There is no performance cliff at 5,000 entities (Notion), no "please wait while we load your collection" (LaunchBox with large libraries), no pagination-as-workaround.

This is the promise: analytical database performance, content library UX, single Rust binary, 256 MB of RAM, running on a Batocera appliance. Nobody else offers this combination because nobody else has thought to point a morsel-driven columnar engine at a content library's UI layer.

---

## Sources

- [Morsel-Driven Parallelism: A NUMA-Aware Query Evaluation Framework (Leis et al., SIGMOD 2014)](https://15721.courses.cs.cmu.edu/spring2024/papers/07-scheduling/p743-leis.pdf)
- [DuckDB Memory Management (2024)](https://duckdb.org/2024/07/09/memory-management)
- [DuckDB External Aggregation (2024)](https://duckdb.org/2024/03/29/external-aggregation)
- [DataFusion SIGMOD 2024 Paper](https://andrew.nerdnetworks.org/pdf/SIGMOD-2024-lamb.pdf)
- [SIEVE: Simpler than LRU (NSDI '24)](https://www.usenix.org/conference/nsdi24/presentation/zhang-yazhuo)
- [Notion: Optimize Database Load Times](https://www.notion.com/help/optimize-database-load-times-and-performance)
- [Pushing Notion to the Limits](https://notionmastery.com/pushing-notion-to-the-limits/)
- [The Data Model Behind Notion](https://www.notion.com/blog/data-model-behind-notion)
- [Polars Internal Architecture](https://docs.pola.rs/user-guide/concepts/streaming/)
- [Late Materialization in Columnar Stores (Abadi et al., ICDE 2007)](https://www.cs.umd.edu/~abadi/papers/abadi-sigmod-2008.pdf)
- [SQLite Query Planner](https://sqlite.org/queryplanner.html)
