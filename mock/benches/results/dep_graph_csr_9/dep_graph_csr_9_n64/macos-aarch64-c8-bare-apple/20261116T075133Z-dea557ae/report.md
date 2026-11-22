# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 19.6x slower than the field

dep_graph_dense (7.02 us) is 19.6x the fastest (359 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (359 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 19.6x the fastest

Fastest dep_graph_csr (359 ns) to slowest dep_graph_dense (7.02 us): 19.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 358.7 ns median
- 2 variants significantly slower than baseline
- Spread: 19.56x (fastest 358.7 ns, slowest 7017.1 ns)

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
| dep_graph_csr | 2448ns | 2452ns | 2415ns | 2446ns | 2490ns | base |
| dep_graph_dense | 9132ns | 9107ns | 9079ns | 9111ns | 9250ns | +272.99% |
| dep_graph_hybrid | 2467ns | 2449ns | 2433ns | 2460ns | 2525ns | +0.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 360ns | 354ns | 368ns | base | 0.178 |
| dep_graph_dense | 7037ns | 6996ns | 7127ns | +1854.99% | 0.009 |
| dep_graph_hybrid | 371ns | 366ns | 380ns | +3.17% | 0.172 |

## Performance model

- Peak throughput: **0.181 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.178 | 98.6% |
| dep_graph_dense | 0.009 | 5.0% |
| dep_graph_hybrid | 0.173 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2448ns | 2448ns | base |
| dep_graph_dense | 9132ns | 9132ns | +272.99% |
| dep_graph_hybrid | 2467ns | 2467ns | +0.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 359ns | base | --- | [358, 361] | --- | --- | --- | --- |
| dep_graph_dense | 7017ns | +6657.3ns (+1856.0%) | [+6653, +6664]ns | [7014, 7020] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 369ns | +11.2ns (+3.1%) | [+10, +12]ns | [369, 370] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 362ns | +1839.4% | +1.5% |
| 1 | 2 | 361ns | +1845.6% | +3.9% |
| 1 | 3 | 356ns | +1879.9% | +3.2% |
| 1 | 4 | 358ns | +1858.6% | +2.7% |
| 1 | 5 | 356ns | +1868.5% | +3.2% |
| 1 | 6 | 356ns | +1867.7% | +4.6% |
| 1 | 7 | 362ns | +1837.3% | +3.6% |
| 1 | 8 | 363ns | +1830.6% | +3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.286 | moderate+ |
| dep_graph_dense | 0.001 | ok |
| dep_graph_hybrid | 0.099 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 65.0ns | 360.0ns | 18.0% | HIGH |
| dep_graph_dense | 8.7ns | 7037.4ns | 0.1% |  |
| dep_graph_hybrid | 70.6ns | 371.4ns | 19.0% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 353.7-368.5 ns)
    353.7 |############################
    354.5 |############################
    355.2 |########################################
    356.0 |####################
    356.7 |#####################################
    357.4 |######################
    358.2 |######################
    358.9 |###########
    359.6 |########
    360.4 |#########################
    361.1 |######################
    361.8 |###############################
    362.6 |#####################################
    363.3 |######################
    364.1 |#########################
    364.8 |###########
    365.5 |#####
    366.3 |##
    367.0 |##
    367.7 |#####
  (11 below, 4 above range)

dep_graph_dense (n=160, range 6996.5-7126.6 ns)
   6996.5 |#########################
   7003.0 |###############################
   7009.5 |########################################
   7016.0 |############################
   7022.5 |#############
   7029.0 |##########
   7035.5 |#######
   7042.0 |#############
   7048.5 |##
   7055.0 |#
   7061.5 |#######
   7068.0 |####
   7074.5 |#
   7081.0 |##
   7087.6 |####
   7094.1 |#
   7100.6 |##
   7107.1 |#
   7113.6 |####
   7120.1 |
  (11 below, 11 above range)

dep_graph_hybrid (n=160, range 365.9-380.2 ns)
    365.9 |#################
    366.7 |########################################
    367.4 |########################################
    368.1 |###################################
    368.8 |###############
    369.5 |#################
    370.2 |########
    370.9 |##
    371.6 |######
    372.4 |###########
    373.1 |########
    373.8 |###############
    374.5 |#####################################
    375.2 |########################
    375.9 |##
    376.6 |##
    377.4 |######
    378.1 |##
    378.8 |##
    379.5 |
  (16 below, 10 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=18.1% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=19.3% of algo (FFI overhead may distort results)
