# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine is an outlier: 2.2x slower than the field

evs_engine (1.23 ms) is 2.2x the fastest (550.40 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (550.40 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std, evs_std_par} vs {evs_engine_par, evs_engine} (72% apart)

The field splits into a fast tier {evs_std, evs_std_par} and a slow tier {evs_engine_par, evs_engine} with a 72% jump between them - a qualitative difference, not a gradient.


## Key findings

- Baseline (evs_std) is the fastest at 550400.2 ns median
- 2 variants significantly slower than baseline
- Spread: 2.24x (fastest 550400.2 ns, slowest 1234044.4 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 1248960ns | 1236091ns | 1142006ns | 1191260ns | 1318974ns | +122.54% |
| evs_engine_par | 968259ns | 967289ns | 910735ns | 955156ns | 992468ns | +72.52% |
| evs_std | 561238ns | 553028ns | 508515ns | 523740ns | 602543ns | base |
| evs_std_par | 570111ns | 563572ns | 498897ns | 536772ns | 612918ns | +1.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 1246063ns | 1140294ns | 1315167ns | +122.99% | 0.842 |
| evs_engine_par | 965838ns | 908407ns | 990095ns | +72.84% | 1.086 |
| evs_std | 558803ns | 506016ns | 600165ns | base | 1.876 |
| evs_std_par | 567361ns | 496235ns | 610206ns | +1.53% | 1.848 |

## Performance model

- Peak throughput: 2.113 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.850 | 40.2% |
| evs_engine_par | 1.087 | 51.4% |
| evs_std | 1.905 | 90.2% |
| evs_std_par | 1.869 | 88.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 1248960ns | 1248960ns | +122.54% |
| evs_engine_par | 968259ns | 968259ns | +72.52% |
| evs_std | 561238ns | 561238ns | base |
| evs_std_par | 570111ns | 570111ns | +1.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 550400ns | base | --- | [509748, 617999] | --- | --- | --- | --- |
| evs_engine | 1234044ns | +716276.9ns (+130.1%) | [+551838, +778133]ns | [1159640, 1306344] | YES | 0.0117 | 0.0078 | 0 |
| evs_engine_par | 964901ns | +392502.4ns (+71.3%) | [+362093, +436351]ns | [942471, 980091] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 561185ns | no significant difference | [-12657, +28950]ns | [525488, 627612] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 517768ns | +142.1% | +85.0% | +3.5% |
| 1 | 2 | 603227ns | +93.5% | +60.5% | -0.3% |

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
- evs_std_par: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 5604089.3ns | 1246063.0ns | 449.7% | HIGH |
| evs_engine_par | 5552242.5ns | 965838.1ns | 574.9% | HIGH |
| evs_std | 5281572.1ns | 558802.6ns | 945.2% | HIGH |
| evs_std_par | 5676869.9ns | 567361.1ns | 1000.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 1140294.5-1315166.9 ns)
  1140294.5 |####################
  1149038.1 |
  1157781.7 |####################
  1166525.4 |####################
  1175269.0 |
  1184012.6 |
  1192756.2 |
  1201499.9 |
  1210243.5 |
  1218987.1 |
  1227730.7 |########################################
  1236474.3 |
  1245218.0 |
  1253961.6 |
  1262705.2 |
  1271448.8 |####################
  1280192.5 |
  1288936.1 |
  1297679.7 |####################
  1306423.3 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 908407.1-990094.7 ns)
  908407.1 |####################
  912491.5 |
  916575.9 |
  920660.2 |
  924744.6 |
  928829.0 |
  932913.4 |
  936997.7 |
  941082.1 |####################
  945166.5 |
  949250.9 |####################
  953335.3 |
  957419.6 |
  961504.0 |########################################
  965588.4 |
  969672.8 |####################
  973757.1 |
  977841.5 |####################
  981925.9 |
  986010.3 |
  (0 below, 1 above range)

evs_std (n=8, range 506015.7-600164.8 ns)
  506015.7 |########################################
  510723.2 |
  515430.6 |
  520138.1 |
  524845.5 |########################################
  529553.0 |
  534260.4 |
  538967.9 |
  543675.4 |
  548382.8 |
  553090.3 |
  557797.7 |
  562505.2 |
  567212.6 |
  571920.1 |####################
  576627.6 |
  581335.0 |
  586042.5 |####################
  590749.9 |
  595457.4 |
  (0 below, 2 above range)

evs_std_par (n=8, range 496234.6-610205.8 ns)
  496234.6 |####################
  501933.2 |
  507631.7 |
  513330.3 |
  519028.8 |
  524727.4 |########################################
  530426.0 |
  536124.5 |
  541823.1 |####################
  547521.6 |
  553220.2 |
  558918.7 |
  564617.3 |
  570315.9 |####################
  576014.4 |
  581713.0 |####################
  587411.5 |
  593110.1 |
  598808.7 |
  604507.2 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=455.3% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=568.1% of algo (FFI overhead may distort results)
- evs_std: bridge=941.7% of algo (FFI overhead may distort results)
- evs_std_par: bridge=1010.9% of algo (FFI overhead may distort results)
