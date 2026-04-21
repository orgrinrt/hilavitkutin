# Synthesis: Novel Library Intelligence — Content-Addressable Dedup, Metadata Fusion, and Features Nobody Does at Scale

**Date:** 2026-03-14
**Context:** Saalis content library manager — synthesising research on filesystem optimisation, HTTP sourcing, *arr stack lessons, and the universal entity model into a vision for genuinely novel features that go beyond cataloguing and downloading.

---

## Premise

Every ROM manager today solves the same problem the same way: scan a folder, match filenames to a database, download box art, display a grid. Skyscraper, EmulationStation, LaunchBox, Pegasus — they are all variations on the same theme. The *arr stack (Sonarr, Radarr, etc.) pushed the state of the art for movies and TV by adding automated sourcing, quality profiles, and upgrade logic, but even that stack has calcified around per-content-type forks and single-source metadata dependencies.

Saalis has the opportunity to leapfrog all of them. Not by being a better grid-of-box-art viewer, but by treating a content library as a living, intelligent system — one that understands what it contains at a deep structural level, learns from its own operations, and participates in a broader ecosystem of knowledge. This document explores eight categories of novel features that, taken together, would make saalis fundamentally different from anything that exists today.

---

## 1. Content-Addressable Storage: Hash-Based Deduplication

### The Insight

A ROM file is immutable. "Super Mario World (USA).sfc" is the same 512 KB file whether it lives in `/roms/snes/`, `/backups/snes/`, or on three different profiles' libraries. Yet every existing ROM manager treats each path as a separate entity, duplicating storage for every copy. On a 2 TB SD card running Batocera — the kind of constrained storage where every gigabyte matters — this waste is unacceptable.

Content-addressable storage (CAS) indexes files by their cryptographic hash rather than their path. If two files produce the same BLAKE3 hash, they are the same file. Period. The filesystem stores one copy; every reference to that content is a pointer (reflink, hardlink, or symlink) to the canonical blob.

### Architecture

The CAS layer sits between saalis's logical library (what the user sees) and the physical filesystem (where bytes live on disk):

```
Logical layer:    Profile A: /snes/Super Mario World.sfc
                  Profile B: /snes/Super Mario World.sfc
                  Profile C: /snes/smw.sfc

CAS layer:        blob/ab/ab3f7c...  (one copy, 512 KB)

Physical layer:   reflink -> blob    (btrfs/APFS)
                  hardlink -> blob   (ext4, same filesystem)
                  symlink -> blob    (fallback, cross-filesystem)
```

On btrfs (which Batocera supports as an optional userdata partition format) and APFS (macOS default), reflinks are the ideal mechanism. A reflink creates a new directory entry pointing to the same physical blocks, consuming zero additional space. The `reflink-copy` crate handles this transparently, falling back to hardlinks on ext4 and then to regular copies only when no dedup mechanism is available.

### Multi-Profile Dedup

This is where CAS becomes transformative for a household appliance. Consider a family Batocera setup with three profiles — parent, older child, younger child. All three want Mario Kart 64. Without CAS, that is three copies of an 8 MB ROM. Trivial for one game, but multiply by a library of 5,000 ROMs and the waste becomes material. With CAS, the blob store holds one copy; each profile's library directory contains a reflink. Adding a game to a new profile is instant and free.

The entity model supports this naturally: each profile's library entry is a separate entity (with its own play history, save states, settings), but all entities reference the same blob hash. The `content_hash` column in the file metadata table serves as both integrity check and dedup key.

### Integrity and Verification

Because every file is stored by hash, verification is built in. On startup or on a scheduled cycle, saalis can re-hash any file and compare against its stored hash. A mismatch means corruption — bit rot, incomplete write, or filesystem error. This is the same principle behind git's object store and IPFS's content addressing, applied to a ROM library.

BLAKE3's Merkle tree structure enables an optimisation here: partial verification. Rather than hashing the entire file, saalis can hash individual chunks and compare against stored chunk hashes. If only one chunk has changed (unlikely for ROMs, but relevant for save states or large ISOs that might be partially overwritten), the corruption can be precisely localised.

