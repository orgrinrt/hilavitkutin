# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.5% (fastest 20.37 us to slowest 20.67 us).


### evs_std is fastest but the noisiest (CV 10.4%)

evs_std wins on median (20.37 us) yet has the highest variance (CV 10.4%), while evs_engine is the steadiest (CV 3.8%, 20.67 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (302 ns) is smaller than the fastest variant's own run-to-run std-dev (2.12 us); the ranking is inside the noise.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (20.37 us median); no rival improves on it (all deltas are >= 0).


### Speed leader evs_std vs stability leader evs_engine (+1% speed for 2.8x steadier)

evs_std is fastest (20.37 us, CV 10.4%); evs_engine gives up 1.5% median for 2.8x lower variance (CV 3.8%).


## Key findings

- Baseline (evs_std) is the fastest at 20373.2 ns median
- Spread: 1.01x (fastest 20373.2 ns, slowest 20674.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, four-stage RAW chain |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000001 |
| Counter frequency | 2112872410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 20847ns | 20804ns | 19992ns | 20096ns | 21625ns | -2.37% |
| evs_std | 21353ns | 20504ns | 20440ns | 20470ns | 22243ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 20721ns | 19891ns | 21498ns | -2.38% | 3.163 |
| evs_std | 21226ns | 20301ns | 22113ns | base | 3.088 |

## Performance model

- Peak throughput: 3.295 Gops/s (evs_engine; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 3.170 | 96.2% |
| evs_std | 3.217 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 20847ns | 20847ns | -2.37% |
| evs_std | 21353ns | 21353ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 20373ns | base | --- | [20330, 20661] | --- | --- | --- | --- |
| evs_engine | 20675ns | no significant difference | [-688, +1033]ns | [19928, 21592] | no | 0.7266 | 0.7266 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 20475ns | +5.0% |
| 1 | 2 | 20355ns | -2.0% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 244506.9ns | 20720.7ns | 1180.0% | HIGH |
| evs_std | 247206.7ns | 21226.0ns | 1164.6% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 19891.1-21497.9 ns)
  19891.1 |########################################
  19971.4 |########################################
  20051.8 |
  20132.1 |
  20212.5 |
  20292.8 |
  20373.1 |
  20453.5 |
  20533.8 |
  20614.2 |
  20694.5 |
  20774.8 |
  20855.2 |
  20935.5 |
  21015.9 |
  21096.2 |
  21176.5 |
  21256.9 |
  21337.2 |########################################
  21417.6 |
  (0 below, 2 above range)

evs_std (n=8, range 20300.9-22113.1 ns)
  20300.9 |########################################
  20391.5 |
  20482.1 |
  20572.7 |################
  20663.3 |
  20753.9 |
  20844.6 |
  20935.2 |
  21025.8 |
  21116.4 |
  21207.0 |
  21297.6 |
  21388.2 |
  21478.8 |
  21569.4 |
  21660.0 |
  21750.6 |
  21841.2 |
  21931.9 |
  22022.5 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=1181.0% of algo (FFI overhead may distort results)
- evs_std: bridge=1195.5% of algo (FFI overhead may distort results)
