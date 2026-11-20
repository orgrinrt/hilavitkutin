# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (112 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 63 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 79% faster than the next best (branch_cmov)

branch_if (63 ns) leads branch_cmov (112 ns) by 79%, a clear separation rather than a photo finish. CV 85.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 45% (significant)

branch_if is -50 ns (45%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (50 ns) is smaller than the fastest variant's own run-to-run std-dev (54 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### branch_if is inconsistent: worst-20% is 3.8x its best-20%

branch_if's best 20% of batches run at 49 ns but its worst 20% at 186 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: branch_if** at 62.7 ns median (-44.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.79x (fastest 62.7 ns, slowest 112.2 ns)

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
| Counter frequency | 2112670986 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 2760ns | 1996ns | 1631ns | 2018ns | 6112ns | base |
| branch_if | 2731ns | 1946ns | 1589ns | 2054ns | 5901ns | -1.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 154ns | 92ns | 338ns | base | 0.415 |
| branch_if | 86ns | 49ns | 186ns | -44.09% | 0.741 |

## Performance model

- Peak throughput: **1.315 Gops/s** (branch_if; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.570 | 43.4% |
| branch_if | 1.021 | 77.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2760ns | 2760ns | base |
| branch_if | 2731ns | 2731ns | -1.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 112ns | base | --- | [112, 112] | --- | --- | --- | --- |
| branch_if | 63ns | -50.5ns (-45.0%) | [-52, -47]ns | [61, 63] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 92ns | -33.8% |
| 1 | 2 | 92ns | -31.8% |
| 1 | 3 | 92ns | -43.8% |
| 1 | 4 | 112ns | -58.0% |
| 1 | 5 | 115ns | -44.7% |
| 1 | 6 | 112ns | -44.2% |
| 1 | 7 | 92ns | -47.5% |
| 1 | 8 | 112ns | -53.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | 0.174 | ok |
| branch_if | 0.007 | ok |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 33.2ns | 154.4ns | 21.5% | HIGH |
| branch_if | 33.3ns | 86.3ns | 38.6% | HIGH |

## Distribution (algo ns)

```
branch_cmov (n=160, range 91.5-338.1 ns)
     91.5 |#############################
    103.9 |########################################
    116.2 |
    128.5 |
    140.8 |
    153.2 |
    165.5 |
    177.8 |
    190.2 |
    202.5 |
    214.8 |#
    227.2 |
    239.5 |
    251.8 |
    264.1 |
    276.5 |#
    288.8 |#
    301.1 |#####
    313.5 |
    325.8 |##
  (12 below, 14 above range)

branch_if (n=160, range 48.7-186.0 ns)
     48.7 |#######################
     55.6 |##############
     62.4 |########################################
     69.3 |
     76.1 |
     83.0 |
     89.9 |
     96.7 |
    103.6 |
    110.5 |
    117.3 |
    124.2 |#
    131.1 |#
    137.9 |
    144.8 |##
    151.7 |
    158.5 |######
    165.4 |
    172.3 |##
    179.1 |##
  (25 below, 14 above range)

```

## Diagnostics

- **branch_cmov**: CV=62.7% (high variance, measurements may be unstable)
- **branch_cmov**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **branch_cmov**: bridge=21.3% of algo (FFI overhead may distort results)
- **branch_if**: CV=62.2% (high variance, measurements may be unstable)
- **branch_if**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **branch_if**: bridge=38.1% of algo (FFI overhead may distort results)
