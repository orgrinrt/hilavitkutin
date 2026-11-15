# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 14.3x slower than the field

dep_graph_dense (2.26 us) is 14.3x the fastest (158 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### dep_graph_csr is fastest but the noisiest (CV 6.9%)

dep_graph_csr wins on median (158 ns) yet has the highest variance (CV 6.9%), while dep_graph_hybrid is the steadiest (CV 4.0%, 165 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (158 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 14.3x the fastest

Fastest dep_graph_csr (158 ns) to slowest dep_graph_dense (2.26 us): 14.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader dep_graph_csr vs stability leader dep_graph_hybrid (+4% speed for 1.7x steadier)

dep_graph_csr is fastest (158 ns, CV 6.9%); dep_graph_hybrid gives up 4.0% median for 1.7x lower variance (CV 4.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 158.3 ns median
- 2 variants significantly slower than baseline
- Spread: 14.25x (fastest 158.3 ns, slowest 2256.4 ns)

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
| dep_graph_csr | 2381ns | 2373ns | 2222ns | 2373ns | 2562ns | base |
| dep_graph_dense | 4511ns | 4482ns | 4202ns | 4505ns | 4836ns | +89.47% |
| dep_graph_hybrid | 2365ns | 2382ns | 2238ns | 2366ns | 2491ns | -0.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 160ns | 149ns | 175ns | base | 0.200 |
| dep_graph_dense | 2268ns | 2115ns | 2425ns | +1315.36% | 0.014 |
| dep_graph_hybrid | 164ns | 155ns | 173ns | +2.06% | 0.196 |

## Performance model

- Peak throughput: **0.215 Gops/s** (dep_graph_csr; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.202 | 93.9% |
| dep_graph_dense | 0.014 | 6.6% |
| dep_graph_hybrid | 0.194 | 90.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2381ns | 2381ns | base |
| dep_graph_dense | 4511ns | 4511ns | +89.47% |
| dep_graph_hybrid | 2365ns | 2365ns | -0.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 158ns | base | --- | [158, 160] | --- | --- | --- | --- |
| dep_graph_dense | 2256ns | +2098.3ns (+1325.6%) | [+2091, +2107]ns | [2250, 2261] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 165ns | +4.4ns (+2.8%) | [+2, +6]ns | [164, 165] | YES | 0.0065 | 0.0065 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 153ns | +1363.5% | +14.8% |
| 1 | 2 | 172ns | +1297.4% | -4.1% |
| 1 | 3 | 171ns | +1196.7% | -3.4% |
| 1 | 4 | 150ns | +1431.3% | +7.1% |
| 1 | 5 | 160ns | +1310.0% | +3.2% |
| 1 | 6 | 158ns | +1362.7% | -2.2% |
| 1 | 7 | 150ns | +1409.3% | +10.1% |
| 1 | 8 | 172ns | +1132.0% | -6.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.246 | moderate- |
| dep_graph_dense | -0.226 | moderate- |
| dep_graph_hybrid | -0.070 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 74.8ns | 160.2ns | 46.7% | HIGH |
| dep_graph_dense | 8.6ns | 2267.5ns | 0.4% |  |
| dep_graph_hybrid | 78.1ns | 163.5ns | 47.8% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 148.6-175.1 ns)
    148.6 |#####################
    149.9 |################################
    151.2 |#####################
    152.5 |##############
    153.9 |###########
    155.2 |#########
    156.5 |##################
    157.8 |################################
    159.2 |#####################################
    160.5 |#######################
    161.8 |####
    163.1 |
    164.5 |##
    165.8 |##
    167.1 |
    168.4 |#######
    169.8 |##############
    171.1 |########################################
    172.4 |#########################
    173.7 |################
  (15 below, 2 above range)

dep_graph_dense (n=160, range 2115.5-2424.6 ns)
   2115.5 |##################
   2130.9 |#########
   2146.4 |####
   2161.9 |###
   2177.3 |
   2192.8 |####
   2208.2 |###
   2223.7 |######
   2239.1 |########################################
   2254.6 |######################################
   2270.1 |###
   2285.5 |######
   2301.0 |######
   2316.4 |###
   2331.9 |
   2347.4 |
   2362.8 |###
   2378.3 |#
   2393.7 |################
   2409.2 |########################################
  (19 below, 6 above range)

dep_graph_hybrid (n=160, range 154.6-172.8 ns)
    154.6 |######
    155.5 |#####
    156.4 |
    157.3 |##
    158.2 |####
    159.1 |########
    160.1 |#######
    161.0 |####
    161.9 |###
    162.8 |####
    163.7 |#########
    164.6 |########################################
    165.5 |###############
    166.4 |#####
    167.3 |##
    168.2 |
    169.1 |
    170.0 |
    170.9 |
    171.8 |
  (15 below, 16 above range)

```

## Diagnostics

- **dep_graph_csr**: bridge=46.7% of algo (FFI overhead may distort results)
- **dep_graph_hybrid**: bridge=47.6% of algo (FFI overhead may distort results)
