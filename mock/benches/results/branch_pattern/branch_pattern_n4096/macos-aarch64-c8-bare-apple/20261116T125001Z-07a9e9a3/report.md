# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (6.50 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 6.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 6.50 us to slowest 6.50 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (29 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: branch_if** at 6496.7 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 6496.7 ns, slowest 6497.1 ns)

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
| branch_cmov | 8597ns | 8587ns | 8576ns | 8588ns | 8644ns | base |
| branch_if | 8596ns | 8586ns | 8574ns | 8586ns | 8646ns | -0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 6504ns | 6488ns | 6542ns | base | 0.630 |
| branch_if | 6506ns | 6488ns | 6550ns | +0.02% | 0.630 |

## Performance model

- Peak throughput: **0.631 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.630 | 99.9% |
| branch_if | 0.630 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 8597ns | 8597ns | base |
| branch_if | 8596ns | 8596ns | -0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 6497ns | base | --- | [6496, 6498] | --- | --- | --- | --- |
| branch_if | 6497ns | no significant difference | [-1, +1]ns | [6496, 6498] | no | 0.7497 | 0.7497 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 6499ns | -0.0% |
| 1 | 2 | 6498ns | -0.0% |
| 1 | 3 | 6497ns | +0.0% |
| 1 | 4 | 6496ns | +0.0% |
| 1 | 5 | 6498ns | -0.0% |
| 1 | 6 | 6497ns | -0.0% |
| 1 | 7 | 6497ns | -0.0% |
| 1 | 8 | 6497ns | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | 0.056 | ok |
| branch_if | 0.292 | moderate+ |

**Consistency summary:**

- **branch_if**: won 0/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 2.7ns | 6504.4ns | 0.0% |  |
| branch_if | 2.6ns | 6505.9ns | 0.0% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 6488.1-6541.9 ns)
   6488.1 |##############
   6490.8 |##################
   6493.5 |###########################
   6496.2 |########################################
   6498.9 |#######################
   6501.6 |###############
   6504.2 |
   6506.9 |#
   6509.6 |##
   6512.3 |#
   6515.0 |#
   6517.7 |#
   6520.4 |
   6523.1 |#
   6525.8 |#
   6528.5 |
   6531.1 |##
   6533.8 |#
   6536.5 |#
   6539.2 |##
  (13 below, 15 above range)

branch_if (n=160, range 6488.3-6549.7 ns)
   6488.3 |#########################
   6491.3 |####################################
   6494.4 |########################################
   6497.5 |#################################
   6500.6 |#####################
   6503.6 |######
   6506.7 |
   6509.8 |
   6512.8 |####
   6515.9 |
   6519.0 |##
   6522.1 |##
   6525.1 |#
   6528.2 |
   6531.3 |
   6534.4 |
   6537.4 |#
   6540.5 |#####
   6543.6 |####
   6546.6 |#
  (10 below, 11 above range)

```
