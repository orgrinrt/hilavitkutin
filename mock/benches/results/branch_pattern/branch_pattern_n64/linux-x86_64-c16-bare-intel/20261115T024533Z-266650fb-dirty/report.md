# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (113 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 63 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 79% faster than the next best (branch_cmov)

branch_if (63 ns) leads branch_cmov (113 ns) by 79%, a clear separation rather than a photo finish. CV 95.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 44% (significant)

branch_if is -50 ns (44%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### branch_if is fastest but the noisiest (CV 95.5%)

branch_if wins on median (63 ns) yet has the highest variance (CV 95.5%), while branch_cmov is the steadiest (CV 86.2%, 113 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (50 ns) is smaller than the fastest variant's own run-to-run std-dev (60 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### branch_if is inconsistent: worst-20% is 3.9x its best-20%

branch_if's best 20% of batches run at 53 ns but its worst 20% at 205 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: branch_if** at 62.8 ns median (-44.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.79x (fastest 62.8 ns, slowest 112.5 ns)

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
| Counter frequency | 2112793168 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 2838ns | 1989ns | 1624ns | 2202ns | 5958ns | base |
| branch_if | 2990ns | 1940ns | 1738ns | 2259ns | 6436ns | +5.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 159ns | 92ns | 332ns | base | 0.402 |
| branch_if | 95ns | 53ns | 205ns | -40.29% | 0.673 |

## Performance model

- Peak throughput: **1.202 Gops/s** (branch_if; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.569 | 47.3% |
| branch_if | 1.018 | 84.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2838ns | 2838ns | base |
| branch_if | 2990ns | 2990ns | +5.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 113ns | base | --- | [112, 113] | --- | --- | --- | --- |
| branch_if | 63ns | -49.9ns (-44.3%) | [-54, -48]ns | [63, 63] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 141ns | -64.6% |
| 1 | 2 | 92ns | -32.1% |
| 1 | 3 | 92ns | +67.1% |
| 1 | 4 | 113ns | -49.0% |
| 1 | 5 | 93ns | -31.6% |
| 1 | 6 | 113ns | -44.5% |
| 1 | 7 | 92ns | -35.9% |
| 1 | 8 | 112ns | -43.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.354 | moderate- |
| branch_if | -0.165 | ok |

**Consistency summary:**

- **branch_if**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 34.1ns | 159.4ns | 21.4% | HIGH |
| branch_if | 36.5ns | 95.2ns | 38.4% | HIGH |

## Distribution (algo ns)

```
branch_cmov (n=160, range 91.8-331.9 ns)
     91.8 |############################
    103.8 |########################################
    115.8 |
    127.8 |
    139.8 |
    151.8 |
    163.8 |#
    175.8 |
    187.9 |
    199.9 |
    211.9 |
    223.9 |
    235.9 |##
    247.9 |
    259.9 |
    271.9 |#
    283.9 |#######
    295.9 |##
    307.9 |#####
    319.9 |##
  (16 below, 10 above range)

branch_if (n=160, range 53.2-205.5 ns)
     53.2 |###################
     60.9 |########################################
     68.5 |
     76.1 |
     83.7 |
     91.3 |
     98.9 |
    106.5 |#
    114.1 |#
    121.7 |#
    129.4 |
    137.0 |#
    144.6 |#
    152.2 |#
    159.8 |#
    167.4 |
    175.0 |#
    182.6 |##
    190.2 |#
    197.9 |
  (18 below, 18 above range)

```

## Diagnostics

- **branch_cmov**: CV=60.9% (high variance, measurements may be unstable)
- **branch_cmov**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **branch_cmov**: bridge=21.1% of algo (FFI overhead may distort results)
- **branch_if**: CV=63.1% (high variance, measurements may be unstable)
- **branch_if**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **branch_if**: bridge=37.9% of algo (FFI overhead may distort results)
