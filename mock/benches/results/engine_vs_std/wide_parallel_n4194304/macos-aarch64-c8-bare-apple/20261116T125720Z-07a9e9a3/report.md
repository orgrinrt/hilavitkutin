# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par beats baseline by 72% (significant)

evs_std_par is -8.53 ms (72%) faster than baseline evs_std, with a CI that excludes zero.


### evs_engine is an outlier: 3.6x slower than the field

evs_engine (11.80 ms) is 3.6x the fastest (3.24 ms), well off the pack.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {evs_std_par, evs_engine_par} vs {evs_std, evs_engine} (260% apart)

The field splits into a fast tier {evs_std_par, evs_engine_par} and a slow tier {evs_std, evs_engine} with a 260% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 3.6x the fastest

Fastest evs_std_par (3.24 ms) to slowest evs_engine (11.80 ms): 3.6x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: evs_std_par at 3235666.6 ns median (-72.5% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.65x (fastest 3235666.6 ns, slowest 11804408.3 ns)

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
| evs_engine | 11848061ns | 11806888ns | 11795308ns | 11803506ns | 11894667ns | +0.14% |
| evs_engine_par | 3394370ns | 3274821ns | 3257883ns | 3268692ns | 3522750ns | -71.31% |
| evs_std | 11831833ns | 11777775ns | 11763608ns | 11775650ns | 11891027ns | base |
| evs_std_par | 3263122ns | 3241192ns | 3233000ns | 3236839ns | 3290365ns | -72.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 11845654ns | 11793067ns | 11892417ns | +0.14% | 0.354 |
| evs_engine_par | 3390027ns | 3255467ns | 3518169ns | -71.34% | 1.237 |
| evs_std | 11829153ns | 11760967ns | 11888529ns | base | 0.355 |
| evs_std_par | 3258371ns | 3228783ns | 3285763ns | -72.45% | 1.287 |

## Performance model

- Peak throughput: 1.299 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.355 | 27.4% |
| evs_engine_par | 1.283 | 98.7% |
| evs_std | 0.356 | 27.4% |
| evs_std_par | 1.296 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 11848061ns | 11848061ns | +0.14% |
| evs_engine_par | 3394370ns | 3394370ns | -71.31% |
| evs_std | 11831833ns | 11831833ns | base |
| evs_std_par | 3263122ns | 3263122ns | -72.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 11774871ns | base | --- | [11772158, 11784667] | --- | --- | --- | --- |
| evs_engine | 11804408ns | +28508.3ns (+0.2%) | [+16975, +37950]ns | [11795742, 11810242] | YES (adj: no) | 0.0703 | 0.0703 | 0 |
| evs_engine_par | 3269958ns | -8509245.8ns (-72.3%) | [-8516825, -8250458]ns | [3258783, 3630258] | YES | 0.0117 | 0.0078 | 0 |
| evs_std_par | 3235667ns | -8531662.5ns (-72.5%) | [-8548267, -8493725]ns | [3229050, 3278567] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 11777408ns | +0.2% | -70.7% | -72.6% |
| 1 | 2 | 11772992ns | +0.3% | -72.3% | -72.3% |

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
| evs_engine | 91359679.2ns | 11845654.2ns | 771.3% | HIGH |
| evs_engine_par | 91653486.5ns | 3390027.1ns | 2703.6% | HIGH |
| evs_std | 91379877.1ns | 11829153.1ns | 772.5% | HIGH |
| evs_std_par | 91437015.6ns | 3258370.8ns | 2806.2% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 11793066.7-11892416.7 ns)
  11793066.7 |########################################
  11798034.2 |####################
  11803001.7 |########################################
  11807969.2 |########################################
  11812936.7 |
  11817904.2 |
  11822871.7 |
  11827839.2 |
  11832806.7 |
  11837774.2 |
  11842741.7 |
  11847709.2 |
  11852676.7 |
  11857644.2 |
  11862611.7 |
  11867579.2 |
  11872546.7 |
  11877514.2 |
  11882481.7 |
  11887449.2 |
  (0 below, 1 above range)

evs_engine_par (n=8, range 3255466.7-3518168.7 ns)
  3255466.7 |########################################
  3268601.8 |##########
  3281736.9 |
  3294872.0 |
  3308007.1 |
  3321142.2 |
  3334277.3 |
  3347412.4 |
  3360547.5 |
  3373682.6 |
  3386817.7 |
  3399952.8 |
  3413087.9 |
  3426223.0 |
  3439358.1 |
  3452493.2 |
  3465628.3 |
  3478763.4 |
  3491898.5 |
  3505033.6 |
  (0 below, 3 above range)

evs_std (n=8, range 11760966.7-11888529.2 ns)
  11760966.7 |#############
  11767344.8 |########################################
  11773722.9 |##########################
  11780101.1 |#############
  11786479.2 |
  11792857.3 |
  11799235.4 |
  11805613.6 |
  11811991.7 |
  11818369.8 |
  11824747.9 |
  11831126.1 |
  11837504.2 |
  11843882.3 |
  11850260.4 |
  11856638.6 |
  11863016.7 |
  11869394.8 |
  11875772.9 |
  11882151.1 |
  (0 below, 1 above range)

evs_std_par (n=8, range 3228783.3-3285762.5 ns)
  3228783.3 |########################################
  3231632.3 |
  3234481.2 |##########################
  3237330.2 |
  3240179.1 |
  3243028.1 |
  3245877.1 |
  3248726.0 |
  3251575.0 |
  3254424.0 |
  3257272.9 |
  3260121.9 |
  3262970.8 |
  3265819.8 |
  3268668.8 |#############
  3271517.7 |
  3274366.7 |
  3277215.6 |#############
  3280064.6 |
  3282913.6 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=773.7% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=2795.4% of algo (FFI overhead may distort results)
- evs_std: bridge=776.2% of algo (FFI overhead may distort results)
- evs_std_par: bridge=2826.4% of algo (FFI overhead may distort results)
