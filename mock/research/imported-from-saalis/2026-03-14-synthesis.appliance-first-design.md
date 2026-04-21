# Appliance-First Design: Lessons for Single-Binary Embedded Systems

**Date:** 2026-03-14
**Context:** Synthesis of filesystem optimisation, SQLite alternatives, hot/cold storage strategies, and *arr stack lessons — applied to saalis's primary deployment target: a single Rust binary running on Batocera/loisto, a Linux-based retro gaming appliance.

**Target hardware:** ARM SBC (RPi4/5, Odroid) or x86 mini-PC. 2-8 GB RAM. SD card or USB storage. Read-only squashfs root filesystem. No display server (framebuffer or Wayland compositor). Gamepad as primary input.

---

## 1. Zero-Config First Run

The appliance promise is that the user plugs in and it works. No setup wizard. No configuration file to edit. No SSH session required. The first-run experience defines whether the product is an appliance or a project.

### Boot-to-Browse Pipeline

When saalis starts for the first time, it must execute a deterministic pipeline without user input:

1. **Discover ROM directories.** Batocera uses a well-known path convention: `/userdata/roms/{system}/`. On first boot, saalis scans these paths using `jwalk` for parallel directory traversal. The system directory names map directly to platform identifiers (snes, megadrive, psx, etc.) — Batocera's naming convention is itself a form of configuration. For non-Batocera deployments, a fallback scan of common paths (/roms, /media, ~/ROMs) provides reasonable defaults.

2. **Auto-catalogue.** Every discovered file is hashed (BLAKE3, streaming to avoid memory pressure) and matched against known ROM databases by CRC32/MD5 for compatibility with No-Intro and Redump DATs. Filename heuristic matching serves as the fallback — strip region tags, revision markers, and format suffixes, then fuzzy-match against the metadata database. The cataloguing runs as a background work unit; the UI becomes available before it completes.

3. **Auto-enrich.** Once entities are catalogued, the enrichment pipeline fires: cover art, descriptions, release dates, genres. This runs at lower priority than the initial scan. The critical insight from the *arr stack research is that enrichment must be incremental and interruptible — if the user unplugs the device mid-enrichment, every entity processed so far must be durably committed.

4. **Generate default views.** The UI constructs views from whatever data is available: "All Games" sorted by name, per-platform collections, "Recently Added." Views degrade gracefully — a game without cover art shows its filename; a game without metadata shows platform and filename. The user sees content within seconds of boot, not minutes.

### What "Zero-Config" Actually Means

Zero-config does not mean zero settings. It means every setting has a sensible default that works without modification. The settings system provides progressive disclosure: the appliance works out of the box, power users can tune everything. The *arr stack's greatest UX failure — quality profiles so complex that TRaSH Guides exists as a de facto required companion — is a direct consequence of violating this principle. Saalis must ship opinionated defaults that satisfy 80% of users without any configuration.

---

## 2. Resilience to Power Loss

Appliances do not have shutdown buttons. Users unplug them. Children trip over power cables. Circuit breakers trip during thunderstorms. Every write operation must assume the power will disappear mid-operation.

### Database Durability

SQLite in WAL mode is the foundation. The research confirms SQLite with WAL is 2-60x faster than rollback journal mode on Raspberry Pi hardware. The critical pragmas for power-loss resilience:

- `PRAGMA journal_mode=WAL` — sequential append writes, concurrent readers.
- `PRAGMA synchronous=NORMAL` — fsync on WAL commit but not on every page write. This trades a theoretical (astronomically unlikely) corruption window for a substantial performance gain. The WAL file itself provides recovery.
- `PRAGMA wal_autocheckpoint=1000` — checkpoint after 1000 pages. Batocera's typical write pattern (burst during catalogue/enrich, then idle) means checkpoints naturally align with idle periods.

### File Operation Atomicity

Every file write outside SQLite uses the write-temp-fsync-rename pattern via the `atomic-write-file` crate. This applies to configuration files, hot store manifests, download state files, and any other persistent data. The research confirms that `tempfile::NamedTempFile::persist()` is insufficient — it does not fsync before rename.

