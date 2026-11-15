# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 393% faster than the next best (dep_graph_dense)

dep_graph_csr (89 ns) leads dep_graph_dense (439 ns) by 393%, a clear separation rather than a photo finish. CV 90.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_hybrid is an outlier: 5.0x slower than the field

dep_graph_hybrid (447 ns) is 5.0x the fastest (89 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 90.0%)

dep_graph_csr wins on median (89 ns) yet has the highest variance (CV 90.0%), while dep_graph_dense is the steadiest (CV 33.9%, 439 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (89 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest dep_graph_csr (89 ns) to slowest dep_graph_hybrid (447 ns): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 3.7x its best-20%

dep_graph_csr's best 20% of batches run at 73 ns but its worst 20% at 273 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 89.0 ns median
- 2 variants significantly slower than baseline
- Spread: 5.02x (fastest 89.0 ns, slowest 447.2 ns)

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
| dep_graph_csr | 2708ns | 1970ns | 1612ns | 1971ns | 6013ns | base |
| dep_graph_dense | 2399ns | 2338ns | 1913ns | 2321ns | 3120ns | -11.40% |
| dep_graph_hybrid | 2535ns | 2344ns | 2102ns | 2346ns | 3536ns | -6.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 123ns | 73ns | 273ns | base | 0.130 |
| dep_graph_dense | 450ns | 360ns | 584ns | +266.93% | 0.036 |
| dep_graph_hybrid | 482ns | 401ns | 668ns | +292.94% | 0.033 |

## Performance model

- Peak throughput: **0.220 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.180 | 81.8% |
| dep_graph_dense | 0.036 | 16.6% |
| dep_graph_hybrid | 0.036 | 16.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2708ns | 2708ns | base |
| dep_graph_dense | 2399ns | 2399ns | -11.40% |
| dep_graph_hybrid | 2535ns | 2535ns | -6.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 89ns | base | --- | [89, 89] | --- | --- | --- | --- |
| dep_graph_dense | 439ns | +349.4ns (+392.6%) | [+349, +350]ns | [438, 439] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 447ns | +358.2ns (+402.5%) | [+357, +359]ns | [447, 448] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 89ns | +395.9% | +312.1% |
| 1 | 2 | 73ns | +393.3% | +513.4% |
| 1 | 3 | 89ns | +307.6% | +401.3% |
| 1 | 4 | 89ns | +392.0% | +400.3% |
| 1 | 5 | 89ns | +393.2% | +403.4% |
| 1 | 6 | 89ns | +392.6% | +403.7% |
| 1 | 7 | 89ns | +392.4% | +402.2% |
| 1 | 8 | 73ns | +503.1% | +515.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.204 | moderate- |
| dep_graph_dense | 0.278 | moderate+ |
| dep_graph_hybrid | -0.009 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 67.5ns | 122.7ns | 55.0% | HIGH |
| dep_graph_dense | 1013.1ns | 450.2ns | 225.1% | HIGH |
| dep_graph_hybrid | 1067.4ns | 482.1ns | 221.4% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 72.8-272.6 ns)
     72.8 |#####
     82.8 |########################################
     92.8 |
    102.7 |
    112.7 |
    122.7 |
    132.7 |
    142.7 |
    152.7 |
    162.7 |
    172.7 |
    182.7 |
    192.7 |
    202.7 |
    212.7 |
    222.6 |
    232.6 |
    242.6 |
    252.6 |
    262.6 |#
  (18 below, 18 above range)

dep_graph_dense (n=160, range 360.5-584.2 ns)
    360.5 |#########
    371.7 |#
    382.8 |
    394.0 |
    405.2 |
    416.4 |
    427.6 |################################
    438.8 |########################################
    450.0 |#
    461.1 |
    472.3 |
    483.5 |
    494.7 |
    505.9 |
    517.1 |
    528.2 |
    539.4 |
    550.6 |
    561.8 |
    573.0 |
  (19 below, 6 above range)

dep_graph_hybrid (n=160, range 400.9-668.1 ns)
    400.9 |
    414.3 |
    427.7 |
    441.0 |########################################
    454.4 |###
    467.7 |
    481.1 |
    494.4 |
    507.8 |
    521.2 |
    534.5 |
    547.9 |
    561.2 |
    574.6 |
    587.9 |
    601.3 |
    614.7 |
    628.0 |
    641.4 |
    654.7 |
  (18 below, 10 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=65.3% (high variance, measurements may be unstable)
- **dep_graph_csr**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **dep_graph_csr**: bridge=55.6% of algo (FFI overhead may distort results)
- **dep_graph_dense**: CV=33.0% (high variance, measurements may be unstable)
- **dep_graph_dense**: bridge=225.9% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=35.8% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: bridge=221.7% of algo (FFI overhead may distort results)
