# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (v3_shapebound, v2_decomposed) are a dead heat (<1%)

v3_shapebound (96 ns) and v2_decomposed (96 ns) differ by 0.57%, inside the noise, even though the wider field spreads 23.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### v1_snapshot's edge over baseline is significant but tiny (-1 ns, 0.85%)

v1_snapshot differs from baseline v0_blob by -1 ns (0.85%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 95.9 ns median (-18.5% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.23x (fastest 95.9 ns, slowest 118.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under L1-evicting heavy-memory mix-in |
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
| v0_blob | 4011ns | 4314ns | 3530ns | 4060ns | 4345ns | base |
| v1_snapshot | 4007ns | 4313ns | 3530ns | 4057ns | 4333ns | -0.10% |
| v2_decomposed | 3909ns | 3563ns | 3529ns | 3887ns | 4352ns | -2.55% |
| v3_shapebound | 3744ns | 3534ns | 3528ns | 3623ns | 4322ns | -6.65% |
| v4_erased | 4008ns | 4316ns | 3531ns | 4059ns | 4332ns | -0.08% |
| v5_handletable | 4011ns | 4317ns | 3533ns | 4063ns | 4335ns | +0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 110ns | 96ns | 119ns | base | 9.347 |
| v1_snapshot | 109ns | 95ns | 119ns | -0.67% | 9.410 |
| v2_decomposed | 106ns | 96ns | 118ns | -3.19% | 9.655 |
| v3_shapebound | 102ns | 95ns | 117ns | -7.31% | 10.084 |
| v4_erased | 109ns | 96ns | 119ns | -0.19% | 9.364 |
| v5_handletable | 110ns | 97ns | 119ns | +0.49% | 9.301 |

## Performance model

- Peak throughput: **10.738 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 8.700 | 81.0% |
| v1_snapshot | 8.782 | 81.8% |
| v2_decomposed | 10.617 | 98.9% |
| v3_shapebound | 10.678 | 99.4% |
| v4_erased | 8.745 | 81.4% |
| v5_handletable | 8.649 | 80.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 4011ns | 4011ns | base |
| v1_snapshot | 4007ns | 4007ns | -0.10% |
| v2_decomposed | 3909ns | 3909ns | -2.55% |
| v3_shapebound | 3744ns | 3744ns | -6.65% |
| v4_erased | 4008ns | 4008ns | -0.08% |
| v5_handletable | 4011ns | 4011ns | +0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 118ns | base | --- | [117, 118] | --- | --- | --- | --- |
| v1_snapshot | 117ns | -1.0ns (-0.8%) | [-1, -1]ns | [116, 117] | YES | 0.0000 | 0.0000 | 0 |
| v2_decomposed | 96ns | -0.8ns (-0.7%) | [-5, -0]ns | [96, 117] | YES | 0.0001 | 0.0000 | 1 |
| v3_shapebound | 96ns | -1.7ns (-1.4%) | [-22, -1]ns | [96, 96] | YES | 0.0000 | 0.0000 | 0 |
| v4_erased | 117ns | no significant difference | [-1, +0]ns | [117, 118] | no | 0.8546 | 0.8546 | 1 |
| v5_handletable | 118ns | +0.6ns (+0.5%) | [+0, +1]ns | [118, 118] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 96ns | +21.5% | +21.6% | -1.0% | -0.4% | +22.9% |
| 1 | 2 | 96ns | +21.1% | +21.5% | -0.9% | +22.0% | +22.7% |
| 1 | 3 | 118ns | -18.8% | -18.5% | -18.9% | -18.1% | -17.8% |
| 1 | 4 | 96ns | +21.3% | +21.6% | +21.4% | +22.4% | +0.6% |
| 1 | 5 | 96ns | +21.3% | -0.8% | -0.8% | +0.1% | +22.7% |
| 2 | 1 | 118ns | -18.8% | -18.4% | -0.9% | -17.9% | -17.8% |
| 2 | 2 | 118ns | -19.0% | -18.3% | -18.8% | -18.0% | -17.7% |
| 2 | 3 | 96ns | -0.9% | +22.0% | -0.6% | +22.6% | +23.1% |
| 2 | 4 | 118ns | -0.9% | -0.6% | -18.8% | -18.0% | +0.8% |
| 2 | 5 | 118ns | -19.1% | -0.4% | -18.8% | +0.1% | +0.6% |
| 3 | 1 | 118ns | -1.0% | -19.0% | -18.8% | +0.3% | +0.6% |
| 3 | 2 | 96ns | -0.9% | -0.4% | +21.7% | +22.2% | +0.8% |
| 3 | 3 | 118ns | -1.0% | -0.5% | -0.9% | -0.2% | +0.4% |
| 3 | 4 | 118ns | -0.7% | -18.4% | -18.6% | -0.2% | -17.6% |
| 3 | 5 | 118ns | -0.8% | -18.6% | -18.6% | +0.1% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.048 | ok |
| v1_snapshot | -0.145 | ok |
| v2_decomposed | -0.008 | ok |
| v3_shapebound | -0.045 | ok |
| v4_erased | -0.057 | ok |
| v5_handletable | -0.148 | ok |

**Consistency summary:**

- **v1_snapshot**: won 11/15, lost 4/15
- **v2_decomposed**: won 11/15, lost 4/15
- **v3_shapebound**: won 13/15, lost 2/15
- **v4_erased**: won 7/15, lost 5/15
- **v5_handletable**: won 4/15, lost 11/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 2288.1ns | 109.6ns | 2088.5% | HIGH |
| v1_snapshot | 2285.7ns | 108.8ns | 2100.3% | HIGH |
| v2_decomposed | 2232.4ns | 106.1ns | 2104.8% | HIGH |
| v3_shapebound | 2137.4ns | 101.5ns | 2104.8% | HIGH |
| v4_erased | 2287.5ns | 109.4ns | 2091.8% | HIGH |
| v5_handletable | 2317.5ns | 110.1ns | 2105.0% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 96.3-119.0 ns)
     96.3 |########################################
     97.4 |
     98.5 |
     99.7 |#
    100.8 |#
    102.0 |
    103.1 |
    104.2 |
    105.4 |
    106.5 |
    107.6 |
    108.8 |
    109.9 |
    111.1 |
    112.2 |
    113.3 |
    114.5 |
    115.6 |
    116.8 |############################
    117.9 |#######################################
  (6 below, 5 above range)

v1_snapshot (n=120, range 95.4-119.4 ns)
     95.4 |#####################################
     96.6 |
     97.8 |
     99.0 |
    100.2 |
    101.4 |
    102.6 |
    103.8 |
    105.0 |
    106.2 |
    107.4 |
    108.6 |
    109.8 |
    111.0 |
    112.2 |
    113.4 |
    114.6 |
    115.8 |########################################
    117.0 |######################
    118.2 |
  (7 below, 3 above range)

v2_decomposed (n=120, range 95.6-118.1 ns)
     95.6 |########################################
     96.7 |
     97.8 |
     99.0 |
    100.1 |
    101.2 |
    102.3 |
    103.5 |
    104.6 |
    105.7 |
    106.8 |
    108.0 |
    109.1 |
    110.2 |
    111.4 |
    112.5 |
    113.6 |
    114.7 |
    115.9 |#######
    117.0 |###############################
  (8 below, 4 above range)

v3_shapebound (n=120, range 95.4-117.4 ns)
     95.4 |########################################
     96.5 |
     97.6 |
     98.7 |#
     99.8 |
    100.9 |
    102.0 |
    103.1 |
    104.2 |
    105.3 |
    106.4 |
    107.5 |
    108.6 |
    109.7 |
    110.8 |
    111.9 |
    113.0 |
    114.1 |
    115.2 |
    116.3 |##############
  (10 below, 6 above range)

v4_erased (n=120, range 96.0-118.8 ns)
     96.0 |###############################
     97.2 |
     98.3 |
     99.5 |
    100.6 |
    101.7 |
    102.9 |
    104.0 |
    105.2 |
    106.3 |
    107.4 |
    108.6 |
    109.7 |
    110.8 |
    112.0 |
    113.1 |
    114.3 |
    115.4 |
    116.5 |########################
    117.7 |########################################
  (13 below, 3 above range)

v5_handletable (n=120, range 96.9-119.3 ns)
     96.9 |#####################
     98.0 |
     99.1 |
    100.2 |
    101.4 |
    102.5 |
    103.6 |
    104.7 |
    105.8 |
    107.0 |
    108.1 |
    109.2 |
    110.3 |
    111.4 |
    112.6 |
    113.7 |
    114.8 |
    115.9 |
    117.0 |
    118.2 |########################################
  (10 below, 3 above range)

```

## Diagnostics

- **v0_blob**: bridge=2091.8% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2111.6% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2099.9% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2105.6% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2104.0% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2107.2% of algo (FFI overhead may distort results)
