# verkko-seat

## Abstract

Thin client tunnel for the verkko ecosystem. How a lightweight
client (web UI, mobile app, terminal) connects to a mesh peer
without joining the mesh as a full participant.

A seat sees the mesh through one peer's eyes. It does not run
gossip, health scoring, convergence, or key management. It
authenticates to a host peer and communicates via request/response
and event subscription.

This avoids running multiple verkko peers on the same device.
One peer serves multiple seats (phone, tablet, web browser)
simultaneously.

## Dependencies

- **verkko-crypto**: TransportAEAD, KeyZeroization
- **verkko-protocol**: Connection, Session, Frame, Stream,
  SchemaSync, SansIO

Does NOT depend on verkko-mesh. The seat does not know about
the mesh. The host peer translates between the seat's
request/response world and the mesh's collective state.

## Defined Concepts

### Seat

A thin client connected to a single host peer via an
asymmetric session.

Invariants:
- A seat is NOT a mesh peer. It does not enroll, does not
  participate in dominance, does not hold channel keys, does
  not run gossip.
- A seat authenticates to one host peer. If that peer goes
  offline, the seat is disconnected.
- A seat has a user identity and a role. The host peer maps
  the role to mesh-level access control.
- Multiple seats can connect to the same host peer
  simultaneously.

### SeatSession

An asymmetric authenticated session between a seat and its
host peer.

Invariants:
- Seat authenticates with a user credential (not a DK/DEK —
  seats do not have device keys).
- Session is encrypted (TransportAEAD) but the seat does not
  participate in mesh key management.
- Session can be resumed (0-RTT) via verkko-protocol's
  SessionResumption. Only idempotent SeatRequests are
  permitted in the 0-RTT window.

### HostPeer

The mesh peer that a seat connects to. Translates between
the seat's request/response model and the mesh's state.

Invariants:
- Host peer handles all mesh operations on behalf of the seat.
- Host peer enforces access control: the seat's role determines
  what queries and operations are permitted.
- Host peer maintains mesh continuity regardless of seat
  connect/disconnect events.

### SeatRequest

A request from seat to host peer. Typed via protobuf schema.

Invariants:
- Request/response is the primary interaction model (not
  pub/sub, not streaming).
- Each request has a correlation ID for matching responses.
- Requests are access-controlled by the seat's role.

### SeatEvent

A push notification from host peer to seat. The seat
subscribes to event types.

Invariants:
- Events are pushed, not polled.
- Seat subscribes to event categories during session setup.
- Events carry enough context for the seat to update its
  local view without additional queries.

### SeatInstruction

A delegated operation from host peer to seat. The seat
executes filesystem or platform operations that the host
peer cannot do remotely.

Invariants:
- Instructions are idempotent (safe to retry).
- Instruction types: download, place, link, remove, verify,
  inventory sync.
- Seat reports completion or failure. Host peer tracks
  instruction state.

### SeatProgress

Progress reporting from seat to host peer during long
operations (downloads, bulk transfers).

Invariants:
- Streamed incrementally, not batched.
- Host peer aggregates and may relay to other seats or
  mesh observers.

### SeatTransport

The transport binding between a seat and its host peer.

Invariants:
- The seat connects via WebSocket to the host peer's HTTP server.
- WebSocket subprotocol: "verkko-seat-v1".
- Each WebSocket binary message carries exactly one verkko protocol
  frame (outer header + encrypted payload).
- Noise handshake (KK or XX) is performed inside the WebSocket
  tunnel. First WebSocket messages carry handshake frames;
  subsequent messages carry post-handshake data frames.
- WebSocket text messages are not used.
- WebSocket close is equivalent to CONN_CLOSE with reason NORMAL.
- Frame size limit of 1232 bytes is preserved regardless of
  WebSocket capacity.
- TLS is required (browser policy).
- Tunnel mode (v1): full verkko protocol stack over WebSocket.
  SACK and retransmission mechanisms operate but have no practical
  effect (WebSocket/TCP provides reliability). The implementation
  MAY skip retransmission timers for sessions known to be on
  reliable transports.
