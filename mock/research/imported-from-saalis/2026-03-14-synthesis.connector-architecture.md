# Synthesis: Connector Architecture Best Practices for Saalis

**Date:** 2026-03-14
**Sources:** `2026-03-14-research.http-batching-and-sourcing.md`, `2026-03-14-research.arr-stack-lessons.md`
**Purpose:** Synthesise findings from both research documents into concrete connector architecture recommendations. Propose approaches that neither document covers in isolation.

---

## 1. From Cardigann YAML to `#[register]` Descriptors

### What Prowlarr Does

Prowlarr's Cardigann framework is a YAML-driven DSL that lets contributors define new indexers (sources) without writing C# code. Each YAML file declares search URL patterns, authentication flows, response parsing rules (CSS selectors, regex, JSON paths), category mappings, and rate limiting parameters. An interpreter evaluates these definitions at runtime. This enables Prowlarr to support 500+ trackers with compiled code only for those requiring genuinely custom authentication. The remaining indexers are pure data -- declarative descriptions of how to talk to an API.

### Where Our `#[register]` System Already Exceeds This

Saalis's `#[register]` + `repr(C)` descriptor + `inventory` pattern is structurally more powerful than Cardigann in several ways. First, Cardigann YAML is interpreted; our descriptors are compiled. A descriptor-driven connector benefits from type checking, zero runtime parsing overhead, and guaranteed ABI stability across plugin boundaries. Second, Cardigann's interpreter is a monolithic runtime that must anticipate every possible source behaviour; our trait system lets each connector implement exactly the methods it needs, with the compiler enforcing completeness. Third, Cardigann cannot express complex logic -- multi-step authentication, token refresh, request signing, or response post-processing that requires branching. Our Rust trait implementations handle arbitrary complexity.

### What We Should Learn Anyway

Despite our structural advantage, Cardigann solves a problem we have not yet addressed: **contribution friction**. Writing a Rust connector plugin, compiling it, and distributing it is a much higher barrier than editing a YAML file. We should adopt a two-tier connector model:

**Tier 1 -- Declarative connectors.** A `SimpleConnector` descriptor that encodes the common case: a base URL, authentication method (API key header, query parameter, Bearer token, or basic auth), a request template (URL pattern with substitution slots), a response parser (JSON path expressions for extracting fields into our metadata model), rate limit parameters, and category mappings. This descriptor is pure data -- a `repr(C)` struct with `&'static str` fields and function pointers that resolve to generic implementations. A contributor defines a `const` value and registers it. No custom logic, no separate crate, no `impl` block with method bodies. The `#[register]` macro can generate the boilerplate from a struct literal.

