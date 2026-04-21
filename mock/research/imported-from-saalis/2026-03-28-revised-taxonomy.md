# Revised Verkko Taxonomy

**Date:** 2026-03-28
**Author:** Refactoring/migration specialist
**Inputs:** Six design documents + three expert reviews
**Goal:** Eliminate all bidirectional dependencies; strict DAG

---

## 1. Diagnosis

### 1.1 verkko-crypto contains mesh-layer bootstrap and distribution

**Genesis bootstrap (Section 1.3).** The crypto document defines "CRDTs
at zero," "one control log entry (self-enrollment)," "epoch 0," and
"metabolic SM in Background." These are mesh coordination concepts.
The crypto document has no dependency on mesh, yet it initializes mesh
state.

**Why it exists.** Genesis was written as a single narrative: generate
keys, then set up the node. The key generation and the founding
transcript hash are genuine crypto operations. Everything after step 4
(initial CRDT state, control log, metabolic state) is mesh
initialization that happens to follow key generation chronologically.
The concept confusion is temporal adjacency mistaken for conceptual
unity.

**Actual dependency direction.** Mesh depends on crypto. The mesh
bootstrap consumes crypto primitives (DK, DEK, CIK generation,
founding transcript hash). Crypto does not need to know what the mesh
does with those keys.

**Device recovery flow (Section 1.4).** References enrollment, SAS
verification, KeyBundle delivery, filter sync, admin revocation, and
re-encryption. Five of these seven steps are mesh concepts. Only "generate
new DK + DEK" and "Noise XX handshake with PSK" are crypto.

**Why it exists.** Same narrative conflation. Recovery was written as a
user story (phone breaks, what happens) rather than decomposed by
layer.

**PCS distribution (Section 2.6).** The `on_cpsk_renegotiation`
pseudocode calls `distribute_via_keybundle` and iterates
`shared_channels(cluster_a, cluster_b)`. KeyBundle is a mesh concept.
Shared channels require cluster topology knowledge.

**Why it exists.** PCS straddles two layers: the HKDF re-seed math is
crypto, but the trigger (CPSK renegotiation) and distribution (KeyBundle)
are mesh scheduling. The v10 document described the complete flow in one
place. The decomposition split the math into crypto but left the
distribution call in the pseudocode.

**Actual dependency direction.** Crypto defines the derivation:
`K_epoch_new = f(K_epoch_current, fresh_cpsk, channel_id)`. Mesh calls
that derivation when scheduling dictates, then distributes via
KeyBundle.

### 1.2 verkko-protocol's MetabolicState depends upward on verkko-mesh

**The violation.** verkko-protocol defines MetabolicState as its own
concept, but the body says the governor's resource-budget parameters
are "derived from the Conductor (see verkko-mesh: ResourceGovernance)."
This is an upward dependency: protocol (lower layer) depends on mesh
(higher layer).

**Why it exists.** The metabolic state machine has two parts:
1. The state machine itself (four states, dwell time, phase jitter,
   heartbeat interval mapping). This is a protocol-layer concept: it
   governs how fast heartbeats are sent, how aggressive congestion
   control is.
2. The governor that decides transitions (CPU pressure, flash budget,
   WiFi capacity, nonce I/O pressure). The governor consumes mesh-layer
   resource governance to produce state transitions.

These were combined because the governor and the state machine are
tightly coupled in the v10 implementation. But they are conceptually
separable: the state machine is a consumer of "what metabolic state
am I in," and the governor is the producer.

**Actual dependency direction.** The metabolic state machine belongs in
the protocol layer (it parameterizes heartbeat interval, LEDBAT targets,
queue behavior). The governor that drives transitions belongs in the mesh
layer (it reads resource pressures that are mesh-level concerns). The
protocol layer exports the MetabolicState type and its parameter maps.
The mesh layer imports MetabolicState and drives transitions via the
governor.

### 1.3 verkko-mesh references relay and matter concepts in its body

**Relay references.** Convergence Front 3 describes "bulk re-encryption
under new keys" with "flash budget moderation" and "deterministic
nonces." Section 7.1 references "speculative P1 re-encryption" with
airtime caps. The VoiceSpec crate structure includes re-encryption as
an integral part. The Mechanism Reference Table and Invariants table
reference verkko-relay concepts. The Boundary Object Table includes
"Re-encryption batch" with relay details.

**Matter references.** The four-reservoir model includes "Content space"
with cuckoo filters. The body references filter sync in the post-entry
checklist, filter root in heartbeat semantics, filter reconciliation
in convergence cost tables. Invariant 5 ("filter-stash consistency")
is a mesh invariant that depends on matter-layer concepts.

**Why they exist.** Convergence is an orchestration protocol that
touches every layer. The mesh document describes the complete
convergence flow, which necessarily references what happens at the relay
layer (re-encryption) and matter layer (filter reconciliation). The
mesh document became the "integration document" by default because
convergence lives there.

**Actual dependency direction.** Mesh should orchestrate convergence
by gating phases and signaling events. Relay should own re-encryption
behavior (triggered by mesh signals). Matter should own filter
reconciliation (triggered by mesh signals). The mesh document should
reference these as "relay performs re-encryption per verkko-relay:
TerritorialReencryption" without describing airtime caps or
deterministic nonce internals.

