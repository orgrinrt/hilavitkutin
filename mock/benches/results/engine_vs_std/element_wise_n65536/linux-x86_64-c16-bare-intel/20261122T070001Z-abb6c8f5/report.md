# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std is fastest but the noisiest (CV 12.1%)

evs_std wins on median (20.44 us) yet has the highest variance (CV 12.1%), while evs_engine is the steadiest (CV 0.4%, 21.52 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.07 us) is smaller than the fastest variant's own run-to-run std-dev (2.48 us); the ranking is inside the noise.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (20.44 us median); no rival improves on it (all deltas are >= 0).


### Speed leader evs_std vs stability leader evs_engine (+5% speed for 29.9x steadier)

evs_std is fastest (20.44 us, CV 12.1%); evs_engine gives up 5.3% median for 29.9x lower variance (CV 0.4%).


## Key findings

- Baseline (evs_std) is the fastest at 20443.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 20443.6 ns, slowest 21518.0 ns)

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
| Counter frequency | 2112534967 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 21661ns | 21621ns | 21584ns | 21608ns | 21719ns | +0.84% |
| evs_std | 21481ns | 20549ns | 20379ns | 20438ns | 22538ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 21534ns | 21406ns | 21599ns | +0.77% | 3.043 |
| evs_std | 21369ns | 20293ns | 22422ns | base | 3.067 |

## Performance model

- Peak throughput: 3.230 Gops/s (evs_std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 3.046 | 94.3% |
| evs_std | 3.206 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 21661ns | 21661ns | +0.84% |
| evs_std | 21481ns | 21481ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 20444ns | base | --- | [20305, 20662] | --- | --- | --- | --- |
| evs_engine | 21518ns | +1113.8ns (+5.4%) | [+849, +1220]ns | [21468, 21624] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 20619ns | +4.2% |
| 1 | 2 | 20305ns | +6.0% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 244727.6ns | 21533.5ns | 1136.5% | HIGH |
| evs_std | 245407.9ns | 21368.8ns | 1148.4% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 21406.4-21599.1 ns)
  21406.4 |########################################
  21416.0 |
  21425.7 |
  21435.3 |
  21444.9 |
  21454.6 |
  21464.2 |########################################
  21473.8 |
  21483.5 |########################################
  21493.1 |
  21502.8 |########################################
  21512.4 |
  21522.0 |########################################
  21531.7 |
  21541.3 |########################################
  21550.9 |
  21560.6 |
  21570.2 |
  21579.8 |
  21589.5 |
  (0 below, 2 above range)

evs_std (n=8, range 20292.7-22421.6 ns)
  20292.7 |########################################
  20399.1 |
  20505.6 |####################
  20612.0 |##########
  20718.5 |
  20824.9 |
  20931.4 |
  21037.8 |
  21144.3 |
  21250.7 |
  21357.2 |
  21463.6 |
  21570.1 |
  21676.5 |
  21782.9 |
  21889.4 |
  21995.8 |
  22102.3 |
  22208.7 |
  22315.2 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=1135.3% of algo (FFI overhead may distort results)
- evs_std: bridge=1197.4% of algo (FFI overhead may distort results)
