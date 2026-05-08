# verkko-matter

## Abstract

Distributed tracking, transfer, and replication of opaque
addressable data across the verkko mesh. How the mesh knows
what exists, where it is, how to get it, and how to keep it
available.

Matter is the reason the mesh exists. Without matter, the mesh
is pointless infrastructure. Without the mesh, matter has
nowhere to live.

This document does not care what matter contains. A 2GB ROM,
a 50KB metadata archive, a database record — all are matter.
Implementors define the semantics. verkko-matter defines the
distribution mechanics.

**Attribution (v10).** This design is the product of forty expert
perspectives across eleven review phases. v10 validated by three
independent reviewers (cryptographic protocol specialist, distributed
systems architect, applied mathematician). See v10 document for full
attribution.

## Dependencies

- **verkko-crypto**: StorageAEAD, DomainSeparatorRegistry,
  EpochRatchet, ChannelKey (for content addressing via BLAKE3,
  SealedRange key operations)
- **verkko-protocol**: Stream, LEDBAT, Connection (for matter
  transfer over bulk tier with background congestion control)
- **verkko-mesh**: Cluster, Gateway, Gossip, HealthPipeline,
  Convergence, Channel (for source selection, advertisement,
  and convergence sequencing)
- **verkko-relay**: Membrane, TerritorialReencryption,
  IdempotentPUT (for at-rest storage and re-encryption)

## Defined Concepts

### Matter

A content-addressed, opaque, integrity-verified unit tracked
by the mesh.

Invariants:
- matter_id = BLAKE3(plaintext). Identity IS the content hash.
- The mesh never inspects matter contents.
- Matter is immutable: same matter_id always refers to the same
  bytes.
- Implementors define what matter represents. verkko does not
  distinguish between types of matter.

### MatterIntegrity

Verified streaming for matter transfer. Bao tree (BLAKE3
verified streaming) for incremental verification.

Invariants:
- Receiver can verify each chunk independently during transfer
  (no need to receive entire blob before verification).
- Integrity failure at any point aborts the transfer and
  triggers a scar on the source.

### ContentFilter

Counting cuckoo filter advertising which matter a cluster holds.
Shared via gossip.

Invariants:
- 10-bit fingerprints, 2-bit saturating counters, 4-entry
  buckets, 90% load factor.
- Supports deletion (saturating counters).
- False positive rate ~0.78%.
- Filter root (keyed Merkle) included in every heartbeat for
  divergence detection.

### MerkleReconciliation

Incremental reconciliation of content filters between peers
after reconnection.

Invariants:
- Only divergent subtrees are exchanged.
- Typical divergence after 4-hour partition: ~20 KB per pair
  (vs ~162 KB full filter sync).
- Depth-8 Merkle tree cache per filter.

### MatterTransfer

Protocol for moving matter between peers. Uses verkko-protocol's
Stream abstraction with bulk tier priority (stream_id 0x1000-0xFFFE)
and LEDBAT pacing for background congestion control.

Invariants:
- LAN path: direct streaming with BLAKE3 verification at end.
- WAN path: bao-tree verified streaming (chunk-level integrity).
- Multi-source: sequential provider fallback.
- Mesh-first: local peer -> mesh peers -> external source.
  External sources are last resort.
- All matter transfers use Stream (bulk tier). LEDBAT pacing
  ensures matter traffic never degrades foreground use.
- BandwidthPolicy sets the ceiling; LEDBAT provides dynamic
  congestion control.

### Stash

Per-peer tracking of locally held matter.

Invariants:
- Maps matter_id to local filesystem path, size, last
  verified timestamp, and status.
- Status: Present, Downloading, Missing, Corrupt.
- Stash is local state, not replicated. Advertised to mesh
  via ContentFilter.

### Replication

Maintaining redundant copies of matter across peers.

Invariants:
- Configurable min_replicas and target_replicas per matter
  priority.
- Background replication work maintains targets.
- Replication IS the backup (no separate backup mechanism).
- Relay peers participate in replication (encrypted at rest
  via Membrane).

### Replication Policy Distribution

Replication targets (min_replicas, target_replicas) are per-channel
configuration distributed via control log entry CHANNEL_CONFIG (type
0x0F):

```
ChannelConfigPayload {
    channel_index: u16,
    min_replicas: u8,
    target_replicas: u8,
    replication_scope: ReplicationScope,  // PerCluster | MeshWide
    bandwidth_policy: u8,
}
```

Subject to fencing token ordering: the dominant admin's configuration
wins after convergence. During grace window, only changes that increase
min_replicas are accepted (safety over liveness).

### SyncReconciliation

Divergence detection and resolution for mesh-wide state.

Invariants:
- Divergence detected via Merkle root comparison in heartbeats.
- Delta sync for incremental changes.
- Merge rules: field-level, deterministic, reproducible on
  all peers.
- Conflict resolution is deterministic: given the same inputs,
  all peers produce the same result.

### SealedRange

