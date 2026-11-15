# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (445 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 396 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 12% faster than the next best (branch_cmov)

branch_if (396 ns) leads branch_cmov (445 ns) by 12%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: branch_if** at 395.6 ns median (-11.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.12x (fastest 395.6 ns, slowest 445.0 ns)

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
| branch_cmov | 2704ns | 2838ns | 2462ns | 2735ns | 2853ns | base |
| branch_if | 2576ns | 2503ns | 2458ns | 2531ns | 2828ns | -4.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 428ns | 389ns | 454ns | base | 0.598 |
| branch_if | 407ns | 388ns | 446ns | -4.91% | 0.629 |

## Performance model

- Peak throughput: **0.660 Gops/s** (branch_if; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.575 | 87.1% |
| branch_if | 0.647 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2704ns | 2704ns | base |
| branch_if | 2576ns | 2576ns | -4.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 445ns | base | --- | [424, 448] | --- | --- | --- | --- |
| branch_if | 396ns | -28.1ns (-6.3%) | [-32, -11]ns | [393, 403] | YES | 0.0000 | 0.0000 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 390ns | -0.1% |
| 1 | 2 | 450ns | -13.3% |
| 1 | 3 | 391ns | +7.7% |
| 1 | 4 | 420ns | -7.0% |
| 1 | 5 | 451ns | -13.3% |
| 1 | 6 | 449ns | -9.8% |
| 1 | 7 | 449ns | -10.7% |
| 1 | 8 | 450ns | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.060 | ok |
| branch_if | -0.068 | ok |

**Consistency summary:**

- **branch_if**: won 6/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 3.2ns | 428.1ns | 0.7% |  |
| branch_if | 3.2ns | 407.0ns | 0.8% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 389.2-453.6 ns)
    389.2 |##############################
    392.4 |########
    395.6 |###
    398.8 |
    402.0 |
    405.3 |
    408.5 |
    411.7 |###
    414.9 |########
    418.2 |######
    421.4 |#########
    424.6 |##
    427.8 |#
    431.0 |
    434.3 |
    437.5 |
    440.7 |####
    443.9 |################
    447.2 |#############################
    450.4 |########################################
  (11 below, 13 above range)

branch_if (n=160, range 387.6-446.2 ns)
    387.6 |########################################
    390.6 |#####################################
    393.5 |############
    396.4 |######
    399.4 |#####
    402.3 |##########
    405.2 |########
    408.1 |######
    411.1 |##
    414.0 |########
    416.9 |############
    419.9 |######
    422.8 |
    425.7 |
    428.6 |#
    431.6 |
    434.5 |#
    437.4 |
    440.4 |#
    443.3 |##
  (14 below, 24 above range)

```
