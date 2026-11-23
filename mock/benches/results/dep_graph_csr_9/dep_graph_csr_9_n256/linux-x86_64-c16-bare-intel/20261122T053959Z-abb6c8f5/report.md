# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_hybrid dominates: 23% faster than the next best (dep_graph_csr)

dep_graph_hybrid (1.20 us) leads dep_graph_csr (1.48 us) by 23%, a clear separation rather than a photo finish. CV 39.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 71.7x slower than the field

dep_graph_dense (86.26 us) is 71.7x the fastest (1.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_hybrid is fastest but the noisiest (CV 39.4%)

dep_graph_hybrid wins on median (1.20 us) yet has the highest variance (CV 39.4%), while dep_graph_dense is the steadiest (CV 9.7%, 86.26 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dep_graph_csr shows alternating (throttle bounce) (autocorr -0.66)

dep_graph_csr's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 71.7x the fastest

Fastest dep_graph_hybrid (1.20 us) to slowest dep_graph_dense (86.26 us): 71.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_hybrid is inconsistent: worst-20% is 1.6x its best-20%

dep_graph_hybrid's best 20% of batches run at 1.19 us but its worst 20% at 1.95 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### dep_graph_hybrid's edge over baseline is significant but tiny (-28 ns, 1.87%)

dep_graph_hybrid differs from baseline dep_graph_csr by -28 ns (1.87%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dep_graph_hybrid** at 1203.2 ns median (-18.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 71.69x (fastest 1203.2 ns, slowest 86255.1 ns)

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
| dep_graph_csr | 3303ns | 3367ns | 2752ns | 3218ns | 4107ns | base |
| dep_graph_dense | 86039ns | 88015ns | 77362ns | 86056ns | 94664ns | +2505.21% |
| dep_graph_hybrid | 3239ns | 2750ns | 2730ns | 2983ns | 4513ns | -1.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 1456ns | 1214ns | 1805ns | base | 0.176 |
| dep_graph_dense | 84283ns | 75785ns | 92723ns | +5688.02% | 0.003 |
| dep_graph_hybrid | 1411ns | 1195ns | 1954ns | -3.11% | 0.181 |

## Performance model

- Peak throughput: **0.214 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.173 | 80.6% |
| dep_graph_dense | 0.003 | 1.4% |
| dep_graph_hybrid | 0.213 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 3303ns | 3303ns | base |
| dep_graph_dense | 86039ns | 86039ns | +2505.21% |
| dep_graph_hybrid | 3239ns | 3239ns | -1.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 1482ns | base | --- | [1481, 1490] | --- | --- | --- | --- |
| dep_graph_dense | 86255ns | +83812.6ns (+5657.1%) | [+74568, +91143]ns | [75836, 92640] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 1203ns | -27.7ns (-1.9%) | [-31, -22]ns | [1199, 1460] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 1488ns | +4994.3% | -19.7% |
| 1 | 2 | 1218ns | +7508.8% | -1.8% |
| 1 | 3 | 1482ns | +5018.2% | -19.3% |
| 1 | 4 | 1215ns | +6136.1% | -1.3% |
| 1 | 5 | 1490ns | +4986.6% | -2.0% |
| 1 | 6 | 1493ns | +6106.8% | -2.1% |
| 1 | 7 | 1221ns | +7487.7% | +19.8% |
| 1 | 8 | 1493ns | +6107.3% | -19.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.661 | HIGH- (thermal bounce) |
| dep_graph_dense | 0.126 | ok |
| dep_graph_hybrid | 0.395 | moderate+ |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 24.0ns | 1456.2ns | 1.6% |  |
| dep_graph_dense | 29.8ns | 84282.7ns | 0.0% |  |
| dep_graph_hybrid | 29.3ns | 1410.8ns | 2.1% |  |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 1213.6-1805.3 ns)
   1213.6 |#################
   1243.2 |
   1272.8 |
   1302.3 |
   1331.9 |
   1361.5 |
   1391.1 |
   1420.7 |
   1450.3 |#####
   1479.9 |########################################
   1509.5 |
   1539.0 |
   1568.6 |
   1598.2 |
   1627.8 |
   1657.4 |
   1687.0 |
   1716.6 |
   1746.2 |
   1775.7 |
  (19 below, 6 above range)

dep_graph_dense (n=160, range 75784.6-92723.0 ns)
  75784.6 |################################
  76631.5 |
  77478.4 |
  78325.3 |
  79172.3 |
  80019.2 |
  80866.1 |
  81713.0 |
  82559.9 |
  83406.9 |
  84253.8 |
  85100.7 |
  85947.6 |
  86794.5 |
  87641.5 |
  88488.4 |
  89335.3 |
  90182.2 |
  91029.1 |
  91876.1 |########################################
  (18 below, 5 above range)

dep_graph_hybrid (n=160, range 1194.7-1954.1 ns)
   1194.7 |########################################
   1232.7 |
   1270.6 |
   1308.6 |
   1346.6 |
   1384.5 |
   1422.5 |#######
   1460.5 |########################
   1498.4 |
   1536.4 |
   1574.4 |
   1612.3 |
   1650.3 |
   1688.3 |
   1726.2 |
   1764.2 |
   1802.2 |
   1840.2 |
   1878.1 |
   1916.1 |
  (16 below, 9 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=25.0% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: CV=33.6% (high variance, measurements may be unstable)
