# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 19.7x slower than the field

dep_graph_dense (7.01 us) is 19.7x the fastest (356 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr shows alternating (throttle bounce) (autocorr -0.69)

dep_graph_csr's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (356 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 19.7x the fastest

Fastest dep_graph_csr (356 ns) to slowest dep_graph_dense (7.01 us): 19.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 355.8 ns median
- 2 variants significantly slower than baseline
- Spread: 19.70x (fastest 355.8 ns, slowest 7009.2 ns)

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
| dep_graph_csr | 2434ns | 2431ns | 2423ns | 2431ns | 2454ns | base |
| dep_graph_dense | 9168ns | 9101ns | 9088ns | 9104ns | 9441ns | +276.66% |
| dep_graph_hybrid | 2453ns | 2448ns | 2441ns | 2448ns | 2479ns | +0.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 356ns | 353ns | 359ns | base | 0.180 |
| dep_graph_dense | 7063ns | 6997ns | 7282ns | +1885.16% | 0.009 |
| dep_graph_hybrid | 368ns | 366ns | 370ns | +3.48% | 0.174 |

## Performance model

- Peak throughput: **0.181 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.180 | 99.2% |
| dep_graph_dense | 0.009 | 5.0% |
| dep_graph_hybrid | 0.174 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2434ns | 2434ns | base |
| dep_graph_dense | 9168ns | 9168ns | +276.66% |
| dep_graph_hybrid | 2453ns | 2453ns | +0.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 356ns | base | --- | [355, 356] | --- | --- | --- | --- |
| dep_graph_dense | 7009ns | +6653.4ns (+1870.0%) | [+6651, +6657]ns | [7007, 7012] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 368ns | +12.5ns (+3.5%) | [+12, +13]ns | [368, 369] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 356ns | +1868.8% | +3.1% |
| 1 | 2 | 356ns | +1870.7% | +3.7% |
| 1 | 3 | 355ns | +1873.6% | +3.7% |
| 1 | 4 | 356ns | +1868.3% | +3.6% |
| 1 | 5 | 355ns | +1872.8% | +3.8% |
| 1 | 6 | 356ns | +1867.3% | +3.6% |
| 1 | 7 | 356ns | +1869.6% | +3.5% |
| 1 | 8 | 356ns | +1970.2% | +3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.687 | HIGH- (thermal bounce) |
| dep_graph_dense | -0.030 | ok |
| dep_graph_hybrid | -0.078 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 63.5ns | 355.8ns | 17.8% | HIGH |
| dep_graph_dense | 7.6ns | 7062.9ns | 0.1% |  |
| dep_graph_hybrid | 69.2ns | 368.2ns | 18.8% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 353.0-358.8 ns)
    353.0 |####################
    353.3 |
    353.6 |######################
    353.9 |
    354.2 |####################
    354.5 |#####################################
    354.8 |####################
    355.0 |
    355.3 |################################
    355.6 |########################################
    355.9 |###################################
    356.2 |
    356.5 |################################
    356.8 |
    357.1 |###########################
    357.4 |####################
    357.7 |###############
    358.0 |
    358.2 |##########
    358.5 |##########
  (14 below, 9 above range)

dep_graph_dense (n=160, range 6996.9-7281.8 ns)
   6996.9 |########################################
   7011.1 |###################
   7025.4 |#
   7039.6 |#####
   7053.9 |##
   7068.1 |#
   7082.4 |
   7096.6 |
   7110.9 |
   7125.1 |
   7139.3 |
   7153.6 |
   7167.8 |
   7182.1 |
   7196.3 |
   7210.6 |
   7224.8 |
   7239.1 |
   7253.3 |
   7267.5 |#
  (16 below, 18 above range)

dep_graph_hybrid (n=160, range 365.7-370.3 ns)
    365.7 |########
    366.0 |
    366.2 |########
    366.4 |
    366.6 |#############
    366.9 |
    367.1 |###################
    367.3 |########################
    367.5 |
    367.8 |###################
    368.0 |
    368.2 |########################
    368.5 |
    368.7 |########################################
    368.9 |
    369.1 |######################################
    369.4 |###############
    369.6 |
    369.8 |###################
    370.1 |
  (15 below, 12 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=17.9% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=18.8% of algo (FFI overhead may distort results)
