# Findings: six-variant resource-storage-model bench

**Date:** 2026-06-20
**Round:** 202606210600. Resolves the resource-storage layout fork by bench, per the op
decision in `202606210600_synthesis-resource-storage-model.md` (bench all six V0-V5; hold
storage impl until the bench decides).
**Bench:** `mock/benches/resource_storage/` run through the mockspace bench harness
(`mockspace-bench-core` / `-harness` / `-macro`, rev `48d0279`): subprocess-isolated variant
cdylibs, hardware-counter timing (`CNTVCT_EL0`), cross-variant byte-exact validation, bootstrap
CI + sign test + quintile analysis, dylib-hash result cache. Outputs land in the benches root as
`rsb_*_n*.csv` + `rsb_*_n*_findings.md`, alongside the other benches. Re-runnable via the
orchestrator binary (or `mock bench`). Axes C and E are tests against the real provider / real
threads (`mock/benches/resource_storage/axis_ce/`), since they are a capacity assertion and a
contention measurement, not per-call latency benches.
**Host:** Apple M1 (aarch64), `rustc 1.98.0-nightly (cced03bfd 2026-05-28)`.
**Timing budget:** 5 passes x 2000 runs x 3 harness_runs, single 0ms cooldown cohort, batched
into thousands of timed samples per variant. The harness reports a median and quintiles per
variant; the best-20% is the low-noise estimator and is cited where the means carry mix-in tail
variance.

## Approach, and the modelling boundary

The bench runs through the mockspace bench framework, the same harness the other stack benches
use, not a hand-rolled timing loop. Each variant is a cdylib that builds its storage layout in
the untimed setup region and runs the morsel loop in the timed region; the harness surrounds the
timed call with a configurable realistic-workload mix-in (scalar dep chains, pointer-chase graph
work, L1-evicting heavy-memory passes, branch pressure), validates the 8-byte FNV-1a checksum
byte-exact across variants (so a measured ratio only ever compares arms proven to compute the
identical result), and does the statistics.

It does not wire the whole engine: per the synthesis the layout fork turns on the storage
layout's codegen and cache behaviour, which the modelled kernels reproduce faithfully. The
workload is one resource with M scalar `Field` members + one `Seq` member + one `Map` member, an
input column, an output column; per record the kernel reads the members, sums the Seq + Map,
reads the input, computes a shared multiply-xor chain, writes the output.

The load-bearing fairness property: every variant derives its resource pointer(s) AND its data
columns from one shared bump `Arena`, mirroring the shipped `ArenaColumnStorage` (every column
bumped from one backing block). A write to the output column and a read of a resource member
therefore share the arena's provenance, the aliasing condition the spec's noalias win defeats.
Distinct buffers would hand LLVM trivial non-aliasing and refute axis A falsely. Verified by the
V0/V1 disasm below.

Marked infidelities: M (scalar member count) is a compile-time const (faithful: a real
`Resource<T>` has a const-derived `Decompose::Leaves` layout, and a runtime length would itself
prevent the residency axis A depends on). The scalar workload's Seq/Map lengths are const (their
sums fold once before the record loop, so this does not touch axis A); axis D sweeps the Seq
length as a runtime payload, which is the in-model large case.

## The six variants

- **V0** one-record opaque blob per resource, read live through a pointer every iteration (the
  shipped `DrainStores` status quo, no snapshot). Control.
- **V1** same blob, value snapshotted to a stack local before the morsel loop (spec 1714-1724).
- **V2** type-unique decomposed columns: each member its own scattered one-record column; snapshot.
- **V3** shape-bound shared columns: members share a column by resource-slot stride; snapshot.
- **V4** loimu-style type-erasure-via-shaping: members in a type-erased byte store, backcast on
  access; snapshot.
- **V5** runtime handle-table: a flat runtime per-leaf store-id table resolved at runtime; snapshot.

## Axis A: register-residency / noalias

The spec's 1.28-1.40x noalias win is the property the bench was built to reproduce or refute.
The perf expert flagged it is NOT a current-repo measurement (inherited from a March-2026
distillation, twice-removed from evidence, and the snapshot is unimplemented in shipped code).
This is the measurement.

The codegen reproduces the mechanism exactly. V0's record loop reloads the blob members every
iteration (verbatim `bench_entry` loop body of `libv0_blob.dylib`, `0x914..0x964`):