### Download Recovery

Incomplete downloads are the most common power-loss casualty. The strategy:

1. **Preallocate with `fallocate`.** Before downloading a 4 GB ISO, preallocate the full size. This fails fast if storage is insufficient and reduces fragmentation.
2. **Write a state file.** A small JSON or binary file records: source URL, expected size, expected hash, bytes written, chunk boundaries. This file is atomically written after each chunk boundary.
3. **Resume on restart.** On startup, scan the download staging directory for incomplete files. Compare the state file against the partial file. Resume from the last confirmed chunk boundary. If the state file is missing or corrupt, delete the partial file and restart the download.
4. **Atomic move on completion.** Once the download is complete and verified (BLAKE3 hash matches), atomically rename from staging to the library location. The library never contains a partial file.

### Archive Extraction Recovery

Extraction of multi-file archives (ZIP, 7z) is not inherently atomic. The approach:

1. Extract to a temporary directory within the same filesystem (enabling atomic rename).
2. Track extraction progress in a state file (archive path, entries extracted, bytes written).
3. On completion, atomically rename the temporary directory to the target location.
4. On crash recovery, if a partial extraction directory exists with a state file, either resume or delete and restart — never leave partial extractions visible to the library.

---

## 3. Memory Pressure Handling

A Raspberry Pi 4 with 2 GB RAM running Batocera has perhaps 1-1.5 GB available after the OS, EmulationStation, and active emulator. Saalis might realistically get 256-512 MB. The hot store, cover art cache, and web server must coexist within this budget.

### SIEVE Eviction

The hot/cold storage research identifies SIEVE as the optimal eviction algorithm for saalis: simpler than LRU, outperforms ARC on 45% of real-world traces, requires no tuning parameters, and uses only 1 bit of metadata per entry. When the unified memory pool (DuckDB-style, single budget for cached data and transient query working memory) reaches its limit, SIEVE selects column segments for eviction. Evicted segments are written to SQLite if dirty, then freed.

### Responding to System Memory Pressure

On Linux, the kernel exposes memory pressure via cgroup v2's `memory.pressure` file (PSI — Pressure Stall Information). Saalis should monitor this:

1. **Register for pressure notifications.** Open `memory.pressure` and use `poll()` with a stall threshold (e.g., "some 100000 1000000" — at least 100ms of stall in any 1-second window). This is more responsive than checking `/proc/meminfo` on a timer.
2. **On pressure notification:** Trigger aggressive eviction in the hot store. Reduce the effective memory budget to 50% of configured. Drop cached cover art that is not currently visible. Free transient query buffers.
3. **On sustained pressure:** Enter degraded mode. Disable background enrichment. Serve directly from SQLite (bypass hot store). Release the hot store entirely if necessary. The UI continues to function — it degrades to higher latency, not failure.
4. **OOM score adjustment.** Set `oom_score_adj` to a moderate positive value (e.g., 500). Saalis should die before the emulator if the system is truly out of memory. The appliance's primary function is playing games, not cataloguing them.

### Graceful Degradation Tiers

| Tier | Available Memory | Behaviour |
|------|-----------------|-----------|
| **Normal** | >256 MB | Full hot store, background enrichment, cover art cache |
| **Constrained** | 128-256 MB | Reduced hot store, deferred enrichment, smaller cover art |
| **Pressure** | 64-128 MB | No hot store (SQLite direct), no background work, text-only fallback for art |
| **Critical** | <64 MB | Serve cached HTML only, no new queries, wait for pressure to subside |

---

## 4. Read-Only Root Filesystem

Batocera's root filesystem is squashfs — a compressed, read-only filesystem image. The system boots from this image and overlays writable storage for user data. This is a deliberate design choice for resilience: the OS cannot be corrupted by power loss because it is never written to.

### Where Saalis Writes

Three writable locations are available:

