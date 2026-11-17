# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par beats baseline by 71% (significant)

evs_std_par is -2.08 ms (71%) faster than baseline evs_std, with a CI that excludes zero.


### evs_engine is an outlier: 3.4x slower than the field

evs_engine (2.97 ms) is 3.4x the fastest (862.84 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_std, evs_engine} (230% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_std, evs_engine} with a 230% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 3.4x the fastest

Fastest evs_std_par (862.84 us) to slowest evs_engine (2.97 ms): 3.4x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 862837.5 ns median (-70.7% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.44x (fastest 862837.5 ns, slowest 2965662.5 ns)

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
| evs_engine | 2968877ns | 2969688ns | 2960808ns | 2967117ns | 2972215ns | +0.40% |
| evs_engine_par | 917841ns | 898533ns | 890258ns | 897442ns | 940035ns | -68.96% |
| evs_std | 2957127ns | 2948271ns | 2942767ns | 2945628ns | 2969342ns | base |
| evs_std_par | 867327ns | 866783ns | 864050ns | 865806ns | 869288ns | -70.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 2965114ns | 2958750ns | 2968142ns | +0.36% | 0.354 |
| evs_engine_par | 912991ns | 887342ns | 935406ns | -69.10% | 1.149 |
| evs_std | 2954565ns | 2940992ns | 2966521ns | base | 0.355 |
| evs_std_par | 862490ns | 858683ns | 864546ns | -70.81% | 1.216 |

## Performance model

- Peak throughput: 1.221 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.354 | 29.0% |
| evs_engine_par | 1.174 | 96.2% |
| evs_std | 0.356 | 29.1% |
| evs_std_par | 1.215 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 2968877ns | 2968877ns | +0.40% |
| evs_engine_par | 917841ns | 917841ns | -68.96% |
| evs_std | 2957127ns | 2957127ns | base |
| evs_std_par | 867327ns | 867327ns | -70.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 2946179ns | base | --- | [2941342, 2947550] | --- | --- | --- | --- |
| evs_engine | 2965662ns | +18916.6ns (+0.6%) | [+17408, +24217]ns | [2961333, 2968558] | YES (adj: no) | 0.0703 | 0.0703 | 0 |
| evs_engine_par | 892942ns | -2051237.6ns (-69.6%) | [-2055250, -2000583]ns | [890733, 941983] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 862838ns | -2083837.5ns (-70.7%) | [-2084950, -2077850]ns | [859875, 864717] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 2947188ns | +0.6% | -68.9% | -70.7% |
| 1 | 2 | 2943438ns | +0.8% | -69.7% | -70.7% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | HIGH- (thermal bounce) |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 22787268.7ns | 2965113.5ns | 768.5% | HIGH |
| evs_engine_par | 22838303.1ns | 912990.6ns | 2501.5% | HIGH |
| evs_std | 22813955.2ns | 2954564.6ns | 772.2% | HIGH |
| evs_std_par | 22797934.4ns | 862489.6ns | 2643.3% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 2958750.0-2968141.6 ns)
  2958750.0 |########################################
  2959219.6 |
  2959689.2 |
  2960158.7 |
  2960628.3 |
  2961097.9 |########################################
  2961567.5 |
  2962037.1 |
  2962506.7 |
  2962976.2 |########################################
  2963445.8 |
  2963915.4 |
  2964385.0 |
  2964854.6 |########################################
  2965324.2 |
  2965793.7 |
  2966263.3 |########################################
  2966732.9 |
  2967202.5 |
  2967672.1 |########################################
  (0 below, 2 above range)

evs_engine_par (n=8, range 887341.7-935406.2 ns)
  887341.7 |####################
  889744.9 |########################################
  892148.2 |########################################
  894551.4 |####################
  896954.6 |
  899357.8 |
  901761.1 |
  904164.3 |
  906567.5 |
  908970.7 |
  911374.0 |
  913777.2 |
  916180.4 |
  918583.6 |
  920986.9 |
  923390.1 |
  925793.3 |
  928196.5 |
  930599.8 |
  933003.0 |
  (0 below, 2 above range)

evs_std (n=8, range 2940991.7-2966520.8 ns)
  2940991.7 |########################################
  2942268.2 |####################
  2943544.6 |
  2944821.1 |####################
  2946097.5 |########################################
  2947374.0 |####################
  2948650.4 |
  2949926.9 |
  2951203.4 |
  2952479.8 |
  2953756.3 |
  2955032.7 |
  2956309.2 |
  2957585.6 |
  2958862.1 |
  2960138.6 |
  2961415.0 |
  2962691.5 |
  2963967.9 |
  2965244.4 |
  (0 below, 1 above range)

evs_std_par (n=8, range 858683.3-864545.8 ns)
  858683.3 |########################################
  858976.4 |
  859269.6 |
  859562.7 |
  859855.8 |########################################
  860148.9 |
  860442.1 |########################################
  860735.2 |
  861028.3 |
  861321.4 |
  861614.6 |
  861907.7 |
  862200.8 |
  862493.9 |########################################
  862787.1 |
  863080.2 |########################################
  863373.3 |
  863666.4 |########################################
  863959.6 |
  864252.7 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=769.0% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=2558.8% of algo (FFI overhead may distort results)
- evs_std: bridge=774.2% of algo (FFI overhead may distort results)
- evs_std_par: bridge=2645.0% of algo (FFI overhead may distort results)
