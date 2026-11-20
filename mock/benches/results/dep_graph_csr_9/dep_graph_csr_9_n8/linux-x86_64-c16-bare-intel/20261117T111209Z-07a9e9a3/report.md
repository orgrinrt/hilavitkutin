# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 184% faster than the next best (dep_graph_dense)

dep_graph_csr (43 ns) leads dep_graph_dense (122 ns) by 184%, a clear separation rather than a photo finish. CV 84.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_hybrid is an outlier: 2.9x slower than the field

dep_graph_hybrid (124 ns) is 2.9x the fastest (43 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 84.5%)

dep_graph_csr wins on median (43 ns) yet has the highest variance (CV 84.5%), while dep_graph_dense is the steadiest (CV 46.7%, 122 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (43 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dep_graph_csr is inconsistent: worst-20% is 3.6x its best-20%

dep_graph_csr's best 20% of batches run at 35 ns but its worst 20% at 127 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 43.1 ns median
- 2 variants significantly slower than baseline
- Spread: 2.88x (fastest 43.1 ns, slowest 124.0 ns)

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
| Counter frequency | 2112765565 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dep_graph_csr | 2570ns | 1926ns | 1576ns | 1874ns | 5650ns | base |
| dep_graph_dense | 2128ns | 2024ns | 1658ns | 1883ns | 3333ns | -17.20% |
| dep_graph_hybrid | 2134ns | 2025ns | 1657ns | 1870ns | 3402ns | -16.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 58ns | 35ns | 127ns | base | 0.138 |
| dep_graph_dense | 130ns | 101ns | 202ns | +124.09% | 0.062 |
| dep_graph_hybrid | 131ns | 102ns | 211ns | +126.88% | 0.061 |

## Performance model

- Peak throughput: **0.226 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.186 | 82.0% |
| dep_graph_dense | 0.065 | 28.8% |
| dep_graph_hybrid | 0.065 | 28.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2570ns | 2570ns | base |
| dep_graph_dense | 2128ns | 2128ns | -17.20% |
| dep_graph_hybrid | 2134ns | 2134ns | -16.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 43ns | base | --- | [37, 43] | --- | --- | --- | --- |
| dep_graph_dense | 122ns | +67.9ns (+157.5%) | [+66, +79]ns | [104, 123] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 124ns | +66.6ns (+154.5%) | [+66, +84]ns | [102, 124] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 35ns | +247.5% | +187.1% |
| 1 | 2 | 43ns | +138.6% | +135.3% |
| 1 | 3 | 35ns | +190.5% | +251.1% |
| 1 | 4 | 35ns | +186.4% | +188.0% |
| 1 | 5 | 37ns | +238.3% | +239.5% |
| 1 | 6 | 43ns | +192.2% | +135.4% |
| 1 | 7 | 36ns | +182.0% | +247.8% |
| 1 | 8 | 43ns | +184.0% | +195.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.453 | moderate- |
| dep_graph_dense | -0.142 | ok |
| dep_graph_hybrid | -0.309 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 65.9ns | 57.8ns | 113.9% | HIGH |
| dep_graph_dense | 1036.0ns | 129.6ns | 799.4% | HIGH |
| dep_graph_hybrid | 1040.4ns | 131.2ns | 792.8% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 35.3-126.9 ns)
     35.3 |#######################################
     39.9 |########################################
     44.5 |
     49.1 |
     53.6 |
     58.2 |
     62.8 |
     67.4 |
     71.9 |#
     76.5 |
     81.1 |
     85.7 |
     90.3 |
     94.8 |
     99.4 |##
    104.0 |##
    108.6 |##
    113.1 |##
    117.7 |##
    122.3 |####
  (22 below, 12 above range)

dep_graph_dense (n=160, range 101.0-201.7 ns)
    101.0 |################################
    106.1 |
    111.1 |
    116.1 |
    121.2 |########################################
    126.2 |######
    131.2 |
    136.3 |
    141.3 |
    146.3 |
    151.4 |
    156.4 |
    161.4 |
    166.5 |
    171.5 |
    176.5 |
    181.6 |
    186.6 |
    191.7 |
    196.7 |
  (18 below, 12 above range)

dep_graph_hybrid (n=160, range 101.5-210.6 ns)
    101.5 |##################################
    107.0 |
    112.4 |
    117.9 |
    123.3 |########################################
    128.8 |#
    134.2 |
    139.7 |
    145.2 |
    150.6 |
    156.1 |
    161.5 |
    167.0 |
    172.4 |
    177.9 |
    183.3 |
    188.8 |
    194.3 |
    199.7 |
    205.2 |
  (14 below, 13 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=63.0% (high variance, measurements may be unstable)
- **dep_graph_csr**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **dep_graph_csr**: bridge=113.9% of algo (FFI overhead may distort results)
- **dep_graph_dense**: CV=44.1% (high variance, measurements may be unstable)
- **dep_graph_dense**: bridge=806.9% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=46.6% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: bridge=790.4% of algo (FFI overhead may distort results)
