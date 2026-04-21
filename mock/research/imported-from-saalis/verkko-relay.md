# verkko-relay

## Abstract

Relay services for the verkko ecosystem. How peers forward
traffic for unreachable peers, how relay topology is decided,
and how encrypted data is stored at rest on relay nodes.

A relay peer is not a full mesh participant in the content
sense — it forwards frames and stores opaque encrypted blobs
without access to plaintext. The relay is a transit layer and
a storage membrane.

**Attribution (v10).** This design is the product of forty expert
perspectives across eleven review phases. v10 validated by three
independent reviewers (cryptographic protocol specialist, distributed
systems architect, applied mathematician). See v10 document for full
attribution.

## Dependencies

- **verkko-crypto**: StorageAEAD, ChannelKey, EpochRatchet,
  DomainSeparatorRegistry, NonceSafety
- **verkko-protocol**: Frame, Connection, NATTraversal
- **verkko-mesh**: Cluster, Gateway, Convergence

## Defined Concepts

### TransitRelay

Frame forwarding through an intermediary peer when direct
connection is not possible.

Invariants:
- Relay peer forwards frames without decrypting content
  (end-to-end encryption preserved).
- Relay peer sees source/destination connection IDs and
  frame sizes, not plaintext.
- Relay-first connectivity. Transit relay provides immediate
  connectivity (~500ms, one RTT). Direct paths are established
  in background via hole punching. When a direct path is verified
  via PATH_CHALLENGE/PATH_RESPONSE, all traffic migrates from
  relay to direct. The relay session is maintained as a warm
  standby (keepalive probes at 2-second intervals, 3.6
  bytes/second per session). If the direct path fails, traffic
  falls back to the relay path immediately.

### RelayTopology

Which peers relay for which, and how relay paths are selected.

Invariants:
- Relay selection based on connectivity graph and health.
- A peer that cannot directly reach another peer routes
  through the best available relay.
- Relay topology adapts when peers join, leave, or change
  network conditions.

### Membrane

Encrypted-at-rest storage on relay peers. The relay holds
ciphertext without access to plaintext.

Invariants:
- All data at rest on relay peers is encrypted under channel
  epoch keys.
- Relay peer cannot read, modify, or selectively delete
  content without detection.
- Relay key ID is salted and rotated (90-day TTL, 30-day
  TOUCH interval).

### TerritorialReencryption

Consistent hash ring assigning matter ownership to peers.
On key revocation, owners re-encrypt their territory.

Invariants:
- Hash ring with 32 vnodes per peer, ring coloring constraint
  (no same-cluster adjacent vnodes).
- Re-encryption uses deterministic nonces: identical inputs
  produce identical ciphertext (idempotent relay PUT).
- Tiered deadlines: P1 (4h), P2 (24h), P3 (bandwidth-
  derived), P4 (epoch-forward, no re-encryption).

### IdempotentPUT

Relay storage write that is safe to repeat.

Invariants:
- If relay_key_id already exists, PUT is a no-op.
- Content comparison unnecessary because deterministic
  nonces guarantee identical ciphertext from identical inputs.

### ConstantRatePadding

Traffic analysis defense. Relay operations are padded to
a constant rate.

Invariants:
- Batch size fixed. Real operations + decoy operations = constant.
- Observer learns batch count but not real operation count
  (within a batch).

### RelayCorrelationThreatModel

What the relay can and cannot learn from traffic patterns.

Invariants:
- Relay sees: connection IDs, frame sizes, timing.
- Relay does not see: plaintext, sender/receiver identity
  beyond connection ID, content type.
- Correlation attacks (timing, volume) are within threat
  model but bounded by constant-rate padding and salted
  key rotation.

## Body

### Transit Forwarding

The relay maintains a forwarding table:

```
ForwardingTable {
    entries: HashMap<(u32, u32), ForwardingEntry>,  // (destination, connection_id)
    max_entries: u16,  // default 256, configurable
}

ForwardingEntry {
    next_hop: SocketAddr,
    created_at: Instant,
    last_activity: Instant,
    bytes_forwarded: u64,
    frames_forwarded: u64,
    ttl: Duration,  // default 5 minutes, refreshed on activity
}
```

