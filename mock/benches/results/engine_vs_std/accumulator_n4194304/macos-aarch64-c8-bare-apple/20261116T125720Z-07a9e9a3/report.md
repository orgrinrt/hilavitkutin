# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par dominates: 12% faster than the next best (evs_std)

evs_std_par (528.71 us) leads evs_std (589.52 us) by 12%, a clear separation rather than a photo finish. CV 1.6%.


### evs_engine is an outlier: 5.3x slower than the field

evs_engine (2.82 ms) is 5.3x the fastest (528.71 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_std, evs_engine_par} vs {evs_engine} (293% apart)

The field splits into a fast tier {evs_std_par, evs_std, evs_engine_par} and a slow tier {evs_engine} with a 293% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.3x the fastest

Fastest evs_std_par (528.71 us) to slowest evs_engine (2.82 ms): 5.3x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 528712.5 ns median (-10.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.33x (fastest 528712.5 ns, slowest 2817412.5 ns)

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
| evs_engine | 2823791ns | 2820938ns | 2790683ns | 2804561ns | 2846490ns | +367.04% |
| evs_engine_par | 722126ns | 721862ns | 704625ns | 710142ns | 735490ns | +19.44% |
| evs_std | 604609ns | 590850ns | 572458ns | 584706ns | 627575ns | base |
| evs_std_par | 535635ns | 532288ns | 526200ns | 529208ns | 542815ns | -11.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 2820036ns | 2788583ns | 2842656ns | +367.93% | 1.487 |
| evs_engine_par | 717783ns | 699608ns | 731194ns | +19.10% | 5.843 |
| evs_std | 602668ns | 570483ns | 625221ns | base | 6.960 |
| evs_std_par | 532229ns | 522758ns | 539965ns | -11.69% | 7.881 |

## Performance model

- Peak throughput: 8.023 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.489 | 18.6% |
| evs_engine_par | 5.852 | 72.9% |
| evs_std | 7.115 | 88.7% |
| evs_std_par | 7.933 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 2823791ns | 2823791ns | +367.04% |
| evs_engine_par | 722126ns | 722126ns | +19.44% |
| evs_std | 604609ns | 604609ns | base |
| evs_std_par | 535635ns | 535635ns | -11.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 589521ns | base | --- | [580150, 622033] | --- | --- | --- | --- |
| evs_engine | 2817412ns | +2227929.1ns (+377.9%) | [+2172242, +2255617]ns | [2792683, 2835767] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 716767ns | +131745.8ns (+22.3%) | [+77575, +153258]ns | [703150, 736092] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 528712ns | -62912.5ns (-10.7%) | [-77175, -39317]ns | [523683, 543808] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 587358ns | +377.3% | +21.6% | -10.0% |
| 1 | 2 | 589521ns | +379.3% | +22.0% | -9.6% |

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
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 22816530.2ns | 2820036.5ns | 809.1% | HIGH |
| evs_engine_par | 22874511.5ns | 717783.3ns | 3186.8% | HIGH |
| evs_std | 22961926.0ns | 602667.7ns | 3810.0% | HIGH |
| evs_std_par | 22797638.5ns | 532229.2ns | 4283.4% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 2788583.3-2842656.2 ns)
  2788583.3 |########################################
  2791286.9 |########################################
  2793990.6 |########################################
  2796694.2 |
  2799397.9 |
  2802101.5 |
  2804805.2 |
  2807508.8 |
  2810212.5 |
  2812916.1 |########################################
  2815619.8 |
  2818323.4 |########################################
  2821027.1 |
  2823730.7 |
  2826434.4 |
  2829138.0 |########################################
  2831841.7 |
  2834545.3 |########################################
  2837249.0 |
  2839952.6 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 699608.3-731193.8 ns)
  699608.3 |########################################
  701187.6 |
  702766.8 |########################################
  704346.1 |########################################
  705925.4 |
  707504.7 |
  709083.9 |########################################
  710663.2 |
  712242.5 |
  713821.8 |
  715401.0 |
  716980.3 |
  718559.6 |
  720138.8 |
  721718.1 |
  723297.4 |########################################
  724876.7 |
  726455.9 |
  728035.2 |########################################
  729614.5 |
  (0 below, 2 above range)

evs_std (n=8, range 570483.3-625220.8 ns)
  570483.3 |########################################
  573220.2 |
  575957.1 |
  578693.9 |########################################
  581430.8 |########################################
  584167.7 |
  586904.6 |########################################
  589641.4 |########################################
  592378.3 |########################################
  595115.2 |
  597852.1 |
  600588.9 |
  603325.8 |
  606062.7 |
  608799.6 |
  611536.4 |
  614273.3 |
  617010.2 |
  619747.1 |########################################
  622483.9 |
  (0 below, 1 above range)

evs_std_par (n=8, range 522758.3-539964.6 ns)
  522758.3 |########################################
  523618.6 |########################################
  524478.9 |########################################
  525339.2 |
  526199.6 |########################################
  527059.9 |
  527920.2 |
  528780.5 |
  529640.8 |
  530501.1 |########################################
  531361.4 |
  532221.8 |
  533082.1 |
  533942.4 |
  534802.7 |
  535663.0 |
  536523.3 |
  537383.6 |
  538243.9 |
  539104.3 |
  (0 below, 3 above range)

```

## Diagnostics

- evs_engine: bridge=809.2% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=3192.6% of algo (FFI overhead may distort results)
- evs_std: bridge=3876.2% of algo (FFI overhead may distort results)
- evs_std_par: bridge=4310.2% of algo (FFI overhead may distort results)
