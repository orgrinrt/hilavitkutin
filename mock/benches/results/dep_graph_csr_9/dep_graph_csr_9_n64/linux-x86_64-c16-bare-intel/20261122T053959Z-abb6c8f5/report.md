# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_hybrid dominates: 13% faster than the next best (dep_graph_csr)

dep_graph_hybrid (332 ns) leads dep_graph_csr (375 ns) by 13%, a clear separation rather than a photo finish. CV 91.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 14.8x slower than the field

dep_graph_dense (4.94 us) is 14.8x the fastest (332 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_hybrid is fastest but the noisiest (CV 91.7%)

dep_graph_hybrid wins on median (332 ns) yet has the highest variance (CV 91.7%), while dep_graph_dense is the steadiest (CV 10.8%, 4.94 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 14.8x the fastest

Fastest dep_graph_hybrid (332 ns) to slowest dep_graph_dense (4.94 us): 14.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_hybrid is inconsistent: worst-20% is 3.3x its best-20%

dep_graph_hybrid's best 20% of batches run at 322 ns but its worst 20% at 1.06 us (3.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dep_graph_hybrid** at 332.4 ns median (-11.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 14.85x (fastest 332.4 ns, slowest 4936.7 ns)

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
| dep_graph_csr | 2856ns | 2253ns | 1843ns | 2127ns | 6057ns | base |
| dep_graph_dense | 7037ns | 6500ns | 6481ns | 6918ns | 7948ns | +146.38% |
| dep_graph_hybrid | 2744ns | 1872ns | 1859ns | 1948ns | 6019ns | -3.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 474ns | 307ns | 1000ns | base | 0.135 |
| dep_graph_dense | 5342ns | 4922ns | 6034ns | +1026.02% | 0.012 |
| dep_graph_hybrid | 483ns | 322ns | 1055ns | +1.80% | 0.133 |

## Performance model

- Peak throughput: **0.209 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.171 | 81.9% |
| dep_graph_dense | 0.013 | 6.2% |
| dep_graph_hybrid | 0.193 | 92.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2856ns | 2856ns | base |
| dep_graph_dense | 7037ns | 7037ns | +146.38% |
| dep_graph_hybrid | 2744ns | 2744ns | -3.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 375ns | base | --- | [374, 375] | --- | --- | --- | --- |
| dep_graph_dense | 4937ns | +4622.4ns (+1233.3%) | [+4615, +4628]ns | [4934, 4940] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 332ns | +22.2ns (+5.9%) | [+19, +24]ns | [332, 334] | YES | 0.0003 | 0.0003 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 375ns | +1508.2% | -12.0% |
| 1 | 2 | 378ns | +1204.7% | -17.0% |
| 1 | 3 | 376ns | +1214.8% | +8.3% |
| 1 | 4 | 309ns | +1491.3% | +8.2% |
| 1 | 5 | 312ns | +1832.2% | +6.6% |
| 1 | 6 | 307ns | +1506.3% | +8.3% |
| 1 | 7 | 307ns | +1505.5% | +7.7% |
| 1 | 8 | 375ns | +1508.9% | -11.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.378 | moderate+ |
| dep_graph_dense | -0.279 | moderate- |
| dep_graph_hybrid | -0.285 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 59.3ns | 474.4ns | 12.5% | HIGH |
| dep_graph_dense | 820.8ns | 5342.4ns | 15.4% | HIGH |
| dep_graph_hybrid | 65.3ns | 483.0ns | 13.5% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 306.9-999.6 ns)
    306.9 |#####################################
    341.5 |########################################
    376.1 |#############
    410.8 |
    445.4 |
    480.0 |
    514.7 |
    549.3 |
    584.0 |
    618.6 |#
    653.2 |
    687.9 |
    722.5 |
    757.1 |
    791.8 |
    826.4 |
    861.0 |
    895.7 |
    930.3 |#
    964.9 |
  (16 below, 20 above range)

dep_graph_dense (n=160, range 4922.5-6034.5 ns)
   4922.5 |########################################
   4978.1 |
   5033.7 |
   5089.3 |
   5144.9 |
   5200.5 |
   5256.1 |
   5311.7 |
   5367.3 |
   5422.9 |
   5478.5 |
   5534.1 |
   5589.7 |
   5645.3 |
   5700.9 |
   5756.5 |
   5812.1 |
   5867.7 |
   5923.3 |
   5978.9 |####################
  (14 below, 15 above range)

dep_graph_hybrid (n=160, range 322.3-1055.5 ns)
    322.3 |########################################
    358.9 |
    395.6 |#######
    432.2 |
    468.9 |
    505.6 |
    542.2 |
    578.9 |
    615.5 |
    652.2 |
    688.9 |
    725.5 |
    762.2 |
    798.8 |
    835.5 |
    872.2 |
    908.8 |
    945.5 |
    982.1 |#
   1018.8 |#
  (14 below, 19 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=61.7% (high variance, measurements may be unstable)
- **dep_graph_csr**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **dep_graph_csr**: bridge=12.3% of algo (FFI overhead may distort results)
- **dep_graph_dense**: bridge=15.5% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=63.1% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **dep_graph_hybrid**: bridge=12.5% of algo (FFI overhead may distort results)
