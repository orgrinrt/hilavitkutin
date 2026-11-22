# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 311% faster than the next best (dep_graph_hybrid)

dep_graph_csr (25 ns) leads dep_graph_hybrid (101 ns) by 311%, a clear separation rather than a photo finish. CV 8.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 22.2x slower than the field

dep_graph_dense (546 ns) is 22.2x the fastest (25 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 8.8%)

dep_graph_csr wins on median (25 ns) yet has the highest variance (CV 8.8%), while dep_graph_dense is the steadiest (CV 2.0%, 546 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (25 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 22.2x the fastest

Fastest dep_graph_csr (25 ns) to slowest dep_graph_dense (546 ns): 22.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 24.6 ns median
- 2 variants significantly slower than baseline
- Spread: 22.18x (fastest 24.6 ns, slowest 545.6 ns)

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
| dep_graph_csr | 2100ns | 2091ns | 2083ns | 2091ns | 2144ns | base |
| dep_graph_dense | 2636ns | 2632ns | 2622ns | 2632ns | 2663ns | +25.55% |
| dep_graph_hybrid | 2198ns | 2189ns | 2181ns | 2189ns | 2244ns | +4.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 24ns | 21ns | 27ns | base | 0.329 |
| dep_graph_dense | 546ns | 534ns | 563ns | +2145.83% | 0.015 |
| dep_graph_hybrid | 101ns | 98ns | 106ns | +317.03% | 0.079 |

## Performance model

- Peak throughput: **0.376 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.325 | 86.4% |
| dep_graph_dense | 0.015 | 3.9% |
| dep_graph_hybrid | 0.079 | 21.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2100ns | 2100ns | base |
| dep_graph_dense | 2636ns | 2636ns | +25.55% |
| dep_graph_hybrid | 2198ns | 2198ns | +4.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 25ns | base | --- | [24, 25] | --- | --- | --- | --- |
| dep_graph_dense | 546ns | +520.9ns (+2117.3%) | [+519, +523]ns | [544, 547] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 101ns | +77.1ns (+313.4%) | [+76, +78]ns | [100, 102] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 25ns | +2042.9% | +298.4% |
| 1 | 2 | 24ns | +2116.4% | +306.6% |
| 1 | 3 | 23ns | +2240.1% | +337.3% |
| 1 | 4 | 24ns | +2116.6% | +313.9% |
| 1 | 5 | 25ns | +2060.4% | +306.9% |
| 1 | 6 | 24ns | +2220.3% | +316.6% |
| 1 | 7 | 24ns | +2137.7% | +313.1% |
| 1 | 8 | 24ns | +2177.2% | +324.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.015 | ok |
| dep_graph_dense | 0.156 | ok |
| dep_graph_hybrid | -0.014 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 71.6ns | 24.3ns | 294.1% | HIGH |
| dep_graph_dense | 7.6ns | 546.5ns | 1.4% |  |
| dep_graph_hybrid | 449.9ns | 101.5ns | 443.3% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 21.3-27.3 ns)
     21.3 |
     21.6 |#######################
     21.9 |###########
     22.2 |
     22.5 |#########################
     22.8 |#########################
     23.1 |#######
     23.4 |
     23.7 |##############################
     24.0 |#######################
     24.3 |
     24.6 |#####################
     24.9 |########################################
     25.2 |################
     25.5 |
     25.8 |############################
     26.1 |################
     26.4 |
     26.7 |#########################
     27.0 |###########
  (15 below, 14 above range)

dep_graph_dense (n=160, range 533.6-562.8 ns)
    533.6 |###########
    535.1 |#####################
    536.5 |##############################
    538.0 |###########
    539.5 |#######################
    540.9 |##############
    542.4 |#####################
    543.8 |##############
    545.3 |########################################
    546.8 |###########
    548.2 |#######################
    549.7 |#####################
    551.1 |##############################
    552.6 |###########
    554.0 |###########
    555.5 |##
    557.0 |#########
    558.4 |##
    559.9 |##
    561.3 |##
  (12 below, 13 above range)

dep_graph_hybrid (n=160, range 97.7-105.9 ns)
     97.7 |#########
     98.1 |############
     98.5 |###############
     98.9 |##############################
     99.4 |##############################
     99.8 |########################################
    100.2 |###########################
    100.6 |###############
    101.0 |#################################
    101.4 |##################
    101.8 |#################################
    102.2 |##############################
    102.6 |####################################
    103.1 |##################
    103.5 |###########################
    103.9 |###############
    104.3 |############
    104.7 |#########
    105.1 |#########
    105.5 |############
  (15 below, 2 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=291.5% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=442.6% of algo (FFI overhead may distort results)
