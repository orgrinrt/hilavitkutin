# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 60.2x slower than the field

dep_graph_dense (87.96 us) is 60.2x the fastest (1.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 60.2x the fastest

Fastest dep_graph_hybrid (1.46 us) to slowest dep_graph_dense (87.96 us): 60.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 1.9x its best-20%

dep_graph_csr's best 20% of batches run at 1.33 us but its worst 20% at 2.53 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dep_graph_hybrid** at 1460.2 ns median (-2.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 60.24x (fastest 1460.2 ns, slowest 87964.9 ns)

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
| dep_graph_csr | 3782ns | 3379ns | 3010ns | 3380ns | 5759ns | base |
| dep_graph_dense | 86113ns | 89743ns | 77357ns | 86137ns | 94796ns | +2177.09% |
| dep_graph_hybrid | 3434ns | 3345ns | 2743ns | 3213ns | 4790ns | -9.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 1665ns | 1328ns | 2527ns | base | 0.154 |
| dep_graph_dense | 84357ns | 75780ns | 92859ns | +4965.60% | 0.003 |
| dep_graph_hybrid | 1499ns | 1199ns | 2087ns | -9.96% | 0.171 |

## Performance model

- Peak throughput: **0.214 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.172 | 80.3% |
| dep_graph_dense | 0.003 | 1.4% |
| dep_graph_hybrid | 0.175 | 82.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 3782ns | 3782ns | base |
| dep_graph_dense | 86113ns | 86113ns | +2177.09% |
| dep_graph_hybrid | 3434ns | 3434ns | -9.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 1493ns | base | --- | [1492, 1493] | --- | --- | --- | --- |
| dep_graph_dense | 87965ns | +84526.0ns (+5663.0%) | [+74566, +90924]ns | [75818, 92636] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 1460ns | -33.8ns (-2.3%) | [-270, -32]ns | [1460, 1461] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 2130ns | +4251.9% | -30.9% |
| 1 | 2 | 1492ns | +6106.9% | -19.5% |
| 1 | 3 | 1221ns | +6105.2% | +19.6% |
| 1 | 4 | 1493ns | +4976.7% | -19.4% |
| 1 | 5 | 1488ns | +6129.3% | -1.7% |
| 1 | 6 | 1493ns | +4978.2% | -2.1% |
| 1 | 7 | 1492ns | +4979.7% | -2.1% |
| 1 | 8 | 1482ns | +6149.7% | -19.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.022 | ok |
| dep_graph_dense | -0.125 | ok |
| dep_graph_hybrid | -0.477 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 27.4ns | 1665.3ns | 1.6% |  |
| dep_graph_dense | 29.7ns | 84356.9ns | 0.0% |  |
| dep_graph_hybrid | 31.3ns | 1499.4ns | 2.1% |  |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 1328.2-2526.6 ns)
   1328.2 |
   1388.1 |
   1448.0 |########################################
   1507.9 |#
   1567.8 |
   1627.8 |
   1687.7 |
   1747.6 |
   1807.5 |
   1867.4 |
   1927.4 |
   1987.3 |
   2047.2 |
   2107.1 |
   2167.0 |
   2227.0 |
   2286.9 |
   2346.8 |
   2406.7 |
   2466.6 |
  (19 below, 11 above range)

dep_graph_dense (n=160, range 75779.9-92859.3 ns)
  75779.9 |#################################
  76633.9 |#
  77487.9 |
  78341.8 |
  79195.8 |
  80049.8 |
  80903.7 |
  81757.7 |
  82611.7 |
  83465.6 |
  84319.6 |
  85173.6 |
  86027.5 |
  86881.5 |
  87735.5 |
  88589.4 |
  89443.4 |
  90297.4 |
  91151.3 |
  92005.3 |########################################
  (12 below, 3 above range)

dep_graph_hybrid (n=160, range 1198.8-2087.3 ns)
   1198.8 |#########################
   1243.2 |
   1287.6 |
   1332.1 |
   1376.5 |
   1420.9 |########################################
   1465.4 |####################
   1509.8 |
   1554.2 |
   1598.6 |
   1643.1 |
   1687.5 |
   1731.9 |
   1776.3 |
   1820.8 |
   1865.2 |
   1909.6 |
   1954.0 |
   1998.5 |
   2042.9 |
  (14 below, 10 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=42.2% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: CV=34.6% (high variance, measurements may be unstable)
