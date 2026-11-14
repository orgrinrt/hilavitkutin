# wide_parallel, get-ready

2 variants, 8 samples per variant.
Baseline: evs_std_startup

## Highlights

Baseline for all deltas below: evs_std_startup. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_startup is fastest but the noisiest (CV 21.9%)

evs_std_startup wins on median (4.55 ms) yet has the highest variance (CV 21.9%), while evs_engine_startup is the steadiest (CV 12.4%, 4.97 ms).


### evs_std_startup shows alternating (throttle bounce) (autocorr -0.50)

evs_std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (415.53 us) is smaller than the fastest variant's own run-to-run std-dev (997.66 us); the ranking is inside the noise.


### No variant beats the baseline (evs_std_startup)

The baseline evs_std_startup is the fastest (4.55 ms median); no rival improves on it (all deltas are >= 0).


### Speed leader evs_std_startup vs stability leader evs_engine_startup (+9% speed for 1.8x steadier)

evs_std_startup is fastest (4.55 ms, CV 21.9%); evs_engine_startup gives up 9.1% median for 1.8x lower variance (CV 12.4%).


### evs_std_startup is inconsistent: worst-20% is 1.6x its best-20%

evs_std_startup's best 20% of batches run at 3.58 ms but its worst 20% at 5.67 ms (1.6x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std_startup) is the fastest at 4552715.8 ns median
- Spread: 1.09x (fastest 4552715.8 ns, slowest 4968249.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010004 |
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine_startup | 4812107ns | 4976115ns | 3864707ns | 4480334ns | 5297786ns | +1.06% |
| evs_std_startup | 4761839ns | 4558431ns | 3585746ns | 3930424ns | 5679424ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine_startup | 4805958ns | 3859662ns | 5291224ns | +1.04% | 0.873 |
| evs_std_startup | 4756268ns | 3580167ns | 5673716ns | base | 0.882 |

## Performance model

- Peak throughput: 1.172 Gops/s (evs_std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine_startup | 0.844 | 72.1% |
| evs_std_startup | 0.921 | 78.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine_startup | 4812107ns | 4812107ns | +1.06% |
| evs_std_startup | 4761839ns | 4761839ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std_startup | 4552716ns | base | --- | [3768442, 5675820] | --- | --- | --- | --- |
| evs_engine_startup | 4968249ns | no significant difference | [-579091, +617561]ns | [3926005, 5337350] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std_startup | evs_engine_startup |
|---|---|---|---|
| 1 | 1 | 3845522ns | +10.1% |
| 1 | 2 | 5599398ns | -6.8% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine_startup | -0.500 | moderate- |
| evs_std_startup | -0.500 | moderate- |

Consistency summary:

- evs_engine_startup: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine_startup | 204331631.2ns | 4805957.6ns | 4251.6% | HIGH |
| evs_std_startup | 208584462.5ns | 4756268.0ns | 4385.5% | HIGH |

## Distribution (algo ns)

```
evs_engine_startup (n=8, range 3859662.2-5291223.7 ns)
  3859662.2 |########################################
  3931240.3 |
  4002818.4 |
  4074396.4 |
  4145974.5 |
  4217552.6 |
  4289130.7 |
  4360708.7 |
  4432286.8 |
  4503864.9 |####################
  4575443.0 |
  4647021.0 |
  4718599.1 |
  4790177.2 |
  4861755.2 |
  4933333.3 |########################################
  5004911.4 |
  5076489.5 |####################
  5148067.5 |
  5219645.6 |
  (0 below, 2 above range)

evs_std_startup (n=8, range 3580167.0-5673715.7 ns)
  3580167.0 |########################################
  3684844.4 |########################################
  3789521.9 |
  3894199.3 |########################################
  3998876.7 |########################################
  4103554.2 |
  4208231.6 |
  4312909.0 |
  4417586.5 |
  4522263.9 |
  4626941.3 |
  4731618.8 |
  4836296.2 |
  4940973.6 |########################################
  5045651.1 |
  5150328.5 |
  5255005.9 |
  5359683.4 |
  5464360.8 |########################################
  5569038.2 |
  (0 below, 2 above range)

```

## Diagnostics

- evs_engine_startup: bridge=4221.8% of algo (FFI overhead may distort results)
- evs_std_startup: CV=21.0% (high variance, measurements may be unstable)
- evs_std_startup: bridge=4610.0% of algo (FFI overhead may distort results)
