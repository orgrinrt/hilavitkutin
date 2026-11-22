# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (13 ns) is smaller than the fastest variant's own run-to-run std-dev (140 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### minmax_intrinsic is inconsistent: worst-20% is 3.6x its best-20%

minmax_intrinsic's best 20% of batches run at 151 ns but its worst 20% at 551 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: minmax_branchless** at 178.8 ns median (-3.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.07x (fastest 178.8 ns, slowest 191.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5d49a3f1b7c06e82 |
| Counter frequency | 2112775109 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2858ns | 2075ns | 1761ns | 2127ns | 6146ns | base |
| minmax_branchless | 2609ns | 2063ns | 1692ns | 2056ns | 5184ns | -8.70% |
| minmax_intrinsic | 2795ns | 2077ns | 1692ns | 2062ns | 6098ns | -2.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 257ns | 159ns | 549ns | base | 15.957 |
| minmax_branchless | 227ns | 151ns | 450ns | -11.59% | 18.049 |
| minmax_intrinsic | 253ns | 151ns | 551ns | -1.32% | 16.171 |

## Performance model

- Peak throughput: **27.165 Gops/s** (minmax_branchless; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 22.105 | 81.4% |
| minmax_branchless | 22.908 | 84.3% |
| minmax_intrinsic | 21.356 | 78.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2858ns | 2858ns | base |
| minmax_branchless | 2609ns | 2609ns | -8.70% |
| minmax_intrinsic | 2795ns | 2795ns | -2.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 185ns | base | --- | [185, 189] | --- | --- | --- | --- |
| minmax_branchless | 179ns | -6.5ns (-3.5%) | [-14, -4]ns | [178, 180] | YES | 0.0000 | 0.0000 | 0 |
| minmax_intrinsic | 192ns | no significant difference | [-0, +0]ns | [185, 193] | no | 0.8115 | 0.8115 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 193ns | -2.0% | +0.1% |
| 1 | 2 | 185ns | -8.1% | +0.3% |
| 1 | 3 | 185ns | -3.5% | -18.1% |
| 1 | 4 | 185ns | -20.5% | -15.0% |
| 1 | 5 | 152ns | +11.8% | -0.3% |
| 1 | 6 | 192ns | -19.6% | +0.0% |
| 1 | 7 | 183ns | -1.9% | -0.2% |
| 1 | 8 | 161ns | +17.0% | +22.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.258 | moderate- |
| minmax_branchless | -0.071 | ok |
| minmax_intrinsic | 0.271 | moderate+ |

**Consistency summary:**

- **minmax_branchless**: won 6/8, lost 2/8
- **minmax_intrinsic**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 33.0ns | 256.7ns | 12.8% | HIGH |
| minmax_branchless | 29.9ns | 226.9ns | 13.2% | HIGH |
| minmax_intrinsic | 32.3ns | 253.3ns | 12.8% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 159.3-548.8 ns)
    159.3 |######
    178.7 |########################################
    198.2 |
    217.7 |
    237.2 |
    256.6 |
    276.1 |
    295.6 |
    315.1 |
    334.6 |
    354.0 |
    373.5 |
    393.0 |
    412.5 |
    432.0 |#
    451.4 |
    470.9 |#
    490.4 |
    509.9 |
    529.4 |
  (14 below, 19 above range)

minmax_branchless (n=160, range 150.8-449.7 ns)
    150.8 |##########
    165.7 |########################################
    180.7 |###################
    195.6 |
    210.6 |
    225.5 |
    240.5 |
    255.4 |
    270.3 |
    285.3 |
    300.2 |
    315.2 |
    330.1 |
    345.1 |
    360.0 |
    375.0 |
    389.9 |
    404.9 |
    419.8 |
    434.8 |
  (16 below, 19 above range)

minmax_intrinsic (n=160, range 151.3-550.6 ns)
    151.3 |######################
    171.2 |##########################
    191.2 |########################################
    211.2 |
    231.1 |
    251.1 |
    271.1 |
    291.0 |
    311.0 |
    331.0 |
    350.9 |
    370.9 |#
    390.9 |
    410.8 |####
    430.8 |
    450.8 |#
    470.7 |
    490.7 |
    510.7 |##
    530.6 |
  (16 below, 17 above range)

```

## Diagnostics

- **minmax_branch**: CV=58.8% (high variance, measurements may be unstable)
- **minmax_branch**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **minmax_branch**: bridge=13.0% of algo (FFI overhead may distort results)
- **minmax_branchless**: CV=61.8% (high variance, measurements may be unstable)
- **minmax_branchless**: bridge=13.3% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: CV=62.1% (high variance, measurements may be unstable)
- **minmax_intrinsic**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **minmax_intrinsic**: bridge=12.5% of algo (FFI overhead may distort results)