### Space Savings Estimate

For a typical retro ROM library of 10,000 files across 30 platforms:
- Without dedup (three profiles, some overlap): ~45 GB total
- With CAS dedup (shared blob store): ~18 GB total (assuming 60% overlap across profiles)
- Savings: ~27 GB — significant on a 64 GB SD card

For modern game ISOs (PS2, GameCube, Wii), where individual files are 1-4 GB, the savings from even modest overlap are measured in tens of gigabytes.

---

## 2. Cross-Connector Metadata Fusion

### The Problem Nobody Solves Well

Every game metadata source is good at something and mediocre at everything else. IGDB has structured gameplay data and platform coverage. Steam has user reviews and playtime statistics. ScreenScraper has box art, cartridge scans, and per-region media. MobyGames has historical credits and detailed platform-specific release information. TheGamesDB has alternative titles and localised descriptions.

Existing tools pick one source and live with its gaps. Skyscraper can scrape from multiple sources but uses a simple "first non-empty value wins" merge strategy. The *arr stack routes through a proxy (SkyHook) that normalises one primary source per content type. Nobody builds a confidence-weighted fusion of multiple sources into a unified metadata profile.

### Confidence-Weighted Fusion

Each metadata field from each source carries an implicit confidence level based on the source's known strengths:

| Field | IGDB | Steam | ScreenScraper | MobyGames | TheGamesDB |
|-------|------|-------|---------------|-----------|------------|
| Title | 0.9 | 0.95 (for Steam games) | 0.7 | 0.85 | 0.8 |
| Release date | 0.9 | 0.9 | 0.6 | 0.95 | 0.7 |
| Genre | 0.85 | 0.7 (user tags) | 0.5 | 0.9 | 0.6 |
| Box art | 0.7 | 0.8 | 0.95 | 0.6 | 0.8 |
| Developer | 0.9 | 0.85 | 0.5 | 0.95 | 0.7 |
| User rating | 0.8 | 0.95 | N/A | 0.7 | 0.6 |
| ROM hash match | N/A | N/A | 0.99 | N/A | N/A |

When fusing, saalis does not simply take the "best" value. It builds a **metadata profile** where each field carries its provenance chain and confidence score:

```
title: "Super Mario Bros."
  sources:
    - IGDB: "Super Mario Bros." (confidence: 0.9)
    - Steam: null
    - ScreenScraper: "Super Mario Bros" (confidence: 0.7, note: missing period)
    - MobyGames: "Super Mario Bros." (confidence: 0.85)
  resolved: "Super Mario Bros." (confidence: 0.97, consensus)
```

The confidence score for the resolved value increases when multiple independent sources agree (Bayesian consensus). When sources disagree — say, one lists the release year as 1985 and another as 1986 — the system flags the conflict rather than silently picking one. The user can resolve conflicts manually, and that resolution becomes a high-confidence override that persists through re-enrichment.

### Field-Level Merge Strategies

Different field types demand different merge strategies:

- **Scalar text** (title, developer): Consensus voting weighted by source confidence. Normalise punctuation and whitespace before comparison.
- **Dates**: Take the source with highest date-specific confidence. Flag discrepancies beyond a threshold (e.g., year differs).
- **Numeric ratings**: Normalise to a common scale (0-100), then compute weighted average. Expose per-source ratings in the UI for users who care about the distinction between IGDB critic score and Steam user review percentage.
- **Sets** (genres, tags, platforms): Union across sources with per-tag confidence. A genre tagged by three sources is more confident than one tagged by a single source.
- **Media** (images, videos): Rank by source confidence for that media type. ScreenScraper wins for box art; Steam wins for screenshots. Store all variants, not just the "winner" — users may prefer one art style over another.
- **Structured data** (credits, relationships): MobyGames is authoritative for credits; IGDB for franchise/series relationships. Merge with dedup on person/role identity.

### Conflict Resolution as a Feature

Rather than hiding disagreements, saalis can surface them as an enrichment quality signal. A game where all five sources agree on every field is "fully confident." A game where sources disagree on the release date has an enrichment quality issue that the user might want to investigate. This transforms metadata conflicts from a bug into a feature — an invitation for the user (or the community) to contribute corrections.

