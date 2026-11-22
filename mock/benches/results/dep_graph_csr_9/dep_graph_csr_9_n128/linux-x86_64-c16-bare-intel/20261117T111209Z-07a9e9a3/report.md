# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 18% faster than the next best (dep_graph_hybrid)

dep_graph_csr (612 ns) leads dep_graph_hybrid (724 ns) by 18%, a clear separation rather than a photo finish. CV 78.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 38.3x slower than the field

dep_graph_dense (23.43 us) is 38.3x the fastest (612 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 78.6%)

dep_graph_csr wins on median (612 ns) yet has the highest variance (CV 78.6%), while dep_graph_dense is the steadiest (CV 7.9%, 23.43 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (612 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 38.3x the fastest

Fastest dep_graph_csr (612 ns) to slowest dep_graph_dense (23.43 us): 38.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 2.7x its best-20%

dep_graph_csr's best 20% of batches run at 608 ns but its worst 20% at 1.65 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 611.6 ns median
- 2 variants significantly slower than baseline
- Spread: 38.31x (fastest 611.6 ns, slowest 23428.5 ns)

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
| dep_graph_csr | 2982ns | 2160ns | 2149ns | 2295ns | 5877ns | base |
| dep_graph_dense | 24215ns | 25354ns | 20737ns | 24972ns | 25421ns | +712.05% |
| dep_graph_hybrid | 3029ns | 2613ns | 2605ns | 2614ns | 4697ns | +1.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 841ns | 608ns | 1648ns | base | 0.152 |
| dep_graph_dense | 22374ns | 19161ns | 23483ns | +2560.44% | 0.006 |
| dep_graph_hybrid | 839ns | 721ns | 1298ns | -0.28% | 0.153 |

## Performance model

- Peak throughput: **0.211 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.209 | 99.4% |
| dep_graph_dense | 0.005 | 2.6% |
| dep_graph_hybrid | 0.177 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2982ns | 2982ns | base |
| dep_graph_dense | 24215ns | 24215ns | +712.05% |
| dep_graph_hybrid | 3029ns | 3029ns | +1.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 612ns | base | --- | [610, 618] | --- | --- | --- | --- |
| dep_graph_dense | 23429ns | +22694.2ns (+3710.6%) | [+22686, +22722]ns | [23426, 23431] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 724ns | +113.3ns (+18.5%) | [+110, +114]ns | [723, 725] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 609ns | +3747.5% | +18.7% |
| 1 | 2 | 609ns | +3750.8% | +19.8% |
| 1 | 3 | 608ns | +3753.4% | +19.1% |
| 1 | 4 | 608ns | +3050.6% | +18.8% |
| 1 | 5 | 744ns | +3052.2% | -2.4% |
| 1 | 6 | 612ns | +3728.5% | +18.1% |
| 1 | 7 | 609ns | +3044.8% | +18.7% |
| 1 | 8 | 743ns | +3053.5% | -2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.199 | ok |
| dep_graph_dense | -0.375 | moderate- |
| dep_graph_hybrid | -0.154 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 38.6ns | 841.0ns | 4.6% |  |
| dep_graph_dense | 661.0ns | 22373.8ns | 3.0% |  |
| dep_graph_hybrid | 60.4ns | 838.6ns | 7.2% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 607.9-1648.4 ns)
    607.9 |########################################
    660.0 |
    712.0 |################
    764.0 |
    816.0 |
    868.0 |
    920.1 |
    972.1 |
   1024.1 |
   1076.1 |
   1128.2 |
   1180.2 |
   1232.2 |
   1284.2 |
   1336.3 |
   1388.3 |
   1440.3 |
   1492.3 |
   1544.3 |
   1596.4 |
  (17 below, 18 above range)

dep_graph_dense (n=160, range 19161.1-23483.4 ns)
  19161.1 |#######
  19377.2 |
  19593.3 |
  19809.4 |
  20025.5 |
  20241.7 |
  20457.8 |
  20673.9 |
  20890.0 |
  21106.1 |
  21322.3 |
  21538.4 |
  21754.5 |
  21970.6 |
  22186.7 |
  22402.8 |
  22619.0 |
  22835.1 |
  23051.2 |
  23267.3 |########################################
  (19 below, 4 above range)

dep_graph_hybrid (n=160, range 721.5-1298.1 ns)
    721.5 |########################################
    750.3 |
    779.2 |
    808.0 |
    836.8 |
    865.6 |
    894.5 |
    923.3 |
    952.1 |
    981.0 |
   1009.8 |
   1038.6 |
   1067.4 |
   1096.3 |
   1125.1 |
   1153.9 |
   1182.8 |
   1211.6 |
   1240.4 |
   1269.2 |
  (10 below, 13 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=57.2% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: CV=44.9% (high variance, measurements may be unstable)
