# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine is an outlier: 5.0x slower than the field

evs_engine (2.79 ms) is 5.0x the fastest (553.65 us), well off the pack.


### evs_std_par is fastest but the noisiest (CV 5.4%)

evs_std_par wins on median (553.65 us) yet has the highest variance (CV 5.4%), while evs_engine is the steadiest (CV 1.3%, 2.79 ms).


### evs_engine_par shows alternating (throttle bounce) (autocorr -0.50)

evs_engine_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_std, evs_engine_par} vs {evs_engine} (266% apart)

The field splits into a fast tier {evs_std_par, evs_std, evs_engine_par} and a slow tier {evs_engine} with a 266% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.0x the fastest

Fastest evs_std_par (553.65 us) to slowest evs_engine (2.79 ms): 5.0x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 553650.0 ns median (-6.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.04x (fastest 553650.0 ns, slowest 2789420.9 ns)

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
| evs_engine | 2799449ns | 2792196ns | 2766233ns | 2782428ns | 2820519ns | +363.32% |
| evs_engine_par | 768217ns | 765608ns | 733908ns | 752814ns | 788346ns | +27.14% |
| evs_std | 604217ns | 594883ns | 579608ns | 583850ns | 625644ns | base |
| evs_std_par | 563854ns | 556167ns | 529358ns | 540144ns | 590260ns | -6.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 2796597ns | 2764108ns | 2817421ns | +364.32% | 1.500 |
| evs_engine_par | 764743ns | 731517ns | 785033ns | +26.97% | 5.485 |
| evs_std | 602299ns | 577617ns | 623488ns | base | 6.964 |
| evs_std_par | 560777ns | 525658ns | 587185ns | -6.89% | 7.479 |

## Performance model

- Peak throughput: 7.979 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.504 | 18.8% |
| evs_engine_par | 5.505 | 69.0% |
| evs_std | 7.077 | 88.7% |
| evs_std_par | 7.576 | 94.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 2799449ns | 2799449ns | +363.32% |
| evs_engine_par | 768217ns | 768217ns | +27.14% |
| evs_std | 604217ns | 604217ns | base |
| evs_std_par | 563854ns | 563854ns | -6.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 592696ns | base | --- | [578367, 637542] | --- | --- | --- | --- |
| evs_engine | 2789421ns | +2197624.9ns (+370.8%) | [+2156992, +2209233]ns | [2771033, 2795675] | YES | 0.0117 | 0.0078 | 0 |
| evs_engine_par | 761925ns | +152958.3ns (+25.8%) | [+148033, +188317]ns | [737008, 788175] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 553650ns | -35991.7ns (-6.1%) | [-73292, -9908]ns | [534417, 601233] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 618821ns | +349.4% | +25.4% | -5.7% |
| 1 | 2 | 584229ns | +377.8% | +27.9% | -7.9% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | HIGH- (thermal bounce) |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 0/2, lost 2/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 22846453.1ns | 2796596.9ns | 816.9% | HIGH |
| evs_engine_par | 22888139.6ns | 764742.7ns | 2992.9% | HIGH |
| evs_std | 22807028.1ns | 602299.0ns | 3786.7% | HIGH |
| evs_std_par | 22873333.3ns | 560777.1ns | 4078.9% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 2764108.3-2817420.8 ns)
  2764108.3 |####################
  2766773.9 |
  2769439.6 |####################
  2772105.2 |
  2774770.8 |
  2777436.4 |
  2780102.1 |####################
  2782767.7 |
  2785433.3 |####################
  2788098.9 |
  2790764.6 |####################
  2793430.2 |########################################
  2796095.8 |
  2798761.4 |
  2801427.1 |
  2804092.7 |
  2806758.3 |
  2809423.9 |
  2812089.6 |
  2814755.2 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 731516.7-785033.3 ns)
  731516.7 |########################################
  734192.5 |
  736868.4 |########################################
  739544.2 |
  742220.0 |
  744895.9 |
  747571.7 |
  750247.5 |########################################
  752923.3 |
  755599.2 |########################################
  758275.0 |
  760950.8 |
  763626.7 |
  766302.5 |########################################
  768978.3 |
  771654.2 |
  774330.0 |
  777005.8 |
  779681.7 |
  782357.5 |
  (0 below, 3 above range)

evs_std (n=8, range 577616.7-623487.5 ns)
  577616.7 |########################################
  579910.2 |####################
  582203.8 |
  584497.3 |####################
  586790.9 |
  589084.4 |
  591377.9 |
  593671.5 |
  595965.0 |
  598258.6 |########################################
  600552.1 |
  602845.6 |
  605139.2 |
  607432.7 |
  609726.3 |
  612019.8 |
  614313.3 |
  616606.9 |
  618900.4 |
  621194.0 |
  (0 below, 2 above range)

evs_std_par (n=8, range 525658.3-587185.4 ns)
  525658.3 |########################################
  528734.7 |
  531811.0 |########################################
  534887.4 |########################################
  537963.7 |
  541040.1 |########################################
  544116.4 |
  547192.8 |
  550269.1 |
  553345.5 |
  556421.9 |
  559498.2 |
  562574.6 |########################################
  565650.9 |
  568727.3 |
  571803.6 |########################################
  574880.0 |
  577956.3 |
  581032.7 |
  584109.0 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=819.9% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=3002.4% of algo (FFI overhead may distort results)
- evs_std: bridge=3848.5% of algo (FFI overhead may distort results)
- evs_std_par: bridge=4121.2% of algo (FFI overhead may distort results)
