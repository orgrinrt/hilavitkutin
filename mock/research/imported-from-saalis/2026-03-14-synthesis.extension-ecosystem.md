# Synthesis: Extension Ecosystem Design — What Makes a Plugin Marketplace Thrive

**Date:** 2026-03-14
**Context:** Saalis uses a `.saalix` extension format with `#[register]` trait-based plugin system, `repr(C)` descriptors, `inventory` collection, and `dlopen` loading. Extensions can provide connectors, themes, metadata tables, notification types, recovery strategies, and more.
**Purpose:** Synthesise lessons from thriving extension ecosystems (Obsidian, VSCode), failed extensibility models (*arr stack), and declarative framework patterns (Prowlarr Cardigann) into a cohesive vision for the saalis extension ecosystem.

---

## 1. Why Extension Ecosystems Matter More Than Features

A content library manager lives or dies by its sources. No matter how elegant the scheduler, how flexible the metadata model, or how polished the UI, saalis is only as useful as the connectors it ships with. The core team cannot write and maintain connectors for every indexer, metadata provider, and download source the community needs. This is not a resource problem — it is a combinatorial one. There are hundreds of game metadata sources, ROM sites, and archive formats. The only sustainable model is one where the community can extend the system without forking it.

The *arr stack is the clearest demonstration of what happens without a plugin system. Sonarr, Radarr, Lidarr, and Readarr each began as full-application forks to support a single new content type. Every upstream bug fix requires cherry-picking across all forks. The workflow, as the maintainers themselves acknowledge, is "much closer to a centralized VCS like Perforce than the normal Git workflow." Community members who want to add a new indexer, modify quality scoring logic, or introduce a novel metadata source have exactly two options: submit a PR to the monolith or build an entirely separate companion application. The "Awesome *Arr" collection documents over fifty such companion tools — each running as its own process, each re-implementing authentication, each maintaining its own configuration. This is the cost of no plugin system.

Saalis's architecture already avoids the *arr content-type fork problem through the `EntityType` trait and unified entity table. But the extension ecosystem question is broader: can the community extend saalis in ways the core team never anticipated? Can a user install a connector for an obscure ROM archive, a theme that matches their Batocera setup, and a notification provider for their self-hosted Gotify instance — all without recompiling anything?

The `.saalix` extension format, `#[register]` macro, `repr(C)` descriptor pattern, and `inventory` collection provide the technical foundation. What follows is the ecosystem design that makes that foundation useful.

---

## 2. Obsidian: The Gold Standard for Community Plugins

Obsidian's community plugin ecosystem is the most successful example in the personal-software space. Over 2,000 community plugins are available, covering everything from Dataview (SQL-like queries over markdown) to Kanban boards to full calendar systems. Several factors drive this success.

**Low barrier to entry.** An Obsidian plugin is a JavaScript/TypeScript project with a `manifest.json`, a `main.js` entry point, and optionally a `styles.css`. The API surface is well-documented and the development loop is fast — edit, reload, test. Plugin authors do not need to understand Obsidian's internals; they interact through a defined API that exposes the vault, editor, settings UI, and workspace.

**Discovery is native.** The community plugin browser is built into Obsidian's settings panel. Users search by keyword, read descriptions, see download counts and last-update dates, and install with one click. There is no need to visit a website, download a file, or understand file paths. The friction between "I want this" and "I have this" is a single button press.

**Auto-update with user control.** Plugins update automatically unless the user opts out. This is critical for security patches and compatibility fixes. Users can also pin specific versions if an update breaks their workflow. The combination of automatic-by-default with manual-override respects both convenience and control.

**Trust model through review, not sandboxing.** Obsidian plugins run with full access to the vault and the Node.js runtime. There is no sandbox. Instead, trust is established through a code review process for initial listing in the community directory, plus the social signal of download counts and GitHub stars. This is a pragmatic tradeoff: sandboxing JavaScript in Electron is technically difficult and would severely limit plugin capabilities. The review process catches obvious malice; the community's collective attention catches subtle problems.

