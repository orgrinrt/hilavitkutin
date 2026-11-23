# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (111 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 63 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 78% faster than the next best (branch_cmov)

branch_if (63 ns) leads branch_cmov (111 ns) by 78%, a clear separation rather than a photo finish. CV 86.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 44% (significant)

branch_if is -49 ns (44%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### branch_if is fastest but the noisiest (CV 86.2%)

branch_if wins on median (63 ns) yet has the highest variance (CV 86.2%), while branch_cmov is the steadiest (CV 82.9%, 111 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (49 ns) is smaller than the fastest variant's own run-to-run std-dev (54 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### branch_if is inconsistent: worst-20% is 3.8x its best-20%

branch_if's best 20% of batches run at 50 ns but its worst 20% at 186 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: branch_if** at 62.8 ns median (-43.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.78x (fastest 62.8 ns, slowest 111.5 ns)

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
| Counter frequency | 2112827530 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 2655ns | 1986ns | 1625ns | 1912ns | 5913ns | base |
| branch_if | 2763ns | 1938ns | 1585ns | 2097ns | 5941ns | +4.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 148ns | 91ns | 326ns | base | 0.433 |
| branch_if | 87ns | 50ns | 186ns | -40.94% | 0.732 |

## Performance model

- Peak throughput: **1.292 Gops/s** (branch_if; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.574 | 44.5% |
| branch_if | 1.020 | 78.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2655ns | 2655ns | base |
| branch_if | 2763ns | 2763ns | +4.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 111ns | base | --- | [111, 112] | --- | --- | --- | --- |
| branch_if | 63ns | -49.0ns (-44.0%) | [-60, -41]ns | [58, 63] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 113ns | -55.6% |
| 1 | 2 | 112ns | -43.9% |
| 1 | 3 | 92ns | -30.9% |
| 1 | 4 | 92ns | -37.1% |
| 1 | 5 | 112ns | -53.7% |
| 1 | 6 | 91ns | -43.8% |
| 1 | 7 | 112ns | -56.7% |
| 1 | 8 | 92ns | -30.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.370 | moderate- |
| branch_if | 0.027 | ok |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 31.7ns | 147.9ns | 21.4% | HIGH |
| branch_if | 33.9ns | 87.4ns | 38.8% | HIGH |

## Distribution (algo ns)

```
branch_cmov (n=160, range 91.0-326.1 ns)
     91.0 |###################################
    102.7 |########################################
    114.5 |#
    126.2 |
    138.0 |
    149.7 |
    161.5 |
    173.3 |
    185.0 |
    196.8 |
    208.5 |
    220.3 |##
    232.0 |
    243.8 |
    255.6 |#
    267.3 |
    279.1 |
    290.8 |
    302.6 |####
    314.3 |##
  (12 below, 15 above range)

branch_if (n=160, range 49.5-186.4 ns)
     49.5 |########################################
     56.4 |#####################################
     63.2 |#####################
     70.1 |##
     76.9 |
     83.8 |
     90.6 |
     97.5 |
    104.3 |
    111.1 |
    118.0 |
    124.8 |
    131.7 |
    138.5 |#
    145.4 |##
    152.2 |##
    159.1 |###
    165.9 |#####
    172.8 |###
    179.6 |#
  (14 below, 15 above range)

```

## Diagnostics

- **branch_cmov**: CV=62.4% (high variance, measurements may be unstable)
- **branch_cmov**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **branch_cmov**: bridge=21.4% of algo (FFI overhead may distort results)
- **branch_if**: CV=61.9% (high variance, measurements may be unstable)
- **branch_if**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **branch_if**: bridge=38.1% of algo (FFI overhead may distort results)
