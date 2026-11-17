# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine is an outlier: 5.3x slower than the field

evs_engine (2.83 ms) is 5.3x the fastest (538.24 us), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_std, evs_engine_par} vs {evs_engine} (285% apart)

The field splits into a fast tier {evs_std_par, evs_std, evs_engine_par} and a slow tier {evs_engine} with a 285% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.3x the fastest

Fastest evs_std_par (538.24 us) to slowest evs_engine (2.83 ms): 5.3x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 538237.5 ns median (-8.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.26x (fastest 538237.5 ns, slowest 2831075.0 ns)

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
| evs_engine | 2838703ns | 2836679ns | 2804108ns | 2820400ns | 2861079ns | +382.01% |
| evs_engine_par | 749644ns | 740017ns | 721133ns | 731306ns | 770525ns | +27.29% |
| evs_std | 588933ns | 588871ns | 572683ns | 581536ns | 598544ns | base |
| evs_std_par | 544869ns | 543388ns | 530850ns | 534811ns | 555917ns | -7.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 2833894ns | 2795900ns | 2856671ns | +382.83% | 1.480 |
| evs_engine_par | 745266ns | 716858ns | 766427ns | +26.98% | 5.628 |
| evs_std | 586932ns | 571092ns | 596421ns | base | 7.146 |
| evs_std_par | 540583ns | 527233ns | 551223ns | -7.90% | 7.759 |

## Performance model

- Peak throughput: 7.955 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.482 | 18.6% |
| evs_engine_par | 5.702 | 71.7% |
| evs_std | 7.145 | 89.8% |
| evs_std_par | 7.793 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 2838703ns | 2838703ns | +382.01% |
| evs_engine_par | 749644ns | 749644ns | +27.29% |
| evs_std | 588933ns | 588933ns | base |
| evs_std_par | 544869ns | 544869ns | -7.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 587004ns | base | --- | [572792, 596533] | --- | --- | --- | --- |
| evs_engine | 2831075ns | +2239575.0ns (+381.5%) | [+2220142, +2281208]ns | [2805017, 2854000] | YES | 0.0078 | 0.0078 | 0 |
| evs_engine_par | 735529ns | +145208.3ns (+24.7%) | [+137275, +176958]ns | [718292, 773492] | YES | 0.0078 | 0.0078 | 0 |
| evs_std_par | 538238ns | -49912.5ns (-8.5%) | [-63317, -18608]ns | [529358, 552483] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 582412ns | +387.3% | +26.9% | -5.9% |
| 1 | 2 | 587004ns | +378.9% | +24.5% | -9.8% |

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
| evs_engine | 22851381.3ns | 2833893.8ns | 806.4% | HIGH |
| evs_engine_par | 22888914.6ns | 745265.6ns | 3071.2% | HIGH |
| evs_std | 22764041.7ns | 586932.3ns | 3878.5% | HIGH |
| evs_std_par | 22931468.8ns | 540583.3ns | 4242.0% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 2795900.0-2856670.8 ns)
  2795900.0 |########################################
  2798938.5 |
  2801977.1 |
  2805015.6 |########################################
  2808054.2 |
  2811092.7 |
  2814131.3 |########################################
  2817169.8 |
  2820208.3 |
  2823246.9 |
  2826285.4 |########################################
  2829324.0 |
  2832362.5 |########################################
  2835401.1 |
  2838439.6 |########################################
  2841478.1 |
  2844516.7 |
  2847555.2 |
  2850593.8 |
  2853632.3 |########################################
  (0 below, 1 above range)

evs_engine_par (n=8, range 716858.3-766427.1 ns)
  716858.3 |########################################
  719336.7 |
  721815.2 |
  724293.6 |
  726772.1 |####################
  729250.5 |
  731728.9 |####################
  734207.4 |
  736685.8 |####################
  739164.2 |####################
  741642.7 |
  744121.1 |
  746599.6 |
  749078.0 |
  751556.4 |
  754034.9 |
  756513.3 |
  758991.8 |
  761470.2 |
  763948.6 |
  (0 below, 2 above range)

evs_std (n=8, range 571091.7-596420.8 ns)
  571091.7 |########################################
  572358.2 |########################################
  573624.6 |
  574891.1 |
  576157.5 |
  577424.0 |
  578690.4 |
  579956.9 |########################################
  581223.3 |
  582489.8 |
  583756.2 |########################################
  585022.7 |
  586289.2 |
  587555.6 |
  588822.1 |########################################
  590088.5 |
  591355.0 |########################################
  592621.4 |
  593887.9 |
  595154.3 |
  (0 below, 2 above range)

evs_std_par (n=8, range 527233.3-551222.9 ns)
  527233.3 |########################################
  528432.8 |########################################
  529632.3 |########################################
  530831.7 |
  532031.2 |########################################
  533230.7 |
  534430.2 |
  535629.7 |
  536829.1 |
  538028.6 |
  539228.1 |
  540427.6 |
  541627.1 |
  542826.5 |########################################
  544026.0 |########################################
  545225.5 |
  546425.0 |
  547624.5 |
  548823.9 |
  550023.4 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=806.1% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=3108.8% of algo (FFI overhead may distort results)
- evs_std: bridge=3875.0% of algo (FFI overhead may distort results)
- evs_std_par: bridge=4241.5% of algo (FFI overhead may distort results)