1. **`/userdata` (persistent partition).** This is the primary writable storage, typically on the same SD card or USB drive as the squashfs root. Batocera formats this as ext4 by default (btrfs optional). Saalis stores its SQLite database, configuration, download state, and cached cover art here. Path: `/userdata/system/saalis/` for the database and config, `/userdata/roms/` for the ROM library itself.

2. **`tmpfs` (RAM-backed).** Batocera mounts tmpfs at `/tmp` and `/run`. The hot store's memory-mapped backing file (if using the persistent mmap strategy) should live on tmpfs during normal operation — it is rebuilt from SQLite on every boot anyway. Temporary download chunks, extraction staging, and transient query results also belong here.

3. **Persistent partition for hot store manifest.** On clean shutdown, the hot store manifest (list of cached segment IDs and access metadata) is written to `/userdata/system/saalis/hot_manifest.bin`. On next boot, this manifest guides prioritised cache warming from SQLite. If the shutdown was unclean (no manifest), the system falls back to cold start with demand-driven loading.

### Filesystem Detection

Saalis must detect its storage environment at startup:

- Is the root filesystem read-only? (Check `statvfs` for `ST_RDONLY` flag.) If yes, do not attempt writes outside `/userdata` and `/tmp`.
- What filesystem is `/userdata`? (Check `/proc/mounts` or `statfs`.) If btrfs, enable reflink copies for ROM installation. If ext4, fall back to `std::io::copy`.
- Is `/userdata` on flash storage? (Check `/sys/block/*/queue/rotational`.) If yes, enable write-minimisation strategies (larger WAL checkpoint intervals, tmpfs for hot store).

---

## 5. USB Hotplug

Users add ROMs by plugging in USB drives. The appliance must detect the drive, scan it, and integrate the content without user intervention.

### Detection Chain

1. **udev rules.** Batocera already handles USB automounting. Saalis registers udev rules (or monitors udev events via `libudev` bindings) to detect new block device appearances. When a USB storage device is mounted, saalis receives a notification with the mount point.

2. **Mount monitoring.** As a fallback (or complement), monitor `/proc/mounts` or use inotify on `/media`/`/run/media` for new mount point appearances. The `notify` crate on Linux uses inotify and can watch these directories.

3. **Scan on mount.** When a new mount is detected, scan for ROM directories using the same heuristics as first-run: look for known platform directory names, common ROM file extensions (.nes, .sfc, .bin, .iso, .chd, .zip, .7z), and Batocera-standard directory structure.

4. **Integration options.** Two strategies, configurable:
   - **Symlink integration:** Create symlinks from the library's platform directories to the USB drive's ROM directories. ROMs appear in the library instantly. When the drive is removed, the symlinks become dangling — saalis marks those entities as "offline" rather than deleting them.
   - **Copy integration:** Copy ROMs from the USB drive to local storage (`/userdata/roms/`). Use reflinks if both are on btrfs, otherwise `std::io::copy`. Show progress in the UI. The user can unplug the drive after copying completes.

### Handling Removal

When a USB drive is unmounted (udev event or inotify on mount point disappearance):

1. Mark all entities sourced from that drive as "offline."
2. Do not delete metadata, cover art, or catalogue entries — the drive may return.
3. In the UI, show offline entities with a visual indicator (greyed out, "drive disconnected" label).
4. If the drive returns (same UUID), automatically restore the entities to "online" status.

The *arr research confirms that handling dangling symlinks gracefully is essential. The filesystem optimisation research notes that symlinks across filesystem boundaries work fine but must be validated on access.

---

## 6. Kiosk-Mode UI

The user sits on a couch, holds a gamepad, and views a TV at 3 metres distance. There is no keyboard. There is no mouse. The UI must work under these constraints.

### Gamepad Navigation Model

The web UI (HTMX + SSE over axum) must translate gamepad input into navigation:

