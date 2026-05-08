# Synthesis: What Saalis Can Contribute Back to the Shared Crate Ecosystem

**Date:** 2026-03-14
**Status:** Novel synthesis — ecosystem contribution roadmap
**Inputs:** `2026-03-14-research.hot-cold-storage-strategies.md`, `2026-03-14-research.sqlite-alternatives.md`, `2026-03-14-research.arr-stack-lessons.md`, `2026-03-14-topic.hilavitkutin-arvo-integration.md`

---

## Preamble

Saalis sits at the intersection of several difficult problems: hot/cold data management on constrained hardware, typed extensibility via a plugin ABI, multi-user settings with compile-time access control, and content-type-agnostic entity modelling. While solving these problems for itself, saalis is developing patterns and infrastructure that have value beyond a single content library manager. This document examines which of those patterns are genuinely reusable, which belong in existing shared crates (hilavitkutin, arvo), and which warrant new shared crates in the polka-dots ecosystem.

The guiding question throughout: **would another Rust application — loimu, a future polka-dots project, or an unrelated third-party crate — benefit from this extraction, or is the value locked inside saalis's specific domain?**

---

## 1. Hot/Cold Sync Layer: `hilavitkutin-persistence`

### The Problem

hilavitkutin is a `no_std + alloc` pipeline engine that operates on in-memory columnar data. It has no opinion about where that data comes from or where it goes when memory runs out. Every application that uses hilavitkutin for its hot data path faces the same question: what happens when the working set exceeds available memory, and how do mutations reach durable storage?

Saalis answers this with a specific combination: SIEVE eviction at column-segment granularity, epoch-batched dirty tracking, and SQLite WAL as the cold backend. But the *shape* of that answer — evict cold segments from the hot store, resurface them on demand, track and flush mutations — is not saalis-specific. It is the fundamental hot/cold bridge problem that any application pairing hilavitkutin with a persistent backend must solve.

### What a Shared Crate Would Provide

A `hilavitkutin-persistence` crate would provide:

**Eviction framework.** A trait-based eviction policy interface with SIEVE as the default implementation. The eviction layer operates on opaque segment identifiers — it does not need to know what a column segment contains, only its access metadata (visited bit, insertion order). SIEVE requires fewer than 20 lines of core logic, making it cheap to include as a default while allowing consumers to substitute ARC, LRU, or custom policies.

**Dirty tracking.** Per-segment dirty bits with epoch-based batching. When a hilavitkutin work unit mutates column data, the persistence layer marks the affected segment dirty. At epoch boundaries (configurable: time-based, work-unit completion, or manual trigger), all dirty segments are collected and presented to a backend-specific flusher. The crate provides the tracking; the consumer provides the flush implementation.

**Resurfacing protocol.** A trait for demand-loading evicted segments from cold storage back into hilavitkutin's column store. The crate provides the cache-miss detection and insertion logic; the consumer implements the actual I/O (SQLite query, Parquet read, network fetch, whatever their backend is).

**Memory budget management.** A unified memory pool inspired by DuckDB's buffer manager, where both cached segments and transient pipeline working memory draw from the same budget. When the budget is exhausted, eviction frees segments to make room for pipeline execution. This avoids the fragmentation problem of separate budgets for "cache" and "compute."

**Startup warming.** A manifest-based cache warming protocol: on clean shutdown, persist a segment manifest (IDs + access metadata); on startup, reload segments in priority order as a background task. The manifest format is simple enough to be backend-agnostic (JSON, bincode, or a custom compact format).

### Why This Is Genuinely Reusable

Any application that uses hilavitkutin on constrained hardware needs this layer. loimu, if it ever manages data that exceeds available memory, would face the identical problem. A game engine using hilavitkutin for ECS-style component storage on a console with fixed RAM would need the same eviction and dirty-tracking machinery. The hot/cold bridge is a natural companion to hilavitkutin, not a saalis-specific concern.

The key insight from the research: the eviction policy (SIEVE), the dirty tracking granularity (column segment), and the write-back strategy (epoch batching) are all orthogonal to the specific cold backend. SQLite, Parquet files, RocksDB, a network service — the persistence layer does not care. It cares about segment identity, access patterns, and mutation tracking.

### What Stays Saalis-Specific

The SQLite-specific flush implementation (writing column segments as rows via rusqlite, managing WAL checkpoints, tuning `PRAGMA cache_size`) is saalis's concern. The FTS5 integration for content search is saalis's concern. The entity-metadata schema mapping is saalis's concern. `hilavitkutin-persistence` provides the framework; saalis provides the SQLite backend.

