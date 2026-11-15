# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 26% faster than the next best (sb_plan)

sb_none (11.02 us) leads sb_plan (13.92 us) by 26%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 468.5x slower than the field

sb_value (5.17 ms) is 468.5x the fastest (11.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sb_none shows alternating (throttle bounce) (autocorr -0.65)

sb_none's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (11.02 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 468.5x the fastest

Fastest sb_none (11.02 us) to slowest sb_value (5.17 ms): 468.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 11023.6 ns median
- 2 variants significantly slower than baseline
- Spread: 468.55x (fastest 11023.6 ns, slowest 5165083.0 ns)

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
| Counter frequency | 2112803027 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 14426ns | 14422ns | 14013ns | 14371ns | 14714ns | base |
| sb_plan | 17269ns | 17348ns | 16636ns | 17224ns | 17654ns | +19.71% |
| sb_value | 5179711ns | 5168675ns | 5104112ns | 5159623ns | 5247643ns | +35806.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 11030ns | 10722ns | 11278ns | base | 0.743 |
| sb_plan | 13875ns | 13386ns | 14169ns | +25.80% | 0.590 |
| sb_value | 5176105ns | 5100542ns | 5244059ns | +46829.22% | 0.002 |

## Performance model

- Peak throughput: **0.764 Gops/s** (sb_none; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.743 | 97.3% |
| sb_plan | 0.589 | 77.0% |
| sb_value | 0.002 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 14426ns | 14426ns | base |
| sb_plan | 17269ns | 17269ns | +19.71% |
| sb_value | 5179711ns | 5179711ns | +35806.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 11024ns | base | --- | [10888, 11150] | --- | --- | --- | --- |
| sb_plan | 13918ns | +2943.9ns (+26.7%) | [+2708, +3061]ns | [13756, 14065] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 5165083ns | +5154324.7ns (+46757.0%) | [+5136087, +5182977]ns | [5146940, 5194359] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 11121ns | +25.3% | +47413.6% |
| 1 | 2 | 10792ns | +27.8% | +47634.7% |
| 1 | 3 | 11056ns | +21.2% | +46486.2% |
| 2 | 1 | 10874ns | +28.5% | +47412.9% |
| 2 | 2 | 11272ns | +26.3% | +45758.1% |
| 2 | 3 | 10902ns | +29.2% | +46677.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.654 | HIGH- (thermal bounce) |
| sb_plan | 0.243 | moderate+ |
| sb_value | -0.086 | ok |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 30725081.7ns | 11029.6ns | 278569.3% | HIGH |
| sb_plan | 30739597.5ns | 13875.0ns | 221546.0% | HIGH |
| sb_value | 30757671.5ns | 5176105.5ns | 594.2% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 10721.9-11278.0 ns)
  10721.9 |#############
  10749.7 |#############
  10777.5 |#############
  10805.3 |
  10833.1 |
  10860.9 |#############
  10888.7 |#############
  10916.5 |
  10944.3 |#############
  10972.1 |
  10999.9 |########################################
  11027.7 |
  11055.5 |##########################
  11083.3 |
  11111.1 |#############
  11138.9 |
  11166.7 |#############
  11194.5 |#############
  11222.3 |
  11250.1 |#############
  (1 below, 2 above range)

sb_plan (n=18, range 13386.4-14169.3 ns)
  13386.4 |##########################
  13425.5 |
  13464.7 |
  13503.8 |#############
  13542.9 |
  13582.1 |
  13621.2 |
  13660.4 |
  13699.5 |#############
  13738.7 |
  13777.8 |#############
  13817.0 |
  13856.1 |#############
  13895.3 |##########################
  13934.4 |########################################
  13973.5 |
  14012.7 |#############
  14051.8 |#############
  14091.0 |#############
  14130.1 |
  (1 below, 3 above range)

sb_value (n=18, range 5100542.3-5244059.3 ns)
  5100542.3 |
  5107718.1 |
  5114894.0 |
  5122069.8 |
  5129245.7 |#############
  5136421.5 |#############
  5143597.4 |########################################
  5150773.2 |#############
  5157949.1 |#############
  5165124.9 |########################################
  5172300.8 |
  5179476.6 |#############
  5186652.5 |
  5193828.3 |
  5201004.2 |#############
  5208180.0 |
  5215355.9 |
  5222531.7 |
  5229707.6 |
  5236883.4 |#############
  (2 below, 3 above range)

```

## Diagnostics

- **sb_none**: bridge=278178.2% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=220247.0% of algo (FFI overhead may distort results)
- **sb_value**: bridge=593.2% of algo (FFI overhead may distort results)
