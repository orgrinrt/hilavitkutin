# verkko-ops

## Abstract

Implementation planning, build order, calibration, verification
strategy, and operational exercises for the verkko system. This
document defines no concepts that other documents depend on. It
is a consumer of all six design documents.

This document answers "how do we build this" and "how do we test
this," not "what does this do." Its consumers are implementors
and operators, not other design documents.

## Dependencies

- **verkko-crypto**: all defined concepts (reference material)
- **verkko-protocol**: all defined concepts (reference material)
- **verkko-mesh**: all defined concepts (reference material)
- **verkko-relay**: all defined concepts (reference material)
- **verkko-seat**: all defined concepts (reference material)
- **verkko-matter**: all defined concepts (reference material)

## Defined Concepts

None. This document defines no concepts that other documents
depend on.

## Body

<!-- moved from verkko-mesh — Mechanism Reference Table -->
### Mechanism Reference Table

Every mechanism has one home section. All other mentions are references. Home section references below use the original v10 numbering; see the provenance markers (`<!-- v10 §X.Y -->`) for where each mechanism now resides.

| # | Mechanism | Home | Key constants |
|---|-----------|------|---------------|
| 1 | Identity model (DK, DEK, CIK) | 1.1 (verkko-crypto) | Ed25519, X25519 |
| 2 | Enrollment binding | 1.2 (verkko-mesh) | 30-day pre-sign expiry |
| 3 | Genesis sequence | 1.3 (verkko-crypto) | EmptyLog witness |
| 4 | Device recovery | 1.4 (verkko-crypto + verkko-mesh) | ~2s LAN, 7 steps |
| 5 | Flat channel keys | 1.5 (verkko-crypto) | 256-bit CSPRNG |
| 6 | Key identification | 1.5 (verkko-crypto) | BLAKE3 8-byte truncation |
| 7 | Broadcast encryption | 2.1 (verkko-crypto) | age-style, 52 bytes/recipient |
| 8 | KeyBundle | 2.2 (verkko-mesh) | Grant/Rotate/Revoke, fencing token |
| 9 | Dominance cascade | 2.3 (verkko-mesh) | multiplication form, argmin, threshold 10 |
| 10 | Ratchet\<Scope\> with micro-epoch | 2.4 (verkko-crypto) | HKDF-Expand, hourly floor, 100-frame micro |
| 11 | Post-entry checklist | 2.5 (verkko-mesh) | filter sync, key delivery, health init |
| 12 | Wire format | 3.1 (verkko-protocol) | 55 bytes/frame (micro-epoch varint) |
| 13 | Nonce safety (monotone invariant) | 3.2 (verkko-crypto) | 3 layers, gap factor 3, NonceSourceFactory, double-write persist |
| 14 | Transport (verkko) | 3.3 (verkko-protocol) | LEDBAT, 1280-byte segments |
| 15 | Gossip + control log | 4.1 (verkko-mesh) | GSet, hash-linked, Lamport |
| 16 | Cuckoo filters + Merkle | 4.2 (verkko-matter) | 12-bit slots, 162KB/filter |
| 17 | HealthPipeline (3 taps) | 5.1 (verkko-mesh) | fast=0.1, slow=0.01, PhysicalBoundary-gated |
| 18 | GovernedResource\<P\> | 6.1 (verkko-mesh) | flash, CPU; closed-loop |
| 19 | ThresholdGuard\<P\> | 6.1 (verkko-mesh) | arena, queue; open-loop |
| 20 | Relay (membrane) | 4.3 (verkko-relay) | 5 operations, 90-day TTL, idempotent PUT |
| 21 | Re-encryption + donation | 4.4 (verkko-relay) | consistent hash, pace line 10%, deterministic nonces |
| 22 | Convergence (3 fronts + wavefront + exit) | 7.1 (verkko-mesh) | gates + timeouts + wavefront gating + 6-step exit |
| 23 | CPSK-seeded PCS | 2.6 (verkko-crypto + verkko-mesh) | DH re-seed on CPSK renegotiation |
| 24 | LKH revocation tree | 2.7 (verkko-mesh) | binary tree, threshold 48 |
| 25 | Delegated enrollment tokens | 1.6 (verkko-mesh) | EDT, SAS-gated key delivery |
| 26 | Sealed-range catch-up | 2.8 (verkko-matter) | three-state model, SealedReason enum |
| 27 | Scar provenance + institutional memory | 5.3 (verkko-mesh) | GSet permanence, 180-day decay, 3 consumers |
| 28 | Constant-rate relay padding | 4.5 (verkko-relay) | per-batch decoy randomization |
| 29 | Dual-mode gossip | 3.5 (verkko-mesh) | flood (convergence) / Plumtree (steady state) |
| 30 | Edge-colored CPSK renegotiation | 3.6 (verkko-mesh) | polygon 1-factorization [Lucas1883], 11 rounds, extendable |
| 31 | Observation-path forwarding | 5.4 (verkko-mesh) | 4-byte flow summary, vertex cover property |
| 32 | Metabolic phase jitter | 6.3 (verkko-protocol) | DK-hash-derived, 25% of dwell time |
| 33 | PhysicalBoundary validation gates | 2.0a (verkko-mesh) | trait, lint-enforced, 6 fixed-point newtypes (Q16.16/Q8.24) |
| 34 | Observation replay | 5.1 (verkko-mesh) | per-observer, stale cutoff, hard_reset |
| 35 | Content integrity | 4.2 (verkko-matter) | blob_id = BLAKE3(plaintext), streaming verification |
| 36 | PathTriangle (diagnostic) | verkko-mesh | R12, body text |
| 37 | ScarClassification (3-layer) | verkko-mesh | R12, body text |
| 38 | RelayForwarder (trait) | verkko-relay | R12, body text |
| 39 | ReplicationTriad | verkko-mesh/matter | R12, body text |
| 40 | GovernorThresholds | verkko-mesh | R12, body text |
| 41 | IntraClusterSession | verkko-mesh | R12, body text |
| 42 | ChannelCreate | verkko-mesh | R12, body text |

