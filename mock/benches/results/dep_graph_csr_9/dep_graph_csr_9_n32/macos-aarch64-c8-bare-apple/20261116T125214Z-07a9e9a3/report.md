# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 14.2x slower than the field

dep_graph_dense (2.12 us) is 14.2x the fastest (149 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (149 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 14.2x the fastest

Fastest dep_graph_csr (149 ns) to slowest dep_graph_dense (2.12 us): 14.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 148.7 ns median
- 2 variants significantly slower than baseline
- Spread: 14.22x (fastest 148.7 ns, slowest 2115.0 ns)

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
| dep_graph_csr | 2229ns | 2225ns | 2217ns | 2225ns | 2252ns | base |
| dep_graph_dense | 4212ns | 4205ns | 4194ns | 4206ns | 4247ns | +88.98% |
| dep_graph_hybrid | 2237ns | 2236ns | 2227ns | 2236ns | 2254ns | +0.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 149ns | 146ns | 152ns | base | 0.215 |
| dep_graph_dense | 2119ns | 2104ns | 2142ns | +1325.64% | 0.015 |
| dep_graph_hybrid | 153ns | 151ns | 156ns | +3.18% | 0.209 |

## Performance model

- Peak throughput: **0.220 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.215 | 97.9% |
| dep_graph_dense | 0.015 | 6.9% |
| dep_graph_hybrid | 0.209 | 95.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2229ns | 2229ns | base |
| dep_graph_dense | 4212ns | 4212ns | +88.98% |
| dep_graph_hybrid | 2237ns | 2237ns | +0.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 149ns | base | --- | [148, 149] | --- | --- | --- | --- |
| dep_graph_dense | 2115ns | +1967.5ns (+1323.1%) | [+1966, +1969]ns | [2114, 2118] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 153ns | +5.0ns (+3.4%) | [+5, +5]ns | [153, 154] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 149ns | +1321.6% | +3.3% |
| 1 | 2 | 149ns | +1314.5% | +3.1% |
| 1 | 3 | 148ns | +1326.3% | +4.0% |
| 1 | 4 | 148ns | +1337.7% | +3.8% |
| 1 | 5 | 149ns | +1314.3% | +1.9% |
| 1 | 6 | 149ns | +1323.7% | +3.1% |
| 1 | 7 | 148ns | +1331.4% | +3.7% |
| 1 | 8 | 150ns | +1312.5% | +1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.245 | moderate- |
| dep_graph_dense | -0.043 | ok |
| dep_graph_hybrid | 0.018 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 68.8ns | 148.6ns | 46.3% | HIGH |
| dep_graph_dense | 8.0ns | 2118.6ns | 0.4% |  |
| dep_graph_hybrid | 74.4ns | 153.3ns | 48.5% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 145.6-151.8 ns)
    145.6 |###########
    145.9 |###############
    146.2 |
    146.5 |########################
    146.8 |####################
    147.1 |
    147.4 |###################
    147.8 |###################
    148.1 |#########
    148.4 |
    148.7 |####################
    149.0 |#################
    149.3 |########################################
    149.6 |
    149.9 |###################
    150.2 |#################
    150.5 |#########
    150.8 |
    151.2 |#################
    151.5 |#
  (12 below, 10 above range)

dep_graph_dense (n=160, range 2103.9-2142.0 ns)
   2103.9 |####################
   2105.8 |########################
   2107.7 |###############
   2109.6 |#################
   2111.5 |###############
   2113.4 |########################################
   2115.3 |###################
   2117.2 |###############
   2119.1 |##############################
   2121.0 |###################
   2122.9 |#############
   2124.8 |###########
   2126.8 |###########
   2128.7 |#####
   2130.6 |#
   2132.5 |#####
   2134.4 |###
   2136.3 |
   2138.2 |
   2140.1 |
  (11 below, 7 above range)

dep_graph_hybrid (n=160, range 150.8-155.9 ns)
    150.8 |###########
    151.0 |###########
    151.3 |
    151.5 |########################
    151.8 |
    152.0 |#####################################
    152.3 |############################
    152.6 |
    152.8 |######################
    153.1 |########################
    153.3 |
    153.6 |########################
    153.8 |
    154.1 |############################
    154.3 |
    154.6 |########################################
    154.8 |########################
    155.1 |
    155.4 |######
    155.6 |###############
  (13 below, 12 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=46.7% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=49.2% of algo (FFI overhead may distort results)
