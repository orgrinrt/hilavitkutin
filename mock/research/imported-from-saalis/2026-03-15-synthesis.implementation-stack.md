# Implementation Stack: Definitive Crate and Technology Choices for Saalis v1

**Date:** 2026-03-15
**Status:** Definitive synthesis
**Inputs:** All research documents (2026-03-14), all deep-dive documents (2026-03-15), final architecture summary (2026-03-14)
**Purpose:** Concrete, actionable implementation stack. Every external crate, every nightly feature, every build configuration decision, in one place.

---

## 1. Crate Dependency Table

Every external crate saalis will use at v1, organised by saalis crate membership. Version ranges follow semver; pin to the latest compatible release at implementation time.

### saalis-primitives

| Crate | Version | Provides | Rationale |
|-------|---------|----------|-----------|
| `serde` | 1.x | `Serialize` / `Deserialize` derives | Universal serialisation; `SettingsValue` requires it |
| `serde_json` | 1.x | JSON serialisation | Settings import/export, API response parsing |
| `inventory` | 0.3 | Typed distributed registration (`submit!`, `iter`) | `#[register]` pattern; constructor-section-based, zero-runtime-cost discovery |
| `strsim` | 0.11 | Levenshtein, Jaro-Winkler, Damerau-Levenshtein | Fuzzy search fallback for "did you mean?" suggestions |
| `blake3` | 1.x | BLAKE3 cryptographic hash (streaming, rayon parallel) | Primary content integrity hash; 3-4 GB/s single-threaded, Merkle-tree parallel via rayon |
| `parking_lot` | 0.12 | `Mutex`, `RwLock`, `Condvar` (sync primitives) | Faster than `std::sync` (no poisoning, smaller footprint); used throughout sync core |
| `flume` | 0.11 | Multi-producer, multi-consumer channel (sync + async) | Bridge between sync scheduler and async SSE; `send()` is blocking, `recv_async()` is async, same channel |
| `thumbhash` | 0.1 | Image placeholder hash (25-35 bytes) | Aspect-ratio-aware placeholders for gallery; no configuration parameters |

### saalis-derive

| Crate | Version | Provides | Rationale |
|-------|---------|----------|-----------|
| `syn` | 2.x | Rust syntax parser (full AST) | Parse impl blocks for `#[register]`, struct definitions for `#[derive(Settings)]` |
| `quote` | 1.x | Quasi-quoting for TokenStream generation | Generate descriptor structs, `inventory::submit!` calls, trait impls |
| `proc-macro2` | 1.x | Compiler-independent proc-macro types | Testable macro logic; all internal code uses `proc_macro2` types |
| `darling` | 0.20 | Ergonomic attribute parsing for derive macros | Structured parsing of `#[setting(...)]` helper attributes; scales to complex attribute grammars |

### saalis-sdk

| Crate | Version | Provides | Rationale |
|-------|---------|----------|-----------|
| `ureq` | 3.x | Synchronous HTTP client | Genuinely sync (no hidden tokio); `#![forbid(unsafe_code)]`; rustls default for static binaries; first-class `Middleware` trait |
| `governor` | 0.8 | GCRA rate limiter | Per-connector rate limiting; token-bucket with burst capacity; `RateLimiter` is `Send + Sync` |
| `sieve-cache` | 0.2 | SIEVE eviction cache (weighted, sharded) | Hot store eviction; `WeightedSieveCache` for variable-size column segments; 2x LRU throughput at 16 threads |

### saalis-core

