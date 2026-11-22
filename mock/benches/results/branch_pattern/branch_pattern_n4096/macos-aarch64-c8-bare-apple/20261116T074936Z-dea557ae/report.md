# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (6.49 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 6.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 6.49 us to slowest 6.49 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (60 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: branch_if** at 6490.4 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 6490.4 ns, slowest 6492.3 ns)

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
| branch_cmov | 8595ns | 8575ns | 8556ns | 8579ns | 8684ns | base |
| branch_if | 8606ns | 8574ns | 8554ns | 8582ns | 8732ns | +0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 6510ns | 6483ns | 6582ns | base | 0.629 |
| branch_if | 6516ns | 6482ns | 6610ns | +0.08% | 0.629 |

## Performance model

- Peak throughput: **0.632 Gops/s** (branch_if; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.631 | 99.8% |
| branch_if | 0.631 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 8595ns | 8595ns | base |
| branch_if | 8606ns | 8606ns | +0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 6492ns | base | --- | [6491, 6494] | --- | --- | --- | --- |
| branch_if | 6490ns | no significant difference | [-4, +1]ns | [6489, 6492] | no | 0.2979 | 0.2979 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 6491ns | -0.0% |
| 1 | 2 | 6502ns | -0.2% |
| 1 | 3 | 6490ns | -0.0% |
| 1 | 4 | 6492ns | +0.3% |
| 1 | 5 | 6490ns | +0.0% |
| 1 | 6 | 6493ns | -0.0% |
| 1 | 7 | 6489ns | +0.1% |
| 1 | 8 | 6490ns | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.202 | moderate- |
| branch_if | -0.314 | moderate- |

**Consistency summary:**

- **branch_if**: won 1/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.8ns | 6510.4ns | 0.0% |  |
| branch_if | 2.8ns | 6515.7ns | 0.0% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 6483.2-6581.8 ns)
   6483.2 |############################
   6488.2 |########################################
   6493.1 |######################
   6498.0 |#######
   6503.0 |######
   6507.9 |####
   6512.8 |##
   6517.7 |###
   6522.7 |###
   6527.6 |##
   6532.5 |####
   6537.4 |####
   6542.4 |##
   6547.3 |###
   6552.2 |##
   6557.2 |
   6562.1 |##
   6567.0 |#
   6571.9 |#
   6576.9 |
  (15 below, 10 above range)

branch_if (n=160, range 6482.2-6610.3 ns)
   6482.2 |########################################
   6488.6 |#####################################
   6495.0 |#
   6501.4 |####
   6507.8 |#
   6514.2 |##
   6520.6 |##
   6527.1 |####
   6533.5 |##
   6539.9 |
   6546.3 |#
   6552.7 |###
   6559.1 |
   6565.5 |###
   6571.9 |
   6578.3 |#
   6584.7 |
   6591.1 |
   6597.5 |#
   6603.9 |#
  (12 below, 11 above range)

```
