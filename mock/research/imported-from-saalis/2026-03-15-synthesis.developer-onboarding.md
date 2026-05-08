# Developer Onboarding Guide: saalis

**Date:** 2026-03-15
**Status:** Definitive synthesis
**Audience:** Rust developers joining the saalis project for the first time
**Inputs:** Final architecture summary, implementation stack synthesis, all research and deep-dive documents

---

## 1. What is saalis?

Saalis (Finnish: *catch, haul, bounty*) is a content library manager that catalogues, enriches, downloads, organises, and serves media collections through a web UI. The primary target is retro game libraries on Batocera-based appliances, but the architecture is content-type agnostic: games today, movies, books, and music later. Saalis ships as a single statically-linked Rust binary with an embedded SQLite database, serving an HTMX-driven web interface. There is no runtime to install, no configuration file to write, and no external service to depend on. You drop one file onto an ARM single-board computer, it starts, it scans your ROM folders, it fetches metadata and cover art from IGDB, ScreenScraper, Steam, and other sources, and it presents a gallery that you browse from your TV with a gamepad. The domain is `saal.is`, the repository is `orgrinrt/saalis` on GitHub.

---

## 2. Architecture Overview

### Sync core, async shell

The single most important architectural decision in saalis is the **sync core / async shell** split. All business logic -- scheduling, persistence, plugin dispatch, query building, settings resolution, image processing, filesystem operations -- runs synchronously on plain OS threads. The only async code in the entire codebase lives in the `saalis-web` crate, which uses axum (backed by tokio) to serve HTTP requests and Server-Sent Events. The boundary is explicit:

- **Core-to-web:** axum handlers call sync core functions via `tokio::task::spawn_blocking`.
- **Web-to-core:** the sync scheduler pushes events to the async SSE layer via `flume` channels (`send()` is blocking, `recv_async()` is async -- same channel object, no runtime mismatch) and `tokio::sync::broadcast` for fan-out.

This eliminates function colouring friction, makes the core trivially testable (plain `#[test]`, no executor), and keeps the plugin ABI simple (no `async fn` across FFI).

### The 5 crates

The workspace contains five crates with a strict dependency hierarchy:

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

- **saalis-primitives** -- Zero-cost type definitions, marker traits, `repr(C)` enums, newtypes. No business logic, no I/O. Every other crate depends on it.
- **saalis-derive** -- All procedural macros (`#[register]`, `#[derive(Settings)]`). Single crate to minimise pipeline-blocking compilation.
- **saalis-sdk** -- The public API surface for connector and plugin authors. Re-exports primitives, adds context types, query/write builders, HTTP client (ureq), and rate limiting (governor).
- **saalis-core** -- The host binary's business logic. Scheduler, persistence (rusqlite), filesystem operations, image pipeline, search, download engine. Sync-only -- no tokio dependency.
- **saalis-web** -- The async shell. axum server, maud templates, HTMX integration, SSE push, static asset serving. The only crate that depends on tokio.

### hilavitkutin integration

The scheduler and data pipeline are being aligned with **hilavitkutin**, a shared `no_std + alloc` pipeline execution engine (from the polka-dots project) that provides DAG scheduling, morsel-driven batching, and work-stealing. saalis wraps hilavitkutin at the scheduler level for data pipeline operations while keeping its own event-driven orchestration layer for I/O-bound connector work. Numeric types are being migrated to **arvo**, a shared `no_std` numeric primitives crate that bans raw primitives in data-facing positions.

### Hot store / cold store

Data lives in two tiers:

- **Hot store** -- An in-memory columnar segment cache using SIEVE eviction (via `sieve-cache`). Serves all UI queries. Segments are variable-size column arrays, sharded across 16 cache partitions for concurrent access from both the scheduler and the web layer.
- **Cold store** -- A single SQLite file in WAL mode. Provides durability. One write connection (owned by the scheduler, `PRAGMA synchronous=NORMAL`, manual checkpoints) and one read connection (`PRAGMA query_only=1`) for demand loads when the hot store misses.

The sync layer bridges them: dirty segments are flushed to SQLite in epoch-batched transactions (one transaction per scheduler work unit), and SIEVE eviction is phase-locked with WAL checkpoints to keep I/O sequential on SD cards.