The undeclared bidirectional dependencies (mesh -> relay, mesh -> matter)
are orchestration references, not true conceptual dependencies. The mesh
does not need relay or matter concepts to define its own concepts
(Cluster, Enrollment, KeyBundle, etc.). It references them only in the
convergence body, which is an integration narrative.

### 1.4 Thirteen informal concepts that need formalization

The API designer identified 13 concepts used across documents without
appearing in any Defined Concepts section. Each one needs a home.

| Concept | Why informal | Correct home | Rationale |
|---------|-------------|-------------|-----------|
| Channel | Assumed as background knowledge from v10 | verkko-mesh | Channel is an organizational unit of the mesh; access control, KeyBundle, and epoch ratchet all operate per-channel |
| PhysicalBoundary | Described in mesh Section 2.0a but not promoted | verkko-mesh | It is the shared trait for f64 newtypes; mesh defines it, protocol and relay consume it |
| ControlLog | Described in mesh Section 1.7 | verkko-mesh | Backbone of trust state; enrollment, dominance, convergence all depend on it |
| Conductor | Named in mesh terminology, used in protocol | verkko-mesh | Governor's meet projection; mesh owns resource governance |
| Memoria | Named in mesh Section 5.3 | verkko-mesh | Named composition of scar GSet with decay; three consumers are all mesh-layer |
| FencingToken | Used throughout KeyBundle and convergence | verkko-mesh | Ordering mechanism for admin authority |
| ObservationPathForwarding | mesh Section 5.4, referenced by protocol | verkko-mesh | Protocol references it by name; must exist as a defined concept |
| FoundingHash | crypto Section 1.3 | verkko-crypto | Mesh identity, deterministic from founding transcript |
| BlobId | matter body, relay body | verkko-matter | Content identity; used for re-encryption nonces across documents |
| SACK | protocol body, mesh health pipeline | verkko-protocol | Input to health scoring and congestion; needs a defined contract |
| ConnectionMigration | protocol body | verkko-protocol | Significant protocol behavior, PATH_CHALLENGE/PATH_RESPONSE |
| 0-RTT Resumption | taxonomy scope, seat SeatSession | verkko-protocol | Listed as in-scope but never defined |
| CIK (in crypto) | Generated in crypto genesis, defined in mesh | Keep in verkko-mesh only | CIK is a cluster coordination concept; crypto provides the X25519 primitive (DEK), mesh defines the organizational key |

---

## 2. The Revised Taxonomy

### 2.1 Proposed document structure: seven documents

The current six documents are mostly correct. The primary structural
change is extracting a seventh document, **verkko-ops**, to absorb
implementation planning, calibration, and integration narratives that
currently bloat verkko-mesh.

Additionally, convergence orchestration references to relay and matter
are restructured so that the mesh document orchestrates via signals
and gates, while the downstream documents own their own behavior
during convergence.

#### Document 1: verkko-crypto

**Purpose.** Cryptographic primitives: key types, derivation, AEAD,
nonce safety, post-compromise security mechanism, domain separators.

**Defined Concepts:**
- DK, DEK, ChannelKey, EpochRatchet, MicroEpochRatchet, FrameKey
- TransportAEAD, StorageAEAD
- NonceSafety, DoubleWritePersist
- BroadcastEncryption, PCSMechanism
- DomainSeparatorRegistry, KeyZeroization
- FoundingHash (new: BLAKE3 of founding transcript; mesh identity)

**Depends on:** Nothing.

**Moved IN:**
- FoundingHash promoted from informal body concept to defined concept.

**Moved OUT:**
- Genesis bootstrap steps 5-6 (CRDT init, control log, metabolic state) -> verkko-mesh
- Device recovery steps 2-7 (enrollment, SAS, KeyBundle, filter sync) -> verkko-mesh
- PCS Section 2.6 distribution pseudocode (`distribute_via_keybundle`, `shared_channels` loop) -> verkko-mesh
- Channel concept references replaced with abstract "per-key-scope" language

#### Document 2: verkko-protocol

**Purpose.** Point-to-point communication: wire format, sessions,
congestion control, heartbeat framing, discovery, NAT traversal.

**Defined Concepts:**
- Frame, Session, Connection, Stream
- Heartbeat (format only; extension fields defined by mesh)
- LEDBAT, VitalQueue, ReplayWindow
- NATTraversal, PeerDiscovery, SchemaSync, SansIO
- MetabolicState (state machine + parameter maps; governor drives from above)
- SACK (new: selective acknowledgement, delivery ratio)
- ConnectionMigration (new: PATH_CHALLENGE/PATH_RESPONSE)

**Depends on:** verkko-crypto (FrameKey, TransportAEAD, NonceSafety,
EpochRatchet, KeyZeroization).

**Moved IN:**
- SACK promoted from informal to defined concept.
- ConnectionMigration promoted from informal to defined concept.

**Moved OUT:**
- ResourceGovernance/Conductor references removed from body. MetabolicState
  accepts governor output as input; does not reference how the governor
  works. The governor is a mesh concept that consumes MetabolicState.