---

## 2. NotificationPayload Registered Trait Pattern

### The Problem

Saalis defines notifications as registered trait objects: a `NotificationPayload` trait that any type can implement, with `#[register]` making all implementations discoverable at startup via `inventory`. This means plugins can define their own notification types without modifying the core notification system — a plugin that adds a new connector type can emit connector-specific notifications that the existing notification infrastructure (Discord, webhook, email) delivers without knowing the payload's concrete type.

### Reusability Assessment

The pattern itself — "typed payloads registered at compile time, dispatched dynamically at runtime" — is broadly useful. Any application with a notification or event system faces the same tension: the core wants to route notifications without knowing every possible payload type, while producers want type safety when constructing payloads.

However, the *notification delivery* aspect (formatting for Discord, constructing webhook JSON, sending emails) is deeply application-specific. The *arr stack's webhook data parity bug — where Discord notifications contained more data than generic webhooks because channel-specific formatting leaked into payload construction — demonstrates that the boundary between "typed payload" and "channel-specific presentation" is where bugs breed.

### What Could Be Extracted

A lightweight `registered-events` or `typed-dispatch` crate that provides:

- A `#[register]` macro for event/payload types with `inventory`-based discovery
- A dispatcher that routes registered payloads to handlers by type
- A trait for payload serialisation (so handlers can inspect arbitrary payloads without knowing their concrete type)

This is smaller than a full notification system. It is the *dispatch mechanism*, not the delivery infrastructure. The value is in the combination of compile-time type safety (producers construct typed payloads) with runtime extensibility (new payload types can be registered without modifying the dispatcher).

### Risk: Over-Extraction

The danger here is extracting something so thin that it provides little value over just using `inventory` directly. The registration pattern (`#[register]` + `inventory` + trait objects) is already well-understood; wrapping it in a crate adds a dependency without necessarily adding capability. This pattern may be better documented as a cookbook recipe in hilavitkutin's documentation rather than extracted as a standalone crate.

**Verdict:** Likely better as a documented pattern than a separate crate, unless at least two applications in the polka-dots ecosystem need it independently.

---

## 3. Settings Derive System with `SetAccess` Compile-Time ACL

### The Problem

Saalis's settings system uses a derive macro to generate typed settings with compile-time role-based access control. Each setting declares which roles (Admin, User, Child) can read or write it, and the derive macro generates accessors that enforce these constraints at the type level. A `Child` role cannot even call the setter for an Admin-only setting — it is a compile error, not a runtime check.

### Why This Is Interesting

Multi-user applications with role hierarchies are common. The typical approach is runtime ACL checks: "is the current user's role >= the required role for this operation?" This works but pushes access violations to runtime, where they manifest as error responses or silent failures. Compile-time ACL means that code which constructs a settings mutation for an unprivileged role simply does not compile.

The mechanism relies on Rust's type system: `SetAccess<Admin>` is a different type from `SetAccess<User>`, and the generated setter methods are only available on the appropriate access type. This is zero-cost at runtime — no role checks, no error branches, no permission denied responses from the settings layer.

### What Could Be Extracted: `typed-settings`

A `typed-settings` crate would provide:

- A derive macro that generates typed setting accessors from struct definitions
- `SetAccess<R>` phantom-typed access tokens parameterised over a role type
- A trait for role hierarchies (`Role: PartialOrd`) so that higher roles implicitly have access to lower-role settings
- Optional serde integration for serialising settings to/from configuration files
- Profile-scoped settings (per-user overrides over a base configuration)

### Genuine Reusability

Any multi-user Rust application — a home automation server, a media server, a NAS management UI — could use typed settings with role-based access. The pattern is not specific to content library management. The role hierarchy (Admin > User > Child or Admin > Operator > Viewer) varies by application, but the mechanism of "derive macro generates accessors gated by role type" is universal.

The profile-scoping aspect (user-specific overrides) is also broadly useful. Saalis's model where credentials are settings (a `Credential` trait with profile-scoped storage) demonstrates that settings and credentials can share infrastructure — a pattern that would benefit any application managing per-user API keys or service credentials.

### What Stays Saalis-Specific

The specific role enum (`Admin`, `User`, `Child` as `repr(C)` fixed variants) is saalis's domain. The credential-as-setting pattern is saalis's innovation but could be documented as a usage example rather than baked into the shared crate. The `typed-settings` crate would provide the derive machinery and access-token types; applications would define their own roles and setting structs.

