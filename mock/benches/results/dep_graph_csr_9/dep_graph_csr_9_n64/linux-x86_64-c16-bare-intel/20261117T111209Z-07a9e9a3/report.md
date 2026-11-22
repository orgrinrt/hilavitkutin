# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 16.0x slower than the field

dep_graph_dense (6.03 us) is 16.0x the fastest (376 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (376 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.0x the fastest

Fastest dep_graph_csr (376 ns) to slowest dep_graph_dense (6.03 us): 16.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_hybrid is inconsistent: worst-20% is 3.0x its best-20%

dep_graph_hybrid's best 20% of batches run at 398 ns but its worst 20% at 1.19 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 376.3 ns median
- 2 variants significantly slower than baseline
- Spread: 16.01x (fastest 376.3 ns, slowest 6025.9 ns)

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
| dep_graph_csr | 2905ns | 2264ns | 2092ns | 2266ns | 5635ns | base |
| dep_graph_dense | 7406ns | 7939ns | 6487ns | 7522ns | 7975ns | +154.93% |
| dep_graph_hybrid | 3276ns | 2293ns | 2283ns | 2449ns | 6751ns | +12.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 482ns | 348ns | 932ns | base | 0.133 |
| dep_graph_dense | 5621ns | 4924ns | 6048ns | +1065.51% | 0.011 |
| dep_graph_hybrid | 576ns | 398ns | 1189ns | +19.43% | 0.111 |

## Performance model

- Peak throughput: **0.184 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.170 | 92.5% |
| dep_graph_dense | 0.011 | 5.8% |
| dep_graph_hybrid | 0.159 | 86.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2905ns | 2905ns | base |
| dep_graph_dense | 7406ns | 7406ns | +154.93% |
| dep_graph_hybrid | 3276ns | 3276ns | +12.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 376ns | base | --- | [376, 378] | --- | --- | --- | --- |
| dep_graph_dense | 6026ns | +5645.1ns (+1500.2%) | [+4898, +5648]ns | [6023, 6028] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 404ns | +28.1ns (+7.5%) | [+27, +30]ns | [403, 404] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 379ns | +1492.5% | +6.3% |
| 1 | 2 | 379ns | +1198.4% | +6.9% |
| 1 | 3 | 375ns | +1507.8% | +7.8% |
| 1 | 4 | 318ns | +1450.5% | +25.4% |
| 1 | 5 | 375ns | +1213.1% | +6.7% |
| 1 | 6 | 376ns | +1502.1% | +6.8% |
| 1 | 7 | 378ns | +1495.8% | +150.4% |
| 1 | 8 | 377ns | +1501.6% | +6.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.084 | ok |
| dep_graph_dense | -0.142 | ok |
| dep_graph_hybrid | -0.163 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 61.2ns | 482.3ns | 12.7% | HIGH |
| dep_graph_dense | 871.7ns | 5620.9ns | 15.5% | HIGH |
| dep_graph_hybrid | 72.2ns | 576.0ns | 12.5% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 348.2-932.2 ns)
    348.2 |########################################
    377.4 |##############################
    406.6 |
    435.8 |
    465.0 |
    494.2 |
    523.4 |
    552.6 |
    581.8 |
    611.0 |
    640.2 |
    669.4 |
    698.6 |
    727.8 |
    757.0 |
    786.2 |
    815.4 |
    844.6 |
    873.8 |#
    903.0 |
  (14 below, 19 above range)

dep_graph_dense (n=160, range 4924.1-6048.5 ns)
   4924.1 |###################
   4980.3 |
   5036.5 |
   5092.8 |
   5149.0 |
   5205.2 |
   5261.4 |
   5317.6 |
   5373.9 |
   5430.1 |
   5486.3 |
   5542.5 |
   5598.7 |
   5655.0 |
   5711.2 |
   5767.4 |
   5823.6 |
   5879.8 |
   5936.0 |
   5992.3 |########################################
  (13 below, 3 above range)

dep_graph_hybrid (n=160, range 398.4-1189.3 ns)
    398.4 |########################################
    437.9 |
    477.5 |
    517.0 |
    556.6 |
    596.1 |
    635.7 |
    675.2 |
    714.8 |
    754.3 |
    793.8 |
    833.4 |
    872.9 |
    912.5 |
    952.0 |
    991.6 |
   1031.1 |
   1070.6 |
   1110.2 |#
   1149.7 |
  (13 below, 20 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=56.1% (high variance, measurements may be unstable)
- **dep_graph_csr**: bridge=12.5% of algo (FFI overhead may distort results)
- **dep_graph_dense**: bridge=15.5% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=56.0% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: bridge=12.6% of algo (FFI overhead may distort results)
