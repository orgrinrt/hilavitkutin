# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (memcpy_intrinsic) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline memcpy_intrinsic has the worst median (1.23 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest memcpy_naive at 1.23 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (495 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### memcpy_unrolled is inconsistent: worst-20% is 1.8x its best-20%

memcpy_unrolled's best 20% of batches run at 1.01 us but its worst 20% at 1.83 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 1.23 us and 1.23 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (-0 ns, 0.04%)

memcpy_naive differs from baseline memcpy_intrinsic by -0 ns (0.04%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_naive** at 1232.3 ns median (-0.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.00x (fastest 1232.3 ns, slowest 1233.2 ns)

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
| memcpy_intrinsic | 3439ns | 3115ns | 2795ns | 3117ns | 5049ns | base |
| memcpy_naive | 3374ns | 3112ns | 2808ns | 3113ns | 4726ns | -1.89% |
| memcpy_unrolled | 3237ns | 3113ns | 2548ns | 2992ns | 4662ns | -5.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 1358ns | 1108ns | 1983ns | base | 0.754 |
| memcpy_naive | 1333ns | 1114ns | 1854ns | -1.84% | 0.768 |
| memcpy_unrolled | 1279ns | 1008ns | 1832ns | -5.86% | 0.801 |

## Performance model

- Peak throughput: **1.016 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.830 | 81.8% |
| memcpy_naive | 0.831 | 81.8% |
| memcpy_unrolled | 0.831 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 3439ns | 3439ns | base |
| memcpy_naive | 3374ns | 3374ns | -1.89% |
| memcpy_unrolled | 3237ns | 3237ns | -5.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 1233ns | base | --- | [1233, 1234] | --- | --- | --- | --- |
| memcpy_naive | 1232ns | -0.5ns (-0.0%) | [-1, -0]ns | [1232, 1232] | YES | 0.0089 | 0.0089 | 0 |
| memcpy_unrolled | 1233ns | -0.8ns (-0.1%) | [-1, -0]ns | [1233, 1233] | YES | 0.0006 | 0.0003 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 1233ns | -0.1% | -0.0% |
| 1 | 2 | 1233ns | -0.0% | -18.3% |
| 1 | 3 | 1233ns | -0.0% | +0.1% |
| 1 | 4 | 1233ns | +0.0% | +0.1% |
| 1 | 5 | 1234ns | -18.2% | -0.0% |
| 1 | 6 | 1234ns | -0.1% | -0.1% |
| 1 | 7 | 1233ns | -0.1% | -18.1% |
| 1 | 8 | 1010ns | +21.9% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.015 | ok |
| memcpy_naive | -0.163 | ok |
| memcpy_unrolled | 0.058 | ok |

**Consistency summary:**

- **memcpy_naive**: won 2/8, lost 1/8
- **memcpy_unrolled**: won 2/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 27.2ns | 1358.2ns | 2.0% |  |
| memcpy_naive | 26.9ns | 1333.1ns | 2.0% |  |
| memcpy_unrolled | 25.7ns | 1278.6ns | 2.0% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 1107.5-1982.7 ns)
   1107.5 |
   1151.3 |
   1195.0 |########################################
   1238.8 |#######
   1282.6 |
   1326.3 |
   1370.1 |
   1413.8 |
   1457.6 |
   1501.3 |
   1545.1 |
   1588.8 |
   1632.6 |
   1676.4 |
   1720.1 |
   1763.9 |
   1807.6 |
   1851.4 |
   1895.1 |
   1938.9 |
  (18 below, 11 above range)

memcpy_naive (n=160, range 1113.9-1854.1 ns)
   1113.9 |
   1150.9 |
   1188.0 |
   1225.0 |########################################
   1262.0 |
   1299.0 |
   1336.0 |
   1373.0 |
   1410.0 |
   1447.0 |
   1484.0 |
   1521.0 |
   1558.1 |
   1595.1 |
   1632.1 |
   1669.1 |
   1706.1 |
   1743.1 |
   1780.1 |
   1817.1 |
  (17 below, 10 above range)

memcpy_unrolled (n=160, range 1008.3-1832.1 ns)
   1008.3 |################
   1049.5 |
   1090.6 |
   1131.8 |
   1173.0 |
   1214.2 |########################################
   1255.4 |
   1296.6 |
   1337.8 |
   1379.0 |
   1420.2 |
   1461.4 |
   1502.6 |
   1543.8 |
   1585.0 |
   1626.2 |
   1667.4 |
   1708.6 |
   1749.7 |
   1790.9 |
  (15 below, 10 above range)

```

## Diagnostics

- **memcpy_intrinsic**: CV=40.2% (high variance, measurements may be unstable)
- **memcpy_naive**: CV=37.1% (high variance, measurements may be unstable)
- **memcpy_unrolled**: CV=39.1% (high variance, measurements may be unstable)
