# element_wise, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup dominates: 4464% faster than the next best (evs_engine_startup)

evs_std_startup (236.66 us) leads evs_engine_startup (10.80 ms) by 4464%, a clear separation rather than a photo finish. CV 10.2%.


### evs_std_startup is fastest but the noisiest (CV 10.2%)

evs_std_startup wins on median (236.66 us) yet has the highest variance (CV 10.2%), while evs_engine_startup is the steadiest (CV 1.8%, 10.80 ms).


### evs_engine_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_engine_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (236.66 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 45.6x the fastest

Fastest evs_std_startup (236.66 us) to slowest evs_engine_startup (10.80 ms): 45.6x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (evs_std_startup) is the fastest at 236660.9 ns median
- 1 variant significantly slower than baseline
- Spread: 45.64x (fastest 236660.9 ns, slowest 10800461.4 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 10845937ns | 10801973ns | 10659569ns | 10718854ns | 10987841ns | +4302.59% |
| evs_std_startup | 246354ns | 238173ns | 221373ns | 233245ns | 262430ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 10844368ns | 10658149ns | 10986246ns | +4331.35% | 0.387 |
| evs_std_startup | 244719ns | 219923ns | 260770ns | base | 17.139 |

## Performance model

- Peak throughput: 19.072 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 0.388 | 2.0% |
| evs_std_startup | 17.723 | 92.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 10845937ns | 10845937ns | +4302.59% |
| evs_std_startup | 246354ns | 246354ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 236661ns | base | --- | [228409, 257112] | --- | --- | --- | --- |
| evs_engine_startup | 10800461ns | +10549341.7ns (+4457.6%) | [+10438226, +10853509]ns | [10677119, 11081918] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 251120ns | +4200.9% |
| 1 | 2 | 233171ns | +4507.9% |

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
| evs_engine_startup | 19778052.1ns | 10844367.9ns | 182.4% | HIGH |
| evs_std_startup | 25722519.0ns | 244719.4ns | 10511.0% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 10658149.1-10986245.9 ns)
  10658149.1 |####################
  10674553.9 |########################################
  10690958.8 |
  10707363.6 |
  10723768.5 |
  10740173.3 |
  10756578.1 |
  10772983.0 |
  10789387.8 |####################
  10805792.7 |########################################
  10822197.5 |
  10838602.3 |
  10855007.2 |
  10871412.0 |
  10887816.9 |
  10904221.7 |
  10920626.5 |
  10937031.4 |
  10953436.2 |
  10969841.1 |
  (0 below, 2 above range)

evs_std_startup (n=8, range 219922.6-260770.2 ns)
  219922.6 |########################################
  221965.0 |
  224007.4 |
  226049.7 |
  228092.1 |########################################
  230134.5 |########################################
  232176.9 |
  234219.3 |########################################
  236261.6 |
  238304.0 |########################################
  240346.4 |
  242388.8 |
  244431.2 |########################################
  246473.5 |
  248515.9 |
  250558.3 |
  252600.7 |
  254643.1 |
  256685.4 |########################################
  258727.8 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine_startup: bridge=182.3% of algo (FFI overhead may distort results)
- evs_std_startup: bridge=10861.5% of algo (FFI overhead may distort results)
