# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (6.77 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 1.59 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon beats baseline by 77% (significant)

ema_neon is -5.23 us (77%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 4.3x slower than the field

ema_autovec (6.77 us) is 4.3x the fastest (1.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_neon shows warm-up / thermal drift (autocorr +0.52)

ema_neon's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.3x the fastest

Fastest ema_neon (1.59 us) to slowest ema_autovec (6.77 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 1589.8 ns median (-76.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.26x (fastest 1589.8 ns, slowest 6769.6 ns)

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
| ema_autovec | 8976ns | 8920ns | 8630ns | 8978ns | 9315ns | base |
| ema_neon | 3785ns | 3855ns | 3618ns | 3778ns | 3974ns | -57.83% |
| ema_scalar | 3913ns | 3912ns | 3681ns | 3912ns | 4149ns | -56.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 6807ns | 6541ns | 7062ns | base | 2.407 |
| ema_neon | 1560ns | 1490ns | 1637ns | -77.08% | 10.504 |
| ema_scalar | 1688ns | 1588ns | 1789ns | -75.20% | 9.706 |

## Performance model

- Peak throughput: **10.998 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.420 | 22.0% |
| ema_neon | 10.306 | 93.7% |
| ema_scalar | 9.699 | 88.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 8976ns | 8976ns | base |
| ema_neon | 3785ns | 3785ns | -57.83% |
| ema_scalar | 3913ns | 3913ns | -56.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 6770ns | base | --- | [6758, 6801] | --- | --- | --- | --- |
| ema_neon | 1590ns | -5232.1ns (-77.3%) | [-5277, -5174]ns | [1542, 1597] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 1689ns | -5082.7ns (-75.1%) | [-5154, -5068]ns | [1688, 1691] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 6692ns | -77.7% | -74.8% |
| 1 | 2 | 6958ns | -78.6% | -75.7% |
| 1 | 3 | 6775ns | -78.0% | -74.9% |
| 1 | 4 | 6774ns | -76.4% | -75.0% |
| 1 | 5 | 6983ns | -77.1% | -75.9% |
| 1 | 6 | 6869ns | -76.7% | -75.4% |
| 1 | 7 | 6919ns | -77.3% | -75.5% |
| 1 | 8 | 6554ns | -75.6% | -75.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.277 | moderate- |
| ema_neon | 0.524 | HIGH+ (drift/warm-up) |
| ema_scalar | -0.039 | ok |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 3.1ns | 6806.8ns | 0.0% |  |
| ema_neon | 3.0ns | 1559.8ns | 0.2% |  |
| ema_scalar | 2.6ns | 1688.1ns | 0.2% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 6541.2-7062.3 ns)
   6541.2 |#########
   6567.3 |#########
   6593.3 |####
   6619.4 |#####
   6645.4 |####
   6671.5 |###
   6697.5 |#
   6723.6 |########
   6749.7 |#############################
   6775.7 |###########
   6801.8 |#
   6827.8 |###
   6853.9 |##
   6879.9 |##
   6906.0 |##
   6932.0 |#
   6958.1 |##
   6984.1 |########################################
   7010.2 |##########
   7036.2 |#####
  (18 below, 6 above range)

ema_neon (n=160, range 1489.7-1636.6 ns)
   1489.7 |###############################
   1497.0 |#######
   1504.3 |
   1511.7 |#
   1519.0 |#
   1526.4 |
   1533.7 |##
   1541.1 |#
   1548.4 |###
   1555.8 |
   1563.1 |##
   1570.5 |
   1577.8 |
   1585.1 |#
   1592.5 |########################################
   1599.8 |######################
   1607.2 |
   1614.5 |
   1621.9 |
   1629.2 |
  (17 below, 12 above range)

ema_scalar (n=160, range 1587.9-1789.2 ns)
   1587.9 |##
   1598.0 |#
   1608.0 |
   1618.1 |####
   1628.2 |##
   1638.2 |
   1648.3 |
   1658.4 |
   1668.4 |
   1678.5 |##############################
   1688.6 |########################################
   1698.6 |######
   1708.7 |#
   1718.8 |
   1728.8 |
   1738.9 |#
   1749.0 |
   1759.0 |
   1769.1 |
   1779.2 |
  (21 below, 21 above range)

```

## Diagnostics

- **ema_neon**: autocorrelation=0.52 (measurement drift or warm-up artifact)
