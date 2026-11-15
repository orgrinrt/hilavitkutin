# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 31.9x slower than the field

dep_graph_dense (23.42 us) is 31.9x the fastest (734 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_hybrid is fastest but the noisiest (CV 67.9%)

dep_graph_hybrid wins on median (734 ns) yet has the highest variance (CV 67.9%), while dep_graph_dense is the steadiest (CV 7.9%, 23.42 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 31.9x the fastest

Fastest dep_graph_hybrid (734 ns) to slowest dep_graph_dense (23.42 us): 31.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_hybrid is inconsistent: worst-20% is 2.5x its best-20%

dep_graph_hybrid's best 20% of batches run at 723 ns but its worst 20% at 1.78 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### dep_graph_hybrid's edge over baseline is significant but tiny (-12 ns, 1.65%)

dep_graph_hybrid differs from baseline dep_graph_csr by -12 ns (1.65%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dep_graph_hybrid** at 733.6 ns median (-1.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 31.93x (fastest 733.6 ns, slowest 23424.1 ns)

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
| dep_graph_csr | 3089ns | 2625ns | 2616ns | 2627ns | 4950ns | base |
| dep_graph_dense | 24212ns | 25344ns | 20730ns | 24964ns | 25441ns | +683.76% |
| dep_graph_hybrid | 3362ns | 2615ns | 2599ns | 2626ns | 6335ns | +8.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 875ns | 743ns | 1398ns | base | 0.146 |
| dep_graph_dense | 22376ns | 19160ns | 23506ns | +2455.89% | 0.006 |
| dep_graph_hybrid | 945ns | 723ns | 1783ns | +7.94% | 0.135 |

## Performance model

- Peak throughput: **0.177 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.171 | 96.9% |
| dep_graph_dense | 0.005 | 3.1% |
| dep_graph_hybrid | 0.174 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 3089ns | 3089ns | base |
| dep_graph_dense | 24212ns | 24212ns | +683.76% |
| dep_graph_hybrid | 3362ns | 3362ns | +8.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 746ns | base | --- | [746, 747] | --- | --- | --- | --- |
| dep_graph_dense | 23424ns | +22672.7ns (+3037.4%) | [+22669, +22678]ns | [23420, 23426] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 734ns | -12.4ns (-1.7%) | [-17, -9]ns | [732, 737] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 746ns | +3038.3% | -2.8% |
| 1 | 2 | 743ns | +2479.4% | -0.8% |
| 1 | 3 | 747ns | +3037.1% | -3.0% |
| 1 | 4 | 746ns | +3042.4% | -1.8% |
| 1 | 5 | 747ns | +2465.0% | -3.1% |
| 1 | 6 | 747ns | +3034.9% | +1.0% |
| 1 | 7 | 747ns | +3036.2% | -2.1% |
| 1 | 8 | 743ns | +3052.8% | +100.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.242 | moderate- |
| dep_graph_dense | -0.374 | moderate- |
| dep_graph_hybrid | -0.022 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 39.4ns | 875.5ns | 4.5% |  |
| dep_graph_dense | 665.7ns | 22375.9ns | 3.0% |  |
| dep_graph_hybrid | 55.2ns | 945.0ns | 5.8% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 742.7-1397.5 ns)
    742.7 |########################################
    775.4 |
    808.2 |
    840.9 |
    873.7 |
    906.4 |
    939.1 |
    971.9 |
   1004.6 |
   1037.4 |
   1070.1 |
   1102.8 |
   1135.6 |
   1168.3 |
   1201.1 |
   1233.8 |
   1266.6 |
   1299.3 |
   1332.0 |
   1364.8 |
  (11 below, 14 above range)

dep_graph_dense (n=160, range 19160.0-23506.5 ns)
  19160.0 |#########
  19377.4 |
  19594.7 |
  19812.0 |
  20029.3 |
  20246.6 |
  20464.0 |
  20681.3 |
  20898.6 |
  21115.9 |
  21333.2 |
  21550.6 |
  21767.9 |
  21985.2 |
  22202.5 |
  22419.9 |
  22637.2 |
  22854.5 |
  23071.8 |
  23289.1 |########################################
  (14 below, 5 above range)

dep_graph_hybrid (n=160, range 723.4-1783.3 ns)
    723.4 |########################################
    776.4 |
    829.4 |
    882.4 |
    935.4 |
    988.4 |#
   1041.4 |
   1094.4 |
   1147.4 |
   1200.4 |
   1253.4 |
   1306.3 |
   1359.3 |
   1412.3 |
   1465.3 |
   1518.3 |
   1571.3 |
   1624.3 |
   1677.3 |
   1730.3 |
  (16 below, 17 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=48.3% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: CV=52.7% (high variance, measurements may be unstable)
