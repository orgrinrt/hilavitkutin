# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine dominates: 115% faster than the next best (evs_std)

evs_engine (49.26 us) leads evs_std (105.99 us) by 115%, a clear separation rather than a photo finish. CV 98.7%.


### evs_engine_par is an outlier: 4.4x slower than the field

evs_engine_par (218.00 us) is 4.4x the fastest (49.26 us), well off the pack.


### evs_engine is fastest but the noisiest (CV 98.7%)

evs_engine wins on median (49.26 us) yet has the highest variance (CV 98.7%), while evs_std_par is the steadiest (CV 22.9%, 212.92 us).


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Wide spread: slowest is 4.4x the fastest

Fastest evs_engine (49.26 us) to slowest evs_engine_par (218.00 us): 4.4x. The strategy choice matters a lot for this workload.


### evs_std is inconsistent: worst-20% is 3.2x its best-20%

evs_std's best 20% of batches run at 41.31 us but its worst 20% at 130.36 us (3.2x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: evs_engine at 49264.7 ns median (-53.5% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 4.43x (fastest 49264.7 ns, slowest 217997.6 ns)

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
| Counter frequency | 2112761676 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 82379ns | 49344ns | 48929ns | 49101ns | 115699ns | -11.18% |
| evs_engine_par | 235578ns | 218560ns | 163131ns | 198595ns | 281426ns | +154.01% |
| evs_std | 92744ns | 106184ns | 41388ns | 59258ns | 130698ns | base |
| evs_std_par | 207794ns | 214387ns | 144715ns | 177948ns | 245949ns | +124.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 82206ns | 48846ns | 115440ns | -11.15% | 0.050 |
| evs_engine_par | 234978ns | 162739ns | 280699ns | +153.96% | 0.017 |
| evs_std | 92526ns | 41309ns | 130362ns | base | 0.044 |
| evs_std_par | 206296ns | 143751ns | 244080ns | +122.96% | 0.020 |

## Performance model

- Peak throughput: 0.099 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.083 | 83.9% |
| evs_engine_par | 0.019 | 18.9% |
| evs_std | 0.039 | 39.0% |
| evs_std_par | 0.019 | 19.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 82379ns | 82379ns | -11.18% |
| evs_engine_par | 235578ns | 235578ns | +154.01% |
| evs_std | 92744ns | 92744ns | base |
| evs_std_par | 207794ns | 207794ns | +124.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 105988ns | base | --- | [41317, 127962] | --- | --- | --- | --- |
| evs_engine | 49265ns | no significant difference | [-45606, +8091]ns | [48936, 163692] | no | 0.7266 | 0.7266 | 0 |
| evs_engine_par | 217998ns | +145352.5ns (+137.1%) | [+121146, +162849]ns | [186073, 273910] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 212915ns | +101455.9ns (+95.7%) | [+65533, +165336]ns | [158160, 226398] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 80150ns | -38.5% | +196.7% | +165.6% |
| 1 | 2 | 105988ns | -37.2% | +105.6% | +81.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 2/2, lost 0/2
- evs_engine_par: won 0/2, lost 2/2
- evs_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 98057.7ns | 82206.1ns | 119.3% | HIGH |
| evs_engine_par | 219811.5ns | 234978.5ns | 93.5% | HIGH |
| evs_std | 130427.3ns | 92526.1ns | 141.0% | HIGH |
| evs_std_par | 234056.9ns | 206296.4ns | 113.5% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 48846.2-115439.9 ns)
  48846.2 |########################################
  52175.9 |
  55505.6 |
  58835.3 |
  62164.9 |
  65494.6 |
  68824.3 |
  72154.0 |
  75483.7 |
  78813.4 |
  82143.0 |########
  85472.7 |
  88802.4 |
  92132.1 |
  95461.8 |
  98791.5 |
  102121.2 |
  105450.8 |
  108780.5 |
  112110.2 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 162738.9-280699.0 ns)
  162738.9 |####################
  168636.9 |
  174534.9 |
  180432.9 |####################
  186330.9 |
  192228.9 |
  198126.9 |
  204024.9 |########################################
  209923.0 |
  215821.0 |
  221719.0 |
  227617.0 |####################
  233515.0 |
  239413.0 |
  245311.0 |
  251209.0 |
  257107.0 |
  263005.0 |
  268903.0 |########################################
  274801.0 |
  (0 below, 1 above range)

evs_std (n=8, range 41309.2-130362.2 ns)
  41309.2 |########################################
  45761.8 |
  50214.5 |
  54667.1 |
  59119.8 |
  63572.4 |
  68025.1 |
  72477.7 |
  76930.4 |
  81383.0 |
  85835.7 |
  90288.3 |#############
  94741.0 |
  99193.6 |
  103646.3 |
  108098.9 |
  112551.6 |
  117004.2 |##########################
  121456.9 |
  125909.5 |#############
  (0 below, 1 above range)

evs_std_par (n=8, range 143750.9-244079.9 ns)
  143750.9 |####################
  148767.4 |
  153783.8 |####################
  158800.3 |
  163816.7 |####################
  168833.2 |
  173849.6 |
  178866.1 |
  183882.5 |
  188899.0 |
  193915.4 |
  198931.9 |
  203948.3 |####################
  208964.8 |
  213981.2 |
  218997.7 |########################################
  224014.1 |####################
  229030.6 |
  234047.0 |
  239063.5 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: CV=59.1% (high variance, measurements may be unstable)
- evs_engine: bridge=123.3% of algo (FFI overhead may distort results)
- evs_engine_par: CV=23.6% (high variance, measurements may be unstable)
- evs_engine_par: bridge=103.0% of algo (FFI overhead may distort results)
- evs_std: CV=46.2% (high variance, measurements may be unstable)
- evs_std: worst_20/best_20 = 3.2x (possible bimodal distribution)
- evs_std: bridge=132.6% of algo (FFI overhead may distort results)
- evs_std_par: CV=23.7% (high variance, measurements may be unstable)
- evs_std_par: bridge=109.9% of algo (FFI overhead may distort results)
