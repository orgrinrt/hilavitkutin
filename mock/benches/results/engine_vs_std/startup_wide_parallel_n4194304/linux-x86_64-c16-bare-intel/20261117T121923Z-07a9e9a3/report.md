# wide_parallel, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 1622% faster than the next best (evs_engine_startup)

evs_std_startup (995.24 us) leads evs_engine_startup (17.13 ms) by 1622%, a clear separation rather than a photo finish. CV 3.3%.


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (995.24 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 17.2x the fastest

Fastest evs_std_startup (995.24 us) to slowest evs_engine_startup (17.13 ms): 17.2x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std_startup) is the fastest at 995238.9 ns median
- 1 variant significantly slower than baseline
- Spread: 17.22x (fastest 995238.9 ns, slowest 17133186.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010004 |
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 17183680ns | 17134731ns | 17069068ns | 17120039ns | 17260064ns | +1631.03% |
| evs_std_startup | 992683ns | 997399ns | 931388ns | 979528ns | 1017873ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 17182179ns | 17066813ns | 17258682ns | +1634.40% | 0.244 |
| evs_std_startup | 990668ns | 929408ns | 1015639ns | base | 4.234 |

## Performance model

- Peak throughput: 4.513 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 0.245 | 5.4% |
| evs_std_startup | 4.214 | 93.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 17183680ns | 17183680ns | +1631.03% |
| evs_std_startup | 992683ns | 992683ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 995239ns | base | --- | [961661, 1021937] | --- | --- | --- | --- |
| evs_engine_startup | 17133186ns | +16130904.4ns (+1620.8%) | [+16087980, +16249949]ns | [17100704, 17243859] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 985859ns | +1643.7% |
| 1 | 2 | 1011983ns | +1592.5% |

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
| evs_engine_startup | 110751686.9ns | 17182178.8ns | 644.6% | HIGH |
| evs_std_startup | 123112462.1ns | 990667.8ns | 12427.2% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 17066813.0-17258682.5 ns)
  17066813.0 |####################
  17076406.5 |
  17085999.9 |
  17095593.4 |####################
  17105186.9 |
  17114780.4 |
  17124373.9 |########################################
  17133967.3 |####################
  17143560.8 |
  17153154.3 |
  17162747.8 |
  17172341.2 |####################
  17181934.7 |
  17191528.2 |
  17201121.6 |
  17210715.1 |
  17220308.6 |
  17229902.1 |
  17239495.6 |####################
  17249089.0 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 929407.9-1015638.8 ns)
  929407.9 |########################################
  933719.4 |
  938031.0 |
  942342.5 |
  946654.1 |
  950965.6 |
  955277.2 |
  959588.7 |########################################
  963900.2 |
  968211.8 |
  972523.3 |
  976834.9 |########################################
  981146.4 |
  985458.0 |
  989769.5 |########################################
  994081.0 |########################################
  998392.6 |########################################
  1002704.1 |
  1007015.7 |
  1011327.2 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=645.2% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=12238.0% of algo (FFI overhead may distort results)