---

## 3. Key Design Principles

### Compile-time type safety

Invalid operations are compile errors, not runtime panics. Column access is typed: `Column<In<Rating>, As<f64>>` is a ZST that exists only at compile time. Entity-metadata relationships are trait bounds: `impl HasMetadata<Rating> for Game` means the compiler rejects code that tries to write a `Rating` to a `User`. Registration gates (`HasDescriptor<D>`) are sealed traits checked by the `#[register]` macro -- applying it to the wrong impl block is a compile error with a clear message.

### No `dyn`, no `String`, no `Vec` at boundaries

Plugin boundaries use `repr(C)` descriptors with `extern "C"` function pointers, not `dyn Trait` objects (vtable layout is unstable across compilation units). Text is `Str`, which is `Cow<'static, str>` -- never heap-allocated `String` where a `&'static str` suffices. Collections at the SDK boundary use slices or iterators, not owned `Vec`s.

### Registered traits + ZSTs

Every extensible concept -- entity types, metadata tables, connectors, source types, work units, subsystems, event kinds, settings domains, themes, archive formats, storage locations -- follows the same pattern:

1. Define a marker trait (e.g., `EntityType: Registrable`).
2. Implement it on a ZST (e.g., `struct Game;`).
3. Apply `#[register]` to the impl.
4. At startup, `inventory::iter::<Descriptor>()` discovers all registered implementations.

This universal pattern means learning one registration mechanism teaches you all of them.

### Context API pattern

Connectors never return data collections. Instead, they receive a `ConnectorContext<C>` and write results through its API:

```rust
fn enrich(&self, ctx: &ConnectorContext<Self>) -> Result<()> {
    let entities = ctx.query::<Titles>().filter::<NeedsEnrichment>()?;
    for entity in entities {
        let data = ctx.http::<Get>(api_url)?;
        ctx.write::<Rating>(entity.id, |row| {
            row.set::<Score>(data.score);
        });
    }
    Ok(())
}
```

This fire-and-forget write pattern means the host controls deduplication, batching, and persistence. Connectors focus on data acquisition, not data management.

### Fire-and-forget writes

Writes go to the hot store immediately and are flushed to SQLite at epoch boundaries. If the process crashes mid-epoch, at most one work unit's worth of mutations is lost. Both catalogue scans and enrichment are idempotent operations, so the cost of replay is low.

---

## 4. The Technology Stack

| Category | Crate | Saalis Crate | Why |
|----------|-------|-------------|-----|
| **Registration** | `inventory` 0.3 | primitives | Zero-runtime-cost plugin discovery via constructor sections |
| **Proc macros** | `syn` 2.x, `quote` 1.x, `proc-macro2` 1.x, `darling` 0.20 | derive | Parse impl blocks, generate descriptors + `inventory::submit!` calls |
| **Serialisation** | `serde` 1.x, `serde_json` 1.x | primitives | Settings, API responses, SSE payloads |
| **Concurrency** | `parking_lot` 0.12, `dashmap` 6.x, `crossbeam-channel` 0.5 | primitives/core | Faster mutexes, concurrent maps, sync channels |
| **Async-sync bridge** | `flume` 0.11 | primitives | Same channel for `send()` (sync) and `recv_async()` (async) |
| **HTTP client** | `ureq` 3.x | sdk | Genuinely sync (no hidden tokio); rustls for static binaries |
| **Rate limiting** | `governor` 0.8 | sdk | Per-connector GCRA rate limiter with burst capacity |
| **Hot store cache** | `sieve-cache` 0.2 | sdk | SIEVE eviction; 2x LRU throughput at 16 threads |
| **Cold store** | `rusqlite` 0.32+ (bundled) | core | Sync SQLite bindings; compiles SQLite into the binary |
| **Migrations** | `rusqlite_migration` 1.x | core | Schema migration via `PRAGMA user_version` |
| **Search** | FTS5 (built into SQLite) + `strsim` 0.11 | core | Full-text search + fuzzy "did you mean?" fallback |
| **Image resize** | `fast_image_resize` 5.x | core | SIMD-accelerated (NEON on ARM, AVX2 on x86); 2.4x faster than `image` |
| **Image decode/encode** | `image` 0.25, `webp` 0.3 | core | Format breadth; lossy WebP for 25-34% smaller files |
| **Placeholders** | `thumbhash` 0.1 | primitives | 25-35 byte aspect-ratio-aware image placeholders |
| **Checksums** | `blake3` 1.x (rayon), `crc32fast` 1.x | primitives/core | Content integrity + No-Intro/Redump compatibility |
| **Archives** | `zip` 2.x, `tar` 0.4, `flate2` 1.x, `zstd` 0.13, `sevenz-rust` 0.6 | core | Streaming extraction for ROM archives |
| **Filesystem** | `reflink-copy` 0.1, `jwalk` 0.8, `notify` 7.x, `atomic-write-file` 0.2, `nix` 0.29 | core | CoW cloning, parallel traversal, change watching, crash-safe writes |
| **Parallelism** | `rayon` 1.x | core | BLAKE3 parallel hashing, directory traversal, batch image processing |
| **Web framework** | `axum` 0.8, `tokio` 1.x | web | Async HTTP with first-class SSE support |
| **Templates** | `maud` 0.27 | web | Compile-time HTML; 72us big-table benchmark; zero runtime allocations |
| **HTMX integration** | `axum-htmx` 0.6 | web | Extractors and response headers for HTMX requests |
| **HTTP middleware** | `tower-http` 0.6 | web | Static file serving, response compression, CORS |
| **SSE streaming** | `tokio-stream` 0.1 | web | `BroadcastStream` adapter for SSE delivery |
| **Dynamic plugins** | `libloading` 0.8 | core | Future `dlopen`/`dlsym` runtime plugin loading |