**Verdict:** Strong candidate for extraction. The compile-time ACL pattern is novel enough and broadly useful enough to justify a standalone crate.

---

## 4. The `#[register]` + `inventory` + `repr(C)` Descriptor Framework

### The Problem

Saalis uses a universal pattern for extensibility: types implement a trait, apply `#[register]`, and a `repr(C)` descriptor struct is generated and collected via `inventory` at link time. At startup, all registered descriptors are available for enumeration without any centralised registration code. This enables plugins to add new entity types, connectors, notification providers, and settings without modifying the host application.

### Could This Be a Standalone Framework Crate?

This is the most ambitious extraction candidate. The pattern combines three orthogonal concerns:

1. **`#[register]` proc macro** — generates an `inventory::submit!` call alongside the trait implementation
2. **`repr(C)` descriptor** — a stable ABI struct that captures metadata about the registered type (name, version, capabilities)
3. **`inventory` collection** — link-time aggregation of all descriptors into a queryable registry

A framework crate — call it `registered` or `crate-registry` — would provide:

- The `#[register]` attribute macro
- A `Descriptor` derive macro for generating `repr(C)` descriptor structs from trait metadata
- A `Registry<T>` type for querying registered implementations by name, capability, or other descriptor fields
- ABI stability guarantees via `repr(C)` layout rules

### The Extraction Challenge

The difficulty is that the *registration target* (what you register) varies dramatically by application. Saalis registers `EntityType`, `Connector`, `NotificationPayload`, `SettingsValue`. A game engine might register `System`, `Component`, `Renderer`. A web framework might register `Middleware`, `Handler`, `Serializer`. The framework must be generic over the registered trait without imposing any semantic requirements.

This is achievable — the macro can be parameterised over the trait being registered, and the descriptor struct can be generated from any trait that provides a `NAME` and optional metadata. But the resulting crate would be quite thin: essentially a convenience wrapper around `inventory` with `repr(C)` descriptor generation. The question is whether that convenience justifies a dependency.

### The *arr Stack Lesson

The *arr stack's lack of a plugin system is their single biggest extensibility gap. Every extension is a separate process communicating over HTTP. The community constantly wants to modify internal behaviour — quality scoring, metadata enrichment, naming logic — but cannot without forking the entire application. The NzbDrone framework extraction proposal (issue #7528) acknowledged this problem but was never implemented because the extraction effort was too large *after the fact*.

This is the key lesson: **extract the extension framework early, before application-specific assumptions calcify into the registration mechanism.** If `#[register]` is extracted as a shared crate now, while saalis's usage patterns are still forming, the resulting framework will be more generic than if we wait until saalis has hardcoded assumptions about what registration means.

The NzbDrone proposal envisioned content-specific namespaces (`NzbDrone.TV`, `NzbDrone.Movie`) with a shared framework as a Git submodule. This failed because the framework was never designed for extraction — it was a monolith from day one. Saalis has the advantage of starting with extraction as a design goal.

### Practical Path

Start by extracting the `#[register]` macro and `inventory`-based collection as a thin shared crate. Leave descriptor generation application-specific for now — let saalis define its own descriptor structs using `repr(C)` conventions. If loimu or another polka-dots project needs the same pattern, the descriptor generation can be generalised at that point with two concrete use cases to guide the abstraction.

**Verdict:** Extract the macro and collection mechanism now. Defer descriptor generation generalisation until a second consumer exists.

---

## 5. The `Str` Primitive: Where Does It Belong?

### Current Design

Saalis defines `Str` as `Cow<'static, str>` — a string type that is either a `&'static str` (for compile-time-known identifiers like `Named::NAME`) or an owned string (for runtime data). `String` is banned from the trait and descriptor surface entirely. For runtime comparison and lookup, strings are interned to `u32` handles.

### arvo's Scope

arvo is defined as a numeric primitives crate: fixed-point types, float wrappers, semantic aliases. It bans raw primitives (`f64`, `u64`) in data-facing positions. `Str` is not numeric — it is a text primitive.

### Options

**Option A: `Str` stays in saalis-sdk.** arvo remains purely numeric. saalis-sdk owns `Str` and the interning system. If loimu needs the same pattern, it either depends on saalis-sdk (creating an unwanted coupling) or duplicates the type.