- The SansIO consumer interface handles WebSocket-delivered bytes
  identically to UDP-delivered bytes.

### Seat Failover

When the host peer's WebSocket connection closes unexpectedly, the seat
attempts reconnection:

1. Retry the last known host address.
2. Try alternate hosts from SeatAuthResult.alternate_hosts.
3. Discover hosts via mDNS (_verkko-seat._tcp.local.).
4. Retry with exponential backoff: 1s, 2s, 4s, 8s, max 30s.

SeatAuthResult extension:

```
alternate_hosts: [AlternateHost; count]

AlternateHost {
    address: [u8; 20],  // AddressHint format
    host_id: u32,
}
```

Cost: 24 bytes per alternate host. At 5 cluster peers: 120 bytes, once
per session establishment. Failover timing: WebSocket close detection
(1-5 seconds) + reconnection (~10ms LAN) = 1-6 seconds.

### HostPeer Capability Enumeration

The host peer MUST be able to perform the following mesh operations
on behalf of connected seats:

1. Filter query: query the local cuckoo filter for content
   matching seat-provided criteria.
2. Channel access check: determine whether the seat's user has
   access to a given channel based on role and ACL.
3. Matter transfer initiation: initiate a MatterTransfer from a
   source peer on behalf of the seat.
4. Control log query: read control log entries visible to the
   seat's user role.
5. Health pipeline query: provide mesh health status appropriate
   to the seat's user role.
6. Configuration update: apply settings changes submitted by an
   admin-role seat.

The orchestration of these operations is the responsibility of the
application layer. This document does not specify orchestration
because it is application-specific.

Note: HostPeer translation from SeatRequest to mesh operations
is an application-layer responsibility, not a verkko protocol
concern.

## Body

### Authentication and session establishment

A seat authenticates to a host peer using a user credential, not
a device key (DK/DEK). The host peer validates the credential and
establishes an asymmetric session.

#### Credential types

    #[repr(u8)]
    enum SeatCredentialType {
        Password        = 0x01,  // Argon2id-hashed password
        ResumptionToken = 0x02,  // 0-RTT session resumption token
        PIN             = 0x03,  // Numeric PIN (Child role)
    }

#### Session establishment (full handshake)

1. Seat initiates a Noise_KK or Noise_XX handshake to the host
   peer's local address (LAN only; seats do not connect over WAN).
2. After Noise handshake completes, the seat sends a SeatAuth
   message in the encrypted channel.
3. The host peer validates the credential and returns SeatAuthResult.
4. On success, the seat sends a SubscriptionRequest listing the
   event categories it wants to receive.
5. The host peer confirms subscriptions and sends an initial
   state snapshot.

#### SeatAuth message (variable length):

    Offset  Width  Field
    ------  -----  -----
     0      1      credential_type: u8 (SeatCredentialType)
     1      1      role_requested: u8 (0=Admin, 1=User, 2=Child)
     2      2      credential_len: u16 (big-endian)
     4      16     user_id: [u8; 16] (UUID)
     20     4      schema_hash: u32 (CRC-32C of protobuf schema)
     24     2      client_version: u16 (seat protocol version)
     26     var    credential: [u8; credential_len]

#### SeatAuthResult message (variable length):

    Offset  Width  Field
    ------  -----  -----
     0      1      status: u8
                    0x00 = SUCCESS
                    0x01 = AUTH_FAILED (bad credential)
                    0x02 = ROLE_DENIED (requested role not allowed)
                    0x03 = TOO_MANY_SEATS (host at capacity)
                    0x04 = VERSION_MISMATCH (seat version too old)
     1      1      granted_role: u8 (actual role granted; may
                   differ from requested if downgraded)
     2      8      session_id: u64
     10     32     resumption_token: [u8; 32] (for 0-RTT resume)
     42     var    schema_update: [u8; ?] (present only if
                   schema_hash did not match; protobuf schema)

#### Session resumption (0-RTT)

A seat with a valid resumption_token MAY skip the full handshake:

1. Seat sends Noise_KK message 1 with the resumption_token
   in the encrypted payload.
