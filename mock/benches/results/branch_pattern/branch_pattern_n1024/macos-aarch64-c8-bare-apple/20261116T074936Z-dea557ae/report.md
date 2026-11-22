# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (1.61 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### branch_if's edge over baseline is significant but tiny (2 ns, 0.13%)

branch_if differs from baseline branch_cmov by 2 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 1612.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 1612.3 ns, slowest 1635.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branchful if/else vs branchless cmov/csel under data-dependent input |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x76543210fedcba98 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 3723ns | 3697ns | 3675ns | 3710ns | 3810ns | base |
| branch_if | 3734ns | 3746ns | 3676ns | 3732ns | 3799ns | +0.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 1624ns | 1606ns | 1663ns | base | 0.630 |
| branch_if | 1632ns | 1607ns | 1664ns | +0.48% | 0.628 |

## Performance model

- Peak throughput: **0.638 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.635 | 99.6% |
| branch_if | 0.626 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 3723ns | 3723ns | base |
| branch_if | 3734ns | 3734ns | +0.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 1612ns | base | --- | [1611, 1614] | --- | --- | --- | --- |
| branch_if | 1636ns | +2.0ns (+0.1%) | [+0, +6]ns | [1629, 1638] | YES | 0.0046 | 0.0046 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 1651ns | -2.3% |
| 1 | 2 | 1608ns | +1.6% |
| 1 | 3 | 1624ns | +1.1% |
| 1 | 4 | 1612ns | -0.2% |
| 1 | 5 | 1612ns | +1.2% |
| 1 | 6 | 1635ns | +0.2% |
| 1 | 7 | 1610ns | +1.9% |
| 1 | 8 | 1611ns | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.334 | moderate- |
| branch_if | -0.187 | ok |

**Consistency summary:**

- **branch_if**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.5ns | 1624.1ns | 0.2% |  |
| branch_if | 2.7ns | 1631.9ns | 0.2% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 1605.8-1662.6 ns)
   1605.8 |#########################
   1608.7 |########################################
   1611.5 |######################
   1614.4 |###
   1617.2 |#####
   1620.0 |###
   1622.9 |
   1625.7 |###
   1628.6 |#
   1631.4 |###
   1634.2 |########
   1637.1 |##########
   1639.9 |########
   1642.8 |
   1645.6 |####
   1648.4 |
   1651.3 |#
   1654.1 |#
   1657.0 |#
   1659.8 |#
  (15 below, 15 above range)

branch_if (n=160, range 1607.1-1664.4 ns)
   1607.1 |############################
   1610.0 |########################
   1612.8 |#######
   1615.7 |#
   1618.6 |
   1621.4 |##
   1624.3 |###
   1627.2 |##
   1630.0 |##
   1632.9 |##########
   1635.8 |#######################
   1638.6 |########################################
   1641.5 |#######################
   1644.4 |##
   1647.2 |#
   1650.1 |
   1653.0 |##
   1655.8 |#####
   1658.7 |
   1661.6 |
  (13 below, 6 above range)

```
