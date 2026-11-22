# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (evs_std) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline evs_std has the worst median (594.49 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest evs_engine at 578.18 us).


### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.8% (fastest 578.18 us to slowest 594.49 us).


### evs_std shows alternating (throttle bounce) (autocorr -0.50)

evs_std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (16.31 us) is smaller than the fastest variant's own run-to-run std-dev (16.40 us); the ranking is inside the noise.


## Key findings

- Fastest: evs_engine at 578179.2 ns median (-2.7% vs baseline)
- Spread: 1.03x (fastest 578179.2 ns, slowest 594487.5 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 587183ns | 580300ns | 571492ns | 574531ns | 600596ns | -4.29% |
| evs_std | 613474ns | 596400ns | 568050ns | 583867ns | 647035ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 585151ns | 569225ns | 598565ns | -4.32% | 7.168 |
| evs_std | 611589ns | 565458ns | 645456ns | base | 6.858 |

## Performance model

- Peak throughput: 7.418 Gops/s (evs_std; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 7.254 | 97.8% |
| evs_std | 7.055 | 95.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 587183ns | 587183ns | -4.29% |
| evs_std | 613474ns | 613474ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 594488ns | base | --- | [573275, 645150] | --- | --- | --- | --- |
| evs_engine | 578179ns | no significant difference | [-51417, +2042]ns | [571050, 602233] | no | 0.2891 | 0.2891 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine |
|---|---|---|---|
| 1 | 1 | 633104ns | -5.5% |
| 1 | 2 | 586075ns | -2.5% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 22853884.4ns | 585151.0ns | 3905.6% | HIGH |
| evs_std | 22895034.4ns | 611588.5ns | 3743.5% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 569225.0-598564.6 ns)
  569225.0 |####################
  570692.0 |########################################
  572159.0 |
  573625.9 |
  575092.9 |####################
  576559.9 |
  578026.9 |
  579493.9 |
  580960.8 |####################
  582427.8 |
  583894.8 |
  585361.8 |
  586828.7 |
  588295.7 |
  589762.7 |
  591229.7 |
  592696.7 |####################
  594163.6 |
  595630.6 |
  597097.6 |
  (0 below, 2 above range)

evs_std (n=8, range 565458.3-645456.2 ns)
  565458.3 |########################################
  569458.2 |########################################
  573458.1 |
  577458.0 |
  581457.9 |########################################
  585457.8 |########################################
  589457.7 |
  593457.6 |
  597457.5 |########################################
  601457.4 |
  605457.3 |
  609457.2 |
  613457.1 |
  617457.0 |########################################
  621456.9 |
  625456.8 |
  629456.7 |
  633456.6 |
  637456.5 |
  641456.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=3945.0% of algo (FFI overhead may distort results)
- evs_std: bridge=3835.7% of algo (FFI overhead may distort results)
