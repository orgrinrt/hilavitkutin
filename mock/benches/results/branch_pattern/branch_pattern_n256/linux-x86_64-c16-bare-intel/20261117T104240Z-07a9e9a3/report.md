# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (425 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 221 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 93% faster than the next best (branch_cmov)

branch_if (221 ns) leads branch_cmov (425 ns) by 93%, a clear separation rather than a photo finish. CV 68.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 48% (significant)

branch_if is -206 ns (48%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### branch_cmov is inconsistent: worst-20% is 2.8x its best-20%

branch_cmov's best 20% of batches run at 348 ns but its worst 20% at 981 ns (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: branch_if** at 220.8 ns median (-48.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.93x (fastest 220.8 ns, slowest 425.2 ns)

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
| branch_cmov | 2784ns | 2309ns | 1888ns | 2234ns | 5329ns | base |
| branch_if | 2499ns | 2108ns | 1723ns | 2101ns | 4467ns | -10.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 512ns | 348ns | 981ns | base | 0.500 |
| branch_if | 261ns | 179ns | 461ns | -49.14% | 0.982 |

## Performance model

- Peak throughput: **1.427 Gops/s** (branch_if; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.602 | 42.2% |
| branch_if | 1.159 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2784ns | 2784ns | base |
| branch_if | 2499ns | 2499ns | -10.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 425ns | base | --- | [424, 426] | --- | --- | --- | --- |
| branch_if | 221ns | -205.8ns (-48.4%) | [-208, -204]ns | [221, 222] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 428ns | -58.6% |
| 1 | 2 | 349ns | -36.7% |
| 1 | 3 | 426ns | -57.3% |
| 1 | 4 | 426ns | -48.8% |
| 1 | 5 | 426ns | -47.8% |
| 1 | 6 | 349ns | -35.5% |
| 1 | 7 | 427ns | -48.3% |
| 1 | 8 | 349ns | -35.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.482 | moderate- |
| branch_if | -0.109 | ok |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 28.6ns | 512.5ns | 5.6% | HIGH |
| branch_if | 28.1ns | 260.6ns | 10.8% | HIGH |

## Distribution (algo ns)

```
branch_cmov (n=160, range 347.6-980.5 ns)
    347.6 |################
    379.2 |#
    410.9 |########################################
    442.5 |
    474.2 |
    505.8 |
    537.4 |
    569.1 |
    600.7 |
    632.4 |
    664.0 |
    695.7 |
    727.3 |
    759.0 |
    790.6 |
    822.3 |
    853.9 |
    885.6 |
    917.2 |
    948.9 |
  (14 below, 19 above range)

branch_if (n=160, range 179.4-460.9 ns)
    179.4 |############
    193.4 |
    207.5 |########################################
    221.6 |#####################################
    235.7 |
    249.7 |
    263.8 |
    277.9 |
    292.0 |
    306.1 |
    320.1 |
    334.2 |
    348.3 |
    362.4 |
    376.4 |
    390.5 |
    404.6 |
    418.7 |
    432.7 |
    446.8 |
  (16 below, 15 above range)

```

## Diagnostics

- **branch_cmov**: CV=60.3% (high variance, measurements may be unstable)
- **branch_if**: CV=57.6% (high variance, measurements may be unstable)
- **branch_if**: bridge=10.8% of algo (FFI overhead may distort results)
