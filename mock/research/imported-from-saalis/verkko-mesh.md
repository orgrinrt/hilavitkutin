# verkko-mesh

## Abstract

Multi-peer trust network for the verkko ecosystem. How peers
form clusters, establish trust hierarchies, distribute keys,
monitor health, disseminate information, elect gateways, and
recover from partitions.

Everything here concerns the collective: more than two peers
coordinating. This document composes verkko-protocol's
point-to-point primitives into a mesh with administration,
health, and convergence.

**Attribution (v10).** This design is the product of forty expert
perspectives across eleven review phases. v10 validated by three
independent reviewers (cryptographic protocol specialist, distributed
systems architect, applied mathematician). See v10 document for full
attribution.

## Dependencies

- **verkko-crypto**: DK, DEK, ChannelKey, EpochRatchet,
  BroadcastEncryption, PCSMechanism, DomainSeparatorRegistry,
  FoundingHash
- **verkko-protocol**: Connection, Heartbeat, Session, Frame,
  VitalQueue, MetabolicState, SACK

## Defined Concepts

### Cluster

A group of 1-6 devices on the same LAN, sharing a cluster
identity key. A household.

Invariants:
- All peers within a cluster trust each other fully.
- Inter-cluster trust requires verification (SAS or invite).
- Maximum 12 clusters in a mesh (pairwise CPSK scaling).

### CIK (Cluster Identity Key)

X25519 key shared among all peers in a cluster. Used as the
Noise static key for inter-cluster sessions.

Invariants:
- CIK is replicated to all cluster peers, encrypted to each
  peer's DEK.
- Compromise of any cluster peer compromises the CIK.
- CIK rotates via signed control log entry (48-hour dual
  acceptance window).

### CPSK (Cluster-Pair Session Key)

Output of a Noise KK handshake between two clusters' gateways.
Per-pair, per-session.

Invariants:
- One CPSK per cluster pair per session.
- CPSK renegotiation on every reconnection, 24-hour timeout,
  and Noise session expiry.
- Fresh CPSK feeds PCSMechanism for epoch key re-seeding.

### Enrollment

The process by which a new device joins a cluster and becomes
a peer in the mesh.

Invariants:
- Enrollment requires admin authorization (enrollment cert
  signed by admin DK).
- Enrollment binding prevents Sybil attacks (pre-computed keys
  useless without enrollment cert).
- SAS verification required before channel keys are delivered.

### EDT (Enrollment Delegation Token)

Signed token allowing a non-admin peer to enroll new devices.

Invariants:
- Max 3 enrollments per EDT, max 7 days validity.
- EDT-enrolled devices enter Active state but channel keys
  are withheld until SAS verification completes.

### KeyBundle

Atomic key distribution message from admin to all peers.
Contains channel keys, fencing token, and admin signature.

Invariants:
- Three operations: Grant, Rotate, Revoke.
- Conflict resolution: Revoke > Grant > Rotate.
- Fencing token provides ordering within one admin's stream.
- Grace window (10 min after partition heal): only Revoke
  accepted from stale fencing tokens.

### DominanceCascade

Deterministic admin succession without consensus or election.
Pure function of shared state.

Invariants:
- Admin = argmin of dominance score across eligible peers.
- Dominance score = BLAKE3(DK || epoch) * (1 + scars) *
  2^max(0, scars - 10). Higher score = less dominant.
- All peers with the same GSet state and epoch compute the
  same admin. No messages needed.
- Enrollment-gated: only enrolled peers are eligible.

### HealthPipeline

Three-tap observation pipeline: classification, scar timing,
circuit breaker.

Invariants:
- Inputs validated by PhysicalBoundary gates (NaN guard,
  range checks, sign constraints).
- Dual EMA (fast alpha=0.1, slow alpha=0.01), time-normalized.
- Effective health = min(fast, slow). Asymmetric: fast
  degradation (~10 obs), slow recovery (~100 obs).
- Per-observer: each peer computes health from its own
  observations only.

### Scar

A permanent record of observed poor behavior by a peer.

Invariants:
- Scars are append-only (GSet). Never removed.
- Scar weight decays over 180 days (read-time projection,
  not mutation).
- Scars affect dominance score and health score.
- Scar provenance tracked (source cluster, transit path hash)
  for correlated-suspicion analysis.

### Scar Classification

Scars carry a ScarAttribution variant that determines the decay period.
The classification is a CRDT-compatible payload extension: the GSet
grows monotonically; merge (set union) is unaffected by payload
complexity; the decay function is a pure function of (scar,
current_time).

Three scar layers:

**Layer 1: Thermal (WiFi-induced).** Decay: 1 + (1 - wifi_quality) * 6
days. Determined by the diagnostic triangle's PathDiagnosis at scar
creation time. The wifi_quality_at_observation is stored in the scar
entry (observation-time state, not query-time local state). All peers
compute the same decay from the same stored value.

```
ScarAttribution::WifiInduced {
    observer: DK,
    wifi_quality_at_observation: u16,  // Q8.8
}
```

Corrected decay formula (Q8.8 fixed-point):

```
fn wifi_scar_decay_days(wifi_quality_q8_8: u16) -> u16 {
    let one_minus_q = 0x0100u16.saturating_sub(wifi_quality_q8_8);
    let days_q8_8 = 0x0100u16.saturating_add(
        one_minus_q.saturating_mul(6));
    days_q8_8 >> 8
}
```

At wifi_quality 0.60: decay = 3 days. At 0.85: decay = 1 day. At 0.95:
decay = 1 day. Worse WiFi produces shorter decay (the scar is more
likely environmental noise).

**Layer 2: Structural (genuine faults).** Decay: 180 days.
DirectObservation or CorrelatedSuspicion attribution. Full dominance
impact.

**Layer 3: Grain boundary (relay/path faults).** Decay: 30 days.
PathFault attribution. Moderate dominance impact on relay selection
priority. Does not affect the destination peer's dominance score
(relay is a pseudo-peer).

### Observation Window

The scar attribution is determined by the minimum WiFi quality observed
during the observation window, not the instantaneous value at scar
creation time. The observation window starts when the first missed
heartbeat occurs and ends when either (a) the scar threshold is
reached, or (b) heartbeats resume.

```
struct ObservationWindow {
    consecutive_misses: u16,
    min_wifi_quality_q8_8: u16,
    window_start_epoch: u64,
}
```

The running minimum ensures that a scar is classified as WifiInduced
if WiFi was bad at any point during the observation window, even if it
recovered before the threshold was reached. The min_wifi_quality is
local state (not replicated); it affects only the attribution field
written to the GSet.

### CircuitBreaker

Two-gate probe mechanism for excluding degraded sources.

Invariants:
- Three states: Closed (normal), HalfOpen (probing), Open
  (excluded).
- Transition to HalfOpen requires both Recovering direction
  AND variance below threshold.
- One probe per minute in Open state.

### Gossip

Dual-mode information dissemination: Plumtree (steady state)
and flood (during convergence).

Invariants:
- Plumtree: eager push on spanning tree, lazy IHAVE on
  non-tree edges.
- Flood: eager push on all edges. Bounded duration.
- Mode switch synchronized via heartbeat flag.
- Anomalous-message detection triggers local flood on
  unexpected eager messages.

### Convergence

Three-front post-partition recovery with sequenced gates.

Invariants:
- Front 1: control log merge + PartitionAnnouncement +
  dominance recomputation.
- Front 2: key distribution + CPSK renegotiation (polygon
  schedule).
- Front 3: at-rest re-encryption under new keys.
- Fronts are strictly sequenced: Front N gate must open
  before Front N+1 begins.
- Wavefront deadline gates Front 1 start.

### PartitionAnnouncement

Unilateral declaration that a peer was unreachable.

Invariants:
- 16 bytes, no acknowledgement required.
- Processed before scar evaluation during convergence.
- EMA reset on receipt (observation history from during
  partition is discarded).
- PartitionAbsence excluded from dominance calculation.

### Gateway

The elected representative of a cluster for inter-cluster
communication.

Invariants:
- At most one gateway per connected component at any time.
- Elected via max-register CRDT (availability primary,
  ETT tiebreaker).
- Gateway handles all inter-cluster frame routing for its
  cluster.

### ResourceGovernance

Framework for managing constrained resources (flash, CPU,
bandwidth, memory).

Invariants:
- GovernedResource: closed-loop PI controller with
  anti-windup. For flash budget, CPU.
- ThresholdGuard: open-loop banded response. For arena,
  queue depth.
- Governor output = min(all resource pressures).
- All resource-budget reads go through governor output
  (Conductor invariant).

### InviteFlow

User-facing onboarding: how a new cluster joins the mesh.

Invariants:
- Invite contains: mesh identity, bootstrap peer address,
  PSK for first handshake.
- SAS verification (6-emoji or word sequence) required for
  trust establishment.
- Invite is a one-time-use credential.

### LKH (Logical Key Hierarchy)

Binary tree for O(log N) revocation at scale (>= 48 peers).

Invariants:
- Revocation cost: O(log N) key wraps + O(log N) HKDF
  derivations.
- Falls back to flat KeyBundle for meshes below threshold.

### Channel

Hierarchical data stream with per-channel access control.

Invariants:
- Each channel has an independently generated 256-bit key
  (see verkko-crypto: ChannelKey).
- Access controlled per-channel, per-peer (off, opaque, read,
  write).
- Channels are the organizational unit for KeyBundle operations
  (Grant, Rotate, Revoke).
- Channel identity is a u16 index.

### Channel Lifecycle

Channel creation is admin-only, consistent with the single-admin-
authority model. Channel creation is blocked during convergence
(convergence_active = 1).

CHANNEL_CREATE control log entry (type 0x0E) payload:

```
ChannelCreatePayload {
    channel_index: u16,
    channel_key: [u8; 32],       // initial 256-bit key (CSPRNG)
    access_defaults: u8,         // per-role default access level
    replication_policy: u8,      // encoded ReplicationPolicy
    display_name_hash: [u8; 8],  // truncated BLAKE3 of name
}
```

The initial channel key is wrapped in a KeyBundle(Grant) and
distributed via the existing BroadcastEnvelope mechanism.

Channel index conflict prevention: indices are assigned by the admin
via a local counter. Single-admin-authority (fencing token) prevents
concurrent conflicting assignments. During partition with split admin,
the higher fencing token wins during Front 2 consensus; the losing
partition's channel creation is orphaned and requires re-creation.

Channel deletion does not exist. GSet semantics: append-only. A
channel is deactivated by revoking all keys (KeyBundle(Revoke)).

EDT scope_flags interaction: the u64 bitmask limits the EDT to
channels that existed at EDT creation time. New channels created
after EDT issuance are not accessible via that EDT. The admin must
reissue the EDT to include new channels.

### PhysicalBoundary

Trait for fixed-point newtypes that validate physical and
computed values at the boundary between hardware measurements
and the abstract state machine.

Invariants:
- All values are fixed-point integers. No floating-point
  types (f32, f64) in the abstract state machine.
- Representation: Q16.16 (u32 with 16 fractional bits) for
  bounded-range values. Q8.24 (u32 with 24 fractional bits)
  for alpha coefficients requiring higher fractional precision.
- `Self::MIN <= v <= Self::MAX` (bounded range, in fixed-point).
- Six instances: SackRatio, CapacityRatio, TimeDelta,
  HealthScore, ScarWeight, DominancePenalty.
- Lint-enforced: raw f64 and f32 cannot appear as parameters
  in the abstract state machine. Fixed-point newtypes only.
- Conversion from hardware measurements (which may be floating-
  point at the driver level) to fixed-point happens at the
  PhysicalBoundary gate and nowhere else.

### ControlLog

Signed, hash-linked, append-only GSet for trust state.

Invariants:
- All control entries are signed by the authoring DK.
- Entries are hash-linked (tamper-evident chain).
- GSet semantics: entries can be added, never removed.
- Scope (2B channel_index) per entry. Lamport clock (8B).
- Compaction: truncate > 90 days with signed snapshot.

### FencingToken

Monotonic sequence number for KeyBundle ordering within one
admin's stream.

Invariants:
- Peers track the highest fencing token seen.
- KeyBundle with a stale fencing token is rejected (except
  during grace window: Revoke only for stale tokens).
- 10-minute grace window after partition heal.

### Conductor

The governor's meet projection over the resource product lattice.
Sole coherence point for resource-budget parameters.

Invariants:
- Governor output = min(all resource pressures).
- For every resource-budget parameter `p`, there exists a pure
  function `f_p` such that `p = f_p(metabolic_state)`.
- No mechanism reads raw resource values to produce budget
  parameters; all consume the governor's metabolic state output.
- `Governed<T>` witness type proves at the type level that a
  value came from the governor.
- New resource dimensions enter the governor's `min()` without
  changing the metabolic state machine or downstream parameter
  maps.

### Memoria

The scar GSet viewed as institutional memory: permanent entries
with time-decayed weight, consumed by three independent scoring
functions.

Invariants:
- Scar entries are permanent GSet members: weight decays but
  existence does not.
