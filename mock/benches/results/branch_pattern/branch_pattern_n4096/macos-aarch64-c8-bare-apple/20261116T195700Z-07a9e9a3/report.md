# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (6.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 6.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (8 ns) is smaller than the fastest variant's own run-to-run std-dev (45 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### branch_if's edge over baseline is significant but tiny (-5 ns, 0.07%)

branch_if differs from baseline branch_cmov by -5 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: branch_if** at 6498.8 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 6498.8 ns, slowest 6506.7 ns)

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
| branch_cmov | 8701ns | 8606ns | 8579ns | 8625ns | 9051ns | base |
| branch_if | 8613ns | 8590ns | 8576ns | 8595ns | 8703ns | -1.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 6583ns | 6490ns | 6848ns | base | 0.622 |
| branch_if | 6517ns | 6489ns | 6592ns | -1.01% | 0.629 |

## Performance model

- Peak throughput: **0.631 Gops/s** (branch_if; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.630 | 99.7% |
| branch_if | 0.630 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 8701ns | 8701ns | base |
| branch_if | 8613ns | 8613ns | -1.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 6507ns | base | --- | [6503, 6522] | --- | --- | --- | --- |
| branch_if | 6499ns | -4.6ns (-0.1%) | [-14, -2]ns | [6497, 6500] | YES | 0.0002 | 0.0002 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 6760ns | -3.9% |
| 1 | 2 | 6499ns | +0.0% |
| 1 | 3 | 6500ns | +0.0% |
| 1 | 4 | 6502ns | -0.0% |
| 1 | 5 | 6502ns | -0.1% |
| 1 | 6 | 6499ns | -0.0% |
| 1 | 7 | 6507ns | -0.1% |
| 1 | 8 | 6745ns | -3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.035 | ok |
| branch_if | 0.425 | moderate+ |

**Consistency summary:**

- **branch_if**: won 3/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.7ns | 6583.5ns | 0.0% |  |
| branch_if | 2.7ns | 6516.9ns | 0.0% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 6490.3-6848.1 ns)
   6490.3 |########################################
   6508.2 |######
   6526.0 |###
   6543.9 |######
   6561.8 |###
   6579.7 |#
   6597.6 |
   6615.5 |###
   6633.4 |#
   6651.3 |
   6669.2 |
   6687.1 |
   6705.0 |
   6722.8 |#
   6740.7 |###
   6758.6 |#
   6776.5 |
   6794.4 |
   6812.3 |
   6830.2 |
  (11 below, 14 above range)

branch_if (n=160, range 6489.5-6592.0 ns)
   6489.5 |################################
   6494.6 |########################################
   6499.7 |########################
   6504.9 |#
   6510.0 |#####
   6515.1 |##
   6520.2 |###
   6525.4 |##
   6530.5 |#
   6535.6 |
   6540.7 |
   6545.9 |#
   6551.0 |
   6556.1 |##
   6561.2 |##
   6566.4 |
   6571.5 |
   6576.6 |
   6581.7 |
   6586.9 |
  (13 below, 15 above range)

```