- **D-pad/left stick:** Directional focus movement across a grid or list of items. The browser does not natively handle gamepad focus, so JavaScript must map gamepad events (via the Gamepad API) to focus changes.
- **A/South button:** Select/confirm. Equivalent to Enter/click.
- **B/East button:** Back/cancel. Navigate up in the view hierarchy.
- **Shoulder buttons (L/R):** Page through collections or categories. Maps to "next page" / "previous page" in paginated views.
- **Start/Menu:** Open settings or system menu.

### TV-Distance Readability

- **Minimum font size:** 24px for body text, 32px for titles. Standard web font sizes (14-16px) are unreadable at 3 metres on a 1080p display.
- **High contrast:** Light text on dark backgrounds. Retro gaming appliances are used in dim living rooms.
- **Large touch targets:** Every interactive element must be at least 48x48px, preferably larger. Focus indicators must be prominent — a subtle outline is invisible on a TV.
- **Cover art dominance:** The primary visual element is cover art, not text. Grid layouts with large thumbnails, minimal metadata text. Details are revealed on selection, not on browse.

### HTMX + SSE for Appliance UI

The HTMX approach has specific advantages for kiosk mode:

- **Server-rendered HTML.** No JavaScript framework to load, parse, and execute. On a Raspberry Pi 4's browser (typically Chromium via CEF or a lightweight WebKit), this eliminates the largest source of startup delay and runtime jank.
- **SSE for live updates.** Server-Sent Events push catalogue progress, enrichment status, and download progress without polling. The connection is lightweight (single HTTP/1.1 connection, text/event-stream) and works through any proxy or reverse proxy.
- **Partial page updates.** HTMX swaps HTML fragments, not full pages. Navigating from the game grid to game details replaces the content area, not the entire page. This eliminates full-page reloads and the associated flash of unstyled content.
- **Minimal client state.** The server holds all state. If the browser crashes and restarts (which happens on resource-constrained hardware), the user returns to a functional UI immediately — no client-side state to reconstruct.

### What This Means in Practice

The UI serves from saalis's embedded axum web server. On Batocera, a lightweight browser (Chromium kiosk mode or `cage` + `wlroots` + WebKit) opens to `http://localhost:{port}`. The browser runs in fullscreen with no address bar, no tabs, no browser chrome. JavaScript is minimal: gamepad input mapping, focus management, and SSE event handlers for HTMX-triggered updates.

---

## 7. Embedded Media Systems Comparison

How do established appliance-mode media systems handle the constraints saalis will face?

### Kodi (XBMC)

Kodi is the closest architectural ancestor to what saalis targets. It runs on the same hardware (RPi, x86 mini-PCs), serves a 10-foot UI optimised for TV viewing with remote/gamepad input, and manages a media library with metadata enrichment.

**What works:** Kodi's skinning system allows radically different UIs on the same engine. Its library scanning is background and incremental. The SQLite-backed library (MyVideos/MyMusic databases) handles hundreds of thousands of items on constrained hardware. Kodi's addon system (Python-based) enables community extensions without recompiling.

**What does not work:** Kodi's C++ codebase is monolithic and difficult to extend at the core level. Python addons are sandboxed but slow. The scraper system (metadata enrichment) is tightly coupled to specific providers, causing breakage when upstream APIs change — the same TVDB lock-in problem the *arr stack suffers from. Startup time on Raspberry Pi is 15-30 seconds, largely due to skin loading and library initialisation.

### Plex / Jellyfin

Both are server-client architectures: a media server scans, catalogues, and transcodes; clients (apps, web UI) connect remotely.

**What works:** The separation of server and client allows the heavy work (scanning, transcoding) to run on capable hardware while the display runs on anything with a browser. Jellyfin's open-source model avoids vendor lock-in. Both handle library updates incrementally with filesystem watching.

**What does not work:** Neither is designed for single-board-computer deployment as a primary server. Plex's server requires significant RAM for transcoding. Jellyfin's .NET runtime adds overhead. Both assume always-on server hardware, not an appliance that boots and shuts down with a power switch.

### EmulationStation / RetroArch