- Heartbeat payload split: protocol defines fixed header (state, rtt,
  loss, jitter, queue, bw, hlc) + variable extension region. Mesh
  defines extension field contents (filter_root, epoch, work_capacity,
  health_canary, observation_summary).
- Re-encryption thread reference in Section 6.2 replaced with
  "application thread" (the threading model mentions a dedicated thread
  for batch AEAD; it does not need to know the batch is re-encryption).
- ObservationPathForwarding reference replaced with "extension field"
  in heartbeat.

#### Document 3: verkko-mesh

**Purpose.** Multi-peer trust network: clusters, enrollment, key
distribution, health, gossip, convergence orchestration, resource
governance.

**Defined Concepts:**
- Cluster, CIK, CPSK, Enrollment, EDT, KeyBundle, DominanceCascade
- HealthPipeline, Scar, CircuitBreaker, Gossip, Convergence
- PartitionAnnouncement, Gateway, ResourceGovernance, InviteFlow, LKH
- Channel (new: hierarchical data stream with per-channel access control)
- PhysicalBoundary (new: trait for f64 newtypes at hardware/abstract boundary)
- ControlLog (new: signed, hash-linked, append-only GSet)
- FencingToken (new: monotonic sequence for KeyBundle ordering)
- Conductor (new: governor meet projection, sole resource-budget coherence point)
- Memoria (new: scar GSet as institutional memory with decay-weighted scoring)
- ObservationPathForwarding (new: 4-byte flow summary, vertex cover property)

**Depends on:**
- verkko-crypto (DK, DEK, ChannelKey, EpochRatchet, BroadcastEncryption,
  PCSMechanism, DomainSeparatorRegistry, FoundingHash)
- verkko-protocol (Connection, Heartbeat, Session, Frame, VitalQueue,
  MetabolicState, SACK)

**Moved IN:**
- Genesis bootstrap steps 5-6 from verkko-crypto (CRDT init, control log
  first entry, metabolic state initialization)
- Device recovery steps 2-7 from verkko-crypto (enrollment flow, SAS
  verification, KeyBundle delivery, filter sync, admin revocation)
- PCS trigger and distribution logic from verkko-crypto Section 2.6
- 13 informal concepts promoted to Defined Concepts (listed above)
- Heartbeat extension field semantics from verkko-protocol

**Moved OUT:**
- VoiceSpec crate structure -> verkko-ops
- Build order (30 items) -> verkko-ops
- Calibration items (16) -> verkko-ops
- Game day exercises (12) -> verkko-ops
- Rejected proposals table -> verkko-ops
- Compile-time enforcement table (24 items) -> verkko-ops
- Convergence Front 3 re-encryption internals (airtime caps,
  deterministic nonce details) replaced with "relay performs
  re-encryption; see verkko-relay: TerritorialReencryption"
- Cuckoo filter implementation details (10-bit fingerprints, 2-bit
  counters, bucket structure) replaced with reference to verkko-matter:
  ContentFilter
- Terminology section DK/DEK/CIK/CPSK redefinitions replaced with
  citations to their home documents

#### Document 4: verkko-relay

**Purpose.** Relay services: frame forwarding, topology, encrypted
at-rest storage, territorial re-encryption.

**Defined Concepts:**
- TransitRelay, RelayTopology, Membrane
- TerritorialReencryption, IdempotentPUT
- ConstantRatePadding, RelayCorrelationThreatModel

**Depends on:**
- verkko-crypto (StorageAEAD, ChannelKey, EpochRatchet,
  DomainSeparatorRegistry, NonceSafety)
- verkko-protocol (Frame, Connection, NATTraversal)
- verkko-mesh (Cluster, Gateway, Convergence)

**Moved IN:**
- Explicit declaration of verkko-mesh dependency. The relay body uses
  cluster identity, membership changes, CIK pubkeys, and convergence
  signals. This was previously undeclared.
- Convergence Front 3 re-encryption details from verkko-mesh (airtime
  caps for speculative P1 re-encryption, flash budget moderation during
  convergence). These are relay-layer behaviors that were described in
  the mesh convergence section.

**Moved OUT:** Nothing.

**Key change:** The relay now explicitly depends on verkko-mesh. This
is the correct direction (relay is a service consumed by the mesh, but
the relay needs to know about cluster membership to assign territorial
ownership). The dependency is now declared and unidirectional.

#### Document 5: verkko-seat

**Purpose.** Thin client tunnel: asymmetric sessions between a
lightweight client and a host mesh peer.

**Defined Concepts:**
- Seat, SeatSession, HostPeer
- SeatRequest, SeatEvent, SeatInstruction, SeatProgress

**Depends on:**
- verkko-crypto (TransportAEAD, KeyZeroization)
- verkko-protocol (Connection, Session, Frame, Stream, SchemaSync,
  SansIO)

**Moved IN/OUT:** No changes. This document is correctly bounded.

#### Document 6: verkko-matter

**Purpose.** Distributed tracking, transfer, and replication of opaque
addressable units across the mesh.

**Defined Concepts:**
- Matter, MatterIntegrity, ContentFilter, MerkleReconciliation
- MatterTransfer, Stash, Replication, SyncReconciliation
- SealedRange, BandwidthPolicy
- BlobId (new: BLAKE3(plaintext), content identity, survives re-encryption)