| Crate | Version | Provides | Rationale |
|-------|---------|----------|-----------|
| `rusqlite` | 0.32+ | SQLite bindings (sync) | Direct sync API; `bundled` feature compiles SQLite into binary; `prepare_cached` for statement reuse |
| `rusqlite_migration` | 1.x | Schema migration via `PRAGMA user_version` | Lightweight; uses correct mechanism (`user_version` in DB header); supports up/down migrations |
| `fast_image_resize` | 5.x | SIMD-accelerated image resizing (NEON on ARM, AVX2 on x86) | 2-2.5x faster than `image` crate on ARM64; essential for cover art thumbnail pipeline |
| `image` | 0.25 | Image decode/encode (JPEG, PNG, WebP, AVIF) | Format breadth; pure Rust; no C dependencies; `fast_image_resize` handles the resize, `image` handles decode/encode |
| `webp` | 0.3 | Lossy WebP encoding (via `libwebp-sys`) | `image` crate only supports lossless WebP; lossy encoding gives 25-34% smaller files than JPEG |
| `zip` | 2.x | ZIP archive read/write (streaming) | Streaming extraction; central directory allows selective file extraction |
| `tar` | 0.4 | tar archive read/write (streaming) | Inherently streaming; pairs with flate2/zstd for compressed tarballs |
| `flate2` | 1.x | gzip/deflate compression | tar.gz extraction |
| `zstd` | 0.13 | Zstandard compression | tar.zst extraction; best ratio-speed trade-off for modern archives |
| `sevenz-rust` | 0.6 | 7z archive extraction | Streaming 7z support; LZMA2 decompression with bounded memory |
| `reflink-copy` | 0.1 | CoW file cloning (btrfs/APFS reflinks) | Instant file installation on btrfs; falls back to copy on ext4 |
| `jwalk` | 0.8 | Parallel directory traversal (rayon-backed) | 4x faster than single-threaded walkdir for startup library reconciliation |
| `notify` | 7.x | Cross-platform filesystem watching (inotify on Linux) | Real-time detection of ROM file changes after initial scan |
| `atomic-write-file` | 0.2 | Crash-safe write-fsync-rename | Correct atomic file writes including directory fsync; `tempfile::persist()` is insufficient |
| `crc32fast` | 1.x | CRC32 checksum (hardware-accelerated) | No-Intro / Redump ROM database compatibility |
| `rayon` | 1.x | Data parallelism thread pool | BLAKE3 parallel hashing (`update_rayon`); image batch processing; jwalk backend |
| `dashmap` | 6.x | Concurrent hash map (sharded) | Lock-free reads; used for concurrent caches and lookup tables shared across scheduler and web layer |
| `crossbeam-channel` | 0.5 | High-performance sync-only channel | Intra-scheduler communication; zero async overhead |
| `tempfile` | 3.x | Temporary files and directories | Extraction staging; test fixtures |
| `nix` | 0.29 | Unix syscall wrappers (`fallocate`, `posix_fadvise`, `statfs`) | Download preallocation; filesystem detection; read-ahead hints |
| `libloading` | 0.8 | Dynamic library loading (`dlopen`/`dlsym`) | Future runtime plugin loading via `repr(C)` vtable + `extern "C"` entry point |

### saalis-web

| Crate | Version | Provides | Rationale |
|-------|---------|----------|-----------|
| `axum` | 0.8 | Async web framework (Router, extractors, SSE) | Built on hyper/tokio; first-class SSE via `axum::response::sse`; `IntoResponse` for maud `Markup` |
| `tokio` | 1.x | Async runtime | Required by axum; `rt-multi-thread` and `macros` features; only used in `saalis-web` |
| `maud` | 0.27 | Compile-time HTML templates (proc macro) | 72us big-table benchmark; zero-alloc rendering; `Render` trait for composable components; `axum` feature for `IntoResponse` |
| `axum-htmx` | 0.6 | HTMX extractors and response headers for axum | `HxRequest`, `HxBoosted` extractors; `HxTrigger`, `HxPushUrl` response headers |
| `tower-http` | 0.6 | HTTP middleware (compression, CORS, static files) | `ServeDir` for static assets; `CompressionLayer` for response compression |
| `tokio-stream` | 0.1 | Stream utilities for tokio | `BroadcastStream` adapter for SSE; converts `broadcast::Receiver` into `Stream` |

### Shared / workspace-level dev-dependencies

| Crate | Version | Provides | Rationale |
|-------|---------|----------|-----------|
| `trybuild` | 1.x | Compile-pass and compile-fail tests | Proc macro error message testing; snapshot `.stderr` files |
| `macrotest` | 1.x | Expansion snapshot tests | Verify `#[register]` and `#[derive(Settings)]` expansion stability |
| `tempfile` | 3.x | Temporary files/dirs for tests | Filesystem and SQLite test isolation |

---

## 2. Per-Crate Breakdown

### 2.1 saalis-primitives

**Purpose:** Zero-cost type definitions, marker traits, enums, newtypes. The foundation that every other crate depends on. No business logic, no I/O.

**External deps:** `serde`, `serde_json`, `inventory`, `strsim`, `blake3`, `parking_lot`, `flume`, `thumbhash`

**Key internal modules:**
- `entity` -- `EntityType` trait, `EntityId` newtype, `SupportsEntity<E>`, `HasMetadata<M>`
- `schema` -- `MetadataTable` trait, `Column<In<M>, As<V>>`, `In<M>`, `As<V>` ZST wrappers
- `priority` -- `Priority` enum (`repr(C)`), `Urgency` enum
- `error` -- `ErrorClass` enum, `ErrorKind` trait, `Transient`/`Fixable`/`Permanent` ZSTs, `AppError<K>` trait
- `settings` -- `SettingsValue` trait, `SettingsDomain` trait, `AccessLevel` trait, `SetAccess<R, A>` trait, `Secret` trait, `Credential` trait, `FieldKind` enum
- `connector` -- `Connector` trait, `Cataloguer<E>`, `Enricher<E>`, `Sourcer<E, S>`, `SourceType`, `SourceUrl`
- `scheduler` -- `WorkUnit<F, W, R>` trait, `EventKind` trait, `EventKindSet`, `Subsystem` trait, `Curator`/`Housekeeper`/`Doctor` ZSTs
- `notification` -- `NotificationPayload` trait
- `identity` -- `Role` trait, `Admin`/`User`/`Child` ZSTs, `LinkedIdentityProvider` trait
- `storage` -- `StorageLocation` trait, location ZSTs (`Roms`, `Temp`, `Cache`, etc.)
- `download` -- `Download` entity ZST, `Downloader<S>` trait, `ArchiveFormat` trait, `Extractor<F>` trait
- `theme` -- `Theme` trait, `ThemeProperty` trait, `Provides<P, G>` trait, property ZSTs
- `str` -- `Str` type (`Cow<'static, str>`), string interning types