2. Host peer validates the token (not expired, not revoked).
3. If valid: session resumes. No SeatAuth exchange needed.
4. If invalid: host peer responds with RESUME_FAILED and the
   seat falls back to full handshake.

Resumption token validity: 24 hours or until the host peer
restarts, whichever comes first.

### Consistency model

The seat operates in one of two consistency modes:

    #[repr(u8)]
    enum SeatConsistencyMode {
        Consistent = 0,
        Snapshot   = 1,
    }

**CONSISTENT mode** (default). The host peer serves current mesh
state. Reads reflect the latest committed state. Writes are
applied immediately and propagated to the mesh.

**SNAPSHOT mode** (during convergence). When the host peer enters
convergence (convergence_active=1), it MUST switch the seat to
SNAPSHOT mode:

1. Host peer sends a ConsistencyModeChange event to all
   connected seats.
2. Reads return a frozen snapshot taken at convergence entry.
3. Writes are queued in a deferred-write buffer on the host peer.
4. All responses include a staleness indicator:
   `staleness_ms: u32` (milliseconds since snapshot was taken).
5. When convergence completes, host peer replays the deferred
   writes (in order received) and switches back to CONSISTENT.

**Deferred write semantics:**
- Writes are replayed in FIFO order after convergence.
- If a deferred write conflicts with a state change during
  convergence (e.g., the target entity was modified by the mesh),
  the host peer applies last-writer-wins with the mesh state
  winning. The seat is notified of the conflict via a
  WriteConflict event.
- Maximum deferred write queue: 1024 entries. If the queue
  is full, additional writes return DEFERRED_QUEUE_FULL and
  the seat MUST NOT retry until convergence ends.

**Convergence duration bound.** The host peer MUST NOT remain
in SNAPSHOT mode for longer than 600 seconds (configurable,
default 120 seconds per wavefront deadline). If convergence
exceeds this bound, the host peer proceeds with available
state and transitions back to CONSISTENT.

### Snapshot Semantics During Convergence

The snapshot is taken at the HLC timestamp when the host peer enters
convergence (convergence_active = 1). The snapshot reflects all
committed state with HLC <= snapshot_hlc. The staleness_ms field in
SeatResponse is: host_peer_hlc.now() - snapshot_hlc.

Deferred writes carry the seat's local HLC timestamp. After
convergence, deferred writes are replayed in HLC order. Conflict
resolution: mesh-wins for security state (KeyBundle, access control);
seat-wins for user content metadata (ratings, tags, descriptions).
The host peer emits WriteConflict events for security-state conflicts
so the seat can display the resolution to the user.

### Request/Response protocol

SeatRequests are the primary interaction model. Each request
carries a correlation_id for matching responses.

#### SeatRequest envelope:

    Offset  Width  Field
    ------  -----  -----
     0      4      correlation_id: u32 (unique per session)
     4      2      request_type: u16
     6      2      payload_len: u16 (big-endian)
     8      var    payload: [u8; payload_len] (protobuf-encoded)

#### SeatResponse envelope:

    Offset  Width  Field
    ------  -----  -----
     0      4      correlation_id: u32 (matches request)
     4      1      status: u8
                    0x00 = OK
                    0x01 = NOT_FOUND
                    0x02 = FORBIDDEN (role insufficient)
                    0x03 = INVALID_REQUEST
                    0x04 = INTERNAL_ERROR
                    0x05 = DEFERRED_QUEUE_FULL
                    0x06 = CONVERGENCE_ACTIVE (write blocked)
     5      4      staleness_ms: u32 (0 in CONSISTENT mode)
     9      2      payload_len: u16 (big-endian)
    11      var    payload: [u8; payload_len] (protobuf-encoded)

#### Access control

The host peer enforces access control based on the seat's
granted role:

    Role    Permissions
    ----    -----------
    Admin   All operations.
    User    Read all. Write own profile. Manage own downloads.
            Cannot modify mesh configuration, enroll devices,
            or manage other users.
    Child   Read non-restricted content. No writes except
            download requests (subject to parental controls).