**Depends on:**
- verkko-crypto (StorageAEAD, DomainSeparatorRegistry, EpochRatchet,
  ChannelKey)
- verkko-mesh (Cluster, Gateway, Gossip, HealthPipeline, Convergence,
  Channel)
- verkko-relay (Membrane, TerritorialReencryption, IdempotentPUT)

**Moved IN:**
- BlobId promoted from informal to defined concept.
- EpochRatchet and ChannelKey added to verkko-crypto dependency list
  (SealedRange uses them).

**Moved OUT:** Nothing.

#### Document 7: verkko-ops (new)

**Purpose.** Implementation planning, build order, calibration,
verification strategy, and operational exercises for the verkko system.

**Defined Concepts:** None. This document defines no concepts that other
documents depend on. It is a consumer of all six design documents.

**Depends on:** All six design documents (as reference material).

**Contains (moved from verkko-mesh):**
- VoiceSpec crate structure and voice-to-crate mapping
- Build order (30 items across 10 phases)
- Calibration items (16 parameters with defaults and measurement plans)
- Game day exercises (12 scenarios)
- Compile-time enforcement table (24 points)
- Rejected proposals table (40+ entries)
- TLA+ verification plan (7 modules, ~1,090 lines)
- Mechanism Reference Table (35 entries)
- Pattern Registry Table (7 structures + named constructions +
  composition table)
- Boundary Object Table (10 entries)

### 2.2 Dependency graph

```
verkko-crypto                       verkko-ops
     |                              (no deps on it;
verkko-protocol                      it depends on
   |    |    \                       all six)
   |    |     \
verkko-mesh  verkko-seat
   |    \
   |     \
verkko-relay  verkko-matter
              /
   (matter depends on relay)
```

More precisely:

```
              verkko-crypto
                   |
              verkko-protocol
             /       |       \
      verkko-mesh  verkko-seat  (no further deps)
       /       \
verkko-relay  verkko-matter
                  |
          (depends on relay)
```

ASCII DAG with all edges:

```
    verkko-crypto ────────────────────────────────────────┐
         |                                                |
    verkko-protocol ──────────────────────────────┐       |
       / |  \                                     |       |
      /  |   \                                    |       |
     /   |    \                                   |       |
verkko-mesh  verkko-seat                          |       |
   |     \                                        |       |
   |      \                                       |       |
verkko-relay ─────────────────────────────────────(+crypto,+protocol)
   |                                              |
verkko-matter ────────────────────────────────────(+crypto,+mesh,+relay)

verkko-ops (reads all six; nothing reads it)
```

Edges:
- crypto -> protocol
- crypto -> mesh
- crypto -> relay
- crypto -> seat
- crypto -> matter
- protocol -> mesh
- protocol -> relay
- protocol -> seat
- mesh -> relay
- mesh -> matter
- relay -> matter

All edges flow downward or sideways-down. No upward edges. No cycles.

**Verification that no upward dependency exists:**

| Document | Depends on | Does NOT depend on |
|----------|-----------|-------------------|
| verkko-crypto | (nothing) | protocol, mesh, relay, seat, matter |
| verkko-protocol | crypto | mesh, relay, seat, matter |
| verkko-mesh | crypto, protocol | relay, seat, matter |
| verkko-relay | crypto, protocol, mesh | seat, matter |
| verkko-seat | crypto, protocol | mesh, relay, matter |
| verkko-matter | crypto, mesh, relay | protocol (transitive via mesh), seat |

Topological sort: crypto, protocol, (mesh, seat), relay, matter, ops.

---

## 3. Migration Plan

### 3.1 Content moving OUT of verkko-crypto

| Content | Source | Target | Why | Cross-reference |
|---------|--------|--------|-----|-----------------|
| Genesis steps 5-6 (CRDT init, control log entry, metabolic state) | crypto 1.3, lines starting "Initial state" | verkko-mesh, new section "Mesh bootstrap" | These initialize mesh state, not crypto state. Crypto provides keys; mesh consumes them. | crypto 1.3 ends at founding_hash. Adds: "For mesh initialization using these keys, see verkko-mesh: Mesh bootstrap." |
| Device recovery steps 2-7 | crypto 1.4, "Locate mesh" through "Filter sync" | verkko-mesh, new section "Device recovery enrollment" | Five of seven steps are mesh operations. | crypto 1.4 retains step 1 (generate DK + DEK) and Noise XX handshake. Adds: "For the enrollment and key delivery flow, see verkko-mesh: Enrollment, Device recovery enrollment." |
| PCS distribution pseudocode | crypto 2.6, `distribute_via_keybundle` call and `shared_channels` loop | verkko-mesh, existing section 2.6 "CPSK-seeded PCS -- trigger and distribution" | Distribution uses KeyBundle and cluster topology, both mesh concepts. | crypto 2.6 pseudocode ends at computing `K_epoch_new`. Adds: "Distribution of the new epoch key is specified in verkko-mesh." |
| Channel concept references | crypto 1.5, "channel_index", "channel" | Replaced with "key scope" or "identified key" language | Channel is a mesh organizational concept. The crypto layer identifies keys by key_id, not by channel. | No cross-reference needed; key_id is already a crypto defined concept by construction. |

### 3.2 Content moving OUT of verkko-protocol