**What saalis should adopt:** native discovery UI within the web interface, one-click install, auto-update with `SdkVersion` compatibility checking, and a community review process for the extension directory. What saalis should not adopt: Obsidian's lack of sandboxing. Rust's `repr(C)` ABI boundary gives us a natural isolation point that JavaScript does not provide.

---

## 3. VSCode: Extension Marketplace at Scale

VSCode's extension marketplace serves over 50,000 extensions to millions of users. Its design choices reflect the challenges of operating at scale.

**API surface discipline.** VSCode exposes a carefully curated API that extensions interact with. The API is versioned, and extensions declare which API version they target in their `package.json`. Extensions that use proposed (unstable) APIs are flagged and cannot be published to the marketplace without special approval. This prevents the ecosystem from depending on internal implementation details that might change.

This maps directly to saalis's `SdkVersion` struct with `major: u16, minor: u16`. Extensions compiled against `SdkVersion { major: 1, minor: 3 }` should load on any host with `major == 1` and `minor >= 3`. The `repr(C)` descriptor pattern enforces this at the ABI level — descriptors have a fixed layout that the host can inspect before calling any plugin code.

**Isolation and performance budgets.** VSCode runs extensions in a separate Extension Host process. Extensions cannot block the UI thread. If an extension takes too long to activate or respond, VSCode can terminate it without crashing the editor. Language servers run in their own processes with defined communication protocols (LSP). This multi-process architecture ensures that a misbehaving extension degrades gracefully rather than taking down the application.

Saalis's sync-core/async-shell architecture provides a natural equivalent. Extensions execute within the scheduler's work unit system. A connector's `Cataloguer::execute()` runs as a scheduled work unit with defined priority and timeout. If a connector hangs, the scheduler can cancel the work unit without affecting other subsystems. The `repr(C)` boundary means that even a segfault in extension code (from unsafe FFI in a poorly written plugin) can be caught at the process boundary if extensions are loaded into isolated threads with panic handlers.

**Marketplace metadata.** VSCode extensions carry rich metadata: categories, tags, changelogs, screenshots, ratings, and verified publisher badges. The marketplace API supports search, filtering, sorting by installs or rating, and trending lists. This metadata is not decorative — it is the primary mechanism by which users discover extensions.

**What saalis should adopt:** `SdkVersion`-based compatibility enforcement, performance budgets via scheduler work unit timeouts, rich extension metadata (description, author, tags, screenshots, compatibility range), and a verified-publisher mechanism for connectors that handle credentials.

---

## 4. The *arr Ecosystem Failure: No Plugin System

The *arr stack's lack of a plugin system is not merely an inconvenience — it is the root cause of its most painful architectural problems, as documented in the research notes.

**Every enhancement requires a companion app.** Want better notifications? Run Notifiarr as a separate process. Want subtitle management? Run Bazarr. Want request management? Run Overseerr. Want configuration management? Run Buildarr. Each tool re-implements HTTP client configuration, authentication, error handling, and logging. Each runs its own web server on its own port. A typical media automation stack runs six to eight separate processes for what is conceptually a single workflow.

**Custom Scripts are a dead end.** The Connect system's Custom Script provider is the closest thing to extensibility. A script receives event data as environment variables and can execute arbitrary logic. But scripts cannot influence internal decisions — they cannot modify quality scoring, inject metadata, or add new entity types. They are output-only hooks, not extension points. The community wants to modify behaviour, not merely observe it.

**REST API extensibility has a ceiling.** The REST API is the primary integration point, and it works well for read-heavy use cases (dashboards, mobile apps, monitoring). But it cannot add new capabilities to the application. You cannot add a new indexer type, a new quality scoring dimension, or a new file naming token through the API. The API exposes what exists; it cannot introduce what does not.

**What saalis should learn:** provide both plugin-level and API-level extensibility. Plugins (`.saalix` archives with compiled Rust code) handle deep integration — new `EntityType`s, `Connector`s, `MetadataTable`s, `RecoveryStrategy`s, `Theme`s. The REST API handles lightweight integration — scripts, dashboards, external tools, mobile apps. Webhooks and event streams handle reactive integration — "notify me when X happens." All three tiers are necessary. The *arr stack has only the latter two, and the ecosystem strains against that ceiling.

---

## 5. Prowlarr's Cardigann: Declarative Connectors Without Compiled Code