---

## 3. Predictive Enrichment

### Beyond Reactive Scraping

Every existing tool enriches metadata reactively: the user adds a game, the tool scrapes. This means the first time a user opens a game's detail page, they either see incomplete data or wait for a network request. Saalis can do better by enriching content *before* the user asks for it.

### Signals for Prediction

Several signals indicate what content a user is likely to add or explore next:

1. **Platform coverage**: If a user has 200 SNES games and adds a new SNES ROM, they probably care about SNES. Pre-enrich the top 100 most popular SNES games that are not yet in the library.

2. **Franchise adjacency**: User adds "The Legend of Zelda: A Link to the Past." Pre-enrich all other Zelda titles across all platforms.

3. **API trending endpoints**: IGDB exposes trending and popular game endpoints. Pre-enrich whatever is trending because it is likely to be requested by multiple saalis users.

4. **Community signals** (see section 4): If many saalis instances are enriching the same title this week, it is trending in the community. Pre-enrich it for everyone.

5. **Seasonal patterns**: Retro gaming communities spike around holidays and specific events (AGDQ/SGDQ for speedrunning). Pre-enrich games associated with these events.

### Budget-Aware Prefetch

Predictive enrichment must respect API rate limits and bandwidth constraints. The enrichment scheduler maintains a **prefetch budget** — a percentage of each connector's rate limit capacity reserved for speculative work. When no user-initiated enrichment is pending, the prefetch budget is spent on predicted content. When the user is actively browsing and triggering real enrichment requests, prefetch pauses entirely to avoid competing for quota.

The budget system interacts with the connector priority order established in the HTTP batching research. IGDB's multi-query endpoint is ideal for prefetch — 10 games per request at 4 req/s means 40 games enriched per second of prefetch budget. MobyGames at 0.2 req/s is never used for prefetch; its quota is too precious.

### Warm Cache as a Feature

The end result is that when a user adds a new game or browses to a game they have not seen before, the metadata is already there. No loading spinner, no "enriching..." placeholder. The library feels omniscient. This is a subtle but powerful UX differentiator — it transforms the perception from "tool that fetches data on demand" to "system that already knows everything."

---

## 4. Community Metadata Sharing

### The Vision

Every saalis instance independently enriches its own library, making the same API calls that thousands of other instances are making. This is wasteful for the APIs (which is why ScreenScraper has tight rate limits and MobyGames charges for higher tiers) and slow for users. What if enrichment results were shared?

### Opt-In Federated Metadata

A saalis community metadata service (hosted at, say, `meta.saal.is`) accepts enrichment results from opt-in instances and serves them to others. When instance A enriches "Chrono Trigger" from IGDB, ScreenScraper, and MobyGames, the fused result (minus any user-specific data) is uploaded to the community service. When instance B adds "Chrono Trigger," it checks the community service first. If a recent, high-confidence metadata profile exists, it is downloaded instantly — no API calls needed.

This creates a **collaborative enrichment network** where the community's collective API quota is amortised across all participants. The first person to enrich a rare title pays the API cost; everyone after them benefits for free.

### Privacy-Preserving Aggregation

The obvious concern: sharing library composition reveals what games a user owns, which may include legally or socially sensitive content. Saalis addresses this through several mechanisms:

**Contribution without revelation**: When uploading enrichment results, the instance sends the metadata payload keyed by a content hash (e.g., BLAKE3 of the ROM) and a platform identifier. It does not send the user's identity, library composition, or any indication of how many games they own. The community service receives "here is metadata for hash X on platform Y" without knowing who sent it or what else they have.

**Differential privacy for aggregation**: If the community service wants to publish aggregate statistics (e.g., "most enriched games this week"), it applies differential privacy — adding calibrated noise to counts so that the presence or absence of any single user's contribution cannot be determined from the published statistics. This is the same technique used by Apple for emoji usage statistics and Google for Chrome telemetry.

**Local-first, network-optional**: Community sharing is entirely opt-in and the system works fully offline. The community layer is a performance optimisation, not a dependency.

