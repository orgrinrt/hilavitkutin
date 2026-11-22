# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (memcpy_intrinsic) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline memcpy_intrinsic has the worst median (20.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest memcpy_unrolled at 20.41 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 20.41 us to slowest 20.41 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (202 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: memcpy_unrolled** at 20410.8 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 20410.8 ns, slowest 20412.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 22622ns | 22524ns | 22503ns | 22537ns | 22997ns | base |
| memcpy_naive | 22635ns | 22528ns | 22502ns | 22541ns | 23048ns | +0.05% |
| memcpy_unrolled | 22612ns | 22526ns | 22502ns | 22539ns | 22938ns | -0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 20507ns | 20402ns | 20846ns | base | 0.799 |
| memcpy_naive | 20519ns | 20401ns | 20897ns | +0.06% | 0.798 |
| memcpy_unrolled | 20497ns | 20401ns | 20793ns | -0.05% | 0.799 |

## Performance model

- Peak throughput: **0.803 Gops/s** (memcpy_naive; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.803 | 99.9% |
| memcpy_naive | 0.803 | 99.9% |
| memcpy_unrolled | 0.803 | 100.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 22622ns | 22622ns | base |
| memcpy_naive | 22635ns | 22635ns | +0.05% |
| memcpy_unrolled | 22612ns | 22612ns | -0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 20413ns | base | --- | [20408, 20433] | --- | --- | --- | --- |
| memcpy_naive | 20413ns | no significant difference | [-2, +0]ns | [20408, 20440] | no | 0.1848 | 0.0924 | 4 |
| memcpy_unrolled | 20411ns | no significant difference | [-1, +3]ns | [20408, 20435] | no | 0.7512 | 0.7512 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 20407ns | +0.0% | -0.0% |
| 1 | 2 | 20418ns | -0.0% | -0.0% |
| 1 | 3 | 20409ns | -0.0% | +0.1% |
| 1 | 4 | 20417ns | -0.0% | -0.1% |
| 1 | 5 | 20422ns | +0.0% | +0.0% |
| 1 | 6 | 20406ns | -0.0% | +0.2% |
| 1 | 7 | 20407ns | +0.1% | +0.0% |
| 1 | 8 | 21119ns | -0.0% | -3.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.028 | ok |
| memcpy_naive | 0.013 | ok |
| memcpy_unrolled | -0.446 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 0/8, lost 1/8
- **memcpy_unrolled**: won 1/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.8ns | 20507.3ns | 0.0% |  |
| memcpy_naive | 3.9ns | 20519.0ns | 0.0% |  |
| memcpy_unrolled | 3.7ns | 20497.0ns | 0.0% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 20402.0-20845.8 ns)
  20402.0 |########################################
  20424.2 |#####
  20446.4 |######
  20468.5 |######
  20490.7 |#####
  20512.9 |
  20535.1 |#
  20557.3 |
  20579.5 |#
  20601.7 |#
  20623.9 |
  20646.1 |
  20668.3 |
  20690.5 |
  20712.7 |
  20734.9 |
  20757.0 |
  20779.2 |
  20801.4 |
  20823.6 |
  (12 below, 16 above range)

memcpy_naive (n=160, range 20401.1-20896.8 ns)
  20401.1 |########################################
  20425.8 |#########
  20450.6 |######
  20475.4 |#######
  20500.2 |##
  20525.0 |#
  20549.8 |
  20574.6 |
  20599.3 |
  20624.1 |#
  20648.9 |
  20673.7 |
  20698.5 |
  20723.3 |
  20748.1 |
  20772.9 |
  20797.6 |
  20822.4 |
  20847.2 |
  20872.0 |
  (14 below, 19 above range)

memcpy_unrolled (n=160, range 20401.4-20793.0 ns)
  20401.4 |########################################
  20421.0 |####
  20440.6 |##########
  20460.2 |##
  20479.8 |#######
  20499.3 |##
  20518.9 |
  20538.5 |
  20558.1 |
  20577.7 |##
  20597.2 |##
  20616.8 |
  20636.4 |
  20656.0 |
  20675.6 |
  20695.1 |
  20714.7 |
  20734.3 |
  20753.9 |
  20773.5 |
  (14 below, 11 above range)

```