**Nightly features required:**
- `min_specialization` -- `SupportsEntity`/`HasMetadata` blanket bridge; `SetAccess` default-access matrix

### 2.2 saalis-derive

**Purpose:** Procedural macros. Single crate to avoid compounding pipeline-blocking compilation.

**External deps:** `syn` (features: `full`), `quote`, `proc-macro2`, `darling`

**Key internal modules:**
- `register` -- `#[register]` attribute macro: parses `impl Trait for Type`, emits original impl + `#[repr(C)]` descriptor + `inventory::submit!`
- `settings` -- `#[derive(Settings)]` derive macro: parses struct fields with `#[setting(...)]` attributes, emits `SettingsValue` impl + entry metadata + registration

**Nightly features required:** None (proc macros run on stable proc-macro API; the generated code uses nightly features, but the macro crate itself does not)

### 2.3 saalis-sdk

**Purpose:** The public API surface for connectors and plugins. Re-exports from primitives plus context types, builders, and HTTP/rate-limiting infrastructure.

**External deps:** `ureq` (features: `json`, `cookies`, `gzip`, `brotli`, `socks-proxy`), `governor`, `sieve-cache` (features: `weighted`, `sync`, `sharded`)

**Key internal modules:**
- `http` -- `HttpClient` trait impl backed by ureq `Agent`; per-connector middleware chain (rate limiter, retry, auth injection)
- `context` -- `ConnectorContext<C>`, `SchedulerContext`, `DownloadContext`, `ExtractionContext`
- `query` -- `QueryBuilder<M>`, `WriteBuilder<M>`, `.of::<T>(id)` API
- `cache` -- `WeightedSieveCache` wrapper for hot store segments; `SegmentKey`, `ColumnSegment` types
- `rate_limit` -- `governor::RateLimiter` per connector; adaptive adjustment from response headers; budget counters

**Nightly features required:**
- `min_specialization` (via re-export of primitives traits that use it)

### 2.4 saalis-core

**Purpose:** The host binary's business logic. Scheduler, persistence, filesystem operations, image pipeline, search. Sync-only; no tokio dependency.

**External deps:** `rusqlite` (features: `bundled`, `backup`, `column_decltype`), `rusqlite_migration`, `fast_image_resize`, `image`, `webp`, `zip`, `tar`, `flate2`, `zstd`, `sevenz-rust`, `reflink-copy`, `jwalk`, `notify`, `atomic-write-file`, `crc32fast`, `rayon`, `dashmap`, `crossbeam-channel`, `tempfile`, `nix`, `libloading`

**Key internal modules:**
- `cold_store` -- `ColdStore` struct (owns write + read `rusqlite::Connection`); typed query wrappers via `ColumnDescriptor` trait bridge; epoch-batched flush; WAL checkpoint control
- `cold_store::migrations` -- SQL files loaded via `include_str!`; applied via `rusqlite_migration`
- `hot_store` -- Columnar segment cache backed by `WeightedSieveCache`; SIEVE eviction; startup warming (4-tier priority); `_hot_manifest` table
- `sync_layer` -- Epoch lifecycle; dirty segment collection; phase-locked eviction/checkpoint; write-only mode detection
- `scheduler` -- Event-driven DAG; work unit dispatch; priority queues; per-connector concurrency caps; `crossbeam-channel` for internal messaging
- `search` -- FTS5 external content table management; tokenizer configuration (`unicode61 remove_diacritics 2 tokenchars '-'`); prefix indexes (`2 3 4`); BM25 with column weights; query sanitisation; `strsim` fuzzy fallback
- `image_pipeline` -- Decode via `image`, resize via `fast_image_resize` (NEON/AVX2), encode to WebP (lossy via `webp` crate) and JPEG fallback; ThumbHash generation; memory-bounded (semaphore, decode at reduced scale)
- `acquisition` -- Download/verify/extract/install/organise work unit DAG; streaming BLAKE3 during download; `reflink-copy` for installation; content-addressable store; symlink trees per profile
- `filesystem` -- `FsCapabilities` detection at startup (`statfs`); `jwalk` reconciliation; `notify` watching; `fallocate` preallocation; `atomic-write-file` for crash safety
- `connector_runtime` -- Batch assembly; cache check (response cache with TTL tiers); budget check; rate limiter gate; circuit breaker; metadata fusion

