# Verkko Design Taxonomy

**Date:** 2026-03-28
**Status:** Revised taxonomy applied (7-doc structure)

---

## Overview

Verkko is the networking substrate for saalis and the broader
hilavitkutin ecosystem. It is a peer-to-peer mesh protocol for
household devices (2-12 clusters, ~36 peers). Verkko defines
itself; saalis adapts to it.

The verkko design is split into seven documents. Each document
defines an encapsulated domain with a strict DAG dependency chain.
Documents reference each other only through defined concepts
(the "public API" of each document).

For the migration rationale and detailed analysis, see
`2026-03-28-revised-taxonomy.md`.

---

## Document structure

Every verkko design document follows this structure:

```
# verkko-{name}

## Abstract
10-20 lines. What this document defines, what depends on it.

## Dependencies
Which other verkko documents this one requires.
References only the Defined Concepts of those documents.

## Body
The full design. Deep-nested headers for searchability.
All internals: algorithms, formulas, wire formats, citations,
proofs, calibration. An agent or human greps for a header
and reads just that section.

## Defined Concepts
The contract this document exposes to documents that depend
on it. Each entry has:
  - Name: the citable identifier
  - What it is: one sentence, no internals
  - Invariant(s): the guarantee(s) dependents can rely on

If a concept is not in this section, dependents cannot
assume it exists or rely on its properties.

## References
Cited works with [AuthorYear] keys.

## Addendum
Design digest and delta vs predecessor.
```

---

## Defined Concepts as public contract

The Defined Concepts section is NOT a summary of the document.
It is a structural glossary with teeth. Each entry names a
concept, states what it is, and declares invariants that
dependents can rely on.

Dependents cite these by name: "The session key is advanced
via the **Epoch Ratchet** (see verkko-crypto)." They do not
need to know it uses HKDF, or that there is a micro-epoch
sub-ratchet, or that the gap formula is `max(65536, burst*3)`.
They know the ratchet exists and what it guarantees.

If a defined concept changes its invariants, all dependents
must be reviewed. If the internal mechanism changes but the
invariants hold, dependents are unaffected.

This is a design contract, not an implementation contract.
No language, no traits, no types. Just names and guarantees.

---

## The seven documents

### 1. verkko-crypto

Keys, derivation, AEAD, nonces.

**Defines:** The cryptographic primitives that all other
documents use without needing to know how they work.

**Scope:**
- Identity keys (DK, DEK)
- Key derivation (epoch ratchet, micro-epoch sub-ratchet)
- AEAD constructions (ChaCha20-Poly1305, XChaCha20-Poly1305)
- Nonce safety (three-layer defense, double-write persist)
- Post-compromise security mechanism
- Broadcast encryption construction
- BLAKE3 domain separator registry
- Key zeroization
- Key bootstrap protocol
- Keyring storage security model

**Depends on:** nothing.

### 2. verkko-protocol

Wire format, sessions, congestion, discovery, NAT.

**Defines:** How two peers establish a connection and exchange
authenticated frames. Everything here is symmetric (both sides
are equal participants) and concerns exactly two endpoints.

**Scope:**
- Wire format (framing, headers, authentication tags)
- Session establishment (Noise KK/XX)
- Congestion control (LEDBAT, coupled at gateway)
- Heartbeat and failure detection
- Queuing (three-tier, vital fair queuing)
- Replay detection (sliding window)
- SACK delivery acknowledgement
- Connection migration
- NAT traversal (hole punching, relay-first fallback)
- Peer discovery (mDNS, DHT, invite hints, cached peers)
- Sans-IO architecture
- Stream multiplexing
- Schema sync (protobuf exchange during handshake)
- 0-RTT resumption

**Depends on:** verkko-crypto (for session keys, frame
encryption, nonce management).

### 3. verkko-mesh

Clusters, trust, health, gossip, convergence.

**Defines:** How peers form a trust network, coordinate
administration, monitor health, disseminate information,
and recover from partitions. Everything here concerns the
multi-peer collective, not individual connections.