The project requires **nightly Rust** for `min_specialization`, used in three patterns: the `SupportsEntity`/`HasMetadata` blanket bridge, the `SetAccess` default-access matrix, and the `Secret` blanket from `Credential`. The fallback (macro-generated explicit impls) is available if `min_specialization` never stabilises.

---

## 5. How Data Flows

The lifecycle of a game entity from initial discovery to browser rendering follows this path:

### Discovery (Connector HTTP call to hot store)

1. The **Curator** subsystem's `Sweep` event fires (periodically or on user request).
2. The scheduler dispatches a `CatalogueWork` unit targeting each registered `Cataloguer` -- e.g., Steam, IGDB.
3. The cataloguer calls the external API via `ctx.http::<Get>(url)`, passing through the connector runtime pipeline: batch assembly, response cache check, budget check, rate limiter (governor), circuit breaker, then ureq HTTP request.
4. Results are written to the hot store via `ctx.write::<Entities>(...)`. The host deduplicates by external ID and creates new entity records.
5. Each new entity emits an `EntityDiscovered` event.

### Enrichment (Hot store population)

6. `EntityDiscovered` triggers `EnrichWork` units. Each registered `Enricher` queries for entities needing enrichment (`ctx.query::<Titles>().filter::<NeedsEnrichment>()`) and writes metadata -- ratings, descriptions, cover art URLs, classifications -- via `ctx.write::<Rating>(id, ...)`.
7. The image pipeline (in saalis-core) downloads cover art, decodes it with the `image` crate, resizes it with `fast_image_resize` (NEON SIMD on ARM), encodes it to lossy WebP, generates a ThumbHash placeholder, and stores the results.
8. Each enriched entity emits an `EntityEnriched` event.

### Cold store sync

9. When the scheduler work unit completes, the sync layer collects all dirty hot-store segments tagged with the current epoch.
10. A single `BEGIN IMMEDIATE` transaction flushes them to SQLite. Dirty bits are cleared.
11. Periodically (after epoch completion, on idle timeout, or on shutdown), a `SQLITE_CHECKPOINT_PASSIVE` call transfers WAL contents to the main database file.

### DataView to browser

12. A user navigates to the library page. The axum handler (in saalis-web) calls `tokio::task::spawn_blocking` to invoke the sync core.
13. The core builds a `DataView` -- a typed query projection that reads from the hot store (SIEVE cache hit) or demand-loads from SQLite (cache miss).
14. The DataView result feeds a maud template (compiled at build time) that renders HTML.
15. For the initial page load, the full HTML shell is returned (layout, nav, HTMX script tags, SSE connection element, gallery content).
16. For subsequent navigation, HTMX sends partial requests (`HX-Request: true`). Handlers detect this via `axum-htmx` extractors and return only the HTML fragment for the swap target.