### Trust and Quality

Community-contributed metadata could be wrong, outdated, or malicious (imagine someone uploading incorrect ROM hashes to cause mis-identification). The trust model addresses this:

- **Source provenance**: Every community metadata entry records which API sources contributed and when. Entries backed by three independent API sources are more trusted than entries from one.
- **Consensus validation**: When multiple independent instances contribute metadata for the same content hash and they agree, confidence increases. When they disagree, the entry is flagged for review.
- **Freshness decay**: Community entries older than a configurable threshold (e.g., 90 days) are treated as stale and re-enriched from primary sources when next requested.
- **Reputation (optional)**: Instances that consistently contribute accurate metadata build implicit reputation. This is tracked by anonymised instance ID, not user identity.

---

## 5. Quality-Aware Sourcing

### Learning From Experience

The *arr stack's quality profile system is static: the user configures preferred qualities and the system follows rules. It does not learn. If a particular indexer consistently provides corrupted downloads or if a specific release group's encodes are always problematic, the user must manually configure blocklists and negative Custom Format scores.

Saalis can automate this through **sourcing intelligence** — a feedback loop where download outcomes inform future source selection.

### The Feedback Loop

Every download produces an outcome:

| Outcome | Signal |
|---------|--------|
| Download completes, hash matches expected | Source is reliable |
| Download completes, hash mismatches | Source provides wrong content (wrong region, bad dump) |
| Download fails (network error) | Source has availability issues |
| Download completes but ROM fails verification (No-Intro/TOSEC mismatch) | Source provides unverified dumps |
| User manually replaces file after download | Source quality was insufficient |
| ROM works in emulator without issues | Source is good for this platform |
| ROM crashes or has glitches in emulator | Source may provide bad dumps for this platform |

Over time, saalis accumulates a per-source quality profile: "Source X provides verified SNES ROMs 98% of the time but GBA ROMs only 72% of the time." This profile weights source selection — when multiple sources offer the same content, saalis prefers the source with the best track record for that platform and region.

### A/B Testing Download Sources

When multiple sources offer the same content with similar quality indicators, saalis can A/B test: download from source A for some titles and source B for others, then compare outcomes over time. This is particularly valuable for sources that are new or have unknown reliability — rather than committing fully to an untested source, saalis samples it and promotes it only if outcomes are good.

The A/B testing framework integrates with the scheduler's work unit system. Each download work unit records its source, and a periodic analysis job computes per-source quality metrics. Source rankings are updated asynchronously — the scheduler consults the current rankings when selecting sources, but rankings are never updated in the hot path.

### Regional and Platform Intelligence

Quality varies not just by source but by content characteristics:

- Japanese ROMs from source A might be excellent, but PAL ROMs from the same source might be bad.
- One source might specialise in CD-based platforms (PS1, Saturn, Sega CD) with proper track listings, while another handles cartridge-based platforms better.
- Some sources are authoritative for specific regions — a European source might have better PAL dumps than a US-focused source.

The quality model captures these dimensions: `quality_score(source, platform, region)` rather than just `quality_score(source)`. This enables nuanced source selection that no existing tool offers.

---

## 6. Smart Deduplication: Beyond Exact Matches

### The Naming Chaos

The retro ROM ecosystem has a naming problem. The same game exists under dozens of names across different naming conventions:

```
Super Mario Bros. (USA).nes              (No-Intro convention)
Super Mario Bros (U) [!].nes             (GoodNES convention)
Super Mario Bros. (1985)(Nintendo).nes   (TOSEC convention)
smb.nes                                  (user shorthand)
Super_Mario_Bros.nes                     (underscore variant)
```

All five are the same game. Some might even be the same ROM dump (identical bytes). Others might be the same game but different dumps (slightly different bytes due to header differences or dump quality).

### Multi-Layer Identification

Saalis implements dedup at three layers, each catching what the previous layer missed:

**Layer 1: Hash-exact dedup.** BLAKE3 hash of the file content. If two files hash identically, they are byte-for-byte duplicates. This catches copies, backups, and files with different names but identical content. This is the CAS layer from section 1.

