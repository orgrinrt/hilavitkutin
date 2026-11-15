# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 63.4x slower than the field

dep_graph_dense (92.67 us) is 63.4x the fastest (1.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 63.4x the fastest

Fastest dep_graph_hybrid (1.46 us) to slowest dep_graph_dense (92.67 us): 63.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 1.5x its best-20%

dep_graph_csr's best 20% of batches run at 1.34 us but its worst 20% at 2.07 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### dep_graph_hybrid's edge over baseline is significant but tiny (-20 ns, 1.38%)

dep_graph_hybrid differs from baseline dep_graph_csr by -20 ns (1.38%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dep_graph_hybrid** at 1462.0 ns median (-1.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 63.38x (fastest 1462.0 ns, slowest 92671.5 ns)

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
| dep_graph_csr | 3574ns | 3369ns | 3037ns | 3368ns | 4728ns | base |
| dep_graph_dense | 92477ns | 94591ns | 83823ns | 94589ns | 94793ns | +2487.76% |
| dep_graph_hybrid | 3529ns | 3347ns | 3338ns | 3348ns | 4262ns | -1.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 1575ns | 1340ns | 2075ns | base | 0.163 |
| dep_graph_dense | 90598ns | 82118ns | 92860ns | +5653.19% | 0.003 |
| dep_graph_hybrid | 1542ns | 1460ns | 1857ns | -2.10% | 0.166 |

## Performance model

- Peak throughput: **0.191 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.172 | 90.1% |
| dep_graph_dense | 0.003 | 1.4% |
| dep_graph_hybrid | 0.175 | 91.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 3574ns | 3574ns | base |
| dep_graph_dense | 92477ns | 92477ns | +2487.76% |
| dep_graph_hybrid | 3529ns | 3529ns | -1.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 1486ns | base | --- | [1482, 1490] | --- | --- | --- | --- |
| dep_graph_dense | 92671ns | +91187.5ns (+6135.4%) | [+91184, +91194]ns | [92667, 92676] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 1462ns | -20.5ns (-1.4%) | [-21, -20]ns | [1461, 1463] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 1488ns | +6128.9% | -1.6% |
| 1 | 2 | 1492ns | +4980.9% | -2.0% |
| 1 | 3 | 1212ns | +7541.8% | +20.5% |
| 1 | 4 | 1488ns | +6125.5% | -1.7% |
| 1 | 5 | 1489ns | +6122.5% | -1.3% |
| 1 | 6 | 1492ns | +6113.9% | -2.0% |
| 1 | 7 | 1482ns | +6155.9% | -1.4% |
| 1 | 8 | 1490ns | +6119.1% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.174 | ok |
| dep_graph_dense | -0.158 | ok |
| dep_graph_hybrid | 0.049 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 26.0ns | 1574.7ns | 1.7% |  |
| dep_graph_dense | 31.3ns | 90597.7ns | 0.0% |  |
| dep_graph_hybrid | 32.3ns | 1541.7ns | 2.1% |  |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 1339.7-2074.9 ns)
   1339.7 |
   1376.5 |
   1413.3 |
   1450.0 |######################################
   1486.8 |########################################
   1523.5 |
   1560.3 |
   1597.0 |
   1633.8 |
   1670.6 |
   1707.3 |
   1744.1 |
   1780.8 |
   1817.6 |
   1854.4 |
   1891.1 |
   1927.9 |
   1964.6 |
   2001.4 |
   2038.1 |
  (17 below, 9 above range)

dep_graph_dense (n=160, range 82118.2-92860.0 ns)
  82118.2 |
  82655.3 |
  83192.4 |
  83729.5 |
  84266.6 |
  84803.7 |
  85340.8 |
  85877.9 |
  86415.0 |
  86952.0 |
  87489.1 |
  88026.2 |
  88563.3 |
  89100.4 |
  89637.5 |
  90174.6 |
  90711.7 |
  91248.8 |
  91785.8 |
  92322.9 |########################################
  (20 below, 4 above range)

dep_graph_hybrid (n=160, range 1459.9-1857.0 ns)
   1459.9 |########################################
   1479.7 |#
   1499.6 |
   1519.5 |
   1539.3 |
   1559.2 |
   1579.0 |
   1598.9 |
   1618.7 |
   1638.6 |
   1658.4 |
   1678.3 |
   1698.2 |
   1718.0 |
   1737.9 |
   1757.7 |
   1777.6 |
   1797.4 |
   1817.3 |
   1837.1 |
  (15 below, 6 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=31.2% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: CV=26.9% (high variance, measurements may be unstable)
