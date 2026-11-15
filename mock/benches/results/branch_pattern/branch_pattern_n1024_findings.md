# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (1.72 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 1.72 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 1.72 us to slowest 1.72 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (80 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: branch_if** at 1723.3 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 1723.3 ns, slowest 1724.0 ns)

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
| branch_cmov | 3920ns | 3943ns | 3690ns | 3893ns | 4234ns | base |
| branch_if | 3915ns | 3945ns | 3688ns | 3899ns | 4190ns | -0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 1710ns | 1609ns | 1851ns | base | 0.599 |
| branch_if | 1707ns | 1609ns | 1826ns | -0.20% | 0.600 |

## Performance model

- Peak throughput: **0.636 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.594 | 93.3% |
| branch_if | 0.594 | 93.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 3920ns | 3920ns | base |
| branch_if | 3915ns | 3915ns | -0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 1724ns | base | --- | [1721, 1726] | --- | --- | --- | --- |
| branch_if | 1723ns | no significant difference | [-1, +16]ns | [1720, 1725] | no | 0.3382 | 0.3382 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 1612ns | +7.0% |
| 1 | 2 | 1790ns | -10.0% |
| 1 | 3 | 1725ns | +0.1% |
| 1 | 4 | 1729ns | -0.1% |
| 1 | 5 | 1724ns | -3.8% |
| 1 | 6 | 1784ns | -9.6% |
| 1 | 7 | 1613ns | +7.2% |
| 1 | 8 | 1613ns | +7.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.046 | ok |
| branch_if | -0.221 | moderate- |

**Consistency summary:**

- **branch_if**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.6ns | 1710.4ns | 0.2% |  |
| branch_if | 2.7ns | 1707.0ns | 0.2% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 1608.9-1850.9 ns)
   1608.9 |#######################
   1621.0 |####
   1633.1 |
   1645.2 |#
   1657.3 |##
   1669.4 |#
   1681.5 |
   1693.6 |
   1705.7 |
   1717.8 |########################################
   1729.9 |####
   1742.0 |
   1754.1 |#
   1766.2 |
   1778.3 |
   1790.4 |
   1802.5 |
   1814.6 |
   1826.7 |#
   1838.8 |#
  (14 below, 25 above range)

branch_if (n=160, range 1609.1-1826.3 ns)
   1609.1 |################
   1620.0 |##
   1630.9 |##
   1641.7 |
   1652.6 |####
   1663.4 |#
   1674.3 |##
   1685.2 |#
   1696.0 |
   1706.9 |#
   1717.7 |########################################
   1728.6 |########
   1739.5 |#
   1750.3 |
   1761.2 |
   1772.0 |##
   1782.9 |
   1793.7 |
   1804.6 |
   1815.5 |
  (19 below, 21 above range)

```