### Live updates via SSE

17. The scheduler sends events through a `flume` channel. The web layer receives via `recv_async()` and broadcasts through `tokio::sync::broadcast`.
18. The `/events` SSE endpoint streams named events (`library-updated`, `download-progress`, `notification`, `scheduler-status`) to the browser.
19. HTMX elements with `sse-swap="library-updated"` swap in fresh HTML when events arrive. For heavier updates, SSE events act as signals that trigger an `hx-get` for the latest fragment.

### Content-negotiated image serving

20. Image URLs use content-hash paths with `Cache-Control: immutable`. The axum image handler inspects the `Accept` header and serves AVIF, WebP, or JPEG accordingly. ThumbHash placeholders are inlined as `data-thumbhash` attributes in the initial HTML for instant visual approximation before images load.

---

## 6. How Plugins Work

### The `#[register]` pipeline

Every registered type follows the same five-step pipeline:

**Step 1: Define the trait.** A registrable trait extends `Registrable` (which is `Named + Display + Send + Sync + 'static`). When a trait is declared as registrable (via a registry-creating macro), the system generates: (a) a `repr(C)` descriptor struct with stable `extern "C"` function pointers, (b) a typed registry, and (c) a blanket impl of `HasDescriptor<D>` for all implementors.

**Step 2: Implement on a ZST.** The plugin author writes a zero-sized type and implements the trait:

```rust
struct Steam;
impl Named for Steam { const NAME: &'static str = "steam"; }
impl fmt::Display for Steam { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Steam") } }

impl Connector for Steam {
    type Settings = SteamSettings;
    fn check_prerequisites(&self) -> Result<()> { Ok(()) }
}
```

**Step 3: Apply `#[register]`.** The attribute macro inspects the impl block, determines which descriptor type to generate (via the trait name lookup table and `HasDescriptor<D>` check), and emits:

1. The original impl block (unchanged).
2. A `#[repr(C)]` descriptor struct instance with `extern "C"` function pointers bridging to the trait methods.
3. An `inventory::submit!` call that registers the descriptor in a platform constructor section (`.init_array` on Linux, `__DATA,__mod_init_func` on macOS).

```rust
#[register]
impl Connector for Steam { ... }

#[register]
impl Cataloguer<Game> for Steam { ... }
```

**Step 4: Startup discovery.** When the binary loads (or when a dynamic plugin is `dlopen`-ed), the constructor sections execute and the descriptors are registered. `inventory::iter::<ConnectorDescriptor>()` now yields the Steam descriptor.

**Step 5: Registry validation and dispatch.** At startup, each domain registry validates its descriptors: SDK version compatibility, name uniqueness, and domain-specific checks (the `WorkUnitRegistry` detects DAG cycles, the `ConnectorRegistry` checks capability coherence). After validation, the scheduler dispatches work through the descriptors' function pointers.

### ABI stability

Descriptors use `repr(C)` layout with `extern "C"` function pointers. The `SdkVersion` type (`major: u16, minor: u16`) in every descriptor enables forward compatibility: the host checks `is_compatible_with()` and rejects plugins built against an incompatible SDK version. `static_assertions::assert_eq_size!` and `assert_eq_align!` checks in the SDK crate guard against accidental layout changes.

### Dynamic loading (future)

`libloading` is included as a dependency for future runtime plugin loading. The infrastructure (`repr(C)` vtable + `extern "C"` entry point + version negotiation) is in place. At v1, all plugins are compiled into the binary via `inventory`.

---

## 7. How the UI Works

### Maud + HTMX + SSE

The UI is server-rendered HTML. There is no JavaScript framework, no virtual DOM, no build step for frontend assets. The total JavaScript budget is approximately 19 KB gzipped:

| File | Size (gzipped) |
|------|----------------|
| `htmx.min.js` (vendored, v4.x) | ~14 KB |
| `sse.js` (HTMX SSE extension) | ~3 KB |
| `gamepad-nav.js` (custom spatial navigation) | ~2 KB |