Prowlarr's Cardigann framework is the most architecturally interesting pattern in the *arr ecosystem. It defines indexer behaviour through YAML files — search URL patterns, authentication flows, response parsing rules, category mappings, and rate limiting parameters. An `IndexerDefinitionUpdateService` downloads definitions from a central server and caches them locally. The Cardigann interpreter executes these definitions at runtime.

The result: 500+ tracker/indexer definitions maintained as YAML files, contributed and updated by community members who do not write C# code. Only trackers requiring truly custom authentication (multi-step login, 2FA, complex cookie handling) need native C# implementations.

**Can saalis have a declarative connector format?** Yes, and it should. Many game metadata sources and ROM archives follow predictable patterns: HTTP GET to a search endpoint, parse HTML or JSON response, extract title/URL/size/checksum fields. A declarative format — call it a Connector Definition — could describe this without compiled code:

```toml
[connector]
name = "example-archive"
type = "cataloguer"
entity_type = "Game"
base_url = "https://archive.example.org"

[auth]
type = "api_key"
header = "X-API-Key"

[search]
method = "GET"
path = "/api/search"
query = { q = "{query}", platform = "{platform}" }

[parse]
format = "json"
results_path = "$.results"
title = "$.name"
url = "$.download_url"
size = "$.file_size"
checksum = "$.md5"
```

This definition file would be bundled inside a `.saalix` archive alongside its `manifest.toml`, but the archive would contain no compiled code — just the definition and optionally an icon. The host would ship a built-in Connector Definition interpreter that handles HTTP requests, response parsing (JSON path, CSS selectors, regex extraction), and standard authentication flows (API key, basic auth, cookie-based login).

**The two-tier model:** simple sources get declarative definitions (low barrier, community-contributed, updatable without recompilation); complex sources get full Rust plugin implementations (full power, `#[register]`, custom authentication, multi-step scraping). This mirrors Prowlarr's split between Cardigann YAML definitions and native C# implementations.

**Definition updates without extension updates.** Declarative definitions could be versioned independently from the extension archive. If a site changes its search URL, the definition can be updated from a central repository without the user reinstalling the extension. This is exactly how Prowlarr's `IndexerDefinitionUpdateService` works, and it dramatically reduces the maintenance burden on both authors and users.

---

## 6. The `.saalix` Format: Technical Advantages

The `.saalix` extension format — a single archive containing `manifest.toml`, compiled shared libraries, and optional assets (themes, fonts, icons, declarative definitions) — provides several advantages over alternative approaches.

**Single-file distribution.** One `.saalix` file contains everything an extension needs. No dependency installation, no build steps, no PATH manipulation. Download, place in extensions directory (or install via UI), restart. This is the simplicity bar set by Obsidian's `.obsidian/plugins/{name}/` directory.

**`SdkVersion` compatibility check before loading.** The `manifest.toml` declares the `SdkVersion` the extension was compiled against. The host reads this before calling `dlopen` on any shared library. If the major version does not match, the extension is not loaded and the user sees a clear error: "Extension X requires SDK v2.x but this saalis version provides SDK v1.x." This prevents ABI mismatch crashes at the earliest possible point.

**`repr(C)` ABI stability.** All types crossing the plugin boundary are `repr(C)` structs with C-compatible layouts. The `#[register]` macro generates descriptor structs that the host can inspect without calling any plugin code. This means the host can enumerate an extension's capabilities (what connectors, themes, metadata tables it provides) by reading static data, before executing any plugin logic. A malicious or buggy extension's code is never called during the discovery phase.

**Asset bundling.** Themes bundle CSS, fonts, and preview images. Connectors can bundle icons and default configuration. The archive format supports this naturally — assets are just files in the archive, referenced by path in the manifest or descriptor constants. The `FontAsset` type's `path: &'static str` field references a path within the `.saalix` archive, and the `ThemeBuilder<Css>` extracts and serves these assets at runtime.

---

## 7. Extension Dependency Resolution

What happens when Extension A (a connector for a specific game archive) needs Extension B's custom metadata table to store platform-specific ROM header information?

This is a real scenario. A community connector for a retro gaming archive might want to store CRC32 checksums, ROM header data, and mapper information in structured metadata tables. These tables might be defined by a separate "Retro Gaming Metadata" extension that multiple connectors share.

**The dependency model should be simple and explicit.** The `manifest.toml` declares dependencies:

```toml
[extension]
name = "retro-archive-connector"
sdk_version = { major = 1, minor = 0 }

