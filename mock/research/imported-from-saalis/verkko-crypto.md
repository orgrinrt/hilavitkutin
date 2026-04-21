# verkko-crypto

## Abstract

Cryptographic primitives for the verkko ecosystem. Key generation,
key derivation, authenticated encryption, nonce management, and
post-compromise security. All other verkko documents depend on this
one for their cryptographic operations.

This document defines the key types, derivation chains, AEAD
constructions, and nonce safety mechanisms. It does not define how
keys are distributed (that is verkko-mesh), how they are used on
the wire (that is verkko-protocol), or what they protect (that is
the concern of each consumer).

No network concepts. No peers. No messages. Pure cryptography.

**Attribution (v10).** This design is the product of forty expert
perspectives across eleven review phases. v10 validated by three
independent reviewers (cryptographic protocol specialist, distributed
systems architect, applied mathematician). See v10 document for full
attribution.

## Dependencies

None. This is the root of the verkko dependency graph.

## Defined Concepts

### DK (Device Key)

Ed25519 signing key. One per device, generated once, permanent.

Invariants:
- DK private key never leaves the device that generated it.
- DK is used exclusively for signing, never for key agreement.
- DK uniquely identifies a device across the mesh lifetime.

### DEK (Device Encryption Key)

X25519 key agreement key. One per device, paired with DK.

Invariants:
- DEK is used exclusively for key agreement, never for signing.
- DEK and DK are cryptographically independent (no birational
  map derivation).

### ChannelKey

256-bit symmetric key for a communication channel. Independently
generated via CSPRNG.

Invariants:
- Each channel key is independent of all other channel keys.
- Channel keys are never derived from each other.

### EpochRatchet

Forward-only key advancement chain. HKDF-based, one direction.

Invariants:
- K_epoch_N cannot be derived from K_epoch_{N+1} (forward secrecy).
- K_epoch_N is deterministic given K_epoch_{N-1} (all peers
  holding the same key derive the same next key).
- Previous epoch keys are zeroized after advancement.

### MicroEpochRatchet

Sub-epoch key advancement for per-frame forward secrecy. Resets
on each macro-epoch advance.

Invariants:
- K_micro_M cannot be derived from K_micro_{M+1}.
- Resets to K_epoch_N at the start of each macro-epoch.
- Frame keys derived from micro-epoch keys are immediately deleted
  after use.

### FrameKey

Single-use symmetric key for one AEAD operation. Derived from
a micro-epoch key and a nonce.

Invariants:
- Each frame key is used for exactly one encrypt or decrypt.
- Frame key is zeroized immediately after use.

### TransportAEAD

ChaCha20-Poly1305. Authenticated encryption for frames in transit.

Invariants:
- 256-bit key, 96-bit sequential nonce, 128-bit auth tag.
- Nonce reuse under the same key is catastrophic (plaintext XOR
  recovery + universal forgery). Prevented by NonceSafety.

### StorageAEAD

XChaCha20-Poly1305. Authenticated encryption for data at rest.

Invariants:
- 256-bit key, 192-bit deterministic nonce, 128-bit auth tag.
- Deterministic nonces: identical inputs produce identical
  ciphertext (enables idempotent relay PUT).

### NonceSafety

Three-layer defense guaranteeing AEAD nonce uniqueness across
crashes, bursts, and restarts.

Invariants:
- No (key, nonce) pair is ever reused for encryption.
- Layer 1 (prevention): nonce high-water mark persisted before
  any burst of AEAD operations.
- Layer 2 (recovery): dynamic gap covers all nonces between
  persists.
- Layer 3 (prediction): velocity monitoring triggers early
  persist or halt.
- Fail-stop on persist failure (availability sacrificed for
  nonce uniqueness).

### DoubleWritePersist

Torn-write-safe persistence for nonce state. Two copies in
separate sectors with CRC-32C checksums.

Invariants:
- At most one copy can be torn per power failure.
- Recovery reads both copies, uses valid copy with highest
  version.
- Halt on double corruption (fail-stop).

### BroadcastEncryption

Age-style multi-recipient envelope. One content key, wrapped
to each recipient.

Invariants:
- Content encrypted once. Key wrapped N-1 times (one per
  recipient cluster).
