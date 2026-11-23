# element_wise, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 4364% faster than the next best (evs_engine_startup)

evs_std_startup (238.14 us) leads evs_engine_startup (10.63 ms) by 4364%, a clear separation rather than a photo finish. CV 8.8%.


### evs_std_startup is fastest but the noisiest (CV 8.8%)

evs_std_startup wins on median (238.14 us) yet has the highest variance (CV 8.8%), while evs_engine_startup is the steadiest (CV 2.2%, 10.63 ms).


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (238.14 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 44.6x the fastest

Fastest evs_std_startup (238.14 us) to slowest evs_engine_startup (10.63 ms): 44.6x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std_startup) is the fastest at 238137.1 ns median
- 1 variant significantly slower than baseline
- Spread: 44.64x (fastest 238137.1 ns, slowest 10630140.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010001 |
| Counter frequency | 2112803726 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 10718696ns | 10631557ns | 10577070ns | 10600314ns | 10842890ns | +4259.46% |
| evs_std_startup | 245872ns | 240155ns | 223785ns | 234567ns | 259873ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 10717248ns | 10575829ns | 10841318ns | +4291.94% | 0.391 |
| evs_std_startup | 244021ns | 222105ns | 257845ns | base | 17.188 |

## Performance model

- Peak throughput: 18.884 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 0.395 | 2.1% |
| evs_std_startup | 17.613 | 93.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 10718696ns | 10718696ns | +4259.46% |
| evs_std_startup | 245872ns | 245872ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 238137ns | base | --- | [224137, 261105] | --- | --- | --- | --- |
| evs_engine_startup | 10630140ns | +10400625.9ns (+4367.5%) | [+10351692, +10484140]ns | [10579276, 10722793] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 250855ns | +4138.4% |
| 1 | 2 | 237273ns | +4380.1% |

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
| evs_engine_startup | 19643448.1ns | 10717248.5ns | 183.3% | HIGH |
| evs_std_startup | 25209609.3ns | 244021.0ns | 10330.9% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 10575829.0-10841318.5 ns)
  10575829.0 |########################################
  10589103.5 |####################
  10602377.9 |
  10615652.4 |####################
  10628926.9 |
  10642201.4 |####################
  10655475.8 |
  10668750.3 |
  10682024.8 |####################
  10695299.3 |
  10708573.7 |
  10721848.2 |####################
  10735122.7 |
  10748397.2 |
  10761671.6 |
  10774946.1 |
  10788220.6 |
  10801495.1 |
  10814769.5 |
  10828044.0 |
  (0 below, 1 above range)

evs_std_startup (n=8, range 222104.9-257845.1 ns)
  222104.9 |####################
  223891.9 |####################
  225678.9 |
  227465.9 |
  229252.9 |
  231040.0 |
  232827.0 |
  234614.0 |
  236401.0 |########################################
  238188.0 |####################
  239975.0 |####################
  241762.0 |
  243549.0 |
  245336.1 |
  247123.1 |
  248910.1 |
  250697.1 |
  252484.1 |
  254271.1 |
  256058.1 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=184.6% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=10533.0% of algo (FFI overhead may distort results)