- Linear 180-day decay applied at read time (not stored).
- Three consumers (health scoring, dominance cascade, gateway
  ETT) each apply their own formula to the same
  `weighted_scar_count` input.

### ObservationPathForwarding

4-byte per-flow health summary with gateway vertex cover property.

Invariants:
- Every inter-cluster data flow generates a FlowSummary.
- Gateway aggregates summaries from all cluster peers.
- Gateway is a vertex cover of all inter-cluster observation
  edges.
- Every inter-cluster data transfer contributes a health summary
  within two consecutive intra-cluster heartbeat intervals.

### HeartbeatExtension

The 18-byte extension region of the heartbeat frame. Carries
mesh-layer state observable by remote peers. The protocol layer
(verkko-protocol: Heartbeat) defines the frame format and the
18-byte transport header. The mesh layer defines the extension
region contents.

Invariants:
- The extension region is exactly 18 bytes.
- Field layout is fixed (big-endian):
  - filter_root (8 bytes): keyed truncated Merkle root.
  - epoch_current (4 bytes): G-Counter macro-epoch.
  - work_capacity (1 byte): min(bw_kbps/128, flash_kb/256, 255).
  - health_canary (1 byte): bits 7-4 = worst health class
    (0=Healthy, 1=Degraded, 2=Bad); bits 3-0 = degraded count
    (0-15, clamped).
  - observation_summary (4 bytes): aggregated FlowSummary.
- Protocol layer treats extension bytes as opaque payload.
  Mesh layer interprets them.
- No version field in the extension region. Changes require
  a protocol version increment.

State byte (offset 0 in transport header, protocol-owned):
    Bits 0-1: MetabolicState (protocol-layer)
    Bit 2:    gossip_mode (0=Plumtree, 1=flood)
    Bit 3:    convergence_active (0=normal, 1=converging)
    Bit 4:    admin_flag (0=non-admin, 1=admin)
    Bits 5-7: reserved (MUST be zero)

gossip_mode, convergence_active, and admin_flag are boundary
objects: protocol owns the bit position; mesh owns the
semantic meaning.

Ownership table:

| Field | Owner | Region | Bytes |
|-------|-------|--------|-------|
| state | verkko-protocol | Transport header | 1 |
| rtt_us | verkko-protocol | Transport header | 2 |
| loss_permille | verkko-protocol | Transport header | 2 |
| jitter_us | verkko-protocol | Transport header | 2 |
| queue_depth | verkko-protocol | Transport header | 1 |
| available_bw_kbps | verkko-protocol | Transport header | 2 |
| hlc_timestamp | verkko-protocol | Transport header | 8 |
| filter_root | verkko-mesh | Extension region | 8 |
| epoch_current | verkko-mesh | Extension region | 4 |
| work_capacity | verkko-mesh | Extension region | 1 |
| health_canary | verkko-mesh | Extension region | 1 |
| observation_summary | verkko-mesh | Extension region | 4 |

Error handling:
| Condition | Response |
|-----------|----------|
| Extension region < 18 bytes | Drop heartbeat. Log diagnostic. |
| epoch_current < previous from same sender | Drop heartbeat. Log warning. |
| health_canary worst_class > 2 | Treat as 2 (Bad). Log warning. |

### ConvergenceGate

Boolean predicate gating the transition between convergence fronts.
Consumers observe the gate's boolean output (open or closed) but
MUST NOT inspect the inputs or mechanisms behind the gate evaluation
(opaque projection principle).

Invariants:
- A gate is either OPEN (true) or CLOSED (false). No partial states.
- A gate, once OPEN, MUST NOT close within the same convergence event.
- Gates are evaluated locally by each peer.
- Gates have timeout fallbacks.

Gate definitions:

    Gate 1: ControlGate (ControlFront -> ConsensusFront)
        Predicate: all_control_entries_processed
            AND all_reachable_keys_obtained
            AND all_ema_resets_applied
        Timeout: 5 minutes.
        On timeout: orphan unretrievable channels. Gate opens.

    Gate 2: ConsensusGate (ConsensusFront -> ReencryptionFront)
        Predicate: highest_fencing_token_confirmed
            AND dominance_resolved
            AND ring_coloring_valid (provided by ConvergenceSignalProvider)
        Timeout: 10 minutes.
        On timeout: highest fencing token seen wins. Gate opens.

    Gate 3: ReencryptionGate (ReencryptionFront -> exit)
        Predicate: all_affected_content_reencrypted_or_epoch_forwarded
            (provided by ConvergenceSignalProvider)
            AND vital_queue_depth < 25%
        Timeout: none (bounded by tiered deadlines).

    Gate 4: ExitGate (exit sequence completion)
        Predicate: gossip_mode_ack_complete
            AND spanning_tree_connected
            AND health_baselines_updated
            AND ring_checksum_verified (provided by ConvergenceSignalProvider)
        Timeout: 3 * wavefront_deadline.

Interface (opaque projection):
    fn is_open(&self) -> bool;

Relay-dependent predicates (ring_coloring_valid,
all_affected_content_reencrypted, ring_checksum_verified)
are provided by the ConvergenceSignalProvider implementation
(supplied by verkko-relay at runtime). Mesh does not evaluate
relay state; it queries opaque boolean signals via same-process
trait implementation.

### ConvergenceSignalProvider

Abstract trait defined in verkko-mesh. Relay implements it.
The dependency direction is preserved: relay depends on mesh
(to know the trait); mesh depends on its own trait (to call it).

    trait ConvergenceSignalProvider {
        fn ring_coloring_valid(&self) -> bool;
        fn reencryption_complete(&self) -> bool;
        fn ring_checksum_verified(&self) -> bool;
    }

### ScarSeverityTier

Classification of scar types into severity tiers.

    #[repr(u8)]
    enum ScarSeverityTier {
        Excluded  = 0,  // weight 0.0 (dominance) / 0.5 (health)
        Transient = 1,  // weight 0.5
        Concerning = 2, // weight 1.0 (default for new types)
        Severe    = 3,  // weight 2.0
    }

Tier assignment table:
    PartitionAbsence           -> 0 (Excluded)
    SourceConfirmed (transient) -> 1 (Transient)
    NoAlternatePath            -> 1 (Transient)
    PathFault                  -> 2 (Concerning)
    CorrelatedSuspicion        -> 2 (Concerning)
    SourceConfirmed (persistent) -> 2 (Concerning)
    IntegrityFailure           -> 3 (Severe)

SourceConfirmed classification rule:
- Transient (Tier 1): first occurrence within a 24-hour window.
- Persistent (Tier 2): second+ occurrence within a 24-hour window.

Updated weighted_scar_count:
    weighted_scar_count(peer, now) =
        sum(tier_weight(s.tier) * decay_weight(s, now)
            for s in scars_targeting(peer))
    where:
        decay_weight(s, now) = max(0.0, 1.0 - (now - s.observation_epoch) / 180_days)

## Body

For the Mechanism Reference Table, Pattern Registry Table, and Boundary Object Table, see verkko-ops.

<!-- v10 preamble: Terminology -->
### Terminology

| Term | Meaning |
|------|---------|
| **Peer** | One saalis daemon on one device. |
| **Cluster** | Peers on the same LAN (household). |
| **Gateway** | Cluster peer handling external communication. Max-register CRDT election. |
| **Mesh** | Clusters connected as friends. Design envelope: 2-12 clusters, ~36 peers. |
| **Channel** | Hierarchical data stream. Access controlled per-channel. |
| **Seat** | UI process attached to a local peer. Not a mesh participant. |
| **Stash** | Peer's local record of held content. |
| **DK** | See verkko-crypto: DK. |
| **DEK** | See verkko-crypto: DEK. |
| **CIK** | See this document: CIK. |
| **CPSK** | See this document: CPSK. |
| **HLC** | Hybrid Logical Clock [Kulkarni2014]. |
| **EDT** | Enrollment Delegation Token. Admin-signed capability for non-admin peer to enroll new devices. |
| **GovernedResource\<P\>** | Closed-loop resource with governor integration. Four threshold bands, four responses. |
| **ThresholdGuard\<P\>** | Open-loop resource with single threshold and local response. |
| **HealthPipeline** | Dual-EMA health scorer with three output taps: classification, scar timing, breaker state. PhysicalBoundary-gated inputs. |
| **Ratchet\<Scope\>** | HKDF [Krawczyk2010] forward ratchet with micro-epoch sub-ratchet. Scope names the key category (epoch, rotation, CIK). |
| **KeyBundle** | Atomic key distribution message. Absorbs CompactRevocationBundle. |
| **NonceSourceFactory** | Factory producing thread-bound NonceSource instances with disjoint nonce ranges. |
| **LKH** | Logical Key Hierarchy [Wong1998, RFC2627]. Binary tree for O(log N) revocation at scale. |
| **ETT** | Expected Transmission Time [Draves2004]. Link quality metric combining delivery ratio, bandwidth, and frame size. |
| **Plumtree** | Epidemic broadcast tree protocol [Leitao2007]. Eager push on spanning tree, lazy IHAVE on remaining edges. |
| **PhysicalBoundary** | Trait for f64 newtypes that validate physical/computed values at the boundary between hardware and abstract state. Lint-enforced. See Section 2.0a. |
| **Conductor** | The governor's meet projection over the resource product lattice. Sole coherence point for resource-budget parameters. See this document: Conductor. Drives verkko-protocol: MetabolicState. |
| **Memoria** | The scar GSet viewed as institutional memory: permanent entries with time-decayed weight, consumed by three independent scoring functions. See Section 5.3. |

**Access levels:** off (no data), opaque (store/forward, no decrypt), read (decrypt), write (modify/propagate). Per-peer.

**Invariant enforcement pattern.** Both compile-time (EmptyLog, ActiveKey/OrphanedKey, DwellSatisfied) and runtime (nonce three-layer defense) enforce invariants via proof witnesses. Compile-time: zero runtime cost. Runtime: I/O cost. Same concept, different audiences, separate sections.

For the Verification Frame, see verkko-ops.

<!-- v10 preamble: Region descriptions -->
### Region 1: Monotone State

Values that only increase. Epoch counters, fencing tokens, control log (GSet [Shapiro2011]), enrollment certificates, scar counter values, flash consumption counters, re-encryption progress, nonce counters, micro-epoch counters, scar provenance entries. All instances of Structure 1 (JoinSemilattice): `ScalarLattice` (merge = max) or `SetLattice` (merge = union). See verkko-ops: Pattern Registry for full instance list.

Caveat: property-based grouping. Convergence ordering derives from the functor diagram (a DAG), not from this grouping.

### Region 2: Contraction Map

Everything that converges toward a moving target. Health EMAs, variance (Welford [Welford1962]), directional classification, scar timing, circuit breaker, partition absence reset. Core operation: `ema(alpha, dt, x_new, x_old)` (see verkko-ops: Pattern Registry, Structure 2).

### Region 3: Scheduling Envelope

Wire format, transport, segmentation, LEDBAT [RFC6817], congestion feedback, queue priorities, connection lifecycle, NAT traversal, gossip topology, CPSK renegotiation scheduling. Interior: `schedule(frame, urgency, bandwidth) -> when`. Detailed content in verkko-protocol and below.

### Region 4: Capability Lattice

Identity, channel keys, KeyBundle, dominance cascade, admin succession, broadcast encryption, CIK rotation, enrollment binding, EDT, LKH tree, cryptographic composition, content integrity. Detailed content in verkko-crypto and below.

### Region 5: Health and Observation

Health pipeline, partition absence, scar provenance, observation-path forwarding. Detailed content below.

### Region 6: Resource Envelope

Memory arenas, flash budget, CPU budget, metabolic state machine, entropic governor, progressive degradation, WritePermit, metabolic phase jitter. Governor: `min(CPU, flash, nonce_persist_io_pressure, min(WiFi_capacity, transport_capacity))`. Detailed content below and in verkko-protocol.

### Cross-Cutting: Post-Partition Convergence

Convergence specification in Section 7.1 below.

<!-- moved from verkko-crypto §1.3 steps 5-6 -->
### Mesh bootstrap

After key generation and founding transcript (see verkko-crypto: Genesis bootstrap), mesh initialization proceeds:

5. **Initial state.** CRDTs at zero. One control log entry (self-enrollment). Epoch 0. Nonce 0. Admin.
6. **Steady state.** First heartbeat. Metabolic SM in Background.

`EmptyLog` witness enforces no CRDT ops before founding transcript. Consumed by first control log append -> `ActiveLog`.

<!-- moved from verkko-crypto §1.4 steps 3-7 -->
### Device recovery enrollment

After replacement device generates new DK + DEK and locates the mesh via Noise XX (see verkko-crypto: Device recovery flow):

3. **Join.** Noise XX [Perrin2018] (IK+XX fallback) with PSK. Admin (or EDT holder) signs enrollment cert.
4. **SAS verification.** 24-hour window. Unverified: flagged, not revoked, admin notified.
5. **Key delivery.** KeyBundle with authorized channel keys. SAS verification must complete before KeyBundle(Grant) is issued.
6. **Revocation.** Admin revokes lost DK. KeyBundle distributed. Re-encryption begins.
7. **Filter sync.** Full-filter-sync from connected clusters.

