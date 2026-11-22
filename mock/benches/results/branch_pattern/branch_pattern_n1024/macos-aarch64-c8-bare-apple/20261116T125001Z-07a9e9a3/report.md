# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (1.61 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 1.61 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 1.61 us to slowest 1.61 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### branch_cmov shows alternating (throttle bounce) (autocorr -0.74)

branch_cmov's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (7 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: branch_if** at 1611.0 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 1611.0 ns, slowest 1611.2 ns)

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
| branch_cmov | 3715ns | 3701ns | 3691ns | 3701ns | 3782ns | base |
| branch_if | 3705ns | 3700ns | 3690ns | 3700ns | 3737ns | -0.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 1617ns | 1606ns | 1647ns | base | 0.633 |
| branch_if | 1612ns | 1606ns | 1620ns | -0.34% | 0.635 |

## Performance model

- Peak throughput: **0.638 Gops/s** (branch_if; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.636 | 99.6% |
| branch_if | 0.636 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 3715ns | 3715ns | base |
| branch_if | 3705ns | 3705ns | -0.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 1611ns | base | --- | [1611, 1612] | --- | --- | --- | --- |
| branch_if | 1611ns | no significant difference | [-1, +1]ns | [1610, 1612] | no | 0.4219 | 0.4219 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 1611ns | +0.0% |
| 1 | 2 | 1611ns | -0.1% |
| 1 | 3 | 1611ns | +0.0% |
| 1 | 4 | 1611ns | -0.0% |
| 1 | 5 | 1611ns | -0.0% |
| 1 | 6 | 1611ns | -0.0% |
| 1 | 7 | 1611ns | +0.1% |
| 1 | 8 | 1611ns | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.736 | HIGH- (thermal bounce) |
| branch_if | -0.538 | HIGH- (thermal bounce) |

**Consistency summary:**

- **branch_if**: won 0/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.5ns | 1617.2ns | 0.2% |  |
| branch_if | 2.7ns | 1611.8ns | 0.2% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 1605.9-1646.8 ns)
   1605.9 |###############
   1608.0 |########################
   1610.0 |########################################
   1612.1 |####################
   1614.1 |########
   1616.1 |####
   1618.2 |##
   1620.2 |#
   1622.3 |
   1624.3 |
   1626.4 |
   1628.4 |#
   1630.5 |
   1632.5 |
   1634.5 |
   1636.6 |
   1638.6 |
   1640.7 |
   1642.7 |
   1644.8 |
  (13 below, 9 above range)

branch_if (n=160, range 1605.5-1619.9 ns)
   1605.5 |################
   1606.2 |##############
   1607.0 |###########
   1607.7 |##################
   1608.4 |###########
   1609.1 |########################################
   1609.8 |################################
   1610.6 |########################################
   1611.3 |##
   1612.0 |################################
   1612.7 |###################################
   1613.4 |#######################
   1614.2 |#########################
   1614.9 |###########
   1615.6 |#######
   1616.3 |
   1617.0 |##
   1617.7 |##
   1618.5 |##
   1619.2 |
  (11 below, 8 above range)

```
