# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (memcpy_intrinsic) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline memcpy_intrinsic has the worst median (1.23 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest memcpy_unrolled at 1.23 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (556 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader memcpy_unrolled vs stability leader memcpy_naive (+0% speed for 1.1x steadier)

memcpy_unrolled is fastest (1.23 us, CV 45.2%); memcpy_naive gives up 0.1% median for 1.1x lower variance (CV 41.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### memcpy_intrinsic is inconsistent: worst-20% is 2.3x its best-20%

memcpy_intrinsic's best 20% of batches run at 1.01 us but its worst 20% at 2.36 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.1% of the fastest

All 3 variants sit between 1.23 us and 1.23 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_unrolled's edge over baseline is significant but tiny (-2 ns, 0.19%)

memcpy_unrolled differs from baseline memcpy_intrinsic by -2 ns (0.19%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_unrolled** at 1231.7 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 1231.7 ns, slowest 1233.5 ns)

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
| Counter frequency | 2112632730 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 3563ns | 3114ns | 2548ns | 3093ns | 5991ns | base |
| memcpy_naive | 3379ns | 3113ns | 2792ns | 3114ns | 4758ns | -5.19% |
| memcpy_unrolled | 3337ns | 3110ns | 2546ns | 3000ns | 5138ns | -6.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 1410ns | 1010ns | 2362ns | base | 0.726 |
| memcpy_naive | 1335ns | 1107ns | 1870ns | -5.27% | 0.767 |
| memcpy_unrolled | 1318ns | 1009ns | 2020ns | -6.50% | 0.777 |

## Performance model

- Peak throughput: **1.015 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.830 | 81.8% |
| memcpy_naive | 0.830 | 81.8% |
| memcpy_unrolled | 0.831 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 3563ns | 3563ns | base |
| memcpy_naive | 3379ns | 3379ns | -5.19% |
| memcpy_unrolled | 3337ns | 3337ns | -6.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 1233ns | base | --- | [1233, 1234] | --- | --- | --- | --- |
| memcpy_naive | 1233ns | no significant difference | [-0, +1]ns | [1233, 1233] | no | 1.0000 | 1.0000 | 0 |
| memcpy_unrolled | 1232ns | -2.3ns (-0.2%) | [-215, -1]ns | [1231, 1232] | YES | 0.0015 | 0.0007 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 1234ns | +0.0% | -18.2% |
| 1 | 2 | 1010ns | +22.1% | +22.2% |
| 1 | 3 | 1233ns | +0.0% | -18.2% |
| 1 | 4 | 1233ns | +0.0% | +0.0% |
| 1 | 5 | 1234ns | -0.0% | -18.2% |
| 1 | 6 | 1234ns | -18.3% | -0.2% |
| 1 | 7 | 2121ns | -41.8% | -41.9% |
| 1 | 8 | 1009ns | +22.3% | +22.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.276 | moderate- |
| memcpy_naive | -0.163 | ok |
| memcpy_unrolled | -0.481 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 2/8, lost 2/8
- **memcpy_unrolled**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 28.3ns | 1409.7ns | 2.0% |  |
| memcpy_naive | 26.8ns | 1335.3ns | 2.0% |  |
| memcpy_unrolled | 26.3ns | 1318.1ns | 2.0% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 1009.7-2362.0 ns)
   1009.7 |####
   1077.3 |
   1144.9 |
   1212.6 |########################################
   1280.2 |
   1347.8 |
   1415.4 |
   1483.0 |
   1550.6 |
   1618.3 |
   1685.9 |
   1753.5 |
   1821.1 |#
   1888.7 |
   1956.3 |
   2024.0 |
   2091.6 |
   2159.2 |
   2226.8 |
   2294.4 |
  (26 below, 16 above range)

memcpy_naive (n=160, range 1106.5-1870.4 ns)
   1106.5 |
   1144.7 |
   1182.9 |
   1221.1 |########################################
   1259.3 |
   1297.5 |
   1335.7 |
   1373.9 |
   1412.1 |
   1450.3 |
   1488.5 |
   1526.7 |
   1564.9 |
   1603.1 |
   1641.3 |
   1679.4 |
   1717.6 |
   1755.8 |
   1794.0 |
   1832.2 |
  (18 below, 11 above range)

memcpy_unrolled (n=160, range 1008.5-2019.6 ns)
   1008.5 |###############
   1059.1 |
   1109.6 |
   1160.2 |
   1210.7 |########################################
   1261.3 |
   1311.8 |
   1362.4 |
   1412.9 |
   1463.5 |
   1514.0 |
   1564.6 |
   1615.2 |
   1665.7 |
   1716.3 |
   1766.8 |
   1817.4 |
   1867.9 |
   1918.5 |
   1969.0 |
  (14 below, 14 above range)

```

## Diagnostics

- **memcpy_intrinsic**: CV=43.7% (high variance, measurements may be unstable)
- **memcpy_naive**: CV=37.9% (high variance, measurements may be unstable)
- **memcpy_unrolled**: CV=42.2% (high variance, measurements may be unstable)