**Tier 2 -- Full trait connectors.** The existing `impl Connector for Steam` pattern, for connectors that need multi-step authentication (Steam's OAuth, IGDB's Twitch Client Credentials flow), custom request construction (IGDB's APICalypse query language), batch assembly, or complex response transformation. These are compiled Rust plugins.

The key insight from Cardigann is not the YAML syntax -- it is the separation between **source definition as data** and **source behaviour as code**. Our Tier 1 connectors achieve the same low barrier without an interpreter, because Rust's const evaluation and the descriptor pattern let us express structured data at compile time.

### Proposed Trait Hierarchy

```
Registrable
  -> Connector          (base: name, capabilities, auth method, health check)
       -> Cataloguer    (search, lookup by ID, list)
       -> Enricher      (enrich existing entity with additional metadata)
       -> Sourcer       (find downloadable sources for an entity)
       -> Downloader<S> (manage download lifecycle for a SourceType S)
```

Each sub-trait is independently registrable. A connector like IGDB might implement `Cataloguer + Enricher` but not `Sourcer`. A connector like a ROM site might implement `Sourcer + Downloader<DirectHttp>` but not `Cataloguer`. The `#[register]` macro is applied per-trait-impl, generating separate descriptors for each role.

---

## 2. Quality Profiles Applied to Game Sourcing

### The *arr Model: Two-Tier Scoring

The *arr stack uses a two-tier quality system. The first tier is a coarse quality ranking -- an ordered list of quality levels (SDTV, 720p WEB, 1080p Blu-ray, 4K Remux, etc.) with a cutoff that stops active upgrades. The second tier is Custom Formats -- regex-based pattern matchers that assign numeric scores to release attributes. The cumulative Custom Format score breaks ties within the same quality tier and drives upgrade decisions.

This system is powerful but suffers from severe usability problems. TRaSH Guides, Recyclarr, and Profilarr all exist primarily to help users configure quality profiles correctly. The two-tier model creates a combinatorial explosion of interactions that users struggle to reason about.

### Adaptation for Game Sourcing

Games differ from media files in a fundamental way: a game's "quality" is not a property of a single axis (resolution) but a multi-dimensional space. A ROM's quality depends on region (USA, Europe, Japan), revision (Rev A, Rev B), format (verified dump vs. underdumped), and source (No-Intro verified, Redump, GoodSet, scene release). A PC game's quality depends on version (latest patch, specific speedrun version), DRM status (DRM-free, cracked, GOG, Steam), language completeness, and extras (soundtrack, manual, artwork).

We propose a **profile-based scoring system** that collapses the *arr's two tiers into a single, more intuitive model:

**Source Quality Profile** -- a named configuration attached to a library or entity type. It contains:

1. **Attribute scorers** -- each scorer is a registered trait implementation that examines one attribute of a source and returns a score. Examples: `RegionScorer` (USA=100, Europe=80, Japan=60 for an English-speaking user), `VerificationScorer` (No-Intro verified=100, GoodSet=50, unknown=0), `DrmScorer` (DRM-free=100, Steam DRM=50), `CompletenessScorer` (all languages=100, base language only=60).
2. **Weights** -- each scorer has a weight in the profile. A user who cares deeply about region but not about extras adjusts weights accordingly.
3. **Minimum threshold** -- a source below this cumulative weighted score is rejected outright.
4. **Upgrade-until threshold** -- once a source meets this score, stop actively searching for upgrades.
5. **Hard rejects** -- specific attribute values that disqualify a source regardless of score (e.g., "never accept underdumped ROMs").

The key UX improvement over the *arr model: **weights and scorers are independently comprehensible**. Each scorer answers one question ("how good is this region?"), and the weight answers "how much do I care about that question?". Users do not need TRaSH Guides to understand this.

Attribute scorers are registered traits. Third-party plugins can add new scorers for attributes the core does not know about (e.g., a retro gaming community plugin that scores based on controller compatibility metadata). The `#[register]` pattern means new scoring dimensions are automatically discovered and available in the UI.

---

## 3. Batch-Friendly Connector Trait Design

### IGDB as the Model

IGDB's Multi-Query endpoint allows up to 10 separate queries per HTTP request, each targeting different resources. At 4 requests/second, this yields an effective throughput of 40 queries/second. The APICalypse query language supports field selection, filtering, sorting, and pagination within each sub-query. This is the gold standard for batch-friendly API design among game metadata sources.

### Designing the Trait for Batching

The connector trait should not require batching -- many APIs do not support it. But it should make batching natural for those that do. The key is to separate **intent** from **transport**.

**Intent layer:** The scheduler produces work units like "enrich entity X with cover art" or "look up entities matching query Q". These are individual, atomic intents.

**Batch assembly layer:** Before intents reach the HTTP client, a batch assembler collects pending intents for the same connector and combines them where the connector advertises batch capability. The connector trait expresses this via an associated type and method:

```rust
trait Cataloguer: Connector {
    /// Maximum number of intents that can be batched into one HTTP request.
    /// Return 1 if the API does not support batching.
    const BATCH_CAPACITY: usize;

    /// The window of time to wait for additional intents before dispatching
    /// a partial batch. Prevents latency inflation when traffic is low.
    const BATCH_WINDOW: Duration;

    /// Look up a single entity. The default implementation delegates to
    /// `lookup_batch` with a slice of one.
    fn lookup(&self, id: &EntityId, ctx: &ConnectorContext) -> Result<Metadata>;

    /// Look up multiple entities in a single API call. The default
    /// implementation calls `lookup` in a loop. Connectors with batch
    /// endpoints override this.
    fn lookup_batch(&self, ids: &[EntityId], ctx: &ConnectorContext) -> Result<Vec<Metadata>> {
        ids.iter().map(|id| self.lookup(id, ctx)).collect()
    }
}
```

The batch assembler in the connector runtime (not in each connector implementation) handles the collection logic: it buffers incoming intents up to `BATCH_CAPACITY` or `BATCH_WINDOW` (whichever comes first), then calls `lookup_batch`. Connectors that override `lookup_batch` with a true batch implementation (IGDB, TheGamesDB, GiantBomb, OpenLibrary) get the throughput benefit automatically. Connectors that do not override it get correct behaviour via the default loop implementation.

This design means connector authors do not need to think about batching unless their API supports it. The runtime handles the orchestration. And because `BATCH_CAPACITY` and `BATCH_WINDOW` are associated constants, they are available at compile time for static analysis and are encoded in the `repr(C)` descriptor.

### Cross-Connector Batch Coordination

When the scheduler needs metadata for an entity from multiple connectors (IGDB for core data, Steam for pricing, ScreenScraper for box art), it should dispatch all three requests concurrently rather than sequentially. The batch assembler operates per-connector, but the scheduler operates across connectors. A single "enrich entity X" work unit fans out into per-connector sub-units that execute in parallel, bounded by each connector's own rate limiter.

---

## 4. Rate Limiting Architecture

### Per-Connector Governor Instances

Each connector owns a `governor::RateLimiter` instance configured from settings. The rate limiter is the gatekeeper between the batch assembler and the HTTP client. Every outbound request passes through it; if the quota is exhausted, the calling thread blocks until a token is available.

**Configuration from Settings:** Rate limit parameters are part of the connector's settings domain:

```rust
#[register]
impl SettingsDomain for IgdbSettings {
    const NAME: &'static str = "connector.igdb";
}

struct IgdbSettings {
    /// Requests per second (default: 4, matching IGDB's documented limit)
    requests_per_second: u32,
    /// Burst capacity (default: 8, matching IGDB's max concurrent requests)
    burst_capacity: u32,
    /// Client ID for Twitch OAuth
    client_id: Secret<String>,
    /// Client secret for Twitch OAuth
    client_secret: Secret<String>,
}
```

The `RateLimiter` is constructed from `requests_per_second` and `burst_capacity` at connector initialisation. If the user changes these settings (perhaps because they have a higher-tier API key), the connector rebuilds its rate limiter without restarting.

### Adaptive Rate Limiting

Static configuration is the baseline. Adaptive adjustment based on response headers is the refinement. The connector runtime inspects every HTTP response for rate limit headers (`X-RateLimit-Remaining`, `X-RateLimit-Reset`, `Retry-After`) and adjusts the governor's effective rate downward when the server signals approaching exhaustion. This is particularly valuable for APIs like TheGamesDB that report `remaining_monthly_allowance` in every response -- the connector can automatically throttle itself as the monthly budget depletes, rather than hitting the wall and failing.

On 429 responses, the connector must respect the `Retry-After` header precisely. The circuit breaker trips after repeated 429s to avoid wasting the remaining budget.

### Global Budget Awareness

Some APIs have per-day or per-month caps (Steam: 100,000/day; ScreenScraper: 50,000/day; TheGamesDB: monthly allowance). The rate limiter handles per-second/per-minute pacing, but it does not track cumulative usage. A separate **budget counter** per connector tracks total requests in the current period. When the budget falls below a configurable reserve (e.g., 10% of daily cap), the connector shifts to a conservation mode: only Critical-priority requests are dispatched, and Normal/Background work is deferred to the next period. This prevents a background enrichment sweep from consuming the entire daily budget before a user needs an interactive search.

---

## 5. The Metadata Proxy Question

### What the *arr Stack Does

Radarr's SkyHook proxy sits between the application and upstream metadata APIs (TMDb, IMDb). It centralises API key management (users never need their own TMDb key), normalises responses across sources, caches responses server-side to reduce upstream load, and provides an abstraction layer that can supplement or correct upstream data.

### Should Saalis Have a Metadata Proxy?

The short answer is: **not at launch, but the architecture should not preclude it**.

Arguments for a proxy:
- Eliminates the need for users to obtain API keys for every service (IGDB requires a Twitch developer account; MobyGames requires a paid subscription for reasonable rate limits)
- Server-side caching benefits all users -- if 1,000 saalis instances all look up "The Legend of Zelda: Breath of the Wild" from IGDB, a proxy serves 999 of them from cache
- Response normalisation can happen once, centrally, rather than in every client
- The proxy can aggregate data from multiple sources before returning it, reducing client complexity

Arguments against:
- Saalis is designed as a self-contained single-binary appliance. A cloud proxy introduces an external dependency, a point of failure, and an ongoing hosting cost
- Privacy: routing all metadata requests through a central server reveals users' library contents to the proxy operator
- The *arr projects have dedicated infrastructure teams. Saalis is a smaller project
- IGDB and Steam are free for non-commercial use. The proxy's primary value (API key centralisation) only matters for services that require registration

The recommended path: design the connector layer so that a proxy is just another connector implementation. A `SaalisProxyConnector` could implement `Cataloguer + Enricher` and route requests through `api.saal.is` instead of directly to IGDB/Steam/etc. Users who want the convenience use the proxy connector; users who want privacy or offline capability use direct connectors. Both register the same way, implement the same traits, and produce the same metadata output. The proxy is an optimisation, not an architectural dependency.

If a proxy is eventually deployed, it should follow these principles:
- **No lock-in.** Users can always switch to direct connectors.
- **Transparent caching.** The proxy's cache TTLs match the direct connector's TTLs. No stale data advantage or disadvantage.
- **No data collection.** The proxy does not log or retain information about which users requested which entities.
- **Fallback.** If the proxy is unreachable, connectors fall back to direct API access (using the user's own keys if configured).

---

## 6. Novel Ideas

### 6.1 Predictive Prefetching

Neither research document addresses anticipatory data fetching. The scheduler currently processes work reactively -- a user adds a game, the system enriches it. But user libraries exhibit strong patterns:

- **Platform affinity.** A user with 50 SNES games is likely to add more SNES games. When the system is idle, it could prefetch the IGDB catalogue for platforms the user has shown interest in, caching the results for instant enrichment when new games are added.
- **Series completion.** A user who has Mega Man 1-4 probably wants Mega Man 5 and 6. The system could pre-enrich related games from the same franchise, genre, or developer.
- **Temporal locality.** After a bulk import (user adds 200 ROMs from a new folder), the system could predict that more imports from the same folder/platform are coming and pre-warm caches accordingly.

Implementation: a `PrefetchHeuristic` registered trait. Each heuristic examines the user's recent activity and library composition, then emits Background-priority work units to pre-populate the cache. Heuristics are cheap to run (they only examine local data) and their output is speculative (Background priority, never displacing real work). If the prediction is wrong, the only cost is a few cached API responses that expire naturally.

### 6.2 Cross-Connector Metadata Fusion

Individual metadata sources are incomplete and biased. IGDB has good structured data (genres, platforms, release dates) but no user reviews. Steam has user reviews and player counts but only for PC games. MobyGames has historical accuracy for retro titles that other sources miss. ScreenScraper has the best box art for retro games.

**Confidence scoring** combines signals from multiple sources into a unified quality metric:

- **Coverage score.** How many sources have data for this entity? An entity found in 5/7 sources has higher confidence than one found in 1/7.
- **Agreement score.** Do sources agree on core facts? If IGDB, Steam, and MobyGames all list the same release year, confidence is high. If they disagree, flag the discrepancy for manual review.
- **Recency score.** How recently was each source's data updated? IGDB data from 2025 is more trustworthy than MobyGames data from 2018 for a game that received post-launch updates.
- **Authority score.** Steam is authoritative for Steam-specific data (achievements, trading cards, pricing). ScreenScraper is authoritative for ROM hashes. IGDB is authoritative for cross-platform metadata. Weight each source's contribution by its authority for the specific attribute.

The fusion engine runs as an Enricher-phase step: after individual connectors have populated their metadata, the fusion engine examines all available data for an entity, computes confidence scores per attribute, and selects the highest-confidence value for each field. Disagreements are stored as metadata annotations for the user to review.

A composite score like `(IGDB_rating * 0.4) + (Steam_positive_ratio * 0.3) + (MobyGames_score * 0.3)` gives a blended quality signal that no single source provides. This is novel -- neither the *arr stack nor any existing game library manager performs cross-source metadata fusion with confidence scoring.

### 6.3 Community Metadata Sharing

If saalis instances could opt in to sharing anonymised metadata corrections and enrichments, the ecosystem would benefit from a network effect:

- **Correction propagation.** User A notices that IGDB lists the wrong release year for an obscure game and corrects it locally. If shared, every other saalis instance that has that game benefits from the correction without the user manually fixing it.
- **ROM hash registry.** ScreenScraper's hash-based lookup is powerful but incomplete. A community registry of hash-to-game mappings, contributed by saalis users, could supplement ScreenScraper's database.
- **Custom metadata.** Community-contributed tags, categories, and descriptions for games that upstream sources poorly serve (homebrew, fan translations, demoscene).

This is architecturally feasible because our entity + metadata model is content-type agnostic. A shared correction is just a metadata record with a `source: "community"` attribution and a confidence score derived from the number of users who corroborated it.

Privacy constraints are essential: sharing is opt-in, contributions are anonymised (no user identity attached), and the shared data is limited to metadata corrections (not library composition, not user activity, not credentials). The sharing mechanism could be a simple REST endpoint on `api.saal.is` that accepts and serves metadata patches -- far lighter infrastructure than a full metadata proxy.

---

## 7. Download Client Abstraction

### The *arr Five-Stage Pipeline

The *arr download pipeline is a mature state machine: Grab (send download to client) -> Downloading (poll for progress) -> Completed Download Handling (scan for usable files) -> Import (rename, hardlink/copy to library) -> Cleanup (remove leftovers). This pipeline is battle-tested across millions of installations and handles edge cases like seeding torrents (hardlinks to avoid duplication), stalled downloads (timeout detection), failed downloads (blocklisting), and cross-filesystem moves (copy fallback).

### Mapping to `Downloader<S>`

Our `Downloader<S>` trait is parameterised by `SourceType`, which is itself a registered trait. This means each download protocol (direct HTTP, BitTorrent, Usenet, local filesystem copy) is a distinct `SourceType` with its own `Downloader` implementation. The trait should model the *arr's state machine explicitly:

```rust
#[register]
trait Downloader<S: SourceType>: Connector {
    type Handle: DownloadHandle;

    /// Stage 1: Initiate the download. Returns a handle for tracking.
    fn grab(&self, source: &Source<S>, dest: &Path, ctx: &ConnectorContext) -> Result<Self::Handle>;

    /// Stage 2: Check download progress. Returns current state.
    fn poll(&self, handle: &Self::Handle, ctx: &ConnectorContext) -> Result<DownloadState>;

    /// Stage 3: Verify and identify completed files.
    fn verify(&self, handle: &Self::Handle, ctx: &ConnectorContext) -> Result<Vec<VerifiedFile>>;

    /// Stage 5: Clean up after import (optional; import itself is core logic, not connector logic).
    fn cleanup(&self, handle: &Self::Handle, ctx: &ConnectorContext) -> Result<()>;
}
```

Note that Stage 4 (Import -- renaming, organising, hardlinking) is **not** part of the `Downloader` trait. Import is core business logic that applies regardless of download protocol. The connector's responsibility ends at delivering verified files to a staging location; the core's Housekeeper subsystem handles organisation.

### DownloadState as a Discriminated Union

```rust
enum DownloadState {
    Queued,
    Downloading { progress: f32, speed_bps: u64, eta_seconds: Option<u64> },
    Seeding { ratio: f32, uploaded_bytes: u64 },
    Paused { reason: PauseReason },
    Completed,
    Failed { error: DownloadError, retryable: bool },
}
```

The `Seeding` state is critical for torrent support. The *arr stack's hardlink strategy applies here: when a torrent enters `Seeding` state, the Housekeeper creates a hardlink (or reflink on supported filesystems) from the download location to the library location. The original file remains available for seeding. When seeding completes or the user removes the torrent, cleanup runs via the `Downloader::cleanup` method.

### Download Client vs. Direct Download

The *arr stack always delegates downloads to an external client (qBittorrent, SABnzbd, etc.) and communicates via the client's API. Saalis should support both modes:

1. **External client mode** (like *arr). A `Downloader<BitTorrent>` implementation that talks to qBittorrent's WebUI API, Transmission's RPC, or Deluge's JSON-RPC. The external client handles the actual transfer; saalis handles the lifecycle.

2. **Integrated download mode.** For direct HTTP downloads (ROM sites, archive.org, GOG), saalis can manage the download internally without an external client. A `Downloader<DirectHttp>` implementation uses the connector's own HTTP client (with its rate limiter) to download files directly. This eliminates the need for users to set up and configure a separate download client for simple use cases.

The external client mode is strictly necessary for BitTorrent (saalis should not embed a torrent client). Integrated mode is appropriate for direct HTTP and potentially for Usenet (via an embedded NZB processor, though this is a later concern).

### Blocklist as a Cross-Cutting Concern

The *arr blocklist pattern -- permanently remembering sources that produced bad downloads -- should be a core feature, not a per-connector concern. When a download fails in a non-retryable way (corrupt file, DMCA takedown, consistently incomplete), the source is blocklisted at the entity + source level. Future sourcing queries for the same entity skip blocklisted sources. The blocklist is stored in the metadata system as an attribute of the entity-source relationship, not as a separate table. This means it participates in the same caching, export, and backup mechanisms as all other metadata.

---

## 8. Putting It All Together: The Connector Runtime

The synthesis of both research documents suggests a connector runtime architecture with these components, layered from outermost (closest to the scheduler) to innermost (closest to the network):

1. **Work unit intake.** The scheduler emits typed work units (CatalogueRequest, EnrichRequest, SourceRequest, DownloadRequest). Each work unit carries a priority (Critical, Normal, Background) and a target entity.

2. **Connector selection.** For metadata operations, the runtime selects connectors based on capability (which connectors implement the needed trait for this entity type?) and priority (IGDB before MobyGames, Steam before RAWG, based on rate limits, data quality, and current budget remaining). Multiple connectors may be selected for cross-connector enrichment.

3. **Batch assembly.** Per-connector, incoming work units are buffered up to `BATCH_CAPACITY` or `BATCH_WINDOW`. When a batch is ready, it is dispatched as a single `lookup_batch` or equivalent call.

4. **Cache check.** Before hitting the network, the runtime checks the response cache. Cache hits at this stage bypass the rate limiter, circuit breaker, and HTTP client entirely. Cache keys include connector ID, endpoint, and parameter hash. TTLs are tiered by data volatility (static metadata: 30 days; dynamic metadata: 24 hours; search results: 1 hour).

5. **Budget check.** For APIs with daily/monthly caps, the budget counter is consulted. If the remaining budget is below the conservation threshold and the work unit is not Critical priority, it is deferred.

6. **Rate limiter.** The `governor::RateLimiter` blocks until a token is available. The effective rate may be reduced below the configured maximum if adaptive rate limiting has detected approaching exhaustion from server response headers.

7. **Circuit breaker.** If the circuit is open (recent sustained failures), the request is immediately failed without reaching the network. In half-open state, a single probe request is allowed through.

8. **HTTP request.** The connector's HTTP client (one per connector, reused) sends the request with conditional headers (If-None-Match for ETags) and compressed encoding (Accept-Encoding: gzip, br). Connection pooling and HTTP/2 multiplexing (where available) handle transport efficiency.

9. **Response handling.** The response is classified (success, cache-valid, rate-limited, server error, client error), rate limit headers are extracted for adaptive adjustment, and the appropriate action is taken (cache and return, retry with backoff, trip circuit breaker, etc.).

10. **Metadata fusion.** After all connectors have returned their results for an entity, the fusion engine computes per-attribute confidence scores and selects the best value for each field. Disagreements are annotated for user review.

This pipeline is not a novel invention -- it is the natural consequence of combining the *arr stack's battle-tested patterns (state machine downloads, quality profiles, blocklists, proxy patterns) with the HTTP research's technical recommendations (batching, rate limiting, caching, compression) and saalis's architectural principles (registered traits, descriptors, sync core). Each component is independently testable, independently configurable via the settings system, and independently replaceable via the plugin system.

---

## 9. Open Questions

These questions are raised by the synthesis but not resolved by either source document:

1. **Connector health dashboard.** The *arr stack surfaces health checks in the UI (connectivity, disk space, indexer status). Should each connector expose a `health(&self) -> HealthStatus` method that the UI polls, showing users which sources are currently available, rate-limited, or circuit-broken?

2. **Offline-first enrichment.** For Batocera appliances that may not have constant internet access, should connectors support exporting and importing metadata packs? A user with internet access could generate a metadata pack for "all SNES games," distribute it, and other users could import it for instant enrichment without any API calls.

3. **Connector capability negotiation.** Should the scheduler query connectors for their current capabilities dynamically? A connector might support batch queries in general but be temporarily degraded (API maintenance, partial outage). A `capabilities(&self) -> ConnectorCapabilities` method that returns current (not static) capabilities would let the scheduler route work more intelligently.

4. **Shared rate limit pools.** If a user runs two connectors that hit the same underlying API (e.g., two different frontends for the same ROM site), should they share a rate limiter? The current model is one governor per connector, but the actual constraint is per-host or per-API-key.

5. **Metadata provenance.** When the fusion engine selects a value from one source over another, should it record which source provided which attribute? This enables users to see "release year from IGDB, box art from ScreenScraper, description from MobyGames" and manually override specific attributions.

---

## Sources

All sources cited in the two input research documents apply. Additional references specific to this synthesis:

- [governor crate documentation](https://docs.rs/governor/latest/governor/)
- [Prowlarr Cardigann definitions](https://github.com/Prowlarr/Prowlarr/tree/develop/src/NzbDrone.Core/Indexers/Definitions)
- [TRaSH Guides quality profiles](https://trash-guides.info/Sonarr/sonarr-setup-quality-profiles/)
- [Radarr SkyHook implementation](https://deepwiki.com/radarr/radarr/3.7-metadata-system)
- [IGDB Multi-Query API](https://api-docs.igdb.com/#multi-query)