Per-frame forwarding procedure:

1. Parse outer header (29 bytes). No AEAD needed.
2. TTL check: if hop_count == 0, drop.
3. Loop detection: if destination matches self, process locally.
4. Lookup forwarding table by (destination, connection_id).
5. If found: decrement hop_count (byte 8), update accounting, forward
   to next_hop.
6. If not found: if SIGNAL frame, handle it; otherwise drop.

Cost per forwarded frame: ~300ns (hash lookup + memcpy + byte
decrement). At 10,000 frames/second: 3ms/second, 0.3% of one ARM
Cortex-A72 core. Memory: 256 entries * ~100 bytes = ~25 KB.

### RelayForwarder Trait

The relay's forwarding function is separate from the Protocol trait.
The relay does not decrypt, does not parse inner payloads, does not
maintain stream state, does not generate SACKs for forwarded traffic.

```rust
trait RelayForwarder {
    fn forward(&mut self, frame: &[u8], source: PeerAddr) -> ForwardAction;
    fn handle_signal(&mut self, signal: &SignalMessage, source: PeerAddr)
        -> SignalResponse;
    fn poll_expiry(&self) -> Option<u64>;
    fn expire(&mut self, now_ms: u64);
}

enum ForwardAction {
    Send { destination: PeerAddr },
    ProcessLocally,
    Drop,
}
```

