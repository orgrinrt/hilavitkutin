# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std dominates: 30% faster than the next best (evs_std_par)

evs_std (398.38 us) leads evs_std_par (516.89 us) by 30%, a clear separation rather than a photo finish. CV 4.2%.


### evs_engine is an outlier: 2.2x slower than the field

evs_engine (870.94 us) is 2.2x the fastest (398.38 us), well off the pack.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (398.38 us median); no rival improves on it (all deltas are >= 0).


### evs_std_par is inconsistent: worst-20% is 1.8x its best-20%

evs_std_par's best 20% of batches run at 354.14 us but its worst 20% at 623.04 us (1.8x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 398384.4 ns median
- 3 variants significantly slower than baseline
- Spread: 2.19x (fastest 398384.4 ns, slowest 870935.3 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 872884ns | 871543ns | 841374ns | 844114ns | 902339ns | +118.26% |
| evs_engine_par | 715270ns | 772240ns | 520554ns | 619917ns | 835464ns | +78.85% |
| evs_std | 399921ns | 399049ns | 382783ns | 383572ns | 416467ns | base |
| evs_std_par | 535082ns | 518674ns | 356338ns | 474910ns | 624898ns | +33.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 872322ns | 840788ns | 901712ns | +118.46% | 1.202 |
| evs_engine_par | 713638ns | 519153ns | 833867ns | +78.72% | 1.469 |
| evs_std | 399300ns | 382058ns | 415946ns | base | 2.626 |
| evs_std_par | 533193ns | 354139ns | 623044ns | +33.53% | 1.967 |

## Performance model

- Peak throughput: 2.961 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.204 | 40.7% |
| evs_engine_par | 1.360 | 45.9% |
| evs_std | 2.632 | 88.9% |
| evs_std_par | 2.029 | 68.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 872884ns | 872884ns | +118.26% |
| evs_engine_par | 715270ns | 715270ns | +78.85% |
| evs_std | 399921ns | 399921ns | base |
| evs_std_par | 535082ns | 535082ns | +33.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 398384ns | base | --- | [382135, 416833] | --- | --- | --- | --- |
| evs_engine | 870935ns | +472550.9ns (+118.6%) | [+424155, +519995]ns | [841074, 902951] | YES | 0.0117 | 0.0078 | 0 |
| evs_engine_par | 770845ns | +371050.5ns (+93.1%) | [+110822, +475840]ns | [527841, 857898] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 516887ns | +117348.5ns (+29.5%) | [+37539, +256439]ns | [450836, 639396] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 382546ns | +135.8% | +117.8% | +48.0% |
| 1 | 2 | 416733ns | +102.1% | +31.6% | +21.9% |

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
| evs_engine | 4106223.7ns | 872322.3ns | 470.7% | HIGH |
| evs_engine_par | 5793190.4ns | 713638.1ns | 811.8% | HIGH |
| evs_std | 4115065.0ns | 399300.3ns | 1030.6% | HIGH |
| evs_std_par | 8975895.5ns | 533192.6ns | 1683.4% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 840788.1-901711.6 ns)
  840788.1 |########################################
  843834.3 |#############
  846880.4 |
  849926.6 |
  852972.8 |
  856019.0 |
  859065.1 |
  862111.3 |
  865157.5 |
  868203.7 |
  871249.8 |
  874296.0 |
  877342.2 |
  880388.4 |
  883434.5 |
  886480.7 |
  889526.9 |
  892573.1 |
  895619.2 |#############
  898665.4 |#############
  (0 below, 2 above range)

evs_engine_par (n=8, range 519153.3-833866.6 ns)
  519153.3 |########################################
  534889.0 |
  550624.6 |
  566360.3 |####################
  582096.0 |
  597831.6 |
  613567.3 |
  629302.9 |
  645038.6 |
  660774.3 |
  676509.9 |
  692245.6 |
  707981.3 |
  723716.9 |
  739452.6 |
  755188.3 |####################
  770923.9 |####################
  786659.6 |
  802395.2 |####################
  818130.9 |
  (0 below, 2 above range)

evs_std (n=8, range 382058.0-415945.6 ns)
  382058.0 |########################################
  383752.4 |
  385446.8 |
  387141.1 |
  388835.5 |
  390529.9 |
  392224.3 |
  393918.7 |
  395613.0 |
  397307.4 |
  399001.8 |
  400696.2 |
  402390.5 |
  404084.9 |
  405779.3 |
  407473.7 |
  409168.1 |
  410862.4 |
  412556.8 |##########
  414251.2 |
  (0 below, 3 above range)

evs_std_par (n=8, range 354138.7-623043.6 ns)
  354138.7 |########################################
  367583.9 |
  381029.2 |
  394474.4 |
  407919.7 |
  421364.9 |
  434810.2 |
  448255.4 |########################################
  461700.7 |
  475145.9 |########################################
  488591.2 |########################################
  502036.4 |
  515481.6 |
  528926.9 |########################################
  542372.1 |
  555817.4 |
  569262.6 |
  582707.9 |
  596153.1 |
  609598.4 |
  (0 below, 3 above range)

```

## Diagnostics

- evs_engine: bridge=471.7% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=853.7% of algo (FFI overhead may distort results)
- evs_std: bridge=1033.0% of algo (FFI overhead may distort results)
- evs_std_par: bridge=1822.5% of algo (FFI overhead may distort results)