**Maud** compiles HTML templates at build time via proc macros. Templates are Rust code: type-checked, zero-allocation at render time, composable via the `Render` trait. The `maud` crate's `axum` feature provides `IntoResponse` for `Markup`, so handlers return `Markup` directly.

**HTMX** handles all dynamic behaviour. Page navigation uses `hx-get` with `hx-push-url` for browser history. Search uses `hx-trigger="input changed delay:300ms"` for debounced partial updates. Infinite scroll uses `hx-trigger="revealed"`. Multi-region updates (e.g., updating both a card and a stats counter) use out-of-band swaps (`hx-swap-oob`). Idiomorph (built into HTMX 4.0+) provides morph swaps that preserve focus, scroll position, and form state.

**SSE** provides real-time push. A single `/events` endpoint streams four event types: `library-updated` (new entities or metadata changes), `download-progress` (percentage, speed, ETA), `notification` (toast messages with urgency levels), and `scheduler-status` (subsystem health). The SSE connection is established declaratively:

```html
<div hx-ext="sse" sse-connect="/events">
  <div sse-swap="library-updated">...</div>
  <div sse-swap="download-progress">...</div>
</div>
```

### Gallery-first design

The default view is a cover art gallery optimised for TV-distance interaction and gamepad navigation. ThumbHash placeholders (25-35 bytes each) are inlined in the initial HTML payload so cards show an instant visual approximation while real images load. The grid preset (264px wide thumbnails) is pre-generated during enrichment; larger presets are generated on demand with on-disk caching.

### Progressive disclosure

The UI follows a five-layer progressive disclosure model:

1. **Browse** -- Gallery view with cover art. Zero configuration. Metadata auto-populated from connected sources.
2. **Filter** -- Filter chips auto-generated from registered metadata columns. Platform, genre, status chips appear based on distinct values in the data.
3. **Customise** -- Column visibility toggles, drag-to-reorder, saved named views stored as user entity metadata.
4. **Extend** -- Install extension plugins that register new metadata tables. New columns, filters, and sort options appear automatically in the UI without any view-specific code.
5. **Create** -- Advanced users define custom scoring profiles, automation rules, and view presets.

Default views are generated automatically from the registered metadata schema. The system inspects column types (`As<Text>`, `As<Integer>`, `As<ImageUrl>`) and constructs appropriate gallery cards, table columns, sort options, and filter controls. No entity type ever presents an empty page.

---

## 8. How to Add a New Connector

This walkthrough adds a hypothetical "MobyGames" connector that catalogues and enriches game entities.

### Step 1: Define the connector type

```rust
// In your connector module or plugin crate
struct MobyGames;

impl Named for MobyGames {
    const NAME: &'static str = "mobygames";
}

impl fmt::Display for MobyGames {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MobyGames")
    }
}
```

### Step 2: Define settings

```rust
#[derive(Settings)]
struct MobyGamesSettings {
    #[setting(domain = "Credentials", scope = "Global", field_kind = "Text")]
    api_key: ApiKey,

    #[setting(domain = "Infrastructure", scope = "Global", field_kind = "Integer")]
    requests_per_second: u32,
}
```

### Step 3: Implement and register `Connector`

```rust
#[register]
impl Connector for MobyGames {
    type Settings = MobyGamesSettings;

    fn check_prerequisites(&self) -> Result<()> {
        // Verify API key is configured, test connectivity
        Ok(())
    }
}
```

### Step 4: Implement and register `Cataloguer<Game>`

```rust
#[register]
impl Cataloguer<Game> for MobyGames {
    fn catalogue(&self, ctx: &ConnectorContext<Self>) -> Result<()> {
        let response = ctx.http::<Get>("https://api.mobygames.com/v1/games")?;
        for item in response.games {
            ctx.write::<Entities>(|row| {
                row.set::<ExternalId<Self>>(item.game_id.to_string());
                row.set::<Title>(item.title);
            });
        }
        Ok(())
    }
}
```

### Step 5: Implement and register `Enricher<Game>`