~2 seconds on LAN. Filter sync: ~10 seconds for 100K entities.

<!-- v10 §1.2 -->
### 1.2 Enrollment binding

```
enrollment_cert = Sign(admin_DK, new_DK_pub || new_DEK_pub || enrollment_epoch || cluster_CIK)
```

Logged to control log GSet (permanent). Dominance cascade requires valid enrollment cert. Bounds Sybil [Douceur2002]: pre-computed low-hash keys useless without enrollment. Residual Sybil surface: (a) leaked pre-signed cert: 1 Sybil device per leaked cert, bounded by 30-day expiry, detectable via control log audit; (b) compromised EDT holder: up to 3 Sybil devices per compromised token, bounded by 7-day expiry, SAS verification gates key delivery. In both cases, Sybil devices cannot decrypt channel data until SAS verification completes (24-hour window). The Sybil bound is: max 3 unkeyed devices per compromised EDT, max 1 unkeyed device per leaked pre-sign cert. "Unkeyed" means enrolled but unable to decrypt any channel data.

**Three-boundary Sybil defense:** (1) Enrollment binding prevents unauthenticated devices from entering the dominance cascade. (2) SAS [RFC6189] -gated key delivery prevents Sybil devices from accessing data. (3) Keyless admin limitation: even if a Sybil device wins dominance, it cannot issue KeyBundle(Grant) (it has no keys to grant); it can only issue KeyBundle(Revoke), which is the operation most resistant to abuse. This defense relies on admin authority for enrollment, consistent with Douceur's impossibility result [Douceur2002]: Sybil resistance without a centralized or logically centralized authority is impossible in a fully decentralized system.

**Sybil detection.** Admin notification fires when: (a) any enrollment cert references an EDT; (b) enrollment count per 24-hour window exceeds the mesh's device count; (c) SAS verification is not completed within 24 hours.

Admin can pre-sign for known DK pubkeys (30 days, distributed via control log).

<!-- v10 §1.6 -->
### 1.6 Delegated enrollment tokens (EDT)

Admin-signed capability that allows a non-admin peer to enroll new devices on the admin's behalf.

```
EDT = Sign(admin_DK,
    delegate_DK_pub || max_enrollments || expiry_epoch || scope_flags
)
```

**Properties:**
- Logged to control log GSet (auditable).
- `max_enrollments`: capped at 3 per token. Prevents bulk Sybil via delegate.
- `expiry_epoch`: maximum 7 days. Short-lived capability.
- `scope_flags`: which channels the delegate can grant access to (u64 bitmask).
- EDT holder signs enrollment cert on admin's behalf. The enrollment cert references the EDT.

**SAS-gated key delivery.** The EDT holder can enroll the device and sign the enrollment cert. KeyBundle(Grant) with channel keys is delayed until SAS verification completes. The enrolling device enters Active state for the mesh but cannot decrypt channel data until SAS is verified and keys are delivered.

**LKH interaction.** EDT holder assigns a temporary leaf in the LKH tree (Section 2.7). On admin's next online period, admin integrates the temporary leaf into the balanced tree via lazy rebalancing. No immediate rebalancing authority needed for the delegate.

**Cost:**
- Wire: ~130 bytes per EDT (signature + fields). Rare (one per delegation event).
- Control log: one entry per EDT issuance, one per enrollment-via-EDT.

<!-- v10 §1.7 -->
### 1.7 Gossip and control log

**Gossip:** `GossipConfig { ack_mode: Eager|Lazy, retransmit: Immediate|OnDemand, priority: Vital|Reactive|Bulk }`.

**Control log:** signed, hash-linked, append-only GSet [Shapiro2011]. Scope (2B channel_index) per entry. Lamport clock [Lamport1978] (8B). Compaction: truncate > 90 days with signed snapshot. 30-day quorum checkpoint optional. **Data plane:** HLC [Kulkarni2014] + peer_id tiebreaker total order.

**Control floor.** HLC provides causal ordering across the data plane. Lamport clocks on control messages provide total ordering for the control plane. Convergence time for filter propagation is proportional to gossip diameter and varies with runtime topology. No tighter bound is specified; see Design Constraints (Section 9).

### Control Log Entry Wire Format

Fixed overhead: 175 bytes (excluding payload).

    Offset  Width  Field
    ------  -----  -----
     0     32      entry_hash: BLAKE3 of bytes [32..end_of_payload]
    32     32      prev_hash: hash of previous entry (0x00 for genesis)
    64      8      lamport_clock: u64 (monotonic per author)
    72      1      version: u8 (0x01)
    73      1      entry_type: u8
    74      2      channel_index: u16 (0xFFFF for mesh-wide)
    76     32      author_dk: Ed25519 public key
   108      2      payload_len: u16 (max 4096)
   110     var     payload
   110+N  64      signature: Ed25519 over entry_hash

Entry types:
    0x01 ENROLLMENT     0x08 PARTITION_ANN
    0x02 REVOCATION     0x09 SCAR
    0x03 KEY_GRANT      0x0A SNAPSHOT
    0x04 KEY_ROTATE     0x0B SUCCESSION
    0x05 KEY_REVOKE     0x0C GATEWAY_CLAIM
    0x06 CIK_ROTATE     0x0D GATEWAY_RELEASE
    0x07 EDT_ISSUE      0x0E CHANNEL_CREATE (reserved)

GSet merge: entries exchanged in batches.
    [u32 batch_count] [batch_count * entry]

Forward compatibility: unknown entry_type values are stored
but not processed. This enables protocol evolution without
breaking the hash chain.

<!-- v10 §2.0a -->
### 2.0a PhysicalBoundary trait

The sole shared trait mandated by the Pattern Registry (see verkko-ops). Covers the 6 fixed-point newtypes that validate physical and computed values at the boundary between hardware measurements and the abstract state machine.

#### Fixed-point arithmetic (design decision)

All arithmetic in the health pipeline, EMA computations, dominance
cascade, scar weighting, and metabolic governor uses fixed-point
integers. Floating-point types (f32, f64) do not appear in any
verkko protocol or state machine computation.

**Representation:**
- Q16.16 (u32 with 16 integer bits, 16 fractional bits) for
  bounded-range values (scores, ratios, weights). Range: [0, 65535]
  with 1/65536 resolution.
- Q8.24 (u32 with 8 integer bits, 24 fractional bits) for alpha
  coefficients requiring higher fractional precision. Range: [0, 255]
  with ~0.00000006 resolution.
- TimeDelta uses u32 milliseconds (not fixed-point). Range: [1, 86400000].

**Rationale:** Fixed-point guarantees deterministic cross-peer
comparison on all target hardware (ARM, x86). Floating-point
rounding differences between platforms could cause peers to
disagree on health classifications, dominance ordering, or
convergence gate predicates. The precision loss from Q16.16 is
irrelevant for health scoring (0.0015% resolution vs WiFi noise).

**Conversion boundary:** Hardware measurements (which may arrive as
floating-point from OS APIs) are converted to fixed-point at the
PhysicalBoundary gate. This is the sole conversion point. All
computation downstream of the gate is integer arithmetic.

```rust
trait PhysicalBoundary: Sealed {
    const MIN: u32;    // Q16.16 fixed-point
    const MAX: u32;    // Q16.16 fixed-point
    fn try_new(raw: u32) -> Option<Self>;
}
```

**Invariant (per type):**
```
1. Self::MIN <= v.as_q16_16() <= Self::MAX     (bounded range)
2. Construction via try_new() is the sole entry point
```

| Type | MIN (Q16.16) | MAX (Q16.16) | Purpose |
|------|-------------|-------------|---------|
| SackRatio | 0x00000000 | 0x00010000 | SACK [RFC2018] delivery ratio (0.0-1.0) |
| CapacityRatio | 0x00000000 | 0x000A0000 | Adaptive CPU setpoint (0.0-10.0) |
| TimeDelta | 1 (ms) | 86400000 (ms) | Time between observations (u32 ms, not Q16.16) |
| HealthScore | 0x00000000 | 0x00010000 | EMA health output (0.0-1.0) |
| ScarWeight | 0x00000000 | 0x00010000 | Decay-weighted scar count (0.0-1.0) |
| DominancePenalty | 0x00010000 | 0xFFFFFFFF | Dominance cascade multiplier (1.0-65535.0) |

**EMA alpha coefficients** use Q8.24 representation:
- alpha_fast = 0.1 -> 0x00199999 (Q8.24)
- alpha_slow = 0.01 -> 0x00028F5C (Q8.24)
- capacity alpha = 0.05 -> 0x000CCCCC (Q8.24)

**Why this trait and not the others.** PhysicalBoundary has a pure, stateless, context-free validation that has been stable across 8 revisions. All 6 instances share the identical constructor signature (`try_new(u32) -> Option<Self>`). The other patterns (EMA, MonotoneLattice) have already diverged: fencing tokens broke pure ScalarLattice by adding a grace window; HealthPipeline EMA diverged by adding dual-EMA, stale cutoff, and PhysicalBoundary gating. A shared trait for a pattern that has already diverged creates blast-radius risk without corresponding benefit.

**Non-trait boundary gates.** The following gates enforce analogous validation but do not share the `try_new(u32)` signature and are not part of the PhysicalBoundary trait. They are listed in the verkko-ops: Pattern Registry under Structure 5 as convention-enforced:
- Network gates: `HeartbeatFields::validate`, `ScarEntry::validate`, `KeyBundlePayload::validate`
- Temporal gates: `HlcTimestamp::try_advance`, `MacroEpoch::from_hlc`
- Storage gate: `NonceHwm::recover` (checksum-validated, structurally different)

**Lint enforcement.** A lint rule prevents raw floating-point types (f32, f64) and raw integer types (u32, Duration, &[u8]) from appearing as parameters to functions in the abstract state machine (outside the boundary modules). Compile-time, not CI-time.

<!-- v10 §2.2 -->
### 2.2 KeyBundle (unified type)

```
KeyBundle {
    operation: u8,                     // Grant | Rotate | Revoke
    channel_keys: [(u16, [u8; 32])],   // 34 bytes per channel
    revoked_peers: [u32],              // 4 bytes per revoked peer
    predecessor_hashes: [[u8; 32]],    // causal dependencies
    sequence: u64,                     // monotonic fencing token
    admin_signature: [u8; 64],         // Ed25519
}
```

Delivered via reliable control channel. Revocation-triggered fresh keys stored in control log entry (wrapped to each authorized DEK). Routine epoch advances: HKDF-derivable, nothing stored. On reconnection: replay control log, extract non-derivable keys.

**Compact revocation:** KeyBundle with `Revoke` + multiple channel_keys. 50 channels: 1,808 bytes, ~4x reduction vs individual KeyBundles.

<!-- v10 §2.3 -->
### 2.3 Dominance cascade (admin succession)

Deterministic from public information. No election, no messages, no consensus. The design avoids distributed consensus by construction [FLP1985]: all state is computed deterministically from eventually consistent CRDTs, not from a consensus protocol. Eligible set: `{p : weighted_scar_count(p) <= 10 AND has_valid_enrollment_cert(p)}`.

```
dominance_score(peer) = BLAKE3(DK_pub || current_epoch)
                        * (1 + weighted_scar_count)
                        * 2^max(0, weighted_scar_count - 10)
```

**Lowest score = dominant admin. `admin = argmin(dominance_score)`.** The linear factor `(1 + scars)` provides smooth degradation across all scar counts. The exponential factor `2^max(0, scars - 10)` activates only above threshold 10, providing accelerating penalty that makes heavily scarred peers astronomically unlikely to dominate even if the eligible set check fails.

**Dominance scar penalty vs health scar penalty.** These are intentionally different formulas. Dominance scoring uses `(1 + scars) * 2^max(0, scars - 10)` because admin eligibility has a hard threshold. Health scoring (Section 5.1) uses `0.9^weighted_scar_count` because source quality degrades smoothly. Both use the same `weighted_scar_count` input from the scar Memoria (Section 5.3).

**Continuity at threshold 10:**
- At scar count 0: `hash * 1 * 1 = hash` (full dominance eligibility).
- At scar count 10: `hash * 11 * 1 = hash * 11` (linear penalty).
- At scar count 11: `hash * 12 * 2 = hash * 24` (exponential kicks in).
- Both sides of threshold 10 give `hash * 11`. Continuous.

**Handicap tournament property.** A peer with N scars needs a hash value that is `(1 + N)` times lower than an unscarred competitor to win dominance. The probability of winning any given epoch is exactly `1 / (1 + N)`. Epoch mixing rotates dominance. If all peers exceed scar threshold, fall back to full peer set (liveness preserved).

**Succession notification.** SuccessionEvent in control log. Local notification dispatch + external channels (fire-and-forget POST, 3 attempts, 1s/5s/30s backoff).

**Admin heartbeat:** 1-bit flag (bit 4 of state u8). Zero wire cost. **Succession timing:** routine 3 missed heartbeats (60s idle, 15s active); emergency 1 missed heartbeat. The heartbeat-based failure detection mechanism is a diamond-P (eventually perfect) failure detector [Chandra1996]: it may produce false positives but eventually converges to accurate failure detection.

