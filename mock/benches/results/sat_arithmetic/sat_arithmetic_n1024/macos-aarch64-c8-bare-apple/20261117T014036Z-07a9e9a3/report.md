# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (655 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 351 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 87% faster than the next best (sat_saturate)

sat_wrap (351 ns) leads sat_saturate (655 ns) by 87%, a clear separation rather than a photo finish. CV 4.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 47% (significant)

sat_wrap is -307 ns (47%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap shows alternating (throttle bounce) (autocorr -0.73)

sat_wrap's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: sat_wrap** at 350.8 ns median (-46.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.87x (fastest 350.8 ns, slowest 655.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Overflow strategy: wrapping_add vs saturating_add vs checked_add |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x52b9de7c318fa064 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 2774ns | 2744ns | 2724ns | 2760ns | 2868ns | base |
| sat_saturate | 2755ns | 2730ns | 2720ns | 2743ns | 2824ns | -0.70% |
| sat_wrap | 2490ns | 2497ns | 2414ns | 2481ns | 2594ns | -10.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 663ns | 647ns | 692ns | base | 1.544 |
| sat_saturate | 659ns | 647ns | 682ns | -0.63% | 1.553 |
| sat_wrap | 355ns | 341ns | 377ns | -46.56% | 2.888 |

## Performance model

- Peak throughput: **3.000 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.563 | 52.1% |
| sat_saturate | 1.564 | 52.1% |
| sat_wrap | 2.919 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2774ns | 2774ns | base |
| sat_saturate | 2755ns | 2755ns | -0.70% |
| sat_wrap | 2490ns | 2490ns | -10.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 655ns | base | --- | [654, 656] | --- | --- | --- | --- |
| sat_saturate | 655ns | no significant difference | [-2, +0]ns | [653, 655] | no | 0.0750 | 0.0750 | 7 |
| sat_wrap | 351ns | -307.1ns (-46.9%) | [-309, -306]ns | [349, 352] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 655ns | -0.4% | -47.4% |
| 1 | 2 | 684ns | -4.6% | -46.9% |
| 1 | 3 | 653ns | +0.6% | -47.0% |
| 1 | 4 | 654ns | -0.4% | -45.7% |
| 1 | 5 | 655ns | -0.4% | -46.5% |
| 1 | 6 | 653ns | +0.1% | -45.8% |
| 1 | 7 | 652ns | +1.5% | -47.0% |
| 1 | 8 | 677ns | -0.2% | -47.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.219 | moderate- |
| sat_saturate | 0.302 | moderate+ |
| sat_wrap | -0.733 | HIGH- (thermal bounce) |

**Consistency summary:**

- **sat_saturate**: won 5/8, lost 2/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 3.1ns | 663.4ns | 0.5% |  |
| sat_saturate | 3.2ns | 659.2ns | 0.5% |  |
| sat_wrap | 2.6ns | 354.5ns | 0.7% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 647.3-692.5 ns)
    647.3 |################
    649.6 |#################
    651.8 |################################
    654.1 |########################################
    656.3 |#########
    658.6 |#
    660.8 |
    663.1 |#
    665.4 |##
    667.6 |#####
    669.9 |######
    672.1 |##
    674.4 |####
    676.7 |#########
    678.9 |####
    681.2 |######
    683.4 |##
    685.7 |########
    687.9 |####
    690.2 |########
  (16 below, 8 above range)

sat_saturate (n=160, range 647.1-682.3 ns)
    647.1 |##########################
    648.9 |########################
    650.6 |##########################
    652.4 |########################################
    654.1 |####################################
    655.9 |########################
    657.7 |############
    659.4 |####
    661.2 |########
    662.9 |####
    664.7 |##########
    666.4 |####
    668.2 |############
    670.0 |##
    671.7 |
    673.5 |########
    675.2 |########
    677.0 |##########
    678.7 |##
    680.5 |
  (18 below, 12 above range)

sat_wrap (n=160, range 341.3-377.4 ns)
    341.3 |#######################
    343.1 |################
    344.9 |########################################
    346.7 |###########################
    348.5 |##################
    350.3 |#############################
    352.1 |#############################
    353.9 |##############
    355.7 |################
    357.5 |##########
    359.4 |##############
    361.2 |##############
    363.0 |####
    364.8 |##########
    366.6 |######
    368.4 |####
    370.2 |
    372.0 |
    373.8 |
    375.6 |##
  (14 below, 11 above range)

```
