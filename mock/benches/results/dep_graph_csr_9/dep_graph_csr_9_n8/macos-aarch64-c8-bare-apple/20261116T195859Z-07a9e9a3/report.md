# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 309% faster than the next best (dep_graph_hybrid)

dep_graph_csr (25 ns) leads dep_graph_hybrid (101 ns) by 309%, a clear separation rather than a photo finish. CV 8.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 22.6x slower than the field

dep_graph_dense (556 ns) is 22.6x the fastest (25 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 8.0%)

dep_graph_csr wins on median (25 ns) yet has the highest variance (CV 8.0%), while dep_graph_dense is the steadiest (CV 3.4%, 556 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (25 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 22.6x the fastest

Fastest dep_graph_csr (25 ns) to slowest dep_graph_dense (556 ns): 22.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 24.6 ns median
- 2 variants significantly slower than baseline
- Spread: 22.59x (fastest 24.6 ns, slowest 555.8 ns)

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
| dep_graph_csr | 2131ns | 2126ns | 2087ns | 2120ns | 2211ns | base |
| dep_graph_dense | 2676ns | 2674ns | 2624ns | 2670ns | 2744ns | +25.53% |
| dep_graph_hybrid | 2192ns | 2189ns | 2178ns | 2190ns | 2215ns | +2.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 25ns | 22ns | 27ns | base | 0.326 |
| dep_graph_dense | 557ns | 534ns | 586ns | +2170.27% | 0.014 |
| dep_graph_hybrid | 100ns | 96ns | 105ns | +309.59% | 0.080 |

## Performance model

- Peak throughput: **0.368 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.325 | 88.4% |
| dep_graph_dense | 0.014 | 3.9% |
| dep_graph_hybrid | 0.080 | 21.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2131ns | 2131ns | base |
| dep_graph_dense | 2676ns | 2676ns | +25.53% |
| dep_graph_hybrid | 2192ns | 2192ns | +2.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 25ns | base | --- | [24, 25] | --- | --- | --- | --- |
| dep_graph_dense | 556ns | +531.1ns (+2158.9%) | [+527, +535]ns | [551, 558] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 101ns | +75.8ns (+308.1%) | [+75, +77]ns | [100, 102] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 25ns | +2068.9% | +294.7% |
| 1 | 2 | 25ns | +2154.3% | +296.3% |
| 1 | 3 | 23ns | +2305.4% | +327.5% |
| 1 | 4 | 25ns | +2180.7% | +320.1% |
| 1 | 5 | 25ns | +2164.3% | +288.5% |
| 1 | 6 | 25ns | +2114.3% | +314.9% |
| 1 | 7 | 24ns | +2208.6% | +334.0% |
| 1 | 8 | 26ns | +2059.0% | +289.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.194 | ok |
| dep_graph_dense | 0.089 | ok |
| dep_graph_hybrid | 0.044 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 73.0ns | 24.5ns | 297.7% | HIGH |
| dep_graph_dense | 7.9ns | 557.0ns | 1.4% |  |
| dep_graph_hybrid | 455.0ns | 100.5ns | 452.8% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 21.8-27.2 ns)
     21.8 |
     22.0 |################
     22.3 |########################
     22.6 |
     22.8 |#####################
     23.1 |##########################
     23.4 |
     23.6 |##################################
     23.9 |
     24.2 |#####################################
     24.5 |#####################################
     24.7 |
     25.0 |##################################
     25.3 |##########################
     25.5 |########################################
     25.8 |
     26.1 |##################################
     26.4 |
     26.6 |#############
     26.9 |################
  (13 below, 11 above range)

dep_graph_dense (n=160, range 534.0-585.7 ns)
    534.0 |##########
    536.6 |########################
    539.2 |#############
    541.8 |##########################
    544.4 |#####################
    547.0 |#####################
    549.6 |########################################
    552.1 |##########
    554.7 |########################
    557.3 |########################################
    559.9 |########################
    562.5 |##################
    565.1 |#####################
    567.6 |##########
    570.2 |################
    572.8 |#############
    575.4 |########
    578.0 |#####
    580.6 |#####
    583.1 |#####
  (13 below, 12 above range)

dep_graph_hybrid (n=160, range 95.6-105.4 ns)
     95.6 |##########
     96.1 |########
     96.6 |########################################
     97.1 |################
     97.6 |#####################
     98.1 |################
     98.6 |##########
     99.1 |#####################
     99.6 |#####################
    100.1 |################
    100.5 |##################
    101.0 |########
    101.5 |##########################
    102.0 |########################################
    102.5 |########################
    103.0 |#####
    103.5 |#############
    104.0 |##########
    104.5 |################
    105.0 |##########################
  (12 below, 9 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=296.3% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=450.6% of algo (FFI overhead may distort results)