```
914: ldp  q0, q1, [x12]      // blob members reload (SIMD)
918: ldp  w0, w1, [x12, #0x20]// blob members reload
91c: ldr  w2, [x13], #0x4    // In column (post-increment walk)
920: ldur q2, [x12, #0x28]   // blob members reload (SIMD)
924: addv.4s s0, v0          // fold the reloaded members
...
95c: str  w0, [x14], #0x4    // Out write (post-increment walk)
964: b.ne 0x914              // loop back
```

`x12` is the blob pointer; LLVM cannot prove the prior `str w0, [x14]` to Out left the blob
unchanged (they share arena provenance), so it reloads the members on each iteration. V1's
`bench_entry` reads all the blob members ONCE before the record loop, folds them into registers
and immediates, and the record loop streams only the In column with ZERO blob reads (the members
ride a `dup.4s` broadcast). All five snapshot variants (V1-V5) show the full hoist; the disasm
confirms V0 reloads and V1-V5 hoist. The property survived the cdylib refactor (fat-LTO
cross-crate inlining preserves it).

The wall-clock impact is nil for the few-small-scalar workload on this host. The record loop is
memory-bound on the streaming In/Out columns, and V0's reload hits the L1-hot blob (one cache
line), so the reload costs nothing measurable. Function-under-test medians vs V0 (best-20% in
parens, the low-noise estimator), clean workload:

| N (bytes) | V0 | V1 | V2 | V3 | V4 | V5 |
|---|----|----|----|----|----|----|
| 1024 | base (28) | -2.9% (28) | +0.6% (28) | -0.6% (28) | +0.4% (28) | +8.0% (30) |
| 262144 | base (6657) | -0.4% (6649) | +0.0% (6651) | +0.5% (6641) | +1.5% (6662) | +0.8% (6698) |

V0-V4 are at parity (best-20% within ~1.5%); only V5 separates, and only at small N (+8% at
N=1024, where the runtime handle-table's double-indirection setup is a larger share of a tiny
per-call cost; it washes to +0.8% by N=262144).

