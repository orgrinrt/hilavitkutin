# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (429 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 221 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 94% faster than the next best (branch_cmov)

branch_if (221 ns) leads branch_cmov (429 ns) by 94%, a clear separation rather than a photo finish. CV 81.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 48% (significant)

branch_if is -207 ns (48%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### branch_if is fastest but the noisiest (CV 81.7%)

branch_if wins on median (221 ns) yet has the highest variance (CV 81.7%), while branch_cmov is the steadiest (CV 72.2%, 429 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### branch_if is inconsistent: worst-20% is 3.5x its best-20%

branch_if's best 20% of batches run at 179 ns but its worst 20% at 620 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: branch_if** at 220.9 ns median (-48.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.94x (fastest 220.9 ns, slowest 428.6 ns)

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
| branch_cmov | 2887ns | 2307ns | 1888ns | 2306ns | 5628ns | base |
| branch_if | 2789ns | 2100ns | 1716ns | 2101ns | 5927ns | -3.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 536ns | 350ns | 1046ns | base | 0.478 |
| branch_if | 293ns | 179ns | 620ns | -45.23% | 0.873 |

## Performance model

- Peak throughput: **1.429 Gops/s** (branch_if; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.597 | 41.8% |
| branch_if | 1.159 | 81.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 2887ns | 2887ns | base |
| branch_if | 2789ns | 2789ns | -3.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 429ns | base | --- | [427, 429] | --- | --- | --- | --- |
| branch_if | 221ns | -207.4ns (-48.4%) | [-209, -205]ns | [221, 222] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 352ns | -49.6% |
| 1 | 2 | 428ns | -48.4% |
| 1 | 3 | 428ns | -57.6% |
| 1 | 4 | 430ns | -49.2% |
| 1 | 5 | 430ns | -48.3% |
| 1 | 6 | 352ns | -36.0% |
| 1 | 7 | 429ns | -48.5% |
| 1 | 8 | 430ns | -47.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.202 | moderate- |
| branch_if | -0.107 | ok |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 29.6ns | 535.6ns | 5.5% | HIGH |
| branch_if | 31.5ns | 293.3ns | 10.7% | HIGH |

## Distribution (algo ns)

```
branch_cmov (n=160, range 350.3-1046.3 ns)
    350.3 |########
    385.1 |###
    419.9 |########################################
    454.7 |
    489.5 |
    524.3 |
    559.1 |
    593.9 |
    628.7 |
    663.5 |
    698.3 |
    733.1 |
    767.9 |
    802.7 |
    837.5 |
    872.3 |
    907.1 |
    941.9 |
    976.7 |
   1011.5 |
  (12 below, 21 above range)

branch_if (n=160, range 179.1-620.1 ns)
    179.1 |########
    201.1 |########################################
    223.2 |###################
    245.2 |
    267.3 |
    289.3 |
    311.4 |
    333.4 |
    355.5 |
    377.5 |
    399.6 |
    421.6 |
    443.7 |
    465.7 |
    487.8 |#
    509.8 |
    531.9 |#
    553.9 |#
    576.0 |
    598.0 |
  (18 below, 20 above range)

```

## Diagnostics

- **branch_cmov**: CV=57.8% (high variance, measurements may be unstable)
- **branch_if**: CV=61.6% (high variance, measurements may be unstable)
- **branch_if**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **branch_if**: bridge=10.8% of algo (FFI overhead may distort results)
