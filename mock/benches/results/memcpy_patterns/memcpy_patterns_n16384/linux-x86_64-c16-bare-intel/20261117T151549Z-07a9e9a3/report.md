# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 18.27 us to slowest 18.27 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### memcpy_naive is fastest but the noisiest (CV 8.8%)

memcpy_naive wins on median (18.27 us) yet has the highest variance (CV 8.8%), while memcpy_intrinsic is the steadiest (CV 7.9%, 18.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (1.61 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader memcpy_naive vs stability leader memcpy_intrinsic (+0% speed for 1.1x steadier)

memcpy_naive is fastest (18.27 us, CV 8.8%); memcpy_intrinsic gives up 0.0% median for 1.1x lower variance (CV 7.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: memcpy_naive** at 18268.5 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 18268.5 ns, slowest 18268.9 ns)

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
| Counter frequency | 2112829592 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 19257ns | 20155ns | 16487ns | 19854ns | 20238ns | base |
| memcpy_naive | 18800ns | 20153ns | 16484ns | 19099ns | 20221ns | -2.37% |
| memcpy_unrolled | 19260ns | 20156ns | 16488ns | 19854ns | 20248ns | +0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 17443ns | 14942ns | 18299ns | base | 0.939 |
| memcpy_naive | 17037ns | 14939ns | 18319ns | -2.33% | 0.962 |
| memcpy_unrolled | 17450ns | 14942ns | 18335ns | +0.04% | 0.939 |

## Performance model

- Peak throughput: **1.097 Gops/s** (memcpy_naive; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.897 | 81.8% |
| memcpy_naive | 0.897 | 81.8% |
| memcpy_unrolled | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 19257ns | 19257ns | base |
| memcpy_naive | 18800ns | 18800ns | -2.37% |
| memcpy_unrolled | 19260ns | 19260ns | +0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 18269ns | base | --- | [18268, 18269] | --- | --- | --- | --- |
| memcpy_naive | 18268ns | no significant difference | [-1, +0]ns | [18258, 18269] | no | 0.8557 | 0.4278 | 1 |
| memcpy_unrolled | 18269ns | no significant difference | [-1, +1]ns | [18268, 18269] | no | 0.8740 | 0.8740 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 14939ns | +22.3% | +0.1% |
| 1 | 2 | 18270ns | +0.0% | -0.0% |
| 1 | 3 | 18269ns | -18.2% | +0.0% |
| 1 | 4 | 18269ns | +0.0% | +0.0% |
| 1 | 5 | 18269ns | +0.0% | -0.0% |
| 1 | 6 | 18270ns | -18.2% | -18.2% |
| 1 | 7 | 18269ns | +0.0% | +0.0% |
| 1 | 8 | 14947ns | +0.0% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.042 | ok |
| memcpy_naive | -0.475 | moderate- |
| memcpy_unrolled | -0.208 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 2/8, lost 1/8
- **memcpy_unrolled**: won 1/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 23.7ns | 17443.2ns | 0.1% |  |
| memcpy_naive | 23.0ns | 17036.6ns | 0.1% |  |
| memcpy_unrolled | 23.6ns | 17450.3ns | 0.1% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 14941.6-18298.9 ns)
  14941.6 |######
  15109.5 |
  15277.4 |
  15445.2 |
  15613.1 |
  15780.9 |
  15948.8 |
  16116.7 |
  16284.5 |
  16452.4 |
  16620.3 |
  16788.1 |
  16956.0 |
  17123.8 |
  17291.7 |
  17459.6 |
  17627.4 |
  17795.3 |
  17963.1 |
  18131.0 |########################################
  (19 below, 4 above range)

memcpy_naive (n=160, range 14939.4-18319.4 ns)
  14939.4 |##############
  15108.4 |
  15277.4 |
  15446.4 |
  15615.4 |
  15784.4 |
  15953.4 |
  16122.4 |
  16291.4 |
  16460.4 |
  16629.4 |
  16798.4 |
  16967.4 |
  17136.4 |
  17305.4 |
  17474.4 |
  17643.4 |
  17812.4 |
  17981.4 |
  18150.4 |########################################
  (24 below, 3 above range)

memcpy_unrolled (n=160, range 14941.6-18335.0 ns)
  14941.6 |#######
  15111.3 |
  15280.9 |
  15450.6 |
  15620.3 |
  15790.0 |
  15959.6 |
  16129.3 |
  16299.0 |
  16468.6 |
  16638.3 |
  16808.0 |
  16977.7 |
  17147.3 |
  17317.0 |
  17486.7 |
  17656.3 |
  17826.0 |
  17995.7 |
  18165.4 |########################################
  (18 below, 6 above range)

```