**Mix-in stress does not crack scalar A.** Under the L1-evicting heavy-memory mix-in the
best-20% stays at parity for V0-V4 (the small blob is re-read every iteration and survives the
between-call eviction); the heavy-memory pass only adds tail variance to the means (V1's mean
swings to +12% at N=16384 while its best-20% is 468ns vs V0's 463ns, i.e. parity). The honest
reading: the snapshot mechanism is real in codegen, and wall-clock-neutral for small scalar
members on this microarchitecture.

**Axis A verdict:** mechanism confirmed (V0 reloads, V1-V5 hoist), wall-clock parity for the
few-small-scalar case. The 1.28-1.40x is NOT reproduced as a scalar wall-clock effect on M1; the
honest outcome is reload-vs-hoist proven in disasm, wall-clock-nil for this regime/host. V5's
runtime indirection is the only scalar-path cost, and only at small N. The snapshot is worth
keeping as free insurance: parity now, real on uarches where the reload is not L1-cheap, and
decisive for large collection members (axis D).

## Axis B: intra-resource read locality (M=64, contiguous blob vs scattered columns)

At M=64 the locality difference between a contiguous blob and scattered per-member columns (one
cache line per leaf) shows. The timed region is the member gather repeated, not a morsel loop
(which would drown the gather in column streaming). Function-under-test mean, decomposed vs blob:

| N (drives pass count) | wide_blob | wide_decomposed |
|---|---|---|
| 256 | base | +73.7% |
| 16384 | base | +100.5% |
| 4194304 | base | +208.0% |

Decomposed (V2's layout) costs 1.7x to 3.1x the contiguous blob for the intra-resource gather: M
scattered cache lines vs one. This is not an L1-residency artifact (it separates at every size
and grows). The blob wins intra-resource locality outright at high member arity.

**Axis B verdict:** the blob (V0/V1) and the contiguous erased store (V4) win intra-resource
locality; the scattered decomposed layout (V2) loses 74% to 208% at M=64. Moot at M=4 (a small
resource is L1-resident regardless), decisive at high arity.

## Axis C: column-count / slot-table capacity (test, not a latency bench)

A capacity assertion against the real `hilavitkutin_providers::ArenaColumnStorage<_, Dim<256>>`
(`axis_ce/` test crate). The per-member-column layouts (V2 decomposed, V5 handle-table) reserve
one column per scalar member, so a resource set's distinct-column count is resources x (members
+ collections); the per-resource layouts (V0/V1 blob, V3 shared, V4 erased) reserve one column
per resource.

The test confirms: 40 resources x (4 members + 2 collections) = 240 columns reserve fine for
V2/V5; 50 resources = 300 columns hits `StorageError::IdOutOfRange` (crosses the engine's 256-
store cap). The per-resource layouts at 40 resources need only 40 columns (V0/V1/V4) or 3 (V3
shares one column), nowhere near the cap.

**Axis C verdict:** V2 and V5 multiply the column count by member arity and cross the `Dim<256>`
cap on realistic resource sets; V0/V1/V3/V4 do not. A hard mark against the per-member-column
layouts for anything but tiny resource sets.

## Axis D: Seq/Map collection-member payload (live-stream vs snapshot-copy)

The one in-model large resource payload is a `Seq`/`Map` collection member (scalar Fields stay
few and small). This is the expert's unbuilt Bench D and the addendum's open fork: when a
resource carries a large collection, does the snapshot mechanism (copy the collection to a local
buffer before the loop, the V1 shape) still pay, or lose to streaming it live from its column
(the V0 shape)? The arm folds the whole Seq per pass, live-stream vs snapshot-copy, byte-
identical output, Seq length swept from L1-resident to 64 MiB. (The bench uses a seed-driven
custom `Routine` so the payload is heap-allocated in the variant from a tiny seed, not carried as
a multi-MiB FFI byte array; `ByteRoutine`'s stack-allocated `[0u8; N]` overflows the stack past a
few MiB, captured as a mockspace follow-up below.)

Function-under-test, snapshot-copy vs live-stream:

| Seq elements | Seq bytes | live (base) | snapshot Δ |
|---|---|---|---|
| 65536 | 256 KiB | base | +2.0% |
| 1048576 | 4 MiB | base | +30.0% |
| 4194304 | 16 MiB | base | +135.6% |
| 16777216 | 64 MiB | base | +146.6% |

Once the Seq exceeds cache the live-stream beats the snapshot-copy by up to 2.5x: the snapshot
pays a full L-element copy up front, while live streams the collection in place. Below ~4 MiB
they are at parity (the copy fits in cache and overlaps the fold). (Sizes past 64 MiB exceed the
harness 300s per-call timeout at this budget; 64 MiB establishes the trend decisively.)

**Axis D verdict:** for large collection members the live-stream (V0 shape) beats snapshot-copy
(V1 shape) by up to 2.5x. The snapshot is correct only for small scalar members; a Seq/Map
member must NOT be snapshot-copied wholesale, it should be streamed live (the snapshot holds the
ptr+len, not the elements). This is the "residency is fine until megabytes" regime.

## Axis E: V3 cross-core false-sharing (test, not a latency bench)

A contention measurement with real threads (`axis_ce/` test crate). V3's shared shape-bound
column means several resources' members live in one column; when resources owned by different
cores write their members, those members can share a cache line. The test runs
`available_parallelism` threads (8 on this host), each owning a distinct slot, in two layouts:
packed (all slots on one 64-byte line, the V3 hazard) vs padded (each slot on its own line, the
per-resource layouts).

Result: packed 164.4ms vs padded 50.0ms, a **3.29x** false-sharing penalty. The per-resource-line
layouts (V0/V1 blob, V2 decomposed, V4 erased) give each resource its own line and do not exhibit
it.

**Axis E verdict:** V3 (shape-bound shared) carries a 3.29x cross-core false-sharing penalty no
other variant has. Combined with the expert's shared-column-resize hazard (a resize of one shared
column invalidates every sharer's handle), a structural mark against V3 for any parallel resource
workload.

## Per-axis winners

- A (register residency): mechanism confirmed for V1-V5 (hoist), V0 reloads; wall-clock parity
  for V0-V4 on small scalars, V5 +8% at small N (runtime indirection). No scalar wall-clock
  winner among V0-V4; snapshot is free insurance.
- B (intra-resource locality): blob (V0/V1) and erased (V4) win; decomposed (V2) loses 74-208% at
  M=64.
- C (column count): V0/V1/V3/V4 win (constant column count); V2/V5 cross the 256-store cap.
- D (large collection payload): live-stream (V0 shape) wins by up to 2.5x; snapshot-copy (V1
  shape) must not be used for large collections.
- E (false-sharing): V0/V1/V2/V4 win (per-resource line); V3 loses 3.29x.

## Recommended winner: one-record blob + stack-local scalar snapshot, live-stream collections

The structural finding: post-snapshot the scalar hot loops converge (V0-V4 at parity on axis A),
so the per-record SCALAR layout question is decided by the non-hot-loop axes, and there the data
discriminates cleanly:

- **V2 and V5 lose axis C** (column count crosses the 256-store cap), V2 loses axis B (scatter,
  +74-208% at M=64), V5 loses axis A small-N (+8%, runtime indirection). Reject for non-tiny
  resource sets.
- **V3 loses axis E** (3.29x false-sharing) plus the resize-invalidates-all-sharers hazard.
  Reject for parallel resource workloads. Its only upside (column-count suppression) is moot
  against a 256-slot store for the few-resources premise.
- **V4 (erased)** matches the blob on B and C but adds the backcast machinery + type-erasure
  round-trip soundness burden for no measured benefit over the blob in the singleton regime.
- **V0 (blob, live)** and **V1 (blob, snapshot)** win or tie B, C, E and are at parity on scalar
  A. V1 adds the snapshot, free at small payload and correct insurance against non-L1-cheap
  reloads.

Recommendation: the **one-record blob (per-resource contiguous value) with a stack-local snapshot
of the small scalar members before the morsel loop** (V1) for the few-small-scalar case, which is
the synthesis's conservative-proven end and the expert's "blob wins the structural singleton
case." It wins or ties every axis and carries the noalias invariant as a stated architectural
guarantee.

The regime split, from axis D: a **large Seq/Map collection member must be streamed live (V0
shape), not snapshot-copied** (live wins by up to 2.5x at 64 MiB). The snapshot copies only the
small scalar Fields and the collection ptr+len, never the collection elements. So the shipped
design is V1 for scalar members + V0-style live streaming for collection members, on a one-record
blob backed by the existing arena, with the noalias invariant (handle store never aliases value
columns; snapshot scalar resource members before the loop) stated explicitly.

This matches the synthesis's locked direction (resource is a handle; the `DrainStores` blob is
drift only in lacking the snapshot; the fix wires the snapshot + the Decompose seam) and keeps
the layout at the blob the bench shows wins the structural case, not the decomposed/shape-bound/
erased layouts that lose on B/C/E for no hot-loop benefit.

