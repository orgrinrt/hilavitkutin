# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (6.53 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 1.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon beats baseline by 77% (significant)

ema_neon is -5.04 us (77%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 4.4x slower than the field

ema_autovec (6.53 us) is 4.4x the fastest (1.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest ema_neon (1.50 us) to slowest ema_autovec (6.53 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 1496.5 ns median (-77.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.36x (fastest 1496.5 ns, slowest 6531.0 ns)

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
| ema_autovec | 8668ns | 8625ns | 8600ns | 8632ns | 8843ns | base |
| ema_neon | 3655ns | 3627ns | 3611ns | 3644ns | 3733ns | -57.83% |
| ema_scalar | 3677ns | 3655ns | 3641ns | 3671ns | 3732ns | -57.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 6570ns | 6518ns | 6709ns | base | 2.494 |
| ema_neon | 1507ns | 1489ns | 1539ns | -77.06% | 10.869 |
| ema_scalar | 1590ns | 1572ns | 1616ns | -75.80% | 10.305 |

## Performance model

- Peak throughput: **11.002 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.509 | 22.8% |
| ema_neon | 10.949 | 99.5% |
| ema_scalar | 10.356 | 94.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 8668ns | 8668ns | base |
| ema_neon | 3655ns | 3655ns | -57.83% |
| ema_scalar | 3677ns | 3677ns | -57.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 6531ns | base | --- | [6529, 6539] | --- | --- | --- | --- |
| ema_neon | 1496ns | -5035.0ns (-77.1%) | [-5039, -5031]ns | [1495, 1501] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 1582ns | -4949.8ns (-75.8%) | [-4955, -4947]ns | [1579, 1588] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 6524ns | -76.5% | -75.8% |
| 1 | 2 | 6531ns | -77.1% | -75.5% |
| 1 | 3 | 6528ns | -77.1% | -75.4% |
| 1 | 4 | 6531ns | -76.8% | -75.9% |
| 1 | 5 | 6529ns | -76.7% | -75.8% |
| 1 | 6 | 6542ns | -76.8% | -75.6% |
| 1 | 7 | 6533ns | -77.2% | -75.6% |
| 1 | 8 | 6558ns | -77.2% | -75.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | 0.020 | ok |
| ema_neon | -0.016 | ok |
| ema_scalar | -0.036 | ok |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 3.3ns | 6569.8ns | 0.0% |  |
| ema_neon | 2.9ns | 1507.4ns | 0.2% |  |
| ema_scalar | 2.7ns | 1590.0ns | 0.2% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 6517.6-6708.8 ns)
   6517.6 |########################################
   6527.2 |##########################
   6536.7 |########
   6546.3 |#######
   6555.8 |###
   6565.4 |####
   6575.0 |####
   6584.5 |##
   6594.1 |##
   6603.7 |#
   6613.2 |#
   6622.8 |
   6632.3 |##
   6641.9 |##
   6651.5 |##
   6661.0 |
   6670.6 |
   6680.2 |
   6689.7 |
   6699.3 |#
  (13 below, 14 above range)

ema_neon (n=160, range 1489.1-1539.4 ns)
   1489.1 |######################
   1491.7 |########################################
   1494.2 |##########################
   1496.7 |############
   1499.2 |#####
   1501.7 |##
   1504.2 |#
   1506.7 |
   1509.2 |
   1511.8 |##
   1514.3 |#####
   1516.8 |###############
   1519.3 |###############################
   1521.8 |#########
   1524.3 |
   1526.8 |
   1529.4 |##
   1531.9 |#
   1534.4 |#
   1536.9 |#
  (16 below, 12 above range)

ema_scalar (n=160, range 1572.3-1615.9 ns)
   1572.3 |####################
   1574.5 |########################################
   1576.7 |######################################
   1578.8 |#########
   1581.0 |##################
   1583.2 |#########
   1585.4 |#######
   1587.6 |#####
   1589.7 |#
   1591.9 |
   1594.1 |#
   1596.3 |###
   1598.5 |
   1600.6 |#######################
   1602.8 |####################
   1605.0 |##########
   1607.2 |#########
   1609.4 |#####################
   1611.5 |#######
   1613.7 |#
  (15 below, 8 above range)

```
