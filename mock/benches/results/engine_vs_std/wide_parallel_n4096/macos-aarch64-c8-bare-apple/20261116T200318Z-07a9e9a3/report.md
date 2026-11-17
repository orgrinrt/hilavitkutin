# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 87% faster than the next best (evs_engine)

evs_std (11.62 us) leads evs_engine (21.70 us) by 87%, a clear separation rather than a photo finish. CV 2.9%.


### evs_engine_par is an outlier: 5.5x slower than the field

evs_engine_par (64.00 us) is 5.5x the fastest (11.62 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (11.62 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 5.5x the fastest

Fastest evs_std (11.62 us) to slowest evs_engine_par (64.00 us): 5.5x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 11625.0 ns median
- 3 variants significantly slower than baseline
- Spread: 5.51x (fastest 11625.0 ns, slowest 63995.8 ns)

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
| evs_engine | 22044ns | 22004ns | 21758ns | 21789ns | 22306ns | +82.81% |
| evs_engine_par | 64441ns | 64438ns | 51508ns | 60117ns | 70917ns | +434.41% |
| evs_std | 12058ns | 11888ns | 11600ns | 11731ns | 12419ns | base |
| evs_std_par | 30653ns | 32392ns | 23917ns | 29322ns | 33335ns | +154.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 21851ns | 21600ns | 22040ns | +86.11% | 0.187 |
| evs_engine_par | 63885ns | 51108ns | 70377ns | +444.14% | 0.064 |
| evs_std | 11741ns | 11492ns | 11931ns | base | 0.349 |
| evs_std_par | 30275ns | 23650ns | 32906ns | +157.87% | 0.135 |

## Performance model

- Peak throughput: 0.356 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.189 | 52.9% |
| evs_engine_par | 0.064 | 18.0% |
| evs_std | 0.352 | 98.9% |
| evs_std_par | 0.128 | 35.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 22044ns | 22044ns | +82.81% |
| evs_engine_par | 64441ns | 64441ns | +434.41% |
| evs_std | 12058ns | 12058ns | base |
| evs_std_par | 30653ns | 30653ns | +154.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 11625ns | base | --- | [11550, 11858] | --- | --- | --- | --- |
| evs_engine | 21704ns | +10183.3ns (+87.6%) | [+9958, +10475]ns | [21667, 22100] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 63996ns | +51962.5ns (+447.0%) | [+46342, +60575]ns | [57983, 72433] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 31967ns | +20370.8ns (+175.2%) | [+12258, +21617]ns | [24117, 33225] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 11646ns | +89.7% | +399.3% | +179.9% |
| 1 | 2 | 11550ns | +87.8% | +471.4% | +171.9% |

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
| evs_engine | 88147.9ns | 21851.0ns | 403.4% | HIGH |
| evs_engine_par | 97584.4ns | 63885.4ns | 152.7% | HIGH |
| evs_std | 88288.6ns | 11740.6ns | 752.0% | HIGH |
| evs_std_par | 94636.5ns | 30275.0ns | 312.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 21600.0-22039.6 ns)
  21600.0 |####################
  21622.0 |
  21644.0 |
  21665.9 |########################################
  21687.9 |########################################
  21709.9 |
  21731.9 |
  21753.9 |
  21775.8 |
  21797.8 |
  21819.8 |
  21841.8 |
  21863.7 |
  21885.7 |
  21907.7 |
  21929.7 |
  21951.7 |
  21973.6 |
  21995.6 |
  22017.6 |
  (0 below, 3 above range)

evs_engine_par (n=8, range 51108.3-70377.1 ns)
  51108.3 |####################
  52071.7 |
  53035.2 |
  53998.6 |
  54962.1 |
  55925.5 |
  56888.9 |
  57852.4 |########################################
  58815.8 |
  59779.2 |
  60742.7 |
  61706.1 |####################
  62669.6 |
  63633.0 |
  64596.4 |
  65559.9 |########################################
  66523.3 |
  67486.8 |
  68450.2 |
  69413.6 |
  (0 below, 2 above range)

evs_std (n=8, range 11491.7-11931.2 ns)
  11491.7 |####################
  11513.7 |
  11535.7 |########################################
  11557.6 |
  11579.6 |
  11601.6 |####################
  11623.6 |####################
  11645.5 |####################
  11667.5 |
  11689.5 |
  11711.5 |
  11733.5 |
  11755.4 |
  11777.4 |
  11799.4 |
  11821.4 |
  11843.3 |####################
  11865.3 |
  11887.3 |
  11909.3 |
  (0 below, 1 above range)

evs_std_par (n=8, range 23650.0-32906.2 ns)
  23650.0 |####################
  24112.8 |####################
  24575.6 |
  25038.4 |
  25501.2 |
  25964.1 |
  26426.9 |
  26889.7 |
  27352.5 |
  27815.3 |
  28278.1 |
  28740.9 |
  29203.8 |
  29666.6 |
  30129.4 |
  30592.2 |####################
  31055.0 |
  31517.8 |########################################
  31980.6 |
  32443.4 |####################
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=405.4% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=153.1% of algo (FFI overhead may distort results)
- evs_std: bridge=756.6% of algo (FFI overhead may distort results)
- evs_std_par: bridge=293.2% of algo (FFI overhead may distort results)