**Nightly features required:**
- `min_specialization` (via primitives)

### 2.5 saalis-web

**Purpose:** Async shell. axum server, HTMX-driven UI, SSE push, static asset serving. The only crate that depends on tokio.

**External deps:** `axum`, `tokio` (features: `rt-multi-thread`, `macros`, `signal`), `maud` (features: `axum`), `axum-htmx`, `tower-http` (features: `fs`, `compression-full`, `cors`), `tokio-stream`

**Key internal modules:**
- `server` -- axum `Router` setup; socket binding; graceful shutdown
- `handlers` -- Route handlers; each wraps sync core calls in `tokio::task::spawn_blocking`
- `sse` -- `/events` endpoint; `tokio::sync::broadcast` channel fed by sync scheduler; SSE with `KeepAlive`; event types: `library-updated`, `download-progress`, `notification`, `scheduler-status`
- `templates` -- maud-based module tree:
  - `layout.rs` -- `base()` function (full HTML shell with htmx script)
  - `components/` -- `nav.rs`, `card.rs`, `form.rs`, `modal.rs`
  - `pages/` -- `home.rs`, `library.rs`, `entity.rs`, `settings.rs`
  - `fragments/` -- HTMX swap targets: `entity_list.rs`, `search.rs`, `metadata.rs`
- `static_assets` -- `tower-http::services::ServeDir` for CSS, htmx JS, gamepad nav JS
- `image_server` -- Content-negotiated image serving (Accept header: AVIF > WebP > JPEG); content-hash URLs with `Cache-Control: immutable`; on-disk thumbnail cache with LRU eviction

**Nightly features required:** None beyond what propagates from primitives

---

## 3. Technology Choices with Rationale

### 3.1 Persistence

**SQLite via rusqlite (bundled, WAL mode)**

The cold store is a single SQLite file compiled into the binary via the `bundled` feature of `libsqlite3-sys`. WAL mode enables concurrent readers alongside the single scheduler writer. Connection management is static allocation: one write connection (`PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL; PRAGMA cache_size=-8000; PRAGMA foreign_keys=ON; PRAGMA temp_store=MEMORY`) owned by the scheduler, one read connection (`PRAGMA query_only=1`) for demand loads. No connection pool (r2d2 dropped).

Chosen over SeaORM because saalis is sync-core. SeaORM's async wrapping adds measurable overhead (12x for a 500-row batch) for zero benefit. The cold store's query surface is 10-15 prepared statements -- an ORM is overkill. `rusqlite_migration` replaces SeaORM's migration framework using `PRAGMA user_version`, which is atomic with the database state.

**SIEVE eviction via sieve-cache**

The hot store uses `WeightedSieveCache` for column segment eviction. SIEVE was selected over LRU (2x throughput at 16 threads), ARC (simpler, matches/beats on web workloads), and S3-FIFO (sufficient scan resistance without the complexity). Weight-based eviction handles variable-size segments naturally. The cache is sharded (16 shards) for concurrent access from scheduler and web layer. NSDI '24 Community Award winner, validated at Meta scale.

### 3.2 HTTP Client

**ureq 3.x**

Genuinely synchronous -- no hidden tokio runtime, no `spawn_blocking` wrapping. The sans-IO architecture (ureq-proto) separates protocol logic from I/O. First-class `Middleware` trait enables composable rate limiting, retry, and auth injection per connector. rustls default produces fully static binaries for Batocera deployment. Streaming bodies (`impl Read`) enable downloading large ROMs without buffering. Connection pooling (3-per-host, 15s idle) handles metadata API reuse automatically.

Chosen over `reqwest::blocking` because reqwest spawns a hidden tokio runtime (architecturally dishonest for a sync core) and pulls in tokio, hyper, and h2 (larger binary, slower compile). HTTP/2 is unnecessary -- all target APIs operate over HTTP/1.1.

### 3.3 Rate Limiting

**governor (GCRA algorithm)**

Per-connector `RateLimiter` configured from settings (`requests_per_second`, `burst_capacity`). GCRA (Generic Cell Rate Algorithm) provides smooth rate limiting without bursts that could trigger server-side throttling. The rate limiter sits in the connector runtime pipeline between batch assembly and the HTTP client. Adaptive adjustment from `X-RateLimit-Remaining` / `Retry-After` response headers supplements static configuration. Budget counters (separate from the rate limiter) track cumulative daily/monthly API usage.

### 3.4 Web Layer

**axum + maud + HTMX (CDN) + SSE**

axum is the async web framework, chosen for its tower middleware ecosystem, first-class extractors, and built-in SSE support (`axum::response::sse::Sse`). maud compiles HTML templates at build time (72us big-table, 124ns teams benchmark -- fastest among compile-time engines). HTMX (14 KB gzipped) handles partial page updates, SSE subscriptions, infinite scroll, debounced search, and out-of-band multi-region swaps. The SSE extension (3 KB) connects to a single `/events` endpoint. Idiomorph (built into HTMX 4.0+) provides morph swaps that preserve focus and scroll state across updates.

