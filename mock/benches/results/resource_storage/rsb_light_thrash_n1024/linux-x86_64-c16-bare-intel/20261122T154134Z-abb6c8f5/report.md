# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### v0_blob dominates: 16% faster than the next best (v3_shapebound)

v0_blob (100 ns) leads v3_shapebound (117 ns) by 16%, a clear separation rather than a photo finish. CV 135.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### v0_blob is fastest but the noisiest (CV 135.2%)

v0_blob wins on median (100 ns) yet has the highest variance (CV 135.2%), while v1_snapshot is the steadiest (CV 52.3%, 117 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### v0_blob shows alternating (throttle bounce) (autocorr -0.53)

v0_blob's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (18 ns) is smaller than the fastest variant's own run-to-run std-dev (136 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (v0_blob)

The baseline v0_blob is the fastest (100 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### v0_blob is inconsistent: worst-20% is 3.2x its best-20%

v0_blob's best 20% of batches run at 96 ns but its worst 20% at 305 ns (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### v3_shapebound's edge over baseline is significant but tiny (-0 ns, 0.30%)

v3_shapebound differs from baseline v0_blob by -0 ns (0.30%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (v0_blob) is the fastest** at 100.5 ns median
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.18x (fastest 100.5 ns, slowest 118.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under light surrounding work |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112842590 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 328ns | 221ns | 215ns | 237ns | 714ns | base |
| v1_snapshot | 298ns | 260ns | 213ns | 247ns | 536ns | -9.26% |
| v2_decomposed | 322ns | 264ns | 216ns | 259ns | 616ns | -1.90% |
| v3_shapebound | 301ns | 262ns | 214ns | 244ns | 556ns | -8.42% |
| v4_erased | 332ns | 265ns | 216ns | 260ns | 665ns | +1.13% |
| v5_handletable | 303ns | 266ns | 217ns | 252ns | 539ns | -7.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 144ns | 96ns | 305ns | base | 7.110 |
| v1_snapshot | 131ns | 96ns | 227ns | -8.85% | 7.801 |
| v2_decomposed | 140ns | 96ns | 260ns | -2.61% | 7.301 |
| v3_shapebound | 132ns | 96ns | 235ns | -8.55% | 7.775 |
| v4_erased | 145ns | 96ns | 283ns | +0.93% | 7.045 |
| v5_handletable | 132ns | 96ns | 227ns | -8.56% | 7.776 |

## Performance model

- Peak throughput: **10.705 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.189 | 95.2% |
| v1_snapshot | 8.733 | 81.6% |
| v2_decomposed | 8.719 | 81.4% |
| v3_shapebound | 8.767 | 81.9% |
| v4_erased | 8.667 | 81.0% |
| v5_handletable | 8.719 | 81.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 328ns | 328ns | base |
| v1_snapshot | 298ns | 298ns | -9.26% |
| v2_decomposed | 322ns | 322ns | -1.90% |
| v3_shapebound | 301ns | 301ns | -8.42% |
| v4_erased | 332ns | 332ns | +1.13% |
| v5_handletable | 303ns | 303ns | -7.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 100ns | base | --- | [96, 118] | --- | --- | --- | --- |
| v1_snapshot | 117ns | no significant difference | [-0, +0]ns | [117, 117] | no | 1.0000 | 1.0000 | 7 |
| v2_decomposed | 117ns | no significant difference | [-0, +4]ns | [117, 118] | no | 0.7281 | 0.5230 | 0 |
| v3_shapebound | 117ns | -0.3ns (-0.3%) | [-0, -0]ns | [97, 117] | YES | 0.0123 | 0.0049 | 5 |
| v4_erased | 118ns | +1.0ns (+1.0%) | [+1, +19]ns | [117, 118] | YES | 0.0000 | 0.0000 | 0 |
| v5_handletable | 117ns | no significant difference | [-0, +1]ns | [117, 118] | no | 0.7281 | 0.5825 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 98ns | +19.7% | +19.4% | -2.5% | -1.5% | -2.5% |
| 1 | 2 | 118ns | -0.3% | -18.5% | -18.3% | +0.8% | -17.9% |
| 1 | 3 | 96ns | +21.7% | +22.1% | -0.4% | +0.6% | -0.4% |
| 1 | 4 | 118ns | -18.5% | +90.1% | +0.3% | -17.6% | -17.7% |
| 1 | 5 | 96ns | -0.3% | -0.2% | -0.8% | +22.7% | +22.7% |
| 2 | 1 | 118ns | -18.3% | -18.5% | +0.0% | +116.9% | -18.0% |
| 2 | 2 | 96ns | -0.1% | +22.0% | +21.2% | +0.4% | +22.3% |
| 2 | 3 | 97ns | +21.8% | -0.3% | -0.8% | +22.1% | +22.0% |
| 2 | 4 | 96ns | +22.1% | -0.5% | +21.7% | +116.1% | -0.3% |
| 2 | 5 | 118ns | +0.0% | -0.4% | -0.3% | -17.8% | +0.4% |
| 3 | 1 | 96ns | +0.1% | -0.3% | -0.3% | +22.9% | +22.7% |
| 3 | 2 | 118ns | -0.4% | -0.2% | -0.5% | +0.4% | +0.4% |
| 3 | 3 | 96ns | -0.1% | +51.9% | -0.1% | +0.1% | +22.0% |
| 3 | 4 | 96ns | -0.2% | +21.9% | +22.3% | +22.9% | +0.0% |
| 3 | 5 | 96ns | +22.1% | -0.1% | -0.3% | +22.4% | +22.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.526 | HIGH- (thermal bounce) |
| v1_snapshot | 0.144 | ok |
| v2_decomposed | -0.086 | ok |
| v3_shapebound | -0.408 | moderate- |
| v4_erased | -0.182 | ok |
| v5_handletable | 0.000 | ok |

**Consistency summary:**

- **v1_snapshot**: won 7/15, lost 5/15
- **v2_decomposed**: won 9/15, lost 6/15
- **v3_shapebound**: won 9/15, lost 4/15
- **v4_erased**: won 3/15, lost 11/15
- **v5_handletable**: won 6/15, lost 8/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 3003.7ns | 144.0ns | 2085.7% | HIGH |
| v1_snapshot | 2752.5ns | 131.3ns | 2096.8% | HIGH |
| v2_decomposed | 2946.5ns | 140.3ns | 2100.8% | HIGH |
| v3_shapebound | 2766.1ns | 131.7ns | 2100.3% | HIGH |
| v4_erased | 3034.3ns | 145.4ns | 2087.5% | HIGH |
| v5_handletable | 2780.5ns | 131.7ns | 2111.4% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 96.2-305.3 ns)
     96.2 |########################################
    106.7 |#
    117.1 |##########################
    127.6 |
    138.1 |
    148.5 |
    159.0 |
    169.4 |
    179.9 |#
    190.3 |
    200.8 |#
    211.2 |
    221.7 |
    232.1 |
    242.6 |
    253.1 |
    263.5 |
    274.0 |
    284.4 |#
    294.9 |
  (13 below, 9 above range)

v1_snapshot (n=120, range 96.0-227.1 ns)
     96.0 |#############################
    102.5 |
    109.1 |
    115.7 |########################################
    122.2 |#
    128.8 |
    135.3 |
    141.9 |
    148.4 |
    155.0 |
    161.6 |
    168.1 |
    174.7 |
    181.2 |
    187.8 |
    194.3 |
    200.9 |
    207.5 |
    214.0 |
    220.6 |
  (6 below, 12 above range)

v2_decomposed (n=120, range 95.7-260.3 ns)
     95.7 |#################################
    103.9 |#
    112.2 |########################################
    120.4 |###
    128.6 |#
    136.8 |#
    145.1 |#
    153.3 |#
    161.5 |##
    169.7 |
    178.0 |##
    186.2 |
    194.4 |
    202.7 |##
    210.9 |##
    219.1 |#
    227.3 |
    235.6 |##
    243.8 |###
    252.0 |#
  (12 below, 12 above range)

v3_shapebound (n=120, range 95.7-235.0 ns)
     95.7 |###################################
    102.6 |
    109.6 |#
    116.6 |########################################
    123.5 |
    130.5 |
    137.5 |
    144.4 |
    151.4 |
    158.4 |
    165.3 |
    172.3 |
    179.3 |
    186.2 |
    193.2 |
    200.2 |
    207.1 |
    214.1 |
    221.1 |
    228.1 |
  (9 below, 13 above range)

v4_erased (n=120, range 96.5-283.3 ns)
     96.5 |########################
    105.8 |
    115.2 |########################################
    124.5 |
    133.8 |#
    143.2 |
    152.5 |#
    161.9 |
    171.2 |
    180.5 |#
    189.9 |
    199.2 |
    208.5 |
    217.9 |
    227.2 |#
    236.6 |
    245.9 |
    255.2 |
    264.6 |
    273.9 |####
  (9 below, 13 above range)

v5_handletable (n=120, range 95.9-227.4 ns)
     95.9 |#######################
    102.5 |
    109.1 |
    115.7 |########################################
    122.2 |
    128.8 |
    135.4 |
    142.0 |
    148.5 |
    155.1 |
    161.7 |
    168.2 |
    174.8 |
    181.4 |
    188.0 |
    194.5 |
    201.1 |
    207.7 |
    214.3 |
    220.8 |
  (12 below, 13 above range)

```

## Diagnostics

- **v0_blob**: CV=94.3% (high variance, measurements may be unstable)
- **v0_blob**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **v0_blob**: bridge=2010.9% of algo (FFI overhead may distort results)
- **v1_snapshot**: CV=46.7% (high variance, measurements may be unstable)
- **v1_snapshot**: bridge=2099.3% of algo (FFI overhead may distort results)
- **v2_decomposed**: CV=45.8% (high variance, measurements may be unstable)
- **v2_decomposed**: bridge=2098.0% of algo (FFI overhead may distort results)
- **v3_shapebound**: CV=49.2% (high variance, measurements may be unstable)
- **v3_shapebound**: bridge=2108.4% of algo (FFI overhead may distort results)
- **v4_erased**: CV=50.4% (high variance, measurements may be unstable)
- **v4_erased**: bridge=2088.2% of algo (FFI overhead may distort results)
- **v5_handletable**: CV=47.0% (high variance, measurements may be unstable)
- **v5_handletable**: bridge=2122.4% of algo (FFI overhead may distort results)
