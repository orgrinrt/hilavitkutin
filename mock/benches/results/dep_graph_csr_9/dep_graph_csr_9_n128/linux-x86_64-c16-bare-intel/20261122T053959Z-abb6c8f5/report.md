# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 29.5x slower than the field

dep_graph_dense (21.31 us) is 29.5x the fastest (723 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 29.5x the fastest

Fastest dep_graph_hybrid (723 ns) to slowest dep_graph_dense (21.31 us): 29.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 2.9x its best-20%

dep_graph_csr's best 20% of batches run at 609 ns but its worst 20% at 1.76 us (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dep_graph_hybrid** at 722.7 ns median (-2.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 29.49x (fastest 722.7 ns, slowest 21309.7 ns)

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
| Counter frequency | 2112878130 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dep_graph_csr | 3210ns | 2626ns | 2147ns | 2545ns | 6265ns | base |
| dep_graph_dense | 23051ns | 23066ns | 20734ns | 23047ns | 25381ns | +618.19% |
| dep_graph_hybrid | 3042ns | 2602ns | 2127ns | 2456ns | 5715ns | -5.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 907ns | 609ns | 1761ns | base | 0.141 |
| dep_graph_dense | 21297ns | 19158ns | 23444ns | +2247.99% | 0.006 |
| dep_graph_hybrid | 845ns | 591ns | 1580ns | -6.89% | 0.152 |

## Performance model

- Peak throughput: **0.217 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.172 | 79.4% |
| dep_graph_dense | 0.006 | 2.8% |
| dep_graph_hybrid | 0.177 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 3210ns | 3210ns | base |
| dep_graph_dense | 23051ns | 23051ns | +618.19% |
| dep_graph_hybrid | 3042ns | 3042ns | -5.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 744ns | base | --- | [744, 746] | --- | --- | --- | --- |
| dep_graph_dense | 21310ns | +19652.7ns (+2641.3%) | [+18556, +22426]ns | [19170, 23414] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 723ns | -20.1ns (-2.7%) | [-22, -18]ns | [721, 724] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 744ns | +2475.5% | -20.5% |
| 1 | 2 | 747ns | +2465.3% | -3.1% |
| 1 | 3 | 748ns | +2462.6% | -3.1% |
| 1 | 4 | 610ns | +3739.6% | -2.9% |
| 1 | 5 | 747ns | +3036.7% | -3.3% |
| 1 | 6 | 744ns | +3048.2% | -1.5% |
| 1 | 7 | 610ns | +3044.0% | -3.1% |
| 1 | 8 | 610ns | +3740.0% | -1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.059 | ok |
| dep_graph_dense | 0.125 | ok |
| dep_graph_hybrid | -0.144 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 42.9ns | 907.0ns | 4.7% |  |
| dep_graph_dense | 630.5ns | 21296.7ns | 3.0% |  |
| dep_graph_hybrid | 56.2ns | 844.6ns | 6.7% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 608.7-1760.5 ns)
    608.7 |#############
    666.3 |
    723.9 |########################################
    781.4 |
    839.0 |
    896.6 |
    954.2 |
   1011.8 |
   1069.4 |
   1127.0 |
   1184.6 |
   1242.2 |
   1299.8 |
   1357.4 |
   1415.0 |
   1472.6 |
   1530.1 |
   1587.7 |
   1645.3 |
   1702.9 |
  (21 below, 25 above range)

dep_graph_dense (n=160, range 19158.3-23444.0 ns)
  19158.3 |##################################
  19372.6 |
  19586.9 |
  19801.2 |
  20015.5 |
  20229.8 |
  20444.0 |
  20658.3 |
  20872.6 |
  21086.9 |
  21301.2 |
  21515.4 |
  21729.7 |
  21944.0 |
  22158.3 |
  22372.6 |
  22586.9 |
  22801.1 |
  23015.4 |
  23229.7 |########################################
  (12 below, 2 above range)

dep_graph_hybrid (n=160, range 590.7-1580.0 ns)
    590.7 |###########################
    640.1 |
    689.6 |########################################
    739.1 |#
    788.5 |
    838.0 |
    887.5 |
    936.9 |
    986.4 |
   1035.9 |
   1085.3 |
   1134.8 |
   1184.3 |
   1233.7 |
   1283.2 |
   1332.6 |
   1382.1 |
   1431.6 |
   1481.0 |
   1530.5 |
  (16 below, 20 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=53.3% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: CV=53.8% (high variance, measurements may be unstable)
