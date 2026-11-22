# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 93% faster than the next best (sat_checked)

sat_wrap (1.33 us) leads sat_checked (2.57 us) by 93%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 49% (significant)

sat_wrap is -1.26 us (49%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: sat_wrap** at 1335.0 ns median (-48.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.93x (fastest 1335.0 ns, slowest 2576.9 ns)

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
| sat_checked | 4734ns | 4655ns | 4645ns | 4701ns | 4922ns | base |
| sat_saturate | 4749ns | 4685ns | 4648ns | 4721ns | 4935ns | +0.32% |
| sat_wrap | 3527ns | 3496ns | 3390ns | 3509ns | 3714ns | -25.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 2614ns | 2565ns | 2724ns | base | 1.567 |
| sat_saturate | 2622ns | 2565ns | 2733ns | +0.32% | 1.562 |
| sat_wrap | 1359ns | 1305ns | 1451ns | -48.01% | 3.014 |

## Performance model

- Peak throughput: **3.138 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.592 | 50.7% |
| sat_saturate | 1.590 | 50.6% |
| sat_wrap | 3.068 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 4734ns | 4734ns | base |
| sat_saturate | 4749ns | 4749ns | +0.32% |
| sat_wrap | 3527ns | 3527ns | -25.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 2573ns | base | --- | [2571, 2575] | --- | --- | --- | --- |
| sat_saturate | 2577ns | no significant difference | [-0, +3]ns | [2574, 2605] | no | 0.2948 | 0.2948 | 6 |
| sat_wrap | 1335ns | -1255.7ns (-48.8%) | [-1264, -1245]ns | [1329, 1355] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 2658ns | -3.3% | -50.2% |
| 1 | 2 | 2568ns | +0.2% | -47.2% |
| 1 | 3 | 2693ns | -2.8% | -50.6% |
| 1 | 4 | 2570ns | -0.0% | -48.5% |
| 1 | 5 | 2569ns | +4.6% | -46.8% |
| 1 | 6 | 2570ns | +0.0% | -47.2% |
| 1 | 7 | 2570ns | +2.4% | -49.2% |
| 1 | 8 | 2711ns | +0.0% | -48.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.271 | moderate- |
| sat_saturate | -0.164 | ok |
| sat_wrap | -0.454 | moderate- |

**Consistency summary:**

- **sat_saturate**: won 2/8, lost 3/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 2.8ns | 2613.7ns | 0.1% |  |
| sat_saturate | 3.0ns | 2622.0ns | 0.1% |  |
| sat_wrap | 2.3ns | 1359.0ns | 0.2% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 2564.8-2723.6 ns)
   2564.8 |########################################
   2572.7 |#############
   2580.7 |#
   2588.6 |
   2596.5 |
   2604.5 |#
   2612.4 |#
   2620.4 |
   2628.3 |#
   2636.2 |
   2644.2 |
   2652.1 |#######
   2660.1 |#####
   2668.0 |#
   2675.9 |
   2683.9 |##
   2691.8 |###
   2699.8 |#
   2707.7 |#
   2715.6 |#
  (17 below, 12 above range)

sat_saturate (n=160, range 2565.4-2733.1 ns)
   2565.4 |########################################
   2573.8 |##########
   2582.2 |####
   2590.6 |##
   2599.0 |###
   2607.3 |##
   2615.7 |####
   2624.1 |###
   2632.5 |
   2640.9 |#
   2649.2 |#
   2657.6 |
   2666.0 |##
   2674.4 |
   2682.8 |###
   2691.2 |
   2699.5 |######
   2707.9 |########
   2716.3 |#######
   2724.7 |#
  (16 below, 7 above range)

sat_wrap (n=160, range 1305.1-1451.0 ns)
   1305.1 |########################################
   1312.4 |#############################
   1319.7 |###############################
   1327.0 |###################################
   1334.3 |##########
   1341.6 |############
   1348.9 |######
   1356.2 |##########
   1363.5 |############
   1370.8 |################
   1378.1 |##########
   1385.4 |##########
   1392.7 |##################
   1400.0 |###########################
   1407.2 |##
   1414.5 |##
   1421.8 |####
   1429.1 |##
   1436.4 |####
   1443.7 |##
  (14 below, 8 above range)

```
