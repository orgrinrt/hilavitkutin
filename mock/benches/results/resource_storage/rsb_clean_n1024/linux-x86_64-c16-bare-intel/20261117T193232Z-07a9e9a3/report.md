# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (69 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader v3_shapebound vs stability leader v5_handletable (+1% speed for 1.1x steadier)

v3_shapebound is fastest (116 ns, CV 59.5%); v5_handletable gives up 0.6% median for 1.1x lower variance (CV 55.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### v4_erased is inconsistent: worst-20% is 3.0x its best-20%

v4_erased's best 20% of batches run at 96 ns but its worst 20% at 289 ns (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 1.4% of the fastest

All 6 variants sit between 116 ns and 118 ns - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v3_shapebound's edge over baseline is significant but tiny (-1 ns, 0.68%)

v3_shapebound differs from baseline v0_blob by -1 ns (0.68%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 116.5 ns median (-1.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 116.5 ns, slowest 118.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under clean morsel loop (algo only) |
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
| v0_blob | 199ns | 166ns | 136ns | 155ns | 394ns | base |
| v1_snapshot | 220ns | 166ns | 142ns | 173ns | 441ns | +10.81% |
| v2_decomposed | 203ns | 166ns | 135ns | 160ns | 398ns | +2.01% |
| v3_shapebound | 196ns | 165ns | 135ns | 160ns | 366ns | -1.47% |
| v4_erased | 212ns | 167ns | 136ns | 164ns | 433ns | +6.58% |
| v5_handletable | 190ns | 166ns | 136ns | 152ns | 359ns | -4.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 138ns | 96ns | 264ns | base | 7.435 |
| v1_snapshot | 152ns | 100ns | 294ns | +10.49% | 6.729 |
| v2_decomposed | 140ns | 96ns | 264ns | +1.53% | 7.323 |
| v3_shapebound | 135ns | 95ns | 239ns | -2.27% | 7.608 |
| v4_erased | 146ns | 96ns | 289ns | +6.30% | 6.994 |
| v5_handletable | 131ns | 96ns | 235ns | -5.08% | 7.833 |

## Performance model

- Peak throughput: **10.767 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 8.707 | 80.9% |
| v1_snapshot | 8.711 | 80.9% |
| v2_decomposed | 8.737 | 81.2% |
| v3_shapebound | 8.793 | 81.7% |
| v4_erased | 8.671 | 80.5% |
| v5_handletable | 8.741 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 199ns | 199ns | base |
| v1_snapshot | 220ns | 220ns | +10.81% |
| v2_decomposed | 203ns | 203ns | +2.01% |
| v3_shapebound | 196ns | 196ns | -1.47% |
| v4_erased | 212ns | 212ns | +6.58% |
| v5_handletable | 190ns | 190ns | -4.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 118ns | base | --- | [97, 118] | --- | --- | --- | --- |
| v1_snapshot | 118ns | no significant difference | [-0, +8]ns | [117, 118] | no | 1.0000 | 1.0000 | 3 |
| v2_decomposed | 117ns | no significant difference | [-1, +2]ns | [117, 117] | no | 0.3941 | 0.3153 | 0 |
| v3_shapebound | 116ns | -0.8ns (-0.7%) | [-1, -0]ns | [116, 117] | YES | 0.0143 | 0.0057 | 1 |
| v4_erased | 118ns | +0.7ns (+0.6%) | [+0, +1]ns | [117, 118] | YES | 0.0086 | 0.0017 | 1 |
| v5_handletable | 117ns | no significant difference | [-0, +0]ns | [97, 118] | no | 0.3941 | 0.3071 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 96ns | +22.1% | +21.8% | +21.5% | +22.7% | +0.4% |
| 1 | 2 | 97ns | +147.7% | +100.7% | -0.2% | +0.9% | -0.4% |
| 1 | 3 | 118ns | +0.1% | -18.5% | -18.7% | -17.7% | -0.0% |
| 1 | 4 | 96ns | -0.3% | +21.8% | -0.6% | +0.4% | +0.3% |
| 1 | 5 | 96ns | +26.1% | -0.5% | -0.9% | +116.8% | -0.3% |
| 2 | 1 | 118ns | -1.0% | -18.3% | -0.7% | +0.6% | +0.0% |
| 2 | 2 | 119ns | -1.2% | -19.1% | -1.2% | -18.5% | -1.0% |
| 2 | 3 | 96ns | +0.0% | +0.1% | -1.1% | +23.1% | +1.1% |
| 2 | 4 | 118ns | +0.0% | -0.5% | -0.7% | +0.4% | -18.2% |
| 2 | 5 | 96ns | +21.1% | +21.5% | -0.8% | +0.6% | +22.6% |
| 3 | 1 | 118ns | -0.9% | -0.3% | -0.9% | -17.7% | -0.2% |
| 3 | 2 | 118ns | -0.1% | -0.3% | -0.9% | +0.6% | -18.0% |
| 3 | 3 | 96ns | +21.9% | +22.3% | +21.3% | +22.4% | +0.1% |
| 3 | 4 | 97ns | -0.8% | +21.0% | +20.9% | -0.5% | +0.1% |
| 3 | 5 | 105ns | +99.9% | -8.7% | +11.0% | +131.2% | +12.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.218 | moderate- |
| v1_snapshot | -0.120 | ok |
| v2_decomposed | -0.047 | ok |
| v3_shapebound | 0.102 | ok |
| v4_erased | -0.116 | ok |
| v5_handletable | -0.045 | ok |

**Consistency summary:**

- **v1_snapshot**: won 6/15, lost 6/15
- **v2_decomposed**: won 8/15, lost 6/15
- **v3_shapebound**: won 11/15, lost 4/15
- **v4_erased**: won 4/15, lost 11/15
- **v5_handletable**: won 6/15, lost 7/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 2877.3ns | 137.7ns | 2089.2% | HIGH |
| v1_snapshot | 3201.4ns | 152.2ns | 2103.7% | HIGH |
| v2_decomposed | 2939.5ns | 139.8ns | 2102.0% | HIGH |
| v3_shapebound | 2842.0ns | 134.6ns | 2111.5% | HIGH |
| v4_erased | 3058.6ns | 146.4ns | 2089.1% | HIGH |
| v5_handletable | 2765.8ns | 130.7ns | 2115.7% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 96.2-264.0 ns)
     96.2 |########################################
    104.6 |##
    113.0 |########################################
    121.4 |
    129.8 |
    138.2 |
    146.6 |
    155.0 |
    163.3 |#
    171.7 |
    180.1 |
    188.5 |
    196.9 |
    205.3 |
    213.7 |
    222.1 |
    230.5 |#
    238.8 |
    247.2 |
    255.6 |
  (9 below, 14 above range)

v1_snapshot (n=120, range 100.3-293.8 ns)
    100.3 |
    110.0 |########################################
    119.6 |######
    129.3 |
    139.0 |
    148.7 |##
    158.3 |
    168.0 |
    177.7 |#
    187.4 |
    197.1 |
    206.7 |
    216.4 |
    226.1 |
    235.8 |
    245.4 |
    255.1 |
    264.8 |
    274.5 |
    284.2 |
  (18 below, 16 above range)

v2_decomposed (n=120, range 95.7-263.5 ns)
     95.7 |#################
    104.1 |#
    112.5 |########################################
    120.9 |##
    129.3 |
    137.7 |
    146.1 |
    154.5 |#
    162.8 |
    171.2 |
    179.6 |
    188.0 |
    196.4 |
    204.8 |
    213.2 |
    221.6 |
    230.0 |
    238.3 |
    246.7 |
    255.1 |
  (13 below, 15 above range)

v3_shapebound (n=120, range 95.1-239.5 ns)
     95.1 |############################
    102.3 |
    109.5 |#############################
    116.8 |########################################
    124.0 |##
    131.2 |
    138.4 |
    145.6 |
    152.9 |#
    160.1 |
    167.3 |#
    174.5 |
    181.7 |
    189.0 |
    196.2 |
    203.4 |
    210.6 |#
    217.8 |
    225.0 |
    232.3 |#
  (12 below, 12 above range)

v4_erased (n=120, range 96.4-289.1 ns)
     96.4 |##################################
    106.0 |
    115.7 |########################################
    125.3 |##
    134.9 |
    144.6 |###
    154.2 |#
    163.8 |
    173.5 |#
    183.1 |
    192.8 |
    202.4 |
    212.0 |
    221.7 |
    231.3 |
    240.9 |
    250.6 |
    260.2 |##
    269.8 |
    279.5 |
  (9 below, 15 above range)

v5_handletable (n=120, range 96.1-234.7 ns)
     96.1 |########################################
    103.1 |
    110.0 |
    116.9 |#####################################
    123.9 |
    130.8 |
    137.7 |
    144.6 |
    151.6 |
    158.5 |
    165.4 |
    172.4 |
    179.3 |
    186.2 |#
    193.1 |
    200.1 |
    207.0 |
    213.9 |
    220.9 |
    227.8 |
  (12 below, 14 above range)

```

## Diagnostics

- **v0_blob**: CV=51.4% (high variance, measurements may be unstable)
- **v0_blob**: bridge=2092.5% of algo (FFI overhead may distort results)
- **v1_snapshot**: CV=49.7% (high variance, measurements may be unstable)
- **v1_snapshot**: bridge=2097.4% of algo (FFI overhead may distort results)
- **v2_decomposed**: CV=50.5% (high variance, measurements may be unstable)
- **v2_decomposed**: bridge=2101.6% of algo (FFI overhead may distort results)
- **v3_shapebound**: CV=51.5% (high variance, measurements may be unstable)
- **v3_shapebound**: bridge=2114.8% of algo (FFI overhead may distort results)
- **v4_erased**: CV=52.3% (high variance, measurements may be unstable)
- **v4_erased**: bridge=2088.1% of algo (FFI overhead may distort results)
- **v5_handletable**: CV=49.8% (high variance, measurements may be unstable)
- **v5_handletable**: bridge=2125.2% of algo (FFI overhead may distort results)