**Option B: A shared `polka-primitives` crate.** A new crate below arvo that provides non-numeric value types: `Str`, interned string handles, and potentially other shared primitives (timestamps, UUIDs, opaque identifiers). arvo depends on `polka-primitives` for the value-type trait hierarchy. Both saalis and loimu depend on `polka-primitives` for `Str`.

**Option C: arvo expands to "all value primitives."** arvo's scope grows beyond numerics to include text primitives. `Str` becomes `arvo::Str`. The crate name ("arvo" — Finnish for "value") already suggests this broader scope.

### Recommendation

Option B is the cleanest. arvo's identity as a numeric primitives crate is clear and useful. Muddying it with text types invites scope creep. A `polka-primitives` crate (or simply `primitives` within the polka-dots workspace) provides a home for `Str`, interned handles, and any future non-numeric shared types without overloading arvo's purpose.

The interning system (producing `u32` handles for O(1) string comparison) is particularly useful as a shared primitive. Any application that needs fast string-keyed lookup — configuration systems, entity registries, asset managers — benefits from consistent interning. Putting this in a shared primitives crate means all polka-dots projects use the same intern table format and can share interned handles across crate boundaries.

**Verdict:** Create `polka-primitives` for `Str`, interned handles, and future non-numeric value types. Keep arvo focused on numerics.

---

## 6. Novel Contribution Ideas

### 6.1 `saalis-persistence`: SIEVE Eviction + Epoch-Batched WAL Flush

This is the concrete instantiation of the `hilavitkutin-persistence` concept from Section 1, but scoped as a saalis-published crate rather than a hilavitkutin extension. The distinction matters: `hilavitkutin-persistence` would be maintained by the hilavitkutin project and provide only the framework traits. `saalis-persistence` would be maintained by saalis and provide the full implementation — SIEVE eviction, epoch batching, SQLite backend, startup warming — as a batteries-included persistence layer.

The value proposition: any Rust application that wants a hot in-memory data layer backed by SQLite can depend on `saalis-persistence` and get production-quality eviction, dirty tracking, and cache warming without building it from scratch. This is more opinionated than `hilavitkutin-persistence` (it assumes SQLite, it assumes SIEVE) but also more useful out of the box.

The SIEVE implementation itself, verified against the NSDI 2024 paper, would be the first production-quality Rust implementation of this algorithm. SIEVE's simplicity (fewer than 20 lines of core logic) means the implementation is auditable, and its parameter-free nature means consumers do not need to tune it.

### 6.2 `saalis-settings` (or `typed-settings`): Typed Settings with Role-Based Access

As discussed in Section 3. The compile-time ACL via `SetAccess<Role>` phantom types is the differentiating feature. Combined with profile-scoped overrides and serde integration, this provides a complete settings layer for multi-user Rust applications.

### 6.3 `.saalix` Extension Packaging Format

Saalis's plugin system uses `#[register]` and `repr(C)` descriptors to define extensions. A `.saalix` package would bundle a compiled plugin (`.so`/`.dylib`/`.dll`) with its descriptor manifest, version metadata, dependency declarations, and optional assets (icons, templates, default configuration).

Could this format be shared? Possibly, if other polka-dots applications adopt the same `#[register]` + `repr(C)` extension model. A shared `.polkax` format could standardise plugin packaging across the ecosystem: plugin discovery, version compatibility checks, dependency resolution, and sandboxing rules.

However, this is premature. Extension packaging formats are useful only when there is a distribution ecosystem (a registry, a package manager, community plugin authors). Saalis does not yet have plugins, let alone a plugin ecosystem. Designing a packaging format before the extension model is proven in production risks over-engineering.

**Verdict:** Document the format as a future goal. Do not extract until saalis has at least 5 community-authored plugins to validate the model.

---

## 7. The *arr Stack's NzbDrone Extraction Failure — and How to Avoid It

### What Happened

The *arr ecosystem's core framework — scheduling, database access, HTTP utilities, notification dispatch, download client abstraction — lives in `NzbDrone.Core` and `NzbDrone.Common`. Every *arr application (Sonarr, Radarr, Lidarr, Readarr) is a fork that carries this shared code as an integral part of its repository. Bug fixes in shared infrastructure must be independently cherry-picked into each fork. The workflow is "much closer to a centralized VCS like Perforce than the normal Git workflow."

