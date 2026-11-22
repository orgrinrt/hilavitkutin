# Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling

3 variants, 160 samples per variant.
Baseline: **match_2_arms**

## Highlights

Baseline for all deltas below: **match_2_arms**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### match_2_arms dominates: 27% faster than the next best (match_8_arms)

match_2_arms (105 ns) leads match_8_arms (134 ns) by 27%, a clear separation rather than a photo finish. CV 84.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (76 ns) is smaller than the fastest variant's own run-to-run std-dev (89 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (match_2_arms)

The baseline match_2_arms is the fastest (105 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### match_2_arms is inconsistent: worst-20% is 3.7x its best-20%

match_2_arms's best 20% of batches run at 85 ns but its worst 20% at 311 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (match_2_arms) is the fastest** at 104.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.73x (fastest 104.8 ns, slowest 181.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Match arm count: 2 vs 8 vs 64, jump-table dispatch scaling |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3c8715d9a4b682f1 |
| Counter frequency | 2112760997 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| match_2_arms | 2770ns | 1990ns | 1627ns | 2057ns | 6054ns | base |
| match_64_arms | 2804ns | 2068ns | 1693ns | 2060ns | 6147ns | +1.23% |
| match_8_arms | 2833ns | 1686ns | 1672ns | 2245ns | 5757ns | +2.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| match_2_arms | 145ns | 85ns | 311ns | base | 7.057 |
| match_64_arms | 246ns | 148ns | 538ns | +69.33% | 4.168 |
| match_8_arms | 223ns | 131ns | 454ns | +53.71% | 4.591 |

## Performance model

- Peak throughput: **12.109 Gops/s** (match_2_arms; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| match_2_arms | 9.766 | 80.7% |
| match_64_arms | 5.650 | 46.7% |
| match_8_arms | 7.662 | 63.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| match_2_arms | 2770ns | 2770ns | base |
| match_64_arms | 2804ns | 2804ns | +1.23% |
| match_8_arms | 2833ns | 2833ns | +2.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| match_2_arms | 105ns | base | --- | [98, 107] | --- | --- | --- | --- |
| match_64_arms | 181ns | +75.2ns (+71.8%) | [+73, +82]ns | [181, 182] | YES | 0.0000 | 0.0000 | 0 |
| match_8_arms | 134ns | +48.4ns (+46.2%) | [+45, +55]ns | [133, 162] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | match_2_arms | match_64_arms | match_8_arms |
|---|---|---|---|---|
| 1 | 1 | 107ns | +68.9% | +23.2% |
| 1 | 2 | 89ns | +103.6% | +48.3% |
| 1 | 3 | 107ns | +69.4% | +24.4% |
| 1 | 4 | 88ns | +67.9% | +51.3% |
| 1 | 5 | 87ns | +72.1% | +52.4% |
| 1 | 6 | 88ns | +109.8% | +48.1% |
| 1 | 7 | 106ns | +68.6% | +52.0% |
| 1 | 8 | 107ns | +38.8% | +147.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| match_2_arms | -0.086 | ok |
| match_64_arms | 0.049 | ok |
| match_8_arms | 0.187 | ok |

**Consistency summary:**

- **match_64_arms**: won 0/8, lost 8/8
- **match_8_arms**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| match_2_arms | 33.0ns | 145.1ns | 22.8% | HIGH |
| match_64_arms | 33.4ns | 245.7ns | 13.6% | HIGH |
| match_8_arms | 33.5ns | 223.0ns | 15.0% | HIGH |

## Distribution (algo ns)

```
match_2_arms (n=160, range 84.6-311.4 ns)
     84.6 |########################################
     95.9 |###################################
    107.2 |##############
    118.6 |#
    129.9 |
    141.3 |
    152.6 |
    164.0 |##
    175.3 |
    186.6 |
    198.0 |
    209.3 |
    220.7 |
    232.0 |#
    243.4 |
    254.7 |
    266.1 |###
    277.4 |####
    288.7 |####
    300.1 |#####
  (10 below, 13 above range)

match_64_arms (n=160, range 148.1-538.1 ns)
    148.1 |##############
    167.6 |########################################
    187.1 |
    206.6 |#
    226.1 |
    245.6 |
    265.1 |
    284.6 |
    304.1 |
    323.6 |
    343.1 |
    362.6 |
    382.1 |
    401.6 |
    421.1 |
    440.6 |
    460.1 |#
    479.6 |#
    499.1 |#
    518.6 |#
  (14 below, 16 above range)

match_8_arms (n=160, range 130.6-454.1 ns)
    130.6 |########################################
    146.8 |#########
    163.0 |#
    179.2 |#
    195.3 |
    211.5 |#
    227.7 |#
    243.9 |
    260.0 |#
    276.2 |##
    292.4 |
    308.6 |
    324.7 |
    340.9 |#
    357.1 |
    373.3 |#
    389.4 |##
    405.6 |####
    421.8 |#
    438.0 |#####
  (19 below, 10 above range)

```

## Diagnostics

- **match_2_arms**: CV=61.2% (high variance, measurements may be unstable)
- **match_2_arms**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **match_2_arms**: bridge=22.7% of algo (FFI overhead may distort results)
- **match_64_arms**: CV=61.7% (high variance, measurements may be unstable)
- **match_64_arms**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **match_64_arms**: bridge=13.5% of algo (FFI overhead may distort results)
- **match_8_arms**: CV=58.8% (high variance, measurements may be unstable)
- **match_8_arms**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **match_8_arms**: bridge=15.0% of algo (FFI overhead may distort results)
