# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par dominates: 15% faster than the next best (evs_std)

evs_std_par (92.22 us) leads evs_std (106.13 us) by 15%, a clear separation rather than a photo finish. CV 3.5%.


### evs_engine is an outlier: 7.9x slower than the field

evs_engine (726.20 us) is 7.9x the fastest (92.22 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Wide spread: slowest is 7.9x the fastest

Fastest evs_std_par (92.22 us) to slowest evs_engine (726.20 us): 7.9x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 92225.0 ns median (-13.1% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 7.87x (fastest 92225.0 ns, slowest 726195.8 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 727976ns | 727142ns | 714517ns | 722603ns | 735371ns | +576.65% |
| evs_engine_par | 232631ns | 233354ns | 228783ns | 231019ns | 234802ns | +116.23% |
| evs_std | 107585ns | 107504ns | 105750ns | 106647ns | 108748ns | base |
| evs_std_par | 93224ns | 94388ns | 85842ns | 92250ns | 95800ns | -13.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 726971ns | 714283ns | 734104ns | +582.96% | 1.442 |
| evs_engine_par | 231083ns | 226600ns | 233475ns | +117.09% | 4.538 |
| evs_std | 106445ns | 105025ns | 107321ns | base | 9.851 |
| evs_std_par | 91184ns | 84742ns | 93354ns | -14.34% | 11.500 |

## Performance model

- Peak throughput: 12.374 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.444 | 11.7% |
| evs_engine_par | 4.527 | 36.6% |
| evs_std | 9.880 | 79.8% |
| evs_std_par | 11.370 | 91.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 727976ns | 727976ns | +576.65% |
| evs_engine_par | 232631ns | 232631ns | +116.23% |
| evs_std | 107585ns | 107585ns | base |
| evs_std_par | 93224ns | 93224ns | -13.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 106129ns | base | --- | [105242, 107075] | --- | --- | --- | --- |
| evs_engine | 726196ns | +620070.8ns (+584.3%) | [+613683, +630975]ns | [720458, 736000] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 231608ns | +124416.7ns (+117.2%) | [+121742, +128392]ns | [227750, 233825] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 92225ns | -14825.0ns (-14.0%) | [-18067, -12633]ns | [87933, 92475] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 106392ns | +578.9% | +117.0% | -15.8% |
| 1 | 2 | 105746ns | +591.1% | +119.0% | -12.6% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | HIGH- (thermal bounce) |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | HIGH- (thermal bounce) |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 0/2, lost 2/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 5657925.0ns | 726970.8ns | 778.3% | HIGH |
| evs_engine_par | 5670260.4ns | 231083.3ns | 2453.8% | HIGH |
| evs_std | 5665952.1ns | 106444.8ns | 5322.9% | HIGH |
| evs_std_par | 5666058.3ns | 91184.4ns | 6213.8% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 714283.3-734104.2 ns)
  714283.3 |########################################
  715274.3 |
  716265.4 |
  717256.4 |
  718247.5 |
  719238.5 |
  720229.6 |########################################
  721220.6 |########################################
  722211.7 |########################################
  723202.7 |
  724193.7 |
  725184.8 |
  726175.8 |
  727166.9 |
  728157.9 |
  729149.0 |########################################
  730140.0 |
  731131.0 |
  732122.1 |########################################
  733113.1 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 226600.0-233475.0 ns)
  226600.0 |########################################
  226943.8 |
  227287.5 |
  227631.3 |########################################
  227975.0 |
  228318.8 |
  228662.5 |
  229006.3 |########################################
  229350.0 |
  229693.8 |
  230037.5 |
  230381.3 |
  230725.0 |
  231068.8 |########################################
  231412.5 |
  231756.3 |########################################
  232100.0 |
  232443.8 |########################################
  232787.5 |
  233131.3 |
  (0 below, 2 above range)

evs_std (n=8, range 105025.0-107320.8 ns)
  105025.0 |####################
  105139.8 |####################
  105254.6 |
  105369.4 |
  105484.2 |
  105599.0 |
  105713.7 |
  105828.5 |
  105943.3 |########################################
  106058.1 |
  106172.9 |####################
  106287.7 |
  106402.5 |
  106517.3 |
  106632.1 |
  106746.9 |####################
  106861.7 |
  106976.5 |####################
  107091.2 |
  107206.0 |
  (0 below, 1 above range)

evs_std_par (n=8, range 84741.7-93354.2 ns)
  84741.7 |##########
  85172.3 |
  85602.9 |
  86033.6 |
  86464.2 |
  86894.8 |
  87325.4 |
  87756.1 |##########
  88186.7 |
  88617.3 |
  89047.9 |
  89478.6 |
  89909.2 |
  90339.8 |
  90770.4 |##########
  91201.1 |
  91631.7 |
  92062.3 |########################################
  92492.9 |
  92923.6 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=778.3% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=2448.4% of algo (FFI overhead may distort results)
- evs_std: bridge=5322.0% of algo (FFI overhead may distort results)
- evs_std_par: bridge=6139.9% of algo (FFI overhead may distort results)
