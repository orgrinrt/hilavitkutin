# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 19.7x slower than the field

dep_graph_dense (7.02 us) is 19.7x the fastest (356 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (356 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 19.7x the fastest

Fastest dep_graph_csr (356 ns) to slowest dep_graph_dense (7.02 us): 19.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 356.2 ns median
- 2 variants significantly slower than baseline
- Spread: 19.69x (fastest 356.2 ns, slowest 7015.2 ns)

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
| dep_graph_csr | 2440ns | 2433ns | 2424ns | 2434ns | 2475ns | base |
| dep_graph_dense | 9181ns | 9108ns | 9089ns | 9119ns | 9458ns | +276.25% |
| dep_graph_hybrid | 2467ns | 2453ns | 2443ns | 2461ns | 2510ns | +1.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 357ns | 353ns | 362ns | base | 0.179 |
| dep_graph_dense | 7070ns | 6995ns | 7296ns | +1881.90% | 0.009 |
| dep_graph_hybrid | 371ns | 366ns | 380ns | +4.03% | 0.172 |

## Performance model

- Peak throughput: **0.181 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.180 | 99.1% |
| dep_graph_dense | 0.009 | 5.0% |
| dep_graph_hybrid | 0.174 | 95.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2440ns | 2440ns | base |
| dep_graph_dense | 9181ns | 9181ns | +276.25% |
| dep_graph_hybrid | 2467ns | 2467ns | +1.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 356ns | base | --- | [356, 357] | --- | --- | --- | --- |
| dep_graph_dense | 7015ns | +6657.3ns (+1869.0%) | [+6656, +6660]ns | [7013, 7018] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 369ns | +13.3ns (+3.7%) | [+12, +14]ns | [369, 369] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 356ns | +1864.1% | +3.2% |
| 1 | 2 | 361ns | +1843.6% | +2.3% |
| 1 | 3 | 358ns | +1857.8% | +2.6% |
| 1 | 4 | 356ns | +1867.6% | +5.3% |
| 1 | 5 | 356ns | +1867.4% | +3.6% |
| 1 | 6 | 355ns | +1961.9% | +5.6% |
| 1 | 7 | 356ns | +1874.2% | +3.8% |
| 1 | 8 | 355ns | +1876.6% | +3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.332 | moderate+ |
| dep_graph_dense | -0.159 | ok |
| dep_graph_hybrid | -0.333 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 63.8ns | 356.7ns | 17.9% | HIGH |
| dep_graph_dense | 7.7ns | 7070.2ns | 0.1% |  |
| dep_graph_hybrid | 69.6ns | 371.1ns | 18.7% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 353.0-361.7 ns)
    353.0 |####################
    353.5 |########################################
    353.9 |#################
    354.3 |##############
    354.8 |######################
    355.2 |#########################
    355.6 |############################
    356.1 |############################
    356.5 |###############################
    356.9 |####################
    357.4 |###########
    357.8 |######################
    358.3 |##################################
    358.7 |###########
    359.1 |##
    359.6 |########
    360.0 |########
    360.4 |##
    360.9 |#####
    361.3 |#####
  (15 below, 18 above range)

dep_graph_dense (n=160, range 6994.9-7295.7 ns)
   6994.9 |#####################################
   7010.0 |########################################
   7025.0 |##########
   7040.0 |####
   7055.1 |###
   7070.1 |####
   7085.1 |
   7100.2 |
   7115.2 |#
   7130.3 |##
   7145.3 |#
   7160.3 |
   7175.4 |
   7190.4 |#
   7205.4 |
   7220.5 |
   7235.5 |
   7250.6 |
   7265.6 |
   7280.6 |
  (12 below, 17 above range)

dep_graph_hybrid (n=160, range 365.9-380.4 ns)
    365.9 |#######
    366.6 |#######################
    367.3 |############################
    368.1 |########################################
    368.8 |#################
    369.5 |#############
    370.2 |#####
    371.0 |#
    371.7 |#####
    372.4 |
    373.1 |#############
    373.9 |####
    374.6 |####################
    375.3 |#############
    376.0 |###########
    376.8 |#
    377.5 |
    378.2 |#
    378.9 |#
    379.7 |
  (14 below, 3 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=18.2% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=18.9% of algo (FFI overhead may distort results)