<!-- moved from verkko-mesh — Pattern Registry Table -->
### Pattern Registry Table

Seven core algebraic structures. Instances are independently implemented unless a shared
trait is explicitly specified. The table names patterns and shared invariants; it does not
mandate shared implementation except where noted.

| # | Structure | Invariants | Instances | Shared trait? |
|---|-----------|-----------|-----------|---------------|
| 1 | **JoinSemilattice (MonotoneLattice)** [Shapiro2011] | `merge(a,b) = max(a,b)` (scalar) or `union(a,b)` (set); idempotent, commutative, associative | Epoch counter, fencing token, control log GSet, enrollment certs, scar counter, flash consumption, re-encryption progress, nonce counter, micro-epoch counter, scar provenance entries, gossip mode flag (14 instances) | No. Pattern only. Fencing tokens have already diverged (grace window). |
| 2 | **Contraction Map (EMA)** [Banach1922] | `alpha_eff = 1-(1-alpha_base)^(dt/dt_ref)`; value in [0,1]; convergent | HealthPipeline fast (alpha=0.1), HealthPipeline slow (alpha=0.01), SACK delivery ratio, capacity ratio (alpha=0.05), PLL correction (alpha=0.01). DualEMA = min(fast,slow). (5 instances + 1 composite) | No. Pattern only. HealthPipeline has already diverged (dual-EMA, stale cutoff, PhysicalBoundary gating absent from others). |
| 3 | **Forward Ratchet** | HKDF-Expand [Krawczyk2010, RFC5869], monotonic, forward-only; previous key deleted after advance | Macro-epoch ratchet, micro-epoch sub-ratchet, CIK rotation (3 instances) | No. Pattern only. |
| 4 | **Typestate Chain** [Strom1986] | Ordered phases with compile-time enforcement; ZST witnesses gate transitions | PartitionHeal\<Phase\>, SourceBreaker\<State\>, MetabolicState, ConvergenceState, KeyLifecycle (Active/Orphaned), EmptyLog/ActiveLog (6 instances) | No. Pattern only. Structurally different per instance. |
| 5 | **BoundedNewtype (PhysicalBoundary)** | `MIN <= v <= MAX`, fixed-point Q16.16 | SackRatio, CapacityRatio, TimeDelta, HealthScore, ScarWeight, DominancePenalty (6 fixed-point newtypes) | **Yes.** `PhysicalBoundary` trait. Narrow scope: fixed-point newtypes only. See verkko-mesh: PhysicalBoundary. |
| 6 | **Resource Monitor** | Capacity tracking with threshold-driven response | GovernedResource (flash, CPU; closed-loop PI), ThresholdGuard (arena, queue; open-loop) (2 types, 4 instances) | No. GovernedResource and ThresholdGuard are structurally too different (closed-loop PI vs open-loop comparator). |
| 7 | **Cyclic Partition** [Karger1997] | Consistent hash ring with coloring constraint; greedy O(N) reassignment | Territorial re-encryption ring (1 instance) | No. Single instance. |

ConvergenceSignalProvider pattern:

Mesh defines an abstract trait (ConvergenceSignalProvider) with
boolean query methods. Relay implements this trait. The runtime
passes relay's implementation to mesh via dependency injection.
The dependency direction is: relay depends on mesh (to know the
trait), mesh depends on its own trait (to call it). No new DAG edge.

Signal delivery is via same-process function call, not wire protocol.