| Content | Source | Target | Why | Cross-reference |
|---------|--------|--------|-----|-----------------|
| ResourceGovernance/Conductor references | protocol 3.3, 3.4, 6.3 | References removed; MetabolicState accepts `Governed<T>` inputs | The governor is a mesh concept. Protocol defines the state machine; mesh drives it. | protocol 6.3 says "MetabolicState transitions are driven by a governor that provides metabolic state as input. See verkko-mesh: Conductor for the governor specification." |
| Heartbeat extension field semantics | protocol 3.4, fields: filter_root, epoch_current, work_capacity, health_canary, observation_summary | verkko-mesh, new section "Heartbeat extension fields" | These fields carry mesh semantics. Protocol owns the format; mesh owns the meaning. | protocol 3.4 defines heartbeat as "31 bytes fixed header + N bytes extension region." References "see verkko-mesh for extension field definitions." |
| Re-encryption thread naming | protocol 6.2, "re-encryption thread" | Renamed to "batch AEAD thread" | Protocol does not need to know the batch is re-encryption. | No cross-reference needed; it is a naming change. |
| ObservationPathForwarding reference | protocol 3.4 | Replaced with "mesh-defined extension field" | Protocol does not depend on mesh. The extension region is opaque to the protocol layer. | "The observation_summary field is defined by the mesh layer; see verkko-mesh: ObservationPathForwarding." This reference moves to documentation/commentary, not to the formal dependency. |

### 3.3 Content moving OUT of verkko-mesh

| Content | Source | Target | Why | Cross-reference |
|---------|--------|--------|-----|-----------------|
| VoiceSpec crate structure | mesh, "VoiceSpec" section | verkko-ops | Implementation planning, not design contract. | mesh adds: "For implementation structure, see verkko-ops." |
| Build order (30 items) | mesh, "Build order" section | verkko-ops | Implementation planning. | Same reference. |
| Calibration items (16) | mesh, "Calibration Items" section | verkko-ops | Runtime tuning parameters, not design contract. | Same reference. |
| Game day exercises (12) | mesh, "Game Day Exercises" section | verkko-ops | Operational testing, not design contract. | Same reference. |
| Compile-time enforcement table (24) | mesh, "Compile-Time Enforcement" section | verkko-ops | Implementation strategy, not design contract. | Same reference. |
| Rejected proposals table | mesh, "Rejected Proposals" section | verkko-ops | Decision log, not design contract. | Same reference. |
| Mechanism Reference Table | mesh, preamble | verkko-ops | Cross-document index; belongs in the integration document. | Same reference. |
| Pattern Registry Table | mesh, preamble | verkko-ops | Cross-document pattern catalog. | Same reference. |
| Boundary Object Table | mesh, preamble | verkko-ops | Cross-document integration points. | Same reference. |
| TLA+ verification plan | mesh, "TLA+ verification" section | verkko-ops | Verification strategy. | Same reference. |
| Front 3 re-encryption internals | mesh 7.1, Front 3 body | verkko-relay (or remain as thin reference) | Re-encryption behavior is relay-owned. Mesh orchestrates via gate signals. | mesh 7.1 Front 3 says "Re-encryption performed per verkko-relay: TerritorialReencryption. Gate: all affected content re-encrypted or epoch-forwarded." |
| Cuckoo filter implementation details | mesh body references to 10-bit fingerprints, bucket structure | verkko-matter: ContentFilter | Filter internals are matter-owned. | mesh references use defined concept name "ContentFilter (see verkko-matter)" without implementation details. |
| Terminology DK/DEK/CIK/CPSK redefinitions | mesh, Terminology section | Replaced with citations | Duplicates crypto and mesh defined concepts. | "DK: see verkko-crypto. DEK: see verkko-crypto. CIK, CPSK: see this document's Defined Concepts." |

### 3.4 Content moving INTO verkko-relay

| Content | Source | Target | Why | Cross-reference |
|---------|--------|--------|-----|-----------------|
| Convergence re-encryption behavior | mesh 7.1 Front 3 details | verkko-relay, new section "Convergence re-encryption" | Relay owns re-encryption behavior, including speculative P1 re-encryption airtime caps and flash budget interaction. Mesh signals when to start; relay decides how. | mesh 7.1 Front 3 references "verkko-relay: TerritorialReencryption, convergence re-encryption." |

### 3.5 New defined concepts (no content moves, just promotions)

| Concept | Document | Status change |
|---------|----------|---------------|
| Channel | verkko-mesh | Body -> Defined Concepts |
| PhysicalBoundary | verkko-mesh | Body (2.0a) -> Defined Concepts |
| ControlLog | verkko-mesh | Body (1.7) -> Defined Concepts |
| FencingToken | verkko-mesh | Body (2.2, 2.3) -> Defined Concepts |
| Conductor | verkko-mesh | Terminology -> Defined Concepts |
| Memoria | verkko-mesh | Body (5.3) -> Defined Concepts |
| ObservationPathForwarding | verkko-mesh | Body (5.4) -> Defined Concepts |
| FoundingHash | verkko-crypto | Body (1.3) -> Defined Concepts |
| BlobId | verkko-matter | Body (4.2) -> Defined Concepts |
| SACK | verkko-protocol | Body (3.3) -> Defined Concepts |
| ConnectionMigration | verkko-protocol | Body (3.3) -> Defined Concepts |