The relay peer runs BOTH the Protocol trait (for its own Noise sessions
with mesh peers, enrollment, SIGNAL messages, heartbeat reception) and
the RelayForwarder trait (for forwarding other peers' frames). The
consumer first checks if a received datagram is addressed to the relay
itself; if so, it feeds it to the Protocol state machine. If not, it
feeds it to the RelayForwarder.

Buffer ownership: the frame is caller-owned. The forwarder reads the
outer header, modifies hop_count in-place, returns ForwardAction.
No arena allocation. Zero allocation per forwarded frame.

### Relay Identity

The relay is a pseudo-peer with a DK/DEK pair but no CIK (it does not
belong to a cluster). The relay authenticates to the mesh via a
relay-specific enrollment process:

1. Admin generates an invite token with token_type = 0x04 (RELAY).
2. Relay generates DK, DEK, connects via Noise XX to the mesh.
3. Admin signs a relay enrollment cert (distinct from peer enrollment).
4. Relay receives a relay-specific control channel key (read-only for
   heartbeats, revocations, and convergence signals).

The relay does not participate in gossip, dominance, convergence, or
health scoring. The relay receives a subset of heartbeat information
sufficient to maintain its forwarding table and TTL timers.

The pseudo-peer model saves ~0.15% CPU steady-state (no gossip
processing, no health pipeline, no dominance computation). The value
is specification simplicity, not performance.

Relay hash ring territory: the relay does not participate in the
consistent hash ring for territorial re-encryption. The relay's
territory (if it holds membrane replicas) is delegated to the nearest
non-relay peer on the hash ring.

Deployment constraint: relay peers must have at least 2 CPU cores.
Single-core devices should not serve as relay peers (DH stall during
initial join: 216ms on single-core, blocking the event loop).

### Relay Resource Governance

Forwarding table limits: max 256 entries (configurable). When full,
new SIGNAL HELLO requests receive ERROR with code RELAY_FULL. The peer
should try another relay.

Bandwidth limits per forwarded session:

```
relay_forwarding_budget = min(
    available_bw_kbps * 0.5,
    sum(active_sessions) * per_session_cap_kbps
)
per_session_cap_kbps = relay_forwarding_budget / active_sessions
```

At 10 Mbps symmetric and 10 active sessions: 500 kbps per session.

Amplification prevention: a relay MUST NOT forward a frame to an
address from which it has not received a recent authenticated frame.
The window is 2x the current heartbeat interval of the forwarded
session (not a fixed 30 seconds). This prevents the relay from being
used as a DDoS reflector while accommodating Background mode heartbeat
intervals.

### Relay Failover

When a peer registers with a relay via SIGNAL HELLO, it also registers
with a secondary relay (the next-best candidate from relay selection).
The secondary relay holds a dormant forwarding entry.

Failover procedure:

1. Relay heartbeat timeout (3 missed keepalive probes at 70% of
   mapping lifetime).
2. Promote secondary relay: activate forwarding entries.
3. Notify remote endpoint via secondary relay: SIGNAL REDIRECT.
4. Select new secondary relay, register via SIGNAL HELLO.

Failover timing: 1 heartbeat timeout (2-30 seconds depending on
metabolic state) + 1 RTT for redirect (~200ms). Total: 2.2-30.2
seconds.

Relay keepalive probes: 2-second interval regardless of metabolic
state. Cost: 2 * 91 bytes/s = 182 bytes/s per relayed session = 3.6
bytes/s as warm standby.

### Relay Health Normalization

The relay path health observed by a WiFi peer includes both the relay's
intrinsic health and the WiFi noise on the first hop. To separate
these:

```
relay_health_normalized = raw_relay_sack_q16 / wifi_quality_q16
```

Clamped to [0, 0x10000]. If wifi_quality is 0, relay health is 0.

Without normalization, a relay with 2% true loss appears to have 17%
loss at 15% WiFi loss. With normalization, the relay's intrinsic health
reads 0.976. False relay failover rate at 15% WiFi loss drops from
~1/hour (without normalization) to effectively zero (with).

Cost: one Q16.16 division per heartbeat tick per relay path. ~20ns on
ARM Cortex-A53. At 11 connections: 220ns per tick. Negligible.

### Three-Tier Discovery Chain

1. **Direct:** address_hints from invite token. Works for LAN peers
   and peers with endpoint-independent NAT mapping.
2. **STUN + hole-punch:** stun_server_hints in invite token (new field,
   +16 bytes per hint). Peer discovers server-reflexive address via
   OBSERVED_ADDR in Noise handshake. Hole-punch via SIGNAL PUNCH
   through relay. Works for ~60% of NAT types.
3. **Cloud relay:** relay_address_hint (new invite token field) or
   relay_dht_key (existing) or community relay (opaque forwarding,
   rate-limited: 100 kbps per pair, 1 Mbps aggregate). Fallback for
   symmetric NAT.

Community relay is an optional deployment component. The protocol
functions without it if at least one non-symmetric-NAT peer exists in
the mesh.

<!-- v10 §4.3 -->
### 4.3 Relay (membrane model)

Content-addressed blob store with dual-interface membrane. Five ops: PUT, GET, DELETE, TOUCH (extend TTL), SIGNAL (connection setup). Default TTL: 90 days, TOUCH every 30 days.

### Relay Operations Wire Format

Relay operation header (13 bytes):
    Offset  Width  Field
    ------  -----  -----
     0      1      op_code
     1      1      flags (bit 0=is_decoy, bit 1=is_batch)
     2      2      request_id: u16 (correlation, monotonic)
     4      8      relay_key_id: HMAC-BLAKE3(channel_salt, key_id)[0..8]
    12      1      reserved

Operation codes:
    0x01 PUT    0x81 PUT_ACK
    0x02 GET    0x82 GET_RESP
    0x03 DELETE 0x83 DELETE_ACK
    0x04 TOUCH  0x84 TOUCH_ACK
    0x05 SIGNAL 0x85 SIGNAL_ACK
    0xFF ERROR

Response codes: 0x80 | op_code.

PUT: idempotent. relay_key_id collision = no-op (ALREADY_EXISTS).
GET: batch mode via is_batch flag. Decoy operations use is_decoy.
SIGNAL: types 0x0001=HELLO, 0x0002=PUNCH, 0x0003=REDIRECT.

### SIGNAL HELLO (0x0001)

Sent by a peer to a relay to register itself as reachable via that
relay. The peer establishes a Noise KK session with the relay, then
sends SIGNAL HELLO.

```
SIGNAL_HELLO {
    signal_type: u16 = 0x0001,
    cluster_destination_id: u32,
    keepalive_interval: u16,  // seconds, 0 = relay decides
    reserved: [u8; 2],
}
// 10 bytes payload
```

On receipt, the relay creates a forwarding table entry with a wildcard
connection_id (0xFFFFFFFF): "any connection to this destination routes
through this relay." Specific entries are created when traffic arrives.

### SIGNAL PUNCH (0x0002)

Relay-assisted hole punching. Adapted from RFC 8445 (ICE) candidate
exchange.

```
SIGNAL_PUNCH {
    signal_type: u16 = 0x0002,
    target_destination_id: u32,
    candidate_count: u8,
    reserved: [u8; 1],
    candidates: [Candidate; candidate_count],
}

Candidate {
    candidate_type: u8,    // 0x01=host, 0x02=srflx, 0x03=relay
    addr_type: u8,         // 0x04=IPv4, 0x06=IPv6
    port: u16,
    address: [u8; 16],
    priority: u32,         // ICE priority (RFC 8445 Section 5.1.2)
}
// 24 bytes per candidate
```

### SIGNAL REDIRECT (0x0003)

Relay tells a peer to use a different relay. Used during relay failover.

```
SIGNAL_REDIRECT {
    signal_type: u16 = 0x0003,
    new_relay_destination_id: u32,
    new_relay_address: [u8; 20],
    reason: u8,  // 0x01=overloaded, 0x02=shutting_down
    reserved: [u8; 1],
}
// 28 bytes payload
```

Constant-rate padding:
    batch_size = HKDF-Expand(epoch_key, "relay:batch_size:" || epoch, 2)
                 as u16 % 16 + 10
    Range: [10, 25]. Changes each macro-epoch.

**Idempotent PUT contract.** PUT is idempotent. If relay_key_id already exists, the PUT is a no-op. Content comparison is unnecessary because deterministic nonces (see verkko-crypto: StorageAEAD) guarantee identical ciphertext from identical inputs.

**Batched GET with decoys:** `BatchGET { relay_key_ids, fake_ids }`. 10 real + randomized decoys.

**Trust boundary.** Relay sees opaque blobs and cluster IDs. Cannot infer friendship, content, channels, topology.

<!-- v10 §4.4 -->
### 4.4 Territorial re-encryption and donation

**Territorial partitioning.** relay_key_id space partitioned via consistent hashing [Karger1997] on DK pubkeys. No coordinator. 32 vnodes/peer max. Capacity smoothing: EMA (alpha=0.05, verkko-ops: Pattern Registry Structure 2 instance), ring recomputed on > 20% change. Capacity: `min(bw_kbps/128, flash_kb/256, 255)`.

**Ring coloring constraint.** No two adjacent vnodes on the hash ring belong to the same cluster. Enforced by a single O(N) greedy reassignment pass after any membership change. See verkko-ops: Pattern Registry, Structure 7.

**Ring coloring algorithm:**
1. Sort vnodes by hash position (deterministic, O(N log N)).
2. Scan left-to-right. For each adjacent same-cluster pair (v_i, v_{i+1}):
   a. Find the nearest non-adjacent position for v_{i+1} (scanning right).
   b. Swap v_{i+1} to that position.
   c. Continue scan from the new position of v_{i+1}.

**Termination proof:** Potential PHI = number of adjacent same-cluster pairs. Each swap reduces PHI by at least 1 (original pair broken). A swap may create at most 1 new pair. PHI decreases by at least 0 per swap, and the scan advances right. Bounded by N positions. Terminates in at most N steps. Total work: at most 2N comparisons.

**Expected swaps derivation.** At 12 clusters with 96 positions per cluster on a 1,152-position ring, the expected number of adjacent same-cluster pairs is `E = 12 * C(96, 2) / C(1152, 2) * 1152 ~ 12 * 96 * 95 / 1151 ~ 95.08`. Greedy cascade adds ~8.5 swaps for total ~103.5. The algorithm is O(N) with ~100 swaps.

**Continuous enforcement.** The ring coloring invariant is checked on every heartbeat cycle at ~5us cost (O(N) scan for adjacent same-cluster pairs). This replaces the 20% capacity change threshold heuristic. If any violation found, trigger reassignment pass.

**Count-balanced, not capacity-weighted.** The hash ring assigns equal vnode counts per peer regardless of WiFi capacity. LEDBAT pacing handles capacity variance. Capacity-weighted vnodes were rejected: they couple a long-lived data structure to transient link quality.

**Retain-until-handoff.** A peer retains ownership of its territorial partition until it explicitly hands off to the new owner.

**Deterministic re-encryption nonces:**
```
re_enc_nonce = BLAKE3("saalis-reenc-v1" || channel_key_new || blob_id || re_encryption_epoch)[0..24]
```

Properties: uniqueness per triple (PRF assumption on BLAKE3), domain separation from transport nonces (different keys), idempotence (identical inputs produce identical ciphertext). Precondition: `channel_key_new` must be identical on all re-encrypting peers (guaranteed for revocation-injected fresh keys).

**Priority:** (1) security exposure, (2) durability risk, (3) remaining. Metabolic pacing via LEDBAT SM.

**Speculative P1 re-encryption.** During convergence (see verkko-mesh: Convergence), gateway peer speculatively re-encrypts P1 blobs immediately after Front 1, concurrent with the polygon schedule. Airtime-capped at `max(1 second, 5% of remaining convergence window airtime)`. At 37-second convergence: ~1.85 seconds. Deterministic nonces make redundant relay PUTs idempotent.

**Tiered deadlines:**

| Priority | Type | Deadline |
|----------|------|----------|
| 1 (Critical) | Key material, control log, credentials | 4 hours |
| 2 (High) | Metadata, ratings, personal data | 24 hours |
| 3 (Normal) | Content chunks, media assets | Bandwidth-derived |
| 4 (Low) | Cached thumbnails, previews | Epoch-forward (no re-encryption) |

**Symbiotic donation.** Surplus peers absorb neighbors' work. No coordination. Idempotent (deterministic nonces). Pace line: 10% behind expected pace. Cascade prevention: `max_absorption = (capacity - own_cost) / per_blob_cost`. SD cards: donation disabled.

<!-- v10 §4.5 -->
### 4.5 Constant-rate relay padding

Relay GET/PUT operations leak aggregate volume information. Constant-rate padding normalizes traffic patterns.

**Mechanism.** Relay traffic is batched into fixed-interval windows. Each window sends a constant number of operations (real + decoy). batch_size constant within an epoch (derived from HKDF of epoch key). Between epochs, batch_size changes.

**Leakage bound.** log2(C(batch_size, real_count)) bits per batch about the real operation count.

**Cost:** ~50 bytes per decoy. At 5 decoys per batch, 10 batches/hour: ~2,500 bytes/hour.

<!-- moved from verkko-mesh §7.1, Front 3 re-encryption details -->
### Convergence re-encryption

When the mesh signals Front 3 (see verkko-mesh: Convergence), the relay performs bulk re-encryption under new keys.

**Flash budget moderation.** Re-encryption batches consume flash budget (see verkko-mesh: ResourceGovernance). Flash budget moderation applies: diminuendo curve, P1 exempt.

**Speculative P1 re-encryption.** During convergence, gateway peer speculatively re-encrypts P1 blobs immediately after Front 1, concurrent with the polygon schedule. Airtime-capped at `max(1 second, 5% of remaining convergence window airtime)`. At 37-second convergence: ~1.85 seconds. Deterministic nonces make redundant relay PUTs idempotent.

**Nonce velocity monitoring.** Pre-batch double-write persist before each re-encryption batch (see verkko-crypto: NonceSafety).

**Gate.** All affected content re-encrypted or epoch-forwarded. Blocked until: Consensus gate open AND vital queue depth < 25%.

<!-- v10 §8 — re-encryption costs -->
### Cost summary (re-encryption)

| Operation | Total |
|-----------|-------|
| Ring coloring check | ~5us/heartbeat |
| CPSK re-seed HKDF | ~1.25us |
| Constant-rate padding | ~0.7 B/s |

## References

[Karger1997] Karger, D., Lehman, E., Leighton, T., Panigrahy, R., Levine, M., Lewin, D. (1997). "Consistent Hashing and Random Trees." STOC 1997.