This is the same pattern as MetabolicGovernor: protocol defines the
abstract contract, mesh provides the concrete implementation.

**Named constructions (not core structures):**
- **DeterministicDerivation:** BLAKE3-based [Aumasson2020] domain-separated derivation. 7 registered domains (see verkko-crypto: DomainSeparatorRegistry). New domains must register.
- **Governor Meet Projection (Conductor):** `min(r_1, ..., r_5)` as the meet in `[0,1]^5`. All resource-budget parameters factor through this meet. See verkko-mesh: Conductor.
- **Scar Memoria:** `GSet<ScarEntry> -> time-decay fold -> cospan of 3 scoring morphisms (health, dominance, ETT)`. Named composition of Structure 1 + time-decay homomorphism + scoring cospan. See verkko-mesh: Memoria.

**Composition table:**

| Mechanism | Structures | Notes |
|-----------|-----------|-------|
| Control log | 1 (GSet) | SetLattice instance |
| Epoch sync | 1 (G-Counter) | Componentwise ScalarLattice |
| Fencing tokens | 1 + 4 | Quotient of ScalarLattice x KeyBundleOp |
| Health scoring | 2 (DualContractionMap) + 5 | Meet of fast/slow EMA |
| Scar Memoria | 1 -> decay -> cospan(Health, ETT, Dominance) | Named composition |
| Key ratchet | 3 (macro + micro) | Nested with coordinator |
| Convergence | 4 + 1 + 2 | TypestateChain gating Structures 1 and 2 |
| Circuit breaker | 4 | Conjunctive gate predicates |
| Resource management | 6 + 5 | ResourceMonitor with BoundedNewtype inputs |
| Governor (Conductor) | 6 -> meet -> 4 | Meet projection mediating 6 and 4 |
| Territorial re-encryption | 7 + DeterministicDerivation | Nonce derivation via BLAKE3 |
| Dominance cascade | 1 (ScalarLattice) | Deterministic scoring |
| Gossip mode | 1 (Boolean lattice) | Subsumable under Structure 1 |
| Metabolic SM | 4 (with override) + 6 | Override for convergence |
| Nonce safety | 1 + 5 + 6 | Three-layer defense |
| Observation replay | 2 + 1 | Time-normalized alpha, per-source dt |

<!-- moved from verkko-mesh — Boundary Object Table -->
### Boundary Object Table

| Object | Home | Consumers | Contract |
|--------|------|-----------|----------|
| Heartbeat | 3 (verkko-protocol) | 2 (observation source), 6 (metabolic interval) | Frame spec in 3. Absence drives contraction map (2). Interval = f(metabolic_state) (6). Carries gossip_mode flag (3.5). |
| Fencing token | 4 (verkko-mesh) | 1 (monotone value) | Sequence in KeyBundle (4). Merge: MonotoneLattice (1). Grace restricted to Revoke only. |
| Scar counter | 2 (verkko-mesh) | 1 (monotone value) | Timing decision in HealthPipeline (2). Value: monotone, decay is read-time (1). |
| Partition announcement | 1 (verkko-mesh) | 2 (EMA reset trigger) | Control log entry (1). Resets fast EMA to slow EMA value (2). Observer quorum required. |
| Source selection predicate | 5 (verkko-matter) | 2 (health filter) | Predicate in cuckoo lookup (5). Requires effective_health >= threshold (2). |
| Re-encryption batch | 5 (verkko-relay) | 1 (nonce burst), 6 (flash/CPU) | Content operation (5). Pre-batch persist mandatory (1). Budget consumed (6). Deterministic nonces (4.4). |
| Scar provenance entry | 2 (verkko-mesh) | 1 (GSet element) | ScarEntry in control log GSet (1). CorrelatedSuspicion log query (2). |
| CPSK re-seed event | 4 (verkko-crypto + verkko-mesh) | 2 (epoch chain) | CPSK renegotiation trigger (4). Re-seeds shared channels (2.6). Scheduled via polygon schedule (3.6). Rate-limited per channel. |
| Gossip mode flag | 3 (verkko-protocol) | 3.5 (mode switch) | 1-bit in heartbeat state byte (3.4). Mesh-wide flood/Plumtree synchronization. |
| Observation-path summary | 2 (verkko-mesh) | 5.4 (aggregation) | 4-byte per-flow summary in intra-cluster heartbeat. Gateway aggregation (5.4). Handoff payload (7.1). |

HeartbeatExtension state byte boundary objects:

| Bit | Bit Position Owner | Semantic Owner | Concept |
|-----|-------------------|----------------|---------|
| 2 | verkko-protocol | verkko-mesh | gossip_mode |
| 3 | verkko-protocol | verkko-mesh | convergence_active |
| 4 | verkko-protocol | verkko-mesh | admin_flag |

