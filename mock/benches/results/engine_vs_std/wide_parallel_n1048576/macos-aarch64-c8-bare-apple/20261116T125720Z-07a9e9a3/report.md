# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par beats baseline by 71% (significant)

evs_std_par is -2.08 ms (71%) faster than baseline evs_std, with a CI that excludes zero.


### evs_engine is an outlier: 3.4x slower than the field

evs_engine (2.96 ms) is 3.4x the fastest (862.24 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_std, evs_engine} (235% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_std, evs_engine} with a 235% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 3.4x the fastest

Fastest evs_std_par (862.24 us) to slowest evs_engine (2.96 ms): 3.4x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 862241.7 ns median (-70.7% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.43x (fastest 862241.7 ns, slowest 2960162.5 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 2963114ns | 2962067ns | 2958367ns | 2960867ns | 2965985ns | +0.27% |
| evs_engine_par | 887671ns | 884479ns | 878642ns | 881817ns | 894319ns | -69.96% |
| evs_std | 2955072ns | 2950283ns | 2944300ns | 2948356ns | 2962802ns | base |
| evs_std_par | 866496ns | 866162ns | 861000ns | 864531ns | 869344ns | -70.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 2961024ns | 2957133ns | 2963258ns | +0.30% | 0.354 |
| evs_engine_par | 884249ns | 875442ns | 890579ns | -70.05% | 1.186 |
| evs_std | 2952106ns | 2942658ns | 2959852ns | base | 0.355 |
| evs_std_par | 862469ns | 856800ns | 865067ns | -70.78% | 1.216 |

## Performance model

- Peak throughput: 1.224 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.354 | 28.9% |
| evs_engine_par | 1.191 | 97.3% |
| evs_std | 0.356 | 29.1% |
| evs_std_par | 1.216 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 2963114ns | 2963114ns | +0.27% |
| evs_engine_par | 887671ns | 887671ns | -69.96% |
| evs_std | 2955072ns | 2955072ns | base |
| evs_std_par | 866496ns | 866496ns | -70.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 2947375ns | base | --- | [2944250, 2952442] | --- | --- | --- | --- |
| evs_engine | 2960162ns | +13158.3ns (+0.4%) | [+9458, +17200]ns | [2958400, 2963958] | YES (adj: no) | 0.0703 | 0.0703 | 0 |
| evs_engine_par | 880454ns | -2068329.2ns (-70.2%) | [-2073592, -2048958]ns | [877417, 897000] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 862242ns | -2084804.1ns (-70.7%) | [-2090892, -2081958]ns | [860117, 866925] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 2947375ns | +0.5% | -69.8% | -70.7% |
| 1 | 2 | 2947792ns | +0.4% | -70.2% | -70.8% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 22780963.5ns | 2961024.0ns | 769.4% | HIGH |
| evs_engine_par | 22836788.6ns | 884249.0ns | 2582.6% | HIGH |
| evs_std | 22804662.5ns | 2952106.2ns | 772.5% | HIGH |
| evs_std_par | 22843663.5ns | 862468.8ns | 2648.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 2957133.3-2963258.3 ns)
  2957133.3 |####################
  2957439.6 |
  2957745.8 |
  2958052.1 |
  2958358.3 |####################
  2958664.6 |
  2958970.8 |
  2959277.1 |
  2959583.3 |########################################
  2959889.6 |
  2960195.8 |####################
  2960502.1 |
  2960808.3 |
  2961114.6 |
  2961420.8 |
  2961727.1 |
  2962033.3 |
  2962339.6 |
  2962645.8 |####################
  2962952.1 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 875441.7-890579.2 ns)
  875441.7 |########################################
  876198.6 |
  876955.4 |########################################
  877712.3 |
  878469.2 |########################################
  879226.1 |########################################
  879982.9 |
  880739.8 |########################################
  881496.7 |
  882253.6 |
  883010.4 |
  883767.3 |
  884524.2 |
  885281.1 |
  886037.9 |########################################
  886794.8 |
  887551.7 |
  888308.6 |
  889065.4 |
  889822.3 |
  (0 below, 2 above range)

evs_std (n=8, range 2942658.3-2959852.1 ns)
  2942658.3 |########################################
  2943518.0 |########################################
  2944377.7 |########################################
  2945237.4 |########################################
  2946097.1 |
  2946956.8 |
  2947816.4 |
  2948676.1 |########################################
  2949535.8 |
  2950395.5 |########################################
  2951255.2 |
  2952114.9 |########################################
  2952974.6 |
  2953834.3 |
  2954694.0 |
  2955553.6 |
  2956413.3 |
  2957273.0 |
  2958132.7 |
  2958992.4 |
  (0 below, 1 above range)

evs_std_par (n=8, range 856800.0-865066.7 ns)
  856800.0 |########################################
  857213.3 |
  857626.7 |
  858040.0 |
  858453.3 |
  858866.7 |
  859280.0 |
  859693.3 |
  860106.7 |########################################
  860520.0 |########################################
  860933.3 |
  861346.7 |
  861760.0 |########################################
  862173.3 |
  862586.7 |########################################
  863000.0 |########################################
  863413.3 |
  863826.7 |
  864240.0 |
  864653.3 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=769.6% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=2593.6% of algo (FFI overhead may distort results)
- evs_std: bridge=774.4% of algo (FFI overhead may distort results)
- evs_std_par: bridge=2648.8% of algo (FFI overhead may distort results)
