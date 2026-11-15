# branching, two transforms joined by a third

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 50% faster than the next best (evs_engine)

evs_std (40.41 us) leads evs_engine (60.50 us) by 50%, a clear separation rather than a photo finish. CV 5.3%.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (40.41 us median); no rival improves on it (all deltas are >= 0).


### evs_engine is inconsistent: worst-20% is 1.6x its best-20%

evs_engine's best 20% of batches run at 48.43 us but its worst 20% at 77.66 us (1.6x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 40410.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.50x (fastest 40410.6 ns, slowest 60503.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | branching, two transforms joined by a third |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000002 |
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 65359ns | 61365ns | 48792ns | 52895ns | 78848ns | +61.54% |
| evs_std | 40461ns | 40702ns | 36823ns | 38738ns | 42662ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 64543ns | 48431ns | 77658ns | +62.62% | 1.015 |
| evs_std | 39689ns | 36552ns | 41520ns | base | 1.651 |

## Performance model

- Peak throughput: 1.793 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.083 | 60.4% |
| evs_std | 1.622 | 90.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 65359ns | 65359ns | +61.54% |
| evs_std | 40461ns | 40461ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 40411ns | base | --- | [37151, 42132] | --- | --- | --- | --- |
| evs_engine | 60503ns | +21386.9ns (+52.9%) | [+11886, +42149]ns | [49037, 83666] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 37361ns | +37.1% |
| 1 | 2 | 41398ns | +93.5% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 340602.1ns | 64542.8ns | 527.7% | HIGH |
| evs_std | 340725.8ns | 39689.2ns | 858.5% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 48430.8-77658.2 ns)
  48430.8 |########################################
  49892.2 |
  51353.5 |
  52814.9 |####################
  54276.3 |####################
  55737.6 |
  57199.0 |
  58660.4 |
  60121.8 |
  61583.1 |
  63044.5 |
  64505.9 |
  65967.2 |####################
  67428.6 |
  68890.0 |
  70351.3 |
  71812.7 |
  73274.1 |
  74735.4 |
  76196.8 |####################
  (0 below, 2 above range)

evs_std (n=8, range 36551.5-41520.5 ns)
  36551.5 |########################################
  36799.9 |
  37048.4 |########################################
  37296.8 |
  37545.3 |########################################
  37793.7 |
  38042.2 |
  38290.6 |
  38539.1 |
  38787.5 |
  39036.0 |
  39284.4 |
  39532.9 |
  39781.3 |
  40029.8 |########################################
  40278.2 |
  40526.7 |########################################
  40775.1 |########################################
  41023.6 |
  41272.0 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: CV=22.0% (high variance, measurements may be unstable)
- evs_engine: bridge=564.9% of algo (FFI overhead may distort results)
- evs_std: bridge=841.1% of algo (FFI overhead may distort results)