```rust
#[register]
impl Enricher<Game> for MobyGames {
    fn enrich(&self, ctx: &ConnectorContext<Self>) -> Result<()> {
        let entities = ctx.query::<Titles>().filter::<NeedsEnrichment>()?;
        for entity in entities {
            let ext_id = ctx.query::<ExternalId<Self>>(entity.id)?;
            let data = ctx.http::<Get>(
                format!("https://api.mobygames.com/v1/games/{}", ext_id)
            )?;
            ctx.write::<Description>(entity.id, |row| {
                row.set::<Text>(data.description);
            });
            ctx.write::<Rating>(entity.id, |row| {
                row.set::<Score>(data.moby_score);
            });
        }
        Ok(())
    }
}
```

### Step 6: Optionally implement `Sourcer<Game, S>`

If MobyGames provides download links (it does not in this example, but the pattern applies to other connectors):

```rust
#[register]
impl Sourcer<Game, DirectDownload> for MobyGames {
    fn resolve(&self, ctx: &ConnectorContext<Self>) -> Result<()> {
        let entities = ctx.query::<Sources>().filter::<NoUrl>()?;
        for entity in entities {
            let url = /* resolve download URL */;
            ctx.write::<Source>(entity.id, |row| {
                row.set::<Url>(url);
            });
        }
        Ok(())
    }
}
```

That is it. The `#[register]` macro handles descriptor generation and `inventory` submission. At startup, the scheduler discovers the connector, validates it, and weaves its work units into the DAG. The settings UI auto-generates input fields from the `#[derive(Settings)]` struct.

---

## 9. How to Add a New Metadata Table

This walkthrough adds a "HowLongToBeat" metadata table for game entities.

### Step 1: Define the table as a ZST

```rust
struct HltbMetadata;

impl Named for HltbMetadata {
    const NAME: &'static str = "hltb_metadata";
}
```

### Step 2: Implement `MetadataTable`

```rust
#[register]
impl MetadataTable for HltbMetadata {}
```

### Step 3: Declare columns as ZSTs

Each column is a type-level declaration using `Column<In<Table>, As<ValueType>>`:

```rust
struct MainStoryHours;
impl Column<In<HltbMetadata>, As<f64>> for MainStoryHours {}

struct CompletionistHours;
impl Column<In<HltbMetadata>, As<f64>> for CompletionistHours {}

struct PlayStyle;
impl Column<In<HltbMetadata>, As<Str>> for PlayStyle {}
```

### Step 4: Declare entity support

```rust
impl SupportsEntity<Game> for HltbMetadata {}
impl HasMetadata<HltbMetadata> for Game {}
```

With `min_specialization`, a blanket impl bridges these: `default impl<E, M> SupportsEntity<E> for M where E: HasMetadata<M>`. Without specialization, the `#[register]` macro generates both impls.

### Step 5: Write a migration

Create a migration op set for the new table:

```rust
impl Migration<1> for HltbMetadata {
    type Ops = (CreateTable<HltbMetadata>,);
}
```

The migration system generates the corresponding SQL (`CREATE TABLE hltb_metadata ...`) from the column type declarations.

### Step 6: Done

The presentation layer automatically discovers the new metadata table and its columns at startup. The table view gains three new columns (hidden by default). The filter builder gains new filter options. Sort options include "Sort by Main Story Hours." No UI code changes are needed.

---

## 10. How to Add a New Theme

Themes are extensions. Properties are intermediate representation (IR) values. Theme values are `const`.

### Step 1: Define the theme ZST

```rust
struct NordTheme;

impl Named for NordTheme {
    const NAME: &'static str = "nord";
}

#[register]
impl Theme for NordTheme {}
```

### Step 2: Provide property values

Each theme property is a registered ZST (`Background`, `Foreground`, `Accent`, etc.) with an associated value type. A theme provides values via `const`:

```rust
impl Provides<Background> for NordTheme {
    const VALUE: Color = Color::hex(0x2E3440);
}

impl Provides<Foreground> for NordTheme {
    const VALUE: Color = Color::hex(0xECEFF4);
}

impl Provides<Accent> for NordTheme {
    const VALUE: Color = Color::hex(0x88C0D0);
}

impl Provides<Surface> for NordTheme {
    const VALUE: Color = Color::hex(0x3B4252);
}

impl Provides<CornerRadius> for NordTheme {
    const VALUE: Length = Length::px(8);
}

impl Provides<BodyFont> for NordTheme {
    const VALUE: FontFamily = FontFamily::new("Inter");
}

impl Provides<IsDark> for NordTheme {
    const VALUE: bool = true;
}
```

