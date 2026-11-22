# Rotate strategy: intrinsic const vs manual shifts vs variable amount

3 variants, 160 samples per variant.
Baseline: **rot_intrinsic_const**

## Highlights

Baseline for all deltas below: **rot_intrinsic_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rot_variable_amount is an outlier: 2.8x slower than the field

rot_variable_amount (220 ns) is 2.8x the fastest (79 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rot_manual_shifts is fastest but the noisiest (CV 87.4%)

rot_manual_shifts wins on median (79 ns) yet has the highest variance (CV 87.4%), while rot_variable_amount is the steadiest (CV 83.1%, 220 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (rot_manual_shifts, rot_intrinsic_const) are a dead heat (<1%)

rot_manual_shifts (79 ns) and rot_intrinsic_const (79 ns) differ by 0.13%, inside the noise, even though the wider field spreads 179.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### rot_manual_shifts's comparison is tie-heavy (10% tied pairs)

10% of paired samples for rot_manual_shifts are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### rot_manual_shifts is inconsistent: worst-20% is 3.7x its best-20%

rot_manual_shifts's best 20% of batches run at 64 ns but its worst 20% at 238 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: rot_manual_shifts** at 78.8 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.80x (fastest 78.8 ns, slowest 220.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Rotate strategy: intrinsic const vs manual shifts vs variable amount |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71f692ab4e08d325 |
| Counter frequency | 2112804954 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rot_intrinsic_const | 2775ns | 1966ns | 1663ns | 2065ns | 6020ns | base |
| rot_manual_shifts | 2716ns | 1962ns | 1603ns | 1985ns | 6022ns | -2.15% |
| rot_variable_amount | 2776ns | 2104ns | 1720ns | 2013ns | 6119ns | +0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rot_intrinsic_const | 111ns | 67ns | 241ns | base | 9.204 |
| rot_manual_shifts | 108ns | 64ns | 238ns | -2.67% | 9.456 |
| rot_variable_amount | 290ns | 180ns | 636ns | +160.30% | 3.536 |

## Performance model

- Peak throughput: **15.888 Gops/s** (rot_manual_shifts; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rot_intrinsic_const | 12.978 | 81.7% |
| rot_manual_shifts | 12.995 | 81.8% |
| rot_variable_amount | 4.646 | 29.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rot_intrinsic_const | 2775ns | 2775ns | base |
| rot_manual_shifts | 2716ns | 2716ns | -2.15% |
| rot_variable_amount | 2776ns | 2776ns | +0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rot_intrinsic_const | 79ns | base | --- | [79, 79] | --- | --- | --- | --- |
| rot_manual_shifts | 79ns | -3.8ns (-4.8%) | [-14, -0]ns | [79, 79] | YES | 0.0000 | 0.0000 | 16 |
| rot_variable_amount | 220ns | +141.4ns (+179.2%) | [+118, +142]ns | [220, 220] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rot_intrinsic_const | rot_manual_shifts | rot_variable_amount |
|---|---|---|---|---|
| 1 | 1 | 65ns | +22.1% | +179.3% |
| 1 | 2 | 79ns | +0.0% | +128.7% |
| 1 | 3 | 65ns | +0.1% | +241.3% |
| 1 | 4 | 79ns | -18.1% | +128.7% |
| 1 | 5 | 79ns | +0.0% | +179.4% |
| 1 | 6 | 79ns | -18.2% | +179.5% |
| 1 | 7 | 86ns | -8.1% | +156.7% |
| 1 | 8 | 79ns | -18.3% | +128.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rot_intrinsic_const | -0.068 | ok |
| rot_manual_shifts | -0.377 | moderate- |
| rot_variable_amount | -0.125 | ok |

**Consistency summary:**

- **rot_manual_shifts**: won 4/8, lost 1/8
- **rot_variable_amount**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rot_intrinsic_const | 34.1ns | 111.3ns | 30.6% | HIGH |
| rot_manual_shifts | 32.8ns | 108.3ns | 30.3% | HIGH |
| rot_variable_amount | 31.2ns | 289.6ns | 10.8% | HIGH |

## Distribution (algo ns)

```
rot_intrinsic_const (n=160, range 66.7-240.9 ns)
     66.7 |
     75.4 |########################################
     84.1 |##
     92.8 |#
    101.6 |
    110.3 |
    119.0 |
    127.7 |
    136.4 |
    145.1 |
    153.8 |
    162.5 |
    171.3 |
    180.0 |
    188.7 |#
    197.4 |#
    206.1 |
    214.8 |##
    223.5 |
    232.2 |#
  (27 below, 13 above range)

rot_manual_shifts (n=160, range 64.5-237.9 ns)
     64.5 |#################################
     73.1 |########################################
     81.8 |
     90.5 |
     99.2 |#
    107.8 |#
    116.5 |
    125.2 |
    133.8 |
    142.5 |
    151.2 |
    159.9 |###
    168.5 |#
    177.2 |##
    185.9 |
    194.6 |
    203.2 |
    211.9 |##
    220.6 |#
    229.3 |##
  (10 below, 17 above range)

rot_variable_amount (n=160, range 180.3-635.7 ns)
    180.3 |##########################
    203.1 |########################################
    225.9 |#
    248.6 |
    271.4 |
    294.2 |#
    316.9 |
    339.7 |
    362.5 |
    385.3 |
    408.0 |
    430.8 |
    453.6 |
    476.3 |#
    499.1 |
    521.9 |#
    544.7 |#
    567.4 |#
    590.2 |#
    613.0 |
  (19 below, 17 above range)

```

## Diagnostics

- **rot_intrinsic_const**: CV=61.1% (high variance, measurements may be unstable)
- **rot_intrinsic_const**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **rot_intrinsic_const**: bridge=30.3% of algo (FFI overhead may distort results)
- **rot_manual_shifts**: CV=63.6% (high variance, measurements may be unstable)
- **rot_manual_shifts**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **rot_manual_shifts**: bridge=30.2% of algo (FFI overhead may distort results)
- **rot_variable_amount**: CV=63.2% (high variance, measurements may be unstable)
- **rot_variable_amount**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **rot_variable_amount**: bridge=10.8% of algo (FFI overhead may distort results)
