# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 16.0x slower than the field

dep_graph_dense (6.02 us) is 16.0x the fastest (377 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 80.9%)

dep_graph_csr wins on median (377 ns) yet has the highest variance (CV 80.9%), while dep_graph_dense is the steadiest (CV 8.8%, 6.02 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (377 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.0x the fastest

Fastest dep_graph_csr (377 ns) to slowest dep_graph_dense (6.02 us): 16.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 3.5x its best-20%

dep_graph_csr's best 20% of batches run at 308 ns but its worst 20% at 1.09 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 377.4 ns median
- 2 variants significantly slower than baseline
- Spread: 15.95x (fastest 377.4 ns, slowest 6021.6 ns)

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
| dep_graph_csr | 3137ns | 2256ns | 1845ns | 2408ns | 6618ns | base |
| dep_graph_dense | 7398ns | 7925ns | 6483ns | 7516ns | 7958ns | +135.79% |
| dep_graph_hybrid | 2779ns | 2282ns | 2036ns | 2283ns | 5013ns | -11.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 521ns | 308ns | 1091ns | base | 0.123 |
| dep_graph_dense | 5618ns | 4926ns | 6041ns | +977.90% | 0.011 |
| dep_graph_hybrid | 487ns | 349ns | 877ns | -6.57% | 0.131 |

## Performance model

- Peak throughput: **0.208 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.170 | 81.6% |
| dep_graph_dense | 0.011 | 5.1% |
| dep_graph_hybrid | 0.159 | 76.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 3137ns | 3137ns | base |
| dep_graph_dense | 7398ns | 7398ns | +135.79% |
| dep_graph_hybrid | 2779ns | 2779ns | -11.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 377ns | base | --- | [375, 378] | --- | --- | --- | --- |
| dep_graph_dense | 6022ns | +5201.6ns (+1378.1%) | [+4907, +5510]ns | [6015, 6022] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 404ns | +22.4ns (+5.9%) | [+6, +27]ns | [401, 406] | YES | 0.0003 | 0.0003 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 309ns | +1849.7% | +29.5% |
| 1 | 2 | 378ns | +1204.5% | -13.7% |
| 1 | 3 | 308ns | +1857.2% | +32.2% |
| 1 | 4 | 310ns | +1492.4% | +32.5% |
| 1 | 5 | 378ns | +1205.3% | +0.3% |
| 1 | 6 | 888ns | +578.6% | -54.2% |
| 1 | 7 | 378ns | +1496.6% | +5.8% |
| 1 | 8 | 375ns | +1509.3% | +7.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.040 | ok |
| dep_graph_dense | -0.141 | ok |
| dep_graph_hybrid | -0.276 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 64.6ns | 521.2ns | 12.4% | HIGH |
| dep_graph_dense | 870.2ns | 5618.4ns | 15.5% | HIGH |
| dep_graph_hybrid | 67.7ns | 487.0ns | 13.9% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 307.9-1090.6 ns)
    307.9 |####################
    347.0 |########################################
    386.2 |#
    425.3 |
    464.4 |#
    503.6 |#
    542.7 |#
    581.8 |#
    621.0 |#
    660.1 |
    699.2 |
    738.4 |
    777.5 |#
    816.6 |
    855.8 |
    894.9 |#
    934.0 |###
    973.2 |
   1012.3 |#
   1051.4 |#
  (18 below, 17 above range)

dep_graph_dense (n=160, range 4925.6-6040.5 ns)
   4925.6 |#################
   4981.4 |
   5037.1 |
   5092.9 |
   5148.6 |
   5204.4 |
   5260.1 |
   5315.8 |
   5371.6 |
   5427.3 |
   5483.1 |
   5538.8 |
   5594.6 |
   5650.3 |
   5706.1 |
   5761.8 |
   5817.6 |
   5873.3 |
   5929.0 |
   5984.8 |########################################
  (16 below, 1 above range)

dep_graph_hybrid (n=160, range 348.6-877.5 ns)
    348.6 |
    375.0 |##########################
    401.4 |########################################
    427.9 |
    454.3 |
    480.8 |
    507.2 |
    533.7 |
    560.1 |
    586.6 |
    613.0 |
    639.4 |
    665.9 |
    692.3 |
    718.8 |
    745.2 |
    771.7 |
    798.1 |
    824.6 |
    851.0 |
  (18 below, 17 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=58.6% (high variance, measurements may be unstable)
- **dep_graph_csr**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **dep_graph_csr**: bridge=12.4% of algo (FFI overhead may distort results)
- **dep_graph_dense**: bridge=15.4% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=57.3% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: bridge=12.4% of algo (FFI overhead may distort results)
