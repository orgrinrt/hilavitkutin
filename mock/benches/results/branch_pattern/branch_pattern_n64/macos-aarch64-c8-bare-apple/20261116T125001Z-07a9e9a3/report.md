# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (82 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 82 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.5% (fastest 82 ns to slowest 82 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: branch_if** at 82.1 ns median (-0.5% vs baseline)
- Spread: 1.00x (fastest 82.1 ns, slowest 82.5 ns)

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
| branch_cmov | 2171ns | 2167ns | 2161ns | 2167ns | 2194ns | base |
| branch_if | 2170ns | 2167ns | 2162ns | 2167ns | 2187ns | -0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 82ns | 77ns | 88ns | base | 0.776 |
| branch_if | 82ns | 77ns | 86ns | -0.92% | 0.783 |

## Performance model

- Peak throughput: **0.827 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.776 | 93.8% |
| branch_if | 0.780 | 94.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2171ns | 2171ns | base |
| branch_if | 2170ns | 2170ns | -0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 82ns | base | --- | [82, 83] | --- | --- | --- | --- |
| branch_if | 82ns | no significant difference | [-1, +0]ns | [81, 82] | no (adj: YES) | 0.0352 | 0.0352 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 84ns | -2.4% |
| 1 | 2 | 82ns | +0.5% |
| 1 | 3 | 83ns | -1.2% |
| 1 | 4 | 82ns | -0.2% |
| 1 | 5 | 82ns | +0.0% |
| 1 | 6 | 82ns | -1.0% |
| 1 | 7 | 83ns | -1.4% |
| 1 | 8 | 82ns | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.043 | ok |
| branch_if | -0.360 | moderate- |

**Consistency summary:**

- **branch_if**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.6ns | 82.5ns | 3.2% |  |
| branch_if | 2.7ns | 81.7ns | 3.3% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 77.3-87.6 ns)
     77.3 |########
     77.9 |############
     78.4 |#####
     78.9 |#
     79.4 |######
     79.9 |######################
     80.4 |########
     80.9 |########
     81.4 |#################
     81.9 |####################
     82.5 |########################################
     83.0 |###################
     83.5 |########
     84.0 |###############
     84.5 |######################
     85.0 |##########
     85.5 |##########
     86.0 |######
     86.5 |
     87.1 |##########
  (9 below, 3 above range)

branch_if (n=160, range 77.4-85.6 ns)
     77.4 |###
     77.8 |######
     78.2 |##########################
     78.6 |##########
     79.0 |#######################
     79.4 |####################
     79.8 |#######################
     80.3 |##########################
     80.7 |##########################
     81.1 |####################
     81.5 |####################################
     81.9 |##############################
     82.3 |##############################
     82.7 |########################################
     83.1 |##########################
     83.5 |################
     84.0 |##############################
     84.4 |####################
     84.8 |#######################
     85.2 |#############
  (12 below, 12 above range)

```
