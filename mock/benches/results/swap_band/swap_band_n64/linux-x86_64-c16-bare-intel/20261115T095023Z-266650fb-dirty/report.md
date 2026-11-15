# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 28% faster than the next best (sb_plan)

sb_none (7.90 us) leads sb_plan (10.11 us) by 28%, a clear separation rather than a photo finish. CV 15.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 2.1x slower than the field

sb_value (16.20 us) is 2.1x the fastest (7.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sb_none is fastest but the noisiest (CV 15.5%)

sb_none wins on median (7.90 us) yet has the highest variance (CV 15.5%), while sb_plan is the steadiest (CV 2.5%, 10.11 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sb_plan shows alternating (throttle bounce) (autocorr -0.69)

sb_plan's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (7.90 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (sb_none) is the fastest** at 7896.5 ns median
- 2 variants significantly slower than baseline
- Spread: 2.05x (fastest 7896.5 ns, slowest 16198.0 ns)

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
| sb_none | 10331ns | 9502ns | 9115ns | 9452ns | 12259ns | base |
| sb_plan | 11780ns | 11736ns | 11386ns | 11711ns | 12082ns | +14.03% |
| sb_value | 17943ns | 17798ns | 17558ns | 17735ns | 18447ns | +73.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 8455ns | 7507ns | 9855ns | base | 0.008 |
| sb_plan | 10167ns | 9782ns | 10455ns | +20.24% | 0.006 |
| sb_value | 16321ns | 15940ns | 16816ns | +93.03% | 0.004 |

## Performance model

- Peak throughput: **0.009 Gops/s** (sb_none; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.008 | 95.1% |
| sb_plan | 0.006 | 74.2% |
| sb_value | 0.004 | 46.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 10331ns | 10331ns | base |
| sb_plan | 11780ns | 11780ns | +14.03% |
| sb_value | 17943ns | 17943ns | +73.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 7896ns | base | --- | [7738, 8118] | --- | --- | --- | --- |
| sb_plan | 10113ns | +2281.6ns (+28.9%) | [+1992, +2487]ns | [10021, 10419] | YES | 0.0309 | 0.0309 | 0 |
| sb_value | 16198ns | +8337.5ns (+105.6%) | [+7943, +8526]ns | [15991, 16244] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 10711ns | -5.9% | +48.8% |
| 1 | 2 | 7821ns | +34.1% | +107.7% |
| 1 | 3 | 7729ns | +26.3% | +109.8% |
| 2 | 1 | 7722ns | +35.0% | +111.5% |
| 2 | 2 | 7810ns | +30.7% | +107.4% |
| 2 | 3 | 7903ns | +26.4% | +102.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.016 | ok |
| sb_plan | -0.690 | HIGH- (thermal bounce) |
| sb_value | -0.030 | ok |

**Consistency summary:**

- **sb_plan**: won 1/6, lost 5/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 281727.9ns | 8455.2ns | 3332.0% | HIGH |
| sb_plan | 239039.3ns | 10166.5ns | 2351.2% | HIGH |
| sb_value | 239762.3ns | 16321.1ns | 1469.0% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 7506.8-9854.8 ns)
   7506.8 |
   7624.2 |########################################
   7741.6 |########################################
   7859.0 |########################################
   7976.4 |#############
   8093.8 |##########################
   8211.2 |
   8328.6 |
   8446.0 |
   8563.4 |
   8680.8 |
   8798.2 |
   8915.6 |
   9033.0 |
   9150.4 |
   9267.8 |
   9385.2 |
   9502.6 |
   9620.0 |
   9737.4 |
  (2 below, 4 above range)

sb_plan (n=18, range 9782.3-10454.8 ns)
   9782.3 |
   9816.0 |#############
   9849.6 |#############
   9883.2 |
   9916.8 |
   9950.5 |
   9984.1 |#############
  10017.7 |
  10051.3 |##########################
  10085.0 |##########################
  10118.6 |#############
  10152.2 |
  10185.8 |#############
  10219.4 |
  10253.1 |
  10286.7 |
  10320.3 |
  10353.9 |
  10387.6 |##########################
  10421.2 |########################################
  (2 below, 2 above range)

sb_value (n=18, range 15940.0-16816.4 ns)
  15940.0 |########################################
  15983.8 |#############
  16027.6 |
  16071.4 |
  16115.2 |##########################
  16159.1 |##########################
  16202.9 |########################################
  16246.7 |#############
  16290.5 |##########################
  16334.3 |
  16378.2 |
  16422.0 |#############
  16465.8 |
  16509.6 |
  16553.4 |
  16597.3 |
  16641.1 |
  16684.9 |
  16728.7 |
  16772.5 |
  (2 below, 1 above range)

```

## Diagnostics

- **sb_none**: bridge=3027.6% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=2363.6% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1476.1% of algo (FFI overhead may distort results)