## Fairness caveats the main agent should scrutinize

1. **Single-arena provenance is the load-bearing fairness property.** All variants derive
   resource + column pointers from one bump arena, so the aliasing condition is real (verified by
   V0 reloading / V1 hoisting in the disasm). Confirm the structure maps to the real engine (one
   arena per fiber; columns + resource value bumped from it).
2. **Modelled, not engine-wired.** Each variant is a layout + accessor, not a real `Scheduler`
   run (the synthesis sanctioned this; the fork is about layout codegen/cache). Morsel-windowing,
   phase analysis, and dispatch are not modelled. If the real dispatch re-resolves a resource per
   morsel, axis A could differ (snapshot per-morsel rather than once); worth checking against the
   real dispatch path.
3. **Apple M1, single host.** The scalar axis-A parity is host-specific (the reload is L1-cheap
   here). On x86 or a costlier-L1 uarch the snapshot could show a real win even for scalars. The
   mechanism (reload vs hoist) is host-independent; the wall-clock nil-effect is not. The
   recommendation keeps the snapshot precisely because it is free insurance.
4. **Axis A scalar parity vs B/D separation.** Scalar layouts are parity because the few small
   members are L1-resident and the snapshot mechanism is wall-clock-neutral there; B and D
   separate because they stress arity (B, M=64 scatter) and payload size (D, MiB collections), the
   regimes where layout actually costs. Both readings are honest; neither contradicts.
5. **Median vs mean under mix-in.** The heavy-memory mix-in adds tail variance to the means; the
   best-20% / median is the low-noise signal and the doc reads it as primary.
6. **C and E are tests, not latency benches.** A capacity assertion and a contention measurement
   against the real provider / real threads (`axis_ce/`), not byte-routine latency benches, so
   they live as tests rather than in the harness matrix (the harness is for comparable per-call
   latency). E is the borderline case (it is a timing comparison); it is a test because it
   measures cache-coherence contention, not a per-call latency ranking.

