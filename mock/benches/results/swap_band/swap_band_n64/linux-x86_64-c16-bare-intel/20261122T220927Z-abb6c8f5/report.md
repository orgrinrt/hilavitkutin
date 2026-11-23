# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 31% faster than the next best (sb_plan)

sb_none (7.76 us) leads sb_plan (10.16 us) by 31%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 2.1x slower than the field

sb_value (16.55 us) is 2.1x the fastest (7.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (7.76 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (sb_none) is the fastest** at 7760.4 ns median
- 2 variants significantly slower than baseline
- Spread: 2.13x (fastest 7760.4 ns, slowest 16553.7 ns)

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
| Counter frequency | 2112749428 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 9352ns | 9370ns | 9029ns | 9335ns | 9539ns | base |
| sb_plan | 11731ns | 11769ns | 11419ns | 11725ns | 11897ns | +25.45% |
| sb_value | 19349ns | 18171ns | 17626ns | 17983ns | 22261ns | +106.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 7741ns | 7435ns | 7914ns | base | 0.008 |
| sb_plan | 10122ns | 9819ns | 10274ns | +30.75% | 0.006 |
| sb_value | 17617ns | 16000ns | 20296ns | +127.58% | 0.004 |

## Performance model

- Peak throughput: **0.009 Gops/s** (sb_none; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.008 | 95.8% |
| sb_plan | 0.006 | 73.1% |
| sb_value | 0.004 | 44.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 9352ns | 9352ns | base |
| sb_plan | 11731ns | 11731ns | +25.45% |
| sb_value | 19349ns | 19349ns | +106.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 7760ns | base | --- | [7660, 7857] | --- | --- | --- | --- |
| sb_plan | 10165ns | +2381.6ns (+30.7%) | [+2223, +2470]ns | [10077, 10214] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 16554ns | +8767.5ns (+113.0%) | [+8427, +9040]ns | [16188, 16745] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 7872ns | +29.1% | +110.4% |
| 1 | 2 | 7652ns | +32.6% | +112.9% |
| 1 | 3 | 7801ns | +30.5% | +116.1% |
| 2 | 1 | 7843ns | +29.8% | +105.0% |
| 2 | 2 | 7760ns | +28.1% | +139.3% |
| 2 | 3 | 7750ns | +31.2% | +110.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.452 | moderate- |
| sb_plan | -0.302 | moderate- |
| sb_value | -0.498 | moderate- |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 239068.4ns | 7741.3ns | 3088.2% | HIGH |
| sb_plan | 239074.2ns | 10122.0ns | 2361.9% | HIGH |
| sb_value | 255611.9ns | 17617.4ns | 1450.9% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 7434.9-7913.7 ns)
   7434.9 |
   7458.9 |#############
   7482.8 |
   7506.8 |
   7530.7 |
   7554.6 |
   7578.6 |
   7602.5 |##########################
   7626.4 |
   7650.4 |##########################
   7674.3 |
   7698.3 |#############
   7722.2 |
   7746.1 |########################################
   7770.1 |
   7794.0 |#############
   7818.0 |#############
   7841.9 |#############
   7865.8 |#############
   7889.8 |
  (1 below, 4 above range)

sb_plan (n=18, range 9818.8-10274.1 ns)
   9818.8 |
   9841.6 |
   9864.3 |
   9887.1 |
   9909.9 |
   9932.6 |########
   9955.4 |
   9978.1 |
  10000.9 |########
  10023.7 |
  10046.4 |########
  10069.2 |########
  10092.0 |########
  10114.7 |
  10137.5 |########
  10160.2 |########################################
  10183.0 |
  10205.8 |
  10228.5 |########
  10251.3 |########
  (2 below, 3 above range)

sb_value (n=18, range 16000.1-20295.7 ns)
  16000.1 |########################
  16214.9 |########################
  16429.7 |########################################
  16644.5 |########
  16859.2 |########
  17074.0 |########
  17288.8 |
  17503.6 |
  17718.4 |
  17933.1 |
  18147.9 |
  18362.7 |########
  18577.5 |
  18792.2 |
  19007.0 |
  19221.8 |
  19436.6 |
  19651.4 |
  19866.1 |
  20080.9 |
  (2 below, 1 above range)

```

## Diagnostics

- **sb_none**: bridge=3081.0% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=2351.9% of algo (FFI overhead may distort results)
- **sb_value**: CV=24.9% (high variance, measurements may be unstable)
- **sb_value**: bridge=1444.1% of algo (FFI overhead may distort results)
