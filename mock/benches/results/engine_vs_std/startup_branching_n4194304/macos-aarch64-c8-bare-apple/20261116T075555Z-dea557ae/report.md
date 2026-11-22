# branching, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 156% faster than the next best (evs_engine_startup)

evs_std_startup (423.82 us) leads evs_engine_startup (1.09 ms) by 156%, a clear separation rather than a photo finish. CV 3.2%.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (423.82 us median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std_startup) is the fastest at 423820.8 ns median
- 1 variant significantly slower than baseline
- Spread: 2.56x (fastest 423820.8 ns, slowest 1085591.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | branching, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010002 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 1132320ns | 1088746ns | 1048275ns | 1063669ns | 1204819ns | +166.10% |
| evs_std_startup | 425519ns | 425933ns | 406383ns | 417867ns | 436042ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 1129841ns | 1046633ns | 1202031ns | +166.80% | 3.712 |
| evs_std_startup | 423474ns | 404075ns | 433971ns | base | 9.905 |

## Performance model

- Peak throughput: 10.380 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 3.864 | 37.2% |
| evs_std_startup | 9.896 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 1132320ns | 1132320ns | +166.10% |
| evs_std_startup | 425519ns | 425519ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 423821ns | base | --- | [410708, 442625] | --- | --- | --- | --- |
| evs_engine_startup | 1085592ns | +675450.0ns (+159.4%) | [+622342, +778442]ns | [1052158, 1203150] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 424500ns | +165.6% |
| 1 | 2 | 418562ns | +159.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine_startup | -0.500 | moderate- |
| evs_std_startup | -0.500 | moderate- |

Consistency summary:

- evs_engine_startup: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine_startup | 25596683.4ns | 1129840.6ns | 2265.5% | HIGH |
| evs_std_startup | 23847166.7ns | 423473.9ns | 5631.3% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 1046633.3-1202031.2 ns)
  1046633.3 |########################################
  1054403.2 |####################
  1062173.1 |
  1069943.0 |####################
  1077712.9 |
  1085482.8 |
  1093252.7 |####################
  1101022.6 |
  1108792.5 |
  1116562.4 |####################
  1124332.3 |
  1132102.2 |
  1139872.1 |
  1147642.0 |
  1155411.9 |
  1163181.8 |
  1170951.7 |
  1178721.6 |
  1186491.5 |
  1194261.4 |
  (0 below, 2 above range)

evs_std_startup (n=8, range 404075.0-433970.8 ns)
  404075.0 |####################
  405569.8 |
  407064.6 |
  408559.4 |
  410054.2 |####################
  411549.0 |
  413043.7 |####################
  414538.5 |
  416033.3 |
  417528.1 |
  419022.9 |
  420517.7 |
  422012.5 |####################
  423507.3 |########################################
  425002.1 |
  426496.9 |
  427991.7 |
  429486.5 |
  430981.2 |
  432476.0 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=2356.1% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=5625.5% of algo (FFI overhead may distort results)
