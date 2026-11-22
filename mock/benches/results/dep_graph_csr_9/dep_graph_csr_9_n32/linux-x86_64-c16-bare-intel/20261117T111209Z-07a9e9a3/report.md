# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 8.3x slower than the field

dep_graph_dense (1.61 us) is 8.3x the fastest (193 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 86.5%)

dep_graph_csr wins on median (193 ns) yet has the highest variance (CV 86.5%), while dep_graph_dense is the steadiest (CV 8.9%, 1.61 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (193 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.3x the fastest

Fastest dep_graph_csr (193 ns) to slowest dep_graph_dense (1.61 us): 8.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### dep_graph_hybrid is inconsistent: worst-20% is 3.4x its best-20%

dep_graph_hybrid's best 20% of batches run at 166 ns but its worst 20% at 563 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 193.4 ns median
- 1 variant significantly slower than baseline
- Spread: 8.30x (fastest 193.4 ns, slowest 1605.3 ns)

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
| dep_graph_csr | 2901ns | 2082ns | 2074ns | 2086ns | 6173ns | base |
| dep_graph_dense | 3465ns | 3514ns | 3130ns | 3515ns | 3647ns | +19.43% |
| dep_graph_hybrid | 2705ns | 2089ns | 1708ns | 2029ns | 5731ns | -6.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 270ns | 193ns | 573ns | base | 0.119 |
| dep_graph_dense | 1582ns | 1431ns | 1666ns | +487.01% | 0.020 |
| dep_graph_hybrid | 265ns | 166ns | 563ns | -1.79% | 0.121 |

## Performance model

- Peak throughput: **0.193 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.165 | 85.9% |
| dep_graph_dense | 0.020 | 10.3% |
| dep_graph_hybrid | 0.158 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2901ns | 2901ns | base |
| dep_graph_dense | 3465ns | 3465ns | +19.43% |
| dep_graph_hybrid | 2705ns | 2705ns | -6.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 193ns | base | --- | [193, 194] | --- | --- | --- | --- |
| dep_graph_dense | 1605ns | +1410.9ns (+729.6%) | [+1410, +1412]ns | [1605, 1606] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 202ns | no significant difference | [-2, +8]ns | [201, 204] | no | 0.0687 | 0.0687 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 193ns | +729.8% | +5.1% |
| 1 | 2 | 193ns | +730.6% | -13.6% |
| 1 | 3 | 194ns | +725.8% | +3.1% |
| 1 | 4 | 194ns | +728.0% | +6.0% |
| 1 | 5 | 193ns | +732.8% | +7.9% |
| 1 | 6 | 193ns | +730.8% | -13.9% |
| 1 | 7 | 193ns | +730.6% | +4.8% |
| 1 | 8 | 194ns | +578.6% | -12.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.182 | ok |
| dep_graph_dense | -0.021 | ok |
| dep_graph_hybrid | -0.431 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 68.3ns | 269.6ns | 25.3% | HIGH |
| dep_graph_dense | 962.9ns | 1582.4ns | 60.8% | HIGH |
| dep_graph_hybrid | 67.4ns | 264.7ns | 25.4% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 192.9-572.7 ns)
    192.9 |########################################
    211.9 |#
    230.9 |
    249.9 |
    268.9 |
    287.9 |
    306.9 |
    325.9 |
    344.9 |
    363.8 |
    382.8 |
    401.8 |
    420.8 |
    439.8 |
    458.8 |
    477.8 |
    496.8 |
    515.8 |
    534.7 |
    553.7 |
  (18 below, 24 above range)

dep_graph_dense (n=160, range 1430.6-1665.8 ns)
   1430.6 |
   1442.4 |
   1454.1 |
   1465.9 |
   1477.7 |
   1489.4 |
   1501.2 |
   1512.9 |
   1524.7 |
   1536.5 |
   1548.2 |
   1560.0 |
   1571.7 |
   1583.5 |
   1595.2 |########################################
   1607.0 |#######
   1618.8 |#####
   1630.5 |
   1642.3 |
   1654.0 |
  (19 below, 3 above range)

dep_graph_hybrid (n=160, range 166.1-563.2 ns)
    166.1 |#####################
    186.0 |########################################
    205.8 |###############
    225.7 |
    245.6 |
    265.4 |
    285.3 |
    305.1 |
    325.0 |
    344.8 |
    364.7 |
    384.5 |
    404.4 |
    424.3 |
    444.1 |
    464.0 |
    483.8 |
    503.7 |
    523.5 |
    543.4 |##
  (16 below, 19 above range)

```

## Diagnostics

- **dep_graph_csr**: CV=62.0% (high variance, measurements may be unstable)
- **dep_graph_csr**: bridge=25.3% of algo (FFI overhead may distort results)
- **dep_graph_dense**: bridge=60.8% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: CV=62.2% (high variance, measurements may be unstable)
- **dep_graph_hybrid**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **dep_graph_hybrid**: bridge=25.7% of algo (FFI overhead may distort results)