Catch-up mechanism for peers that were offline during key
changes.

Invariants:
- Three states: fast-forward, active-with-sealed-range,
  suspended.
- Sealed range content is inaccessible until UnsealGrant.
- SealedReason enum: Absence vs Revocation (different unseal
  paths).
- Current-epoch traffic always decryptable immediately on
  reconnect.

### BandwidthPolicy

Inter-cluster time-based bandwidth rules.

Invariants:
- Policies published via gossip (time windows, rate limits).
- Per-cluster rules (max request rates, blocked windows).
- LEDBAT (see verkko-protocol: LEDBAT) enforces at transport
  level; policies set the ceiling.
- Matter transfers governed by the intersection of
  BandwidthPolicy ceilings and LEDBAT congestion control.

### MatterChunk

Bridge between multi-gigabyte blobs and 1232-byte UDP payloads.
Bao verified streaming with 1024-byte chunks (bao standard
default; non-negotiable in v1).

Invariants:
- Each chunk independently verifiable via bao proof.
- Chunk header (52 bytes) carries transfer_id, blob_id,
  chunk_index, total_chunks, and length fields.
- Chunks that span multiple frames use stream-layer reassembly.

### TransferSession

Stateful context for a single matter transfer between two peers.

Invariants:
- Identified by transfer_id (u64, CSPRNG-generated by requester).
- Lifecycle: TransferRequest -> TransferBegin -> chunk streaming
  -> completion or TransferAbort.
- Multi-source: requester opens separate sessions with different
  sources, each with a unique transfer_id.
- have_bitmap communicates partial state for resumption.

### BlobId

Content identity. BLAKE3 hash of plaintext content.

Invariants:
- `blob_id = BLAKE3(plaintext_content)`.
- BlobId IS the content's identity; immutable.
- BlobId survives re-encryption (re-encryption changes the
  ciphertext but the plaintext, and thus blob_id, is unchanged).
- Used as input to re-encryption nonce derivation (see
  verkko-relay: TerritorialReencryption).
- Cuckoo filter fingerprints derive from blob_id.

## Body

<!-- v10 §2.8 -->
### 2.8 Sealed-range catch-up (three-state model)

Replaces the binary suspended/active gate with a three-state model that provides immediate current-epoch access while sealing historical content.

```
on_reconnect(peer, last_known_epoch):
    if current_epoch - last_known_epoch <= MICRO_CATCH_UP_WINDOW:
        peer.fast_forward(last_known_epoch, current_epoch)
        peer.state = Active

    elif current_epoch - last_known_epoch <= MAX_OFFLINE_EPOCHS:
        peer.fast_forward(last_known_epoch, current_epoch)
        peer.state = Active
        peer.sealed_range = SealedRange {
            start: last_known_epoch,
            end: current_epoch,
            reason: SealedReason::Absence,
        }

    else:
        peer.state = Suspended
        send_catch_up_request(peer)
```

**MICRO_CATCH_UP_WINDOW:** 2 macro-epochs (2 hours). **MAX_OFFLINE_EPOCHS:** 24 macro-epochs (24 hours).

**SealedReason enum:**
```rust
enum SealedReason {
    Absence,    // HKDF chain intact, policy-enforced
    Revocation, // HKDF chain broken, crypto-enforced
}
```

For `Absence` sealed ranges, admin can issue UnsealGrant without re-verifying identity (keys are valid; the seal is a precaution). For `Revocation` sealed ranges, CatchUpGrant with the fresh key implicitly unseals; no separate UnsealGrant needed.

**CatchUpGrant specification.** Carries the current macro-epoch key and micro-epoch counter for each authorized channel. Enters the pending buffer as KeyBundle(Grant) (see verkko-mesh: KeyBundle). The key lifecycle applies identically.

**Cost:**
- Wire: 8 bytes per content routing request (epoch field, already present).
- Memory: 17 bytes per peer for sealed_range (16 + SealedReason u8, aligned to 16).

<!-- v10 §3.7 -->
### 3.7 Incremental filter reconciliation

On reconnection, filter state is reconciled via Merkle tree [Merkle1988] walk. This is a form of anti-entropy reconciliation [Demers1987]: only divergent subtrees are exchanged. Typical divergence after a 4-hour partition: ~20 KB per cluster pair. At 36 reconnecting pairs: ~720 KB total.

Full-filter-sync remains as a fallback for large divergence (> 50% of filter changed) or first contact. Paced at 128 KB/s.

<!-- v10 §4.2 -->
### 4.2 Cuckoo filters, content routing, and content integrity

**Counting cuckoo filters** [Fan2014]**.** 10-bit fingerprints + 2-bit saturating counters, 4-entry buckets, 90% load. 12-bit slots. 100K entities: ~162KB/filter. 11 filters in managed arena.

**Staleness:** 8-byte keyed truncated Merkle root in heartbeat (see verkko-protocol: Heartbeat). 32-byte full root every 10 minutes. Incremental tree: depth-8 cache (16.4KB/filter, 180KB total), dirty-bit optimization.

