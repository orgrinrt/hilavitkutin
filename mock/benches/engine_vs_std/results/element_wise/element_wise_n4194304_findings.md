# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 4.00 ms to slowest 4.01 ms).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4.61 us) is smaller than the fastest variant's own run-to-run std-dev (133.19 us); the ranking is inside the noise.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (4.00 ms median); no rival improves on it (all deltas are >= 0).


## Key findings

- Baseline (evs_std) is the fastest at 4004031.3 ns median
- Spread: 1.00x (fastest 4004031.3 ns, slowest 4008642.2 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 4030615ns | 4013458ns | 3622540ns | 3877533ns | 4247445ns | +0.79% |
| evs_std | 3999203ns | 4008340ns | 3747529ns | 3936765ns | 4108950ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 4026016ns | 3617800ns | 4242737ns | +0.78% | 1.042 |
| evs_std | 3994681ns | 3743086ns | 4104255ns | base | 1.050 |

## Performance model

- Peak throughput: 1.159 Gops/s (evs_engine; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 1.046 | 90.3% |
| evs_std | 1.048 | 90.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 4030615ns | 4030615ns | +0.79% |
| evs_std | 3999203ns | 3999203ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 4004031ns | base | --- | [3912412, 4091306] | --- | --- | --- | --- |
| evs_engine | 4008642ns | no significant difference | [-294613, +222559]ns | [3821170, 4290724] | no | 0.7266 | 0.7266 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 4079735ns | +4.0% |
| 1 | 2 | 3916179ns | -1.0% |

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
| evs_engine | 23462874.7ns | 4026015.6ns | 582.8% | HIGH |
| evs_std | 24077218.5ns | 3994681.4ns | 602.7% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 3617799.7-4242737.1 ns)
  3617799.7 |########################################
  3649046.6 |
  3680293.4 |
  3711540.3 |
  3742787.2 |
  3774034.0 |
  3805280.9 |########################################
  3836527.8 |########################################
  3867774.7 |
  3899021.5 |
  3930268.4 |########################################
  3961515.3 |
  3992762.1 |
  4024009.0 |
  4055255.9 |########################################
  4086502.7 |
  4117749.6 |
  4148996.5 |
  4180243.3 |########################################
  4211490.2 |
  (0 below, 2 above range)

evs_std (n=8, range 3743086.3-4104254.7 ns)
  3743086.3 |####################
  3761144.7 |
  3779203.1 |
  3797261.6 |
  3815320.0 |
  3833378.4 |
  3851436.8 |
  3869495.2 |
  3887553.6 |
  3905612.1 |########################################
  3923670.5 |
  3941728.9 |
  3959787.3 |####################
  3977845.7 |
  3995904.2 |
  4013962.6 |
  4032021.0 |####################
  4050079.4 |
  4068137.8 |####################
  4086196.3 |####################
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=594.6% of algo (FFI overhead may distort results)
- evs_std: bridge=600.7% of algo (FFI overhead may distort results)
