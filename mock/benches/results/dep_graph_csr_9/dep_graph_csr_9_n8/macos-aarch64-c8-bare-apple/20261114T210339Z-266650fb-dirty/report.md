# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_csr dominates: 304% faster than the next best (dep_graph_hybrid)

dep_graph_csr (27 ns) leads dep_graph_hybrid (110 ns) by 304%, a clear separation rather than a photo finish. CV 8.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### dep_graph_dense is an outlier: 21.4x slower than the field

dep_graph_dense (581 ns) is 21.4x the fastest (27 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 8.7%)

dep_graph_csr wins on median (27 ns) yet has the highest variance (CV 8.7%), while dep_graph_dense is the steadiest (CV 3.8%, 581 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (27 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 21.4x the fastest

Fastest dep_graph_csr (27 ns) to slowest dep_graph_dense (581 ns): 21.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 27.1 ns median
- 2 variants significantly slower than baseline
- Spread: 21.42x (fastest 27.1 ns, slowest 580.6 ns)

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
| dep_graph_csr | 2279ns | 2360ns | 2079ns | 2301ns | 2416ns | base |
| dep_graph_dense | 2883ns | 2984ns | 2640ns | 2920ns | 3013ns | +26.47% |
| dep_graph_hybrid | 2369ns | 2344ns | 2176ns | 2376ns | 2542ns | +3.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 27ns | 24ns | 31ns | base | 0.293 |
| dep_graph_dense | 576ns | 545ns | 604ns | +2014.60% | 0.014 |
| dep_graph_hybrid | 111ns | 100ns | 121ns | +306.27% | 0.072 |

## Performance model

- Peak throughput: **0.332 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.295 | 88.8% |
| dep_graph_dense | 0.014 | 4.1% |
| dep_graph_hybrid | 0.073 | 22.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2279ns | 2279ns | base |
| dep_graph_dense | 2883ns | 2883ns | +26.47% |
| dep_graph_hybrid | 2369ns | 2369ns | +3.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 27ns | base | --- | [27, 28] | --- | --- | --- | --- |
| dep_graph_dense | 581ns | +552.9ns (+2040.2%) | [+549, +558]ns | [575, 585] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 110ns | +82.9ns (+305.9%) | [+82, +85]ns | [108, 111] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 25ns | +2200.0% | +335.7% |
| 1 | 2 | 25ns | +2216.3% | +337.8% |
| 1 | 3 | 28ns | +2006.7% | +272.0% |
| 1 | 4 | 28ns | +2016.8% | +292.7% |
| 1 | 5 | 29ns | +1956.4% | +288.3% |
| 1 | 6 | 28ns | +1952.4% | +273.5% |
| 1 | 7 | 27ns | +2004.5% | +313.5% |
| 1 | 8 | 28ns | +1899.6% | +325.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.398 | moderate+ |
| dep_graph_dense | 0.469 | moderate+ |
| dep_graph_hybrid | -0.118 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 77.9ns | 27.3ns | 285.9% | HIGH |
| dep_graph_dense | 8.9ns | 576.4ns | 1.5% |  |
| dep_graph_hybrid | 474.6ns | 110.7ns | 428.5% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 24.1-30.6 ns)
     24.1 |######
     24.4 |############
     24.7 |####################################
     25.1 |
     25.4 |#################################
     25.7 |##################
     26.0 |####################################
     26.4 |
     26.7 |########################################
     27.0 |####################################
     27.4 |###########################
     27.7 |#################################
     28.0 |#########
     28.3 |
     28.7 |###########################
     29.0 |####################################
     29.3 |##################
     29.7 |
     30.0 |###############
     30.3 |###############
  (14 below, 14 above range)

dep_graph_dense (n=160, range 545.0-603.5 ns)
    545.0 |####################
    548.0 |####################
    550.9 |#########################
    553.8 |##############
    556.7 |##############
    559.7 |###########
    562.6 |###########
    565.5 |########
    568.4 |###########
    571.4 |####################
    574.3 |######################
    577.2 |##############
    580.1 |############################
    583.1 |######################
    586.0 |###############################
    588.9 |########################################
    591.8 |###############################
    594.8 |###############################
    597.7 |###########
    600.6 |########
  (11 below, 9 above range)

dep_graph_hybrid (n=160, range 100.2-121.0 ns)
    100.2 |#############
    101.3 |############
    102.3 |###############
    103.4 |#####
    104.4 |############
    105.4 |######
    106.5 |###################
    107.5 |############
    108.5 |#################
    109.6 |#############
    110.6 |############
    111.7 |###
    112.7 |#####
    113.7 |###
    114.8 |######
    115.8 |######
    116.8 |########################
    117.9 |########################################
    118.9 |############
    120.0 |######
  (12 below, 4 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=283.8% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=432.0% of algo (FFI overhead may distort results)