Protocol defines the byte layout. Mesh defines what the bits mean.

### Crypto Consumer Relationships

| Primitive | Consumer | Usage |
|-----------|----------|-------|
| DK, DEK | verkko-mesh | Device enrollment, key binding |
| CIK | verkko-mesh | Cluster identity, Noise static key |
| EpochRatchet | verkko-mesh | Epoch key management |
| ChannelKey | verkko-mesh, verkko-matter | Channel encryption |
| TransportAEAD | verkko-protocol | Frame encryption |
| StorageAEAD | verkko-relay | At-rest encryption |
| BroadcastEncryption | verkko-mesh | Multi-recipient messaging |
| FoundingHash | verkko-mesh | Mesh identity |

<!-- moved from verkko-mesh — Verification Frame -->
### Verification Frame

The mesh's correctness factors into three independent verification tasks:

1. **Data input correctness.** All physical values enter through PhysicalBoundary gates (see verkko-mesh: PhysicalBoundary). Verified by lint enforcement. 6 fixed-point newtypes (trait-enforced, Q16.16/Q8.24); 3 network gates and 2 temporal gates (convention-enforced, listed in Pattern Registry). No floating-point types in the abstract state machine.
2. **Data output consistency.** All peer-agreement-critical derivations use registered domain separators (see verkko-crypto: DomainSeparatorRegistry). Verified by domain separator registry. 7 registrations.
3. **Control-flow correctness.** All phase transitions enforced by TypestateChain instances (Pattern Registry, Structure 4). Verified by the type system at compile time. 6 instances.

Between the three, the remaining audit task is: verify the algebraic laws of the seven core structures via property-based tests.

Two-axis gap classification:

| Dimension | Definition | Test |
|-----------|-----------|------|
| DAG-structural | Edge correctness | Does filling this gap require adding/removing a dependency edge? |
| Interface-complete | Contract sufficiency | Can a downstream consumer build against this node's defined concepts? |

A gap that is DAG-sound but interface-incomplete blocks implementers
without violating the topological structure.

<!-- moved from verkko-mesh — TLA+ verification -->
### TLA+ Verification Plan

~1,090 lines. Verifiable by TLC in under 30 minutes for 6 clusters, 10 channels. See [Lamport2002] for TLA+ methodology.

| # | Module | Lines (est.) | Priority |
|---|--------|-------------|----------|
| 1 | Nonce safety (full: threading + crash + factory + burst_rate + double-write) | ~240 | Critical |
| 2 | Convergence fronts + wavefront gating + ring recomputation + deterministic re-enc + exit sequence | ~330 | Critical |
| 3 | Dominance cascade (multiplication formula, argmin, adversarial resistance) | ~150 | High |
| 4 | Donation idempotence (deterministic nonces, relay PUT) | ~90 | High |
| 5 | Fencing token (grace window, Revoke-only stale acceptance) | ~110 | Medium |
| 6 | Gateway singularity (per-component, CPSK epoch binding) | ~130 | Medium |
| 7 | Polygon schedule (vertex collision freedom, parity handling, extended rounds) | ~100 | Medium |

### ProVerif Verification Plan

660 lines of ProVerif [Blanchet2001, Blanchet2008] across five domains and three sub-compositions. One week of work. See verkko-crypto: Cryptographic composition for scope.

<!-- moved from verkko-mesh — VoiceSpec -->
### VoiceSpec: Crate Structure and Build Order

Five crates with strict dependency ordering.

```
voice_wire           (no deps)
voice_transport      (depends on voice_wire)
voice_state          (depends on voice_wire, voice_transport)
voice_authority      (depends on voice_wire, voice_transport, voice_state)
voice_maintenance    (depends on all above)
```

| Voice | Crate | Contents | Enforcement points |
|-------|-------|----------|-------------------|
| 1 | voice_wire | Segment framing, AEAD (dual-domain registry), nonce sequencing, NonceSourceFactory, double-write persist, PhysicalBoundary trait + gates | 4, 5, 13, 18, 19-22, 24 |
| 2 | voice_transport | Noise handshakes, LEDBAT (coupled), CPSK epoch binding, polygon schedule | 6, 7 |
| 3 | voice_state | Heartbeats (with gossip_mode flag, observation summary), cuckoo filters, G-Counter epoch, pending buffers, micro-epoch, dual-mode gossip, wavefront timer | 14 |
| 4 | voice_authority | KeyBundle, enrollment, EDT, LKH, dominance (multiplication formula), fencing tokens, control log, founding transcript, sealed-range catch-up | 1, 2, 3, 8, 9, 12, 15 |
| 5 | voice_maintenance | Health scoring (receiver-observed, PhysicalBoundary-gated, observation replay), circuit breaker, scar provenance + Memoria, partition absence (observer quorum), observation-path forwarding, re-encryption (deterministic nonces, ring coloring), flash budget, nonce velocity, stability metrics, constant-rate padding, PI anti-windup, metabolic phase jitter, Governed\<T\> wrapper, content integrity | 10, 11, 16, 23 |