**Layer 2: Header-normalised hash.** Many ROM formats have headers that vary between dumps but do not affect gameplay (iNES headers for NES, SMC headers for SNES). Saalis strips known headers before hashing, producing a "content hash" that identifies the game data regardless of header variations. Two ROMs that differ only in their iNES header (e.g., mapper number corrected in a later dump) hash identically at this layer.

**Layer 3: Fuzzy identification.** For files that are genuinely different dumps of the same game, saalis uses a multi-signal approach:

- **Filename parsing**: Extract game title, region, tags, and flags from filenames using format-specific grammars (No-Intro, GoodTools, TOSEC). After extraction, normalise to a canonical form and compare.
- **No-Intro/TOSEC/Redump DAT matching**: These databases catalogue known good dumps with their CRC32, MD5, and SHA-1 hashes. If two files match different entries in the same DAT group (e.g., "[!]" good dump vs. "[b1]" bad dump), they are the same game.
- **Cross-reference via metadata sources**: ScreenScraper identifies ROMs by hash. If two different ROM files both resolve to the same ScreenScraper game ID, they are the same game regardless of filename.
- **Content similarity hashing**: For ROMs where none of the above methods work (hacks, translations, unlicensed games), a locality-sensitive hash (e.g., ssdeep-style fuzzy hash) can detect structural similarity. Two ROMs that share 95% of their bytes are likely variants of the same game.

### Dedup Actions

When duplicates are detected, saalis does not silently delete anything. Instead, it presents the user with a dedup report:

- "These 3 files are byte-identical. Keep the one with the best filename (No-Intro convention) and replace the others with reflinks?"
- "These 2 files are different dumps of the same game. File A is a verified good dump ([!]); File B is an unverified dump. Keep both, or replace B with A?"
- "These files appear to be the same game based on filename analysis, but their hashes differ significantly. They may be different versions or hacks. Mark as related?"

The user decides; saalis executes. Over time, saalis learns the user's preferences ("always prefer No-Intro verified dumps", "always keep multiple regions") and can auto-resolve common cases with user confirmation.

---

## 7. Automatic Collection Curation

### From Flat Library to Structured Knowledge

A library of 10,000 ROMs organised only by platform is barely better than a folder full of files. The real value emerges when the library understands relationships between its contents.

### Franchise Detection

Using metadata from IGDB (which has explicit franchise and series relationships), saalis automatically groups games into franchises:

- **The Legend of Zelda** (23 games across 12 platforms)
- **Final Fantasy** (47 games across 15 platforms)
- **Sonic the Hedgehog** (38 games across 14 platforms)

Franchise grouping enables a "franchise view" in the UI — tap on Zelda and see every Zelda game you own, organised chronologically, with gaps highlighted ("you have A Link to the Past but not the original Legend of Zelda — want to add it?").

### Recommendation Engine

With enriched genre, theme, gameplay mechanic, and player rating data from multiple sources, saalis can recommend games the user does not yet own:

- **"Because you played" recommendations**: User has many JRPGs from the SNES era. Recommend SNES JRPGs they are missing (Earthbound, Chrono Trigger, Secret of Mana).
- **"Complete the set" recommendations**: User has 8 of 10 games in a franchise. Suggest the missing two.
- **"Critics' choice" recommendations**: Surface highly-rated games on platforms the user has that they have not added.
- **"Deep cuts" recommendations**: Using community signals (section 4), recommend games that are popular among users with similar libraries but that this user has not discovered.

The recommendation engine runs as a background work unit on the scheduler, updating suggestions weekly or when the library changes significantly.

### Auto-Tagging and Smart Collections

Beyond franchises, saalis can automatically generate collections based on enrichment data:

- **Genre collections**: "All platformers", "All RPGs", "All fighting games"
- **Era collections**: "8-bit era", "16-bit golden age", "Early 3D"
- **Theme collections**: "Games with co-op", "Games under 2 hours", "Speedrun favourites"
- **Quality collections**: "Hall of fame (90+ rating)", "Hidden gems (high rating, low popularity)", "So bad it's good (low rating, cult following)"

