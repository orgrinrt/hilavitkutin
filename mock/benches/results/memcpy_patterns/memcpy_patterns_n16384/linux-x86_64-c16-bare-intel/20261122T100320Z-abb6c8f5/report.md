# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 18.27 us to slowest 18.27 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### memcpy_intrinsic is fastest but the noisiest (CV 8.8%)

memcpy_intrinsic wins on median (18.27 us) yet has the highest variance (CV 8.8%), while memcpy_unrolled is the steadiest (CV 7.9%, 18.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (1.61 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (18.27 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader memcpy_intrinsic vs stability leader memcpy_unrolled (+0% speed for 1.1x steadier)

memcpy_intrinsic is fastest (18.27 us, CV 8.8%); memcpy_unrolled gives up 0.0% median for 1.1x lower variance (CV 7.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 18265.6 ns median
- Spread: 1.00x (fastest 18265.6 ns, slowest 18268.2 ns)

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
| Counter frequency | 2112541357 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 18799ns | 20151ns | 16481ns | 19092ns | 20239ns | base |
| memcpy_naive | 18792ns | 20151ns | 16483ns | 19095ns | 20192ns | -0.04% |
| memcpy_unrolled | 19259ns | 20153ns | 16486ns | 19850ns | 20257ns | +2.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 17029ns | 14936ns | 18306ns | base | 0.962 |
| memcpy_naive | 17029ns | 14939ns | 18288ns | +0.00% | 0.962 |
| memcpy_unrolled | 17451ns | 14940ns | 18348ns | +2.48% | 0.939 |

## Performance model

- Peak throughput: **1.097 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.897 | 81.8% |
| memcpy_naive | 0.897 | 81.8% |
| memcpy_unrolled | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 18799ns | 18799ns | base |
| memcpy_naive | 18792ns | 18792ns | -0.04% |
| memcpy_unrolled | 19259ns | 19259ns | +2.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 18266ns | base | --- | [18258, 18268] | --- | --- | --- | --- |
| memcpy_naive | 18268ns | no significant difference | [-1, +3]ns | [18257, 18268] | no | 0.5777 | 0.3041 | 0 |
| memcpy_unrolled | 18268ns | no significant difference | [-1, +1]ns | [18267, 18269] | no | 0.5777 | 0.5777 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 14943ns | +0.0% | +22.3% |
| 1 | 2 | 14943ns | +22.3% | -0.0% |
| 1 | 3 | 18269ns | +0.0% | +0.0% |
| 1 | 4 | 18269ns | -0.0% | +0.0% |
| 1 | 5 | 18269ns | -18.2% | +0.0% |
| 1 | 6 | 18270ns | -18.2% | -0.0% |
| 1 | 7 | 18270ns | -0.0% | -18.2% |
| 1 | 8 | 14939ns | +22.3% | +22.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | 0.258 | moderate+ |
| memcpy_naive | 0.059 | ok |
| memcpy_unrolled | -0.375 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 2/8, lost 2/8
- **memcpy_unrolled**: won 1/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 23.1ns | 17029.1ns | 0.1% |  |
| memcpy_naive | 22.9ns | 17029.5ns | 0.1% |  |
| memcpy_unrolled | 23.5ns | 17451.2ns | 0.1% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 14935.5-18306.1 ns)
  14935.5 |####################
  15104.1 |
  15272.6 |
  15441.1 |
  15609.6 |
  15778.2 |
  15946.7 |
  16115.2 |
  16283.7 |
  16452.3 |
  16620.8 |
  16789.3 |
  16957.9 |
  17126.4 |
  17294.9 |
  17463.4 |
  17632.0 |
  17800.5 |
  17969.0 |
  18137.6 |########################################
  (9 below, 4 above range)

memcpy_naive (n=160, range 14938.6-18288.2 ns)
  14938.6 |#################
  15106.1 |
  15273.6 |
  15441.0 |
  15608.5 |
  15776.0 |
  15943.5 |
  16111.0 |
  16278.5 |
  16445.9 |
  16613.4 |
  16780.9 |
  16948.4 |
  17115.9 |
  17283.3 |
  17450.8 |
  17618.3 |
  17785.8 |
  17953.3 |
  18120.8 |########################################
  (17 below, 4 above range)

memcpy_unrolled (n=160, range 14939.6-18348.2 ns)
  14939.6 |######
  15110.0 |
  15280.5 |
  15450.9 |
  15621.3 |
  15791.8 |
  15962.2 |
  16132.6 |
  16303.1 |
  16473.5 |
  16643.9 |
  16814.3 |
  16984.8 |
  17155.2 |
  17325.6 |
  17496.1 |
  17666.5 |
  17836.9 |
  18007.4 |
  18177.8 |########################################
  (21 below, 6 above range)

```
