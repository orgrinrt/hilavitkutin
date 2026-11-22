# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 89% faster than the next best (evs_engine)

evs_std (11.85 us) leads evs_engine (22.41 us) by 89%, a clear separation rather than a photo finish. CV 0.3%.


### evs_engine_par is an outlier: 5.0x slower than the field

evs_engine_par (59.58 us) is 5.0x the fastest (11.85 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (11.85 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 5.0x the fastest

Fastest evs_std (11.85 us) to slowest evs_engine_par (59.58 us): 5.0x. The strategy choice matters a lot for this workload.


### evs_std_par is inconsistent: worst-20% is 1.6x its best-20%

evs_std_par's best 20% of batches run at 20.98 us but its worst 20% at 33.84 us (1.6x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 11850.0 ns median
- 3 variants significantly slower than baseline
- Spread: 5.03x (fastest 11850.0 ns, slowest 59575.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, four independent heavy chains |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000004 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 22826ns | 22521ns | 22133ns | 22283ns | 23406ns | +87.50% |
| evs_engine_par | 60372ns | 60033ns | 53725ns | 58150ns | 63700ns | +395.91% |
| evs_std | 12174ns | 12042ns | 11892ns | 11950ns | 12412ns | base |
| evs_std_par | 28479ns | 29438ns | 21200ns | 22806ns | 34554ns | +133.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 22502ns | 22033ns | 22863ns | +89.92% | 0.182 |
| evs_engine_par | 59956ns | 53350ns | 63300ns | +406.05% | 0.068 |
| evs_std | 11848ns | 11792ns | 11883ns | base | 0.346 |
| evs_std_par | 27923ns | 20983ns | 33844ns | +135.68% | 0.147 |

## Performance model

- Peak throughput: 0.347 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.183 | 52.6% |
| evs_engine_par | 0.069 | 19.8% |
| evs_std | 0.346 | 99.5% |
| evs_std_par | 0.142 | 40.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 22826ns | 22826ns | +87.50% |
| evs_engine_par | 60372ns | 60372ns | +395.91% |
| evs_std | 12174ns | 12174ns | base |
| evs_std_par | 28479ns | 28479ns | +133.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 11850ns | base | --- | [11800, 11900] | --- | --- | --- | --- |
| evs_engine | 22408ns | +10600.0ns (+89.5%) | [+10200, +11208]ns | [22067, 23117] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 59575ns | +47691.7ns (+402.5%) | [+44925, +52400]ns | [56742, 64242] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 28867ns | +17012.5ns (+143.6%) | [+9192, +22100]ns | [21083, 33958] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 11808ns | +89.9% | +385.8% | +81.3% |
| 1 | 2 | 11862ns | +86.0% | +429.6% | +185.8% |

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
| evs_engine | 88836.5ns | 22502.1ns | 394.8% | HIGH |
| evs_engine_par | 93933.3ns | 59956.2ns | 156.7% | HIGH |
| evs_std | 90152.1ns | 11847.9ns | 760.9% | HIGH |
| evs_std_par | 93311.5ns | 27922.9ns | 334.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 22033.3-22862.5 ns)
  22033.3 |########################################
  22074.8 |
  22116.2 |
  22157.7 |
  22199.1 |
  22240.6 |
  22282.1 |
  22323.5 |
  22365.0 |#############
  22406.5 |##########################
  22447.9 |
  22489.4 |
  22530.8 |
  22572.3 |
  22613.8 |
  22655.2 |
  22696.7 |
  22738.1 |
  22779.6 |
  22821.1 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 53350.0-63300.0 ns)
  53350.0 |########################################
  53847.5 |
  54345.0 |
  54842.5 |
  55340.0 |
  55837.5 |
  56335.0 |########################################
  56832.5 |
  57330.0 |
  57827.5 |########################################
  58325.0 |########################################
  58822.5 |
  59320.0 |
  59817.5 |
  60315.0 |########################################
  60812.5 |
  61310.0 |########################################
  61807.5 |
  62305.0 |
  62802.5 |
  (0 below, 2 above range)

evs_std (n=8, range 11791.7-11883.3 ns)
  11791.7 |########################################
  11796.3 |########################################
  11800.9 |
  11805.4 |
  11810.0 |
  11814.6 |########################################
  11819.2 |
  11823.8 |
  11828.4 |
  11832.9 |
  11837.5 |########################################
  11842.1 |
  11846.7 |
  11851.3 |
  11855.8 |########################################
  11860.4 |
  11865.0 |########################################
  11869.6 |
  11874.2 |
  11878.7 |
  (0 below, 2 above range)

evs_std_par (n=8, range 20983.3-33843.8 ns)
  20983.3 |########################################
  21626.3 |####################
  22269.3 |
  22912.4 |
  23555.4 |
  24198.4 |####################
  24841.4 |
  25484.5 |
  26127.5 |
  26770.5 |
  27413.5 |
  28056.5 |
  28699.6 |
  29342.6 |
  29985.6 |
  30628.6 |
  31271.7 |
  31914.7 |
  32557.7 |
  33200.7 |####################
  (0 below, 3 above range)

```

## Diagnostics

- evs_engine: bridge=396.5% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=158.4% of algo (FFI overhead may distort results)
- evs_std: bridge=756.6% of algo (FFI overhead may distort results)
- evs_std_par: CV=21.5% (high variance, measurements may be unstable)
- evs_std_par: bridge=319.4% of algo (FFI overhead may distort results)
