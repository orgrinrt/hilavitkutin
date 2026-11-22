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

### ema_neon shows alternating (throttle bounce) (autocorr -0.70)

ema_neon's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.4x the fastest

Fastest ema_neon (1.49 us) to slowest ema_autovec (6.53 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ema_neon** at 1490.8 ns median (-77.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.38x (fastest 1490.8 ns, slowest 6525.2 ns)

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
| ema_autovec | 8627ns | 8617ns | 8607ns | 8618ns | 8677ns | base |
| ema_neon | 3637ns | 3628ns | 3622ns | 3628ns | 3680ns | -57.84% |
| ema_scalar | 3662ns | 3658ns | 3652ns | 3658ns | 3685ns | -57.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ema_autovec | 6531ns | 6516ns | 6563ns | base | 2.509 |
| ema_neon | 1494ns | 1488ns | 1509ns | -77.13% | 10.967 |
| ema_scalar | 1577ns | 1570ns | 1588ns | -75.85% | 10.388 |

## Performance model

- Peak throughput: **11.013 Gops/s** (ema_neon; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ema_autovec | 2.511 | 22.8% |
| ema_neon | 10.990 | 99.8% |
| ema_scalar | 10.403 | 94.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ema_autovec | 8627ns | 8627ns | base |
| ema_neon | 3637ns | 3637ns | -57.84% |
| ema_scalar | 3662ns | 3662ns | -57.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ema_autovec | 6525ns | base | --- | [6524, 6527] | --- | --- | --- | --- |
| ema_neon | 1491ns | -5033.4ns (-77.1%) | [-5035, -5032]ns | [1490, 1491] | YES | 0.0000 | 0.0000 | 0 |
| ema_scalar | 1575ns | -4949.1ns (-75.8%) | [-4950, -4948]ns | [1574, 1577] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ema_autovec | ema_neon | ema_scalar |
|---|---|---|---|---|
| 1 | 1 | 6524ns | -77.1% | -75.9% |
| 1 | 2 | 6525ns | -77.2% | -75.9% |
| 1 | 3 | 6524ns | -77.1% | -75.8% |
| 1 | 4 | 6527ns | -77.2% | -75.9% |
| 1 | 5 | 6526ns | -77.2% | -75.9% |
| 1 | 6 | 6522ns | -77.2% | -75.9% |
| 1 | 7 | 6527ns | -77.1% | -75.8% |
| 1 | 8 | 6525ns | -77.2% | -75.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ema_autovec | -0.344 | moderate- |
| ema_neon | -0.704 | HIGH- (thermal bounce) |
| ema_scalar | -0.064 | ok |

**Consistency summary:**

- **ema_neon**: won 8/8, lost 0/8
- **ema_scalar**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ema_autovec | 2.8ns | 6531.0ns | 0.0% |  |
| ema_neon | 2.9ns | 1494.0ns | 0.2% |  |
| ema_scalar | 2.5ns | 1577.1ns | 0.2% |  |

## Distribution (algo ns)

```
ema_autovec (n=160, range 6516.1-6562.5 ns)
   6516.1 |####################
   6518.4 |#############################
   6520.7 |########################################
   6523.1 |##############################
   6525.4 |####################################
   6527.7 |#############################
   6530.0 |############
   6532.3 |##########
   6534.7 |#########
   6537.0 |#
   6539.3 |#####
   6541.6 |###
   6544.0 |###
   6546.3 |
   6548.6 |#
   6550.9 |
   6553.2 |#
   6555.6 |
   6557.9 |#
   6560.2 |
  (14 below, 15 above range)

ema_neon (n=160, range 1487.7-1509.3 ns)
   1487.7 |#############################
   1488.8 |################
   1489.9 |########################################
   1491.0 |#####################
   1492.0 |#####################
   1493.1 |#######
   1494.2 |###########
   1495.3 |#
   1496.4 |###
   1497.4 |#
   1498.5 |
   1499.6 |#
   1500.7 |#
   1501.7 |##
   1502.8 |
   1503.9 |
   1505.0 |##
   1506.1 |
   1507.1 |
   1508.2 |
  (13 below, 11 above range)

ema_scalar (n=160, range 1570.2-1588.5 ns)
   1570.2 |########################
   1571.1 |##################################
   1572.0 |########################################
   1572.9 |#####################################
   1573.8 |#####################################
   1574.8 |#############
   1575.7 |#####################
   1576.6 |#############
   1577.5 |##########################
   1578.4 |#############################
   1579.3 |#############################
   1580.3 |#############
   1581.2 |##########
   1582.1 |##################
   1583.0 |#####
   1583.9 |
   1584.8 |
   1585.7 |########
   1586.7 |
   1587.6 |#####
  (14 below, 8 above range)

```
