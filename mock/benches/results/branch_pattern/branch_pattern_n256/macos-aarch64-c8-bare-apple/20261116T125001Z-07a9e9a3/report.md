# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (390 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 390 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 390 ns to slowest 390 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (10 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: branch_if** at 390.0 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 390.0 ns, slowest 390.2 ns)

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
| branch_cmov | 2515ns | 2475ns | 2467ns | 2479ns | 2671ns | base |
| branch_if | 2493ns | 2473ns | 2466ns | 2474ns | 2578ns | -0.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 397ns | 385ns | 428ns | base | 0.645 |
| branch_if | 392ns | 385ns | 406ns | -1.22% | 0.653 |

## Performance model

- Peak throughput: **0.665 Gops/s** (branch_if; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.656 | 98.6% |
| branch_if | 0.656 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2515ns | 2515ns | base |
| branch_if | 2493ns | 2493ns | -0.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 390ns | base | --- | [390, 391] | --- | --- | --- | --- |
| branch_if | 390ns | no significant difference | [-0, +0]ns | [389, 391] | no | 0.2852 | 0.2852 | 12 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 398ns | -2.1% |
| 1 | 2 | 389ns | +0.1% |
| 1 | 3 | 390ns | -0.3% |
| 1 | 4 | 390ns | -0.1% |
| 1 | 5 | 406ns | -0.3% |
| 1 | 6 | 390ns | -0.1% |
| 1 | 7 | 390ns | +0.0% |
| 1 | 8 | 389ns | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.212 | moderate- |
| branch_if | -0.129 | ok |

**Consistency summary:**

- **branch_if**: won 4/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 3.2ns | 396.9ns | 0.8% |  |
| branch_if | 3.3ns | 392.1ns | 0.8% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 385.0-427.5 ns)
    385.0 |###############
    387.2 |##########################
    389.3 |########################################
    391.4 |###############
    393.5 |####
    395.7 |#
    397.8 |#
    399.9 |###
    402.0 |###
    404.1 |######
    406.3 |#
    408.4 |##
    410.5 |#
    412.6 |###
    414.8 |
    416.9 |
    419.0 |
    421.1 |
    423.3 |
    425.4 |
  (13 below, 5 above range)

branch_if (n=160, range 384.7-405.6 ns)
    384.7 |##########
    385.8 |###############
    386.8 |############
    387.9 |########################################
    388.9 |##########################
    389.9 |########################################
    391.0 |##################
    392.0 |################
    393.1 |####
    394.1 |##########
    395.2 |#
    396.2 |
    397.2 |
    398.3 |#
    399.3 |
    400.4 |###
    401.4 |#
    402.4 |######
    403.5 |#
    404.5 |####
  (10 below, 10 above range)

```
