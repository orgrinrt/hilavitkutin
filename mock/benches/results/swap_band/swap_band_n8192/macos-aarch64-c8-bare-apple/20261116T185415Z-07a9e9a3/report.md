# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 19% faster than the next best (sb_plan)

sb_none (12.72 us) leads sb_plan (15.11 us) by 19%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 201.3x slower than the field

sb_value (2.56 ms) is 201.3x the fastest (12.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sb_plan shows alternating (throttle bounce) (autocorr -0.52)

sb_plan's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (12.72 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 201.3x the fastest

Fastest sb_none (12.72 us) to slowest sb_value (2.56 ms): 201.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 12720.8 ns median
- 2 variants significantly slower than baseline
- Spread: 201.29x (fastest 12720.8 ns, slowest 2560512.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | next-frame asymmetry: clean skip vs value-swap cone vs plan band |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000002 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 18484ns | 18627ns | 17485ns | 18308ns | 19249ns | base |
| sb_plan | 20671ns | 20625ns | 19849ns | 20516ns | 21314ns | +11.83% |
| sb_value | 2580908ns | 2566446ns | 2529475ns | 2564453ns | 2631308ns | +13862.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 12794ns | 12406ns | 13159ns | base | 0.640 |
| sb_plan | 15106ns | 14496ns | 15533ns | +18.07% | 0.542 |
| sb_value | 2574581ns | 2523176ns | 2624737ns | +20023.00% | 0.003 |

## Performance model

- Peak throughput: **0.660 Gops/s** (sb_none; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.644 | 97.5% |
| sb_plan | 0.542 | 82.1% |
| sb_value | 0.003 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 18484ns | 18484ns | base |
| sb_plan | 20671ns | 20671ns | +11.83% |
| sb_value | 2580908ns | 2580908ns | +13862.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 12721ns | base | --- | [12577, 13035] | --- | --- | --- | --- |
| sb_plan | 15106ns | +2354.1ns (+18.5%) | [+2119, +2598]ns | [14879, 15415] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 2560512ns | +2547375.0ns (+20025.3%) | [+2538846, +2571856]ns | [2551269, 2584694] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 12521ns | +17.6% | +20328.2% |
| 1 | 2 | 12646ns | +22.1% | +20211.3% |
| 1 | 3 | 12929ns | +14.0% | +19597.6% |
| 2 | 1 | 12579ns | +19.2% | +20244.6% |
| 2 | 2 | 12854ns | +18.7% | +19958.5% |
| 2 | 3 | 13142ns | +15.6% | +19394.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.012 | ok |
| sb_plan | -0.515 | HIGH- (thermal bounce) |
| sb_value | -0.234 | moderate- |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 45702799.5ns | 12794.2ns | 357214.5% | HIGH |
| sb_plan | 45956299.8ns | 15105.6ns | 304234.3% | HIGH |
| sb_value | 45845537.7ns | 2574580.6ns | 1780.7% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 12405.6-13159.0 ns)
  12405.6 |
  12443.2 |
  12480.9 |##########################
  12518.6 |#############
  12556.3 |########################################
  12593.9 |
  12631.6 |##########################
  12669.3 |
  12707.0 |
  12744.6 |
  12782.3 |#############
  12820.0 |#############
  12857.6 |
  12895.3 |#############
  12933.0 |
  12970.7 |#############
  13008.3 |
  13046.0 |
  13083.7 |#############
  13121.4 |########################################
  (1 below, 1 above range)

sb_plan (n=18, range 14495.8-15533.3 ns)
  14495.8 |
  14547.7 |
  14599.6 |####################
  14651.5 |
  14703.3 |########################################
  14755.2 |
  14807.1 |####################
  14859.0 |
  14910.8 |####################
  14962.7 |####################
  15014.6 |########################################
  15066.5 |
  15118.3 |####################
  15170.2 |####################
  15222.1 |####################
  15274.0 |
  15325.8 |
  15377.7 |####################
  15429.6 |########################################
  15481.5 |
  (1 below, 3 above range)

sb_value (n=18, range 2523176.4-2624736.8 ns)
  2523176.4 |####################
  2528254.4 |
  2533332.4 |####################
  2538410.5 |
  2543488.5 |####################
  2548566.5 |########################################
  2553644.5 |########################################
  2558722.5 |########################################
  2563800.6 |####################
  2568878.6 |####################
  2573956.6 |####################
  2579034.6 |
  2584112.6 |
  2589190.7 |####################
  2594268.7 |
  2599346.7 |
  2604424.7 |########################################
  2609502.7 |
  2614580.8 |
  2619658.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **sb_none**: bridge=358688.6% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=303132.1% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1782.7% of algo (FFI overhead may distort results)