- Header MAC binds sender identity to the envelope.
- AEAD associated data includes the envelope header.

### PCSMechanism

Post-compromise security via fresh keying material injection.
Mixes new Diffie-Hellman output into the epoch ratchet.

Invariants:
- After a PCS re-seed with uncompromised keying material,
  the adversary who knew the previous epoch key cannot derive
  the new epoch key.
- PCS requires at least one honest key exchange since the
  compromise.

### DomainSeparatorRegistry

Registry of all BLAKE3 and HKDF domain separator strings.
Prevents cross-domain key/hash collisions.

Invariants:
- No two derivations with different purposes share the same
  domain separator.
- All domain separators are versioned or structurally distinct.

### KeyZeroization

All key material is overwritten with zeros before deallocation.

Invariants:
- No key material (epoch keys, frame keys, private keys, CPSKs)
  survives in memory after its useful lifetime ends.

### FoundingHash

BLAKE3 hash of the founding transcript. Mesh identity.

Invariants:
- FoundingHash is deterministic: identical founding transcript
  produces identical hash.
- FoundingHash appears in invite tokens, snapshots, join
  confirmations.
- FoundingHash uniquely identifies a mesh across its lifetime.

### DomainSeparator

repr(C) enum enumerating all registered domain-separated hash and
derivation usages. This enum is the sole API for domain-separated
hashing in the verkko protocol. Raw BLAKE3 MUST NOT be called
directly from protocol crates; all keyed BLAKE3 usage MUST go
through a function that takes a DomainSeparator variant.

    #[repr(C)]
    enum DomainSeparator {
        KeyId         = 0,  // "saalis-keyid-v1"
        ReencNonce    = 1,  // "saalis-reenc-v1"
        FoundingHash  = 2,  // (raw BLAKE3, structurally unique)
        PhaseJitter   = 3,  // "saalis-jitter-v1"
        HbJitter      = 4,  // "saalis-hbjitter-v1"
        SessionId     = 5,  // "saalis-sessid-v1"
        DestinationId = 6,  // "saalis-destid-v1"
        ResumeToken   = 7,  // "saalis-resume-v1"
        DhtKey        = 8,  // "saalis-dht-v1"
    }

Invariants:
- No two variants share the same domain separator string.
- The domain separator string for each variant is fixed at
  compile time.
- Adding a new domain-separated hash usage REQUIRES adding a
  new variant to this enum. The enum is the registry.
- Protocol crates MUST NOT import blake3::hash or
  blake3::Hasher directly. They MUST use the verkko-crypto
  domain-separated API.
- HMAC-BLAKE3 (used for relay_key_id) is a different construction
  from keyed BLAKE3 and is explicitly excluded from this registry.

Padding rule:
    key = domain_separator_bytes || 0x00 * (32 - len(domain_separator_bytes))
    All domain separator strings are <= 32 bytes.

API surface:
    fn domain_hash(domain: DomainSeparator, input: &[u8]) -> [u8; 32];
    fn domain_hash_truncated(domain: DomainSeparator, input: &[u8]) -> &[u8];

## Body

<!-- v10 §1.1 -->
### 1.1 Identity and key model

Each peer: **DK** (Ed25519 [Bernstein2012], signing) + **DEK** (X25519 [Bernstein2006], encryption). DK and DEK are bound to a device at enrollment time; the enrollment ceremony is defined by the consumer. Each cluster shares a **CIK** (X25519, Noise/DHT) + **signing key** (Ed25519, control log/filters). CIK is a mesh-layer organizational key; crypto provides the X25519 primitive. CIK replicated to all cluster peers, encrypted to each DEK.

No K_mesh. Mesh = pairwise CPSK relationships. Cost: 11 extra AEAD ops per broadcast (2.2us at 12 clusters).

<!-- v10 §1.3 -->
### 1.3 Genesis bootstrap sequence

Deterministic state machine:

