# branching, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 3913% faster than the next best (evs_engine_startup)

evs_std_startup (222.33 us) leads evs_engine_startup (8.92 ms) by 3913%, a clear separation rather than a photo finish. CV 2.5%.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (222.33 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 40.1x the fastest

Fastest evs_std_startup (222.33 us) to slowest evs_engine_startup (8.92 ms): 40.1x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std_startup) is the fastest at 222331.3 ns median
- 1 variant significantly slower than baseline
- Spread: 40.13x (fastest 222331.3 ns, slowest 8922710.6 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 8943565ns | 8924058ns | 8746498ns | 8855135ns | 9059155ns | +3881.59% |
| evs_std_startup | 224623ns | 224154ns | 216104ns | 220987ns | 229480ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 8941979ns | 8744714ns | 9057614ns | +3914.69% | 0.469 |
| evs_std_startup | 222731ns | 214621ns | 227559ns | base | 18.831 |

## Performance model

- Peak throughput: 19.543 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 0.470 | 2.4% |
| evs_std_startup | 18.865 | 96.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 8943565ns | 8943565ns | +3881.59% |
| evs_std_startup | 224623ns | 224623ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 222331ns | base | --- | [217379, 230631] | --- | --- | --- | --- |
| evs_engine_startup | 8922711ns | +8700901.1ns (+3913.5%) | [+8556698, +8787899]ns | [8774077, 9009467] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 217713ns | +3951.6% |
| 1 | 2 | 224333ns | +3911.7% |

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
| evs_engine_startup | 22015317.4ns | 8941978.8ns | 246.2% | HIGH |
| evs_std_startup | 30570413.3ns | 222731.2ns | 13725.2% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 8744714.4-9057613.9 ns)
  8744714.4 |####################
  8760359.4 |####################
  8776004.4 |
  8791649.3 |
  8807294.3 |
  8822939.3 |
  8838584.3 |
  8854229.2 |####################
  8869874.2 |
  8885519.2 |
  8901164.2 |
  8916809.1 |########################################
  8932454.1 |
  8948099.1 |
  8963744.1 |
  8979389.0 |####################
  8995034.0 |####################
  9010679.0 |
  9026324.0 |
  9041968.9 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 214620.6-227558.7 ns)
  214620.6 |########################################
  215267.5 |
  215914.4 |
  216561.3 |
  217208.2 |########################################
  217855.1 |########################################
  218502.0 |
  219148.9 |
  219795.8 |
  220442.7 |
  221089.6 |########################################
  221736.6 |
  222383.5 |
  223030.4 |########################################
  223677.3 |
  224324.2 |
  224971.1 |########################################
  225618.0 |
  226264.9 |
  226911.8 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=246.8% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=13271.5% of algo (FFI overhead may distort results)