**Source selection:** `possession AND access AND reachability AND health(effective_score) AND NOT assembling(raptor)`. Tiebreaker: relationship age (control log HLC).

**Content integrity.** `blob_id = BLAKE3(plaintext_content)`. The blob_id is the content's identity. Cuckoo filter fingerprints derive from blob_id; if blob_id is defined, the filter binding follows by construction. Re-encryption nonce derivation uses blob_id as input (see verkko-relay: TerritorialReencryption); the re-encryption invariant is preserved because blob_id survives re-encryption by construction.

**Streaming verification requirement.** Content exceeding 256 KB must support incremental verification against blob_id during transfer, without buffering the complete blob. This is a protocol-level requirement: peers can reject corrupted content after the first invalid chunk rather than buffering the entire blob. The choice of streaming verification algorithm is an implementation decision.

**IntegrityFailure scar.** `ScarAttribution::IntegrityFailure { source: PeerId, blob_id: [u8; 32] }` added to the ScarAttribution enum (see verkko-mesh: Scar). Fires when streaming verification detects content that does not match its blob_id. The scar is attributed to the source peer that provided the content.

<!-- v10 §8 — filter costs -->
### Cost summary (filter and content routing)

| Operation | Total |
|-----------|-------|
| Filter updates | ~18 B/s (active) |
| Deferred reconciliation | ~256us |
| Filter reconciliation (convergence) | ~720 KB (36 pairs * 20 KB) |

### MatterChunkProtocol

Bridge between multi-gigabyte blobs and 1232-byte UDP payloads.
Uses bao verified streaming with 1024-byte chunks (bao standard
default). Chunk size is fixed at 1024 bytes in v1. The
TransferBegin chunk_size field exists for future negotiation
but MUST be 1024 in protocol version 1.

Chunk frame (msg_type = 0x04, 52-byte header):

    Offset  Width  Field
    ------  -----  -----
     0      8      transfer_id: u64 (identifies transfer session)
     8     32      blob_id: [u8; 32] (BLAKE3 of complete plaintext)
    40      4      chunk_index: u32 (0-based bao tree position)
    44      4      total_chunks: u32
    48      2      chunk_data_len: u16
    50      2      proof_len: u16
    52      var    chunk_data
     ?      var    bao_proof

Chunk header appears once per chunk, not once per frame. Chunks
that span multiple frames use stream-layer reassembly.

Frame budget:
    Available per first frame: 1118 bytes (chunk_data + bao_proof)
    Available per continuation frame: 1170 bytes
    Worst case (2GB blob): 2 frames per chunk

TransferRequest (msg_type = 0x0D, 44-byte header):
    [8]  transfer_id (CSPRNG-generated by requester)
    [32] blob_id
    [4]  have_bitmap_len (u32, number of u64 words)
    [var] have_bitmap (bitfield of already-held chunks)

TransferBegin (msg_type = 0x0E, 88-byte header):
    [8]  transfer_id (echoed)
    [32] blob_id (echoed)
    [4]  total_chunks
    [8]  blob_size (u64, total bytes)
    [32] bao_root (root hash for verification)
    [2]  chunk_size (u16, typically 1024)
    [2]  reserved

TransferAbort (msg_type = 0x0F, 12-byte header):
    [8]  transfer_id
    [1]  reason (0x00=COMPLETED, 0x01=INTEGRITY_FAILURE,
         0x02=SOURCE_GONE, 0x03=CANCELLED, 0x04=STORAGE_FULL,
         0x05=RATE_LIMITED, 0x06=CONN_CLOSING, 0xFF=UNSPECIFIED)
    [1]  reserved
    [2]  detail_len

Transfer lifecycle:
1. Requester sends TransferRequest with blob_id and have_bitmap.
2. Source replies with TransferBegin (total_chunks, bao_root).
3. Source streams chunks on bulk-tier stream (LEDBAT-paced).
4. Requester verifies each chunk against bao_proof.
5. On integrity failure: abort, scar the source.
6. On completion: verify blob_id = BLAKE3(reassembled plaintext).

Control messages use reactive tier (0x0100-0x0FFF). Chunk data
uses bulk tier (0x1000-0xFFFE).

Multi-source: requester opens separate transfer sessions with
different sources. Each has a unique transfer_id. The have_bitmap
communicates partial state for resumption.

## References

[Demers1987] Demers, A., Greene, D., Hauser, C., Irish, W., Larson, J., Shenker, S., Sturgis, H., Swinehart, D., Terry, D. (1987). "Epidemic Algorithms for Replicated Database Maintenance." PODC 1987.

[Fan2014] Fan, B., Andersen, D.G., Kaminsky, M., Mitzenmacher, M. (2014). "Cuckoo Filter: Practically Better Than Bloom." CoNEXT 2014.

[Merkle1988] Merkle, R. (1988). "A Digital Signature Based on a Conventional Encryption Function." CRYPTO 1987.