**Fencing tokens:** `sequence` field in KeyBundle. Peers track highest. 10-minute grace after partition heal. **Grace restriction: Revoke only.** During the 10-minute grace window, only KeyBundles with `Revoke` operation are accepted from stale fencing tokens. Stale-token Rotate and Grant are discarded.

**Dead branch resolution.** Grants: auto-ratified if valid enrollment cert, logged, admin notified. Rotations: discarded (dominant wins). Revocations: always honored (revoke-wins). Unratified grants expire 10 minutes. **Conflict:** Revoke > Grant > Rotate. Ties: Lamport.

**Transient dominance disagreement.** During GSet convergence, peers may compute dominance scores from different GSet states. Two peers may simultaneously believe they are admin. This is a transient condition bounded by flood mode's mixing time (one WAN RTT, ~200ms) during convergence. Mitigation: (1) fencing tokens prevent conflicting grants; (2) revoke-wins ensures no revocation is lost; (3) Front 1 enforces GSet merge before dominance-dependent operations.

**Next-admin display:** every device shows next-in-line, updated each epoch. **CIK rotation:** signed control log entry, accept both CIKs for 48 hours. Emergency: Noise XX + TOFU + admin notification.

<!-- v10 §2.5 -->
### 2.5 Post-entry checklist

Shared steps after path-specific auth/keygen: (1) filter sync, (2) key delivery via KeyBundle, (3) control log replay, (4) health init.

<!-- v10 §2.6 — PCS TRIGGER and DISTRIBUTION -->
### 2.6 CPSK-seeded PCS — trigger and distribution

For the PCS key derivation mechanism, see verkko-crypto: PCSMechanism.

**Per-channel re-seed rate limiting.** During convergence, a per-channel counter limits the number of re-seed events per polygon schedule round. The limit is computed at runtime from observed conditions:

```
max_reseeds_per_round = pending_buffer_remaining / (observed_frame_rate * round_duration * frame_size)
```

Evaluated at the start of each polygon schedule round. This makes the rate limit adaptive to metabolic state: at Stress mode (higher frame rate), the limit is tighter; at Background mode (lower frame rate), the limit is looser. The adaptation is real because it uses the observed frame rate, not a compile-time constant.

When the limit is reached, additional pairs are deferred to extended polygon schedule rounds within the same convergence event (not to the next event). The polygon schedule grows additional rounds with vertex-collision-free pair assignment until all pairs complete. Worst case for one globally shared channel with 66 pairs at limit 5: `ceil(66/5) = 14` rounds * 2s = 28s of polygon schedule time. Total convergence (10s wavefront + 28s polygon + 5s stabilization) = 43s. The design accepts this extension for the worst case.

**Edge-colored scheduling interaction (Section 3.6).** During post-partition convergence, CPSK renegotiations are scheduled via the polygon 1-factorization [Lucas1883]. Re-seed chain proofs (30 bytes per channel per pair) replace full KeyBundle distribution.

**Sealed-range catch-up ordering.** When a peer reconnects after a partition and CPSK renegotiation is about to re-seed a channel, sealed-range catch-up (see verkko-matter: SealedRange) completes first. The CPSK re-seed is deferred until after the catch-up delivers the current epoch key. This prevents the catch-up work from being wasted by an immediate re-seed.

<!-- v10 §2.7 -->
### 2.7 LKH revocation tree

At small mesh sizes (< 48 peers), flat key wrapping (one wrap per recipient) is sufficient. At larger mesh sizes (>= 48 peers), transition to a Logical Key Hierarchy (LKH) [Wong1998, RFC2627] binary tree:

```
LKH Tree (8 peers example):
           K_root
          /      \
       K_L1      K_R1
      /    \    /    \
    K_LL  K_LR K_RL  K_RR
    / \   / \  / \   / \
   P1 P2 P3 P4 P5 P6 P7 P8
```

**Revocation cost:** O(log N) key wraps + O(log N) HKDF derivations. Leaf assignment at enrollment. Lazy rebalancing when imbalance > 2.

**EDT interaction.** Temporary leaves appended to rightmost branch, integrated on admin's next online period.

## Intra-Cluster Communication

### Session Model

Intra-cluster peers use Noise KK with each peer's DEK as the static
key. One session per peer pair within the cluster. CIK MUST NOT be
used as the static key for intra-cluster Noise KK: CIK is shared
across all cluster peers, and Noise KK with identical static keys on
both sides degenerates (the handshake produces symmetric DH outputs
in both directions).

Session lifecycle:
- Initiated on peer enrollment (enrollment cert contains new_DEK_pub).
- Renegotiated on DEK rotation (rare).
- Torn down on peer departure (enrollment revocation, clean shutdown).
- No polygon scheduling (LAN RTT is negligible).

### dest_peer Assignment

Sequential u16 at enrollment, starting from 0x0001, incrementing for
each new peer. The assignment is logged in the enrollment cert (one
additional u16 field). dest_peer 0x0000 is broadcast (all cluster
peers). dest_peer values are never reused within a cluster's lifetime
(even after peer revocation, the value is retired).

### Gateway Routing Within Cluster

The gateway maintains a routing table mapping dest_peer values to
intra-cluster session handles. When a frame arrives from an inter-
cluster session with dest_peer != 0x0000, the gateway looks up
dest_peer in the routing table and forwards the frame on the
corresponding intra-cluster session. The inner header is not modified;
only the outer header changes (new connection_id, new nonce,
decremented hop_count).

### Intra-Cluster Heartbeats

Fixed 1-second interval (LAN bandwidth is not a constraint; the
variable interval is for WAN conservation). Same transport header as
inter-cluster heartbeats. Simplified 8-byte extension region:
filter_root (8 bytes) only. Other extension fields (epoch_current,
work_capacity, health_canary, observation_summary) are inter-cluster
concerns.

### Intra-Cluster Health Scoring

Simplified. No dual-EMA. A sliding window of the last 32 heartbeat
results (u32 bitmask). Health degradation triggers when fewer than 22
of the last 32 heartbeats were received (approximately 30% loss
sustained over 32 seconds). This threshold absorbs WiFi burst losses
of up to 10 consecutive misses without triggering degradation.

Scar threshold: 30 consecutive missed heartbeats (30 seconds). At 15%
WiFi packet loss, the probability of 30 consecutive misses is
0.15^30, which is effectively zero. Only genuine 30+ second outages
produce scars. WiFi-induced scars use the WifiInduced attribution
(see Scar Classification) with 1-7 day adaptive decay.

Gateway election dampening: gateway re-election requires sustained
intra-cluster health degradation (10+ seconds below threshold) or
explicit peer departure. Transient WiFi-induced degradation (< 10
seconds) does not trigger gateway re-election.

### Resource Accounting

Intra-cluster sessions use a separate ThresholdGuard instance: 10 KB
arena allocation per peer, 60 KB maximum for a 6-peer cluster. Total
managed arena: 256 KB (200 KB inter-cluster + 56 KB intra-cluster).

### Gateway Handoff

Gateway handoff tears down and re-establishes inter-cluster sessions.
Connection_id is NOT preserved across gateway handoff (the new gateway
is a new Noise endpoint with a different DEK).

Handoff procedure (4 phases):

1. **Prepare:** New gateway receives CPSK session parameters from old
   gateway via intra-cluster vital-tier reliable stream.
2. **Establish:** New gateway initiates Noise KK sessions with all
   inter-cluster gateways using the transferred CPSK parameters.
3. **Migrate:** Old gateway stops accepting new inter-cluster frames.
   New gateway begins accepting. Overlap window: both gateways forward
   intra-cluster frames during the transition.
4. **Teardown:** Old gateway tears down its inter-cluster sessions.
   New gateway is sole inter-cluster endpoint.

CIK rotation ordering: CIK distribution (intra-cluster, vital tier,
reliable delivery) MUST complete before the new gateway establishes
inter-cluster CPSK sessions. The new gateway waits for intra-cluster
CIK distribution confirmation before initiating inter-cluster
handshakes.

Relay migration (distinct from gateway handoff): connection_id IS
preserved. The relay is not a Noise endpoint; only the forwarding path
changes. PATH_CHALLENGE through the new relay verifies reachability.

<!-- v10 §3.5 -->
### 3.5 Dual-mode gossip

Inter-cluster gossip operates in two modes, matched to the mesh's risk profile.

**Steady-state mode: Plumtree** [Leitao2007]**.**

Epidemic broadcast tree protocol between gateways. Eager push on a spanning tree. Lazy IHAVE on all non-tree edges with delay = 1 RTT.

- Amplification: ~1.3x (11 eager + ~3 lazy IHAVE per dissemination event at 12 gateways).
- Tree optimization: weighted Plumtree edge-swap during the 5-second stabilization window after flood exit. Plumtree constructs an arbitrary spanning tree from arrival order; the edge-swap pass improves the tree using heartbeat RTT weights but does not guarantee an MST.
- Lazy repair: detects withholding by eager-tree nodes within 1 RTT (~200ms).

**Note on amplification.** The 1.3x figure applies to well-optimized trees with reasonable diameter. At 12 nodes on K_12, tree topology has significant impact. The edge-swap optimization during stabilization reduces amplification toward the 1.3x target. Actual amplification may be 1.3-2x depending on tree quality. The bandwidth budget (Section 8) uses 1.3x; if tree quality is persistently poor (measurable via amplification monitoring), the Plumtree RTT shift recompute threshold (calibration item 14) triggers re-optimization.

**Convergence mode: Flood.**

Every gateway sends every gossip message to all 11 other gateways. Deduplication via message ID. This is a form of epidemic dissemination [Demers1987, Karp2000] where the O(log n) dissemination bound [Karp2000] ensures rapid convergence.

- Amplification: 12x.
- Duration: bounded by convergence window.

**Why flood during convergence is mandatory.** Gossip mixing time on a graph with Fiedler value [Fiedler1973] 0.167 (one bridge edge after partition heal) is O(n * log(n) / lambda_2) = O(12 * 2.48 / 0.167) ~ 178 rounds (Boyd et al. [Boyd2006]). At 5-second heartbeat intervals: ~15 minutes to guarantee all gateways have received all messages. Flood has mixing time O(1). The 178x gap makes flood mandatory during convergence.

**Mode transition protocol.**

1. **Entering convergence.** Gateway entering Converging state sets gossip_mode=1. Any gateway receiving gossip_mode=1 switches to flood locally.

2. **Anomalous-message detection.** When a Plumtree gateway receives an eager message from a non-tree edge with a message ID not in recent_ihave_set, it switches to flood locally before the next heartbeat. This reduces the mode transition window from one heartbeat interval to one message round-trip. This also provides Byzantine detection: a malicious gateway flooding messages on non-tree edges triggers the same defense as a legitimate mode transition.

3. **Exiting convergence.** See convergence exit sequence (Section 7.1, Exit Transition).

4. **Flaky gateway dampening.** Exponential backoff to flood triggers from the same gateway: `flood_cooldown[gw] = min(flood_cooldown[gw] * 2, 300_000)`. After two triggers within dwell time (20s), subsequent triggers from that gateway are suppressed until cooldown expires. Other gateways can still trigger flood.

5. **Damping.** The 20-second metabolic dwell time prevents rapid oscillation. A false convergence trigger locks the mesh in flood for at least 20 seconds. Additional airtime: ~0.6% of a 7.2 Mbps WiFi link. Small enough to not worsen the WiFi contention that triggered the false convergence.

**Intra-cluster dissemination.** Reliable broadcast (iterated unicast) for Vital/Reactive messages. Best-effort broadcast for gossip IHAVE.

<!-- v10 §3.6 -->
### 3.6 Edge-colored CPSK renegotiation schedule

CPSK renegotiations scheduled via the polygon round-robin tournament (Lucas [Lucas1883], standard 1-factorization of complete graphs [Anderson1997, Wallis2007]) to eliminate vertex collision and smooth peak load.

**The 1-factorization of K_12.** For n = 12 vertices (even), the schedule produces n - 1 = 11 rounds, each containing n/2 = 6 edges (a perfect matching). By Vizing's theorem [Vizing1964], the chromatic index of K_12 is 11, so this schedule is optimal. No fewer rounds are possible.

**Construction.** Fix vertex n-1 (vertex 11 in 0-indexed). In round r (0 <= r <= 10):
- Vertex 11 plays vertex r.
- For i in 1..5: vertex (r - i) mod 11 plays vertex (r + i) mod 11.

**Vertex-to-gateway mapping.** Gateways sorted by median WAN RTT (ascending). Lowest-RTT gateway assigned to the center position (vertex 11). This minimizes total schedule duration on high-latency meshes. When WANs are fast enough that the 2-second floor dominates, the assignment has no effect.

**Steady-state schedule.** Slot spacing: `rehandshake_interval / 11` ~ 5.5 minutes at 60-minute interval.

**Convergence schedule (compressed).** Slot spacing: `max(3 * wan_rtt_estimate, 2_seconds)`. At 400ms WAN RTT: 2-second slots. 11 slots = 22 seconds total.

