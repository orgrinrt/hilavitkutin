# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 639% faster than the next best (evs_engine)

evs_std (5.54 us) leads evs_engine (40.89 us) by 639%, a clear separation rather than a photo finish. CV 3.3%.


### evs_engine_par is an outlier: 20.7x slower than the field

evs_engine_par (114.72 us) is 20.7x the fastest (5.54 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (5.54 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std} vs {evs_engine, evs_std_par, evs_engine_par} (639% apart)

The field splits into a fast tier {evs_std} and a slow tier {evs_engine, evs_std_par, evs_engine_par} with a 639% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 20.7x the fastest

Fastest evs_std (5.54 us) to slowest evs_engine_par (114.72 us): 20.7x. The strategy choice matters a lot for this workload.


### evs_std_par is inconsistent: worst-20% is 3.6x its best-20%

evs_std_par's best 20% of batches run at 29.32 us but its worst 20% at 105.31 us (3.6x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 5537.4 ns median
- 3 variants significantly slower than baseline
- Spread: 20.72x (fastest 5537.4 ns, slowest 114724.8 ns)

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
| Counter frequency | 2112831643 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 41528ns | 41230ns | 36411ns | 39157ns | 44585ns | +616.70% |
| evs_engine_par | 127331ns | 115182ns | 91442ns | 97081ns | 158991ns | +2097.55% |
| evs_std | 5794ns | 5859ns | 5371ns | 5698ns | 5972ns | base |
| evs_std_par | 69661ns | 55988ns | 29625ns | 34533ns | 106015ns | +1102.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 41209ns | 36147ns | 44225ns | +658.01% | 0.099 |
| evs_engine_par | 126782ns | 91086ns | 158416ns | +2232.09% | 0.032 |
| evs_std | 5436ns | 5102ns | 5548ns | base | 0.753 |
| evs_std_par | 69038ns | 29321ns | 105308ns | +1169.92% | 0.059 |

## Performance model

- Peak throughput: 0.803 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.100 | 12.5% |
| evs_engine_par | 0.036 | 4.4% |
| evs_std | 0.740 | 92.1% |
| evs_std_par | 0.074 | 9.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 41528ns | 41528ns | +616.70% |
| evs_engine_par | 127331ns | 127331ns | +2097.55% |
| evs_std | 5794ns | 5794ns | base |
| evs_std_par | 69661ns | 69661ns | +1102.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 5537ns | base | --- | [5132, 5547] | --- | --- | --- | --- |
| evs_engine | 40894ns | +35576.4ns (+642.5%) | [+31058, +38529]ns | [36591, 44088] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 114725ns | +109379.4ns (+1975.3%) | [+85984, +171724]ns | [91236, 177268] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 54988ns | +49443.4ns (+892.9%) | [+27418, +115728]ns | [32520, 121272] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 5537ns | +584.3% | +1958.4% | +893.0% |
| 1 | 2 | 5338ns | +718.6% | +2049.2% | +1112.4% |

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
| evs_engine | 57323.6ns | 41208.8ns | 139.1% | HIGH |
| evs_engine_par | 56481.2ns | 126782.4ns | 44.5% | HIGH |
| evs_std | 66915.6ns | 5436.4ns | 1230.9% | HIGH |
| evs_std_par | 68226.0ns | 69038.4ns | 98.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 36146.9-44225.3 ns)
  36146.9 |####################
  36550.8 |####################
  36954.7 |
  37358.7 |
  37762.6 |
  38166.5 |
  38570.4 |
  38974.4 |####################
  39378.3 |
  39782.2 |
  40186.1 |
  40590.0 |########################################
  40994.0 |
  41397.9 |
  41801.8 |
  42205.7 |
  42609.7 |
  43013.6 |####################
  43417.5 |
  43821.4 |####################
  (0 below, 1 above range)

evs_engine_par (n=8, range 91085.5-158416.1 ns)
  91085.5 |########################################
  94452.0 |####################
  97818.6 |
  101185.1 |####################
  104551.6 |
  107918.1 |
  111284.7 |
  114651.2 |
  118017.7 |
  121384.3 |
  124750.8 |####################
  128117.3 |####################
  131483.8 |
  134850.4 |
  138216.9 |
  141583.4 |
  144950.0 |
  148316.5 |
  151683.0 |
  155049.5 |
  (0 below, 2 above range)

evs_std (n=8, range 5102.0-5548.1 ns)
   5102.0 |########
   5124.3 |########
   5146.6 |
   5168.9 |
   5191.2 |
   5213.5 |
   5235.8 |
   5258.1 |
   5280.4 |
   5302.7 |
   5325.0 |
   5347.3 |
   5369.6 |
   5391.9 |
   5414.3 |
   5436.6 |
   5458.9 |
   5481.2 |
   5503.5 |
   5525.8 |########################################
  (0 below, 1 above range)

evs_std_par (n=8, range 29320.7-105307.9 ns)
  29320.7 |########################################
  33120.1 |########################################
  36919.4 |
  40718.8 |
  44518.1 |
  48317.5 |
  52116.8 |
  55916.2 |
  59715.6 |
  63514.9 |
  67314.3 |
  71113.6 |####################
  74913.0 |
  78712.3 |
  82511.7 |
  86311.1 |
  90110.4 |
  93909.8 |####################
  97709.1 |
  101508.5 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=137.2% of algo (FFI overhead may distort results)
- evs_engine_par: CV=30.1% (high variance, measurements may be unstable)
- evs_engine_par: bridge=47.9% of algo (FFI overhead may distort results)
- evs_std: bridge=1203.3% of algo (FFI overhead may distort results)
- evs_std_par: CV=57.1% (high variance, measurements may be unstable)
- evs_std_par: worst_20/best_20 = 3.6x (possible bimodal distribution)
- evs_std_par: bridge=124.5% of algo (FFI overhead may distort results)