Managed: 256 KB (connection tables, stream state, SACK tracking). Total
protocol memory: 576 KB (64 KB hot + 256 KB pool + 256 KB managed).
Hard constraint: gossip uses unreliable datagrams.

```
Hot arena:    64 KB   (per-frame transients, reset per event loop iteration)
Pool arena:  256 KB   (retransmission buffers, assembly, verification)
Managed:     256 KB   (connection tables, stream state, SACK tracking)
Total:       576 KB
```

Managed arena breakdown:
- Inter-cluster connections (11): ~176 KB
- Intra-cluster connections (5): ~50 KB
- Protocol-level state: ~30 KB

Hard constraint: gossip MUST use unreliable datagrams at the protocol
level. If gossip uses reliable-ordered streams, the convergence gossip
burst (2,200+ frames) requires 2.7 MB of retransmission state,
exceeding the pool arena by 10x. This is an arena-driven design
requirement.

Convergence burst: 2,200-frame gossip burst requires ~52 drain-reset
cycles, completing in ~1.5ms on a Pi Zero. Within the 2-second
heartbeat budget.

### Build Order

| Phase | Items | Content |
|-------|-------|---------|
| 1: Wire/transport | 1-4 | Segmentation, reliability, event loop, LEDBAT (coupled), STUN |
| 2: Crypto foundation | 5-7 | Noise KK/XX, dual-domain AEAD + nonce safety (all three layers + double-write + NonceSourceFactory), DK/DEK separation, BLAKE3 test vectors |
| 3: Content routing | 8-9 | Counting cuckoo filter + Merkle, adaptive heartbeat, content integrity (blob_id) |
| 4: Key distribution | 10-12 | KeyBundle, enrollment + EDT + genesis + EmptyLog, connection migration |
| 5: Health/trust | 13-15 | HealthPipeline (PhysicalBoundary-gated, observation replay, lineage tracking), circuit breaker, scar provenance + Memoria, observation-path forwarding, entropic governor + Governed\<T\> |
| 6: Territory/donation | 16-18 | Dominance cascade (multiplication formula), territorial re-encryption (deterministic nonces, ring coloring), symbiotic donation |
| 7: Convergence/relay | 19-21 | Three-front convergence (wavefront gating, polygon schedule with extended rounds, exit sequence), pending buffer split, relay (idempotent PUT), constant-rate padding |
| 8: Gossip/topology | 22-24 | Dual-mode gossip (flood/Plumtree, anomalous-message detection), metabolic phase jitter, gateway election (ETT tiebreaker) |
| 9: Observability | 25-27 | Structured logging, device recovery, TLA+ verification |
| 10: Scaling | 28-29 | LKH tree, sealed-range catch-up, CPSK-seeded PCS |
| Post-MVP | -- | Raptor codes, cross-path probe |

### Abstract Protocol Interface (Language-Neutral)

The abstract polling model is two-phase:

1. feed(input): processes one input event, mutates internal state.
2. poll_output(): drains one output event. Returns None when empty.
3. next_deadline(): queries the earliest timer deadline.

This model provides structural reentrancy prevention independent of
any language's type system: feed() cannot be called from within
poll_output() processing because poll_output() does not accept inputs.

The Rust implementation uses a four-method model (handle, poll_transmit,
poll_event, next_deadline, handle_timeout) that separates transmit
outputs from application events for type-level output separation and
arena-epoch batching. Both models implement the same wire protocol.

<!-- moved from verkko-mesh — Calibration Items -->
### Calibration Items (16)