**Extended rounds for rate-limited channels.** When the per-channel re-seed rate limiter (Section 2.6) defers pairs, the polygon schedule generates additional rounds with vertex-collision-free pair assignment. The schedule extends until all pairs complete. This keeps convergence in a single event rather than deferring to subsequent events. Worst case: 14 rounds * 2s = 28s polygon schedule time.

**Re-seed chain proof.** During convergence, re-seed events emit chain proofs instead of full KeyBundles. A re-seed chain proof is a derivation proof (30 bytes per channel) that allows recipients to derive the new key from the previous key and the proof material. ~99% reduction in per-event re-seed wire cost.

**Odd cluster count handling.** Add a dummy vertex (a "bye"). Edges incident to the dummy vertex are no-ops.

**Airtime cost.** Per slot per gateway: 3 Noise KK messages (600 bytes) + 1 re-seed chain proof (~300 bytes) = ~900 bytes. On 7.2 Mbps WiFi: ~2ms airtime per slot.

<!-- v10 §5.1 -->
### 5.1 HealthPipeline (3 output taps)

Unified pipeline consuming dual-EMA state. Three output taps: health classification, scar timing, circuit breaker state. All inputs gated by PhysicalBoundary newtypes (Section 2.0a).

**NaN guard (critical).** When `sack_sent == 0` (idle Background-mode connection), `observed_delivery = 0/0 = NaN`. NaN permanently poisons the health EMA. The `SackRatio::try_new` gate prevents this:
```rust
fn try_new(delivered: u32, sent: u32) -> Option<SackRatio> {
    if sent == 0 { return None; }
    let r = SackRatio::try_new(
        ((delivered as u64 * 0x10000) / sent as u64) as u32
    ).unwrap_or(SackRatio::MAX);  // Q16.16 fixed-point
    debug_assert!(r.is_finite());
    Some(SackRatio(r))
}
```
When `SackRatio::try_new` returns `None`, the EMA update is skipped. No observation recorded. The health score retains its previous value. This is correct: no data received means no information about link quality.

**Input: dual-timescale EMA with directional classification.**

Fast EMA: alpha_base=0.1, ~10 observations to react. Slow EMA: alpha_base=0.01, ~100 observations. Health key: `(source_cluster, channel, transit_path)`. See verkko-ops: Pattern Registry, Structure 2 for the EMA contraction map [Banach1922] invariants.

**Time-normalized alpha:**
```
alpha_effective = 1 - (1 - alpha_base)^(dt / dt_reference)
```
dt_reference = 5 seconds. Response time independent of observation rate. When `dt` is tracked per attributed source, the time-normalized formula correctly handles observation frequency differences, including aggregated observations (Section 5.4).

**Effective health:** `min(fast_ema, slow_ema)`. Asymmetric: recovery ~100 observations, degradation ~10.

**Observation weighting (receiver-observed).** Link quality uses receiver-measured delivery ratios (via SACK state) instead of sender-reported loss_permille. SACK-based delivery ratio averaged over the metabolic dwell time with EMA alpha=0.1 to smooth bursty WiFi loss.

```
// Receiver-observed delivery ratio (from SACK state, EMA-smoothed)
observed_delivery: SackRatio = SackRatio::try_new(sack_delivered, sack_sent)?
// Returns None if sack_sent == 0; caller skips EMA update

// Per-observation weighting
weight = if observation_received { 1.0 } else { 1.0 - observed_delivery.value() }
fast_ema = alpha * (observation * weight) + (1 - alpha) * fast_ema
```

**Observation replay during convergence.** On partition heal, GSet merge delivers scar entries from the partition period. These are replayed into the health EMA.

**Replay constraints:**
1. **Per-observer only.** Replay processes only the local cluster's scar entries for EMA updates. Remote clusters' entries affect `weighted_scar_count` (additive merge) but not the health EMA. The time-normalized alpha assumes successive observations from the same observer.

2. **Stale cutoff.** For observations older than the stale cutoff, the time-normalized alpha rounds to exactly 1.0 in IEEE 754 binary64, making the EMA update a hard reset (discards all history). This is made explicit:

| EMA type | alpha_base | Stale cutoff (retained weight < ULP of 1.0) |
|----------|-----------|----------------------------------------------|
| Fast | 0.1 | ~29 minutes (~1,748 seconds) |
| Slow | 0.01 | ~4.9 hours (~17,480 seconds) |

```rust
fn replay_observation(ema: &mut f64, obs: HealthScore, dt: TimeDelta, alpha_base: f64) {
    let stale_cutoff = STALE_CUTOFF[alpha_base]; // precomputed per EMA type
    if dt.seconds() > stale_cutoff {
        hard_reset(ema, obs);
    } else {
        let alpha = 1.0 - (1.0 - alpha_base).powf(dt.seconds() / DT_REFERENCE);
        *ema = alpha * obs.value() + (1.0 - alpha) * *ema;
    }
}
```

3. **Replay order.** Entries sorted by original timestamp, processed sequentially with per-observation time-normalized alpha.

**Observation lineage tracking.** Health and ETT [Draves2004] (Section 7.2) are parallel consumers of the same SACK data through different formulas. To prevent temporal skew between these parallel consumers, each observation carries an `observation_id` derived from the SACK packet's nonce or sequence number. Each consumer records the last observation_id it processed. If any consumer falls more than 5 observations behind (skew budget), it processes a batch catch-up. Lineage tracking provides same-source guarantees without the coupling cost of a synchronization barrier.

#### HealthPipeline operating modes

    #[repr(u8)]
    enum HealthMode {
        Normal = 0,
        Damped = 1,
    }

NORMAL mode: default. EMA alpha values: fast=0.1, slow=0.01.

DAMPED mode: active during convergence. Attenuated EMA alpha values.
    damped_alpha_fast = alpha_fast * 0.3
    damped_alpha_slow = alpha_slow * 0.1

Transitions:
    NORMAL -> DAMPED: any ConvergenceGate opens. Immediate.
    DAMPED -> NORMAL: ExitGate opened and exit sequence completed.

Distributed coordination: remote peers detect convergence via the
convergence_active bit (bit 3 of state byte) in HeartbeatExtension.

Zero-observation bypass: if a peer has sent zero heartbeats for 3+
consecutive periods, the damping mask MUST NOT apply. Absence is a
genuine liveness signal.

Invariants:
- DAMPED mode existence is a protocol constant. Not optional.
- Coefficients are calibration items: 0.0 < coefficient < 1.0.
- Mode transitions do not reset EMA state.

#### Arithmetic Safety Invariant

All computation in the health pipeline, scar system, dominance
cascade, and metabolic governor uses fixed-point integer
arithmetic (see Section 2.0a). NaN and infinity cannot occur
by construction.

The remaining risks are overflow and underflow of fixed-point
intermediates. Guards:
- All intermediate multiplications MUST use u64 to avoid u32
  overflow before truncation back to Q16.16.
- EMA update: `new_value = old + alpha * (observation - old)`
  computed as `old + ((alpha_q8_24 as u64 * diff as u64) >> 24)`
  with saturating arithmetic.
- Division by zero: if a denominator is zero, substitute the
  previous valid result. If no previous valid result exists,
  use the type's default (0x00000000 for scores, 0x00010000
  for weights).
- MetabolicGovernor::evaluate(): if any pressure value is
  outside [0, 0x00010000], clamp to range before evaluation.
- Dominance score comparison: deterministic; identical inputs
  produce identical outputs on all platforms (integer comparison).

**Tap 1: Health classification.**

```rust
#[repr(u8)]
enum HealthDirection {
    Recovering = 0,
    Degrading  = 1,
}

#[repr(u8)]
enum VarianceClass {
    Consistent   = 0,
    Intermittent = 1,
}

struct HealthClassification {
    direction: HealthDirection,
    variance: VarianceClass,
}
```

Three health states:

| State | Condition |
|-------|-----------|
| Healthy | slow EMA > 0.92 |
| Degraded | slow EMA 0.50 - 0.92 |
| Bad | slow EMA < 0.50 OR fast EMA < 0.30 |

**Tap 2: Scar timing.**

```rust
enum ScarTiming {
    Immediate,
    DelayedShort { remaining_observations: u8 },
    DelayedLong { remaining_observations: u8 },
}

fn classify_scar_timing(c: HealthClassification, cfg: &ScarConfig) -> ScarTiming {
    match (c.variance, c.direction) {
        (Consistent, _)           => ScarTiming::Immediate,
        (Intermittent, Degrading) => DelayedShort { remaining: cfg.degrading_delay },   // 5
        (Intermittent, Recovering)=> DelayedLong { remaining: cfg.recovering_delay },    // 20
    }
}
```

**Scar timing with observation-density scaling:**
```
effective_scar_delay = base_scar_delay * max(1, ceil(sqrt(min_sources / active_sources)))
```

**Scar attribution:**
```rust
enum ScarAttribution {
    SourceConfirmed { source: PeerId },
    PathFault { path: PathId },
    CorrelatedSuspicion { gateway: ClusterId, sources: u8, confidence: u16 }, // Q8.8
    NoAlternatePath { source: PeerId },
    PartitionAbsence { peer: PeerId, window_start: u32, window_end: HlcTimestamp },
    IntegrityFailure { source: PeerId, blob_id: [u8; 32] },
}
```

`PartitionAbsence` excluded from dominance score. `IntegrityFailure` fires when streaming verification (see verkko-matter: MatterIntegrity) detects content that does not match its blob_id.

**Scar mechanics.** Count (u16, not bool). Scar decay is defined in Section 5.3 (Memoria). **Health scar penalty:** `scarred_score = effective_score * (0.9 ^ weighted_scar_count)`.

**Tap 3: Circuit breaker.**

Three states with typestate enforcement (`SourceBreaker<State>`):
- **Closed:** source at full rate.
- **HalfOpen:** triggered at Degraded. One request per N seconds. N scales with health.
- **Open:** triggered at Bad. Source excluded. One probe per minute.

**Two-gate probe condition.** Both HealthDirection Recovering AND Variance below threshold required for probe rate increase.

**Five-tempo recovery (derivation).** Emergent from scar timing + two-gate breaker + auto-clear. Consequence of the monotone-contraction composition: monotone state (GSet scar entries) feeds the contraction map (dual-EMA decay), and the contraction map's output (health classification) drives monotone state transitions (scar creation). The feedback between regions 1 and 2 produces the five tempos as emergent behavior from the region boundary contract.

**The monotone-contraction boundary is loop-free at the type level.** Region 1 (monotone) exports GSet entries. Region 2 (contraction) imports GSet entries and exports health classifications. The dependency appears circular (scar creation feeds back to GSet) but is actually a spiral: each iteration creates new monotone state, never modifying existing state. Creation adds elements; decay reduces their weight. These commute because they operate on different aspects. This spiral structure gives convergence order-independence.

## Path Diagnostics

Three health measurements form a diagnostic triangle for every
inter-cluster connection:

```
PathTriangle {
    direct_health: HealthScore,
    relay_health_normalized: HealthScore,
    wifi_quality: CapacityRatio,
}
```

The relay_health_normalized value is WiFi-normalized:

```
normalized = raw_relay_sack_q16 / wifi_quality_q16
```

Clamped to [0, 0x10000]. This strips the WiFi contribution from relay
path measurement, preventing false relay failover when WiFi is noisy.
Without normalization, a relay with 2% true loss appears to have 17%
loss at 15% WiFi loss; with normalization, relay intrinsic health
reads 0.976.

### PathDiagnosis

8-way classification on three boolean thresholds (threshold: 0.75 in
Q16.16 = 0x0000C000):

| direct | relay | wifi | Diagnosis | Automated Response |
|--------|-------|------|-----------|--------------------|
| good | good | good | AllHealthy | none |
| good | bad | good | RelayFault | failover to warm standby |
| bad | good | good | NatExpired | send NAT keepalive; do NOT trigger convergence |
| bad | bad | good | DestinationFault | scar destination (structural); trigger PARTITION_ANN after threshold |
| good | good | bad | WifiTransient | attenuate scar generation |
| good | bad | bad | WifiPlusRelay | attenuate scars AND failover relay |
| bad | good | bad | WifiPlusDirect | attenuate scars AND renew NAT keepalive |
| bad | bad | bad | LocalLinkDown | halt outgoing; enter degraded mode; delay convergence |

### Temporal Stability

A diagnosis must persist for 3 consecutive evaluations (6-15 seconds)
before triggering automated responses. This prevents transient
diagnoses during EMA convergence from prematurely triggering responses.

### PARTITION_ANN Trigger (resolves OQ-3)

PARTITION_ANN is emitted when DestinationFault persists for >=
partition_threshold seconds AND remediation attempts (NAT keepalive,
relay failover) have failed. This provides a locally-computable,
deterministic trigger condition. The PARTITION_ANN entry is appended
to the GSet control log.

<!-- v10 §5.2 -->
### 5.2 Partition absence and announcement

**PartitionAnnouncement.** Unilateral, write-only control log annotation.