[dependencies]
retro-gaming-metadata = ">=1.0.0"
```

The host resolves dependencies at load time. If `retro-gaming-metadata` is not installed, the host reports a clear error: "Extension 'retro-archive-connector' requires 'retro-gaming-metadata' v1.0.0 or later. Install it from the extension directory."

**What should not cross extension boundaries:** direct Rust type sharing. Extension A should not `use` types from Extension B's crate. Instead, Extension B registers metadata tables (ZST markers implementing `MetadataTable`) that Extension A references by name. The `#[register]` + `inventory` pattern already supports this — Extension A's work units can query metadata tables by their registered name, and the host's registry resolves the concrete table at runtime.

**Dependency depth should be limited.** A dependency chain deeper than two levels (A depends on B depends on C) signals architectural problems. Extensions should depend on shared metadata definitions, not on other extensions' business logic. The registry pattern encourages this naturally: extensions interact through registered traits and the host's context services, not through direct inter-extension calls.

---

## 8. Sandboxing and Trust

The `repr(C)` ABI boundary is not a security sandbox, but it is a natural isolation point that can be strengthened.

**What the ABI boundary provides today:** the host never calls arbitrary Rust code across the boundary. It reads `repr(C)` descriptors (static data with fixed layout), then calls `extern "C"` functions with defined signatures. The calling convention is stable and inspectable. The host controls when and how plugin code executes — always within a scheduler work unit, always with defined timeout, always with error recovery.

**What sandboxing could add:** restrict what host services an extension can access. The `ConnectorContext<C>` struct is the extension's window into the host. Today, it provides `ctx.query()`, `ctx.write()`, and HTTP client access. A sandboxing layer could restrict this per-extension:

- A **theme extension** has no `ConnectorContext` at all — it only provides static data (property values, CSS fragments, font assets). No network access, no database access, no filesystem access beyond reading its own archive. This is inherently safe.
- A **connector extension** gets `ConnectorContext` with network access scoped to its declared `base_url` domains, database writes scoped to its declared metadata tables, and no filesystem access beyond the `Temp` and `PluginData` storage locations.
- A **notification provider extension** gets read access to notification payloads and network access for delivery, but no write access to the entity database.

**The trust model should be layered:**

1. **Unsigned extensions** — user installs manually, sees a warning. Full functionality but no directory listing.
2. **Community-reviewed extensions** — listed in the extension directory after code review. Carry a "reviewed" badge.
3. **Verified-publisher extensions** — author identity confirmed, automated CI testing against each SDK release, carry a "verified" badge. Required for extensions that handle credentials (`Credential` trait implementations).

This mirrors Obsidian's community review process but adds the verified-publisher tier for extensions that touch sensitive data. The *arr stack's approach of "trust everything because there is no plugin system" is not viable once third-party code can access credentials and network resources.

---

## 9. Theme Marketplace with Preview

Themes are extensions (Section 16 of the architecture decisions). A theme implements the `Theme` trait, provides values for `ThemeProperty` ZSTs (`Background`, `Foreground`, `Accent`, `Surface`, etc.) via `Provides<P>` implementations, and optionally bundles fonts via `BundledFonts`. The `ThemeBuilder<Css>` translates property values into CSS and `@font-face` rules.

**Preview before installing.** The extension directory can render theme previews without installing the theme. The preview pipeline:

1. The extension author includes a `preview.png` screenshot in the `.saalix` archive, referenced in `manifest.toml`.
2. For richer previews, the directory server can extract the theme's property values (they are static `const` values in `repr(C)` descriptors) and render a standardised preview layout with the theme's colours, fonts, and corner radii applied. This happens server-side — no untrusted code runs on the user's machine during preview.
3. After installation, a "Try theme" button applies the theme temporarily without committing it to settings. The user browses their library with the new theme, then confirms or reverts.

