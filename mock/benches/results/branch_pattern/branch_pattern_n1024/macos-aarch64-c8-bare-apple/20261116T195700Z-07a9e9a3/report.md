# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 1.61 us to slowest 1.61 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (13 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (1.61 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 1611.7 ns median
- Spread: 1.00x (fastest 1611.7 ns, slowest 1612.5 ns)

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
| branch_cmov | 3712ns | 3701ns | 3692ns | 3702ns | 3762ns | base |
| branch_if | 3717ns | 3702ns | 3692ns | 3707ns | 3773ns | +0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 1614ns | 1606ns | 1632ns | base | 0.634 |
| branch_if | 1619ns | 1607ns | 1645ns | +0.25% | 0.633 |

## Performance model

- Peak throughput: **0.638 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.635 | 99.6% |
| branch_if | 0.635 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 3712ns | 3712ns | base |
| branch_if | 3717ns | 3717ns | +0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 1612ns | base | --- | [1611, 1612] | --- | --- | --- | --- |
| branch_if | 1612ns | no significant difference | [-0, +2]ns | [1611, 1613] | no | 0.3723 | 0.3723 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 1611ns | +0.0% |
| 1 | 2 | 1625ns | -0.9% |
| 1 | 3 | 1611ns | +1.7% |
| 1 | 4 | 1610ns | +0.2% |
| 1 | 5 | 1611ns | +0.0% |
| 1 | 6 | 1612ns | -0.0% |
| 1 | 7 | 1612ns | -0.1% |
| 1 | 8 | 1611ns | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.168 | ok |
| branch_if | -0.206 | moderate- |

**Consistency summary:**

- **branch_if**: won 1/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.5ns | 1614.5ns | 0.2% |  |
| branch_if | 2.4ns | 1618.6ns | 0.2% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 1605.7-1631.9 ns)
   1605.7 |############
   1607.0 |#############
   1608.3 |#######################
   1609.6 |####################################
   1610.9 |########################################
   1612.2 |##########################
   1613.5 |##################
   1614.8 |################
   1616.2 |####
   1617.5 |#
   1618.8 |#
   1620.1 |
   1621.4 |
   1622.7 |###
   1624.0 |
   1625.3 |###
   1626.6 |###
   1627.9 |
   1629.3 |
   1630.6 |
  (12 below, 15 above range)

branch_if (n=160, range 1606.7-1644.9 ns)
   1606.7 |#####################
   1608.6 |######################################
   1610.5 |##############################
   1612.4 |########################################
   1614.3 |#########
   1616.2 |#
   1618.1 |#
   1620.1 |#
   1622.0 |##
   1623.9 |##
   1625.8 |##
   1627.7 |#
   1629.6 |#
   1631.5 |#
   1633.5 |#####
   1635.4 |##
   1637.3 |#####
   1639.2 |#####
   1641.1 |############
   1643.0 |#####
  (10 below, 6 above range)

```