### Step 3: Implement the theme translator

The theme translator converts IR property values into CSS fragments:

```rust
impl ThemeTranslator<Css> for NordTheme {
    fn translate(&self) -> Fragment<Css> {
        // Generated from Provides<P> implementations
        // Produces CSS custom properties: --bg: #2E3440; --fg: #ECEFF4; ...
    }
}
```

### Step 4: Optionally create variants

A variant is a theme that groups under a parent:

```rust
struct NordLight;

impl Named for NordLight { const NAME: &'static str = "nord-light"; }

#[register]
impl Theme for NordLight {}

impl AsVariantOf<NordTheme> for NordLight {}

impl Provides<Background> for NordLight {
    const VALUE: Color = Color::hex(0xECEFF4);
}
// Override only the properties that differ; inherit the rest
```

### Step 5: Bundle fonts (optional)

```rust
impl Provides<BundledFonts> for NordTheme {
    const VALUE: &'static [FontAsset] = &[
        FontAsset::new("Inter", include_bytes!("fonts/Inter.woff2"), FontStyle::Normal),
    ];
}
```

The theme system generates CSS from the IR values at startup. Users select themes via a `ThemePreference` metadata table on their user entity. The UI applies the generated CSS custom properties.

---

## 11. Testing

### The testing pyramid

saalis follows a 60/15/10/10/5 testing distribution:

| Layer | Proportion | Count (est.) | What |
|-------|-----------|-------------|------|
| **Unit tests** | 60% | 500-1000 | Work units, settings resolution, query builders, entity model, column operations. Plain `#[test]`, no async executor. |
| **Integration tests** | 15% | 100-200 | SQLite migrations, hot-cold flush/hydration, plugin loading, settings cascade. Uses `tempfile` for isolated databases. |
| **Snapshot tests** | 10% | 50-100 | HTML template output, SQL query shape, API responses. Uses `insta` with `.snap` files. |
| **Property-based + fuzz** | 10% | 30-50 properties, 5-10 fuzz targets | Settings cascade invariants, query builder correctness, parser robustness. Uses `proptest` and `cargo-fuzz`. |
| **E2E browser tests** | 5% | 10-15 | Critical user journeys (browse library, trigger enrichment, change settings). Uses Playwright. |

### What to test and how

**Proc macros:** `trybuild` for compile-fail tests (applying `#[register]` to invalid targets produces clear error messages). `macrotest` for expansion snapshots (verify generated descriptor layout, `inventory::submit!` call shape).

**Sync core:** Plain `#[test]` functions. Construct test contexts with stubbed handles. Call `execute()` on work units and assert on emitted events/writes.

**SQLite:** `tempfile::NamedTempFile` for isolated databases. Test migration forward-application and idempotency. Test hot-cold flush round-trips.

**Web handlers:** `tower::ServiceExt::oneshot` sends synthetic requests to the axum router without a TCP listener. Set `HX-Request: true` header for HTMX partial response testing.

**ABI stability:** `static_assertions::assert_eq_size!` and `assert_eq_align!` on all `repr(C)` descriptor types. Any layout change becomes a compile error.

### Dev dependencies

Key testing crates: `tempfile`, `trybuild`, `macrotest`, `insta` (with `json` and `redactions` features), `proptest`, `criterion` (benchmarks with CI regression tracking), `divan` (ergonomic micro-benchmarks with allocation counting), `static_assertions`, `mockall`.

### CI tiers

| Tier | Trigger | Time budget | Contents |
|------|---------|-------------|----------|
| 1 (fast) | Every push | < 5 min | `cargo check`, `clippy`, `fmt --check`, unit tests |
| 2 (standard) | Every push | < 15 min | Integration tests, proc macro UI tests, snapshot tests |
| 3 (extended) | PR merge to main | < 30 min | Property-based tests (high iteration count), benchmark regression |
| 4 (nightly) | Nightly / weekly | < 2 hours | Fuzzing (time-boxed per target), full E2E with Playwright |

