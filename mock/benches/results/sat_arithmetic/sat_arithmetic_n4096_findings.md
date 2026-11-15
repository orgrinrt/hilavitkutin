# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (2.75 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 1.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 98% faster than the next best (sat_saturate)

sat_wrap (1.34 us) leads sat_saturate (2.66 us) by 98%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 49% (significant)

sat_wrap is -1.35 us (49%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_checked is an outlier: 2.0x slower than the field

sat_checked (2.75 us) is 2.0x the fastest (1.34 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sat_saturate's edge over baseline is significant but tiny (-17 ns, 0.61%)

sat_saturate differs from baseline sat_checked by -17 ns (0.61%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: sat_wrap** at 1343.8 ns median (-51.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.05x (fastest 1343.8 ns, slowest 2748.1 ns)

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
| sat_checked | 4868ns | 4973ns | 4656ns | 4889ns | 5013ns | base |
| sat_saturate | 4837ns | 4811ns | 4675ns | 4841ns | 4986ns | -0.64% |
| sat_wrap | 3484ns | 3481ns | 3378ns | 3470ns | 3632ns | -28.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 2692ns | 2576ns | 2773ns | base | 1.522 |
| sat_saturate | 2676ns | 2586ns | 2761ns | -0.61% | 1.531 |
| sat_wrap | 1346ns | 1301ns | 1411ns | -50.01% | 3.044 |

## Performance model

- Peak throughput: **3.148 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.490 | 47.4% |
| sat_saturate | 1.539 | 48.9% |
| sat_wrap | 3.048 | 96.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 4868ns | 4868ns | base |
| sat_saturate | 4837ns | 4837ns | -0.64% |
| sat_wrap | 3484ns | 3484ns | -28.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 2748ns | base | --- | [2671, 2751] | --- | --- | --- | --- |
| sat_saturate | 2662ns | -16.7ns (-0.6%) | [-79, -0]ns | [2657, 2667] | YES | 0.0464 | 0.0464 | 2 |
| sat_wrap | 1344ns | -1354.8ns (-49.3%) | [-1360, -1334]ns | [1332, 1349] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 2583ns | +6.5% | -48.9% |
| 1 | 2 | 2756ns | -3.3% | -51.3% |
| 1 | 3 | 2658ns | -0.9% | -47.4% |
| 1 | 4 | 2755ns | -3.2% | -51.2% |
| 1 | 5 | 2752ns | -3.3% | -52.5% |
| 1 | 6 | 2753ns | -3.6% | -51.0% |
| 1 | 7 | 2751ns | -4.6% | -50.4% |
| 1 | 8 | 2617ns | +3.5% | -50.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.216 | moderate- |
| sat_saturate | -0.085 | ok |
| sat_wrap | -0.119 | ok |

**Consistency summary:**

- **sat_saturate**: won 6/8, lost 2/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 3.0ns | 2692.0ns | 0.1% |  |
| sat_saturate | 2.7ns | 2675.5ns | 0.1% |  |
| sat_wrap | 2.3ns | 1345.7ns | 0.2% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 2575.7-2772.9 ns)
   2575.7 |####
   2585.5 |##
   2595.4 |#
   2605.3 |#####
   2615.1 |######
   2625.0 |#
   2634.8 |##
   2644.7 |#####
   2654.6 |######
   2664.4 |####
   2674.3 |###
   2684.1 |##
   2694.0 |
   2703.9 |
   2713.7 |
   2723.6 |
   2733.4 |
   2743.3 |####################
   2753.2 |########################################
   2763.0 |###
  (21 below, 6 above range)

sat_saturate (n=160, range 2586.1-2760.8 ns)
   2586.1 |###
   2594.9 |###
   2603.6 |###
   2612.3 |############
   2621.1 |######
   2629.8 |#
   2638.5 |
   2647.3 |###########################
   2656.0 |############################
   2664.7 |###############
   2673.5 |#####
   2682.2 |###
   2690.9 |
   2699.7 |##
   2708.4 |##
   2717.1 |
   2725.9 |
   2734.6 |
   2743.3 |##############
   2752.1 |########################################
  (18 below, 9 above range)

sat_wrap (n=160, range 1301.3-1411.0 ns)
   1301.3 |########################################
   1306.8 |##########################
   1312.2 |######
   1317.7 |###
   1323.2 |######
   1328.7 |############
   1334.2 |###
   1339.7 |######################
   1345.2 |##########################
   1350.7 |#################
   1356.1 |##########
   1361.6 |######
   1367.1 |######
   1372.6 |#
   1378.1 |#
   1383.6 |
   1389.1 |########
   1394.6 |#################
   1400.0 |########################
   1405.5 |#
  (15 below, 4 above range)

```