1. **Key generation.** DK, DEK, CIK, signing key from local CSPRNG. No external seeding.
2. **Self-test.** CSPRNG entropy: 32 bytes, non-zero, non-repeating across 3 samples. Failure: abort, no persistent state, actionable error.
3. **Self-enrollment.** Self-sign enrollment cert. Only self-signed enrollment ever.
4. **Founding transcript.**
   ```
   founding_transcript = LEN(DK_pub) || DK_pub || LEN(DEK_pub) || DEK_pub
                       || LEN(CIK_pub) || CIK_pub || LEN(signing_pub) || signing_pub
                       || LEN(enrollment_cert) || enrollment_cert
                       || LEN(genesis_timestamp) || genesis_timestamp
   founding_hash = BLAKE3(founding_transcript)
   ```
   Founding hash = mesh identity. Appears in invite tokens, snapshots, join confirmations. See FoundingHash defined concept.

Mesh initialization using these keys (CRDT init, control log, metabolic state) is the responsibility of the consumer.

<!-- v10 §1.4 -->
### 1.4 Device recovery flow

1. **Replacement device:** install saalis, generate new DK + DEK.
2. **Locate mesh.** Any enrolled device -> "add device" -> QR code (CIK pubkey, PSK, address hints, relay DHT key, founding hash, expiry, nonce). Noise XX [Perrin2018] (IK+XX fallback) with PSK.

The enrollment and key delivery flow (join, SAS verification, key delivery, revocation, filter sync) is the responsibility of the consumer. This section defines only the cryptographic primitives used.

<!-- v10 §1.5 -->
### 1.5 Channel keys and identification

Every channel key is an independently generated 256-bit random value. No hierarchy. Capability-based access (Dennis & Van Horn, 1966). Independent revocation per channel. Wire cost: 50 channels = 2,500 bytes/peer (15KB at 6 peers, 12 UDP segments).

**Key identification:**
```
key_id = BLAKE3(K_epoch_N || channel_index)[0..8]
```
8 bytes. Birthday collision at 2^32 keys (mesh will never reach this). Channel path, epoch, and data are inside the encrypted payload; transit peers cannot see which channel or epoch.

<!-- v10 §2.1 -->
### 2.1 Broadcast encryption

Age-style multi-recipient envelopes:
```
BroadcastEnvelope {
    // Header (29 bytes)
    version: u8,
    recipient_count: u8,
    algorithm: u8,                          // AEAD algorithm identifier
    sender_cluster_id: u32,
    key_id: [u8; 8],                        // BLAKE3 key identifier (Section 1.5)
    nonce: [u8; 12],                        // broadcast envelope nonce (fresh random)
    flags: u8,
    _padding: u8,

    // Payload
    wrapped_keys: [(u32 cluster_id, [u8; 48])],  // 52 bytes per recipient
    ciphertext: AEAD(key=dek, nonce, plaintext),
    auth_tag: [u8; 16],

    // Envelope authentication
    header_mac: [u8; 32],                   // BLAKE3-MAC(sender_DK, header || wrapped_keys)
}
// 29 + 572 + 200 + 16 + 32 = 849 bytes at 11 recipients
```

One content encryption. N-1 key wraps. At 11 recipients: 572 bytes wrapping overhead. 200-byte broadcast: 849 bytes total. Fits in one 1280-byte UDP segment. The header_mac binds the wrapped key list to the sender's identity, preventing a man-in-the-middle from substituting their own public key.

<!-- v10 §2.4 -->
### 2.4 Ratchet\<Scope\> with micro-epoch sub-ratchet

One ratchet per channel. Monotonic, forward-only. Scope names the key category as documentation, not runtime enforcement. See verkko-ops: Pattern Registry, Structure 3.

**Macro-epoch ratchet (hourly floor):**
```
K_epoch_0 = channel_key (initial random, CSPRNG)
K_epoch_N = HKDF-Expand(prk=K_epoch_{N-1}, info="epoch:advance", len=32)
```

**Micro-epoch sub-ratchet (per 100 frames):**
```
K_micro_0 = K_epoch_N  (resets to macro-epoch key on each macro-epoch advance)
K_micro_M = HKDF-Expand(prk=K_micro_{M-1}, info="micro:advance", len=32)
```

Micro-epoch counter increments every 100 frames sent on a channel. Counter resets to 0 on each macro-epoch advance (hourly). The micro-epoch counter is carried as a separate varint field in the encrypted payload (not bit-packed into the epoch u32). This preserves full u32 = 490,567 years at hourly epochs.

