# Synthesis: Content Acquisition Pipeline

**Date:** 2026-03-14
**Sources:**
- `2026-03-14-research.filesystem-optimisation.md`
- `2026-03-14-research.arr-stack-lessons.md`
**Context:** End-to-end pipeline for acquiring, verifying, installing, and organising game content (ROMs, ISOs, disc images) on a Batocera appliance with constrained memory and storage.

---

## 1. Pipeline Overview

The content acquisition pipeline is the path a game takes from "saalis knows it exists" to "the user can launch it." In the *arr stack this is the grab-download-import-organise sequence. In saalis it must be tighter: a single DAG of work units flowing through the scheduler, each stage emitting events that trigger the next. The pipeline is:

```
SourceResolved
  -> DownloadWork       (grab + transfer)
  -> VerifyWork         (BLAKE3 streaming checksum)
  -> ExtractWork        (archive decompression, if needed)
  -> InstallWork        (reflink/hardlink/copy to library path)
  -> OrganiseWork       (symlink into emulator directory structure)
  -> ContentInstalled   (terminal event — UI update, notification)
```

Every stage is a registered work unit under the Curator subsystem. The scheduler's DAG wires them via triggers/emits. Failure at any stage emits a `PipelineStageFailure` event that the Doctor can act on (retry, blocklist source, notify user). The entire pipeline is idempotent: re-running any stage with the same inputs produces the same result without duplication.

This document synthesises the filesystem research and *arr architecture lessons into concrete design decisions for each stage.

---

## 2. Stage 1: Download (DownloadWork)

### Triggers and Emits

- **Triggers:** `SourceResolved` — a Sourcer connector has located a download URL, torrent magnet, or local file path for a game.
- **Emits:** `DownloadComplete` on success, `PipelineStageFailure` on failure.
- **Batch target:** 1 (downloads are inherently per-item).

### Preallocation

Before writing any bytes, call `fallocate(FALLOC_FL_KEEP_SIZE)` via the `nix` crate to reserve the expected file size on disk. This achieves two things:

1. **Early out-of-space detection.** A 4 GB ISO download that would fail at 3.8 GB instead fails immediately, before any bandwidth is spent.
2. **Reduced fragmentation.** ext4 can allocate contiguous extents when the full size is known in advance.

On btrfs, fallocate reserves allocator space without physical writes — same benefit. On macOS (APFS), `fcntl(F_PREALLOCATE)` is the equivalent, though less critical since APFS uses copy-on-write allocation.

### Download Target Directory

Following the *arr stack's lesson on hardlink compatibility, all downloads land in a staging directory on the **same filesystem** as the library:

```
/userdata/saalis/
  staging/          <- downloads land here
  store/            <- content-addressable blob storage
  library/          <- organised per-emulator symlink trees
```

Same-filesystem staging is essential. If staging and library are on different mount points, reflinks and hardlinks fail, and every "install" becomes a full copy. The *arr community learned this the hard way — TRaSH Guides' most-read page is about getting the directory structure right for hardlinks. Saalis should enforce this at configuration time: warn (or refuse) if staging and library paths resolve to different filesystem IDs (compare `stat().st_dev`).

### Streaming BLAKE3 During Download

Rather than downloading to disk and then hashing in a separate pass, DownloadWork feeds every received chunk through a `blake3::Hasher` before writing it. This is essentially free: BLAKE3 processes data at 3-4 GB/s single-threaded (NEON on ARM, AVX2 on x86), which is orders of magnitude faster than any network transfer. The download I/O is the bottleneck, never the hash computation.

The hash accumulates as bytes arrive. When the download completes, the final BLAKE3 digest is immediately available — no second pass over the file. This streaming verification pattern eliminates an entire pipeline stage for the common case.

```
network -> chunk -> blake3::Hasher::update(chunk) -> file.write(chunk) -> repeat
                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                    adds ~0% overhead at network speeds
```

