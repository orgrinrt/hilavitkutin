# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 733% faster than the next best (dep_graph_hybrid)

dep_graph_csr (52 ns) leads dep_graph_hybrid (434 ns) by 733%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 16.6x slower than the field

dep_graph_dense (866 ns) is 16.6x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (52 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.6x the fastest

Fastest dep_graph_csr (52 ns) to slowest dep_graph_dense (866 ns): 16.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 52.1 ns median
- 2 variants significantly slower than baseline
- Spread: 16.61x (fastest 52.1 ns, slowest 865.6 ns)

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
| dep_graph_csr | 2146ns | 2149ns | 2112ns | 2143ns | 2188ns | base |
| dep_graph_dense | 2978ns | 2954ns | 2916ns | 2958ns | 3102ns | +38.80% |
| dep_graph_hybrid | 2532ns | 2539ns | 2498ns | 2532ns | 2568ns | +18.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 52ns | 50ns | 55ns | base | 0.306 |
| dep_graph_dense | 874ns | 843ns | 923ns | +1572.91% | 0.018 |
| dep_graph_hybrid | 435ns | 426ns | 444ns | +732.07% | 0.037 |

## Performance model

- Peak throughput: **0.322 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.307 | 95.4% |
| dep_graph_dense | 0.018 | 5.7% |
| dep_graph_hybrid | 0.037 | 11.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2146ns | 2146ns | base |
| dep_graph_dense | 2978ns | 2978ns | +38.80% |
| dep_graph_hybrid | 2532ns | 2532ns | +18.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 52ns | base | --- | [52, 52] | --- | --- | --- | --- |
| dep_graph_dense | 866ns | +812.7ns (+1559.9%) | [+809, +818]ns | [862, 870] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 434ns | +382.5ns (+734.2%) | [+381, +384]ns | [432, 435] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 53ns | +1550.5% | +719.2% |
| 1 | 2 | 53ns | +1504.6% | +704.9% |
| 1 | 3 | 51ns | +1675.8% | +762.5% |
| 1 | 4 | 51ns | +1688.8% | +752.2% |
| 1 | 5 | 53ns | +1538.3% | +717.2% |
| 1 | 6 | 53ns | +1544.8% | +725.9% |
| 1 | 7 | 52ns | +1566.3% | +744.5% |
| 1 | 8 | 52ns | +1515.7% | +723.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.146 | ok |
| dep_graph_dense | 0.086 | ok |
| dep_graph_hybrid | 0.007 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 72.4ns | 52.2ns | 138.7% | HIGH |
| dep_graph_dense | 9.5ns | 873.7ns | 1.1% |  |
| dep_graph_hybrid | 449.7ns | 434.5ns | 103.5% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 49.7-55.2 ns)
     49.7 |
     50.0 |######################
     50.2 |###########################
     50.5 |
     50.8 |######################
     51.1 |########################################
     51.3 |
     51.6 |###########################
     51.9 |###################################
     52.2 |
     52.4 |#####################################
     52.7 |#########################
     53.0 |
     53.3 |######################
     53.5 |###############
     53.8 |
     54.1 |###########################
     54.4 |##########
     54.6 |
     54.9 |##########
  (17 below, 14 above range)

dep_graph_dense (n=160, range 842.9-922.7 ns)
    842.9 |#########################
    846.9 |#########################
    850.8 |#########################
    854.8 |#######################
    858.8 |########################################
    862.8 |#######################
    866.8 |############################
    870.8 |#####################
    874.8 |####
    878.8 |#######################
    882.8 |##############
    886.8 |####
    890.8 |#######
    894.7 |####
    898.7 |####
    902.7 |#########
    906.7 |#########
    910.7 |#######
    914.7 |##############
    918.7 |#######
  (13 below, 9 above range)

dep_graph_hybrid (n=160, range 426.0-443.6 ns)
    426.0 |#####
    426.8 |###############
    427.7 |#######
    428.6 |######################
    429.5 |####################
    430.4 |###########################
    431.2 |##############################
    432.1 |##############################
    433.0 |##########
    433.9 |###########################
    434.8 |##
    435.7 |#######
    436.5 |#####
    437.4 |###############
    438.3 |###################################
    439.2 |########################################
    440.1 |####################
    440.9 |#######
    441.8 |#################
    442.7 |#######
  (14 below, 5 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=139.2% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=103.6% of algo (FFI overhead may distort results)