| # | Parameter | Default | Measurement |
|---|-----------|---------|-------------|
| 1 | Pace line trigger | 10% behind pace | Donation frequency on target hardware |
| 2 | LEDBAT extreme asymmetry | Dual controllers for typical DSL | 100:1 satellite asymmetry |
| 3 | Small-mesh stability | Below 8 peers: bounded degradation | Convergence time on 3/5-peer meshes |
| 4 | Ed25519 yield interval | 8ms | Emulator frame drops at 8ms vs 16ms |
| 5 | FTL GC stalls | Deferral via write manager | Stall frequency/duration on target eMMC |
| 6 | Degraded threshold FP rate | 0.92 | False-Degraded rate with normal jitter |
| 7 | Metabolic dwell time | 4 heartbeat intervals (20s) | Oscillation frequency on Batocera |
| 8 | Scar auto-clear obs count | 100 over 30 days | Background observation rates |
| 9 | Variance threshold | 3 sigma from 24h baseline | Welford measurements on target hardware |
| 10 | Breaker probe increase | 30% | HalfOpen-to-Closed transition time |
| 11 | ScarConfig obs counts | 5 (degrading), 20 (recovering) | False scar rate and latency |
| 12 | Nonce velocity canary | 50% of gap | Canary log frequency in normal operation |
| 13 | Nonce velocity persist trigger | 75% of gap | Adaptive persist frequency during burst |
| 14 | Plumtree RTT shift recompute threshold | 50% | MST stability under typical RTT variance |
| 15 | Wavefront p95 reconnect latency multiplier | 1.5x | False-partition rate for slow pairs |
| 16 | Metabolic phase jitter ratio | 25% of dwell time | Kuramoto order parameter on 12-cluster mesh |
| 17 | Governor Background->Idle threshold | 0x00003333 (0.20) | 0.10-0.30 |
| 18 | Governor Idle->Active threshold | 0x00008000 (0.50) | 0.40-0.60 |
| 19 | Governor Active->Stress threshold | 0x0000CCCC (0.80) | 0.70-0.90 |
| 20 | Governor hysteresis margin | 0x00000CCC (0.05) | 0.03-0.10 |
| 21 | Intra-cluster scar threshold | 30 misses | 20-50 |
| 22 | Intra-cluster health window | 32 heartbeats | 16-64 |
| 23 | Diagnostic triangle stability | 3 ticks | 2-5 |
| 24 | WiFi scar decay base | 1 day | 1-3 |
| 25 | WiFi scar decay multiplier | 6 | 4-10 |
| 26 | Relay amplification window | 2x heartbeat interval | 1.5x-3x |

<!-- moved from verkko-mesh — Game Day Exercises -->
### Game Day Exercises (12)

1. **Genesis self-test failure.** Corrupt CSPRNG. Verify: clean abort, no persistent state, actionable error.

2. **Device recovery on headless NAS.** Break admin phone. Verify: dominance cascade (multiplication formula) promotes NAS, succession notification fires, new phone enrolled < 30s.

3. **Gateway crash during Front 1.** Kill gateway mid-KeyBundle. Verify: 5-minute timeout, channel orphaned, admin notified, Fronts 2-3 proceed.

4. **Flash budget exhaustion.** 5 revocations in 1 hour. Monitor: rolling window, diminuendo activation, nonce velocity < 75% gap, pre-batch double-write persist fires.

5. **Degraded gateway (10% loss).** 24 hours. Verify: PhysicalBoundary-gated health observations, Degrading direction, 5-obs fast-track scar, HalfOpen. On recovery: both gates open, probe +30%, Closed.

6. **Nonce gap exhaustion with double-write.** Mock 200,000 AEAD frames in 90s. Verify: double-write persist, dynamic gap = max(65,536, 400,000). Kill at frame 100,000. Verify: recovery reads valid copy, restart at persisted + 400,000. Simulate torn write: one copy invalid, recovery uses valid copy. burst_rate loaded from disk.

7. **Partition with observer quorum.** Partition 3 peers for 4 hours. Heal. Verify: PartitionAnnouncement with observer quorum check. Deferred attribution when quorum not met. EMA reset after quorum met.

8. **Dual-constraint governor.** Gaming (CPU 80%) + revocation. Verify: governor selects min(CPU, flash, nonce_io_pressure). Anti-windup prevents integrator overshoot. Governed\<T\> wraps all budget parameters.

9. **EDT enrollment without admin.** Admin offline. EDT-holding peer enrolls new device. Verify: enrollment cert signed, SAS pending, KeyBundle(Grant) delayed.

10. **Sealed-range catch-up with SealedReason.** Peer offline 12 hours (no revocation). Verify: SealedReason::Absence, fast-forward, current-epoch traffic decrypts. Admin issues UnsealGrant. Peer offline 12 hours with revocation during absence: SealedReason::Revocation, CatchUpGrant implicitly unseals. Sealed-range catch-up completes before CPSK re-seed.

11. **Convergence with polygon schedule, rate limiting, and exit sequence.** Partition 6 clusters for 2 hours, one globally shared channel. Heal. Verify: polygon schedule extends beyond 11 rounds due to rate limiter. All pairs complete in single convergence event. Exit sequence: gossip mode switch-back, spanning tree construction, dwell time restoration, health baseline update, ring checksum verification. Total convergence < 45 seconds.

12. **Observation-path handoff.** Gateway re-election during active traffic. Verify: old gateway includes observation summary (44 bytes). New gateway bootstraps with per-source last_update. No health discontinuity.

<!-- moved from verkko-mesh — Compile-Time Enforcement -->
### Compile-Time Enforcement (24 points)

