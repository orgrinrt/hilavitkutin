# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 744% faster than the next best (dep_graph_hybrid)

dep_graph_csr (51 ns) leads dep_graph_hybrid (432 ns) by 744%, a clear separation rather than a photo finish. CV 3.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 16.7x slower than the field

dep_graph_dense (857 ns) is 16.7x the fastest (51 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (51 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.7x the fastest

Fastest dep_graph_csr (51 ns) to slowest dep_graph_dense (857 ns): 16.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 51.2 ns median
- 2 variants significantly slower than baseline
- Spread: 16.74x (fastest 51.2 ns, slowest 856.9 ns)

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
| dep_graph_csr | 2144ns | 2130ns | 2121ns | 2136ns | 2191ns | base |
| dep_graph_dense | 2964ns | 2946ns | 2929ns | 2956ns | 3022ns | +38.27% |
| dep_graph_hybrid | 2524ns | 2516ns | 2508ns | 2517ns | 2564ns | +17.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 52ns | 49ns | 54ns | base | 0.310 |
| dep_graph_dense | 861ns | 842ns | 891ns | +1568.74% | 0.019 |
| dep_graph_hybrid | 433ns | 428ns | 439ns | +738.92% | 0.037 |

## Performance model

- Peak throughput: **0.325 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.312 | 96.3% |
| dep_graph_dense | 0.019 | 5.8% |
| dep_graph_hybrid | 0.037 | 11.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2144ns | 2144ns | base |
| dep_graph_dense | 2964ns | 2964ns | +38.27% |
| dep_graph_hybrid | 2524ns | 2524ns | +17.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 51ns | base | --- | [51, 52] | --- | --- | --- | --- |
| dep_graph_dense | 857ns | +805.4ns (+1573.0%) | [+803, +808]ns | [855, 859] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 432ns | +380.4ns (+743.1%) | [+380, +381]ns | [432, 433] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 51ns | +1619.8% | +754.3% |
| 1 | 2 | 51ns | +1584.3% | +753.9% |
| 1 | 3 | 52ns | +1561.5% | +728.6% |
| 1 | 4 | 51ns | +1575.6% | +748.8% |
| 1 | 5 | 52ns | +1535.4% | +724.9% |
| 1 | 6 | 52ns | +1531.9% | +728.6% |
| 1 | 7 | 51ns | +1576.0% | +748.3% |
| 1 | 8 | 51ns | +1567.9% | +740.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.069 | ok |
| dep_graph_dense | -0.049 | ok |
| dep_graph_hybrid | -0.409 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 70.7ns | 51.6ns | 137.1% | HIGH |
| dep_graph_dense | 7.9ns | 860.7ns | 0.9% |  |
| dep_graph_hybrid | 452.7ns | 432.7ns | 104.6% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 49.3-54.4 ns)
     49.3 |
     49.5 |############
     49.8 |########################################
     50.1 |
     50.3 |###########################
     50.6 |#############################
     50.8 |
     51.1 |###################################
     51.3 |
     51.6 |#########################
     51.8 |###########################
     52.1 |
     52.4 |###########################
     52.6 |
     52.9 |################
     53.1 |##########
     53.4 |
     53.6 |##############
     53.9 |
     54.1 |############
  (15 below, 12 above range)

dep_graph_dense (n=160, range 841.9-890.5 ns)
    841.9 |################
    844.3 |################################
    846.8 |########################
    849.2 |#############################
    851.6 |##########################
    854.1 |########################################
    856.5 |##################################
    858.9 |##########################
    861.4 |################################
    863.8 |#####################
    866.2 |#############
    868.7 |#############
    871.1 |##########################
    873.5 |#####
    875.9 |##########
    878.4 |#####
    880.8 |##
    883.2 |#####
    885.7 |##
    888.1 |##
  (16 below, 5 above range)

dep_graph_hybrid (n=160, range 427.5-439.5 ns)
    427.5 |#########
    428.1 |##################
    428.7 |#######
    429.3 |#######
    429.9 |#####################################
    430.5 |#####################
    431.1 |########################################
    431.7 |#####################
    432.3 |################################
    432.9 |################
    433.5 |#######################
    434.1 |#########################
    434.7 |#######
    435.3 |################
    435.9 |
    436.5 |#######
    437.1 |##
    437.7 |#########
    438.3 |####
    438.9 |##
  (16 below, 12 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=140.0% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=104.4% of algo (FFI overhead may distort results)