**Variant packs.** The architecture supports `AsVariantOf<T>` — a variant of an existing theme (e.g., a "Catppuccin Latte" variant of the "Catppuccin" theme). Variant packs are separate extensions that register `Provides<P, AsVariantOf<Catppuccin>>` for all properties without modifying the original theme extension. This enables community contributions (custom colour variants) without coordination with the original theme author.

---

## 10. Novel Ideas for Ecosystem Growth

### 10.1 Connector Template Generator

A CLI tool (or web-based wizard) that scaffolds a new connector extension in minutes:

```
saalis new connector --name "my-archive" --type cataloguer --entity-type Game --auth api_key
```

This generates a `.saalix`-ready project structure: `Cargo.toml` with `saalis-sdk` dependency, `manifest.toml` with boilerplate filled in, stub implementations of `Connector`, `Cataloguer<Game>`, and settings, and a README with development instructions. The scaffold compiles and loads immediately — it discovers zero entities but demonstrates the full lifecycle.

For declarative connectors, the generator produces a connector definition TOML file instead of Rust code, with annotated fields explaining each section.

### 10.2 Automated Compatibility Testing via CI

The saalis project publishes a GitHub Action (or equivalent) that extension authors add to their CI pipelines:

1. Build the extension against the latest SDK release.
2. Load the extension into a headless saalis instance.
3. Verify that all registered descriptors are valid (`Registry::validate()`).
4. Run the extension's declared test suite (mock HTTP responses, verify parsed output).
5. Report `SdkVersion` compatibility range: "This extension works with SDK v1.0 through v1.4."

When saalis releases a new SDK version, the CI runs against all listed extensions and flags compatibility breaks before users encounter them. Extension authors receive automated PRs or notifications when their extension needs updating. This is similar to how Homebrew's CI tests formulae against new macOS releases.

### 10.3 Community-Contributed Quality Scores

After installing an extension, users can rate it on three axes:

- **Reliability** — does the connector consistently find and download content?
- **Performance** — does it respond quickly, or does it slow down sweeps?
- **Maintenance** — is the author responsive to issues and SDK updates?

These scores are aggregated and displayed in the extension directory alongside install counts and last-update date. Extensions with persistently low reliability scores trigger a "community warning" badge. Extensions that have not been updated for two major SDK versions are marked "potentially incompatible."

Quality scores serve a dual purpose: they help users choose between competing extensions for the same source, and they incentivise authors to maintain their extensions. The *arr ecosystem relies entirely on GitHub stars and word-of-mouth for quality signals — a structured scoring system is more useful.

### 10.4 Extension Packs

An extension pack is a meta-extension — a `.saalix` archive whose manifest lists other extensions as dependencies but contains no code of its own. Installing a pack installs all its constituent extensions.

Example packs:

- **"Retro Gaming Starter"** — connectors for major ROM archives, retro-gaming metadata tables, a CRT-inspired theme, and platform detection for common emulators.
- **"Modern PC Gaming"** — Steam metadata connector, IGDB enricher, GOG connector, a modern dark theme.
- **"Movie Library"** — TMDb metadata connector, subtitle connector, media-server notification provider, a cinema-inspired gallery theme.

Packs lower the activation energy for new users. Instead of discovering and installing six extensions individually, a user installs one pack that configures a complete workflow for their use case. This is the Obsidian equivalent of "starter kits" that community members share as lists of recommended plugins — but formalised and installable.

Pack authors can update the pack manifest to add or replace extensions as the ecosystem evolves. Users who installed via a pack can choose to receive pack updates (automatically getting new extensions the pack author adds) or detach from the pack and manage extensions individually.

### 10.5 Extension Spotlight and Curation

A curated "Extension Spotlight" section in the extension directory, updated periodically, highlights:

- **New extensions** — recently published, meeting minimum quality criteria.
- **Rising extensions** — growing install counts and high ratings.
- **Staff picks** — extensions the core team considers well-built, serving as examples for other authors.

Curation creates a feedback loop: featured extensions get more installs, which generates more ratings, which provides better quality signals, which improves future curation. Without curation, extension directories become dominated by whatever ranks first alphabetically or by install count (which favours incumbents over better alternatives).

