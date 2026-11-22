# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### v4_erased dominates: 20% faster than the next best (v1_snapshot)

v4_erased (97 ns) leads v1_snapshot (117 ns) by 20%, a clear separation rather than a photo finish. CV 62.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### v4_erased is fastest but the noisiest (CV 62.9%)

v4_erased wins on median (97 ns) yet has the highest variance (CV 62.9%), while v5_handletable is the steadiest (CV 47.1%, 118 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (21 ns) is smaller than the fastest variant's own run-to-run std-dev (61 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### v3_shapebound is inconsistent: worst-20% is 2.6x its best-20%

v3_shapebound's best 20% of batches run at 96 ns but its worst 20% at 247 ns (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### v2_decomposed's edge over baseline is significant but tiny (-1 ns, 0.47%)

v2_decomposed differs from baseline v0_blob by -1 ns (0.47%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v4_erased** at 97.0 ns median (-17.5% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.22x (fastest 97.0 ns, slowest 118.1 ns)

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
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 299ns | 262ns | 214ns | 243ns | 553ns | base |
| v1_snapshot | 304ns | 263ns | 215ns | 259ns | 527ns | +1.43% |
| v2_decomposed | 299ns | 263ns | 214ns | 248ns | 535ns | -0.19% |
| v3_shapebound | 312ns | 263ns | 215ns | 252ns | 587ns | +4.09% |
| v4_erased | 290ns | 219ns | 215ns | 233ns | 538ns | -2.99% |
| v5_handletable | 300ns | 267ns | 218ns | 253ns | 522ns | +0.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 131ns | 96ns | 236ns | base | 7.790 |
| v1_snapshot | 132ns | 96ns | 219ns | +0.06% | 7.785 |
| v2_decomposed | 130ns | 95ns | 223ns | -1.05% | 7.873 |
| v3_shapebound | 136ns | 96ns | 247ns | +3.39% | 7.534 |
| v4_erased | 127ns | 96ns | 225ns | -3.56% | 8.077 |
| v5_handletable | 130ns | 96ns | 217ns | -1.38% | 7.898 |

## Performance model

- Peak throughput: **10.726 Gops/s** (v2_decomposed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 8.711 | 81.2% |
| v1_snapshot | 8.775 | 81.8% |
| v2_decomposed | 8.752 | 81.6% |
| v3_shapebound | 8.767 | 81.7% |
| v4_erased | 10.557 | 98.4% |
| v5_handletable | 8.671 | 80.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 299ns | 299ns | base |
| v1_snapshot | 304ns | 304ns | +1.43% |
| v2_decomposed | 299ns | 299ns | -0.19% |
| v3_shapebound | 312ns | 312ns | +4.09% |
| v4_erased | 290ns | 290ns | -2.99% |
| v5_handletable | 300ns | 300ns | +0.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 118ns | base | --- | [96, 118] | --- | --- | --- | --- |
| v1_snapshot | 117ns | no significant difference | [-1, +0]ns | [117, 117] | no | 0.1232 | 0.0985 | 1 |
| v2_decomposed | 117ns | -0.5ns (-0.5%) | [-1, -0]ns | [116, 117] | YES | 0.0004 | 0.0001 | 0 |
| v3_shapebound | 117ns | -0.5ns (-0.4%) | [-1, -0]ns | [116, 117] | YES | 0.0456 | 0.0274 | 1 |
| v4_erased | 97ns | no significant difference | [-1, +0]ns | [97, 102] | no | 1.0000 | 1.0000 | 3 |
| v5_handletable | 118ns | +0.3ns (+0.3%) | [+0, +1]ns | [118, 118] | YES | 0.0088 | 0.0035 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 96ns | +28.6% | +21.9% | -0.5% | +0.0% | +22.9% |
| 1 | 2 | 96ns | -0.6% | -0.7% | +146.9% | +0.4% | +0.1% |
| 1 | 3 | 96ns | -0.5% | -0.5% | +21.8% | +0.5% | +0.4% |
| 1 | 4 | 96ns | +21.4% | -0.2% | +21.7% | +0.5% | +22.7% |
| 1 | 5 | 96ns | -0.6% | -0.9% | -0.6% | +1.0% | +0.0% |
| 2 | 1 | 118ns | -0.8% | -0.4% | -0.6% | +0.2% | +0.6% |
| 2 | 2 | 118ns | -19.0% | -18.6% | -18.8% | -17.9% | +0.1% |
| 2 | 3 | 118ns | -19.0% | -18.6% | -18.5% | -17.9% | -18.1% |
| 2 | 4 | 96ns | +20.8% | -0.6% | -0.4% | +0.4% | +22.7% |
| 2 | 5 | 118ns | -1.1% | -0.6% | -0.8% | -17.9% | -18.2% |
| 3 | 1 | 96ns | +21.6% | +21.8% | +21.7% | +5.6% | +0.2% |
| 3 | 2 | 96ns | +20.9% | +21.8% | +20.7% | +22.6% | +22.9% |
| 3 | 3 | 118ns | -0.2% | -0.1% | -0.7% | +0.0% | +0.8% |
| 3 | 4 | 179ns | -34.7% | -34.4% | -46.5% | -46.1% | -45.8% |
| 3 | 5 | 96ns | +21.0% | +21.5% | +22.0% | +22.9% | +22.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.018 | ok |
| v1_snapshot | 0.012 | ok |
| v2_decomposed | 0.408 | moderate+ |
| v3_shapebound | -0.096 | ok |
| v4_erased | 0.085 | ok |
| v5_handletable | -0.394 | moderate- |

**Consistency summary:**

- **v1_snapshot**: won 9/15, lost 6/15
- **v2_decomposed**: won 11/15, lost 4/15
- **v3_shapebound**: won 9/15, lost 6/15
- **v4_erased**: won 4/15, lost 9/15
- **v5_handletable**: won 3/15, lost 11/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 2723.6ns | 131.5ns | 2071.8% | HIGH |
| v1_snapshot | 2794.5ns | 131.5ns | 2124.5% | HIGH |
| v2_decomposed | 2735.7ns | 130.1ns | 2103.3% | HIGH |
| v3_shapebound | 2856.0ns | 135.9ns | 2101.2% | HIGH |
| v4_erased | 2655.0ns | 126.8ns | 2094.1% | HIGH |
| v5_handletable | 2738.5ns | 129.6ns | 2112.3% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 96.3-235.6 ns)
     96.3 |########################################
    103.3 |
    110.2 |#
    117.2 |################################
    124.2 |
    131.1 |
    138.1 |
    145.1 |
    152.0 |
    159.0 |
    166.0 |
    172.9 |######
    179.9 |
    186.9 |
    193.8 |#
    200.8 |
    207.8 |
    214.7 |
    221.7 |
    228.7 |
  (13 below, 11 above range)

v1_snapshot (n=120, range 95.5-218.8 ns)
     95.5 |###########
    101.7 |
    107.9 |
    114.0 |########################################
    120.2 |###
    126.4 |
    132.5 |
    138.7 |#
    144.9 |
    151.0 |
    157.2 |
    163.4 |
    169.5 |
    175.7 |
    181.8 |
    188.0 |
    194.2 |
    200.3 |
    206.5 |
    212.7 |
  (14 below, 12 above range)

v2_decomposed (n=120, range 95.5-223.4 ns)
     95.5 |#############################
    101.9 |
    108.3 |
    114.7 |########################################
    121.1 |#
    127.5 |
    133.9 |
    140.2 |
    146.6 |
    153.0 |
    159.4 |
    165.8 |
    172.2 |
    178.6 |
    185.0 |
    191.4 |
    197.8 |
    204.2 |
    210.6 |
    217.0 |
  (7 below, 13 above range)

v3_shapebound (n=120, range 95.6-247.3 ns)
     95.6 |######################
    103.2 |
    110.8 |########################################
    118.3 |#
    125.9 |
    133.5 |
    141.1 |
    148.7 |
    156.3 |#
    163.9 |
    171.5 |
    179.0 |
    186.6 |
    194.2 |
    201.8 |
    209.4 |
    217.0 |
    224.6 |
    232.2 |
    239.7 |
  (10 below, 13 above range)

v4_erased (n=120, range 96.1-225.4 ns)
     96.1 |########################################
    102.6 |
    109.0 |
    115.5 |####################
    121.9 |
    128.4 |
    134.9 |
    141.3 |#
    147.8 |
    154.3 |
    160.7 |
    167.2 |
    173.7 |
    180.1 |#
    186.6 |
    193.1 |
    199.5 |
    206.0 |#
    212.5 |
    218.9 |
  (16 below, 12 above range)

v5_handletable (n=120, range 96.3-216.7 ns)
     96.3 |#########################
    102.4 |
    108.4 |
    114.4 |########################################
    120.4 |
    126.4 |
    132.4 |
    138.5 |
    144.5 |
    150.5 |
    156.5 |
    162.5 |
    168.6 |
    174.6 |
    180.6 |
    186.6 |
    192.6 |
    198.7 |
    204.7 |
    210.7 |
  (11 below, 13 above range)

```

## Diagnostics

- **v0_blob**: CV=46.0% (high variance, measurements may be unstable)
- **v0_blob**: bridge=2094.0% of algo (FFI overhead may distort results)
- **v1_snapshot**: CV=43.6% (high variance, measurements may be unstable)
- **v1_snapshot**: bridge=2113.2% of algo (FFI overhead may distort results)
- **v2_decomposed**: CV=46.2% (high variance, measurements may be unstable)
- **v2_decomposed**: bridge=2105.9% of algo (FFI overhead may distort results)
- **v3_shapebound**: CV=48.9% (high variance, measurements may be unstable)
- **v3_shapebound**: bridge=2109.4% of algo (FFI overhead may distort results)
- **v4_erased**: CV=48.1% (high variance, measurements may be unstable)
- **v4_erased**: bridge=2083.9% of algo (FFI overhead may distort results)
- **v5_handletable**: CV=42.9% (high variance, measurements may be unstable)
- **v5_handletable**: bridge=2111.6% of algo (FFI overhead may distort results)
