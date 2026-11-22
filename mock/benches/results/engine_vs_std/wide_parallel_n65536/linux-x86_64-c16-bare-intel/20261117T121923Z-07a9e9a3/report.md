# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine_par is an outlier: 2.3x slower than the field

evs_engine_par (1.45 ms) is 2.3x the fastest (623.09 us), well off the pack.


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine, evs_std} vs {evs_engine_par} (117% apart)

The field splits into a fast tier {evs_std_par, evs_engine, evs_std} and a slow tier {evs_engine_par} with a 117% jump between them - a qualitative difference, not a gradient.


### Speed leader evs_std_par vs stability leader evs_std (+7% speed for 41.4x steadier)

evs_std_par is fastest (623.09 us, CV 5.4%); evs_std gives up 7.2% median for 41.4x lower variance (CV 0.1%).


### evs_engine_par is inconsistent: worst-20% is 2.8x its best-20%

evs_engine_par's best 20% of batches run at 668.14 us but its worst 20% at 1.86 ms (2.8x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: evs_std_par at 623087.6 ns median (-6.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.32x (fastest 623087.6 ns, slowest 1447133.6 ns)

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
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 665964ns | 666210ns | 655815ns | 659030ns | 673703ns | -0.30% |
| evs_engine_par | 1411046ns | 1449057ns | 668691ns | 1053890ns | 1864502ns | +111.24% |
| evs_std | 667967ns | 667989ns | 666378ns | 667564ns | 668666ns | base |
| evs_std_par | 624665ns | 623728ns | 566085ns | 609939ns | 650354ns | -6.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 665733ns | 655619ns | 673456ns | -0.30% | 0.098 |
| evs_engine_par | 1409427ns | 668142ns | 1862383ns | +111.07% | 0.046 |
| evs_std | 667764ns | 666214ns | 668462ns | base | 0.098 |
| evs_std_par | 624035ns | 565562ns | 649683ns | -6.55% | 0.105 |

## Performance model

- Peak throughput: 0.116 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.098 | 84.9% |
| evs_engine_par | 0.045 | 39.1% |
| evs_std | 0.098 | 84.7% |
| evs_std_par | 0.105 | 90.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 665964ns | 665964ns | -0.30% |
| evs_engine_par | 1411046ns | 1411046ns | +111.24% |
| evs_std | 667967ns | 667967ns | base |
| evs_std_par | 624665ns | 624665ns | -6.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 667758ns | base | --- | [666923, 668739] | --- | --- | --- | --- |
| evs_engine | 665931ns | no significant difference | [-9690, +6223]ns | [658118, 674060] | no | 1.0000 | 1.0000 | 0 |
| evs_engine_par | 1447134ns | +780565.2ns (+116.9%) | [+48239, +1273007]ns | [715655, 1940716] | YES (adj: no) | 0.2109 | 0.0703 | 0 |
| evs_std_par | 623088ns | no significant difference | [-71761, +829]ns | [597268, 668636] | no | 0.4336 | 0.2891 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 667758ns | -1.4% | +82.9% | -7.8% |
| 1 | 2 | 667844ns | +0.8% | +150.6% | -6.1% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | HIGH- (thermal bounce) |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 1/2
- evs_engine_par: won 0/2, lost 2/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 986030.4ns | 665732.8ns | 148.1% | HIGH |
| evs_engine_par | 3307455.9ns | 1409427.4ns | 234.7% | HIGH |
| evs_std | 985780.3ns | 667763.6ns | 147.6% | HIGH |
| evs_std_par | 1022213.2ns | 624035.4ns | 163.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 655619.0-673456.4 ns)
  655619.0 |####################
  656510.9 |
  657402.7 |########################################
  658294.6 |
  659186.5 |####################
  660078.3 |
  660970.2 |
  661862.1 |
  662753.9 |
  663645.8 |
  664537.7 |
  665429.6 |
  666321.4 |
  667213.3 |
  668105.2 |
  668997.0 |
  669888.9 |
  670780.8 |
  671672.6 |########################################
  672564.5 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 668141.6-1862383.0 ns)
  668141.6 |########################################
  727853.7 |
  787565.7 |
  847277.8 |
  906989.9 |
  966702.0 |
  1026414.0 |
  1086126.1 |####################
  1145838.2 |
  1205550.3 |
  1265262.3 |####################
  1324974.4 |
  1384686.5 |
  1444398.5 |
  1504110.6 |
  1563822.7 |####################
  1623534.8 |
  1683246.8 |
  1742958.9 |####################
  1802671.0 |
  (0 below, 2 above range)

evs_std (n=8, range 666214.1-668461.6 ns)
  666214.1 |########################################
  666326.5 |
  666438.8 |
  666551.2 |
  666663.6 |
  666776.0 |
  666888.3 |########################################
  667000.7 |
  667113.1 |
  667225.5 |
  667337.8 |########################################
  667450.2 |
  667562.6 |
  667675.0 |########################################
  667787.3 |########################################
  667899.7 |
  668012.1 |
  668124.5 |
  668236.8 |########################################
  668349.2 |
  (0 below, 2 above range)

evs_std_par (n=8, range 565562.2-649682.8 ns)
  565562.2 |########################################
  569768.2 |
  573974.3 |
  578180.3 |
  582386.3 |
  586592.3 |
  590798.4 |
  595004.4 |########################################
  599210.4 |
  603416.5 |
  607622.5 |########################################
  611828.5 |
  616034.5 |
  620240.6 |########################################
  624446.6 |########################################
  628652.6 |########################################
  632858.7 |
  637064.7 |
  641270.7 |
  645476.7 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=148.1% of algo (FFI overhead may distort results)
- evs_engine_par: CV=36.6% (high variance, measurements may be unstable)
- evs_engine_par: bridge=218.0% of algo (FFI overhead may distort results)
- evs_std: bridge=147.6% of algo (FFI overhead may distort results)
- evs_std_par: bridge=160.7% of algo (FFI overhead may distort results)