| # | Property | Mechanism | Prevents |
|---|----------|-----------|----------|
| 1 | Convergence front ordering | `PartitionHeal<Phase>` typestate | Bulk before Consensus |
| 2 | Intra-front ordering | `RevocationsApplied<Phase>` witness | Grants before revocations |
| 3 | Key lifecycle | `ActiveKey` / `OrphanedKey` distinct types | Encrypt with orphaned key |
| 4 | Nonce uniqueness | `IssuedNonce` affine type | Nonce reuse |
| 5 | Crash recovery ordering | `RecoveredSession` / `ReadySession` | AEAD before gap persist |
| 6 | Metabolic transitions | ZST markers + `DwellSatisfied` witness | Invalid state jumps |
| 7 | Channel access | `ChannelHandle<Level>` phantom type | Read-only peer encrypting |
| 8 | KeyBundle operations | `KeyBundleOp` ADT, non-empty sets | Revoke with empty set |
| 9 | Fencing tokens | Opaque `FencingToken` newtype | Comparison bypass |
| 10 | Flash budget | `WritePermit` resource token | Unaccounted writes |
| 11 | Circuit breaker | `SourceBreaker<State>` typestate | Invalid transitions |
| 12 | Revocation scope | `RevocationScope` sum type | Intra-cluster revoke without CIK rotation |
| 13 | Signing/encryption separation | `DeviceKey` / `DeviceEncryptionKey` | Birational map misuse |
| 14 | Health classification | `HealthClassification` product type | Unsigned gap; unhandled quadrant |
| 15 | Partition absence | `ScarAttribution::PartitionAbsence` | Partition scars affecting dominance |
| 16 | Nonce velocity | `ControlMetric` supertrait, const floor | Gap exhaustion; unsafe factor tuning |
| 17 | Voice isolation | Crate-level boundaries per voice | Cross-voice internal access |
| 18 | Nonce threading | `NonceSourceFactory` + `PhantomData<ThreadId>` | Cross-thread NonceSource sharing |
| 19 | PhysicalBoundary: SackRatio | `SackRatio` newtype, fallible constructor | NaN from 0/0 entering EMA |
| 20 | PhysicalBoundary: TimeDelta | `TimeDelta` newtype, positive-only | Negative time delta inverting EMA |
| 21 | PhysicalBoundary: CapacityRatio | `CapacityRatio` newtype, bounded | Infinity from zero-denominator |
| 22 | PhysicalBoundary: NonceHwm | `NonceHwm` newtype, checksum-validated | Torn-write HWM entering nonce state |
| 23 | Resource-budget provenance | `Governed<T>` wrapper, governor-module-private constructor | Raw resource reads bypassing governor |
| 24 | PhysicalBoundary trait | Sealed trait with const bounds, lint-enforced, fixed-point only | Raw float or integer in abstract state machine |

**Outside type system:** EMA math (tests), CRDT lattice properties (TLA+), crypto primitives (audited libs), calibration parameters (runtime), network topology/timing (runtime), flash wear (hardware), emergent interactions (integration tests), PI crossover (simulation).

<!-- moved from verkko-mesh — Rejected Proposals -->
### Rejected Proposals