**Scope:**
- Cluster model (CIK, intra-cluster trust)
- Cluster-pair sessions (CPSK, renegotiation)
- Enrollment and delegated enrollment (EDT)
- Key distribution (KeyBundle, fencing tokens, grace window)
- Dominance cascade and admin succession
- Health pipeline (dual EMA, PhysicalBoundary gates, NaN guard)
- Scar mechanics and provenance
- Circuit breaker
- Gossip (Plumtree/flood, mode switching, anomalous detection)
- Convergence (three-front, polygon schedule, wavefront gating)
- Partition handling (PartitionAnnouncement, EMA reset)
- Gateway election (max-register CRDT, ETT tiebreaker)
- Resource governance (GovernedResource, ThresholdGuard, governor)
- Metabolic state machine
- Observation-path forwarding
- LKH binary tree (revocation at scale)
- Invite flow and mesh bootstrap

**Depends on:** verkko-crypto (for key types, derivation,
broadcast encryption), verkko-protocol (for connections,
heartbeats, frame delivery).

### 4. verkko-relay

Forwarding, topology, membrane, re-encryption.

**Defines:** How peers route traffic for unreachable peers
and store encrypted matter at rest on behalf of the mesh.

**Scope:**
- Transit frame forwarding
- Relay topology decisions (who relays for whom)
- Relay trust model
- Relay membrane (encrypted-at-rest storage)
- Territorial re-encryption (hash ring, ring coloring)
- Re-encryption tiered deadlines
- Deterministic re-encryption nonces
- Relay storage TTL (90-day rotation, TOUCH)
- Idempotent PUT contract
- Constant-rate padding
- Correlation attack surface and threat model

**Depends on:** verkko-crypto (for re-encryption keys, AEAD),
verkko-protocol (for frame forwarding),
verkko-mesh (for cluster membership, convergence signals).

### 5. verkko-seat

Thin client tunnel, asymmetric sessions.

**Defines:** How a thin client (web UI, mobile app, terminal)
connects to a peer without joining the mesh as a full
participant. A seat tunnels through a single peer and sees
the mesh through that peer's eyes.

A seat:
- Does NOT run gossip, health scoring, or convergence
- Does NOT hold channel keys or participate in dominance
- Does NOT enroll in the mesh as a peer
- DOES authenticate to a host peer
- DOES send requests and receive responses/events
- DOES report progress on delegated operations

**Scope:**
- Seat-to-peer connection model (asymmetric, not peer-to-peer)
- Seat authentication (user credential, role mapping)
- Request/response protocol
- Event subscription (push from host peer to seat)
- Progress reporting (download progress, operation status)
- Instruction execution (host peer delegates filesystem ops)
- Seat lifecycle (connect, disconnect, reconnect; host peer
  handles mesh continuity)
- Multiple seats per peer (living room box serves phone,
  tablet, web browser simultaneously)

**Depends on:** verkko-crypto (for session encryption),
verkko-protocol (for wire framing, connection establishment).

**Does NOT depend on:** verkko-mesh. The seat does not know
about the mesh. The host peer translates between the seat's
request/response world and the mesh's gossip/convergence world.

### 6. verkko-matter

Filters, transfer, stash, sync, replication.

**Defines:** How the mesh tracks, advertises, transfers,
replicates, and protects opaque addressable units of data.

**The term "matter":**

Matter is a content-addressed, opaque, integrity-verified unit
tracked by the mesh. A matter has an identity
(`matter_id = BLAKE3(plaintext)`), territorial ownership,
replication state, and encryption state. The mesh tracks,
advertises, transfers, replicates, and re-encrypts matter
without knowledge of its contents.

Implementors define what matter represents (game ROMs, metadata
archives, media files, database records). Verkko does not
distinguish between types of matter. A 2GB ROM and a 50KB
metadata archive are both matter. A row in a shared database
is matter. Anything the mesh needs to track is matter.

Without matter, the mesh has no purpose. Without the mesh,
matter has nowhere to live.

**Scope:**
- Matter identity (content-addressed, BLAKE3)
- Matter integrity (bao-tree verified streaming)
- Content routing (cuckoo filters, Merkle reconciliation)
- Matter transfer protocol (LAN vs WAN paths, multi-source)
- Matter advertising (peers announce what they hold)
- Stash (per-peer tracking of local matter)
- Replication (redundancy targets, background replication)
- Sync and reconciliation (divergence detection, delta sync,
  merge rules)
- Sealed-range catch-up
- Bandwidth policies (inter-cluster time-based rules)

**Depends on:** verkko-crypto (for content-addressing, integrity
hashing, re-encryption keys), verkko-mesh (for cluster topology,
health-based source selection, gossip dissemination, convergence
sequencing, gateway routing), verkko-relay (for territorial
ownership, re-encryption, at-rest storage).

