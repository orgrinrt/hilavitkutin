# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 6.9x slower than the field

dep_graph_dense (1.32 us) is 6.9x the fastest (193 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 77.5%)

dep_graph_csr wins on median (193 ns) yet has the highest variance (CV 77.5%), while dep_graph_dense is the steadiest (CV 13.5%, 1.32 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (193 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.9x the fastest

Fastest dep_graph_csr (193 ns) to slowest dep_graph_dense (1.32 us): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_csr is inconsistent: worst-20% is 3.4x its best-20%

dep_graph_csr's best 20% of batches run at 158 ns but its worst 20% at 533 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 193.1 ns median
- 2 variants significantly slower than baseline
- Spread: 6.85x (fastest 193.1 ns, slowest 1323.0 ns)

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
| dep_graph_csr | 2720ns | 2067ns | 1690ns | 2044ns | 5780ns | base |
| dep_graph_dense | 3016ns | 2880ns | 2868ns | 2880ns | 3573ns | +10.87% |
| dep_graph_hybrid | 2654ns | 2085ns | 1700ns | 2078ns | 5337ns | -2.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 253ns | 158ns | 533ns | base | 0.127 |
| dep_graph_dense | 1385ns | 1312ns | 1655ns | +447.83% | 0.023 |
| dep_graph_hybrid | 263ns | 165ns | 530ns | +3.97% | 0.122 |

## Performance model

- Peak throughput: **0.203 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.166 | 81.6% |
| dep_graph_dense | 0.024 | 11.9% |
| dep_graph_hybrid | 0.152 | 74.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2720ns | 2720ns | base |
| dep_graph_dense | 3016ns | 3016ns | +10.87% |
| dep_graph_hybrid | 2654ns | 2654ns | -2.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 193ns | base | --- | [193, 193] | --- | --- | --- | --- |
| dep_graph_dense | 1323ns | +1130.0ns (+585.2%) | [+1121, +1132]ns | [1314, 1323] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 210ns | +17.7ns (+9.2%) | [+17, +18]ns | [210, 211] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 158ns | +731.2% | +23.2% |
| 1 | 2 | 158ns | +737.5% | +9.2% |
| 1 | 3 | 193ns | +579.0% | +9.3% |
| 1 | 4 | 194ns | +581.6% | -18.5% |
| 1 | 5 | 193ns | +579.0% | +9.3% |
| 1 | 6 | 193ns | +579.7% | +9.1% |
| 1 | 7 | 157ns | +734.2% | +33.7% |
| 1 | 8 | 193ns | +733.4% | +9.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.063 | ok |
| dep_graph_dense | -0.030 | ok |
| dep_graph_hybrid | -0.272 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 63.7ns | 252.8ns | 25.2% | HIGH |
| dep_graph_dense | 833.1ns | 1384.9ns | 60.2% | HIGH |
| dep_graph_hybrid | 76.3ns | 262.8ns | 29.0% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 157.6-532.9 ns)
    157.6 |################
    176.4 |########################################
    195.1 |#
    213.9 |
    232.7 |
    251.4 |
    270.2 |
    289.0 |#
    307.7 |
    326.5 |
    345.2 |
    364.0 |
    382.8 |
    401.5 |
    420.3 |
    439.0 |#
    457.8 |#
    476.6 |
    495.3 |#
    514.1 |
  (13 below, 17 above range)

dep_graph_dense (n=160, range 1312.0-1654.7 ns)
   1312.0 |########################################
   1329.2 |#
   1346.3 |
   1363.4 |
   1380.6 |
   1397.7 |
   1414.8 |
   1432.0 |
   1449.1 |
   1466.2 |
   1483.4 |
   1500.5 |
   1517.6 |
   1534.8 |
   1551.9 |
   1569.0 |
   1586.2 |
   1603.3 |#####
   1620.5 |#
   1637.6 |
  (14 below, 6 above range)

dep_graph_hybrid (n=160, range 165.2-529.6 ns)
    165.2 |#########
    183.4 |##########
    201.7 |########################################
    219.9 |#
    238.1 |
    256.3 |
    274.5 |
    292.8 |
    311.0 |
    329.2 |
    347.4 |
    365.7 |
    383.9 |
    402.1 |
    420.3 |
    438.5 |
    456.8 |#
    475.0 |
    493.2 |
    511.4 |
  (16 below, 20 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=59.2% (high variance, measurements may be unstable)
- **dep_graph_csr**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **dep_graph_csr**: bridge=25.3% of algo (FFI overhead may distort results)
- **dep_graph_dense**: bridge=60.4% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=62.0% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **dep_graph_hybrid**: bridge=24.7% of algo (FFI overhead may distort results)
