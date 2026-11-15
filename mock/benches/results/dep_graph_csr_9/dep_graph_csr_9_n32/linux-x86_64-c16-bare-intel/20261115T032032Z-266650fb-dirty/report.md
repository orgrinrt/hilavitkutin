# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 8.3x slower than the field

dep_graph_dense (1.60 us) is 8.3x the fastest (193 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 98.2%)

dep_graph_csr wins on median (193 ns) yet has the highest variance (CV 98.2%), while dep_graph_dense is the steadiest (CV 15.1%, 1.60 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (193 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.3x the fastest

Fastest dep_graph_csr (193 ns) to slowest dep_graph_dense (1.60 us): 8.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 3.8x its best-20%

dep_graph_csr's best 20% of batches run at 158 ns but its worst 20% at 597 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 193.2 ns median
- 2 variants significantly slower than baseline
- Spread: 8.29x (fastest 193.2 ns, slowest 1600.8 ns)

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
| dep_graph_csr | 2802ns | 2071ns | 1693ns | 2079ns | 6079ns | base |
| dep_graph_dense | 3262ns | 3499ns | 2866ns | 3213ns | 3804ns | +16.41% |
| dep_graph_hybrid | 2757ns | 2086ns | 1903ns | 2089ns | 5618ns | -1.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 267ns | 158ns | 597ns | base | 0.120 |
| dep_graph_dense | 1494ns | 1313ns | 1737ns | +459.17% | 0.021 |
| dep_graph_hybrid | 274ns | 185ns | 559ns | +2.75% | 0.117 |

## Performance model

- Peak throughput: **0.203 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.166 | 81.7% |
| dep_graph_dense | 0.020 | 9.9% |
| dep_graph_hybrid | 0.154 | 75.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2802ns | 2802ns | base |
| dep_graph_dense | 3262ns | 3262ns | +16.41% |
| dep_graph_hybrid | 2757ns | 2757ns | -1.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 193ns | base | --- | [193, 194] | --- | --- | --- | --- |
| dep_graph_dense | 1601ns | +1167.5ns (+604.3%) | [+1122, +1277]ns | [1326, 1602] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 208ns | +14.6ns (+7.6%) | [+14, +17]ns | [207, 211] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 194ns | +729.2% | +8.5% |
| 1 | 2 | 193ns | +730.4% | +3.0% |
| 1 | 3 | 158ns | +739.7% | +29.9% |
| 1 | 4 | 158ns | +924.6% | +9.0% |
| 1 | 5 | 194ns | +578.3% | +9.6% |
| 1 | 6 | 193ns | +738.0% | +6.3% |
| 1 | 7 | 194ns | +577.8% | +7.4% |
| 1 | 8 | 193ns | +580.3% | +12.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.291 | moderate+ |
| dep_graph_dense | -0.387 | moderate- |
| dep_graph_hybrid | -0.216 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 65.1ns | 267.1ns | 24.4% | HIGH |
| dep_graph_dense | 905.1ns | 1493.7ns | 60.6% | HIGH |
| dep_graph_hybrid | 68.4ns | 274.5ns | 24.9% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 157.9-596.6 ns)
    157.9 |#########
    179.8 |########################################
    201.7 |##
    223.7 |
    245.6 |
    267.5 |
    289.5 |
    311.4 |
    333.3 |
    355.3 |
    377.2 |
    399.1 |
    421.1 |
    443.0 |
    464.9 |
    486.9 |
    508.8 |
    530.8 |
    552.7 |
    574.6 |##
  (11 below, 19 above range)

dep_graph_dense (n=160, range 1313.3-1736.5 ns)
   1313.3 |########################################
   1334.5 |
   1355.6 |
   1376.8 |
   1398.0 |
   1419.1 |
   1440.3 |
   1461.4 |
   1482.6 |
   1503.8 |
   1524.9 |
   1546.1 |
   1567.2 |
   1588.4 |########################
   1609.6 |#########################
   1630.7 |##
   1651.9 |
   1673.0 |
   1694.2 |
   1715.4 |
  (16 below, 3 above range)

dep_graph_hybrid (n=160, range 185.2-559.0 ns)
    185.2 |###########
    203.9 |########################################
    222.6 |
    241.3 |
    260.0 |
    278.7 |
    297.4 |
    316.1 |
    334.7 |
    353.4 |
    372.1 |
    390.8 |
    409.5 |
    428.2 |
    446.9 |
    465.6 |
    484.3 |
    503.0 |
    521.6 |
    540.3 |
  (16 below, 21 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=71.1% (high variance, measurements may be unstable)
- **dep_graph_csr**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dep_graph_csr**: bridge=25.2% of algo (FFI overhead may distort results)
- **dep_graph_dense**: bridge=60.4% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=59.0% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: worst_20/best_20 = 3.0x (possible bimodal distribution)
- **dep_graph_hybrid**: bridge=24.8% of algo (FFI overhead may distort results)