**Per-frame key derivation:**
```
K_frame = HKDF-Expand(prk=K_micro_M, info=nonce, len=32)
```
Frame key derived from micro-epoch key and nonce. Immediately deleted after encryption/decryption. Forward secrecy window: 100 frames (one micro-epoch).

**Key deletion.** After micro-epoch advance, previous micro-epoch key deleted. After macro-epoch advance, previous macro-epoch key deleted. Per-frame keys deleted immediately after use.

A key is ACTIVE (current epoch) or ORPHANED (post-revocation, re-encryption window, then deleted).

**Session boundaries:**
```
Session = (session_key, epoch, membership_set, frozen: bool)
Session_ID = BLAKE3(session_key || channel_path || generation)[0..8]
```

**Epoch triggers.** Event-driven with hourly floor: (1) `epoch = floor(hlc.wall_ms / 3600000)`, peers compute independently; (2) membership change: immediate with fresh key; (3) idle: all connections idle 60 seconds. Normal: local HKDF. Revocation: admin injects fresh random key via the consumer's key distribution mechanism.

**Harmonic epoch staggering:**
```
epoch_offset_ms = (BLAKE3(channel_key)[0..2] as u16) * 3600000 / 65536
```
Applied only to 7-day automatic rotation. Not to hourly advances (keyless) or revocation-triggered advances (immediate).

**Epoch sync:** G-Counter. Heartbeat carries macro-epoch (u32). Fast-forward via HKDF: 5 epochs = 2.5us. Micro-epoch sync: recipients iterate from their current micro-epoch key, computing key_id at each step, until matching the incoming frame's key_id. Maximum iteration bounded by micro-epoch checkpoint interval (720).

**Micro-epoch checkpoints.** Every 720 micro-epoch advances within a single macro-epoch, a checkpoint records the micro-epoch key. This bounds catch-up derivation to 720 HKDF calls maximum = ~90us per channel. Storage: 50 channels * 4 checkpoints/day * 34 bytes = 6,800 bytes/day.

**Micro-epoch wire cost.** The micro-epoch counter is a varint in the encrypted payload. Normal operation (counter < 128): 1 byte. High activity (counter 128-16383): 2 bytes. Total frame overhead: 54 + 1 = 55 bytes typical.

**Rotation commit.** Sender-asynchronous, no quorum. Recipient on unknown key_id: (1) check pending buffer; (2) iterate micro-epoch HKDF forward up to 720 steps, checking key_id at each step; (3) buffer frame (ring, 32/key_id, LRU); (4) control: dedicated pool, never evicted; (5) data: `max(5s, 3*jitter_99th)` then drop; (6) if > 50% allocation: include key_id hint in heartbeat.

**Dual-key window.** Non-revocation only: both keys for 3 heartbeat intervals (15s active). Disabled for revocation. **Auto rotation:** 7-day. No health-triggered acceleration.

<!-- v10 §2.6 — PCS MECHANISM (key derivation math) -->
### 2.6 CPSK-seeded post-compromise security (PCS) — mechanism

When a CPSK is renegotiated, the fresh X25519 DH output seeds an epoch re-key. The derivation per key scope:

```
re_key_contribution = HKDF-Expand(
    prk = fresh_cpsk,
    info = "epoch-reseed:" || key_scope_id,
    len = 32
)
K_epoch_new = HKDF-Expand(
    prk = K_epoch_current,
    info = "reseed:" || re_key_contribution,
    len = 32
)
```

**PCS window.** For key scopes shared by exactly two clusters: PCS window = that pair's CPSK session lifetime (24h default). For globally-shared key scopes: expected re-seed time is ~22 minutes (24h / 66 pairs at 12 clusters, geometric distribution).

**Micro-epoch interaction.** CPSK re-seed advances the macro epoch. Micro-epoch counter resets to 0.

**Cost:**
- CPU: one HKDF-Expand per key scope per re-seed: 125ns * 10 key scopes = 1.25us.

Distribution of the new epoch key is the responsibility of the consumer. This section defines only the derivation mechanism.

<!-- v10 §3.2 -->
### 3.2 Nonce safety: monotone invariant enforcement

