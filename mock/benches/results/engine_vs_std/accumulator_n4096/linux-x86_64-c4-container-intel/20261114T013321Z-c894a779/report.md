# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 644% faster than the next best (evs_engine)

evs_std (2.29 us) leads evs_engine (17.05 us) by 644%, a clear separation rather than a photo finish. CV 27.7%.


### evs_std_par is an outlier: 26.2x slower than the field

evs_std_par (60.02 us) is 26.2x the fastest (2.29 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (2.29 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std} vs {evs_engine, evs_engine_par, evs_std_par} (644% apart)

The field splits into a fast tier {evs_std} and a slow tier {evs_engine, evs_engine_par, evs_std_par} with a 644% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 26.2x the fastest

Fastest evs_std (2.29 us) to slowest evs_std_par (60.02 us): 26.2x. The strategy choice matters a lot for this workload.


### evs_std is inconsistent: worst-20% is 1.9x its best-20%

evs_std's best 20% of batches run at 1.42 us but its worst 20% at 2.66 us (1.9x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 2291.6 ns median
- 3 variants significantly slower than baseline
- Spread: 26.19x (fastest 2291.6 ns, slowest 60015.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | accumulator, a transform feeding the append surface |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000003 |
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 19040ns | 17217ns | 14494ns | 15119ns | 23118ns | +690.71% |
| evs_engine_par | 52344ns | 52047ns | 40002ns | 44476ns | 61331ns | +2073.77% |
| evs_std | 2408ns | 2640ns | 1559ns | 2022ns | 2910ns | base |
| evs_std_par | 67094ns | 60466ns | 52228ns | 56214ns | 78970ns | +2686.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 18828ns | 14323ns | 22907ns | +770.27% | 0.218 |
| evs_engine_par | 52005ns | 39761ns | 60930ns | +2303.78% | 0.079 |
| evs_std | 2163ns | 1423ns | 2664ns | base | 1.893 |
| evs_std_par | 66716ns | 51939ns | 78593ns | +2983.75% | 0.061 |

## Performance model

- Peak throughput: 2.878 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.240 | 8.3% |
| evs_engine_par | 0.079 | 2.8% |
| evs_std | 1.787 | 62.1% |
| evs_std_par | 0.068 | 2.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 19040ns | 19040ns | +690.71% |
| evs_engine_par | 52344ns | 52344ns | +2073.77% |
| evs_std | 2408ns | 2408ns | base |
| evs_std_par | 67094ns | 67094ns | +2686.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 2292ns | base | --- | [1446, 2510] | --- | --- | --- | --- |
| evs_engine | 17048ns | +15138.3ns (+660.6%) | [+12946, +17553]ns | [14391, 20064] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 51707ns | +49738.8ns (+2170.5%) | [+38496, +61609]ns | [39941, 63851] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 60015ns | +57723.5ns (+2519.0%) | [+51741, +91722]ns | [54315, 93168] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 2368ns | +726.8% | +2522.0% | +2270.3% |
| 1 | 2 | 1493ns | +879.1% | +2723.6% | +5170.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 0/2, lost 2/2
- evs_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 19306.1ns | 18828.2ns | 102.5% | HIGH |
| evs_engine_par | 22345.8ns | 52005.5ns | 43.0% | HIGH |
| evs_std | 20225.5ns | 2163.5ns | 934.9% | HIGH |
| evs_std_par | 21854.4ns | 66716.5ns | 32.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 14323.1-22906.9 ns)
  14323.1 |########################################
  14752.3 |####################
  15181.5 |####################
  15610.7 |
  16039.9 |
  16469.1 |
  16898.2 |
  17327.4 |
  17756.6 |
  18185.8 |
  18615.0 |####################
  19044.2 |####################
  19473.4 |
  19902.6 |####################
  20331.8 |
  20761.0 |
  21190.2 |
  21619.4 |
  22048.5 |
  22477.7 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 39761.1-60929.8 ns)
  39761.1 |########################################
  40819.5 |
  41878.0 |
  42936.4 |
  43994.8 |####################
  45053.3 |
  46111.7 |
  47170.2 |
  48228.6 |####################
  49287.0 |
  50345.5 |
  51403.9 |
  52462.3 |
  53520.8 |
  54579.2 |####################
  55637.7 |
  56696.1 |
  57754.5 |
  58813.0 |
  59871.4 |####################
  (0 below, 2 above range)

evs_std (n=8, range 1423.2-2664.1 ns)
   1423.2 |########################################
   1485.2 |####################
   1547.3 |
   1609.3 |
   1671.4 |
   1733.4 |
   1795.5 |
   1857.5 |
   1919.5 |
   1981.6 |
   2043.6 |
   2105.7 |
   2167.7 |
   2229.8 |####################
   2291.8 |####################
   2353.8 |####################
   2415.9 |
   2477.9 |####################
   2540.0 |
   2602.0 |
  (0 below, 1 above range)

evs_std_par (n=8, range 51938.6-78592.5 ns)
  51938.6 |########################################
  53271.3 |########################################
  54604.0 |########################################
  55936.7 |
  57269.4 |########################################
  58602.1 |
  59934.8 |
  61267.5 |########################################
  62600.2 |
  63932.9 |########################################
  65265.6 |
  66598.3 |
  67931.0 |
  69263.7 |
  70596.3 |
  71929.0 |
  73261.7 |
  74594.4 |
  75927.1 |
  77259.8 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: CV=32.2% (high variance, measurements may be unstable)
- evs_engine: bridge=106.3% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=41.9% of algo (FFI overhead may distort results)
- evs_std: CV=29.3% (high variance, measurements may be unstable)
- evs_std: bridge=850.9% of algo (FFI overhead may distort results)
- evs_std_par: CV=24.3% (high variance, measurements may be unstable)
- evs_std_par: bridge=35.9% of algo (FFI overhead may distort results)