Total JS budget: ~22 KB (htmx core + SSE extension + custom gamepad/spatial-nav script). Compare: React + ReactDOM alone is 42 KB gzipped.

SSE was chosen over WebSocket because the communication pattern is unidirectional (server pushes state to browser). SSE provides automatic reconnection with exponential backoff, HTTP/2 multiplexing, and zero additional runtime dependencies (it is just a long-lived HTTP response). `tokio::sync::broadcast` feeds the SSE stream from the sync scheduler -- `broadcast::Sender::send()` is sync-safe.

### 3.5 Search

**FTS5 (external content, unicode61) + strsim**

FTS5 ships with SQLite (zero additional dependency). External content tables avoid duplicating metadata already in the entity tables. Configuration: `tokenize = "unicode61 remove_diacritics 2 tokenchars '-'"` for diacritic-insensitive, hyphen-aware tokenisation. Prefix indexes (`prefix = '2 3 4'`) provide sub-millisecond autocomplete. BM25 ranking with column weights (`title: 10.0, description: 1.0, developer: 5.0, tags: 3.0`) ensures title matches dominate. `highlight()` and `snippet()` produce ready-to-serve HTML fragments.

Fuzzy fallback: when FTS5 returns few results, `strsim::jaro_winkler` computes similarity against the top-N most popular titles. Threshold 0.75-0.80 produces reasonable "did you mean?" suggestions without false positives.

Chosen over Tantivy (separate index directory, segment merges, added binary size) and MeiliSearch/milli (LMDB dependency, unstable internal API). FTS5 is sufficient for 10k-100k game titles and keeps everything in a single WAL-protected SQLite file.

### 3.6 Images

**fast_image_resize + image (decode) + webp (lossy encode)**