### 7. verkko-ops

Implementation planning, build order, calibration, verification.

**Defines:** No design concepts. Contains implementation planning,
build order, calibration items, game day exercises, compile-time
enforcement points, rejected proposals, mechanism reference table,
pattern registry, boundary object table, verification frame, and
TLA+/ProVerif verification plans.

**Depends on:** All six design documents (as reference material).
Nothing depends on verkko-ops.

---

## Dependency graph

```
verkko-crypto
     |
verkko-protocol
   |         \
verkko-mesh   verkko-seat
   |    \
verkko-relay  verkko-matter
                  |
            (also depends on relay)

verkko-ops (leaf — depends on all six, nothing depends on it)
```

Strictly acyclic. No circular dependencies.

verkko-crypto is the root. Everything depends on it.
verkko-protocol depends only on crypto.
verkko-mesh and verkko-seat depend on crypto and protocol.
verkko-relay depends on crypto, protocol, and mesh.
verkko-matter depends on crypto, mesh, and relay.
verkko-ops depends on all six; nothing depends on it.

Topological sort: crypto, protocol, (mesh, seat), relay,
matter, ops.

---

## What exists vs what is missing

### Exists (in v10 design, needs extraction and splitting)

| Document | Coverage from v10 |
|----------|------------------|
| verkko-crypto | ~95%. Identity, key derivation, AEAD, nonces, PCS, broadcast encryption. Missing: key bootstrap, keyring storage. |
| verkko-protocol | ~60%. Wire format, LEDBAT, heartbeat, queuing. Missing: NAT traversal, discovery, sans-IO, stream multiplexing, schema sync, 0-RTT. |
| verkko-mesh | ~90%. Clusters, enrollment, KeyBundle, dominance, health, gossip, convergence, gateway, governance. Missing: invite flow, bootstrap. |
| verkko-relay | ~70%. Membrane model, re-encryption, hash ring, ring coloring, idempotent PUT, padding. Missing: transit forwarding spec, topology decisions, correlation threat model. |
| verkko-seat | ~0%. The concept is established but no design exists. |
| verkko-matter | ~40%. Cuckoo filters, Merkle reconciliation, sealed-range. Missing: blob transfer protocol, stash, replication, sync, download coordination, bandwidth policies. |

### Known gaps (not designed yet)

**Critical (mesh cannot function without these):**
- NAT traversal (verkko-protocol)
- Peer discovery chain (verkko-protocol)
- Key bootstrap — first mesh formation (verkko-crypto)
- Invite flow — user-facing onboarding (verkko-mesh)

**High (mesh functions but is incomplete):**
- Blob/matter transfer protocol (verkko-matter)
- Sync and reconciliation (verkko-matter)
- Seat protocol (verkko-seat)
- Transit relay forwarding spec (verkko-relay)

**Medium (mesh works, these add completeness):**
- Stash — per-peer matter tracking (verkko-matter)
- Replication targets and background work (verkko-matter)
- Bandwidth policies (verkko-matter)
- Stream multiplexing (verkko-protocol)
- Schema sync (verkko-protocol)
- 0-RTT resumption (verkko-protocol)

---

## Relationship to existing documents

The current v10 design (`2026-03-28-final-design-v10.mesh-crypto-and-retrieval.md`)
is the authoritative source for the crypto, mesh, and partial
relay/matter content. It will be split into the six documents
above. v10 is not discarded — it is decomposed.

The existing `mock/crates/verkko/DESIGN.md.tmpl` (SCTP-style
transport) is superseded. It was designed for a client-server
model that no longer exists. Some ideas from it (sans-IO,
stream multiplexing, schema sync, 0-RTT) will be incorporated
into verkko-protocol, but the wire format, handshake, and
congestion control from v10 take precedence.

---

## Naming conventions

- Documents: `verkko-{name}.md` in `mock/research/`
- Defined concepts: PascalCase names (EpochRatchet, DualEma,
  Matter, Seat)
- Citations within documents: `[AuthorYear]` format
- Cross-document references: "see verkko-{name}" with the
  defined concept name

---

## Status

All seven documents have been written and the revised taxonomy
applied. The migration from the original 6-doc structure to the
7-doc structure (adding verkko-ops) is complete. Content has been
moved according to the migration plan in
`2026-03-28-revised-taxonomy.md`. No content was deleted; all
content was moved verbatim with provenance markers and updated
cross-references.