---

## 4. The Hard Cases

### 4.1 PCS Mechanism: crypto math vs mesh scheduling

**The straddling.** Post-compromise security requires:
- Crypto: HKDF re-seed derivation that mixes fresh DH output into the
  epoch ratchet. Pure math, no network knowledge.
- Mesh: trigger scheduling (polygon 1-factorization, per-channel rate
  limiting, convergence interaction), distribution (KeyBundle or chain
  proofs), and the decision of when to renegotiate CPSKs.

**Split.** verkko-crypto defines PCSMechanism as a pure derivation:
given `(K_epoch_current, fresh_cpsk, channel_id)`, produce
`K_epoch_new`. This is the defined concept with the invariant "after
re-seed with uncompromised material, the adversary cannot derive the
new key."

verkko-mesh defines the trigger policy: when CPSKs are renegotiated
(every reconnection, 24-hour timeout, Noise session expiry), the mesh
calls PCSMechanism for each shared channel and distributes the result
via KeyBundle(Rotate) or re-seed chain proofs. The polygon schedule,
rate limiting, and convergence interaction all live in verkko-mesh.

**Which document gets which piece.**
- verkko-crypto: PCSMechanism defined concept + HKDF derivation formula
- verkko-mesh: Section 2.6 (trigger, scheduling, distribution, rate
  limiting, convergence interaction)

### 4.2 MetabolicState: protocol state machine vs mesh governor

**The straddling.** MetabolicState is consumed by protocol-layer
mechanisms (heartbeat interval, LEDBAT target delay, queue behavior)
but driven by mesh-layer inputs (CPU budget, flash budget, WiFi
capacity, nonce I/O pressure).

**Split.** The state machine (four states, dwell time, phase jitter,
parameter maps) stays in verkko-protocol. It is a protocol concept: it
governs how the transport layer behaves.

The governor (the Conductor) moves to verkko-mesh as ResourceGovernance
/ Conductor. The mesh layer computes `metabolic_state = f(resource_pressures)`
and provides the result to the protocol layer. The protocol layer
consumes `Governed<MetabolicState>` without knowing how the governor
works.

The dependency direction is correct: mesh depends on protocol (mesh
imports MetabolicState), not protocol depends on mesh. The mesh layer
drives MetabolicState transitions; the protocol layer reacts to them.

**Which document gets which piece.**
- verkko-protocol: MetabolicState defined concept, state machine, dwell
  time, phase jitter, parameter maps (heartbeat interval, LEDBAT
  targets)
- verkko-mesh: Conductor defined concept, ResourceGovernance defined
  concept, governor inputs (CPU, flash, WiFi, nonce I/O), governor
  algorithm (min of pressures), Governed<T> wrapper

### 4.3 Heartbeat: protocol frame vs mesh semantics

**The straddling.** The heartbeat is a protocol-layer frame (it is sent
on the wire by the transport layer) but carries mesh-layer semantics
(filter roots, epoch counters, health canaries, observation summaries,
gossip mode flags).

**Split.** verkko-protocol defines the heartbeat frame format with a
fixed header and a variable-length extension region. The fixed header
contains transport-layer fields: metabolic state, RTT, loss, jitter,
queue depth, available bandwidth, HLC timestamp. These are fields the
protocol layer needs for its own operation (congestion control, failure
detection, clock sync).

verkko-mesh defines the extension fields: filter_root, epoch_current,
work_capacity, health_canary, observation_summary, gossip_mode flag.
These are mesh-layer data piggybacked on the protocol heartbeat to
avoid a separate mesh heartbeat.

**Which document gets which piece.**
- verkko-protocol: Heartbeat defined concept, fixed header format,
  transport-layer fields, heartbeat interval (from MetabolicState)
- verkko-mesh: Heartbeat extension field definitions, semantic
  interpretation, new section "Heartbeat extension fields"

### 4.4 Convergence: mesh orchestration vs relay/matter behavior

**The straddling.** Convergence is a mesh-layer orchestration protocol
that triggers behaviors in the relay layer (re-encryption) and matter
layer (filter reconciliation). The mesh document currently describes
the complete flow including relay and matter internals.

**Split.** Convergence stays in verkko-mesh as the orchestration
protocol: entry, wavefront gating, three-front gate sequence, exit
transition. The mesh defines the gates and signals. Each downstream
document owns its own behavior during convergence:

- verkko-relay owns re-encryption behavior during convergence
  (speculative P1, airtime caps, flash budget interaction, deterministic
  nonces). A new section "Convergence re-encryption" in verkko-relay
  describes what the relay does when the mesh signals "Front 3: begin
  re-encryption."
- verkko-matter owns filter reconciliation during convergence (Merkle
  tree walk, divergent subtree exchange). verkko-matter already describes
  this adequately.

