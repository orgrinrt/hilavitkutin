# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 33% faster than the next best (sb_plan)

sb_none (9.40 us) leads sb_plan (12.49 us) by 33%, a clear separation rather than a photo finish. CV 6.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 551.7x slower than the field

sb_value (5.19 ms) is 551.7x the fastest (9.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sb_none shows alternating (throttle bounce) (autocorr -0.55)

sb_none's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (9.40 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 551.7x the fastest

Fastest sb_none (9.40 us) to slowest sb_value (5.19 ms): 551.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 9398.8 ns median
- 2 variants significantly slower than baseline
- Spread: 551.69x (fastest 9398.8 ns, slowest 5185207.3 ns)

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
| Counter frequency | 2112775712 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 12891ns | 12827ns | 12446ns | 12743ns | 13335ns | base |
| sb_plan | 16566ns | 15942ns | 15613ns | 15889ns | 18058ns | +28.50% |
| sb_value | 5183720ns | 5188699ns | 5086424ns | 5165319ns | 5259969ns | +40111.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 9488ns | 9075ns | 9917ns | base | 0.863 |
| sb_plan | 12993ns | 12197ns | 14208ns | +36.94% | 0.630 |
| sb_value | 5180061ns | 5082804ns | 5256182ns | +54494.64% | 0.002 |

## Performance model

- Peak throughput: **0.903 Gops/s** (sb_none; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.872 | 96.6% |
| sb_plan | 0.656 | 72.7% |
| sb_value | 0.002 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 12891ns | 12891ns | base |
| sb_plan | 16566ns | 16566ns | +28.50% |
| sb_value | 5183720ns | 5183720ns | +40111.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 9399ns | base | --- | [9252, 9463] | --- | --- | --- | --- |
| sb_plan | 12486ns | +3161.0ns (+33.6%) | [+2943, +3246]ns | [12388, 12621] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 5185207ns | +5175986.1ns (+55070.4%) | [+5125111, +5216731]ns | [5134367, 5223591] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 9224ns | +38.7% | +57103.1% |
| 1 | 2 | 9410ns | +30.5% | +54636.9% |
| 1 | 3 | 9341ns | +34.0% | +55818.7% |
| 2 | 1 | 9423ns | +34.6% | +54103.1% |
| 2 | 2 | 9190ns | +35.4% | +55275.7% |
| 2 | 3 | 9582ns | +28.6% | +53958.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.553 | HIGH- (thermal bounce) |
| sb_plan | -0.313 | moderate- |
| sb_value | -0.080 | ok |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 30803839.1ns | 9488.2ns | 324653.4% | HIGH |
| sb_plan | 30761528.3ns | 12992.9ns | 236755.8% | HIGH |
| sb_value | 30817867.3ns | 5180060.8ns | 594.9% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 9075.1-9916.9 ns)
   9075.1 |#############
   9117.2 |#############
   9159.3 |#############
   9201.4 |#############
   9243.5 |#############
   9285.6 |
   9327.7 |#############
   9369.7 |########################################
   9411.8 |########################################
   9453.9 |
   9496.0 |##########################
   9538.1 |
   9580.2 |#############
   9622.3 |
   9664.4 |
   9706.4 |#############
   9748.5 |
   9790.6 |
   9832.7 |
   9874.8 |
  (1 below, 1 above range)

sb_plan (n=18, range 12196.9-14208.0 ns)
  12196.9 |####################
  12297.5 |####################
  12398.0 |########################################
  12498.6 |########################################
  12599.1 |####################
  12699.7 |####################
  12800.2 |
  12900.8 |
  13001.3 |
  13101.9 |
  13202.4 |
  13303.0 |
  13403.5 |
  13504.1 |
  13604.7 |
  13705.2 |
  13805.8 |
  13906.3 |
  14006.9 |
  14107.4 |
  (1 below, 1 above range)

sb_value (n=18, range 5082803.7-5256181.5 ns)
  5082803.7 |########################################
  5091472.6 |
  5100141.5 |####################
  5108810.4 |
  5117479.3 |
  5126148.2 |####################
  5134817.1 |####################
  5143486.0 |########################################
  5152154.9 |
  5160823.7 |
  5169492.6 |
  5178161.5 |####################
  5186830.4 |####################
  5195499.3 |####################
  5204168.2 |
  5212837.1 |####################
  5221506.0 |########################################
  5230174.9 |####################
  5238843.8 |
  5247512.6 |
  (1 below, 3 above range)

```

## Diagnostics

- **sb_none**: bridge=326544.1% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=245830.1% of algo (FFI overhead may distort results)
- **sb_value**: bridge=591.5% of algo (FFI overhead may distort results)