```rust
#[repr(C)]
struct PartitionAnnouncement {
    peer_id: PeerId,           // u32, 4 bytes
    last_known_epoch: u32,     // 4 bytes
    rejoined_at: HlcTimestamp, // 8 bytes
}
// 16 bytes.
```

**Processing with observer quorum:**
```rust
fn attribute_partition(
    announcement: &PartitionAnnouncement,
    observations: &[HealthObservation],
    slow_ema_at_start: HealthScore,  // Q16.16 fixed-point
    expected_observer_count: u8,
    actual_observer_count: u8,
) -> PartitionAttribution {
    if actual_observer_count < expected_observer_count / 2 {
        PartitionAttribution::Deferred
    } else if observations.is_empty() {
        PartitionAttribution::Absence { reset_fast_ema_to: slow_ema_at_start }
    } else {
        PartitionAttribution::ActiveDegradation
    }
}
```

If fewer than half of expected observers have contributed scar entries, the attribution is deferred. Deferred attributions are re-evaluated when additional GSet entries arrive, up to the Front 1 timeout.

<!-- v10 §5.3 -->
### 5.3 Scar provenance, decay, and institutional memory (Memoria)

The scar GSet is the mesh's institutional memory. Scar entries are permanent GSet members: weight decays but existence does not. A mesh with rich scar history routes conservatively around historically unreliable peers, even during healthy periods.

**ScarEntry structure:**
```
ScarEntry {
    target_dk: [u8; 32],
    scar_type: ScarType,
    observer_dk: [u8; 32],
    transit_path_hash: [u8; 8],
    source_cluster_cik: [u8; 8],
    observation_epoch: u64,
}
```

**Scar decay.** Linear decay, applied at read time (not stored):
```
fn scar_weight_q16(scar: &Scar, now_epoch: u64) -> u32 {
    let dt_hours = now_epoch - scar.observation_epoch;
    let decay_hours = scar.decay_days() as u64 * 24;
    let decay_q16 = (dt_hours * 0x10000u64) / decay_hours;
    if decay_q16 >= 0x10000 { 0 }
    else { 0x10000u32 - decay_q16 as u32 }
}

fn decay_days(&self) -> u16 {
    match self.attribution {
        WifiInduced { wifi_quality, .. } => {
            wifi_scar_decay_days(wifi_quality) // 1-7 days
        }
        PathFault { .. } => 30,
        _ => 180,
    }
}
```
weighted_scar_count(peer, now) = sum(scar_weight_q16(s, now) for s in scars_targeting(peer))
At age = 180 days, weight drops from ~0.006 to 0. This cliff goes in the safe direction (peer becomes more eligible for dominance, link becomes healthier in scoring).

**Three consumers (cospan):**
1. **Health scoring** (Section 5.1): `scarred_score = effective_score * (0.9 ^ weighted_scar_count)`. Smooth exponential degradation.
2. **Dominance cascade** (Section 2.3): `(1 + weighted_scar_count) * 2^max(0, weighted_scar_count - 10)`. Hard threshold with exponential penalty.
3. **Gateway ETT** (Section 7.2): tiebreaker in gateway election when delivery ratio is equal.

Each consumer applies its own formula to the same `weighted_scar_count` input. The formulas are intentionally different because the three consumers have different risk profiles.

**CorrelatedSuspicion trigger.** Requires >= 2 scar entries for the same source_cluster_cik, from different observer_dk values, with different transit_path_hash values, within the same observation window.

<!-- v10 §5.4 -->
### 5.4 Observation-path forwarding

**Mechanism.** Every inter-cluster data flow generates a 4-byte health summary:

```
FlowSummary {
    frames_seen: u16,
    auth_failures: u8,
    latency_class: u8,
}
```

Gateway aggregates summaries from all cluster peers. Aggregation: `max(auth_failures)`, `weighted_avg(latency_class, frames_seen)`, `sum(frames_seen)`.

**FlowSummary attribution.** Each FlowSummary is attributed to the gateway PeerId that produced the aggregation. The health EMA tracks `last_update` per attributed source (per gateway PeerId). The time-normalized alpha uses `dt = now - last_update[source_gateway]`, which correctly handles the observation frequency of aggregated observations without requiring a separate aggregation-aware alpha formula.

**Graph-theoretic property.** Gateway is a vertex cover of all inter-cluster observation edges. G_obs >= G_data at the gateway vertex.

**Observation-path invariant.** Every inter-cluster data transfer contributes a health summary within two consecutive intra-cluster heartbeat intervals.

**Gateway handoff.** Old gateway includes aggregated flow summary table in gateway-release entry. Payload: 44 bytes. Zero-observation window eliminated.

<!-- moved from verkko-protocol §3.4 — extension field semantics -->
### Heartbeat extension fields

The heartbeat frame (see verkko-protocol: Heartbeat) carries a fixed transport header and a variable extension region. The extension fields carry mesh-layer semantics:

- `filter_root` (8 bytes): keyed truncated Merkle root for divergence detection.
- `epoch_current` (4 bytes): G-Counter sync (macro-epoch).
- `work_capacity` (1 byte): re-encryption/donation capacity. Computed as `min(bw_kbps/128, flash_kb/256, 255)`.
- `health_canary` (1 byte): top 4 bits = worst classification; bottom 4 bits = degraded count.
- `observation_summary` (4 bytes): aggregated cross-cluster observation (see ObservationPathForwarding).
- `gossip_mode` (bit 2 of state byte): 0 = Plumtree, 1 = flood.

<!-- moved from verkko-crypto §2.6 — PCS distribution logic -->
### PCS trigger and distribution (mesh-layer)

For the PCS key derivation mechanism (HKDF re-seed formula), see verkko-crypto: PCSMechanism.

When a CPSK is renegotiated (on every reconnection, every 24-hour session timeout, and every Noise session expiry), the mesh calls PCSMechanism for each shared channel and distributes the result:

```
on_cpsk_renegotiation(cluster_a, cluster_b, fresh_cpsk):
    for channel in shared_channels(cluster_a, cluster_b):
        K_epoch_new = PCSMechanism(K_epoch_current, fresh_cpsk, channel_id)
        distribute_via_keybundle(channel, K_epoch_new, Rotate)
```

**Cost:**
- Wire: one KeyBundle(Rotate) per re-seed, per affected channel. At 52 bytes/recipient and 36 peers: ~18KB per re-seed event. At ~3 re-seeds/hour (device reconnections): ~54KB/hour.
- Wire (convergence, with chain proofs): 30 bytes per channel per pair, 11 slots * 6 pairs * ~300 bytes = ~20 KB total, spread over 22 seconds (may extend to ~28 seconds with rate limiting).

For PCS trigger scheduling, see Convergence, polygon schedule (Section 3.6), and per-channel rate limiting (Section 2.6).

### Replication Orchestration

Three interlocking components:

**Component 1: Priority Queue (WHAT).** most_under_replicated() selects
the blob with the highest deficit. Deficit is weighted by priority:
P1 at 3.0x, P2 at 2.0x, P3 at 1.0x, P4 at 0.5x. The queue determines
the replication face: the system replicates the most under-replicated,
highest-priority content first.

**Component 2: Probabilistic Roll (WHO).** Each peer independently
decides whether to initiate replication. The roll is deterministic per
(blob_id, my_dk, epoch):

```
let probability_q16_16 = ((deficit as u32) << 16)
    / (known_holders as u32 + 1);
let roll_q16_16 = (BLAKE3("saalis-repl-v1" || blob_id || my_dk
    || epoch)[0] as u32) << 8;
should_replicate = roll_q16_16 < probability_q16_16
```

Fixed-point Q16.16 arithmetic. No floating-point. Coordination-free.
Expected redundant transfers = O(1) regardless of holder count.

**Component 3: Supersaturation Rate (HOW MANY).** Controls concurrent
TransferSessions (not individual chunks). Each TransferSession runs
its own bao streaming at LEDBAT pace.

```
fn replication_sessions_to_open(
    deficit: u8,
    priority: MatterPriority,
    metabolic: MetabolicState,
) -> u8 {
    let target = priority.target_replicas();
    let saturation_q16 = ((deficit as u32) << 16)
        / (target as u32);
    let priority_weight_q16 = match priority {
        P1 => 0x00030000,  // 3.0x
        P2 => 0x00020000,  // 2.0x
        P3 => 0x00010000,  // 1.0x
        P4 => 0x00008000,  // 0.5x
    };
    let weighted = ((saturation_q16 as u64
        * priority_weight_q16 as u64) >> 16)
        .min(0x10000) as u32;
    let base_rate = match metabolic {
        Background => 1u32,
        Idle => 3,
        Active => 2,
        Stress => 1,
    };
    let scaled = (base_rate * weighted) >> 16;
    scaled.max(1) as u8
}
```

Minimum is always 1 session. Budget is NEVER zero in any metabolic
state.

### Replication Gate

REPLICATION_READY = filter_reconciliation_complete AND
convergence_front >= Front 2. The gate prevents replication decisions
on stale filter data. After the gate opens, sessions come online
gradually under Conductor governance.

### Replication Targets

Replication targets (min_replicas, target_replicas) are per-channel
configuration, distributed via the control log as CHANNEL_CONFIG
entries (type 0x0F). Subject to fencing token ordering: the dominant
admin's configuration wins after convergence.

### ResourceGovernance — Conductor specification

The governor (Conductor) drives MetabolicState transitions (see verkko-protocol: MetabolicState). The mesh layer computes `metabolic_state = f(resource_pressures)` and provides the result to the protocol layer.

**Governor inputs (four):**
1. CPU/flash/nonce pressure.
2. WiFi-layer capacity: `MCS_throughput * (1 - retry_fraction)` from nl80211.
3. Transport-layer capacity: LEDBAT available_bw_kbps.
4. Phase jitter on state transitions.

Governor: `min(CPU, flash, nonce_io_pressure, min(WiFi_capacity, transport_capacity))`.

**Conductor invariant (governor-as-sole-coherence-point).** The governor is the sole source of resource-budget state for all resource-sensitive parameters. For every resource-budget parameter `p` (targets, limits, thresholds), there exists a pure function `f_p` such that `p = f_p(metabolic_state)`. No mechanism reads raw resource values to produce budget parameters; all consume the governor's metabolic state output.

Physical observations (e.g., LEDBAT `min_rtt`, `current_delay`) are not resource-budget parameters. They enter through convention-enforced boundary gates independently of the governor. The governor mediates budgets; boundary gates mediate observations.

**Extensibility consequence.** New resource dimensions enter the governor's `min()` without changing the metabolic state machine, LEDBAT parameters, or heartbeat intervals. The downstream parameter maps are unchanged.

**Governed\<T\> witness type.** Resource-budget parameters are wrapped in `Governed<T>`, constructible only inside the governor module. A function that needs a resource-budget parameter takes `Governed<T>`, proving at the type level that the value came from the governor. The `MetabolicEpoch` field inside `Governed` detects stale governor outputs.

### Conductor Threshold Table

The governor maps the min-projection pressure to MetabolicState via a
threshold table with hysteresis. All values are Q16.16 fixed-point.

```rust
struct GovernorThresholds {
    thresholds: [u32; 3],    // Background->Idle, Idle->Active, Active->Stress
    hysteresis: u32,
}

const DEFAULT_THRESHOLDS: GovernorThresholds = GovernorThresholds {
    thresholds: [
        0x00003333,  // 0.20 -- Background -> Idle
        0x00008000,  // 0.50 -- Idle -> Active
        0x0000CCCC,  // 0.80 -- Active -> Stress
    ],
    hysteresis: 0x00000CCC,  // 0.05
};
```

### Governor Output

The governor output carries both state and bottleneck type:

```rust
struct GovernorOutput {
    state: MetabolicState,
    bottleneck: ResourceType,
    pressure: CapacityRatio,
}
```

The consumer's response table is two-dimensional:

| State | CPU bottleneck | WiFi bottleneck |
|-------|----------------|-----------------|
| Active | heartbeat 2s, repl sessions 2 | heartbeat 5s, repl sessions 2 |
| Stress | heartbeat 5s, repl sessions 1 | heartbeat 10s, repl sessions 1 |

WiFi bottleneck widens heartbeat intervals more aggressively (reduce
traffic on lossy link). Replication session count is unchanged
(replication is not CPU-bound when WiFi is the bottleneck).

### Convergence Override

During convergence, the governor bypasses thresholds:
`return max(candidate, MetabolicState::Active)`. Convergence exit
(Step 3) sets last_transition_ms = now_ms when releasing the override,
ensuring dwell time applies to the post-convergence transition.

### Calibration

The three threshold values and hysteresis margin are calibration items
(extending the existing 16-item calibration table in verkko-ops). They
should be empirically tuned on target hardware (Batocera/Pi 4/5).

<!-- v10 §6.1 -->
### 6.1 Resource management: GovernedResource\<P\> and ThresholdGuard\<P\>

GovernedResource and ThresholdGuard remain separate types. GovernedResource is a stateful closed-loop PI controller. ThresholdGuard is a stateless open-loop comparator. Forcing these into one abstraction conflates their operational semantics. See verkko-ops: Pattern Registry, Structure 6.