If the source provides a CRC32 or MD5 checksum (common for ROM databases like No-Intro and Redump), a secondary hasher runs in parallel. BLAKE3 is stored as the internal integrity hash; the legacy hash is compared against the source's declared value for provenance verification.

### Torrent-Aware Downloads

When the source is a torrent, DownloadWork delegates to an external client (following the *arr pattern) rather than implementing BitTorrent in-process. The work unit:

1. Sends the magnet/torrent to the configured download client (qBittorrent, Transmission) via its API.
2. Assigns a saalis-specific category label to isolate from the user's other downloads.
3. Polls the client for status (or subscribes to its event feed if available).
4. On completion, reads the file from the client's completed directory — which is configured to be within `staging/` on the same filesystem.

The BLAKE3 streaming hash is not possible for torrent downloads (the client writes the file), so a post-download hash pass is required. Use `posix_fadvise(POSIX_FADV_SEQUENTIAL)` before hashing to hint the kernel for aggressive read-ahead, then `posix_fadvise(POSIX_FADV_DONTNEED)` after hashing to release the pages from cache.

---

## 3. Stage 2: Verify (VerifyWork)

### When Verification Is Separate

For direct downloads with streaming BLAKE3, verification is folded into DownloadWork — there is no separate stage. VerifyWork exists as an independent stage for cases where the hash was not computed during download:

- Torrent downloads (file written by external client).
- Manual file imports (user places a ROM file into a watched directory).
- Re-verification sweeps (periodic integrity checks by the Housekeeper).

### BLAKE3 With Rayon Parallelism

For post-download verification, use `blake3::Hasher::update_rayon()` to hash across all available cores. On a 4-core Batocera device hashing a 4 GB ISO:

- Single-threaded BLAKE3 with NEON: approximately 1 second.
- 4-thread rayon BLAKE3: approximately 250 ms.

The Merkle tree structure of BLAKE3 makes this parallelism free — different chunks of the file are hashed independently and combined. No other hash algorithm offers this.

### Filesystem-Level Checksums on btrfs

On btrfs, data checksumming is built into the filesystem (crc32c by default, with xxhash, sha256, and blake2b available). Every read verifies the on-disk checksum automatically. This means that for users on btrfs, application-level re-verification sweeps are redundant — the filesystem already detects bit rot on every access.

Saalis should detect btrfs at startup (check the filesystem type via `statfs`) and skip periodic re-verification sweeps on btrfs volumes. The initial post-download BLAKE3 hash is still computed (for provenance tracking and deduplication), but the Housekeeper's periodic integrity checks become no-ops on btrfs.

