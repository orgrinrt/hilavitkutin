# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (1.61 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 295 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 29% faster than the next best (ema_scalar)

ema_neon (295 ns) leads ema_scalar (380 ns) by 29%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 82% (significant)

ema_neon is -1.32 us (82%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 5.5x slower than the field

ema_autovec (1.61 us) is 5.5x the fastest (295 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.5x the fastest

Fastest ema_neon (295 ns) to slowest ema_autovec (1.61 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 295.4 ns median (-81.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.46x (fastest 295.4 ns, slowest 1614.2 ns)

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
| ema_autovec | 3730ns | 3698ns | 3681ns | 3711ns | 3834ns | base |
| ema_neon | 2442ns | 2434ns | 2411ns | 2437ns | 2486ns | -34.53% |
| ema_scalar | 2472ns | 2456ns | 2442ns | 2466ns | 2518ns | -33.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 1627ns | 1603ns | 1673ns | base | 2.518 |
| ema_neon | 295ns | 291ns | 300ns | -81.83% | 13.862 |
| ema_scalar | 381ns | 373ns | 390ns | -76.59% | 10.759 |

## Performance model

- Peak throughput: **14.076 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.537 | 18.0% |
| ema_neon | 13.866 | 98.5% |
| ema_scalar | 10.785 | 76.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 3730ns | 3730ns | base |
| ema_neon | 2442ns | 2442ns | -34.53% |
| ema_scalar | 2472ns | 2472ns | -33.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 1614ns | base | --- | [1611, 1621] | --- | --- | --- | --- |
| ema_neon | 295ns | -1319.2ns (-81.7%) | [-1324, -1317]ns | [295, 296] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 380ns | -1234.8ns (-76.5%) | [-1238, -1232]ns | [378, 381] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 1611ns | -81.6% | -76.4% |
| 1 | 2 | 1610ns | -81.6% | -76.6% |
| 1 | 3 | 1608ns | -81.5% | -76.5% |
| 1 | 4 | 1610ns | -81.8% | -76.5% |
| 1 | 5 | 1671ns | -82.5% | -77.4% |
| 1 | 6 | 1611ns | -81.5% | -76.2% |
| 1 | 7 | 1637ns | -82.1% | -76.7% |
| 1 | 8 | 1628ns | -81.8% | -76.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.217 | moderate- |
| ema_neon | -0.207 | moderate- |
| ema_scalar | 0.216 | moderate+ |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.9ns | 1626.5ns | 0.2% |  |
| ema_neon | 2.9ns | 295.5ns | 1.0% |  |
| ema_scalar | 2.4ns | 380.7ns | 0.6% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 1603.3-1673.2 ns)
   1603.3 |###########################
   1606.8 |########################################
   1610.3 |##############################
   1613.8 |#########
   1617.3 |############
   1620.8 |############
   1624.3 |######
   1627.8 |######
   1631.3 |#######
   1634.8 |############
   1638.2 |###############
   1641.7 |####
   1645.2 |###
   1648.7 |#
   1652.2 |#
   1655.7 |#
   1659.2 |#
   1662.7 |###
   1666.2 |###
   1669.7 |####
  (15 below, 12 above range)

ema_neon (n=160, range 291.0-300.2 ns)
    291.0 |##############
    291.4 |#########
    291.9 |##############
    292.4 |##################
    292.8 |#########
    293.3 |###########
    293.7 |########################################
    294.2 |###########
    294.7 |#######################
    295.1 |#######
    295.6 |##############
    296.0 |#####################
    296.5 |##################
    297.0 |##################
    297.4 |#######
    297.9 |##############################
    298.3 |###########
    298.8 |#########
    299.3 |################
    299.7 |#########
  (14 below, 11 above range)

ema_scalar (n=160, range 373.0-390.3 ns)
    373.0 |#############################
    373.9 |##########
    374.7 |##################
    375.6 |########################
    376.5 |#####################
    377.3 |########################################
    378.2 |#############
    379.1 |########################
    379.9 |##################
    380.8 |################
    381.7 |################################
    382.5 |################
    383.4 |########################
    384.3 |##########
    385.1 |##################
    386.0 |########
    386.9 |########
    387.7 |#############
    388.6 |##
    389.5 |#############
  (12 below, 12 above range)

```
