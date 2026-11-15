# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 188% faster than the next best (dep_graph_dense)

dep_graph_csr (43 ns) leads dep_graph_dense (124 ns) by 188%, a clear separation rather than a photo finish. CV 89.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_hybrid is an outlier: 2.9x slower than the field

dep_graph_hybrid (125 ns) is 2.9x the fastest (43 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 89.2%)

dep_graph_csr wins on median (43 ns) yet has the highest variance (CV 89.2%), while dep_graph_hybrid is the steadiest (CV 48.0%, 125 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (43 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dep_graph_csr is inconsistent: worst-20% is 3.7x its best-20%

dep_graph_csr's best 20% of batches run at 35 ns but its worst 20% at 132 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 43.2 ns median
- 2 variants significantly slower than baseline
- Spread: 2.89x (fastest 43.2 ns, slowest 124.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x13579bdf2468ace0 |
| Counter frequency | 2112740176 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dep_graph_csr | 2737ns | 1920ns | 1568ns | 2122ns | 5752ns | base |
| dep_graph_dense | 2275ns | 2022ns | 1654ns | 2012ns | 3685ns | -16.90% |
| dep_graph_hybrid | 2260ns | 2022ns | 1824ns | 2022ns | 3410ns | -17.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 62ns | 35ns | 132ns | base | 0.129 |
| dep_graph_dense | 140ns | 103ns | 225ns | +124.31% | 0.057 |
| dep_graph_hybrid | 140ns | 113ns | 210ns | +124.26% | 0.057 |

## Performance model

- Peak throughput: **0.226 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.185 | 81.8% |
| dep_graph_dense | 0.064 | 28.4% |
| dep_graph_hybrid | 0.064 | 28.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2737ns | 2737ns | base |
| dep_graph_dense | 2275ns | 2275ns | -16.90% |
| dep_graph_hybrid | 2260ns | 2260ns | -17.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 43ns | base | --- | [43, 43] | --- | --- | --- | --- |
| dep_graph_dense | 124ns | +81.2ns (+188.1%) | [+79, +82]ns | [124, 125] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 125ns | +82.3ns (+190.6%) | [+82, +84]ns | [124, 125] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 36ns | +182.9% | +244.3% |
| 1 | 2 | 43ns | +139.4% | +140.2% |
| 1 | 3 | 37ns | +238.9% | +243.7% |
| 1 | 4 | 43ns | +188.5% | +187.6% |
| 1 | 5 | 43ns | +189.8% | +189.5% |
| 1 | 6 | 43ns | +182.8% | +194.3% |
| 1 | 7 | 35ns | +254.6% | +249.2% |
| 1 | 8 | 35ns | +254.5% | +250.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.048 | ok |
| dep_graph_dense | 0.463 | moderate+ |
| dep_graph_hybrid | -0.178 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 70.6ns | 62.3ns | 113.5% | HIGH |
| dep_graph_dense | 1111.5ns | 139.6ns | 795.9% | HIGH |
| dep_graph_hybrid | 1105.3ns | 139.6ns | 791.7% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 35.4-131.6 ns)
     35.4 |####################
     40.2 |########################################
     45.0 |#
     49.8 |
     54.6 |
     59.4 |
     64.2 |
     69.1 |
     73.9 |
     78.7 |
     83.5 |
     88.3 |
     93.1 |#
     97.9 |#
    102.8 |###
    107.6 |#
    112.4 |#
    117.2 |#####
    122.0 |
    126.8 |#
  (18 below, 13 above range)

dep_graph_dense (n=160, range 102.8-224.5 ns)
    102.8 |#######
    108.8 |
    114.9 |
    121.0 |########################################
    127.1 |
    133.2 |
    139.3 |
    145.4 |
    151.5 |
    157.5 |
    163.6 |
    169.7 |
    175.8 |
    181.9 |
    188.0 |
    194.1 |
    200.2 |
    206.2 |
    212.3 |
    218.4 |
  (15 below, 15 above range)

dep_graph_hybrid (n=160, range 112.9-210.5 ns)
    112.9 |
    117.8 |
    122.7 |########################################
    127.6 |##
    132.5 |
    137.3 |
    142.2 |
    147.1 |
    152.0 |
    156.8 |
    161.7 |
    166.6 |
    171.5 |
    176.4 |
    181.2 |
    186.1 |
    191.0 |
    195.9 |
    200.7 |
    205.6 |
  (17 below, 12 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=61.9% (high variance, measurements may be unstable)
- **dep_graph_csr**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **dep_graph_csr**: bridge=115.5% of algo (FFI overhead may distort results)
- **dep_graph_dense**: CV=45.5% (high variance, measurements may be unstable)
- **dep_graph_dense**: bridge=795.4% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=42.9% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: bridge=794.1% of algo (FFI overhead may distort results)
