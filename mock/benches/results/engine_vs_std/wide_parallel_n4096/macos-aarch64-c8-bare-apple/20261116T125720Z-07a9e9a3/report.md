# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 90% faster than the next best (evs_engine)

evs_std (11.63 us) leads evs_engine (22.13 us) by 90%, a clear separation rather than a photo finish. CV 1.1%.


### evs_engine_par is an outlier: 5.8x slower than the field

evs_engine_par (67.06 us) is 5.8x the fastest (11.63 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (11.63 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 5.8x the fastest

Fastest evs_std (11.63 us) to slowest evs_engine_par (67.06 us): 5.8x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 11629.1 ns median
- 3 variants significantly slower than baseline
- Spread: 5.77x (fastest 11629.1 ns, slowest 67058.4 ns)

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
| evs_engine | 22596ns | 22221ns | 21858ns | 22194ns | 23081ns | +89.91% |
| evs_engine_par | 66698ns | 67475ns | 52175ns | 61281ns | 74392ns | +460.59% |
| evs_std | 11898ns | 11879ns | 11592ns | 11692ns | 12129ns | base |
| evs_std_par | 31486ns | 31438ns | 29225ns | 30797ns | 32569ns | +164.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 22484ns | 21758ns | 22965ns | +93.12% | 0.182 |
| evs_engine_par | 66278ns | 51700ns | 73979ns | +469.27% | 0.062 |
| evs_std | 11643ns | 11508ns | 11769ns | base | 0.352 |
| evs_std_par | 31127ns | 29092ns | 32056ns | +167.35% | 0.132 |

## Performance model

- Peak throughput: 0.356 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.185 | 52.0% |
| evs_engine_par | 0.061 | 17.2% |
| evs_std | 0.352 | 99.0% |
| evs_std_par | 0.132 | 37.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 22596ns | 22596ns | +89.91% |
| evs_engine_par | 66698ns | 66698ns | +460.59% |
| evs_std | 11898ns | 11898ns | base |
| evs_std_par | 31486ns | 31486ns | +164.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 11629ns | base | --- | [11517, 11792] | --- | --- | --- | --- |
| evs_engine | 22129ns | +10458.3ns (+89.9%) | [+10300, +12117]ns | [22042, 23633] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 67058ns | +55395.8ns (+476.4%) | [+43267, +60850]ns | [55008, 72358] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 30942ns | +19425.0ns (+167.0%) | [+18508, +20692]ns | [30317, 32425] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 11767ns | +87.9% | +419.4% | +166.6% |
| 1 | 2 | 11517ns | +100.0% | +499.9% | +168.7% |

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
| evs_engine | 89536.5ns | 22484.4ns | 398.2% | HIGH |
| evs_engine_par | 98274.0ns | 66278.1ns | 148.3% | HIGH |
| evs_std | 89023.9ns | 11642.7ns | 764.6% | HIGH |
| evs_std_par | 93272.9ns | 31127.1ns | 299.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 21758.3-22964.6 ns)
  21758.3 |####################
  21818.6 |
  21878.9 |
  21939.2 |
  21999.6 |####################
  22059.9 |####################
  22120.2 |########################################
  22180.5 |
  22240.8 |
  22301.1 |
  22361.4 |
  22421.8 |####################
  22482.1 |
  22542.4 |
  22602.7 |
  22663.0 |
  22723.3 |
  22783.6 |
  22843.9 |
  22904.3 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 51700.0-73979.2 ns)
  51700.0 |####################
  52814.0 |
  53927.9 |####################
  55041.9 |
  56155.8 |
  57269.8 |
  58383.8 |
  59497.7 |
  60611.7 |####################
  61725.6 |
  62839.6 |
  63953.5 |
  65067.5 |
  66181.5 |########################################
  67295.4 |
  68409.4 |
  69523.3 |
  70637.3 |####################
  71751.3 |####################
  72865.2 |
  (0 below, 1 above range)

evs_std (n=8, range 11508.3-11768.8 ns)
  11508.3 |########################################
  11521.3 |#############
  11534.3 |
  11547.4 |
  11560.4 |
  11573.4 |
  11586.4 |
  11599.5 |
  11612.5 |
  11625.5 |
  11638.5 |
  11651.5 |
  11664.6 |
  11677.6 |
  11690.6 |
  11703.6 |
  11716.7 |
  11729.7 |##########################
  11742.7 |
  11755.7 |
  (0 below, 2 above range)

evs_std_par (n=8, range 29091.7-32056.2 ns)
  29091.7 |####################
  29239.9 |
  29388.2 |
  29536.4 |
  29684.6 |
  29832.8 |
  29981.1 |
  30129.3 |
  30277.5 |####################
  30425.7 |####################
  30574.0 |
  30722.2 |
  30870.4 |########################################
  31018.7 |
  31166.9 |
  31315.1 |
  31463.3 |
  31611.6 |
  31759.8 |
  31908.0 |
  (0 below, 3 above range)

```

## Diagnostics

- evs_engine: bridge=404.8% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=146.8% of algo (FFI overhead may distort results)
- evs_std: bridge=767.2% of algo (FFI overhead may distort results)
- evs_std_par: bridge=299.7% of algo (FFI overhead may distort results)
