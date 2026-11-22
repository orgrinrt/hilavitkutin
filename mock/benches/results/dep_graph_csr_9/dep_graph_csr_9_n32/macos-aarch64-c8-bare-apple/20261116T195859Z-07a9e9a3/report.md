# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 14.2x slower than the field

dep_graph_dense (2.12 us) is 14.2x the fastest (150 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (150 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 14.2x the fastest

Fastest dep_graph_csr (150 ns) to slowest dep_graph_dense (2.12 us): 14.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 149.6 ns median
- 2 variants significantly slower than baseline
- Spread: 14.19x (fastest 149.6 ns, slowest 2123.3 ns)

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
| dep_graph_csr | 2249ns | 2229ns | 2220ns | 2236ns | 2317ns | base |
| dep_graph_dense | 4231ns | 4216ns | 4196ns | 4220ns | 4297ns | +88.10% |
| dep_graph_hybrid | 2243ns | 2237ns | 2228ns | 2237ns | 2277ns | -0.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 149ns | 146ns | 153ns | base | 0.214 |
| dep_graph_dense | 2129ns | 2106ns | 2166ns | +1324.70% | 0.015 |
| dep_graph_hybrid | 154ns | 151ns | 157ns | +2.92% | 0.208 |

## Performance model

- Peak throughput: **0.220 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.214 | 97.4% |
| dep_graph_dense | 0.015 | 6.9% |
| dep_graph_hybrid | 0.208 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2249ns | 2249ns | base |
| dep_graph_dense | 4231ns | 4231ns | +88.10% |
| dep_graph_hybrid | 2243ns | 2243ns | -0.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 150ns | base | --- | [149, 150] | --- | --- | --- | --- |
| dep_graph_dense | 2123ns | +1974.1ns (+1319.6%) | [+1971, +1978]ns | [2120, 2127] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 154ns | +4.1ns (+2.7%) | [+4, +5]ns | [153, 154] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 149ns | +1316.0% | +3.8% |
| 1 | 2 | 148ns | +1331.6% | +3.7% |
| 1 | 3 | 150ns | +1320.1% | +3.1% |
| 1 | 4 | 149ns | +1325.9% | +2.5% |
| 1 | 5 | 151ns | +1319.9% | +1.7% |
| 1 | 6 | 150ns | +1310.5% | +2.1% |
| 1 | 7 | 148ns | +1343.1% | +4.7% |
| 1 | 8 | 151ns | +1305.6% | +2.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.310 | moderate- |
| dep_graph_dense | -0.181 | ok |
| dep_graph_hybrid | -0.039 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 68.7ns | 149.4ns | 46.0% | HIGH |
| dep_graph_dense | 8.5ns | 2129.0ns | 0.4% |  |
| dep_graph_hybrid | 74.4ns | 153.8ns | 48.4% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 145.7-153.1 ns)
    145.7 |##########
    146.1 |#########################
    146.5 |#################
    146.8 |##########
    147.2 |####################
    147.6 |#####
    147.9 |##############################
    148.3 |
    148.7 |###############
    149.1 |########################################
    149.4 |################################
    149.8 |######################
    150.2 |#################
    150.5 |#################
    150.9 |#########################
    151.3 |
    151.6 |#######
    152.0 |###########################
    152.4 |###############
    152.7 |#####
  (10 below, 13 above range)

dep_graph_dense (n=160, range 2106.1-2166.0 ns)
   2106.1 |###########
   2109.1 |######################
   2112.1 |########################################
   2115.1 |##########################
   2118.1 |##########################
   2121.1 |##########################
   2124.1 |######################
   2127.1 |##########################
   2130.1 |###############
   2133.0 |#################
   2136.0 |####
   2139.0 |######
   2142.0 |###########
   2145.0 |##
   2148.0 |####
   2151.0 |######
   2154.0 |######
   2157.0 |###########
   2160.0 |####
   2163.0 |######
  (14 below, 11 above range)

dep_graph_hybrid (n=160, range 150.9-156.6 ns)
    150.9 |##################
    151.2 |
    151.5 |###########
    151.8 |
    152.1 |##################
    152.4 |################################
    152.6 |#########################
    152.9 |
    153.2 |##############################
    153.5 |
    153.8 |################
    154.1 |#########################
    154.3 |########################################
    154.6 |
    154.9 |#######################
    155.2 |############################
    155.5 |
    155.8 |##############
    156.1 |#######################
    156.3 |
  (16 below, 12 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=46.4% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=48.8% of algo (FFI overhead may distort results)