These are saalis's direct neighbours in the Batocera ecosystem.

**What works:** EmulationStation is purpose-built for the exact hardware and UI constraints saalis faces. Its gamelist.xml format is simple and human-editable. The theming system supports TV-distance UIs with gamepad navigation. RetroArch's core system (dynamically loaded emulator cores) is the gold standard for runtime-extensible emulation. Both start quickly on constrained hardware.

**What does not work:** EmulationStation's metadata is filesystem-based (XML files per system directory), not database-backed. Searching across systems requires scanning all XML files. There is no enrichment pipeline — metadata is manually curated or batch-imported via scraper tools. No quality management, no source tracking, no download integration. EmulationStation is a launcher, not a library manager.

### Lessons Synthesised

The pattern across all these systems: **startup time and input responsiveness are the user-visible metrics that matter most on appliance hardware.** Kodi's 15-30 second startup is tolerable because users expect it. EmulationStation's 3-5 second startup sets a higher bar that saalis should target. Both achieve this by deferring heavy work (library scanning, metadata enrichment) to background processes that run after the UI is responsive.

---

## 8. Startup Time

From power-on to usable UI. This is the appliance's first impression on every boot.

### The Time Budget

On a Raspberry Pi 4 running Batocera:

- **Kernel + initramfs:** ~3 seconds (Batocera-optimised)
- **Batocera services + EmulationStation:** ~5-8 seconds
- **Saalis target:** UI responsive within 2 seconds of saalis process start

The 2-second target is aggressive but achievable with the right startup sequence.

### Prioritised Startup Sequence

1. **Bind the web server immediately** (< 50ms). Open the socket, start accepting connections. Serve a static "loading" page if the hot store is not ready. This prevents the browser from showing a connection-refused error.

2. **Open the SQLite database** (< 100ms). Verify integrity with a quick `PRAGMA integrity_check` on the header (not a full check). Set WAL mode and pragmas.

3. **Load the hot store manifest** (< 100ms). Read `hot_manifest.bin` from the previous session. This is a small file (segment IDs + access metadata) that tells the warming process what to load first.

4. **Serve the initial view from SQLite** (< 500ms). The first page of content (e.g., 20 items for the default "All Games" grid) can be loaded directly from SQLite with a simple query. Cover art paths are resolved but images are loaded lazily by the browser. The UI is now interactive.

5. **Begin background warming** (ongoing). A background work unit reads the hot store manifest and loads column segments in priority order. As segments load, query performance improves. The user may not notice — the first page was served from SQLite, and subsequent pages benefit from the warming cache.

6. **Begin background scanning** (ongoing, lower priority). Check for new/changed/removed ROM files by comparing filesystem state against the catalogue. This runs at the lowest scheduler priority, below cache warming and user-initiated queries.

### Lazy Metadata Loading

Metadata that is not needed for the initial grid view is loaded on demand:

- **Grid view needs:** Title, cover art path, platform. These are in the "essential" column group, loaded first.
- **Detail view needs:** Description, release date, genre, publisher, screenshots. Loaded when the user selects a game.
- **Enrichment data:** Ratings, reviews, related games. Loaded in background, displayed when available.

This three-tier loading strategy means the initial view requires minimal data per entity (title + art path + platform = ~200 bytes each). For a library of 10,000 games, the initial grid data is ~2 MB — trivially fast to load from SQLite.

---

## 9. Storage Management

SD cards have limited write endurance. Consumer-grade cards tolerate 3,000-10,000 write cycles per cell. At 4 KB page size, a 32 GB card with 3,000-cycle endurance and wear levelling can sustain roughly 96 TB of total writes — but concentrated writes to a few blocks (e.g., a hot SQLite WAL file) can exhaust those blocks far sooner than the theoretical total.

### Minimising Writes

1. **Hot store on tmpfs.** The in-memory columnar store's working data lives in RAM. If using the mmap-backed persistent store, the backing file lives on tmpfs (`/tmp` or `/run`), which is RAM-backed and generates zero flash writes.

