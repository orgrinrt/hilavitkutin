# EMA formulation: Q0.32 saturating fixed-point vs float vs integer-shift

3 variants, 160 samples per variant.
Baseline: **ema_form_float**

## Highlights

Baseline for all deltas below: **ema_form_float**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ema_form_intshift dominates: 48% faster than the next best (ema_form_float)

ema_form_intshift (296 ns) leads ema_form_float (439 ns) by 48%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_form_intshift beats baseline by 37% (significant)

ema_form_intshift is -162 ns (37%) faster than baseline ema_form_float, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: ema_form_intshift** at 295.8 ns median (-32.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.60x (fastest 295.8 ns, slowest 474.2 ns)

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
| ema_form_float | 2784ns | 2765ns | 2576ns | 2783ns | 2995ns | base |
| ema_form_intshift | 2598ns | 2718ns | 2359ns | 2630ns | 2739ns | -6.70% |
| ema_form_q32 | 2789ns | 2885ns | 2589ns | 2814ns | 2913ns | +0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_form_float | 442ns | 408ns | 478ns | base | 0.579 |
| ema_form_intshift | 288ns | 261ns | 309ns | -34.84% | 0.889 |
| ema_form_q32 | 461ns | 428ns | 486ns | +4.35% | 0.555 |

## Performance model

- Peak throughput: **0.982 Gops/s** (ema_form_intshift; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_form_float | 0.583 | 59.4% |
| ema_form_intshift | 0.865 | 88.1% |
| ema_form_q32 | 0.540 | 55.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_form_float | 2784ns | 2784ns | base |
| ema_form_intshift | 2598ns | 2598ns | -6.70% |
| ema_form_q32 | 2789ns | 2789ns | +0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_form_float | 439ns | base | --- | [424, 463] | --- | --- | --- | --- |
| ema_form_intshift | 296ns | -161.6ns (-36.8%) | [-167, -151]ns | [286, 299] | YES | 0.0000 | 0.0000 | 0 |
| ema_form_q32 | 474ns | +12.5ns (+2.8%) | [+8, +33]ns | [452, 478] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_form_float | ema_form_intshift | ema_form_q32 |
|---|---|---|---|---|
| 1 | 1 | 423ns | -37.8% | +13.1% |
| 1 | 2 | 474ns | -37.5% | -6.0% |
| 1 | 3 | 421ns | -28.1% | +13.2% |
| 1 | 4 | 412ns | -26.6% | +16.5% |
| 1 | 5 | 473ns | -44.1% | +1.1% |
| 1 | 6 | 414ns | -27.3% | +5.0% |
| 1 | 7 | 410ns | -26.5% | +17.0% |
| 1 | 8 | 471ns | -39.2% | -8.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_form_float | -0.444 | moderate- |
| ema_form_intshift | -0.216 | moderate- |
| ema_form_q32 | -0.445 | moderate- |

**Consistency summary:**

- **ema_form_intshift**: won 8/8, lost 0/8
- **ema_form_q32**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_form_float | 2.7ns | 442.0ns | 0.6% |  |
| ema_form_intshift | 2.8ns | 288.0ns | 1.0% |  |
| ema_form_q32 | 2.9ns | 461.3ns | 0.6% |  |

## Distribution (algo ns)

```
ema_form_float (n=160, range 408.4-477.6 ns)
    408.4 |########################################
    411.9 |#########
    415.3 |####
    418.8 |######
    422.3 |######
    425.7 |#####
    429.2 |#
    432.6 |#
    436.1 |##
    439.5 |########
    443.0 |#
    446.5 |
    449.9 |#
    453.4 |
    456.8 |
    460.3 |##
    463.7 |#
    467.2 |######
    470.7 |###########################
    474.1 |##################################
  (10 below, 3 above range)

ema_form_intshift (n=160, range 260.6-309.2 ns)
    260.6 |##########################
    263.1 |##########
    265.5 |############
    267.9 |###
    270.4 |#####
    272.8 |######
    275.2 |########
    277.6 |##########
    280.1 |#####
    282.5 |#####
    284.9 |#####
    287.4 |###
    289.8 |
    292.2 |##########
    294.7 |##########
    297.1 |#################
    299.5 |##########################
    301.9 |########################################
    304.4 |#################################
    306.8 |##########
  (12 below, 4 above range)

ema_form_q32 (n=160, range 427.6-486.2 ns)
    427.6 |#########
    430.5 |############
    433.4 |########
    436.4 |#####
    439.3 |#######
    442.2 |#########
    445.2 |#######
    448.1 |#######
    451.0 |##
    454.0 |
    456.9 |
    459.8 |#
    462.8 |##
    465.7 |
    468.6 |###
    471.6 |#########
    474.5 |#########
    477.4 |#####################
    480.4 |########################################
    483.3 |###############
  (10 below, 4 above range)

```
