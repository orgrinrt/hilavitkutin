# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 189% faster than the next best (evs_engine)

evs_std (20.39 us) leads evs_engine (59.00 us) by 189%, a clear separation rather than a photo finish. CV 15.1%.


### evs_engine_par is an outlier: 13.4x slower than the field

evs_engine_par (273.52 us) is 13.4x the fastest (20.39 us), well off the pack.


### evs_std is fastest but the noisiest (CV 15.1%)

evs_std wins on median (20.39 us) yet has the highest variance (CV 15.1%), while evs_engine is the steadiest (CV 1.6%, 59.00 us).


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (20.39 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 13.4x the fastest

Fastest evs_std (20.39 us) to slowest evs_engine_par (273.52 us): 13.4x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std) is the fastest at 20385.8 ns median
- 3 variants significantly slower than baseline
- Spread: 13.42x (fastest 20385.8 ns, slowest 273518.8 ns)

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
| evs_engine | 59024ns | 59095ns | 57792ns | 58267ns | 59900ns | +164.49% |
| evs_engine_par | 280716ns | 274106ns | 238673ns | 260620ns | 306299ns | +1157.91% |
| evs_std | 22316ns | 20483ns | 20389ns | 20444ns | 24202ns | base |
| evs_std_par | 244724ns | 248057ns | 201092ns | 225814ns | 269815ns | +996.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 58912ns | 57633ns | 59779ns | +165.70% | 1.112 |
| evs_engine_par | 279931ns | 238166ns | 305233ns | +1162.54% | 0.234 |
| evs_std | 22172ns | 20303ns | 24007ns | base | 2.956 |
| evs_std_par | 243571ns | 200323ns | 268241ns | +998.55% | 0.269 |

## Performance model

- Peak throughput: 3.228 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.111 | 34.4% |
| evs_engine_par | 0.240 | 7.4% |
| evs_std | 3.215 | 99.6% |
| evs_std_par | 0.265 | 8.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 59024ns | 59024ns | +164.49% |
| evs_engine_par | 280716ns | 280716ns | +1157.91% |
| evs_std | 22316ns | 22316ns | base |
| evs_std_par | 244724ns | 244724ns | +996.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 20386ns | base | --- | [20319, 27433] | --- | --- | --- | --- |
| evs_engine | 58999ns | +37975.1ns (+186.3%) | [+32232, +39442]ns | [57848, 59746] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 273519ns | +253170.8ns (+1241.9%) | [+219032, +296388]ns | [239384, 316691] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 246901ns | +226536.9ns (+1111.2%) | [+185724, +231560]ns | [206028, 259105] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 20348ns | +193.4% | +1353.3% | +962.8% |
| 1 | 2 | 20524ns | +182.5% | +1137.7% | +1143.7% |

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
| evs_engine | 244799.5ns | 58911.5ns | 415.5% | HIGH |
| evs_engine_par | 555505.0ns | 279931.4ns | 198.4% | HIGH |
| evs_std | 247895.7ns | 22172.2ns | 1118.0% | HIGH |
| evs_std_par | 588260.6ns | 243571.4ns | 241.5% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 57633.0-59778.9 ns)
  57633.0 |########################################
  57740.3 |########################################
  57847.6 |
  57954.9 |
  58062.2 |########################################
  58169.5 |
  58276.8 |
  58384.1 |
  58491.4 |########################################
  58598.7 |
  58705.9 |
  58813.2 |
  58920.5 |
  59027.8 |
  59135.1 |
  59242.4 |
  59349.7 |########################################
  59457.0 |
  59564.3 |########################################
  59671.6 |########################################
  (0 below, 1 above range)

evs_engine_par (n=8, range 238165.6-305232.8 ns)
  238165.6 |########################################
  241519.0 |
  244872.3 |
  248225.7 |
  251579.0 |
  254932.4 |
  258285.8 |
  261639.1 |
  264992.5 |
  268345.8 |####################
  271699.2 |########################################
  275052.6 |
  278405.9 |
  281759.3 |####################
  285112.6 |
  288466.0 |
  291819.4 |
  295172.7 |
  298526.1 |
  301879.4 |
  (0 below, 2 above range)

evs_std (n=8, range 20303.4-24006.6 ns)
  20303.4 |########################################
  20488.6 |########
  20673.7 |
  20858.9 |
  21044.0 |
  21229.2 |
  21414.4 |
  21599.5 |
  21784.7 |
  21969.8 |
  22155.0 |
  22340.1 |
  22525.3 |
  22710.5 |
  22895.6 |
  23080.8 |
  23265.9 |
  23451.1 |
  23636.3 |
  23821.4 |
  (0 below, 2 above range)

evs_std_par (n=8, range 200323.4-268240.9 ns)
  200323.4 |########################################
  203719.3 |########################################
  207115.1 |
  210511.0 |
  213906.9 |
  217302.8 |
  220698.6 |
  224094.5 |########################################
  227490.4 |
  230886.3 |
  234282.1 |
  237678.0 |
  241073.9 |########################################
  244469.8 |
  247865.6 |########################################
  251261.5 |########################################
  254657.4 |
  258053.3 |########################################
  261449.1 |
  264845.0 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=413.6% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=199.2% of algo (FFI overhead may distort results)
- evs_std: bridge=1194.2% of algo (FFI overhead may distort results)
- evs_std_par: bridge=233.7% of algo (FFI overhead may distort results)