These collections update dynamically as new games are added and metadata is enriched. The user can pin collections they like, hide ones they do not, and create custom collections using the same tag and filter primitives.

### Relationship Graph

At the deepest level, saalis maintains a relationship graph between entities:

- Game A is a **sequel** to Game B
- Game C is a **remake** of Game D
- Game E is a **port** of Game F
- Game G is a **romhack** of Game H
- Game I **shares developer** with Game J

This graph enables navigation patterns that no ROM manager currently supports: "Show me all games by the team that made Chrono Trigger" (which leads to Xenogears, Xenoblade, and others). "Show me the complete version history of Final Fantasy IV across all platforms" (original SNES, PS1 port, GBA remake, DS remake, PSP complete collection, mobile port, pixel remaster).

---

## 8. Novel Archival Features

### Versioned ROMs

Speedrunners care deeply about ROM versions. The original "The Legend of Zelda: Ocarina of Time" v1.0 has different behaviour from v1.1 or v1.2 — specific glitches exist only in specific versions, and world records are tracked per-version. No existing ROM manager handles this.

Saalis's CAS layer naturally supports versioned ROMs: each version is a different blob with a different hash. The entity model links them as versions of the same game:

```
entity: Zelda OoT
  versions:
    - v1.0 (USA) -> blob:abc123  [speedrun: any%]
    - v1.1 (USA) -> blob:def456  [speedrun: 100%]
    - v1.2 (USA) -> blob:ghi789  [general play]
    - Rev A (JPN) -> blob:jkl012  [Japanese speedrun]
```

The user can select which version to launch by default, or saalis can auto-select based on context (speedrun profile uses v1.0, casual profile uses v1.2).

### Save State Management Across Platforms

Save states are currently platform-specific chaos. RetroArch stores them in one format, standalone emulators in another, and there is no way to manage them across a library. Saalis treats save states as first-class entities:

- **Save states are content-addressed**: Each save state is stored in the CAS blob store with its own hash. This prevents corruption from overwrites and enables unlimited save history.
- **Linked to game version**: A save state records which ROM version it was created with, which emulator, and which core. This prevents the common problem of loading a save state with an incompatible ROM version.
- **Cross-profile sharing**: On a family Batocera setup, a parent can share a save state with a child (e.g., "here's the game saved right before the final boss so you can practice").
- **Backup and sync**: Save states participate in the same backup system as ROMs. Because they are content-addressed, incremental backups only transfer new or changed saves.
- **Timeline view**: For a given game, show a timeline of all save states with timestamps, screenshots (if the emulator provides them), and notes. The user can jump to any point in their play history.

### ROM Patching as a First-Class Operation

IPS, BPS, and UPS patches are the standard mechanism for distributing fan translations, bug fixes, and ROM hacks. Currently, patching is a manual process: download the patch, find the base ROM, run a separate patching tool, hope you used the right base ROM.

Saalis integrates patching into the library workflow:

- **Patch entities**: A patch is an entity in the library, linked to its base ROM by content hash. The patch metadata includes: target ROM hash, resulting ROM hash, patch format, description, author.
- **Automatic base ROM matching**: When the user adds a patch, saalis checks if the required base ROM exists in the CAS store (by hash). If it does, the patch can be applied immediately. If not, saalis tells the user exactly which ROM version they need.
- **Non-destructive patching**: The base ROM is never modified. Patching produces a new blob in the CAS store. The user can have both the original and the patched version simultaneously, consuming minimal additional space (the patched ROM is a separate blob, but on btrfs with reflinks, only the changed blocks consume additional space at the filesystem level).
- **Patch chain management**: Some games have multiple patches (translation + bug fix + widescreen hack). Saalis models these as a directed acyclic graph of patches, each with its own prerequisites. "Apply Japanese-to-English translation, then apply the uncensored restoration patch, then apply the widescreen hack" becomes a single operation.
- **Community patch registry**: Tied to the community metadata sharing system (section 4), users can discover available patches for games in their library. "A fan translation of this Japanese-only RPG exists — want to apply it?"

### Integrity Archival