On ext4 (Batocera's default), there is no data checksumming — only metadata. Application-level BLAKE3 re-verification remains necessary.

### Legacy Checksum Compatibility

ROM databases (No-Intro, Redump, TOSEC) publish CRC32, MD5, and SHA-1 checksums. VerifyWork computes these alongside BLAKE3 and stores all hashes in the metadata table. When matching against a ROM database, the legacy hash is used for lookup; the BLAKE3 hash is used for internal integrity tracking. This dual-hash approach avoids re-hashing when a user wants to verify their ROM collection against a No-Intro DAT file.

---

## 4. Stage 3: Extract (ExtractWork)

### Triggers and Emits

- **Triggers:** `DownloadComplete` (or `VerifyComplete` if verification was separate).
- **Emits:** `ExtractionComplete` on success.
- **Skip condition:** If the downloaded file is already in its final format (an uncompressed `.sfc`, `.nes`, `.iso`), ExtractWork emits `ExtractionComplete` immediately without any I/O.

### Streaming Extraction for Memory-Constrained Devices

Batocera appliances frequently run on devices with 1-4 GB of RAM. A naive extraction of a 2 GB 7z archive containing a 4 GB ISO would require holding the entire decompressed output in memory. This is not acceptable.

Streaming extraction reads from the archive and writes to disk without buffering the full output:

- **ZIP** (`zip` crate): Streaming by default. Entries are decompressed one at a time. ZIP's central directory allows extracting individual files without reading the entire archive — useful for multi-game archives where only one ROM is wanted.
- **tar.gz / tar.zst** (`tar` + `flate2`/`zstd`): Tar is inherently streaming. The decompression layer wraps the input stream, and entries are written to disk as they are decoded.
- **7z** (`sevenzip-mt`): Uses block-level parallelism — compressed blocks are decompressed in parallel on the rayon thread pool, and each block's output is written to disk and the block's memory freed immediately. Memory usage is bounded to the LZMA2 dictionary size (typically 64 MB) plus one output buffer per active thread. This is critical for solid 7z archives where single-threaded decompression would be CPU-bound for minutes.
- **RAR** (`unrar`): The C binding handles its own streaming internally.

### Streaming Hash During Extraction

The same pattern used during download applies here: as bytes are decompressed and written to the output file, they pass through a BLAKE3 hasher. When extraction completes, the hash of the extracted content is known without a second read pass. This is particularly valuable for large ISOs extracted from 7z archives — the extraction may take minutes, and a separate hash pass would double the wall-clock time.

### Extraction to Staging, Not Library

Extracted files land in `staging/`, not directly in the library. This keeps the library clean during multi-file extractions that might fail partway through. Only after extraction completes and the output is verified does InstallWork move the content to its final location.

---

## 5. Stage 4: Install (InstallWork)

### The Core Operation: Reflink-First Installation

InstallWork moves verified content from `staging/` to `store/` (the content-addressable storage, described in section 9). The installation strategy follows a strict preference order:

1. **Reflink** (`reflink-copy::reflink_or_copy`): On btrfs or APFS, this is a metadata-only operation — the file appears to be copied, but the underlying disk blocks are shared. A 4 GB ISO "copies" in microseconds. The space cost is zero. Since ROMs are never modified after installation, the blocks remain shared forever.

2. **Hardlink** (`std::fs::hard_link`): On ext4 or XFS where reflinks are unavailable but staging and store are on the same filesystem. The file is not copied at all — two directory entries point to the same inode. Space cost is zero. Unlike reflinks, hardlinks share the inode directly, so both paths must remain valid while the other exists.

3. **`std::io::copy`**: Fallback when reflinks and hardlinks are both unavailable (cross-filesystem moves, FAT32 external drives). Rust's `io::copy` on Linux automatically uses `copy_file_range` > `sendfile` > `splice` > userspace loop, so this is already as optimised as possible without manual syscall invocation.

### Atomic Installation: Write-Fsync-Rename

For the copy fallback path, installation must be crash-safe. A power failure mid-copy must not leave a partial file in the store that appears valid. The pattern:

1. Create a temporary file in `store/` (same directory = same filesystem, guaranteeing atomic rename).
2. Copy all data to the temporary file.
3. `fsync()` the file descriptor to flush data to disk.
4. `fsync()` the directory descriptor to persist the directory entry.
5. `rename()` the temporary file to its final content-addressed path.

The `atomic-write-file` crate handles steps 1-5 correctly, including the directory fsync that `tempfile::NamedTempFile::persist()` omits. For reflinks and hardlinks, atomicity is inherent — they are metadata operations that are atomic on all relevant filesystems.

### Cleanup of Staging

After successful installation, the staging file is removed. For torrent sources that are still seeding, the staging file is left in place — the hardlink (or reflink) in `store/` shares the same blocks, so the torrent client continues seeding from the staging path while the library serves from the store path. This is exactly the *arr hardlink-for-seeding pattern, applied to game content.

When seeding completes, a cleanup work unit removes the staging copy. If hardlinks were used, removing the staging path does not affect the store copy (the inode's link count drops from 2 to 1, and the data persists). If reflinks were used, removing either copy does not affect the other (reflinks are independent once created).

---

## 6. Stage 5: Organise (OrganiseWork)

### Symlink Trees Per Emulator

Batocera expects ROMs in specific directory structures:

```
/userdata/roms/
  snes/       <- RetroArch SNES core
  psx/        <- DuckStation / Beetle PSX
  n64/        <- Mupen64Plus
  ...
```

OrganiseWork creates symlinks from the emulator-expected paths to the content-addressable store:

```
/userdata/roms/snes/Super Mario World.sfc
  -> /userdata/saalis/store/ab/cd/abcdef1234...5678.sfc
```

Symlinks are the correct tool here, not hardlinks, because:
- Emulator directories may be on different filesystems (USB drives, network shares).
- Symlinks are visible in directory listings (the user can see they are links).
- Removing a symlink never affects the source file.
- Symlinks work across mount boundaries.

OrganiseWork handles dangling symlinks gracefully — if a store file is removed (user deletes a game), the symlink is cleaned up on the next Housekeeper sweep. If an external drive is disconnected, dangling symlinks to that drive are detected and reported but not removed (the drive may return).

### Profile-Specific Views

Different user profiles (Admin, User, Child) may have different game libraries. OrganiseWork creates per-profile symlink trees:

```
/userdata/roms-profiles/
  admin/
    snes/Super Mario World.sfc -> /userdata/saalis/store/ab/cd/...
    psx/Final Fantasy VII (Disc 1).chd -> /userdata/saalis/store/ef/01/...
  child/
    snes/Super Mario World.sfc -> /userdata/saalis/store/ab/cd/...
    <- no PSX games (parental restriction)
```

The physical ROM file exists once in the store. Each profile gets its own symlink tree with only the games that profile is authorised to access. Adding a game to a profile is creating a symlink. Removing it is removing a symlink. No data is copied or moved.

---

## 7. The *arr Hardlink Pattern: Application to Game Sourcing

The *arr stack's hardlink strategy exists to solve a specific problem: a file must be available at two paths simultaneously (the download client's seeding path and the library's organised path) without doubling disk usage. This is directly applicable to saalis when sourcing games from torrents.

### Where It Applies

**Torrent-sourced ROMs:** When a torrent download completes, the torrent client continues seeding from the original file path. Saalis needs the same file in the store under a content-addressed name. A hardlink (or reflink on btrfs) satisfies both requirements with zero additional space.

**ROM sharing across profiles:** When two profiles both have the same game, the underlying file is the same. Rather than two copies or even two hardlinks, both profiles' symlink trees point to the same store path. This is inherently deduplicated by the content-addressable store design.

### Where It Does Not Apply

**Cross-filesystem scenarios:** If the user's download client writes to an external drive and the library is on the internal drive, hardlinks are impossible. Saalis must detect this (compare `st_dev` from stat) and fall back to copy-then-delete. The UI should recommend configuring the download client to use the same filesystem as the library, echoing the *arr community's hard-won lesson.

**Non-torrent sources:** Direct HTTP downloads land in saalis-managed staging, and there is no external client that needs continued access. The staging file can be renamed (atomic move) directly into the store rather than hardlinked.

---

## 8. Directory Scanning With jwalk

### Startup Library Reconciliation

When saalis starts, it must reconcile its database against the actual filesystem state. Files may have been added, removed, or modified while saalis was not running. inotify does not detect changes made while the watcher was not running.

`jwalk` parallelises directory traversal at the directory level using rayon, achieving approximately 4x the throughput of single-threaded `walkdir` on directories with 10,000+ entries. For a ROM collection spread across multiple system directories (each containing hundreds to thousands of files), this translates to a startup scan completing in seconds rather than tens of seconds.

The scan collects `(path, mtime, size, inode)` tuples and compares them against the stored catalogue:

- **New files** (path not in DB): Queue for identification and import.
- **Modified files** (mtime or size changed): Queue for re-verification.
- **Missing files** (path in DB, not on disk): Mark as unavailable; check if the symlink target is on a disconnected removable drive before removing the catalogue entry.
- **Unchanged files** (path, mtime, size all match): Skip — no work needed.

After the initial scan, `notify` watches the ROM directories for real-time incremental updates. On Linux (inotify), watches are per-directory — a library with 500 directories requires approximately 500 watches, well within the default 8,192 limit.

### Stat Avoidance

On ext4 (which supports `d_type` in `getdents64`), `jwalk` can determine whether an entry is a file or directory without a separate `stat()` call. This eliminates thousands of syscalls during the initial scan. The metadata comparison (mtime, size) still requires stat, but only for entries that need checking — not for every directory entry encountered during traversal.

---

## 9. Content-Addressable Storage

### The Concept

Instead of storing ROMs at user-meaningful paths (`Super Mario World.sfc`), the store uses content-addressed paths derived from the file's BLAKE3 hash:

```
/userdata/saalis/store/
  ab/cd/abcdef1234567890abcdef1234567890abcdef1234567890abcdef12345678.sfc
  ef/01/ef01234567890abcdef1234567890abcdef1234567890abcdef1234567890ab.chd
```

The first two levels of directory nesting (`ab/cd/`) prevent any single directory from accumulating too many entries (which degrades filesystem performance on ext4 with htree and on btrfs with its B-tree directory index).

### Why Content-Addressable

**Automatic deduplication.** If the same ROM is downloaded from two different sources, or if two users import the same file, it hashes to the same path. The second write is a no-op (the file already exists). No separate deduplication pass is needed.

**Integrity by construction.** The path encodes the expected hash. Verifying a file's integrity is comparing `blake3::hash(file_contents)` against the filename. If they mismatch, the file is corrupt — no database lookup required.

**Profile sharing without copies.** Multiple profiles can reference the same store path via symlinks. One 4 GB ISO serves all profiles that have access to that game. On a device with 128 GB of storage, this can be the difference between fitting 30 games and fitting 30 games with 3 profiles.

**Safe concurrent access.** Content-addressed files are immutable by definition (modifying the content changes the hash, which changes the path). Multiple work units, or even multiple saalis instances, can read store files without locks.

### Garbage Collection

A store file is eligible for garbage collection when no symlink in any profile's library tree points to it. The Housekeeper's `PruneStaleData` work unit periodically scans the store for unreferenced blobs. On btrfs, this is assisted by the filesystem's own refcount tracking — a reflinked file with only one remaining reference is a candidate. On ext4, the link count (`stat().st_nlink`) indicates whether any hardlinks remain.

---

## 10. Lazy Extraction With On-Demand Decompression

### The Idea

Many ROM formats are distributed as compressed archives (7z, zip) containing a single large file. The traditional pipeline fully extracts the archive before installation. An alternative: store the archive as-is in the content-addressable store and decompress on-the-fly when the emulator launches.

### How It Would Work

For ZIP archives specifically (not 7z, not RAR — only ZIP supports true random access), the archive itself is the stored blob. When the emulator needs the ROM, saalis uses FUSE (Filesystem in Userspace) to present the archive's contents as a virtual directory:

```
/userdata/roms/snes/Super Mario World.sfc
  -> FUSE mount of /userdata/saalis/store/ab/cd/...zip:/Super Mario World.sfc
```

The FUSE layer decompresses on read, caching decompressed pages in memory. For emulators that read the entire ROM into memory at startup (most RetroArch cores), the full file is decompressed into RAM once — the same as if it were stored uncompressed. For emulators that use memory-mapped I/O, the FUSE layer handles page faults transparently.

### Trade-offs

**Space saving:** A zstd-compressed ROM collection can be 30-60% smaller than uncompressed. On a 128 GB device, this is 40-75 GB of recovered space.

**Startup latency:** Decompression adds 1-5 seconds to game launch for a 500 MB ROM. For sub-100 MB ROMs (SNES, GBA, N64), the latency is imperceptible.

**Complexity:** FUSE integration is a significant addition. The `fuser` crate provides a Rust FUSE interface, but FUSE is not available on all Batocera builds. This should be an opt-in feature for advanced users, not the default path.

**Recommendation:** Implement lazy extraction as a v2 feature. For v1, fully extract and store uncompressed. The content-addressable store still provides deduplication benefits without the FUSE complexity.

---

## 11. Delta Updates for ROM Patches

### The Problem

ROM patches (IPS, BPS, UPS formats) modify a base ROM to produce a variant — translation patches, bug fixes, randomisers. The traditional approach: apply the patch to produce a new file, store the full result. For a 32 MB SNES ROM with a 4 KB translation patch, this means storing 32 MB of nearly-identical data.

### The Approach

Store the base ROM in the content-addressable store. Store the patch as a separate small blob. At install time (OrganiseWork), apply the patch on-the-fly to produce the output file — but instead of storing the output, use btrfs `FICLONERANGE` to share the unmodified blocks with the base ROM and only allocate new blocks for the modified regions.

On btrfs, the sequence is:

1. `reflink` the base ROM to a new path (instant, zero-copy).
2. Apply the patch to the reflinked copy (only modified bytes are written; CoW allocates new blocks only for the changed extents).
3. The result shares 99.9% of its blocks with the base ROM.

For a 32 MB ROM with a 4 KB patch, the patched variant occupies approximately 4 KB of additional disk space (one or two btrfs extents), not 32 MB. With 10 translation patches for the same game, total overhead is approximately 40 KB instead of 320 MB.

On ext4 (no reflinks), this optimisation is not available — each patched variant is a full copy. This is acceptable; the delta update approach is a btrfs bonus, not a requirement.

### Integration With Work Units

A `PatchWork` unit triggers on `ContentInstalled` when the installed content has pending patches in the metadata. It emits `ContentPatched`, which flows back through OrganiseWork to create the appropriate symlinks for the patched variant.

---

## 12. btrfs as the Recommended Filesystem

The research findings strongly favour recommending btrfs over ext4 for saalis users who are willing to reformat their userdata partition. The combined benefits are substantial:

| Capability | ext4 | btrfs |
|-----------|------|-------|
| Reflink install (instant copy) | No (full copy) | Yes (microseconds) |
| Data checksumming (bit rot detection) | No | Yes (crc32c default) |
| Transparent compression (zstd) | No | Yes (30-60% space saving) |
| Delta update efficiency | No (full copy per variant) | Yes (CoW shared blocks) |
| Deduplication of identical content | Hardlinks only | Reflinks (independent copies that share blocks) |
| Content-addressable GC via refcount | No | Yes (`fiemap` extent tracking) |

The performance trade-off (btrfs is slower for sequential writes and SQLite operations) is acceptable because saalis's dominant I/O pattern is large sequential reads (loading ROMs into emulators), not writes. The SQLite write penalty is mitigated by the fire-and-forget write scheduler that batches writes and checkpoints WAL periodically.

Saalis should detect the filesystem type at startup and advertise the available optimisations in the UI: "Your storage is ext4. Installation uses file copies. Reformatting to btrfs enables instant installation, data integrity checking, and compression."

Batocera already supports btrfs as a userdata format and ships btrfs tools. The migration path (backup, reformat, restore) can be documented, though saalis should never perform it automatically.

---

## 13. Pipeline as DAG Work Units

### Extended Event Kinds

The scheduler's event taxonomy from the existing design needs extension for the full acquisition pipeline:

```
// --- Acquisition pipeline events (additions to existing EventKind) ---

/// Sourcer resolved a download/access location (already exists)
SourceResolved,

/// Download completed — file is in staging/ (already exists)
DownloadComplete,

/// Verification passed — BLAKE3 hash confirmed
VerifyComplete,

/// Extraction completed (or skipped for uncompressed files) (already exists)
ExtractionComplete,

/// File installed to content-addressable store
ContentInstalled,

/// Symlinks created in profile library trees
ContentOrganised,

/// A pipeline stage failed — carries stage ID and error classification
PipelineStageFailure,

/// Patch applied to installed content
ContentPatched,
```

### DAG Fragment for Acquisition

```
SourceResolved
    |
    v
DownloadWork
    |
    v
DownloadComplete -----> VerifyWork (only for torrent/manual import)
    |                       |
    | (direct download      v
    |  already verified) VerifyComplete
    |                       |
    +<----------------------+
    |
    v
ExtractWork
    |
    v
ExtractionComplete
    |
    v
InstallWork
    |
    v
ContentInstalled
    |
    +-----------> PatchWork (if patches pending)
    |                 |
    |                 v
    |            ContentPatched
    |                 |
    +<----------------+
    |
    v
OrganiseWork
    |
    v
ContentOrganised (terminal — update UI, fire notification)
```

### Concurrency

DownloadWork is naturally bounded by network bandwidth and download client slots. ExtractWork is CPU-bound (decompression) and should be limited to one concurrent extraction to avoid memory pressure on constrained devices. InstallWork is fast (reflink/hardlink) and can be highly concurrent. OrganiseWork is trivial (symlink creation) and can be batched aggressively.

The scheduler's per-connector concurrency cap and rate-limit token bucket handle download concurrency. A separate resource-based concurrency limit (memory budget) should gate extraction concurrency.

### Progress Reporting

Each work unit reports progress through the scheduler's status tracking, which feeds the web UI via WebSocket:

- DownloadWork: bytes downloaded / total bytes (percentage + speed).
- VerifyWork: bytes hashed / total bytes.
- ExtractWork: bytes decompressed / total decompressed size (if known from archive headers).
- InstallWork: typically instant (reflink) — no progress needed.
- OrganiseWork: symlinks created / total.

The *arr stack pushes these updates via SignalR. Saalis pushes via axum WebSocket, keeping the sync-core/async-shell boundary. The work unit writes progress to a `WorkUnitStatus` struct that the web layer polls (or subscribes to via a channel).

---

## 14. Error Handling and Recovery

### Blocklisting

Following the *arr pattern, when a source repeatedly fails (download errors, checksum mismatches, corrupt archives), the source is blocklisted — saalis will not attempt to download from that source again for that content. The Sourcer can find alternative sources. This is stored as metadata on the entity, not as a global ban.

### Retry With Backoff

Transient failures (network timeouts, rate limiting) enter the scheduler's deferred queue with exponential backoff. The existing `max_retries` constant on WorkUnit handles this. Permanent failures (404, authentication failure) escalate immediately to the notification system.

### Partial Download Resumption

For HTTP sources that support `Range` headers, DownloadWork stores the byte offset of the last successful write. On retry, it resumes from that offset. The BLAKE3 hasher state is not resumable (it is an in-memory structure), so after resumption, a full post-download hash verification is required instead of streaming verification. This is a trade-off: resumable downloads save bandwidth but require a verification pass.

### Crash Recovery

The staging directory may contain partially written files after a crash. On startup, the Housekeeper scans staging for files without a corresponding in-progress download record in the database. These orphans are deleted. The atomic write-fsync-rename pattern ensures that the store never contains partial files — if a crash occurs during InstallWork's copy fallback, the temporary file in the store directory is cleaned up (it has a `.tmp` suffix that the Housekeeper recognises).

---

## 15. Putting It All Together

The content acquisition pipeline combines filesystem primitives, scheduler architecture, and *arr lessons into a coherent system. The key insight is that the pipeline is not a linear sequence of functions but a DAG of independent work units, each of which can be retried, parallelised, and observed independently.

On a btrfs-formatted Batocera device, the happy path for installing a directly-downloaded 4 GB ISO is:

1. **SourceResolved** emitted by Sourcer connector.
2. **DownloadWork** fallocates 4 GB, streams the download while computing BLAKE3.
3. **DownloadComplete** emitted with the BLAKE3 hash (no separate verify step).
4. **ExtractWork** detects the file is already an ISO — emits **ExtractionComplete** immediately.
5. **InstallWork** computes the content-addressed path from the BLAKE3 hash, calls `reflink` from staging to store. Completes in microseconds.
6. **OrganiseWork** creates a symlink from `/userdata/roms/psx/Game.iso` to the store path for each authorised profile.
7. **ContentOrganised** — user sees the game appear in the UI. Total pipeline time: dominated entirely by download speed.

On ext4, the same pipeline replaces the reflink with `std::io::copy` (which uses `copy_file_range`), adding 10-30 seconds for the 4 GB copy. Everything else is identical.

The content-addressable store, profile-specific symlink trees, and filesystem-aware installation strategy combine to give saalis a file management system that is more efficient than the *arr stack's hardlink approach (reflinks are strictly better than hardlinks — they share blocks but are independently deletable) while remaining correct on filesystems that lack CoW support.

The pipeline's integration with the hilavitkutin scheduler (once the WorkUnit trait alignment is resolved) means that every stage is observable, retriable, and composable. Adding a new pipeline stage — say, a `TranscodeWork` for converting between ROM formats — is adding a work unit that triggers on `ExtractionComplete` and emits a new `TranscodeComplete` event. The DAG absorbs it without modification to existing stages.

---

## Appendix: Crate Dependencies for the Pipeline

```
# Filesystem operations
reflink-copy          # CoW file cloning (reflink-first installation)
atomic-write-file     # Crash-safe write-fsync-rename
nix                   # fallocate, posix_fadvise, statfs (filesystem detection)

# Directory scanning
jwalk                 # Parallel directory traversal (startup reconciliation)
notify                # Real-time filesystem watching (runtime updates)

# Checksumming
blake3                # Primary internal hash (streaming + rayon parallel)
crc32fast             # CRC32 for No-Intro/Redump compatibility
md-5                  # MD5 for legacy ROM database compatibility

# Archive extraction
zip                   # ZIP extraction (streaming, random access)
tar                   # tar extraction (streaming)
flate2                # gzip decompression
zstd                  # zstd decompression
sevenz-rust2          # 7z extraction (streaming, LZMA2)
sevenzip-mt           # 7z parallel block decompression (memory-efficient)
unrar                 # RAR extraction (C binding, proprietary format)

# Async compression (for download-and-extract streaming)
async-compression     # Tokio-compatible AsyncRead/AsyncWrite adaptors

# Temporary files
tempfile              # Scratch files during extraction
```

---

## Appendix: Filesystem Detection at Startup

```rust
use nix::sys::statfs::statfs;

const BTRFS_MAGIC: i64 = 0x9123683E;
const EXT4_MAGIC: i64   = 0xEF53;
const XFS_MAGIC: i64    = 0x58465342;

fn detect_fs_capabilities(path: &Path) -> FsCapabilities {
    let fs = statfs(path).expect("statfs failed");
    let fs_type = fs.filesystem_type().0 as i64;

    FsCapabilities {
        reflinks: matches!(fs_type, BTRFS_MAGIC | XFS_MAGIC),
        data_checksums: fs_type == BTRFS_MAGIC,
        compression: fs_type == BTRFS_MAGIC,
        hardlinks: true, // all relevant Linux filesystems
        fallocate: true,  // ext4, btrfs, xfs, f2fs
    }
}
```

This detection runs once at startup and influences the installation strategy, verification frequency, and UI recommendations for the remainder of the session.