### Event subscription

Seats subscribe to event categories during session establishment.

#### SubscriptionRequest:

    Offset  Width  Field
    ------  -----  -----
     0      2      category_count: u16
     2      var    categories: [u16; category_count]

#### Event categories:

    Category ID  Name              Description
    -----------  ----              -----------
    0x0001       CONTENT_CHANGE    Entity added, modified, removed
    0x0002       DOWNLOAD_PROGRESS Download state changes
    0x0003       MESH_STATUS       Peer join/leave, convergence
    0x0004       NOTIFICATION      User-facing notifications
    0x0005       HEALTH_SUMMARY    Periodic health digest
    0x0006       CONSISTENCY_MODE  CONSISTENT/SNAPSHOT transitions

#### SeatEvent envelope:

    Offset  Width  Field
    ------  -----  -----
     0      2      category: u16
     2      4      event_id: u32 (monotonic per category)
     6      2      payload_len: u16
     8      var    payload: [u8; payload_len] (protobuf-encoded)

Events are pushed. The seat does not poll. If the seat cannot
keep up (receive buffer full), the host peer MAY drop non-vital
events and send a EVENTS_DROPPED event with the count of dropped
events.

### SeatInstruction protocol

Instructions are delegated operations from host to seat.

#### SeatInstruction envelope:

    Offset  Width  Field
    ------  -----  -----
     0      4      instruction_id: u32 (unique per session)
     4      1      instruction_type: u8
                    0x00 = DOWNLOAD (fetch matter from external source)
                    0x01 = PLACE (move file to target path)
                    0x02 = LINK (create symlink)
                    0x03 = REMOVE (delete file)
                    0x04 = VERIFY (check file integrity)
                    0x05 = INVENTORY_SYNC (report local filesystem state)
     5      2      payload_len: u16
     7      var    payload: [u8; payload_len] (protobuf-encoded)

#### SeatInstructionResult:

    Offset  Width  Field
    ------  -----  -----
     0      4      instruction_id: u32 (matches instruction)
     4      1      status: u8
                    0x00 = COMPLETED
                    0x01 = FAILED (see error_code)
                    0x02 = IN_PROGRESS (see progress field)
     5      2      error_code: u16 (0 if not failed)
     7      var    detail: [u8; ?] (error message or progress)

Instructions are idempotent. The seat MUST be able to receive
the same instruction_id twice and produce the same result without
side effects beyond the first execution.

### SeatProgress reporting

    Offset  Width  Field
    ------  -----  -----
     0      4      instruction_id: u32
     4      8      bytes_completed: u64 (big-endian)
     12     8      bytes_total: u64 (big-endian)
     20     2      rate_kbps: u16

Progress messages are streamed incrementally. The host peer
aggregates and MAY relay to other seats or mesh observers.

### Error code table

    Code    Name                 Recovery action
    ----    ----                 ---------------
    0x0000  OK                   None.
    0x0001  AUTH_FAILED          Re-enter credentials.
    0x0002  ROLE_DENIED          Request lower role or contact admin.
    0x0003  TOO_MANY_SEATS       Disconnect another seat or wait.
    0x0004  VERSION_MISMATCH     Update seat software.
    0x0005  RESUME_FAILED        Full handshake.
    0x0010  NOT_FOUND            Entity does not exist. No retry.
    0x0011  FORBIDDEN            Role insufficient. No retry.
    0x0012  INVALID_REQUEST      Fix request. No retry.
    0x0013  INTERNAL_ERROR       Retry after backoff. Report bug if persistent.
    0x0014  DEFERRED_QUEUE_FULL  Wait for convergence to end.
    0x0015  CONVERGENCE_ACTIVE   Write blocked. Wait or read-only.
    0x0020  EVENTS_DROPPED       Re-subscribe or request full state.
    0x0030  INSTRUCTION_FAILED   Check error_code in detail field.
    0x0031  DISK_FULL            Free space on seat device.
    0x0032  PERMISSION_DENIED    Check filesystem permissions.
    0x0033  INTEGRITY_FAILED     Re-download. Report to host for scar.