---

## 12. Building and Deploying

### Local development build

```bash
# Standard debug build
cargo build

# Run the binary (serves web UI on default port)
cargo run

# Run tests
cargo test --workspace
```

The workspace uses Rust edition 2024 and requires **nightly** for `min_specialization`. Pin the toolchain in `rust-toolchain.toml`.

### Release build

```bash
cargo build --release
```

The release profile applies aggressive optimisations:

```toml
[profile.release]
opt-level = 3          # or "z" for size-optimised Batocera builds
lto = "fat"            # whole-program link-time optimisation
codegen-units = 1      # single codegen unit for maximum optimisation
strip = "symbols"      # strip debug symbols
panic = "abort"        # no unwinding overhead
```

Certain performance-critical dependencies (`libsqlite3-sys`, `blake3`, `fast_image_resize`) are built with `opt-level = 2` even in dev profile to avoid painfully slow debug builds.

### Cross-compilation for Batocera (ARM)

The primary deployment target is `aarch64-unknown-linux-musl`, producing a fully static binary with zero runtime dependencies:

```bash
# Using cargo-zigbuild (faster, no Docker needed)
cargo zigbuild --release --target aarch64-unknown-linux-musl

# Using cross-rs (Docker-based, full sysroot)
cross build --release --target aarch64-unknown-linux-musl
```

The musl target combined with `rusqlite/bundled` (statically links SQLite) and ureq's rustls (no OpenSSL dependency) produces a single self-contained binary. NEON SIMD is enabled via `rustflags = ["-C", "target-feature=+neon"]` in `.cargo/config.toml` for `fast_image_resize` and BLAKE3 performance on ARM.

Expected binary size: 8-12 MB (with all optimisations). This replaces what would otherwise require a Node.js runtime (50+ MB), a Python interpreter (30+ MB), or a .NET runtime (80+ MB).

### Batocera package

Batocera uses pacman (Arch Linux format). The package structure:

```
saalis-0.1.0-1-aarch64/
  .PKGINFO
  usr/bin/saalis              # the static binary
  userdata/system/configs/saalis.toml   # default configuration
  etc/init.d/S99saalis        # init script for auto-start
```

Build with:

```bash
tar -czf saalis-0.1.0-1-aarch64.pkg.tar.zst --zstd .PKGINFO usr/ userdata/ etc/
```

Install on Batocera:

```bash
pacman -U /path/to/saalis-0.1.0-1-aarch64.pkg.tar.zst
```

### Release automation

`cargo-dist` handles tag-triggered GitHub Releases with matrix builds (x86_64 + aarch64), artifact naming, and SBOM generation. `cargo-release` handles version bumping, git tagging, and pushing:

```bash
cargo release patch  # bumps 0.1.0 -> 0.1.1, commits, tags, pushes
# GitHub Actions picks up the tag and runs cargo-dist
```

### Supply chain security

- `cargo-auditable` embeds the full dependency tree in the binary for post-hoc vulnerability scanning.
- `cargo-vet` maintains an audit ledger of reviewed crate versions.
- `Cargo.lock` and `rust-toolchain.toml` are committed for reproducible builds.

---

## Quick Reference: Where to Find Things

| Topic | Authoritative document |
|-------|----------------------|
| All architectural decisions | `docs/plans/2026-03-10-final-architecture-decisions.md` |
| Type catalogue (every type, trait, ZST) | `docs/plans/2026-03-14-final-architecture-summary.md` |
| Complete technology stack | `docs/research/2026-03-15-synthesis.implementation-stack.md` |
| Connector architecture | `docs/research/2026-03-14-synthesis.connector-architecture.md` |
| Persistence strategy | `docs/research/2026-03-14-synthesis.persistence-strategy.md` |
| UI / progressive disclosure | `docs/research/2026-03-14-synthesis.progressive-ux-strategy.md` |
| Testing strategies | `docs/research/2026-03-15-deepdive.testing-strategies.md` |
| Cross-compilation and CI/CD | `docs/research/2026-03-15-deepdive.cross-compilation-ci.md` |
| hilavitkutin/arvo integration | `docs/plans/2026-03-14-topic.hilavitkutin-arvo-integration.md` |