Decode via the `image` crate (JPEG, PNG, WebP, BMP). Resize via `fast_image_resize` with NEON on ARM64 (2.4x faster than `image` crate's Lanczos3). Encode to lossy WebP via the `webp` crate (wraps `libwebp-sys` -- small C dependency, 25-34% smaller than JPEG). JPEG fallback for clients that do not support WebP. Content negotiation via `Accept` header in axum handlers.

ThumbHash placeholders (25-35 bytes per image) are generated at enrichment time, stored as blobs, and inlined into the initial HTML payload as `data-thumbhash` attributes. A small client-side script (included in the 22 KB JS budget) decodes them to canvas for instant visual approximation before real images load.

Hybrid thumbnail strategy: pre-generate the `grid` preset (264px wide) during enrichment; generate `detail` (528px) and others on demand with on-disk caching. Memory bounded via sequential processing with explicit drop and decode at reduced JPEG scale (1/8 for large sources).

### 3.7 Checksums

**BLAKE3 (streaming, rayon parallel)**

Primary internal integrity hash. Streaming during download adds zero overhead at network speeds (BLAKE3 processes at 3-4 GB/s single-threaded with NEON on ARM). Post-download verification uses `blake3::Hasher::update_rayon()` for parallel hashing across all cores. Content-addressable store paths are derived from BLAKE3 digests.

`crc32fast` provides CRC32 for No-Intro/Redump ROM database compatibility. Legacy hashes are computed alongside BLAKE3 for provenance verification.

### 3.8 Archives

- **ZIP:** `zip` crate. Streaming extraction. Central directory enables selective file extraction for multi-game archives.
- **tar.gz / tar.zst:** `tar` + `flate2` (gzip) / `zstd` (zstandard). Inherently streaming. Zstandard offers superior ratio-speed tradeoff.
- **7z:** `sevenz-rust`. LZMA2 decompression with bounded memory usage (dictionary size + one output buffer).
- **RAR:** Deferred to v2. The `unrar` crate requires a C binding to the proprietary RAR decompression library. For v1, treat RAR archives as unsupported and prompt the user to extract manually.

### 3.9 Filesystem

- **reflink-copy:** CoW file cloning on btrfs/APFS. Installation of a 4 GB ISO completes in microseconds on btrfs. Falls back to `std::io::copy` (which uses `copy_file_range` on Linux) on ext4.
- **jwalk:** Parallel directory traversal for startup library reconciliation. ~4x throughput vs single-threaded walkdir on directories with 10k+ entries.
- **notify:** Real-time filesystem watching via inotify (Linux). Watches ROM directories for additions, removals, and modifications after the initial scan.
- **atomic-write-file:** Correct write-fsync-rename for crash-safe file operations. Includes the directory fsync that `tempfile::NamedTempFile::persist()` omits.
- **nix:** `fallocate` for download preallocation (early out-of-space detection), `posix_fadvise` for sequential read-ahead hints, `statfs` for filesystem type detection at startup.

### 3.10 Channels and Concurrency

- **flume:** Async/sync bridge channel. The sync scheduler sends events via `sender.send()` (blocking but near-instant). The async SSE handler receives via `receiver.recv_async().await`. Same channel object, no runtime mismatch.
- **crossbeam-channel:** Intra-scheduler sync-to-sync communication. Maximum performance, no async overhead. `select!` macro for multiplexing.
- **parking_lot:** `Mutex` and `RwLock` replacements. No poisoning (a panicked thread does not permanently lock the mutex). Smaller and faster than `std::sync`. Used throughout the sync core.
- **dashmap:** Concurrent sharded hash map. Lock-free reads via epoch-based reclamation. Used for response caches, connector state, and lookup tables shared between scheduler and web layer.
- **rayon:** Data-parallel thread pool. Powers BLAKE3 parallel hashing, jwalk directory traversal, and batch image processing. Separate from tokio's blocking thread pool.

### 3.11 Serialisation

**serde + serde_json**

`serde` provides the `Serialize` / `Deserialize` derive ecosystem. `serde_json` handles JSON for API responses (IGDB, Steam, ScreenScraper), settings import/export, and SSE event payloads. No other serialisation format is needed at v1; bincode or MessagePack could be added later for segment serialisation if profiling demands it.

### 3.12 Plugin Loading

**inventory (static) + libloading (dynamic, future)**

`inventory` handles all in-binary plugin discovery. Every `#[register]`-annotated impl generates an `inventory::submit!` call that runs at program load time via constructor sections (`.init_array` on Linux, `__DATA,__mod_init_func` on macOS). `inventory::iter::<T>()` iterates all registered items of type `T`. No unregistration, no ordering guarantees -- both acceptable for the saalis use case.

`libloading` is included as a dependency for future runtime plugin loading via `dlopen`/`dlsym`. The plugin ABI is `repr(C)` vtable + `extern "C"` entry point + version negotiation. Not exercised at v1 but the dependency is present so the infrastructure can be built incrementally.

### 3.13 Proc Macros

**syn + quote + proc-macro2 + darling**

All macro logic lives in a single `saalis-derive` crate (minimises pipeline-blocking compilation). `syn` 2.x with `features = ["full"]` parses `ItemImpl` (for `#[register]`) and `DeriveInput` (for `#[derive(Settings)]`). `quote` generates the output `TokenStream`. `darling` handles structured attribute parsing for `#[setting(...)]` helper attributes, scaling gracefully as the attribute grammar grows.

Error reporting uses `syn::Error::new_spanned` for precise source-location diagnostics. The generated code includes compile-time trait-bound assertions (zero-cost `const _: () = { ... }` blocks) rather than attempting type introspection at macro expansion time.

### 3.14 Testing

- **trybuild:** Compile-pass and compile-fail snapshot tests for proc macros. Ensures `#[register]` and `#[derive(Settings)]` produce clear, well-located error messages for invalid inputs.
- **macrotest:** Expansion snapshot tests. Catches unintended changes in generated code via `.expanded.rs` diffs.
- **tempfile:** Temporary directories for SQLite database tests and filesystem operation tests. Each test gets an isolated environment.

---

## 4. Nightly Features Inventory

Saalis requires nightly Rust. The following unstable features are used, with their purpose and fallback strategy.

| Feature | Feature Gate | Used In | Purpose | Fallback If Never Stabilised |
|---------|-------------|---------|---------|------------------------------|
| `min_specialization` | `#![feature(min_specialization)]` | `saalis-primitives` | Three patterns: (1) `SupportsEntity`/`HasMetadata` blanket bridge -- a `default impl<E, M> SupportsEntity<E> for M where E: HasMetadata<M>` that can be overridden by explicit impls. (2) `SetAccess` default-access matrix -- `default impl<R, D> SetAccess<R, Hidden> for D` with admin and per-domain overrides. (3) `Secret` blanket from `Credential` (standard blanket, does not itself require specialization but coexists with the above). | (1) Generate explicit `impl SupportsEntity<E> for M` from the `#[register]` macro for each association. The macro knows both sides and can emit both impls. More boilerplate but mechanically automatable. (2) Generate explicit `SetAccess` impls for each `(Role, Domain)` pair from `#[derive(Settings)]`. The macro has the role list and domain list at hand. (3) No change needed -- standard blanket impl works on stable. |

**Risk assessment:** `min_specialization` has been unstable since 2020. The standard library depends on it extensively, which means it is maintained and tested even though unstable. The risk is not breakage but perpetual nightly requirement. The fallback (macro-generated impls) is available at any time and requires only a change to `saalis-derive` output, not to user-facing API.

**Other nightly features considered but NOT used:**
- `specialization` (full) -- unsound (lifetime dispatch vulnerability). Explicitly avoided.
- `generic_const_exprs` -- would enable `Migration<const V: u32>` more ergonomically but is too unstable and frequently broken.
- `adt_const_params` -- would allow const generic parameters of arbitrary types. Deferred; not needed for v1.
- `type_alias_impl_trait` -- stabilised in Rust 1.75. No longer nightly.
- `async_fn_in_trait` -- stabilised in Rust 1.75. Not relevant (sync core).

---

## 5. Build Configuration

### 5.1 Workspace Structure

```toml
# /Cargo.toml (workspace root)
[workspace]
resolver = "2"
members = [
    "crates/saalis-primitives",
    "crates/saalis-derive",
    "crates/saalis-sdk",
    "crates/saalis-core",
    "crates/saalis-web",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "nightly"
license = "AGPL-3.0-or-later"
repository = "https://github.com/orgrinrt/saalis"

[workspace.dependencies]
# Shared versions -- all crates reference these
serde = { version = "1", features = ["derive"] }
serde_json = "1"
inventory = "0.3"
parking_lot = "0.12"
flume = "0.11"
blake3 = { version = "1", features = ["rayon"] }
rayon = "1"
tempfile = "3"
```

### 5.2 Dependency Graph

```
saalis-web
  -> saalis-core
       -> saalis-sdk
            -> saalis-primitives
            -> saalis-derive (proc-macro)
       -> saalis-primitives
       -> saalis-derive (proc-macro)
  -> saalis-primitives
  -> saalis-derive (proc-macro)
```

`saalis-derive` is a proc-macro crate and sits on the critical compilation path. All other crates depend on it transitively. Keeping it lean (only `syn`, `quote`, `proc-macro2`, `darling`) minimises the pipeline-blocking cost.

### 5.3 Feature Flags

```toml
# saalis-core/Cargo.toml
[features]
default = ["bundled-sqlite"]
bundled-sqlite = ["rusqlite/bundled"]
system-sqlite = []  # link against system libsqlite3

# saalis-web/Cargo.toml
[features]
default = []
dev-reload = ["tower-livereload"]  # live reload during development
```

```toml
# rusqlite configuration (saalis-core)
[dependencies.rusqlite]
version = "0.32"
features = ["bundled", "backup", "column_decltype"]
```

```toml
# ureq configuration (saalis-sdk)
[dependencies.ureq]
version = "3"
features = ["json", "cookies", "gzip", "brotli", "socks-proxy"]
```

```toml
# maud configuration (saalis-web)
[dependencies.maud]
version = "0.27"
features = ["axum"]
```

### 5.4 Profile Settings

```toml
# /Cargo.toml (workspace root)

[profile.dev]
opt-level = 0
debug = true
# Fast iteration; no special settings

[profile.dev.package.libsqlite3-sys]
opt-level = 2  # SQLite is painfully slow at opt-level 0

[profile.dev.package.blake3]
opt-level = 2  # Hash performance matters even in dev

[profile.dev.package.fast_image_resize]
opt-level = 2  # SIMD codegen needs optimization

[profile.release]
opt-level = 3
lto = "fat"           # Full link-time optimisation -- smaller binary, better inlining
codegen-units = 1     # Single codegen unit for maximum optimisation (slower compile, faster binary)
strip = "symbols"     # Strip debug symbols from release binary
panic = "abort"       # No unwinding overhead; smaller binary; appliance can just restart
overflow-checks = false  # Performance; checked in dev profile

[profile.release-debug]
inherits = "release"
debug = true
strip = "none"
# For profiling: release-speed code with debug info
```

### 5.5 Cross-Compilation (Batocera ARM target)

```toml
# /.cargo/config.toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[target.aarch64-unknown-linux-musl]
linker = "aarch64-linux-musl-gcc"
rustflags = ["-C", "target-feature=+neon"]  # Ensure NEON SIMD for fast_image_resize and BLAKE3
```

Build command for Batocera deployment:
```
cargo build --release --target aarch64-unknown-linux-musl
```

musl produces a fully static binary with no dynamic library dependencies. Combined with `rusqlite/bundled` (statically links SQLite) and ureq's rustls default (no OpenSSL dependency), the output is a single self-contained binary.

### 5.6 Binary Size Budget

Estimated release binary size with `lto = "fat"`, `codegen-units = 1`, `strip = "symbols"`, `panic = "abort"`:

| Component | Approximate Size |
|-----------|-----------------|
| SQLite (bundled) | ~800 KB |
| rustls + webpki-roots | ~1.5 MB |
| libwebp (via webp crate) | ~400 KB |
| image codecs (JPEG, PNG, WebP decode) | ~600 KB |
| fast_image_resize | ~200 KB |
| axum + hyper + tokio | ~2 MB |
| maud (zero runtime cost -- compiled to string writes) | ~0 KB |
| Application logic | ~3-5 MB |
| **Estimated total** | **~8-12 MB** |

This is well within acceptable limits for a Batocera appliance. The single binary replaces what would otherwise be a Node.js runtime (50+ MB), a Python interpreter (30+ MB), or a .NET runtime (80+ MB).

---

## 6. HTMX Distribution

HTMX and its extensions are loaded as static JS files served by `tower-http::services::ServeDir`. They are vendored into the repository (not fetched from a CDN at runtime) to ensure the appliance works offline.

| File | Source | Size (gzipped) |
|------|--------|----------------|
| `htmx.min.js` | https://unpkg.com/htmx.org@4.x | ~14 KB |
| `sse.js` | htmx SSE extension | ~3 KB |
| `gamepad-nav.js` | Custom (saalis) | ~2 KB |
| **Total** | | **~19 KB** |

Idiomorph morph swap styles are built into HTMX 4.0+ (`innerMorph`, `outerMorph`) and do not require a separate extension script.

---

## 7. SQLite Pragma Configuration

### Write Connection (scheduler thread)

```sql
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = -8000;         -- 8 MB page cache
PRAGMA foreign_keys = ON;
PRAGMA temp_store = MEMORY;
PRAGMA page_size = 4096;           -- match ext4 block size
PRAGMA wal_autocheckpoint = 0;     -- manual checkpoints only
```

### Read Connection (demand-load)

```sql
PRAGMA journal_mode = WAL;         -- inherits from database
PRAGMA query_only = 1;             -- safety: prevent accidental writes
PRAGMA cache_size = -4000;         -- 4 MB page cache
```

### Write-Only Mode (entire library fits in memory)

When the hot store holds the complete library, the write connection switches to maximum write throughput:

```sql
PRAGMA cache_size = -64;           -- minimal 64 KB (no reads from SQLite)
PRAGMA locking_mode = EXCLUSIVE;   -- hold lock forever (no readers)
```

Checkpoints use `SQLITE_CHECKPOINT_PASSIVE` during normal operation (does not block readers) and `SQLITE_CHECKPOINT_TRUNCATE` on shutdown (cleans WAL file).

---

## 8. Summary: Why This Stack

The implementation stack is unified by three principles:

**Sync core, async shell.** tokio exists only in `saalis-web`. Every other crate is synchronous. This eliminates function-colouring friction, makes the core trivially testable, and keeps the plugin ABI simple. The boundary is explicit: `spawn_blocking` for handler-to-core calls, `flume` / `broadcast` channels for core-to-handler events.

**Single binary, zero configuration.** `rusqlite/bundled` embeds SQLite. ureq's rustls embeds TLS roots. maud compiles templates into the binary. HTMX is vendored. musl linking produces a static executable. The result is one file that runs on any Linux ARM or x86 system without installing dependencies.

**Right-sized tools.** rusqlite instead of SeaORM (the cold store has 15 prepared statements, not a complex relational schema). ureq instead of reqwest (genuinely sync, not a hidden async facade). SIEVE instead of LRU (better miss ratios with simpler code). FTS5 instead of Tantivy (same SQLite file, zero operational overhead). maud instead of Tera (compile-time, 10x faster, type-checked). Every choice optimises for the actual workload: a single-user appliance with 10k-100k entities, running on ARM hardware with 2-8 GB RAM.

---

## Sources

This document synthesises findings from:

- [Persistence Strategy Synthesis](2026-03-14-synthesis.persistence-strategy.md)
- [Connector Architecture Synthesis](2026-03-14-synthesis.connector-architecture.md)
- [Content Acquisition Pipeline Synthesis](2026-03-14-synthesis.content-acquisition-pipeline.md)
- [Appliance-First Design Synthesis](2026-03-14-synthesis.appliance-first-design.md)
- [SeaORM vs rusqlite Deep Dive](2026-03-15-deepdive.seaorm-vs-rusqlite.md)
- [ureq HTTP Client Deep Dive](2026-03-15-deepdive.ureq-http-client.md)
- [Maud Templates Deep Dive](2026-03-15-deepdive.maud-templates.md)
- [HTMX + SSE Patterns Deep Dive](2026-03-15-deepdive.htmx-sse-patterns.md)
- [inventory + dlopen Plugins Deep Dive](2026-03-15-deepdive.inventory-dlopen-plugins.md)
- [SIEVE Eviction Deep Dive](2026-03-15-deepdive.sieve-eviction.md)
- [Full-Text Search Deep Dive](2026-03-15-deepdive.full-text-search.md)
- [Image Handling Deep Dive](2026-03-15-deepdive.image-handling.md)
- [Async-Sync Bridging Deep Dive](2026-03-15-deepdive.async-sync-bridging.md)
- [min_specialization Deep Dive](2026-03-15-deepdive.min-specialization.md)
- [Proc Macro Patterns Deep Dive](2026-03-15-deepdive.proc-macro-patterns.md)
- [Final Architecture Summary](../plans/2026-03-14-final-architecture-summary.md)
