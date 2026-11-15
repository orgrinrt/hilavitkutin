# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 92% faster than the next best (sat_checked)

sat_wrap (363 ns) leads sat_checked (698 ns) by 92%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 47% (significant)

sat_wrap is -330 ns (47%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap is fastest but the noisiest (CV 5.6%)

sat_wrap wins on median (363 ns) yet has the highest variance (CV 5.6%), while sat_saturate is the steadiest (CV 4.4%, 699 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: sat_wrap** at 363.3 ns median (-47.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.92x (fastest 363.3 ns, slowest 699.0 ns)

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
| sat_checked | 2904ns | 2909ns | 2718ns | 2889ns | 3138ns | base |
| sat_saturate | 2914ns | 2911ns | 2737ns | 2911ns | 3097ns | +0.32% |
| sat_wrap | 2527ns | 2541ns | 2402ns | 2525ns | 2659ns | -12.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 698ns | 652ns | 758ns | base | 1.467 |
| sat_saturate | 699ns | 656ns | 747ns | +0.17% | 1.464 |
| sat_wrap | 363ns | 344ns | 389ns | -47.97% | 2.819 |

## Performance model

- Peak throughput: **2.977 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.468 | 49.3% |
| sat_saturate | 1.465 | 49.2% |
| sat_wrap | 2.819 | 94.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2904ns | 2904ns | base |
| sat_saturate | 2914ns | 2914ns | +0.32% |
| sat_wrap | 2527ns | 2527ns | -12.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 698ns | base | --- | [695, 700] | --- | --- | --- | --- |
| sat_saturate | 699ns | no significant difference | [-2, +1]ns | [697, 700] | no | 0.3755 | 0.3755 | 6 |
| sat_wrap | 363ns | -330.4ns (-47.4%) | [-332, -329]ns | [358, 365] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 667ns | +0.8% | -46.9% |
| 1 | 2 | 653ns | +7.9% | -42.9% |
| 1 | 3 | 702ns | -0.8% | -47.0% |
| 1 | 4 | 699ns | +0.5% | -47.1% |
| 1 | 5 | 752ns | -11.9% | -53.7% |
| 1 | 6 | 658ns | +6.9% | -45.8% |
| 1 | 7 | 751ns | -6.8% | -53.7% |
| 1 | 8 | 700ns | -0.2% | -47.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.280 | moderate- |
| sat_saturate | -0.383 | moderate- |
| sat_wrap | 0.037 | ok |

**Consistency summary:**

- **sat_saturate**: won 4/8, lost 4/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 2.9ns | 698.2ns | 0.4% |  |
| sat_saturate | 3.0ns | 699.4ns | 0.4% |  |
| sat_wrap | 2.6ns | 363.3ns | 0.7% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 651.7-758.4 ns)
    651.7 |########################
    657.0 |###########
    662.4 |##
    667.7 |#######
    673.0 |##
    678.4 |
    683.7 |####
    689.0 |###############
    694.4 |########################################
    699.7 |######################################
    705.0 |#######
    710.4 |##
    715.7 |#
    721.1 |
    726.4 |#
    731.7 |
    737.1 |
    742.4 |##
    747.7 |##########
    753.1 |########################
  (15 below, 7 above range)

sat_saturate (n=160, range 655.7-746.6 ns)
    655.7 |###
    660.2 |######
    664.8 |
    669.3 |##
    673.9 |#
    678.4 |#
    683.0 |##
    687.5 |###
    692.1 |################################
    696.6 |########################################
    701.2 |############################
    705.7 |######
    710.2 |#
    714.8 |#
    719.3 |#
    723.9 |
    728.4 |
    733.0 |
    737.5 |
    742.1 |##
  (20 below, 23 above range)

sat_wrap (n=160, range 344.0-388.7 ns)
    344.0 |####################
    346.2 |################################
    348.5 |###########################
    350.7 |############
    352.9 |###############
    355.2 |############
    357.4 |####################
    359.6 |##########
    361.9 |#################
    364.1 |########################################
    366.3 |##############################
    368.6 |#########################
    370.8 |###########################
    373.1 |#########################
    375.3 |###############
    377.5 |#####
    379.8 |
    382.0 |
    384.2 |
    386.5 |
  (15 below, 11 above range)

```
