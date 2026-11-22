# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (11.82 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_std_par at 3.28 ms).


### evs_std_par beats baseline by 72% (significant)

evs_std_par is -8.56 ms (72%) faster than baseline evs_std, with a CI that excludes zero.


### evs_std is an outlier: 3.6x slower than the field

evs_std (11.82 ms) is 3.6x the fastest (3.28 ms), well off the pack.


### evs_std_par is fastest but the noisiest (CV 11.2%)

evs_std_par wins on median (3.28 ms) yet has the highest variance (CV 11.2%), while evs_engine is the steadiest (CV 0.2%, 11.81 ms).


### Top two (evs_std_par, evs_engine_par) are a dead heat (<1%)

evs_std_par (3.28 ms) and evs_engine_par (3.28 ms) differ by 0.06%, inside the noise, even though the wider field spreads 261.0%.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_engine, evs_std} (260% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_engine, evs_std} with a 260% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 3.6x the fastest

Fastest evs_std_par (3.28 ms) to slowest evs_std (11.82 ms): 3.6x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 3275650.0 ns median (-72.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.61x (fastest 3275650.0 ns, slowest 11824350.0 ns)

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
| evs_engine | 11821909ns | 11812717ns | 11795583ns | 11803811ns | 11842065ns | -0.72% |
| evs_engine_par | 3364780ns | 3283638ns | 3270058ns | 3279025ns | 3452777ns | -71.74% |
| evs_std | 11908135ns | 11827371ns | 11771750ns | 11791653ns | 12029594ns | base |
| evs_std_par | 3459705ns | 3281242ns | 3231858ns | 3237931ns | 3682998ns | -70.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 11818798ns | 11792058ns | 11839125ns | -0.73% | 0.355 |
| evs_engine_par | 3359392ns | 3265458ns | 3446946ns | -71.78% | 1.249 |
| evs_std | 11905344ns | 11769425ns | 12026690ns | base | 0.352 |
| evs_std_par | 3454818ns | 3226367ns | 3678706ns | -70.98% | 1.214 |

## Performance model

- Peak throughput: 1.300 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.355 | 27.3% |
| evs_engine_par | 1.280 | 98.4% |
| evs_std | 0.355 | 27.3% |
| evs_std_par | 1.280 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 11821909ns | 11821909ns | -0.72% |
| evs_engine_par | 3364780ns | 3364780ns | -71.74% |
| evs_std | 11908135ns | 11908135ns | base |
| evs_std_par | 3459705ns | 3459705ns | -70.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 11824350ns | base | --- | [11770883, 12200058] | --- | --- | --- | --- |
| evs_engine | 11809504ns | no significant difference | [-391550, +23750]ns | [11796000, 11842200] | no | 1.0000 | 1.0000 | 0 |
| evs_engine_par | 3277642ns | -8526529.2ns (-72.1%) | [-8919883, -8450600]ns | [3272442, 3383558] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 3275650ns | -8563795.8ns (-72.4%) | [-8884833, -8132933]ns | [3230250, 3690500] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 11986154ns | -1.5% | -72.6% | -72.2% |
| 1 | 2 | 11824350ns | +0.0% | -72.3% | -72.7% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 0/2
- evs_engine_par: won 2/2, lost 0/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 91398764.6ns | 11818797.9ns | 773.3% | HIGH |
| evs_engine_par | 91502434.4ns | 3359391.7ns | 2723.8% | HIGH |
| evs_std | 92150000.0ns | 11905343.7ns | 774.0% | HIGH |
| evs_std_par | 91610157.3ns | 3454817.7ns | 2651.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 11792058.3-11839125.0 ns)
  11792058.3 |########################################
  11794411.6 |########################################
  11796765.0 |########################################
  11799118.3 |
  11801471.6 |
  11803825.0 |
  11806178.3 |########################################
  11808531.6 |########################################
  11810885.0 |
  11813238.3 |
  11815591.7 |
  11817945.0 |
  11820298.3 |
  11822651.7 |
  11825005.0 |
  11827358.3 |
  11829711.7 |
  11832065.0 |
  11834418.3 |########################################
  11836771.7 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 3265458.3-3446945.8 ns)
  3265458.3 |########################################
  3274532.7 |########################################
  3283607.1 |
  3292681.4 |
  3301755.8 |
  3310830.2 |
  3319904.6 |
  3328978.9 |
  3338053.3 |
  3347127.7 |
  3356202.1 |
  3365276.4 |
  3374350.8 |
  3383425.2 |#############
  3392499.6 |
  3401573.9 |
  3410648.3 |
  3419722.7 |
  3428797.1 |
  3437871.4 |
  (0 below, 1 above range)

evs_std (n=8, range 11769425.0-12026689.6 ns)
  11769425.0 |########################################
  11782288.2 |
  11795151.5 |
  11808014.7 |
  11820877.9 |##########################
  11833741.1 |#############
  11846604.4 |
  11859467.6 |
  11872330.8 |
  11885194.1 |
  11898057.3 |
  11910920.5 |
  11923783.7 |
  11936647.0 |
  11949510.2 |
  11962373.4 |
  11975236.7 |
  11988099.9 |
  12000963.1 |
  12013826.3 |
  (0 below, 2 above range)

evs_std_par (n=8, range 3226366.7-3678706.2 ns)
  3226366.7 |########################################
  3248983.7 |
  3271600.7 |
  3294217.6 |##########
  3316834.6 |
  3339451.6 |##########
  3362068.6 |
  3384685.5 |
  3407302.5 |
  3429919.5 |
  3452536.5 |
  3475153.5 |
  3497770.4 |
  3520387.4 |
  3543004.4 |
  3565621.4 |
  3588238.3 |
  3610855.3 |
  3633472.3 |
  3656089.3 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine: bridge=773.4% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=2788.9% of algo (FFI overhead may distort results)
- evs_std: bridge=772.4% of algo (FFI overhead may distort results)
- evs_std_par: bridge=2793.0% of algo (FFI overhead may distort results)