```
GovernedResource<P: Policy> {
    capacity: P::Capacity,
    consumed: P::Counter,
    ratio: f64,
    thresholds: [Threshold; 4],
    responses: [Response; 4],
    governor_input: fn(&self) -> f64,
}

ThresholdGuard<P: Policy> {
    capacity: P::Capacity,
    consumed: P::Counter,
    threshold: P::Limit,
    response: fn(&self) -> Action,
}
```

**GovernedResource instances:**

**Flash budget.** Rolling 24-hour window (24 slots, 96 bytes). Dynamic WAL subtraction. Diminuendo: linear from full rate at 50% consumed to 20% rate at 80%. P1 exempt. Admin notification at 10% reserve.

**WritePermit resource token.** Every flash write requires a WritePermit. Unaccounted writes prevented at compile time.

**CPU budget.** PI controller with anti-windup (Kp=2.0, Ki=0.1, 4 bytes). Anti-windup: integrator clamped to MAX_INTEGRATOR = 10.0.

**Adaptive setpoint:**
```
setpoint = base_setpoint * capacity_ratio
capacity_ratio = EMA(observed_throughput / expected_throughput, alpha=0.05)
```

**CapacityRatio guard:** `if expected_throughput_q16_16 == 0 { return 0x00010000u32; }`. This prevents division by zero. In Q16.16, the zero check is an integer comparison.

**ThresholdGuard instances:**

**Memory arenas.** Pre-allocate 5% of physical RAM at startup. 512MB target: 25.6MB hard cap.

| Arena | Size (512MB) | Allocator | Purpose |
|-------|-------------|-----------|---------|
| Hot arena | 64KB | Bump | Per-frame transient |
| Pool arena | 256KB | Fixed-block | Per-epoch transient, dual-use data-pending/Raptor assembly |
| Managed arena | ~199KB baseline | System (budgeted) | Long-lived state |

**Mandatory mlock check.** At startup, verify RLIMIT_MEMLOCK >= 3MB. If insufficient, abort with actionable error message. No fallback.

**Managed arena steady state:**

| Component | Bytes |
|-----------|-------|
| Health scores (dual EMA + scars, 35 peers) | 630 |
| Capacity-weighted hash ring (32 vnodes/peer) | 13,900 |
| Merkle tree caches (11 filters) | 180,000 |
| Replay window bitmaps (11 sessions) | 1,400 |
| Donation progress counters | 144 |
| Nonce high-water marks + velocity + burst_rate | 120 |
| Scar count cache | 70 |
| Lamport clocks | 96 |
| Congestion feedback state | 44 |
| Day-of-week anomaly baselines | 1,400 |
| Structured log state | 16 |
| Sealed range state (35 peers) | 560 |
| Observation-path flow summaries (11 connections) | 44 |
| Reconnection latency history (66 pairs) | 528 |
| Flood cooldown state (12 gateways) | 96 |
| Recent IHAVE set (ring buffer, 100 entries) | 800 |
| **Total** | **~200 KB** |

<!-- v10 §7.1 -->
### 7.1 Convergence: entry, three fronts, wavefront gating, and exit

Unified convergence specification. Entry gates, three sequenced fronts, wavefront-aware gating, and a 6-step exit sequence. Post-partition convergence has self-stabilizing properties [Dijkstra1974]: regardless of the state the system is in when the partition heals, the convergence protocol drives it toward a consistent global state.

#### Entry

First CPSK reconnection starts the wavefront timer. Gossip mode switches to flood. Metabolic override forces Active/Stress (see verkko-protocol: MetabolicState).

**Wavefront timer:**
```
wan_rtt = inter_cluster_rtt - median(intra_cluster_rtts)
wavefront_deadline = max(
    5 * max(wan_rtt_estimates),
    p95_reconnect_latency * 1.5,
    10_seconds
)
```

Three terms, each from a different analysis domain:
- **WAN RTT term:** strips WiFi-layer RTT inflation. The 5x multiplier accounts for 2-message Noise KK handshake (1 RTT) with potential fallback to 3-message Noise XX (2 RTTs) plus one application-level retry (1 RTT) plus headroom for NAT hole-punch latency.
- **p95 reconnect latency term:** empirically adaptive, tracks 95th percentile across past convergence events. Cost: 528 bytes for 66 pairs.
- **10-second floor:** handles unknown factors.

**Convergence deadline on constrained links.** On slow uplinks (256 Kbps), convergence traffic can saturate the link, causing RTT inflation that lengthens the wavefront deadline formula. The 70/30 vital fair queuing (see verkko-protocol: VitalQueue) ensures CPSK handshakes complete within their polygon schedule round budget (23ms queuing delay) even under gossip traffic. The wavefront deadline uses historical (pre-convergence) RTT estimates, not live measurements during convergence. This prevents the circular dependency where saturated-link RTT inflates the deadline formula. If a convergence event exceeds the deadline, late-arriving pairs are treated as still-partitioned and converge in a subsequent cycle.

Front 1 waits until the wavefront deadline expires or all expected pairs reconnect.

#### Front 1: Control

| Step | Action |
|------|--------|
| 1 | Control log merge (GSet union) |
| 2 | PartitionAnnouncement processing (with observer quorum) |
| 3 | Health EMA reset for partition-absent peers |
| 4 | Observation replay: local-cluster scar entries, per-observer, stale cutoff respected |
| 5 | Key retrieval for rotated channels |
| 6 | P1-P2 re-encryption (speculative P1 at gateway, concurrent) |
| Gate | All control entries processed, all reachable keys obtained, EMA resets applied |
| Timeout | Orphan unretrievable channels after 5 minutes. Apply EMA resets even if key retrieval incomplete. |

### Filter Reconciliation (Front 1.5)

Filter reconciliation begins after ControlGate opens (Front 1
complete). Runs concurrently with Fronts 2 and 3. Merkle delta exchange
with each reconnected cluster pair. Completes asynchronously. No
blocking gate.

Initial delta exchange expected within 2 heartbeat intervals (~10
seconds at Active). Remaining divergence resolves via steady-state
anti-entropy. Filter reconciliation does not depend on key state
finalization because filters advertise blob_ids (content hashes),
not encrypted content.

The convergence exit sequence Step 5 (update health baselines) waits
for the initial Merkle delta exchange to complete, ensuring health
baselines are computed with accurate filter state.

#### Front 2: Consensus

| Step | Action |
|------|--------|
| 1 | Revocations before grants (RevocationsApplied witness) |
| 2 | Fencing token resolution (grace restriction: Revoke only for stale tokens) |
| 3 | Dominance recomputation (multiplication formula, argmin) |
| 4 | Hash ring recomputation with ring coloring constraint |
| Gate | Highest fencing token confirmed, dominance resolved |
| Timeout | Highest fencing token wins after 10 minutes |

#### Front 3: Bulk re-encryption

Re-encryption performed per verkko-relay: TerritorialReencryption. Gate: all affected content re-encrypted or epoch-forwarded. Blocked until: Consensus gate open AND vital queue depth < 25%.

#### Convergence sequencing (ordered phases)

1. **Wavefront gate.** First CPSK reconnection starts the wavefront timer. Gossip mode switches to flood. Front 1 waits until deadline expires or all expected pairs reconnect. Duration: up to 10 seconds typical.

2. **Edge-colored renegotiation schedule.** Starts at wavefront deadline. Polygon round-robin [Lucas1883]: 11+ slots (extendable by rate limiter), 6 pairs per slot, one per gateway. Slot spacing: `max(3 * wan_rtt_estimate, 2_seconds)`. Duration: 22-28 seconds. Sealed-range catch-up completes before CPSK re-seed for each pair (see verkko-matter: SealedRange).

3. **Speculative P1 re-encryption.** Begins at wavefront deadline (concurrent with step 2). Relay performs speculative P1 re-encryption; see verkko-relay: TerritorialReencryption, convergence re-encryption.

4. **Front 2 epoch snapshot.** Runs concurrently with steps 2-3. Hash ring and territorial assignment are deterministic. Ring coloring constraint enforced.

5. **Front 3 bulk re-encryption.** Begins after Front 2 gate opens and vital queue depth < 25%.

6. **Exit transition (6-step gated sequence).**

#### Exit Transition

Gate 0: All three convergence fronts report COMPLETE. Metabolic override remains Active/Stress.

**Step 1: Gossip mode switch-back.** Admin broadcasts gossip_mode=0 (switch to Plumtree). Gate 1: All peers ACK gossip_mode=0 within wavefront_timer. Fallback: If timer expires without unanimity, retry broadcast. After 3 retries, admin forces transition and logs a scar for non-ACK peers.

**Step 2: Spanning tree construction.** Gate 2: Spanning tree is connected (all peers have at least one eager parent). Deterministic from ring coloring; computed locally.

**Step 3: Restore metabolic dwell time enforcement.** Metabolic state transitions are once again subject to dwell_time_s. Flag flip on the metabolic SM. No gate needed.

**Step 4: Re-enable phase jitter with anti-synchronization.** Each peer computes its jitter from deterministic_jitter() with its DK_pub. Jitter values are staggered by construction. No coordination needed. Steps 3 and 4 execute in parallel.

**Step 5: Update health baselines.** Each peer computes new health EMA from post-convergence observations. Minimum observation window: 3 heartbeat rounds at Active interval. This is the bottleneck step.

**Step 6: Ring coloring verification.** Gate 6: Ring checksum matches expected (computed from converged state). Continuous enforcement resumes.

Sequencing: 0 -> 1 -> 2 -> (3, 4 in parallel) -> 5 -> 6.

#### Convergence duration bound

    convergence_deadline = max(120_seconds, configurable_deadline)
    max_configurable_deadline = 600_seconds

On deadline expiry:
1. All open ConvergenceGates are forced open.
2. Incomplete fronts are logged.
3. Exit sequence begins immediately.
4. Incomplete re-encryption continues as steady-state work.
5. Seats are transitioned back to CONSISTENT mode.

**Total convergence duration:** 10s wavefront + 22-28s polygon + 5s stabilization = 37-43 seconds. During this window, flood dissemination ensures maximum reliability.

**Convergence burst cost (worst case, 12 clusters):**

| Phase | Duration | WAN cost |
|-------|----------|----------|
| Wavefront (flood gossip) | 10 seconds | ~130 KB (2 HB rounds * 132 msgs * 91 B + control log merge ~76 KB) |
| Polygon schedule + re-seed proofs | 22-28 seconds | ~22 KB (11+ proofs * 11 gateways * ~180 B) |
| Filter reconciliation (Merkle) | concurrent | ~720 KB (36 pairs * 20 KB) |
| Stabilization (flood) | 5 seconds | ~26 KB |
| **Total** | **37-43 seconds** | **~900 KB** |

Pre-optimization estimate: ~67 MB. Reduction factor: **74x**.

<!-- v10 §7.2 -->
### 7.2 Gateway election

Max-register CRDT. No explicit election. **Primary criterion:** availability. **Tiebreaker:** ETT [Draves2004] from receiver-observed delivery ratios, with scar Memoria adjustment (Section 5.3).

**ETT computation.** Expected Transmission Time = frame_size / (delivery_ratio * bandwidth). The delivery ratio uses receiver-observed SACK measurements.

**Receiver-observed metric aggregation.** Median of pairwise observations from cluster peers. Robust to < 50% Byzantine reporters.

**Hysteresis:** 10% margin for 3 consecutive heartbeat intervals. **Handoff:** gateway-claim on LAN, gateway-release with CPSK session state (~200 bytes) plus aggregated observation-path flow summaries (44 bytes).

<!-- v10 §8 — health scoring costs -->
### Cost summary (health scoring)

| Operation | Total |
|-----------|-------|
| Health update (22 obs) | ~5us |
| Scar provenance BLAKE3 | ~30ns/scar |
| Observation-path aggregation | ~50ns/heartbeat |
| Ring coloring check | ~5us/heartbeat |
| Spanning tree edge-swap optimization | ~2us/recompute |
| CPSK re-seed HKDF | ~1.25us |
| Deferred reconciliation | ~256us |
| **Per second (amortized)** | **6-60us** |

<!-- v10 §9 -->
### Formal Foundation

#### Four-reservoir model

| Reservoir | State variable | Convergence | Stability |
|-----------|---------------|-------------|-----------|
| Key space | Epoch vector + micro-epoch | Lattice join + PCS re-seed (see verkko-crypto: PCSMechanism) | Asymptotic |
| Trust space | Health vector + scar provenance | EMA contraction, PhysicalBoundary-gated | Asymptotic |
| Content space | Cuckoo filter state/cluster | Full-filter-sync / incremental Merkle (see verkko-matter: MerkleReconciliation) | Asymptotic |
| Kernel space | CPU, memory, I/O bandwidth | Entropic governor (PI with anti-windup) | ISS-stable |

#### Consistency model

The system is AP (available and partition-tolerant) in the CAP theorem [Gilbert2002] classification: during network partitions, each partition continues to operate independently with eventual consistency [Bailis2013, Viotti2016] upon reconnection. Strong consistency is not required and not provided.

