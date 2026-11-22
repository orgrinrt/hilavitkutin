# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_form_intshift dominates: 56% faster than the next best (ema_form_float)

ema_form_intshift (264 ns) leads ema_form_float (411 ns) by 56%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 36% (significant)

ema_form_intshift is -148 ns (36%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_q32's edge over baseline is significant but tiny (6 ns, 1.42%)

ema_form_q32 differs from baseline ema_form_float by 6 ns (1.42%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ema_form_intshift** at 263.8 ns median (-35.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.58x (fastest 263.8 ns, slowest 417.5 ns)

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
| ema_form_float | 2600ns | 2584ns | 2571ns | 2592ns | 2650ns | base |
| ema_form_intshift | 2381ns | 2376ns | 2352ns | 2378ns | 2417ns | -8.42% |
| ema_form_q32 | 2532ns | 2511ns | 2502ns | 2521ns | 2594ns | -2.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 412ns | 406ns | 422ns | base | 0.621 |
| ema_form_intshift | 264ns | 258ns | 270ns | -36.03% | 0.970 |
| ema_form_q32 | 419ns | 412ns | 432ns | +1.70% | 0.610 |

## Performance model

- Peak throughput: **0.994 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.623 | 62.7% |
| ema_form_intshift | 0.970 | 97.7% |
| ema_form_q32 | 0.613 | 61.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 2600ns | 2600ns | base |
| ema_form_intshift | 2381ns | 2381ns | -8.42% |
| ema_form_q32 | 2532ns | 2532ns | -2.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 411ns | base | --- | [410, 412] | --- | --- | --- | --- |
| ema_form_intshift | 264ns | -147.5ns (-35.9%) | [-149, -147]ns | [263, 265] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 418ns | +5.8ns (+1.4%) | [+5, +7]ns | [417, 418] | YES | 0.0000 | 0.0000 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 413ns | -36.2% | +2.5% |
| 1 | 2 | 410ns | -35.5% | +2.2% |
| 1 | 3 | 412ns | -35.4% | +1.5% |
| 1 | 4 | 409ns | -34.7% | +2.6% |
| 1 | 5 | 415ns | -37.0% | +0.5% |
| 1 | 6 | 417ns | -35.8% | +0.0% |
| 1 | 7 | 410ns | -36.0% | +1.5% |
| 1 | 8 | 409ns | -35.9% | +2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | 0.021 | ok |
| ema_form_intshift | -0.430 | moderate- |
| ema_form_q32 | 0.217 | moderate+ |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 0/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 3.0ns | 412.4ns | 0.7% |  |
| ema_form_intshift | 2.7ns | 263.8ns | 1.0% |  |
| ema_form_q32 | 2.7ns | 419.4ns | 0.6% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 406.2-422.1 ns)
    406.2 |############
    407.0 |###################################
    407.8 |###################################
    408.6 |########################################
    409.4 |###########################
    410.2 |####################
    411.0 |###########################
    411.7 |#########################
    412.5 |#########################
    413.3 |#####
    414.1 |#######
    414.9 |#################
    415.7 |##
    416.5 |###########################
    417.3 |############
    418.1 |###############
    418.9 |#######
    419.7 |#####
    420.5 |#####
    421.3 |
  (12 below, 7 above range)

ema_form_intshift (n=160, range 257.6-269.7 ns)
    257.6 |##
    258.2 |######################
    258.8 |########
    259.4 |############################
    260.0 |##############
    260.6 |######################
    261.2 |#####
    261.8 |######################
    262.4 |##############
    263.0 |######################
    263.6 |##################################
    264.2 |##############
    264.8 |########################################
    265.4 |#####
    266.0 |##############
    266.6 |#########################
    267.3 |##############
    267.9 |#####################################
    268.5 |##
    269.1 |####################
  (15 below, 14 above range)

ema_form_q32 (n=160, range 411.6-432.3 ns)
    411.6 |##################
    412.7 |#########################
    413.7 |##################
    414.7 |##############
    415.8 |########################################
    416.8 |###########################
    417.8 |###################################
    418.9 |#############################
    419.9 |##############
    421.0 |####
    422.0 |##############
    423.0 |##########
    424.1 |################
    425.1 |####
    426.1 |######
    427.2 |##
    428.2 |##
    429.2 |##
    430.3 |##
    431.3 |
  (13 below, 9 above range)

```