For users who care about preservation, saalis offers archival-grade integrity features:

- **No-Intro/TOSEC/Redump verification**: Compare every ROM against known-good databases. Flag unverified dumps, bad dumps, and overdumps. Show a "library health" dashboard: "4,832 of 5,000 ROMs are verified good dumps."
- **Parity data**: Generate Reed-Solomon parity files (PAR2) for the blob store. If a ROM is corrupted by bit rot, saalis can automatically repair it from parity data without re-downloading.
- **Audit log**: Every file operation (add, remove, move, patch, verify) is logged with timestamps and hashes. The audit log is itself an append-only, hash-chained structure — a blockchain in the original, non-cryptocurrency sense of the word. This provides tamper-evident proof of library provenance for collectors and archivists.

---

## Convergence: How These Features Interact

These eight feature categories are not independent — they form a reinforcing system:

1. **CAS** enables dedup (section 6), versioned ROMs (section 8), and efficient community sharing (section 4 — share blob hashes, not full files).
2. **Metadata fusion** (section 2) feeds the recommendation engine (section 7) and provides the confidence signals for predictive enrichment (section 3).
3. **Community sharing** (section 4) accelerates predictive enrichment (section 3 — community trends are predictive signals) and improves quality-aware sourcing (section 5 — aggregate source quality data from the community).
4. **Smart dedup** (section 6) keeps the CAS store clean and enables the "library health" archival view (section 8).
5. **Quality-aware sourcing** (section 5) improves the outcomes that feed back into the sourcing model, creating a virtuous cycle.
6. **Automatic curation** (section 7) makes the library navigable enough that users actually discover and use the content that the other features worked to enrich, deduplicate, and organise.

The underlying theme is that a content library is not a static collection of files with metadata — it is a **knowledge graph** with content-addressed integrity, confidence-weighted metadata, community-contributed intelligence, and self-improving quality signals. Every file operation, every enrichment request, every user interaction generates data that makes the system smarter.

No existing tool treats a content library this way. LaunchBox is a pretty frontend. The *arr stack is an acquisition pipeline. Skyscraper is a scraper. Saalis is the first tool that treats the library itself as an intelligent, self-organising, community-connected system.

---

## Implementation Priority

For saalis v1, not all of these features need to ship. But the **architectural foundations** must be laid so that they can be added incrementally:

| Feature | v1 Foundation | Later Enhancement |
|---------|---------------|-------------------|
| Content-addressable storage | BLAKE3 hashing of all files, blob store structure | Reflink dedup, multi-profile sharing |
| Metadata fusion | Multi-source enrichment with provenance tracking | Confidence weighting, conflict resolution UI |
| Predictive enrichment | Background enrichment scheduler with budget | Trending API integration, community signals |
| Community sharing | (Deferred) | Full federated metadata service |
| Quality-aware sourcing | Source outcome logging | Feedback-driven source ranking, A/B testing |
| Smart dedup | Hash-based exact dedup, filename parsing | Header-normalised hashing, fuzzy matching |
| Automatic curation | Franchise grouping from IGDB data | Recommendation engine, smart collections |
| Archival features | ROM verification against DATs | Versioned ROMs, patch management, PAR2 |

The critical insight is that the entity model, the CAS blob store, and the scheduler work unit system are the three pillars that make all of these features possible. Get those right, and every feature in this document is an incremental addition. Get them wrong, and each feature requires re-architecting the foundation.

---

## Sources

Research documents synthesised:
- [HTTP Batching and Sourcing Pipeline Optimisation](./2026-03-14-research.http-batching-and-sourcing.md) — connector rate limits, batching strategies, caching
- [The *arr Stack: Architecture Lessons for Saalis](./2026-03-14-research.arr-stack-lessons.md) — quality profiles, download pipeline, metadata handling, extensibility
- [Filesystem Access Optimisation](./2026-03-14-research.filesystem-optimisation.md) — reflinks, checksums, atomic operations, archive extraction
- [Universal Entity Model: Risks, Gotchas, and Mitigations](./2026-03-14-research.universal-entity-model-gotchas.md) — EAV trade-offs, query performance, type safety