---

## 11. Revenue Model: Should Extension Authors Monetise?

This is a question worth asking explicitly, because the answer shapes the ecosystem's culture.

**The case for monetisation.** Premium connectors for commercial metadata APIs (where the author pays for API access and passes the cost to users) have a legitimate cost structure. A connector that provides high-quality, verified game metadata by maintaining a curated database has ongoing costs. Allowing authors to charge for extensions could sustain higher-quality, more reliable connectors than volunteer effort alone.

**The case against monetisation.** The *arr ecosystem thrives on free, open-source contributions. Introducing paid extensions creates a two-tier ecosystem where free extensions are seen as inferior, fragments the community between paying and non-paying users, and introduces payment processing, refund handling, and licensing enforcement complexity. Obsidian's community plugins are entirely free; VSCode's marketplace allows paid extensions but the overwhelming majority are free.

**The recommended position for saalis:** do not build payment infrastructure into the extension system. Instead, support extension authors through:

- **Donation links** in extension metadata (GitHub Sponsors, Ko-fi, Patreon). The extension directory displays these prominently.
- **Attribution** — extension authors are credited in the UI when their extension is active.
- **No restrictions on external monetisation** — if an author wants to distribute a premium extension through their own website as a `.saalix` file, they can. The system loads any valid `.saalix` archive regardless of provenance.

This avoids the overhead of running a payment platform while allowing authors who want to monetise to do so through existing channels. If the ecosystem grows to the point where a formal marketplace with payments makes sense, that decision can be revisited with data about actual demand.

---

## 12. The Extension Ecosystem as Competitive Advantage

The *arr stack has no plugin system. Playnite's extension API is C#-specific and tied to a Windows desktop application. LaunchBox's plugin system is closed-source. EmulationStation has no extension system at all. RetroArch's cores are compiled C/C++ with a complex ABI.

Saalis's extension ecosystem — with `.saalix` single-file distribution, `SdkVersion` compatibility enforcement, `repr(C)` ABI stability, declarative connector definitions for simple sources, full Rust plugin power for complex ones, theme marketplace with preview, extension packs for quick setup, and a community quality scoring system — would be a genuine differentiator.

The technical foundation is already designed: `#[register]` macro, `inventory` collection, `repr(C)` descriptors, `HasDescriptor<D>` compile-time gates, `ConnectorContext<C>` for scoped host access. The ecosystem design described in this document is the social and UX layer that makes that foundation accessible to community contributors.

The measure of success is not how many extensions the core team ships. It is how many extensions the community creates that the core team never anticipated. A connector for an archive the team has never heard of. A theme that matches a specific Linux distribution's aesthetic. A notification provider for a self-hosted service with twelve users. A metadata table for storing speedrun categories and personal best times. These are the extensions that make saalis indispensable — and they can only come from an ecosystem that is easy to enter, rewarding to participate in, and trusted by users.

---

## Summary

| Dimension | Design Decision |
|-----------|----------------|
| **Discovery** | Native extension browser in web UI; search, filter by category, sort by installs/rating |
| **Installation** | One-click install from directory; manual `.saalix` sideloading supported |
| **Updates** | Auto-update with `SdkVersion` compatibility check; version pinning available |
| **Simple sources** | Declarative connector definitions (TOML/YAML); no compiled code needed |
| **Complex sources** | Full Rust plugin via `#[register]` + `repr(C)` + `inventory` |
| **Dependencies** | Declared in `manifest.toml`; resolved at load time; depth limited to 2 |
| **Sandboxing** | `ConnectorContext` scoping; themes have no context; connectors have scoped access |
| **Trust** | Three tiers: unsigned, community-reviewed, verified-publisher |
| **Themes** | Preview before install; variant packs via `AsVariantOf<T>`; font bundling |
| **Scaffolding** | `saalis new connector` CLI generator for both Rust and declarative formats |
| **CI** | Published GitHub Action for automated compatibility testing against SDK versions |
| **Quality** | Community ratings on reliability/performance/maintenance axes |
| **Packs** | Meta-extensions that install curated sets for specific use cases |
| **Monetisation** | No built-in payments; donation links in metadata; unrestricted sideloading |