## Run-2 confirmation and final synthesis (2026-07-02)

Run-2 is a second full pass of the identical 16-arm matrix on the same host, independent of run-1
(the harness `run` path does not consult the dylib cache, and no `.bench_cache` exists in the
bench dir, so no run-1 sample leaks into run-2). Total 4478.5s (~74 min), all 16 arms completed,
byte-exact checksum validation passed on every arm. Both runs are archived under
`mock/benches/runs/run1` and `mock/benches/runs/run2`; the cross-run aggregation is
`mock/benches/resource_storage/aggregate.py` (median/min per (section, n, variant), ratio vs
baseline, run-to-run delta, plus the V4/V1 tiebreak table). Axes C and E were re-run as tests
after the harness exited (running them concurrently would perturb the memory-bound arms).

Every axis verdict from run-1 reproduces. The best-20% (min) is read as the low-noise estimator
where the means carry mix-in tail variance; run-to-run agreement is reported on it.

- **Axis A (scalar residency).** V0-V4 at parity on the best-20% across both runs at every size
  (clean/light/heavy, N = 1024/16384/262144); the largest V0-V4 spread on any arm's best-20% is
  under 3%. V5 (runtime handle-table) is the only separator and only at small N (+8% to +16% on
  the N=1024 best-20%, washing to under +1% by N=262144), the runtime double-indirection setup as
  a share of a tiny per-call cost. Confirmed: no scalar wall-clock winner among V0-V4; the
  snapshot is wall-clock-neutral for small scalars on M1; V5's runtime table is the one scalar
  cost.
- **Axis B (intra-resource locality, M=64).** `wide_decomposed` vs `wide_blob`, both runs:
  N=256 1.90x / 1.92x, N=16384 1.87x / 1.89x, N=4194304 3.03x / 3.06x. Rock-stable across runs
  (within 1.5% on every arm). The scattered decomposed layout costs 1.9x to 3.1x the contiguous
  blob for the member gather; confirmed decisive at high member arity.
- **Axis C (column count).** Re-asserted (deterministic capacity check): 240 columns reserve
  fine, 300 hits `StorageError::IdOutOfRange` at the `Dim<256>` cap. V2/V5 (per-member columns)
  cross it on realistic resource sets; V0/V1/V3/V4 (per-resource) do not.
- **Axis D (large collection payload, live-stream vs snapshot-copy).** The headline axis, and the
  one that most tightly reproduces. seq_live vs seq_snapshot median (best-20% min in parens),
  both runs:

  | Seq bytes | live vs snapshot run1 | live vs snapshot run2 | best-20% run2 |
  |---|---|---|---|
  | 256 KiB | 0.978x | 0.986x | 0.99x (parity, fits cache) |
  | 4 MiB | 0.781x | 0.748x | 0.81x |
  | 16 MiB | 0.422x | 0.412x | 0.44x |
  | 64 MiB | 0.384x | 0.394x | 0.40x |

  The seq_live and seq_snapshot best-20% minima agree across runs within under 1% at every size
  (e.g. 64 MiB: seq_live min 1519560ns run1 / 1509341ns run2; seq_snapshot min 3751180ns run1 /
  3762342ns run2). Live-stream beats snapshot-copy by ~2.5x once the collection exceeds cache,
  parity below ~4 MiB. Confirmed twice, tightly. This is the load-bearing shape finding: a
  Seq/Map member must be streamed live, never snapshot-copied wholesale.
- **Axis E (V3 cross-core false-sharing).** Second data point: packed 157.9ms vs padded 46.0ms =
  **3.44x** penalty (run-1 was 3.29x). Both in the 3.3x-3.4x band; the shape-bound shared column
  carries a false-sharing penalty no per-resource-line layout has. Confirmed.

### V1-vs-V4 tiebreak, resolved

The tiebreak rule (`202606210600_topic.v1-v4-tiebreak.md`): if V4 (type-erased static-shape,
plugin-interop) falls noticeably behind V1 (blob + scalar snapshot, native monomorphised
dispatch), choose V1; if near-identical or only minorly different, choose V4 (or the hybrid),
because the plugin/wasm dynamic-loadability benefit is then essentially free.