**Which document gets which piece.**
- verkko-mesh: Convergence defined concept, gate sequence, Front 1
  (control), Front 2 (consensus), Front 3 (gate signal only: "relay
  performs re-encryption; gate: all affected content re-encrypted"),
  exit transition
- verkko-relay: Convergence re-encryption behavior (speculative P1,
  airtime caps, priority tiering during convergence)
- verkko-matter: Filter reconciliation timing during convergence

### 4.5 Territorial re-encryption: relay ownership vs mesh membership

**The straddling.** The hash ring for territorial re-encryption is
partitioned by DK pubkeys (crypto concept), changes when mesh
membership changes (mesh concept), and the re-encryption itself uses
StorageAEAD (crypto) and lives in the relay. The ring coloring
constraint prevents same-cluster adjacent vnodes (requires cluster
knowledge from mesh).

**Split.** The hash ring and re-encryption algorithm stay in
verkko-relay. The relay declares a dependency on verkko-mesh for Cluster
and membership knowledge. This is the correct direction: the relay is a
service that needs to know about mesh topology to assign ownership.
The mesh does not need to know about hash rings.

**Which document gets which piece.**
- verkko-relay: Hash ring, ring coloring, vnode assignment, deterministic
  nonces, re-encryption algorithm, tiered deadlines, symbiotic donation
- verkko-mesh: Cluster membership (consumed by relay), convergence gate
  signaling (consumed by relay)

### 4.6 CIK: generated in crypto, organizational in mesh

**The straddling.** CIK is an X25519 key (crypto primitive) used as the
Noise static key for inter-cluster sessions (mesh coordination
concept). It is generated during genesis (crypto) but replicated to
cluster peers (mesh) and used for session establishment (protocol/mesh
boundary).

**Split.** CIK is a mesh concept. The crypto layer provides the X25519
key generation primitive (DEK generation). The mesh layer defines CIK as
an organizational concept: "the X25519 key shared among all peers in a
cluster, used as the Noise static key for inter-cluster sessions." CIK
generation uses the same X25519 key generation as DEK, but the concept
of a cluster-shared key is a mesh concept.

**Which document gets which piece.**
- verkko-crypto: X25519 key generation (already covered by DEK)
- verkko-mesh: CIK defined concept (already there), CIK replication
  mechanism, CIK rotation protocol

---

## 5. New Documents

### 5.1 Added: verkko-ops

**Justification for adding.**

The bar: "this concept has its own identity, its own consumers, and
its own evolution rate."

- **Own identity.** verkko-ops is the implementation and operations
  guide. It answers "how do we build this" and "how do we test this,"
  not "what does this do."
- **Own consumers.** Its consumers are implementors and operators, not
  other design documents. No design document references verkko-ops
  concepts. No defined concept lives here.
- **Own evolution rate.** Build order, calibration parameters, game day
  exercises, and rejected proposals change at a different rate than the
  design contracts. Adding a new calibration item does not change any
  defined concept. Rejecting a proposal does not change any invariant.
  These evolve with implementation progress, not design decisions.

**Impact on verkko-mesh.** Moving ~600 lines of implementation planning
out of verkko-mesh reduces it from ~1,464 lines to ~850 lines. This
brings it closer in scale to the other documents (crypto: 535, protocol:
420, relay: 189, matter: 237) while remaining the largest (expected,
because mesh coordination is the most complex layer).

### 5.2 Not removed: no documents merged

The bar for removing: "these two documents always change together."

No pair of documents meets this bar. verkko-crypto and verkko-protocol
evolve independently (crypto primitives vs wire format). verkko-relay
and verkko-matter are related but have different concerns (storage/
forwarding vs content tracking). verkko-seat is intentionally isolated.

---

## 6. Validation

### 6.1 New device joins the mesh

**Document dependency chain:**

1. Generate DK + DEK. **verkko-crypto** (DK, DEK defined concepts).
2. Discover existing device. **verkko-protocol** (PeerDiscovery).
3. Connect via Noise XX. **verkko-protocol** (Session). Uses
   verkko-crypto (TransportAEAD) for session encryption.
4. Receive invite token with FoundingHash. **verkko-crypto**
   (FoundingHash). Token format defined in **verkko-mesh** (InviteFlow).
5. Admin signs enrollment cert. **verkko-mesh** (Enrollment). Uses
   verkko-crypto (DK for signing).
6. SAS verification. **verkko-mesh** (Enrollment invariant).
7. KeyBundle(Grant) delivered. **verkko-mesh** (KeyBundle).
8. Filter sync. **verkko-matter** (ContentFilter, MerkleReconciliation).
   Uses verkko-mesh (Gossip) for dissemination.
9. Post-entry checklist. **verkko-mesh** (health init, control log
   replay).

**Dependency chain:** crypto -> protocol -> mesh -> matter.
**Direction:** strictly downward. No upward or lateral dependencies
needed.

### 6.2 Key revocation and re-encryption

**Document dependency chain:**

1. Admin issues KeyBundle(Revoke). **verkko-mesh** (KeyBundle,
   FencingToken).
2. Fresh random keys injected. **verkko-crypto** (ChannelKey, EpochRatchet
   broken and restarted).
3. Dual-key window disabled. **verkko-crypto** (EpochRatchet invariant
   for revocation).
4. Hash ring assigns territory. **verkko-relay**
   (TerritorialReencryption). Uses verkko-mesh (Cluster) for membership.
5. Deterministic nonces computed. **verkko-crypto** (StorageAEAD,
   DomainSeparatorRegistry).
6. Re-encryption with XChaCha20-Poly1305. **verkko-crypto**
   (StorageAEAD).
7. Idempotent PUT to relay. **verkko-relay** (IdempotentPUT).
8. Pre-batch nonce persist. **verkko-crypto** (NonceSafety,
   DoubleWritePersist).
9. Tiered deadlines enforced. **verkko-relay**
   (TerritorialReencryption).

**Dependency chain:** mesh -> crypto, relay -> crypto, relay -> mesh.
**Direction:** strictly within the DAG. Mesh and relay both consume
crypto. Relay consumes mesh. No upward dependencies.

### 6.3 Seat connects and requests content

**Document dependency chain:**

1. Seat authenticates with user credential. **verkko-seat**
   (SeatSession).
2. Session encrypted. **verkko-crypto** (TransportAEAD).
3. Connection established. **verkko-protocol** (Connection, Session,
   Frame).
4. Schema sync. **verkko-protocol** (SchemaSync).
5. Seat sends request. **verkko-seat** (SeatRequest).
6. Host peer translates to mesh query. **Internal to host peer**
   (host peer is a mesh participant; it uses verkko-mesh and
   verkko-matter internally, but the seat does not).
7. Source selection. **verkko-matter** (source selection predicate).
   Uses verkko-mesh (HealthPipeline, Gossip).
8. Transfer. **verkko-matter** (MatterTransfer).
9. Host peer streams to seat via protocol streams. **verkko-protocol**
   (Stream). **verkko-seat** (SeatEvent or streaming mechanism).

**Dependency chain:** crypto -> protocol -> seat (for the seat path).
The host peer internally uses crypto -> protocol -> mesh -> matter, but
this is invisible to the seat.
**Direction:** the seat path has no mesh dependency. Correct by design.

### 6.4 Partition heals

**Document dependency chain:**

1. First CPSK reconnection. **verkko-mesh** (Convergence entry). Uses
   verkko-protocol (Connection, Session).
2. Gossip switches to flood. **verkko-mesh** (Gossip).
3. Metabolic override. **verkko-mesh** (Conductor drives
   MetabolicState). Uses verkko-protocol (MetabolicState type).
4. Wavefront deadline. **verkko-mesh** (Convergence).
5. Front 1: control log merge, PartitionAnnouncement, EMA reset,
   observation replay. **verkko-mesh** (ControlLog, PartitionAnnouncement,
   HealthPipeline, Scar).
6. Front 2: revocations, fencing tokens, dominance recomputation, hash
   ring recomputation. **verkko-mesh** (KeyBundle, FencingToken,
   DominanceCascade). **verkko-relay** (TerritorialReencryption for hash
   ring, uses mesh Cluster for ring coloring).
7. CPSK renegotiation via polygon schedule. **verkko-mesh** (CPSK,
   Convergence). Uses verkko-crypto (PCSMechanism for re-seed
   derivation).
8. Sealed-range catch-up. **verkko-matter** (SealedRange). Uses
   verkko-crypto (EpochRatchet, ChannelKey) and verkko-mesh (KeyBundle
   for CatchUpGrant).
9. Front 3: re-encryption. **verkko-relay**
   (TerritorialReencryption). Uses verkko-crypto (StorageAEAD,
   NonceSafety).
10. Filter reconciliation. **verkko-matter** (MerkleReconciliation,
    ContentFilter).
11. Exit transition. **verkko-mesh** (Convergence exit).

**Dependency chain:** mesh orchestrates, calling down into relay and
matter for specific behaviors. Crypto is consumed by all three. Protocol
is consumed by mesh. No upward dependencies. The mesh signals gates;
relay and matter execute their own behavior.

**Direction verified:** mesh -> relay, mesh -> matter, relay -> crypto,
matter -> crypto. All within the DAG. No lateral dependencies between
relay and matter during convergence (they are independently signaled by
mesh).

---

## Summary of changes

| Change | Type | Impact |
|--------|------|--------|
| Add verkko-ops (7th document) | Structural | ~600 lines out of mesh; no new concepts |
| verkko-relay declares dependency on verkko-mesh | Dependency correction | Eliminates undeclared bidirectional coupling |
| verkko-protocol removes upward references to mesh | Dependency correction | MetabolicState stays in protocol; governor moves to mesh |
| Heartbeat split into fixed header + extension | Boundary clarification | Protocol owns format; mesh owns extension semantics |
| Genesis/recovery mesh steps move from crypto to mesh | Content migration | Crypto stays dependency-free |
| PCS distribution moves from crypto to mesh | Content migration | Crypto defines math; mesh defines scheduling |
| 11 concepts promoted to Defined Concepts | Contract hygiene | Eliminates informal concept usage |
| BlobId added to verkko-matter | Contract hygiene | Content identity formalized |
| FoundingHash added to verkko-crypto | Contract hygiene | Mesh identity formalized |
| EpochRatchet + ChannelKey added to matter deps | Dependency correction | SealedRange explicitly declares what it uses |
| Convergence Front 3 re-encryption details move to relay | Content migration | Relay owns re-encryption behavior |
| DK/DEK/CIK/CPSK terminology redefinitions removed from mesh | Duplication elimination | Citations replace redefinitions |