A proposal to extract NzbDrone as a shared package (GitHub issue #7528) was acknowledged as valuable but never implemented. The maintainers cited the effort required to disentangle shared infrastructure from application-specific logic — a separation that was never maintained during development.

### Why It Failed

Three factors:

1. **No separation boundary from day one.** NzbDrone.Core mixes framework-level code (scheduler, HTTP client, database migration runner) with domain-specific code (TV series management, episode parsing). Without a clear boundary, extraction requires auditing every class to determine what is shared and what is specific.

2. **Implicit coupling through inheritance.** The C# codebase uses class inheritance extensively. `ModelBase` is the base class for all database entities, and it carries framework assumptions (integer ID, lazy loading patterns) that domain classes depend on. Extracting the framework means either breaking inheritance chains or carrying domain assumptions into the shared package.

3. **No second consumer during development.** Radarr, Lidarr, and Readarr were forks, not consumers of a shared library. They inherited the code rather than depending on it. Without a second consumer imposing the discipline of a stable API surface, the "shared" code was free to evolve in application-specific directions.

### How Saalis Avoids This

Saalis has structural advantages:

**Trait-based extension, not inheritance.** Rust's trait system enforces composition over inheritance. `EntityType` is a trait, not a base class. There is no `ModelBase` to carry implicit framework coupling into domain types.

**The polka-dots ecosystem provides real second consumers.** hilavitkutin and arvo are designed to be shared across saalis, loimu, and polka-dots. This is not theoretical — the integration topic document already maps concrete alignment questions between saalis's WorkUnit and hilavitkutin's WorkUnit. A real second consumer imposes real API stability discipline.

**Extraction-first design philosophy.** Saalis's crate structure (saalis-primitives, saalis-derive, saalis-sdk, saalis-core, saalis-web) already separates concerns into layers with explicit dependency directions. The primitives and SDK crates are designed to be consumed by plugins without depending on the host application. This separation — enforced by the Rust compiler's crate boundary rules — prevents the kind of cross-layer coupling that made NzbDrone extraction impossible.

### Concrete Lessons

1. **Extract shared infrastructure before it has application-specific barnacles.** The time to extract is now, while saalis's patterns are forming, not after years of domain-specific evolution.

2. **Every shared crate must have at least two consumers from the start.** A "shared" crate with one consumer is just a library that happens to live in a separate repository. The second consumer is what forces generality.

3. **Traits at boundaries, not types.** The persistence layer should depend on traits (`ColdBackend`, `EvictionPolicy`, `DirtyTracker`), not on concrete types (`SqliteBackend`, `SievePolicy`). This is what makes `hilavitkutin-persistence` genuinely shared rather than just "saalis's persistence code in a separate crate."

4. **The fork-based extension model is a dead end.** The *arr stack proves this conclusively. Any pattern that requires forking the application to add a content type, connector, or notification provider will eventually collapse under maintenance burden. Saalis's `#[register]` model avoids this entirely — but only if the registration framework itself is extracted and stable.

---

## 8. Genuinely Reusable vs. Saalis-Specific

### Genuinely Reusable (Extract)

| Pattern | Proposed Crate | Why Reusable |
|---------|---------------|--------------|
| SIEVE eviction + epoch dirty tracking | `hilavitkutin-persistence` | Any hilavitkutin consumer on constrained hardware |
| Typed settings with compile-time ACL | `typed-settings` | Any multi-user Rust application |
| `Str` + interned string handles | `polka-primitives` | Any polka-dots project needing text primitives |
| `#[register]` + `inventory` collection | `registered` (thin macro crate) | Any application with plugin extensibility |
| SIEVE algorithm implementation | Part of `hilavitkutin-persistence` | Any Rust project needing a modern cache eviction policy |
| Unified memory pool with budget | Part of `hilavitkutin-persistence` | Any application managing in-memory data under memory pressure |

### Likely Reusable (Extract When Second Consumer Exists)

| Pattern | Notes |
|---------|-------|
| `repr(C)` descriptor generation | Wait for loimu or another project to need it |
| Profile-scoped settings overrides | Useful in multi-user apps but extract only when validated |
| Startup cache warming from manifest | Generic concept but implementation details vary by backend |
| Predictive prefetch framework | Access pattern prediction is domain-dependent |

### Saalis-Specific (Do Not Extract)

| Pattern | Why Specific |
|---------|-------------|
| Entity/metadata table schema | Content library domain model |
| Connector/Cataloguer pattern | Content sourcing domain |
| Subsystem grouping (Curator/Housekeeper/Doctor) | saalis's specific operational taxonomy |
| Quality scoring and upgrade logic | Content quality domain |
| `.saalix` packaging format | No ecosystem to consume it yet |
| SQLite cold-store implementation details | Backend choice is application-specific |
| Download pipeline state machine | Content acquisition domain |
| Hardlink/filesystem management | Content organisation domain |
| Cadence timers and MIN_CADENCE floors | saalis's specific scheduling policy |

### The Grey Zone

The **NotificationPayload registered trait pattern** sits in the grey zone. The dispatch mechanism (typed payloads, registered at compile time, routed dynamically) is generic. But it is also thin enough that documenting it as a pattern in hilavitkutin's or `registered`'s documentation may provide more value than a standalone crate. A crate with 50 lines of code and a single trait is hard to justify as a separate dependency.

The **event DAG scheduler** (triggers/emits as associated types on work units) is also in the grey zone. hilavitkutin provides data-DAG scheduling (read/write column dependencies). Saalis adds event-DAG scheduling on top. An event-DAG scheduler is broadly useful (CI/CD pipelines, workflow engines, build systems all use DAGs), but saalis's specific formulation — where events are registered types and DAG edges are declared via associated types — is tightly coupled to the `#[register]` pattern. If `registered` is extracted, the event DAG scheduler could build on it. But extracting both together without a second consumer risks creating an unused framework.

---

## 9. Proposed Extraction Roadmap

### Phase 1: Foundation (Concurrent with saalis development)

1. **`polka-primitives`** — Extract `Str` (`Cow<'static, str>`), interned string handles (`InternedStr`, `InternHandle(u32)`), and the interning table. This is small, well-defined, and immediately needed by both saalis and arvo.

2. **`registered`** — Extract the `#[register]` proc macro and `inventory`-based collection mechanism. Keep it minimal: the macro, a `Registry<T>` query type, and documentation showing the `repr(C)` descriptor pattern as a recommended usage.

### Phase 2: Infrastructure (After saalis persistence layer is proven)

3. **`hilavitkutin-persistence`** — Extract the eviction framework (SIEVE default), dirty tracking, memory budget, and resurfacing protocol as trait-based abstractions. Saalis provides the SQLite backend; other consumers provide their own backends.

4. **`typed-settings`** — Extract the settings derive macro, `SetAccess<Role>` phantom types, and profile-scoping mechanism. This can happen once saalis's settings system is stable and loimu or another project confirms it would use the same pattern.

### Phase 3: Ecosystem (After community plugin adoption)

5. **`.polkax` extension format** — Standardise plugin packaging *if and only if* multiple polka-dots applications share the `#[register]` extension model and community authors need a distribution mechanism.

6. **Event DAG scheduler** — Extract *if and only if* a second consumer needs event-driven DAG scheduling distinct from hilavitkutin's data-driven DAG.

### Anti-Goals

- **Do not extract prematurely.** A shared crate with one consumer and no stability guarantee is worse than application-internal code. The cost of a premature extraction is a maintenance burden (two repositories, version coordination, breaking change management) without the benefit of reuse.
- **Do not extract domain logic.** The entity model, connector pattern, quality scoring, and content organisation are saalis's domain. Extracting them as "shared" would create a framework looking for applications rather than a library serving actual needs.
- **Do not conflate "could be shared" with "should be shared."** Many patterns *could* theoretically be reused. The filter is: does a concrete second consumer exist or is one planned within the next 6 months? If not, the pattern stays internal and is documented for future extraction.

---

## 10. Closing Observations

Saalis's most valuable ecosystem contribution is not any single crate but the *combination* of patterns it validates: `#[register]` for extensibility, `repr(C)` descriptors for ABI stability, SIEVE for cache eviction, compile-time ACL for settings, and the hot/cold bridge for hilavitkutin persistence. Each of these is a known concept; saalis's contribution is demonstrating that they compose into a coherent application architecture that avoids the *arr stack's fork-based extension failure.

The *arr stack's NzbDrone extraction failure is the clearest cautionary tale. They built a successful framework and never extracted it, leading to a fork ecosystem with O(N) maintenance burden per content type. Saalis's advantage is that it starts with extraction as a design goal, the polka-dots ecosystem provides real second consumers, and Rust's crate boundaries enforce separation in a way that C#'s project structure does not.

The concrete deliverables are modest: `polka-primitives` for shared text types, `registered` for the registration macro, `hilavitkutin-persistence` for the hot/cold bridge, and `typed-settings` for role-based settings. Four crates, each solving one well-defined problem, each with at least two potential consumers. That is the right scope — not a grand unified framework, but focused libraries that earn their existence through actual reuse.