The v4/v1 median ratio across all nine scalar arms, both runs: run-1 spanned 0.946x to 1.049x;
run-2 tightened to 0.991x to 1.017x. Every arm is within +-1.7%, and the aggregate is
indistinguishable from 1.0x. This lands squarely in the **"near-identical" branch**: V4 carries
no measurable scalar-latency penalty against V1 on this host, and it ties the blob on axes B, C,
and E as well (it is a contiguous erased byte store with a per-resource line). So V4's only real
cost is the backcast machinery and the type-erasure round-trip soundness burden; against that, it
buys plugin-crossing capability the monomorphised blob structurally cannot provide.

The bench therefore reduces the V1-vs-V4 question to a pure requirements question, with the
latency dimension removed: **is a plugin-crossing resource (a resource defined or accessed across
a cdylib/wasm boundary) an actual requirement for hilavitkutin's consumers?** If yes, the hybrid
(the v4-plugin-interop topic's shape) adopts the erased static-shape boundary at zero latency
penalty. If no, the pure V1 blob is the simpler default.

**Resolved (op, 2026-07-02): go with the hybrid, global-capable.** Op's reasoning: it is more
future-proof and performs about the same, so it carries only upsides; the erasure complexity is a
complexity-versus-simplicity concern, which is an anti-axis in this workspace and not counted
against. The hybrid is the erased static-shape addressing (V4, enabling plugin/wasm resource
crossing) layered with V1's access discipline (scalar snapshot, live-streamed collections) on the
one-record blob. The per-resource-vs-global sub-question resolves to global-capable/uniform: every
resource uses the erased addressing (any resource plugin-capable without a design change,
maximally future-proof), at the parity cost the bench measured. See
`mock/design_rounds/202606210600_topic.hybrid-decision.md`.

### Locked by the bench (both runs, independent of the V4 requirements call)

- **Resource is a handle, not an inline-value store** (R5:1689). Unchanged from the synthesis.
- **The per-record value layout is a one-record blob** (per-resource contiguous value on the
  arena), NOT decomposed per-member columns (V2) and NOT shape-bound shared columns (V3). V2
  loses axis B (scatter, up to 3.1x) and axis C (column-count cap); V3 loses axis E (3.4x
  false-sharing) plus the resize-invalidates-all-sharers hazard. Neither buys a hot-loop win to
  justify the cost. This reverses the addendum's original "decompose to shape-bound columns."
- **Scalar Field members: stack-snapshot before the morsel loop.** Wall-clock-neutral on M1 (free
  insurance), mechanism proven in the V0-reloads / V1-hoists disasm, and real on any uarch where
  the reload is not L1-cheap.
- **Seq/Map collection members: stream live from the column, never snapshot-copy.** ~2.5x at
  64 MiB, confirmed in both runs. The snapshot copies only the small scalars and the collection
  ptr+len, not the elements.
- **The noalias invariant is architectural:** the handle store never aliases the value columns
  (separate provenance), and scalar members are snapshotted before the loop so LLVM keeps them in
  registers across it.

The one layout question the bench left open, V1 pure-blob vs V1+V4 hybrid, is now decided: op
picked the hybrid (global-capable). The storage-model layout is fully settled.

### Drift-fix scope (bench-backed, hybrid addressing)

The `DrainStores` one-record blob (`resource/bindings.rs`) is drift only in lacking the snapshot,
not in being a blob (the bench confirms the blob is the right per-record layout). The fix is
additive, not a decomposition rewrite: (1) add the scalar stack-snapshot before the morsel loop,
(2) wire live-streamed access for Seq/Map members, (3) state the noalias invariant, and (4) per
op's hybrid decision, route value access through the erased static-shape descriptor rather than a
monomorphised concrete-type pointer (parity in-process, buys plugin/wasm crossing). The
`CollectionBytes`/`ResourceFootprint` substrate (#163/#164) supplies the collection-size term for
the A3b L1 morsel formula (blob-stride + collection-bytes fold, unaffected by the addressing
choice), which unblocks once this lands.

## Mockspace follow-up (for op)

`mockspace-bench-core`'s `ByteRoutine::build_input` allocates the input as a stack array
(`let mut buf = [0u8; IN]`), which overflows the stack for large IN (the 16/64 MiB seqd sizes
crashed before this round switched seqd to a seed-driven custom `Routine`). Large-payload byte
benches should heap-allocate the input. One-line follow-up: make `ByteRoutine::build_input` build
into a heap `Vec` rather than a stack array, so large-N byte benches do not each need a custom
`Routine`.
