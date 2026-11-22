# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (1.61 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 293 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 30% faster than the next best (ema_scalar)

ema_neon (293 ns) leads ema_scalar (380 ns) by 30%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 82% (significant)

ema_neon is -1.32 us (82%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 5.5x slower than the field

ema_autovec (1.61 us) is 5.5x the fastest (293 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.5x the fastest

Fastest ema_neon (293 ns) to slowest ema_autovec (1.61 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 293.3 ns median (-81.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.50x (fastest 293.3 ns, slowest 1612.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | EMA streaming pass: NEON vs scalar vs autovec under realistic workload |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ema_autovec | 3723ns | 3704ns | 3696ns | 3712ns | 3782ns | base |
| ema_neon | 2447ns | 2432ns | 2425ns | 2440ns | 2493ns | -34.26% |
| ema_scalar | 2482ns | 2462ns | 2455ns | 2475ns | 2530ns | -33.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 1619ns | 1605ns | 1648ns | base | 2.529 |
| ema_neon | 295ns | 290ns | 303ns | -81.80% | 13.898 |
| ema_scalar | 380ns | 373ns | 390ns | -76.52% | 10.773 |

## Performance model

- Peak throughput: **14.136 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.541 | 18.0% |
| ema_neon | 13.965 | 98.8% |
| ema_scalar | 10.779 | 76.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 3723ns | 3723ns | base |
| ema_neon | 2447ns | 2447ns | -34.26% |
| ema_scalar | 2482ns | 2482ns | -33.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 1612ns | base | --- | [1611, 1614] | --- | --- | --- | --- |
| ema_neon | 293ns | -1318.1ns (-81.8%) | [-1320, -1317]ns | [293, 294] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 380ns | -1233.4ns (-76.5%) | [-1234, -1232]ns | [379, 381] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 1631ns | -82.1% | -77.0% |
| 1 | 2 | 1634ns | -82.1% | -76.5% |
| 1 | 3 | 1611ns | -81.6% | -76.4% |
| 1 | 4 | 1610ns | -81.8% | -76.3% |
| 1 | 5 | 1608ns | -81.6% | -76.5% |
| 1 | 6 | 1613ns | -81.6% | -76.6% |
| 1 | 7 | 1614ns | -81.9% | -76.4% |
| 1 | 8 | 1609ns | -81.8% | -76.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | 0.426 | moderate+ |
| ema_neon | 0.024 | ok |
| ema_scalar | -0.215 | moderate- |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.7ns | 1619.3ns | 0.2% |  |
| ema_neon | 2.7ns | 294.7ns | 0.9% |  |
| ema_scalar | 2.1ns | 380.2ns | 0.6% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 1604.6-1648.4 ns)
   1604.6 |######################
   1606.8 |####################################
   1609.0 |####################
   1611.2 |########################################
   1613.4 |###################
   1615.5 |######
   1617.7 |####
   1619.9 |###
   1622.1 |####
   1624.3 |#
   1626.5 |#
   1628.7 |########
   1630.9 |####
   1633.0 |###
   1635.2 |#
   1637.4 |############
   1639.6 |############
   1641.8 |######
   1644.0 |####
   1646.2 |#
  (17 below, 7 above range)

ema_neon (n=160, range 289.8-302.8 ns)
    289.8 |#################
    290.4 |######################
    291.1 |############################
    291.7 |###########
    292.4 |########################################
    293.0 |#########
    293.7 |##############################
    294.3 |#########
    295.0 |###########
    295.6 |###############
    296.3 |#################
    296.9 |###################
    297.6 |#
    298.2 |########################
    298.9 |#######
    299.5 |###
    300.2 |#####
    300.8 |
    301.5 |
    302.1 |
  (13 below, 2 above range)

ema_scalar (n=160, range 372.6-389.7 ns)
    372.6 |###########
    373.5 |####################
    374.3 |###############################
    375.2 |#########################
    376.0 |##############
    376.9 |####################
    377.7 |###########
    378.6 |####################
    379.5 |########################################
    380.3 |########################################
    381.2 |############################
    382.0 |####################
    382.9 |#########################
    383.7 |#################
    384.6 |#################
    385.4 |###########
    386.3 |####################
    387.1 |########
    388.0 |##
    388.9 |#################
  (16 below, 3 above range)

```