| Domain | Isolation | Mechanism |
|--------|----------|-----------|
| Control log | Causal within scope, total via Lamport + dominance | CRDT GSet [Shapiro2011], hash-linked |
| Key state | Single-writer with fencing token (grace: Revoke only) | Existing sequence field |
| Content routing | Eventual with periodic anti-entropy [Demers1987] | Keyed Merkle root, incremental reconciliation |
| Health scores | Local-only, PhysicalBoundary-gated, per-observer replay | Time-normalized dual EMA, observation-path forwarding |
| Resources | Proportional control, ISS-stable | PI governor with anti-windup |

#### Partition vs peer failure (design decision)

verkko does not distinguish partition from peer failure. Both
are handled by the same recovery path: heartbeat timeout
triggers degraded health scoring, reconnection triggers
sealed-range catch-up or fast-forward (see verkko-matter:
SealedRange), and convergence heals state. The dominance
cascade and CRDT control log are designed for eventual
consistency after partition healing.

The PARTITION_ANN control log entry (type 0x08) records the
observation that a peer was unreachable. It does not diagnose
why. The PartitionAttribution algorithm uses observer quorum
to classify the absence as Absence or ActiveDegradation, not
to distinguish "partitioned" from "offline."

Rationale: in a household mesh (2-12 clusters, ~36 peers),
"I am partitioned from everyone" and "everyone else is offline"
have identical recovery paths. Adding a partition detector
would add complexity for zero behavioral difference.

#### Verification Frame invariant

The PhysicalBoundary input gates (Section 2.0a + convention-enforced boundary gates) and the DomainSeparator output registrations (see verkko-crypto: DomainSeparatorRegistry) together bound the abstract state machine. If both registries are complete (all physical/network/temporal inputs gated, all peer-agreement-critical outputs registered), then correctness of the intermediate computation reduces to verifying the seven algebraic structures (see verkko-ops: Pattern Registry) via property-based tests plus the six TypestateChain instances via compile-time enforcement.

#### Invariants (8)

| # | Invariant | Status | Enforcement mechanism | Conditions |
|---|-----------|--------|-----------------------|------------|
| 1 | **Key freshness** (tiered) | Enforced | Micro-epoch + per-frame derivation + CPSK-seeded PCS. | P4 acknowledged not re-encrypted. |
| 2 | **Gateway singularity** (per-component) | Enforced | CPSK epoch binding. | Per-connected-component. |
| 3 | **Admin authority** | Enforced | Grace restricted to Revoke only. Multiplication formula with argmin. | ProVerif pre-implementation deliverable. |
| 4 | **Nonce uniqueness** | Enforced | NonceSourceFactory (CE #18). Double-write persist with CRC-32C (see verkko-crypto: NonceSafety). | Fail-stop on double corruption. |
| 5 | **Filter-stash consistency** | Partially enforced | Control floor in HLC. Incremental Merkle. | Convergence time proportional to gossip diameter. Irreducible. |
| 6 | **Territorial completeness** | Enforced | Retain-until-handoff + deterministic nonces + ring coloring (see verkko-relay: TerritorialReencryption). | Relay idempotent-PUT contract. |
| 7 | **Cascade boundedness** | Partially enforced | Cooperative load shedding with max_absorption. Anti-windup PI. | Bounded to EMA tracking error. Irreducible. |
| 8 | **Donation idempotence** | Enforced | Deterministic nonces + relay idempotent PUT (see verkko-relay: IdempotentPUT). | Deterministic nonces require identical channel_key_new. |

#### Axioms (7)

1. Cryptographic primitives are ideal (IND-CCA2, PRF, collision-resistant, EUF-CMA).
2. Fair scheduling: every message on non-failed link is eventually delivered.
3. Bounded partition duration.
4. Honest admins.
5. Bounded clock skew (1 second).
6. Finite flash endurance.
7. Independent task structure (blob re-encryption tasks).

For TLA+ verification plan, see verkko-ops.

For Compile-Time Enforcement table (24 points), see verkko-ops.

<!-- v10 §9.1-9.3 -->
### Design Constraints

#### 9.1 Design envelope: 12-cluster ceiling

Pairwise CPSKs required for the no-intermediary invariant. At 12 clusters: C(12,2) = 66 session keys.

**Three-tier scaling path:**
1. Second independent mesh (zero implementation cost).
2. Selective channel bridging (future work).
3. CIK-bridged CPSK (long-term protocol extension).

#### 9.2 Filter-stash convergence bound (Invariant 5)

Eventually consistent [Bailis2013] with convergence time proportional to gossip diameter. In a well-connected mesh: 2-3 gossip rounds (10-30 seconds at active heartbeat intervals). Incremental Merkle reconciliation reduces convergence data by ~98%.

#### 9.3 Cascade boundedness approximation (Invariant 7)

EMA tracking error bounded by `alpha * max_deviation`. At typical hardware stability: < 5% of capacity. Worst-case recovery time (K=3, N=5): 252 seconds.

<!-- v10 §10 -->
### Open Design Questions

#### 10.1 ProVerif formal verification

660 lines of ProVerif [Blanchet2001, Blanchet2008] across five domains and three sub-compositions. One week of work.

#### 10.2 DK encryption at rest

Encrypt DK at rest with key derived from user-provided PIN/passphrase via Argon2id. Headless devices: hardware-bound secret.

#### 10.3 LKH tree management under EDT

Temporary leaves appended to rightmost branch. Lazy rebalancing when imbalance > 2.

<!-- v10 Emergent Mechanisms -->
### Emergent Mechanisms (derivations)

These behaviors emerge from designed mechanism interactions. Verified by integration test and game day exercise. Each is a consequence of named mechanisms; no re-explanations here.

**Breathing pattern.** Flash budget inhales (active: CPU-bound) and exhales (background: flash-bound). Conductor selects binding constraint. Phase jitter makes the diurnal cycle an overlapping wave.

**Directional trust.** Signed gap gives direction. Product type preserves both axes. Scar responds with asymmetric patience (20/5 obs). Breaker responds with conditional eagerness (both gates). Five-tempo recovery sequence.

**Causal gaps.** PartitionAnnouncement converts absence into causal information with observer quorum. Processing order enforced by convergence typestate. Dominance cascade undisrupted by partition artifacts.

**Nonce safety feedback loop.** Velocity -> adaptive persist (double-write) -> flash I/O pressure -> governor throttle -> lower velocity. Self-regulating. Governor reads I/O pressure, not velocity.

**PCS healing.** CPSK renegotiation triggers epoch re-seed (see verkko-crypto: PCSMechanism). Multiple pairs renegotiating via polygon schedule create overlapping PCS windows. Peak load smoothed to 1/11th of unscheduled burst, with per-channel rate limiting preventing buffer overflow.

**Deterministic convergence.** Post-partition, both partitions re-encrypting the same blob produce identical relay state. No coordinator needed.

**Gossip mode adaptation.** The mesh adapts gossip dissemination to algebraic connectivity [Fiedler1973]. Strong graph: Plumtree efficiency. Weak graph: flood reliability. Anomalous-message detection provides immediate Byzantine tree-corruption defense.

**Observation completeness.** Observation-path forwarding ensures gateway vertex cover of all inter-cluster observation edges, independent of data path shape.

**Convergence as steady state at higher pressure.** Convergence uses the same mechanisms as steady state. The rate limiter applies the same buffer constraint. Observation replay uses the same time-normalized alpha (with explicit hard_reset for stale observations). Weighted fair queuing uses the same 70/30 split. PhysicalBoundary gates do not distinguish convergence from steady state.

**Conductor extensibility.** New resource dimensions enter the governor's `min()` without growing the metabolic state machine (see verkko-protocol: MetabolicState). Downstream parameter maps are unchanged.

**Institutional memory.** A mesh with accumulated scars routes conservatively around historically unreliable peers, even during healthy periods. The Memoria (Section 5.3) produces this behavior from GSet persistence and decay-weighted scoring.

For VoiceSpec crate structure and build order, see verkko-ops.

For Calibration Items (16), see verkko-ops.

For Game Day Exercises (12), see verkko-ops.

For Rejected Proposals, see verkko-ops.

**Flash (daily, eMMC)**

Dynamic WAL subtraction resolves 77% margin. Diminuendo curve. P1 exempt. 20% reserve. Nonce persist: double-write with CRC-32C, 48 bytes per copy, two copies in separate sectors.

## References

[Anderson1997] Anderson, I. (1997). *Combinatorial Designs and Tournaments.* Oxford University Press.

[Bailis2013] Bailis, P., Ghodsi, A. (2013). "Eventual Consistency Today: Limitations, Extensions, and Beyond." ACM Queue 11(3).

[Banach1922] Banach, S. (1922). "Sur les operations dans les ensembles abstraits et leur application aux equations integrales." Fundamenta Mathematicae 3, 133-181.

[Blanchet2001] Blanchet, B. (2001). "An Efficient Cryptographic Protocol Verifier Based on Prolog Rules." CSFW 2001.

[Blanchet2008] Blanchet, B. (2008). "A Computationally Sound Mechanized Prover for Security Protocols." IEEE TDSC 5(4).

[Boyd2006] Boyd, S., Ghosh, A., Prabhakar, B., Shah, D. (2006). "Randomized Gossip Algorithms." IEEE Trans. Info. Theory 52(6), 2508-2530.

[Chandra1996] Chandra, T.D., Toueg, S. (1996). "Unreliable Failure Detectors for Reliable Distributed Systems." JACM 43(2), 225-267.

[Demers1987] Demers, A., Greene, D., Hauser, C., Irish, W., Larson, J., Shenker, S., Sturgis, H., Swinehart, D., Terry, D. (1987). "Epidemic Algorithms for Replicated Database Maintenance." PODC 1987.

[Dijkstra1974] Dijkstra, E.W. (1974). "Self-Stabilizing Systems in Spite of Distributed Control." CACM 17(11), 643-644.

[Douceur2002] Douceur, J.R. (2002). "The Sybil Attack." IPTPS 2002, LNCS 2429, 251-260.

[Draves2004] Draves, R., Padhye, J., Zill, B. (2004). "Routing in Multi-Radio, Multi-Hop Wireless Mesh Networks." MobiCom 2004.

[Fiedler1973] Fiedler, M. (1973). "Algebraic Connectivity of Graphs." Czechoslovak Math. J. 23(2), 298-305.

[FLP1985] Fischer, M.J., Lynch, N.A., Paterson, M.S. (1985). "Impossibility of Distributed Consensus with One Faulty Process." JACM 32(2), 374-382.

[Gilbert2002] Gilbert, S., Lynch, N. (2002). "Brewer's Conjecture and the Feasibility of Consistent, Available, Partition-Tolerant Web Services." ACM SIGACT News 33(2), 51-59.

[Karp2000] Karp, R., Schindelhauer, C., Shenker, S., Vocking, B. (2000). "Randomized Rumor Spreading." FOCS 2000.

[Kulkarni2014] Kulkarni, S., Demirbas, M., Madappa, D., Avva, B., Leone, M. (2014). "Logical Physical Clocks and Consistent Snapshots in Globally Distributed Databases." OPODIS 2014.

[Lamport1978] Lamport, L. (1978). "Time, Clocks, and the Ordering of Events in a Distributed System." CACM 21(7), 558-565.

[Lamport2002] Lamport, L. (2002). *Specifying Systems: The TLA+ Language and Tools for Hardware and Software Engineers.* Addison-Wesley.

[Leitao2007] Leitao, J., Pereira, J., Rodrigues, L. (2007). "Epidemic Broadcast Trees." SRDS 2007.

[Lucas1883] Lucas, E. (1883). *Recreations Mathematiques,* Vol. 2. Gauthier-Villars.

[RFC2018] Mathis, M., Mahdavi, J., Floyd, S., Romanow, A. (1996). "TCP Selective Acknowledgment Options." RFC 2018.

[RFC2627] Wallner, D., Harder, E., Agee, R. (1999). "Key Management for Multicast: Issues and Architectures." RFC 2627.

[RFC6189] Zimmermann, P., Johnston, A., Callas, J. (2011). "ZRTP: Media Path Key Agreement for Unicast Secure RTP." RFC 6189.

[Shapiro2011] Shapiro, M., Preguica, N., Baquero, C., Zawirski, M. (2011). "Conflict-free Replicated Data Types." SSS 2011 / INRIA RR-7506.

[Viotti2016] Viotti, P., Vukolic, M. (2016). "Consistency in Non-Transactional Distributed Storage Systems." ACM Computing Surveys 49(1).

[Vizing1964] Vizing, V.G. (1964). "On an Estimate of the Chromatic Class of a p-Graph." Diskretnyj Analiz 3, 25-30.

[Wallis2007] Wallis, W.D. (2007). *Introduction to Combinatorial Designs* (2nd ed.). Chapman & Hall/CRC.

[Welford1962] Welford, B.P. (1962). "Note on a Method for Calculating Corrected Sums of Squares and Products." Technometrics 4(3), 419-420.

[Wong1998] Wong, C.K., Gouda, M., Lam, S.S. (1998). "Secure Group Communications Using Key Graphs." IEEE/ACM Trans. Networking.
