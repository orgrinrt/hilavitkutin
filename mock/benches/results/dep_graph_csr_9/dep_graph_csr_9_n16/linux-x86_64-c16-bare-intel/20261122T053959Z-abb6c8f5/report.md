# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 305% faster than the next best (dep_graph_dense)

dep_graph_csr (89 ns) leads dep_graph_dense (360 ns) by 305%, a clear separation rather than a photo finish. CV 89.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_hybrid is an outlier: 5.0x slower than the field

dep_graph_hybrid (447 ns) is 5.0x the fastest (89 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 89.4%)

dep_graph_csr wins on median (89 ns) yet has the highest variance (CV 89.4%), while dep_graph_hybrid is the steadiest (CV 32.7%, 447 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (89 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest dep_graph_csr (89 ns) to slowest dep_graph_hybrid (447 ns): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 3.7x its best-20%

dep_graph_csr's best 20% of batches run at 73 ns but its worst 20% at 267 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 88.9 ns median
- 2 variants significantly slower than baseline
- Spread: 5.03x (fastest 88.9 ns, slowest 446.8 ns)

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
| dep_graph_csr | 2613ns | 1966ns | 1606ns | 1852ns | 5901ns | base |
| dep_graph_dense | 2473ns | 1928ns | 1911ns | 2070ns | 4244ns | -5.35% |
| dep_graph_hybrid | 2349ns | 2345ns | 1919ns | 2241ns | 3102ns | -10.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 118ns | 73ns | 267ns | base | 0.135 |
| dep_graph_dense | 462ns | 358ns | 790ns | +291.36% | 0.035 |
| dep_graph_hybrid | 447ns | 366ns | 588ns | +278.56% | 0.036 |

## Performance model

- Peak throughput: **0.220 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.180 | 81.8% |
| dep_graph_dense | 0.044 | 20.2% |
| dep_graph_hybrid | 0.036 | 16.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2613ns | 2613ns | base |
| dep_graph_dense | 2473ns | 2473ns | -5.35% |
| dep_graph_hybrid | 2349ns | 2349ns | -10.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 89ns | base | --- | [89, 89] | --- | --- | --- | --- |
| dep_graph_dense | 360ns | +286.1ns (+321.9%) | [+286, +287]ns | [360, 375] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 447ns | +357.4ns (+402.0%) | [+305, +359]ns | [446, 447] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 73ns | +394.6% | +404.3% |
| 1 | 2 | 73ns | +391.4% | +403.3% |
| 1 | 3 | 89ns | +303.5% | +403.7% |
| 1 | 4 | 89ns | +393.0% | +404.3% |
| 1 | 5 | 73ns | +671.0% | +514.8% |
| 1 | 6 | 89ns | +303.6% | +401.0% |
| 1 | 7 | 89ns | +392.6% | +310.4% |
| 1 | 8 | 73ns | +392.9% | +514.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.126 | ok |
| dep_graph_dense | -0.057 | ok |
| dep_graph_hybrid | 0.058 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 64.9ns | 118.1ns | 55.0% | HIGH |
| dep_graph_dense | 1041.9ns | 462.3ns | 225.4% | HIGH |
| dep_graph_hybrid | 988.4ns | 447.2ns | 221.0% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 72.7-266.5 ns)
     72.7 |########################
     82.4 |########################################
     92.1 |
    101.8 |
    111.5 |
    121.2 |
    130.9 |
    140.5 |
    150.2 |
    159.9 |
    169.6 |
    179.3 |#
    189.0 |
    198.7 |
    208.4 |###
    218.1 |
    227.8 |
    237.5 |
    247.2 |
    256.8 |
  (25 below, 18 above range)

dep_graph_dense (n=160, range 358.3-789.6 ns)
    358.3 |########################################
    379.9 |
    401.4 |
    423.0 |###################
    444.6 |##
    466.1 |
    487.7 |
    509.3 |
    530.8 |
    552.4 |
    573.9 |
    595.5 |
    617.1 |
    638.6 |
    660.2 |
    681.8 |
    703.3 |
    724.9 |
    746.5 |
    768.0 |
  (16 below, 17 above range)

dep_graph_hybrid (n=160, range 365.9-588.0 ns)
    365.9 |################
    377.0 |
    388.1 |
    399.2 |
    410.4 |
    421.5 |
    432.6 |
    443.7 |########################################
    454.8 |##
    465.9 |
    477.0 |
    488.1 |
    499.2 |
    510.3 |
    521.4 |
    532.5 |
    543.6 |
    554.7 |
    565.8 |
    576.9 |
  (17 below, 7 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=67.3% (high variance, measurements may be unstable)
- **dep_graph_csr**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **dep_graph_csr**: bridge=54.0% of algo (FFI overhead may distort results)
- **dep_graph_dense**: CV=44.9% (high variance, measurements may be unstable)
- **dep_graph_dense**: bridge=226.4% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=32.7% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: bridge=220.6% of algo (FFI overhead may distort results)
