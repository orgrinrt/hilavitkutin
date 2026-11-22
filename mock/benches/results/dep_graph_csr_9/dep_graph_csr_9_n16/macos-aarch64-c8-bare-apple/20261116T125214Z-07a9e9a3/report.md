# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 745% faster than the next best (dep_graph_hybrid)

dep_graph_csr (51 ns) leads dep_graph_hybrid (431 ns) by 745%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 16.8x slower than the field

dep_graph_dense (857 ns) is 16.8x the fastest (51 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (51 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.8x the fastest

Fastest dep_graph_csr (51 ns) to slowest dep_graph_dense (857 ns): 16.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 51.0 ns median
- 2 variants significantly slower than baseline
- Spread: 16.81x (fastest 51.0 ns, slowest 857.1 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dep_graph_csr | 2127ns | 2125ns | 2117ns | 2125ns | 2145ns | base |
| dep_graph_dense | 2956ns | 2941ns | 2932ns | 2942ns | 3024ns | +38.96% |
| dep_graph_hybrid | 2518ns | 2514ns | 2503ns | 2514ns | 2545ns | +18.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 51ns | 49ns | 54ns | base | 0.312 |
| dep_graph_dense | 864ns | 846ns | 903ns | +1587.35% | 0.019 |
| dep_graph_hybrid | 430ns | 422ns | 438ns | +740.40% | 0.037 |

## Performance model

- Peak throughput: **0.330 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.314 | 95.2% |
| dep_graph_dense | 0.019 | 5.7% |
| dep_graph_hybrid | 0.037 | 11.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2127ns | 2127ns | base |
| dep_graph_dense | 2956ns | 2956ns | +38.96% |
| dep_graph_hybrid | 2518ns | 2518ns | +18.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 51ns | base | --- | [51, 51] | --- | --- | --- | --- |
| dep_graph_dense | 857ns | +806.0ns (+1580.4%) | [+805, +808]ns | [855, 859] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 431ns | +380.4ns (+745.9%) | [+379, +381]ns | [430, 432] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 52ns | +1549.5% | +726.9% |
| 1 | 2 | 51ns | +1585.0% | +736.2% |
| 1 | 3 | 51ns | +1558.6% | +743.9% |
| 1 | 4 | 51ns | +1570.3% | +751.7% |
| 1 | 5 | 51ns | +1560.5% | +737.7% |
| 1 | 6 | 50ns | +1599.3% | +757.3% |
| 1 | 7 | 51ns | +1600.4% | +728.3% |
| 1 | 8 | 51ns | +1602.5% | +757.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.095 | ok |
| dep_graph_dense | 0.361 | moderate+ |
| dep_graph_hybrid | -0.277 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 70.7ns | 51.2ns | 138.1% | HIGH |
| dep_graph_dense | 7.7ns | 864.3ns | 0.9% |  |
| dep_graph_hybrid | 445.4ns | 430.5ns | 103.5% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 48.6-54.1 ns)
     48.6 |############
     48.8 |
     49.1 |###########################
     49.4 |################
     49.7 |
     50.0 |################
     50.2 |########################################
     50.5 |
     50.8 |#############################
     51.1 |###############################
     51.3 |
     51.6 |##########
     51.9 |###########################
     52.2 |
     52.5 |#####################
     52.7 |##################
     53.0 |################
     53.3 |
     53.6 |############
     53.9 |
  (12 below, 14 above range)

dep_graph_dense (n=160, range 846.3-903.3 ns)
    846.3 |####################
    849.2 |##########################
    852.0 |###############################
    854.9 |########################################
    857.7 |###############################
    860.6 |#####################
    863.4 |#######################
    866.3 |##########
    869.1 |##########
    872.0 |########
    874.8 |
    877.7 |###
    880.5 |
    883.4 |
    886.2 |#
    889.1 |
    891.9 |
    894.8 |
    897.6 |
    900.5 |
  (15 below, 8 above range)

dep_graph_hybrid (n=160, range 421.9-438.5 ns)
    421.9 |##########
    422.8 |############
    423.6 |#######
    424.4 |##########
    425.2 |###############
    426.1 |###############
    426.9 |############
    427.7 |#######
    428.5 |#######
    429.4 |####################
    430.2 |#####################################
    431.0 |#########################
    431.9 |########################################
    432.7 |################################
    433.5 |###################################
    434.3 |#########################
    435.2 |############
    436.0 |
    436.8 |#####
    437.7 |
  (17 below, 11 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=139.6% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=103.2% of algo (FFI overhead may distort results)