2. **WAL checkpointing strategy.** Do not use aggressive checkpointing. Set `PRAGMA wal_autocheckpoint` to a high value (e.g., 10000 pages, ~40 MB). Trigger manual checkpoints at natural boundaries: after a catalogue cycle completes, after enrichment finishes, on clean shutdown. Avoid checkpointing during active browse sessions.

3. **Batch writes.** The scheduler's epoch-based dirty tracking collects all mutations within a work unit and flushes them to SQLite in a single transaction. This converts many small writes into one sequential WAL append. The *arr stack's "database is locked" errors under concurrent load validate this design — saalis's single-writer scheduler prevents this class of error entirely.

4. **Configuration caching.** Read configuration from SQLite once at startup, cache in memory. Write configuration changes immediately (they are rare and user-initiated), but do not re-read from disk on every access.

5. **Cover art write-once.** Downloaded cover art is written to `/userdata/system/saalis/art/{entity_id}.webp` once and never modified. If art needs to be updated (better source found), write the new file with a different name and atomically update the database reference. The old file is deleted in a deferred cleanup pass.

### Write Amplification Awareness

SQLite's page-based storage means a single-byte change rewrites a full 4 KB page. The research confirms this is inherent to SQLite's architecture. Mitigations:

- Use `PRAGMA page_size=4096` (matching the filesystem block size, which is ext4's default). Misaligned page sizes cause additional write amplification at the filesystem level.
- Group related metadata updates into single transactions to amortise page rewrites.
- Consider increasing page size to 8192 or 16384 for the metadata-heavy workload — larger pages reduce the number of page writes for update-heavy columns, at the cost of more data per rewrite.

---

## 10. Novel Ideas

### 10.1 Instant-On with Persistent Memory-Mapped Hot Store

The hot store research recommends SIEVE eviction with a persisted manifest for warm restarts. A more aggressive variant: memory-map the entire columnar store to a file.

**Concept:** The hot store's column segments are laid out in a contiguous memory-mapped file (`mmap` with `MAP_SHARED`). During normal operation, reads and writes go through memory-mapped pointers — no serialisation/deserialisation overhead. On clean shutdown, the file is `msync`'d and its path recorded. On next boot, `mmap` the same file. The hot store is instantly warm — no SQLite queries needed for cached data.

**Where to store the file:** On tmpfs during operation (RAM-backed, fast, no flash wear). On clean shutdown, copy to persistent storage (`/userdata/system/saalis/hot_store.bin`). On next boot, copy from persistent storage to tmpfs and mmap. This gives the performance of RAM-backed storage with the persistence of disk-backed storage, at the cost of one file copy per boot/shutdown cycle.

**Trade-offs:**
- The mmap file must have a stable binary layout. Column segment format changes require migration or invalidation.
- Unclean shutdown loses the hot store (tmpfs is volatile). Fall back to manifest-based warming or cold start.
- The file size is bounded by available RAM and tmpfs capacity. On a 2 GB system, the hot store file might be 256 MB — a 256 MB copy on startup adds ~1 second on USB 3.0, ~5 seconds on USB 2.0.

**Verdict:** Worth prototyping. The "instant warm" benefit is substantial for appliances that boot frequently. The complexity is manageable if the column segment layout is fixed and versioned. For first release, the manifest-based approach (load segment IDs from a small file, then fetch from SQLite) is simpler and sufficient.

### 10.2 Satellite Mode

**Concept:** Two saalis instances — a "headless" instance on a NAS (or any always-on server) and a "display" instance on the Batocera appliance. The NAS instance handles the heavy work: downloading, enriching, organising. The appliance instance handles display and launching.

**Sync protocol:** The NAS instance exposes a sync endpoint. The appliance instance periodically (or on-demand) pulls:
- SQLite database snapshots (or WAL deltas)
- Cover art files (delta sync — only new/changed)
- Entity status updates (new content available, enrichment complete)

The appliance stores a local SQLite copy and art cache. Browsing and searching are fully local — no network latency. Launching a game reads the ROM from the NAS via the network (NFS/SMB mount, already standard in Batocera).

**Why this matters:** Many retro gaming setups involve a NAS for ROM storage with multiple appliances (living room, bedroom, kids' room). Currently, each appliance maintains its own gamelist.xml and scraper data independently. Satellite mode provides a single source of truth with lightweight local replicas.

**Implementation path:** This is a v2+ feature. The core requirement is that the SQLite schema and art storage are designed for replication from the start — which they are, since SQLite's single-file design makes snapshot-based replication trivial.

### 10.3 Hardware-Adaptive Quality

**Concept:** Saalis detects its hardware capabilities at startup and adjusts quality parameters automatically.

**Cover art resolution:**
- Raspberry Pi 4 (1 GB): 200x200 thumbnails, 400x400 detail view. WebP format, quality 60.
- Raspberry Pi 5 (4 GB): 300x300 thumbnails, 600x600 detail view. WebP quality 75.
- x86 mini-PC (8 GB): 400x400 thumbnails, 800x800 detail view. WebP quality 85.

**Detection method:** Read `/proc/cpuinfo` for CPU model, `/proc/meminfo` for total RAM, check for GPU capabilities via `/sys/class/drm/`. Map to a hardware tier (low/medium/high). Each tier defines default quality parameters that propagate through the enrichment pipeline.

**UI complexity scaling:**
- Low tier: Simple grid layout, no animations, no blur effects, minimal JavaScript. Every frame matters on a Pi 4 running Chromium.
- High tier: Smooth transitions, background blur on detail views, parallax scrolling on collection headers.

**Background work throttling:**
- Low tier: One enrichment worker, 1-second delay between API calls, suspend enrichment when the UI is actively scrolling.
- High tier: Four enrichment workers, concurrent API calls (respecting rate limits), background work does not affect UI responsiveness.

**Why this is better than user configuration:** The user does not know (and should not need to know) that their Pi 4 should use lower-resolution cover art. The appliance adapts to its own hardware. Power users can override the detected tier in settings.

### 10.4 Resumable First Run via Checkpoint Journal

The first-run scan of a large ROM library (tens of thousands of files across dozens of platform directories) can take minutes on an SD card. If the user unplugs mid-scan, the entire scan should not restart from scratch.

**Concept:** The scanning work unit maintains a checkpoint journal: a small file recording the last fully-scanned directory. On restart, resume from the checkpoint rather than the beginning. Within a directory, files are processed in deterministic order (sorted by name), so the checkpoint can record both directory and file position.

This is a refinement of the download recovery pattern applied to the catalogue scan itself. The journal is written atomically after each directory completes, so the worst case on power loss is re-scanning a single directory.

### 10.5 Offline Metadata Bundles

**Concept:** Ship a compressed metadata bundle (title, platform, region, year, genre) for the most common ROM sets (No-Intro, Redump) within the saalis binary itself or as a downloadable supplementary file. First-run enrichment for known ROMs becomes a local database lookup rather than an API call.

**Impact:** A No-Intro DAT file for all platforms compresses to roughly 50-100 MB. Embedding this as a build-time resource is excessive, but downloading it once and caching on `/userdata` is viable. The first-run experience improves dramatically — instead of "scanning... enriching... (waiting for API responses)," the user sees fully-catalogued games within the time it takes to scan the filesystem.

**Combined with CRC matching:** Hash each ROM, look up in the local metadata bundle by CRC32 or SHA-1 (the standard identifiers in No-Intro DATs). Matches are instant. Only unmatched ROMs (hacks, homebrew, non-standard dumps) require online enrichment.

### 10.6 Power-State-Aware Scheduling

**Concept:** Detect whether the appliance is running on AC power or battery (relevant for portable devices like the Steam Deck or handheld retro devices). On battery, disable all background work (enrichment, download, scan). On AC power, resume background work.

**Detection:** Read `/sys/class/power_supply/*/online` and `/sys/class/power_supply/*/status`. Subscribe to udev events for power supply changes.

**Combined with time-of-day awareness:** If the appliance is typically used in the evening (8 PM - midnight), schedule heavy background work (full library rescan, bulk enrichment) for off-peak hours. The scheduler already supports priority-based work units — this adds a time-based priority modifier.

---

## Synthesis: The Appliance Design Checklist

The research documents converge on a set of principles that define appliance-first design for a single-binary system like saalis:

**1. The user never configures.** Every setting has a default that works. Hardware detection replaces user input. Directory conventions replace path configuration. The system adapts to its environment, not the reverse.

**2. Every write is crash-safe.** WAL mode for SQLite. Atomic renames for files. State journals for long-running operations. The system recovers from power loss without user intervention and without data corruption.

**3. Memory is a shared, finite resource.** A single unified pool with SIEVE eviction. Cooperative degradation under pressure. The hot store is a performance optimisation, not a correctness requirement — the system functions (slower) without it.

**4. The root filesystem is sacred.** Never write to it. Everything mutable lives on `/userdata` (persistent) or tmpfs (volatile). Detect the storage environment at startup and adapt.

**5. External storage comes and goes.** USB drives appear and disappear. Network shares become unreachable. Entities sourced from removable media go "offline" gracefully and come "online" automatically when the storage returns.

**6. The UI is a 10-foot experience.** Large text, high contrast, cover art dominance, gamepad navigation. Server-rendered HTML eliminates client-side complexity. Partial updates via HTMX prevent full-page reloads.

**7. Startup time is a feature.** Bind the socket immediately. Serve from SQLite while the hot store warms. Load metadata lazily. Show content before enrichment completes. The target is 2 seconds from process start to interactive UI.

**8. Flash storage has a finite lifespan.** Minimise writes. Use tmpfs for volatile data. Batch database writes. Checkpoint at natural boundaries, not continuously. Write cover art once, never rewrite.

**9. Learn from the ecosystem.** The *arr stack proves that SQLite + WAL works for single-user appliances. Kodi proves that 10-foot UIs work on ARM hardware. EmulationStation proves that 3-5 second startup is achievable. Jellyfin proves that server-rendered UIs work for media browsing. Take what works; avoid what does not.

**10. Design for the hardware you have, not the hardware you wish you had.** A Raspberry Pi 4 with 2 GB RAM and an SD card is the floor, not the ceiling. Every architectural decision must be validated against this baseline. Features that work on x86 with NVMe but fail on ARM with SD card are not features — they are bugs.

---

## Sources

This synthesis draws from the following research documents:

- [Filesystem Access Optimisation](2026-03-14-research.filesystem-optimisation.md) — reflinks, zero-copy transfers, io_uring, atomic writes, directory traversal, archive extraction, checksums, file watching
- [SQLite Alternatives](2026-03-14-research.sqlite-alternatives.md) — DuckDB, libSQL/Limbo, ReDB, Sled, RocksDB, LanceDB, LMDB, Polars/DataFusion evaluation
- [Hot/Cold Storage Strategies](2026-03-14-research.hot-cold-storage-strategies.md) — eviction algorithms (LRU, ARC, SIEVE), resurfacing, dirty tracking, industry implementations, startup warming
- [*arr Stack Lessons](2026-03-14-research.arr-stack-lessons.md) — architecture evolution, data model, indexer abstraction, download pipeline, quality profiles, metadata handling, extensibility

Additional references:

- [Batocera Architecture Wiki](https://wiki.batocera.org/batocera.linux_architecture)
- [Linux PSI (Pressure Stall Information)](https://docs.kernel.org/accounting/psi.html)
- [Gamepad API (W3C)](https://www.w3.org/TR/gamepad/)
- [HTMX Documentation](https://htmx.org/docs/)
- [Kodi Wiki — Skinning](https://kodi.wiki/view/Skinning)
- [EmulationStation Documentation](https://emulationstation.org/gettingstarted.html)
