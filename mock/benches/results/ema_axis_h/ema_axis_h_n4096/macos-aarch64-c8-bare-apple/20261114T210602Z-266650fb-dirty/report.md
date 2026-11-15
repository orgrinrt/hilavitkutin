# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (1.70 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 310 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon dominates: 27% faster than the next best (ema_scalar)

ema_neon (310 ns) leads ema_scalar (395 ns) by 27%, a clear separation rather than a photo finish. CV 6.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ema_neon beats baseline by 81% (significant)

ema_neon is -1.38 us (81%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 5.5x slower than the field

ema_autovec (1.70 us) is 5.5x the fastest (310 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_neon is fastest but the noisiest (CV 6.6%)

ema_neon wins on median (310 ns) yet has the highest variance (CV 6.6%), while ema_scalar is the steadiest (CV 4.7%, 395 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.5x the fastest

Fastest ema_neon (310 ns) to slowest ema_autovec (1.70 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 310.4 ns median (-81.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.47x (fastest 310.4 ns, slowest 1696.5 ns)

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
| ema_autovec | 3921ns | 3900ns | 3691ns | 3883ns | 4264ns | base |
| ema_neon | 2586ns | 2586ns | 2416ns | 2568ns | 2810ns | -34.05% |
| ema_scalar | 2582ns | 2580ns | 2450ns | 2566ns | 2760ns | -34.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 1709ns | 1607ns | 1863ns | base | 2.397 |
| ema_neon | 313ns | 291ns | 342ns | -81.70% | 13.093 |
| ema_scalar | 396ns | 374ns | 423ns | -76.85% | 10.352 |

## Performance model

- Peak throughput: **14.053 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.414 | 17.2% |
| ema_neon | 13.196 | 93.9% |
| ema_scalar | 10.370 | 73.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 3921ns | 3921ns | base |
| ema_neon | 2586ns | 2586ns | -34.05% |
| ema_scalar | 2582ns | 2582ns | -34.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 1696ns | base | --- | [1638, 1723] | --- | --- | --- | --- |
| ema_neon | 310ns | -1381.5ns (-81.4%) | [-1411, -1331]ns | [305, 313] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 395ns | -1306.0ns (-77.0%) | [-1323, -1252]ns | [389, 400] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 1615ns | -81.6% | -76.6% |
| 1 | 2 | 1746ns | -82.0% | -75.4% |
| 1 | 3 | 1841ns | -83.8% | -78.6% |
| 1 | 4 | 1726ns | -81.7% | -78.0% |
| 1 | 5 | 1613ns | -80.5% | -74.8% |
| 1 | 6 | 1854ns | -83.1% | -79.0% |
| 1 | 7 | 1767ns | -83.3% | -77.3% |
| 1 | 8 | 1614ns | -79.1% | -76.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.188 | ok |
| ema_neon | -0.433 | moderate- |
| ema_scalar | -0.459 | moderate- |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.8ns | 1709.0ns | 0.2% |  |
| ema_neon | 2.9ns | 312.8ns | 0.9% |  |
| ema_scalar | 2.8ns | 395.7ns | 0.7% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 1607.1-1862.8 ns)
   1607.1 |########################################
   1619.9 |############
   1632.6 |######
   1645.4 |#
   1658.2 |###
   1671.0 |#
   1683.8 |####
   1696.6 |#
   1709.4 |##########
   1722.1 |######################
   1734.9 |###
   1747.7 |
   1760.5 |#
   1773.3 |#
   1786.1 |
   1798.9 |#
   1811.6 |#
   1824.4 |#
   1837.2 |###
   1850.0 |#######################
  (14 below, 13 above range)

ema_neon (n=160, range 291.5-341.8 ns)
    291.5 |########################################
    294.0 |####################
    296.5 |################
    299.0 |###
    301.5 |#########
    304.0 |################
    306.6 |#####
    309.1 |##############
    311.6 |#######################
    314.1 |###########################
    316.6 |#
    319.2 |
    321.7 |
    324.2 |#
    326.7 |
    329.2 |#####
    331.7 |#
    334.3 |#####################
    336.8 |######################################
    339.3 |##########
  (15 below, 3 above range)

ema_scalar (n=160, range 374.0-423.3 ns)
    374.0 |##########
    376.5 |########################################
    379.0 |#####################
    381.4 |##############
    383.9 |##########
    386.4 |############
    388.8 |########
    391.3 |############
    393.7 |######
    396.2 |####
    398.7 |###########################
    401.1 |#########################
    403.6 |##################
    406.0 |###############################
    408.5 |##########
    411.0 |####
    413.4 |########
    415.9 |##
    418.4 |
    420.8 |
  (16 below, 16 above range)

```