Three independent proofs that the monotonicity invariant holds, each under different failure assumptions. Not three thresholds on a budget. Three barriers against nonce reuse with ChaCha20-Poly1305, which yields XOR of two plaintexts, enables universal forgery. Silent, retroactive, permanent.

**NonceSourceFactory (compile-time enforcement point #18).**

```rust
struct NonceSourceFactory { /* manages range allocation */ }

/// Thread-bound nonce source. Send + !Sync.
struct NonceSource {
    range_start: u96,
    range_end: u96,
    current: u96,
    burst_rate: u32,
    _thread: PhantomData<ThreadId>,
}
impl !Sync for NonceSource {}
```

Standard range allocation: event loop at [0, 2^95), re-encryption thread at [2^95, 2^96). Disjoint by construction.

Issued nonces are affine types: consumed once by AEAD encrypt, then dropped. Compile-time reuse prevention.

**Nonce zeroization.** On NonceSource drop, all state is zeroized via `SecretVec`.

#### Layer 1: Prevention (pre-batch nonce persist)

Assumption: fsync completes. Before any operation that changes AEAD throughput regime, persist nonce high-water mark.

**Three trigger cases:**
1. Before a re-encryption batch.
2. Before a donation batch.
3. Before a bulk KeyBundle distribution.

**Critical ordering:** persist MUST complete (fsync returns) before first AEAD operation of batch. Blocking fsync, not async write.

```
persisted_hwm = current_nonce + dynamic_gap
flush_to_disk(persisted_hwm)    // double-write with checksum (see below)
barrier()
```

#### Layer 2: Recovery (dynamic gap sizing)

```
gap = max(65_536, observed_burst_rate * GAP_SAFETY_FACTOR)
```

**Gap safety factor: 3.** Persist delay (worst 1.5x on eMMC under WAL pressure) and throughput ramp (1.3x on ARM from NEON pipeline warming) anticorrelate. Factor 3 provides 2.3x margin over ramp alone, 2x over persist delay alone.

**Observed burst rate: persisted.** u32 tracking max AEAD encrypt frame count between consecutive persists. Persisted alongside nonce high-water marks in the double-write persist. On crash recovery: burst_rate loaded from disk, gap computed from persisted value.

**Persist format:** N * 16-byte entries (12-byte HWM + 4-byte burst_rate) per NonceSource instance. Format is versioned (1 byte header).

#### Layer 3: Prediction (nonce velocity + adaptive persist)

Nonce velocity = AEAD encrypt operations since last persist (u64 per NonceSource, reset on persist). Does NOT count decrypts.

| Velocity / gap ratio | Response |
|---------------------|----------|
| 0.00 - 0.50 | None |
| 0.50 - 0.75 | Diagnostic log entry (canary) |
| 0.75 - 1.00 | Immediate adaptive persist; log warning |
| 1.00+ | Halt AEAD encrypt if persist has failed |

**Primary response is persist, not throttle.** Throttling closes the faucet; persist drains the tub.

**On persist failure.** NonceSource returns `Err(NoncePersistFailed)`. All AEAD encrypt halts. Decrypt continues.

#### Double-write nonce persist (eMMC atomicity)

The sector write atomicity assumption on consumer eMMC is not guaranteed by JEDEC eMMC 5.1 [JEDEC2015]. A torn write during power failure can leave nonce HWMs in an inconsistent state, causing silent nonce reuse. The double-write pattern eliminates this assumption.

**Persist file structure:**
```
Copy A at offset 0:
  [u32 version]
  [u96 event_loop_hwm]     12 bytes
  [u96 reencrypt_hwm]      12 bytes
  [u64 burst_rate_el]       8 bytes
  [u64 burst_rate_re]       8 bytes
  [u32 crc32c]              4 bytes
  Total: 48 bytes

Copy B at offset 4096 (separate sector):
  (identical structure)
```

**Write protocol:**
1. Write copy A. `fsync()`.
2. Write copy B. `fsync()`.
3. Re-read copy B and verify checksum matches.

**Recovery protocol:**
1. Read both copies. Compute CRC-32C [Castagnoli1993] checksums.
2. If both valid: use the one with the higher version number.
3. If one valid, one torn: use the valid one.
4. If both torn: halt. Refuse to start AEAD operations. Alert the admin. Fail-stop preserves nonce uniqueness at the cost of availability.

**`NonceHwm::recover` is a boundary gate** that validates both sector checksums before producing a trusted HWM value. This gate uses checksum-validated storage recovery, which is structurally different from the range-bounded fixed-point PhysicalBoundary trait. It is listed in the verkko-ops: Pattern Registry as a convention-enforced boundary gate, not a PhysicalBoundary trait implementor.

**Cost:** one additional `fsync()` per persist cycle (~2ms). Total additional I/O over a year: ~720 extra fsyncs * 48 bytes = ~34 KB. Negligible. Probability of both copies torn in same power event: ~(1/10000)^2 = 1e-8 per power failure during persist.

<!-- v10 §4.1 -->
### 4.1 Cryptographic composition

Six primitives composed across five isolation domains:

1. **Session establishment:** Noise KK/XX [Perrin2018] with CIK (X25519 [Bernstein2006, RFC7748]). Produces CPSK.
2. **Epoch ratchet:** HKDF-Expand [Krawczyk2010, RFC5869] with channel keys. Produces epoch keys. Micro-epoch sub-ratchet within each macro-epoch.
3. **Data encryption:** Transport: ChaCha20-Poly1305 [Bernstein2008, RFC8439] with per-frame keys, 96-bit sequential nonces. At-rest re-encryption: XChaCha20-Poly1305 [Arciszewski2020] with deterministic nonces, 192-bit nonces derived via BLAKE3 [Aumasson2020] with domain separation.
4. **Content routing:** BLAKE3 keyed hashing for filter roots, key identification, re-encryption nonces, content integrity.
5. **Authentication:** Ed25519 [Bernstein2012] signatures on control log, filter updates, enrollment certs. Strict verification required to prevent malleability [Chalkias2020].

**Counterpoint rules:** X25519 only in Noise + CIK replication. Ed25519 only for signing. HKDF output only as epoch/micro-epoch keys and PCS re-seed. Transport nonces are per-session monotonic counters; re-encryption nonces are deterministic BLAKE3 derivations. BLAKE3 only for non-secret hashing + keyed MAC + deterministic re-encryption nonce derivation + content integrity.

**Break matrix:** if one primitive breaks, blast radius bounded to its domain.

**Required pre-implementation work:** targeted ProVerif [Blanchet2001] verification of revocation flow (~100 lines) plus five-domain composition verification (~560 lines). Total: ~660 lines ProVerif. The composition security argument follows the universally composable (UC) framework [Canetti2001] for independent domain isolation.

**BLAKE3 domain separator registry:**

| Usage | Domain separator | Input |
|-------|-----------------|-------|
| Key identification | `"saalis-keyid-v1"` | channel key + index |
| Re-encryption nonce | `"saalis-reenc-v1"` | new key + blob_id + epoch |
| Relay key ID | (uses channel_salt via HMAC [Bellare1996, RFC2104]) | channel key + salt |
| Founding hash | (uses founding_transcript directly) | founding transcript |
| PCS re-seed contribution | (via HKDF, not raw BLAKE3) | fresh CPSK + channel_id |
| Metabolic phase jitter | `"saalis-jitter-v1"` | DK_pub + epoch |
| Heartbeat jitter | `"saalis-hbjitter-v1"` | DK_pub + epoch |

All BLAKE3 uses are listed here. Future additions must register a unique domain separator.

**BLAKE3 implementation equivalence requirement.** All peers must produce identical outputs from identical inputs for all registered domain separators. This is a correctness invariant, not a performance guideline. All implementations must use BLAKE3 as specified in the BLAKE3 paper [Aumasson2020]. Version: BLAKE3 1.x (any minor version producing identical output for the same input). Implementations must pass the following test vector table before deployment:

| Input | Domain separator | Expected output (first 8 bytes, hex) |
|-------|-----------------|--------------------------------------|
| (defined at implementation time with concrete test vectors) | each registered domain | (computed from reference implementation) |

A peer whose BLAKE3 output diverges from the reference on any registered domain separator will silently partition from the mesh. There is no runtime detection mechanism for BLAKE3 divergence. Prevention is the only defense.

**AEAD registry.** Two AEAD constructions [Rogaway2002], each in its own domain:

| Domain | Construction | Nonce | Usage |
|--------|-------------|-------|-------|
| Transport | ChaCha20-Poly1305 [RFC8439] | 96-bit sequential monotonic | Wire frames (see verkko-protocol: Frame) |
| Re-encryption | XChaCha20-Poly1305 [Arciszewski2020] | 192-bit deterministic (BLAKE3) | At-rest re-encryption (see verkko-relay: TerritorialReencryption) |

Transport nonces are sequential monotonic counters (per-sender, per-session). The counter scheme guarantees uniqueness without randomness. ChaCha20-Poly1305 is the correct choice: smaller nonce (12 bytes), no wasted entropy. Re-encryption nonces are deterministic (`BLAKE3("saalis-reenc-v1" || key || blob_id || epoch)[0..24]`). These are pseudorandom values that must avoid collision across independent peers. XChaCha20-Poly1305 is the correct choice: 192-bit nonce space provides sufficient birthday bound margin. The three-layer nonce defense applies only to transport nonces. Re-encryption nonces are unique by construction (PRF assumption on BLAKE3) and bypass the NonceSource entirely.

## References

[Arciszewski2020] Arciszewski, S. (2020). "XChaCha20-Poly1305 Construction." IRTF CFRG draft-arciszewski-xchacha.

[Aumasson2020] Aumasson, J.-P., Neves, S., O'Hearn, Z., Winnerlein, C. (2020). "BLAKE3 -- one function, fast everywhere." BLAKE3 specification.

[Bellare1996] Bellare, M., Canetti, R., Krawczyk, H. (1996). "Keying Hash Functions for Message Authentication." CRYPTO 1996.

[Bernstein2005] Bernstein, D.J. (2005). "The Poly1305-AES message-authentication code." FSE 2005.

[Bernstein2006] Bernstein, D.J. (2006). "Curve25519: New Diffie-Hellman Speed Records." PKC 2006.

[Bernstein2008] Bernstein, D.J. (2008). "ChaCha, a variant of Salsa20." SASC 2008.

[Bernstein2012] Bernstein, D.J., Duif, N., Lange, T., Schwabe, P., Yang, B.-Y. (2012). "High-speed high-security signatures." J. Cryptographic Engineering 2(2), 77-89.

[Blanchet2001] Blanchet, B. (2001). "An Efficient Cryptographic Protocol Verifier Based on Prolog Rules." CSFW 2001.

[Blanchet2008] Blanchet, B. (2008). "A Computationally Sound Mechanized Prover for Security Protocols." IEEE TDSC 5(4).

[Canetti2001] Canetti, R. (2001). "Universally Composable Security: A New Paradigm for Cryptographic Protocols." FOCS 2001.

[Castagnoli1993] Castagnoli, G., Braeuer, S., Herrmann, M. (1993). "Optimization of Cyclic Redundancy-Check Codes with 24 and 32 Parity Bits." IEEE Trans. Communications 41(6).

[Chalkias2020] Chalkias, K., Garillot, F., Nikolaenko, V. (2020). "Taming the Many EdDSAs." SSR 2020, LNCS 12529.

[JEDEC2015] JEDEC (2015). "Embedded Multi-Media Card (eMMC) Electrical Standard (5.1)." JESD84-B51.

[Krawczyk2010] Krawczyk, H. (2010). "Cryptographic Extraction and Key Derivation: The HKDF Scheme." CRYPTO 2010.

[Perrin2018] Perrin, T. (2018). "The Noise Protocol Framework." Revision 34. noiseprotocol.org.

[RFC2104] Krawczyk, H., Bellare, M., Canetti, R. (1997). "HMAC: Keyed-Hashing for Message Authentication." RFC 2104.

[RFC5869] Krawczyk, H., Eronen, P. (2010). "HMAC-based Extract-and-Expand Key Derivation Function (HKDF)." RFC 5869.

[RFC7748] Langley, A., Hamburg, M., Turner, S. (2016). "Elliptic Curves for Security." RFC 7748.

[RFC8439] Nir, Y., Langley, A. (2018). "ChaCha20 and Poly1305 for IETF Protocols." RFC 8439.

[Rogaway2002] Rogaway, P. (2002). "Authenticated-Encryption with Associated-Data." CCS 2002.