| Proposal | Rejection |
|----------|-----------|
| Trophic cascade | Insufficient observation diversity at 12 clusters |
| Gauss-Markov weighting | Sub-microsecond precision invisible at household scale |
| Convalescence protocol | Rate-limited bao verification disproportionate |
| Harmonized scar decay | Too rare at 36 peers |
| Lyapunov runtime monitor | Four direct metrics more useful than composite V(x) |
| Four convergence modalities | Academic; drives no implementation decision |
| Nyquist covariance constraint | Vanishingly unlikely violation |
| Entropy production rate | Not a design constraint |
| HKDF key hierarchy | Race conditions, detachment ordering; flat keys simpler |
| K_mesh | Single compromise point; pairwise CPSKs more secure |
| Level 0 plaintext | Exposes type, introduces downgrade attack |
| Relay as one-peer cluster | Full protocol stack unnecessary; membrane simpler |
| Health-triggered rotation acceleration | No security benefit without revocation; positive feedback |
| Single-gate breaker probe | Direction alone insufficient; requires variance |
| Velocity-to-throttle direct | Throttle does not reduce gap consumption; persist correct |
| Pre-batch persist on metabolic transition | Metabolic transitions are continuous, not discrete |
| missed_succession bool | Derivable from epoch comparison |
| Dynamic gap factor 4 | Persist delay and throughput ramp anticorrelate; factor 3 sufficient |
| User-set admin priority list | Less reliable than cascade; UI display solves discoverability |
| MeshEntry\<Mode\> | 80% mode-specific branches; enrollment Sybil boundary not capturable |
| ResourceBudget\<Nonce\> | Nonce is invariant enforcement, not resource consumption |
| Sub-meshing with super-peer | Introduces trusted intermediary. Violates NoTrustedIntermediary. |
| Quorum enrollment | Weakens Sybil boundary. EDT preserves single-authority. |
| Bloom clocks for cross-channel causality | No invariant requires cross-channel data-plane causality |
| Health digest as corroboration | Digest lacks source identity. Scar provenance replaces. |
| Per-message forward secrecy | Per-message DH for broadcast prohibitive. Micro-epoch bounds. |
| Gateway-bypass for bulk transfers | Decouples observation from control. Observation-path forwarding replaces. |
| Predictive gateway migration | Control-loop instability. Reactive re-election correct. |
| Bloom cascade pre-announcement | Merkle reconciliation strictly better |
| Re-seed coalescing window | Superseded by polygon schedule |
| Relay-assisted gossip | Increases relay's observational power, violates membrane model |
| Capacity-weighted vnodes | Couples long-lived hash ring to transient link quality |
| Blanket flood gossip | Dual-mode strictly better. Matches gossip to risk profile. |
| Cut-size-proportional wavefront floor | Conflates parallelism with sequentiality |
| Mon-Con adjunction (formal claim) | Not standard category theory. Replaced with concrete description of monotone-contraction composition. |
| Channel-scoped re-seed deduplication | Conflates key advancement (per-channel) with pairwise handshakes (per-pair). |
| Strict vital sub-priority | Starves heartbeats during concurrent enrollment + CPSK bursts. 70/30 weighted fair queuing preserves priority without starvation. |
| Abolish ring coloring | At ~95 expected collisions, the constraint prevents deterministic re-encryption waste. Continuous enforcement is cheap (~5us/heartbeat). |
| Congestion-aware convergence mode | Adds complexity without clear benefit. Historical RTT in wavefront formula plus vital fair queuing handles constrained links. |
| GovernedResource/ThresholdGuard unification (U6) | Closed-loop PI feedback cannot be unified with open-loop threshold response. Monomorphism exists but does not imply subtype relationship. Duplication is cheaper than the wrong abstraction. |
| HKDF/BLAKE3 single abstraction (U7) | Different security properties; surface similarity is not structural identity. Key derivation and hash-mode BLAKE3 serve different domains. |
| Shared EMA trait (Ema\<G\>) | HealthPipeline EMA has already diverged (dual-EMA, stale cutoff, PhysicalBoundary gating). Shared trait creates 7-section blast radius on divergence vs 2-section without. Pattern Registry names the pattern without coupling. |
| Shared MonotoneLattice trait | Fencing tokens have already diverged (grace window). Pattern documented in registry; independent implementation preserves evolution. |
| TransitionActive\<From, To\> compound typestate | Hides ordering rather than revealing it. Two state machines with explicit sequencing invariant are clearer. The transient state exists regardless of state-machine count; naming it does not eliminate the window. |
| Aggregation-specific alpha formula | Time-normalized alpha already handles aggregation frequency when dt is tracked per attributed source. Two spec lines (FlowSummary attribution, per-source last_update) suffice. No new formula needed. |

## References

[Aumasson2020] Aumasson, J.-P., Neves, S., O'Hearn, Z., Winnerlein, C. (2020). "BLAKE3 -- one function, fast everywhere." BLAKE3 specification.

[Banach1922] Banach, S. (1922). "Sur les operations dans les ensembles abstraits et leur application aux equations integrales." Fundamenta Mathematicae 3, 133-181.

[Blanchet2001] Blanchet, B. (2001). "An Efficient Cryptographic Protocol Verifier Based on Prolog Rules." CSFW 2001.

[Blanchet2008] Blanchet, B. (2008). "A Computationally Sound Mechanized Prover for Security Protocols." IEEE TDSC 5(4).

[Karger1997] Karger, D., Lehman, E., Leighton, T., Panigrahy, R., Levine, M., Lewin, D. (1997). "Consistent Hashing and Random Trees." STOC 1997.

[Krawczyk2010] Krawczyk, H. (2010). "Cryptographic Extraction and Key Derivation: The HKDF Scheme." CRYPTO 2010.

[Lamport2002] Lamport, L. (2002). *Specifying Systems: The TLA+ Language and Tools for Hardware and Software Engineers.* Addison-Wesley.

[Lucas1883] Lucas, E. (1883). *Recreations Mathematiques,* Vol. 2. Gauthier-Villars.

[Shapiro2011] Shapiro, M., Preguica, N., Baquero, C., Zawirski, M. (2011). "Conflict-free Replicated Data Types." SSS 2011 / INRIA RR-7506.

[Strom1986] Strom, R.E., Yemini, S. (1986). "Typestate: A Programming Language Concept for Enhancing Software Reliability." IEEE TSE 12(1), 157-171.
