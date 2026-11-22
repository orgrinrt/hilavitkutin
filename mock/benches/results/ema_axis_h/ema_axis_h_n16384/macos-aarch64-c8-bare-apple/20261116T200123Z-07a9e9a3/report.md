# EMA streaming pass: NEON vs scalar vs autovec under realistic workload

3 variants, 160 samples per variant.
Baseline: **ema_autovec**

## Highlights

Baseline for all deltas below: **ema_autovec**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (ema_autovec) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline ema_autovec has the worst median (6.53 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest ema_neon at 1.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### ema_neon beats baseline by 77% (significant)

ema_neon is -5.03 us (77%) faster than baseline ema_autovec, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### ema_autovec is an outlier: 4.4x slower than the field

ema_autovec (6.53 us) is 4.4x the fastest (1.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ema_scalar shows alternating (throttle bounce) (autocorr -0.74)

ema_scalar's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.4x the fastest

Fastest ema_neon (1.49 us) to slowest ema_autovec (6.53 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 1492.5 ns median (-77.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.37x (fastest 1492.5 ns, slowest 6527.1 ns)

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
| ema_autovec | 8637ns | 8620ns | 8609ns | 8625ns | 8700ns | base |
| ema_neon | 3654ns | 3628ns | 3621ns | 3633ns | 3747ns | -57.70% |
| ema_scalar | 3668ns | 3657ns | 3652ns | 3658ns | 3713ns | -57.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 6540ns | 6517ns | 6596ns | base | 2.505 |
| ema_neon | 1501ns | 1488ns | 1540ns | -77.04% | 10.913 |
| ema_scalar | 1580ns | 1570ns | 1600ns | -75.84% | 10.368 |

## Performance model

- Peak throughput: **11.013 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.510 | 22.8% |
| ema_neon | 10.978 | 99.7% |
| ema_scalar | 10.390 | 94.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 8637ns | 8637ns | base |
| ema_neon | 3654ns | 3654ns | -57.70% |
| ema_scalar | 3668ns | 3668ns | -57.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 6527ns | base | --- | [6526, 6529] | --- | --- | --- | --- |
| ema_neon | 1492ns | -5034.8ns (-77.1%) | [-5036, -5032]ns | [1491, 1493] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 1577ns | -4950.0ns (-75.8%) | [-4951, -4948]ns | [1575, 1578] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 6526ns | -77.1% | -75.8% |
| 1 | 2 | 6528ns | -77.2% | -75.8% |
| 1 | 3 | 6526ns | -77.2% | -75.9% |
| 1 | 4 | 6544ns | -77.2% | -75.9% |
| 1 | 5 | 6530ns | -76.3% | -75.9% |
| 1 | 6 | 6526ns | -77.2% | -75.8% |
| 1 | 7 | 6525ns | -77.1% | -75.8% |
| 1 | 8 | 6528ns | -77.2% | -75.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.044 | ok |
| ema_neon | -0.145 | ok |
| ema_scalar | -0.738 | HIGH- (thermal bounce) |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.9ns | 6540.1ns | 0.0% |  |
| ema_neon | 2.9ns | 1501.3ns | 0.2% |  |
| ema_scalar | 2.5ns | 1580.2ns | 0.2% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 6517.1-6596.1 ns)
   6517.1 |#############################
   6521.1 |#############################
   6525.0 |########################################
   6529.0 |#######################
   6532.9 |####
   6536.9 |####
   6540.8 |####
   6544.8 |#
   6548.7 |##
   6552.7 |#
   6556.6 |##
   6560.6 |##
   6564.5 |##
   6568.5 |######
   6572.4 |##
   6576.4 |#
   6580.3 |##
   6584.3 |###
   6588.2 |
   6592.2 |##
  (12 below, 11 above range)

ema_neon (n=160, range 1487.7-1539.7 ns)
   1487.7 |##################################
   1490.3 |########################################
   1492.9 |###################
   1495.5 |####
   1498.1 |
   1500.7 |##
   1503.3 |
   1505.9 |
   1508.5 |##
   1511.1 |
   1513.7 |##
   1516.3 |#
   1518.9 |###
   1521.5 |
   1524.1 |
   1526.7 |
   1529.3 |
   1531.9 |
   1534.5 |
   1537.1 |
  (13 below, 21 above range)

ema_scalar (n=160, range 1570.1-1600.3 ns)
   1570.1 |####################
   1571.6 |#######################
   1573.1 |########################################
   1574.6 |#####################
   1576.1 |###########################
   1577.6 |#######################
   1579.1 |#############################
   1580.6 |################
   1582.2 |
   1583.7 |#######
   1585.2 |#######
   1586.7 |#######
   1588.2 |#
   1589.7 |#
   1591.2 |#
   1592.7 |#
   1594.3 |
   1595.8 |
   1597.3 |
   1598.8 |###
  (15 below, 16 above range)

```
