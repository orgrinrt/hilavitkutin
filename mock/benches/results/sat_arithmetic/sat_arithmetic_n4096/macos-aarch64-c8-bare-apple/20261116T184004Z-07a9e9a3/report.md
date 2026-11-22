# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 97% faster than the next best (sat_checked)

sat_wrap (1.33 us) leads sat_checked (2.62 us) by 97%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 49% (significant)

sat_wrap is -1.29 us (49%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: sat_wrap** at 1330.0 ns median (-49.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.97x (fastest 1330.0 ns, slowest 2619.6 ns)

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
| sat_checked | 4749ns | 4741ns | 4735ns | 4742ns | 4783ns | base |
| sat_saturate | 4749ns | 4740ns | 4734ns | 4740ns | 4789ns | -0.00% |
| sat_wrap | 3461ns | 3451ns | 3443ns | 3451ns | 3509ns | -27.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 2623ns | 2613ns | 2645ns | base | 1.561 |
| sat_saturate | 2623ns | 2612ns | 2646ns | -0.01% | 1.562 |
| sat_wrap | 1334ns | 1323ns | 1356ns | -49.16% | 3.071 |

## Performance model

- Peak throughput: **3.096 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.564 | 50.5% |
| sat_saturate | 1.564 | 50.5% |
| sat_wrap | 3.080 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 4749ns | 4749ns | base |
| sat_saturate | 4749ns | 4749ns | -0.00% |
| sat_wrap | 3461ns | 3461ns | -27.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 2620ns | base | --- | [2618, 2620] | --- | --- | --- | --- |
| sat_saturate | 2620ns | no significant difference | [-1, +0]ns | [2618, 2620] | no | 0.2948 | 0.2948 | 6 |
| sat_wrap | 1330ns | -1290.0ns (-49.2%) | [-1291, -1289]ns | [1329, 1331] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 2622ns | -0.1% | -49.3% |
| 1 | 2 | 2618ns | +0.1% | -49.1% |
| 1 | 3 | 2619ns | -0.0% | -49.2% |
| 1 | 4 | 2620ns | -0.0% | -49.3% |
| 1 | 5 | 2618ns | +0.1% | -49.2% |
| 1 | 6 | 2620ns | +0.0% | -49.3% |
| 1 | 7 | 2619ns | -0.0% | -49.2% |
| 1 | 8 | 2619ns | +0.0% | -49.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.452 | moderate- |
| sat_saturate | -0.246 | moderate- |
| sat_wrap | -0.127 | ok |

**Consistency summary:**

- **sat_saturate**: won 1/8, lost 0/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 2.9ns | 2623.2ns | 0.1% |  |
| sat_saturate | 2.8ns | 2623.1ns | 0.1% |  |
| sat_wrap | 2.2ns | 1333.8ns | 0.2% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 2612.6-2645.4 ns)
   2612.6 |#################
   2614.3 |##########################
   2615.9 |#######################
   2617.5 |#################
   2619.2 |########################################
   2620.8 |####################
   2622.4 |################
   2624.1 |###########
   2625.7 |####
   2627.4 |####
   2629.0 |
   2630.6 |#
   2632.3 |#
   2633.9 |
   2635.6 |#
   2637.2 |#
   2638.8 |#
   2640.5 |#
   2642.1 |
   2643.8 |##
  (15 below, 13 above range)

sat_saturate (n=160, range 2612.4-2645.5 ns)
   2612.4 |###################
   2614.0 |##########################
   2615.7 |###########################
   2617.4 |##################################
   2619.0 |########################################
   2620.7 |###############################
   2622.3 |########################
   2624.0 |###############
   2625.6 |###
   2627.3 |#####
   2628.9 |
   2630.6 |#
   2632.3 |###
   2633.9 |
   2635.6 |
   2637.2 |
   2638.9 |
   2640.5 |
   2642.2 |#
   2643.8 |
  (14 below, 11 above range)

sat_wrap (n=160, range 1323.1-1356.4 ns)
   1323.1 |##################
   1324.7 |################
   1326.4 |###########################
   1328.1 |################################
   1329.7 |########################################
   1331.4 |###################################
   1333.1 |##########
   1334.7 |######
   1336.4 |
   1338.1 |
   1339.7 |
   1341.4 |###
   1343.1 |#
   1344.7 |#######
   1346.4 |#
   1348.0 |###
   1349.7 |###
   1351.4 |#
   1353.0 |
   1354.7 |###
  (15 below, 7 above range)

```
