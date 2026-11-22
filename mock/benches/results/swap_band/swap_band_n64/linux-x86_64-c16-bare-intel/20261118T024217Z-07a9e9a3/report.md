# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 41% faster than the next best (sb_plan)

sb_none (6.61 us) leads sb_plan (9.30 us) by 41%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 2.5x slower than the field

sb_value (16.59 us) is 2.5x the fastest (6.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (6.61 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (sb_none) is the fastest** at 6611.6 ns median
- 2 variants significantly slower than baseline
- Spread: 2.51x (fastest 6611.6 ns, slowest 16590.3 ns)

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
| sb_none | 8224ns | 8221ns | 8184ns | 8215ns | 8257ns | base |
| sb_plan | 10934ns | 10897ns | 10862ns | 10890ns | 11037ns | +32.95% |
| sb_value | 18180ns | 18216ns | 17743ns | 18101ns | 18516ns | +121.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 6613ns | 6573ns | 6642ns | base | 0.010 |
| sb_plan | 9328ns | 9256ns | 9418ns | +41.05% | 0.007 |
| sb_value | 16551ns | 16120ns | 16869ns | +150.28% | 0.004 |

## Performance model

- Peak throughput: **0.010 Gops/s** (sb_none; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.010 | 99.4% |
| sb_plan | 0.007 | 70.7% |
| sb_value | 0.004 | 39.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 8224ns | 8224ns | base |
| sb_plan | 10934ns | 10934ns | +32.95% |
| sb_value | 18180ns | 18180ns | +121.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 6612ns | base | --- | [6600, 6631] | --- | --- | --- | --- |
| sb_plan | 9297ns | +2676.7ns (+40.5%) | [+2656, +2707]ns | [9280, 9310] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 16590ns | +9955.9ns (+150.6%) | [+9743, +10167]ns | [16341, 16760] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 6619ns | +39.9% | +150.0% |
| 1 | 2 | 6584ns | +41.2% | +149.9% |
| 1 | 3 | 6611ns | +41.3% | +152.4% |
| 2 | 1 | 6575ns | +41.4% | +149.2% |
| 2 | 2 | 6640ns | +40.2% | +143.3% |
| 2 | 3 | 6650ns | +39.8% | +153.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.003 | ok |
| sb_plan | -0.079 | ok |
| sb_value | -0.367 | moderate- |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 239054.4ns | 6613.0ns | 3614.9% | HIGH |
| sb_plan | 239063.4ns | 9327.8ns | 2562.9% | HIGH |
| sb_value | 239382.7ns | 16551.4ns | 1446.3% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 6572.9-6642.0 ns)
   6572.9 |########################################
   6576.4 |
   6579.8 |
   6583.3 |####################
   6586.8 |
   6590.2 |
   6593.7 |
   6597.1 |########################################
   6600.6 |####################
   6604.0 |
   6607.5 |####################
   6610.9 |########################################
   6614.4 |
   6617.8 |####################
   6621.3 |
   6624.8 |########################################
   6628.2 |
   6631.7 |####################
   6635.1 |
   6638.6 |####################
  (1 below, 3 above range)

sb_plan (n=18, range 9255.8-9417.7 ns)
   9255.8 |#############
   9263.9 |#############
   9272.0 |########################################
   9280.1 |
   9288.2 |########################################
   9296.3 |##########################
   9304.4 |########################################
   9312.5 |
   9320.6 |
   9328.7 |
   9336.8 |#############
   9344.9 |
   9353.0 |
   9361.1 |
   9369.2 |
   9377.2 |
   9385.3 |
   9393.4 |
   9401.5 |
   9409.6 |#############
  (1 below, 2 above range)

sb_value (n=18, range 16120.1-16869.3 ns)
  16120.1 |
  16157.6 |########################################
  16195.1 |####################
  16232.5 |
  16270.0 |####################
  16307.4 |
  16344.9 |
  16382.3 |####################
  16419.8 |########################################
  16457.3 |
  16494.7 |
  16532.2 |####################
  16569.6 |
  16607.1 |####################
  16644.6 |####################
  16682.0 |####################
  16719.5 |
  16756.9 |####################
  16794.4 |########################################
  16831.9 |########################################
  (1 below, 1 above range)

```

## Diagnostics

- **sb_none**: bridge=3615.2% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=2571.5% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1441.1% of algo (FFI overhead may distort results)
