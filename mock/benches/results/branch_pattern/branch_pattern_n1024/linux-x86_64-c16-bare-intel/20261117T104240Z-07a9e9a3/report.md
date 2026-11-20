# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (branch_cmov) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline branch_cmov has the worst median (1.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest branch_if at 715 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### branch_if dominates: 97% faster than the next best (branch_cmov)

branch_if (715 ns) leads branch_cmov (1.41 us) by 97%, a clear separation rather than a photo finish. CV 49.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_if beats baseline by 49% (significant)

branch_if is -687 ns (49%) faster than baseline branch_cmov, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### branch_if is fastest but the noisiest (CV 49.9%)

branch_if wins on median (715 ns) yet has the highest variance (CV 49.9%), while branch_cmov is the steadiest (CV 22.3%, 1.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### branch_if is inconsistent: worst-20% is 1.8x its best-20%

branch_if's best 20% of batches run at 695 ns but its worst 20% at 1.26 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: branch_if** at 714.6 ns median (-49.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.97x (fastest 714.6 ns, slowest 1411.0 ns)

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
| branch_cmov | 3231ns | 2954ns | 2940ns | 3040ns | 4093ns | base |
| branch_if | 2724ns | 2260ns | 2238ns | 2450ns | 4033ns | -15.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 1538ns | 1397ns | 1947ns | base | 0.666 |
| branch_if | 853ns | 695ns | 1259ns | -44.55% | 1.201 |

## Performance model

- Peak throughput: **1.472 Gops/s** (branch_if; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.726 | 49.3% |
| branch_if | 1.433 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 3231ns | 3231ns | base |
| branch_if | 2724ns | 2724ns | -15.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 1411ns | base | --- | [1409, 1413] | --- | --- | --- | --- |
| branch_if | 715ns | -687.3ns (-48.7%) | [-704, -680]ns | [711, 859] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 1410ns | -51.3% |
| 1 | 2 | 1720ns | -59.0% |
| 1 | 3 | 1408ns | -49.8% |
| 1 | 4 | 1408ns | -38.7% |
| 1 | 5 | 1403ns | -49.1% |
| 1 | 6 | 1722ns | -59.1% |
| 1 | 7 | 1411ns | -39.1% |
| 1 | 8 | 1402ns | -37.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.367 | moderate- |
| branch_if | 0.099 | ok |

**Consistency summary:**

- **branch_if**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 21.3ns | 1538.1ns | 1.4% |  |
| branch_if | 23.6ns | 852.9ns | 2.8% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 1397.0-1946.7 ns)
   1397.0 |########################################
   1424.5 |
   1452.0 |
   1479.5 |
   1507.0 |
   1534.5 |
   1561.9 |
   1589.4 |
   1616.9 |
   1644.4 |
   1671.9 |
   1699.4 |############
   1726.9 |###
   1754.3 |
   1781.8 |
   1809.3 |
   1836.8 |
   1864.3 |
   1891.8 |
   1919.2 |
  (16 below, 6 above range)

branch_if (n=160, range 695.4-1258.8 ns)
    695.4 |########################################
    723.6 |#
    751.8 |
    779.9 |
    808.1 |
    836.3 |###################
    864.4 |#############
    892.6 |
    920.8 |
    948.9 |
    977.1 |
   1005.3 |
   1033.4 |
   1061.6 |
   1089.8 |
   1117.9 |
   1146.1 |
   1174.3 |
   1202.4 |
   1230.6 |
  (16 below, 8 above range)

```

## Diagnostics

- **branch_cmov**: CV=20.5% (high variance, measurements may be unstable)
- **branch_if**: CV=41.8% (high variance, measurements may be unstable)
