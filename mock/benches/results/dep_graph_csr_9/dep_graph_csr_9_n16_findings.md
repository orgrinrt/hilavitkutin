# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 739% faster than the next best (dep_graph_hybrid)

dep_graph_csr (54 ns) leads dep_graph_hybrid (452 ns) by 739%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 16.8x slower than the field

dep_graph_dense (904 ns) is 16.8x the fastest (54 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 6.7%)

dep_graph_csr wins on median (54 ns) yet has the highest variance (CV 6.7%), while dep_graph_dense is the steadiest (CV 4.2%, 904 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (54 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 16.8x the fastest

Fastest dep_graph_csr (54 ns) to slowest dep_graph_dense (904 ns): 16.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 53.8 ns median
- 2 variants significantly slower than baseline
- Spread: 16.81x (fastest 53.8 ns, slowest 904.2 ns)

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
| dep_graph_csr | 2245ns | 2194ns | 2113ns | 2222ns | 2448ns | base |
| dep_graph_dense | 3162ns | 3128ns | 2941ns | 3169ns | 3361ns | +40.81% |
| dep_graph_hybrid | 2680ns | 2679ns | 2527ns | 2659ns | 2899ns | +19.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 55ns | 50ns | 60ns | base | 0.293 |
| dep_graph_dense | 910ns | 856ns | 962ns | +1566.00% | 0.018 |
| dep_graph_hybrid | 460ns | 430ns | 503ns | +741.74% | 0.035 |

## Performance model

- Peak throughput: **0.318 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.297 | 93.6% |
| dep_graph_dense | 0.018 | 5.6% |
| dep_graph_hybrid | 0.035 | 11.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2245ns | 2245ns | base |
| dep_graph_dense | 3162ns | 3162ns | +40.81% |
| dep_graph_hybrid | 2680ns | 2680ns | +19.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 54ns | base | --- | [53, 55] | --- | --- | --- | --- |
| dep_graph_dense | 904ns | +852.3ns (+1584.3%) | [+843, +859]ns | [898, 911] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 452ns | +399.6ns (+742.8%) | [+395, +409]ns | [448, 462] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 52ns | +1665.4% | +857.8% |
| 1 | 2 | 56ns | +1589.4% | +669.7% |
| 1 | 3 | 59ns | +1419.1% | +649.9% |
| 1 | 4 | 58ns | +1397.0% | +655.4% |
| 1 | 5 | 54ns | +1487.3% | +762.0% |
| 1 | 6 | 52ns | +1656.6% | +777.6% |
| 1 | 7 | 53ns | +1697.9% | +773.8% |
| 1 | 8 | 53ns | +1597.6% | +738.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.442 | moderate+ |
| dep_graph_dense | 0.303 | moderate+ |
| dep_graph_hybrid | -0.233 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 75.4ns | 54.6ns | 137.9% | HIGH |
| dep_graph_dense | 8.9ns | 910.4ns | 1.0% |  |
| dep_graph_hybrid | 468.5ns | 460.0ns | 101.9% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 50.4-60.4 ns)
     50.4 |############################
     50.9 |#######################
     51.4 |##############
     51.9 |#######################
     52.4 |#########################
     52.9 |########################################
     53.4 |################
     53.9 |##############
     54.4 |#########################
     54.9 |##############
     55.4 |###########
     55.9 |#########
     56.4 |##
     56.9 |###########
     57.4 |#########
     57.9 |##############
     58.4 |#########
     58.9 |##
     59.4 |###########
     59.9 |#######
  (9 below, 17 above range)

dep_graph_dense (n=160, range 855.8-962.1 ns)
    855.8 |############
    861.1 |#####
    866.5 |#####
    871.8 |##
    877.1 |#######
    882.4 |#################
    887.7 |########################################
    893.0 |###################################
    898.3 |#################
    903.6 |#########################
    909.0 |##############################
    914.3 |#######
    919.6 |
    924.9 |##
    930.2 |##
    935.5 |#################
    940.8 |###############
    946.1 |############
    951.5 |################################
    956.8 |##############################
  (20 below, 13 above range)

dep_graph_hybrid (n=160, range 430.0-503.4 ns)
    430.0 |#############
    433.7 |##########
    437.4 |####
    441.0 |###########
    444.7 |########################################
    448.4 |#################
    452.1 |##
    455.7 |####
    459.4 |####################
    463.1 |##################################
    466.7 |####
    470.4 |
    474.1 |#
    477.7 |
    481.4 |##
    485.1 |#
    488.8 |#
    492.4 |#
    496.1 |###########
    499.8 |##########################
  (14 below, 3 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=139.8% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=102.8% of algo (FFI overhead may distort results)
