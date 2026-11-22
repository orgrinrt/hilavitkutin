# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_form_intshift dominates: 57% faster than the next best (ema_form_float)

ema_form_intshift (262 ns) leads ema_form_float (410 ns) by 57%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 36% (significant)

ema_form_intshift is -149 ns (36%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_q32's edge over baseline is significant but tiny (6 ns, 1.51%)

ema_form_q32 differs from baseline ema_form_float by 6 ns (1.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ema_form_intshift** at 261.7 ns median (-36.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.59x (fastest 261.7 ns, slowest 415.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6c97b3e25a1d8f04 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_form_float | 2595ns | 2590ns | 2584ns | 2591ns | 2619ns | base |
| ema_form_intshift | 2370ns | 2366ns | 2361ns | 2366ns | 2391ns | -8.66% |
| ema_form_q32 | 2524ns | 2521ns | 2515ns | 2521ns | 2543ns | -2.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 410ns | 407ns | 415ns | base | 0.624 |
| ema_form_intshift | 262ns | 256ns | 267ns | -36.27% | 0.979 |
| ema_form_q32 | 417ns | 411ns | 425ns | +1.59% | 0.614 |

## Performance model

- Peak throughput: **1.001 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.624 | 62.3% |
| ema_form_intshift | 0.978 | 97.8% |
| ema_form_q32 | 0.616 | 61.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 2595ns | 2595ns | base |
| ema_form_intshift | 2370ns | 2370ns | -8.66% |
| ema_form_q32 | 2524ns | 2524ns | -2.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 410ns | base | --- | [410, 411] | --- | --- | --- | --- |
| ema_form_intshift | 262ns | -148.8ns (-36.3%) | [-150, -148]ns | [261, 262] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 416ns | +6.2ns (+1.5%) | [+6, +6]ns | [415, 417] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 411ns | -36.5% | +1.5% |
| 1 | 2 | 410ns | -36.3% | +1.4% |
| 1 | 3 | 411ns | -36.2% | +1.1% |
| 1 | 4 | 410ns | -36.0% | +1.5% |
| 1 | 5 | 410ns | -36.0% | +1.6% |
| 1 | 6 | 409ns | -36.0% | +1.5% |
| 1 | 7 | 411ns | -36.4% | +1.2% |
| 1 | 8 | 411ns | -36.3% | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.196 | ok |
| ema_form_intshift | 0.350 | moderate+ |
| ema_form_q32 | -0.186 | ok |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 2.9ns | 410.4ns | 0.7% |  |
| ema_form_intshift | 2.6ns | 261.5ns | 1.0% |  |
| ema_form_q32 | 2.5ns | 416.9ns | 0.6% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 406.6-414.7 ns)
    406.6 |########
    407.0 |########
    407.4 |#############
    407.8 |##################
    408.2 |########################
    408.6 |##########################
    409.0 |##################
    409.4 |################################
    409.8 |########################
    410.2 |##################
    410.6 |#####################################
    411.0 |########################################
    411.4 |########################
    411.8 |##########
    412.3 |##########
    412.7 |################
    413.1 |##################
    413.5 |##########
    413.9 |##########
    414.3 |##
  (14 below, 6 above range)

ema_form_intshift (n=160, range 255.8-267.3 ns)
    255.8 |##################
    256.4 |#########
    257.0 |##################
    257.5 |########################
    258.1 |######
    258.7 |##################
    259.3 |##################
    259.8 |####################################
    260.4 |##################
    261.0 |##############################
    261.6 |##############################
    262.1 |##############################
    262.7 |##################
    263.3 |########################################
    263.8 |##################
    264.4 |##################
    265.0 |########################################
    265.6 |#########
    266.1 |###############
    266.7 |###############
  (14 below, 4 above range)

ema_form_q32 (n=160, range 410.8-424.7 ns)
    410.8 |######
    411.5 |##########
    412.2 |############
    412.9 |##########
    413.6 |################
    414.3 |######################
    415.0 |########################################
    415.7 |######################################
    416.4 |####
    417.1 |############################
    417.7 |##################
    418.4 |############
    419.1 |##########################
    419.8 |####################
    420.5 |################
    421.2 |######
    421.9 |##
    422.6 |
    423.3 |
    424.0 |
  (14 below, 3 above range)

```
