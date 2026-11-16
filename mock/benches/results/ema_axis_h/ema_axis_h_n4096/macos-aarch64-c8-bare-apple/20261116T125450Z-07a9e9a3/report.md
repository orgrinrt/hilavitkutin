# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (1.61 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 292 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 29% faster than the next best (ema_scalar)

ema_neon (292 ns) leads ema_scalar (377 ns) by 29%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 82% (significant)

ema_neon is -1.32 us (82%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 5.5x slower than the field

ema_autovec (1.61 us) is 5.5x the fastest (292 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.5x the fastest

Fastest ema_neon (292 ns) to slowest ema_autovec (1.61 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 292.5 ns median (-81.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.50x (fastest 292.5 ns, slowest 1609.6 ns)

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
| ema_autovec | 3707ns | 3701ns | 3694ns | 3702ns | 3737ns | base |
| ema_neon | 2438ns | 2430ns | 2424ns | 2431ns | 2477ns | -34.23% |
| ema_scalar | 2462ns | 2459ns | 2454ns | 2459ns | 2481ns | -33.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 1612ns | 1604ns | 1630ns | base | 2.540 |
| ema_neon | 293ns | 289ns | 300ns | -81.80% | 13.961 |
| ema_scalar | 378ns | 372ns | 386ns | -76.56% | 10.836 |

## Performance model

- Peak throughput: **14.155 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.545 | 18.0% |
| ema_neon | 14.003 | 98.9% |
| ema_scalar | 10.868 | 76.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 3707ns | 3707ns | base |
| ema_neon | 2438ns | 2438ns | -34.23% |
| ema_scalar | 2462ns | 2462ns | -33.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 1610ns | base | --- | [1608, 1611] | --- | --- | --- | --- |
| ema_neon | 292ns | -1316.6ns (-81.8%) | [-1317, -1316]ns | [292, 293] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 377ns | -1232.2ns (-76.6%) | [-1232, -1231]ns | [376, 378] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 1610ns | -81.9% | -76.5% |
| 1 | 2 | 1611ns | -81.9% | -76.6% |
| 1 | 3 | 1610ns | -81.8% | -76.6% |
| 1 | 4 | 1608ns | -81.8% | -76.5% |
| 1 | 5 | 1607ns | -81.8% | -76.5% |
| 1 | 6 | 1609ns | -81.9% | -76.6% |
| 1 | 7 | 1610ns | -81.5% | -76.5% |
| 1 | 8 | 1611ns | -81.8% | -76.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | 0.492 | moderate+ |
| ema_neon | -0.039 | ok |
| ema_scalar | -0.242 | moderate- |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.6ns | 1612.3ns | 0.2% |  |
| ema_neon | 2.7ns | 293.4ns | 0.9% |  |
| ema_scalar | 2.1ns | 378.0ns | 0.6% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 1603.7-1629.8 ns)
   1603.7 |########################
   1605.0 |######################################
   1606.3 |################################
   1607.6 |########################
   1608.9 |########################################
   1610.2 |##############################
   1611.5 |##############################
   1612.8 |################
   1614.1 |######################
   1615.4 |####
   1616.7 |######
   1618.0 |##
   1619.3 |####
   1620.7 |####
   1622.0 |##
   1623.3 |##
   1624.6 |####
   1625.9 |
   1627.2 |
   1628.5 |##
  (12 below, 5 above range)

ema_neon (n=160, range 289.4-300.1 ns)
    289.4 |########
    289.9 |#############################
    290.4 |########
    291.0 |###################
    291.5 |########################
    292.0 |########################################
    292.6 |#############
    293.1 |#############
    293.6 |###############
    294.2 |###########################
    294.7 |######
    295.3 |############
    295.8 |###
    296.3 |#####
    296.9 |######
    297.4 |#####
    297.9 |#
    298.5 |###
    299.0 |#
    299.5 |
  (14 below, 3 above range)

ema_scalar (n=160, range 372.1-386.5 ns)
    372.1 |######################
    372.8 |##############################
    373.5 |###################################
    374.2 |#######
    375.0 |###############
    375.7 |########################################
    376.4 |###################################
    377.1 |##########
    377.8 |####################
    378.6 |###########################
    379.3 |##########
    380.0 |####################
    380.7 |#########################
    381.4 |#################
    382.2 |##########
    382.9 |##########
    383.6 |##
    384.3 |##
    385.0 |##
    385.7 |##
  (12 below, 10 above range)

```
