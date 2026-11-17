# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_form_intshift dominates: 56% faster than the next best (ema_form_float)

ema_form_intshift (264 ns) leads ema_form_float (412 ns) by 56%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 36% (significant)

ema_form_intshift is -149 ns (36%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_form_q32's edge over baseline is significant but tiny (5 ns, 1.12%)

ema_form_q32 differs from baseline ema_form_float by 5 ns (1.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: ema_form_intshift** at 264.0 ns median (-36.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.58x (fastest 264.0 ns, slowest 417.5 ns)

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
| ema_form_float | 2611ns | 2594ns | 2586ns | 2606ns | 2648ns | base |
| ema_form_intshift | 2401ns | 2378ns | 2362ns | 2391ns | 2470ns | -8.03% |
| ema_form_q32 | 2535ns | 2524ns | 2517ns | 2526ns | 2580ns | -2.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 413ns | 408ns | 421ns | base | 0.619 |
| ema_form_intshift | 264ns | 257ns | 272ns | -36.03% | 0.968 |
| ema_form_q32 | 419ns | 411ns | 429ns | +1.30% | 0.611 |

## Performance model

- Peak throughput: **0.995 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.621 | 62.4% |
| ema_form_intshift | 0.970 | 97.4% |
| ema_form_q32 | 0.613 | 61.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 2611ns | 2611ns | base |
| ema_form_intshift | 2401ns | 2401ns | -8.03% |
| ema_form_q32 | 2535ns | 2535ns | -2.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 412ns | base | --- | [411, 414] | --- | --- | --- | --- |
| ema_form_intshift | 264ns | -149.2ns (-36.2%) | [-151, -147]ns | [263, 265] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 418ns | +4.6ns (+1.1%) | [+3, +6]ns | [417, 419] | YES | 0.0000 | 0.0000 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 411ns | -35.0% | +1.7% |
| 1 | 2 | 411ns | -35.4% | +1.4% |
| 1 | 3 | 418ns | -37.4% | -0.3% |
| 1 | 4 | 411ns | -36.5% | +2.0% |
| 1 | 5 | 411ns | -36.3% | +1.3% |
| 1 | 6 | 410ns | -33.9% | +2.1% |
| 1 | 7 | 418ns | -37.1% | +0.4% |
| 1 | 8 | 413ns | -35.0% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.280 | moderate- |
| ema_form_intshift | -0.184 | ok |
| ema_form_q32 | -0.477 | moderate- |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 2.7ns | 413.4ns | 0.7% |  |
| ema_form_intshift | 2.5ns | 264.4ns | 1.0% |  |
| ema_form_q32 | 2.6ns | 418.7ns | 0.6% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 407.6-420.7 ns)
    407.6 |##############
    408.2 |###############################
    408.9 |########
    409.5 |########################################
    410.2 |#####################################
    410.8 |######################
    411.5 |###############################
    412.2 |####################
    412.8 |##############
    413.5 |######################
    414.1 |####################
    414.8 |########
    415.4 |#####
    416.1 |#########################
    416.7 |#####
    417.4 |######################
    418.1 |###########
    418.7 |############################
    419.4 |#################
    420.0 |##
  (14 below, 9 above range)

ema_form_intshift (n=160, range 257.3-272.4 ns)
    257.3 |####
    258.0 |####
    258.8 |#####################
    259.5 |################
    260.3 |#######################
    261.0 |##############
    261.8 |#####################
    262.6 |########################################
    263.3 |#######
    264.1 |##############
    264.8 |################################
    265.6 |#####################
    266.3 |###########
    267.1 |###########
    267.8 |#########
    268.6 |################
    269.4 |###########
    270.1 |##############
    270.9 |####
    271.6 |#########
  (15 below, 13 above range)

ema_form_q32 (n=160, range 411.5-429.3 ns)
    411.5 |################
    412.4 |##################
    413.3 |########################
    414.2 |##################################
    415.0 |########################
    415.9 |########################################
    416.8 |#############################
    417.7 |##########################
    418.6 |#####################################
    419.5 |#############################
    420.4 |#############################
    421.3 |#############
    422.2 |##
    423.1 |##########
    424.0 |#############
    424.9 |#############
    425.7 |########
    426.6 |##
    427.5 |##
    428.4 |#####
  (12 below, 5 above range)

```
