# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 35.1x slower than the field

dep_graph_dense (26.46 us) is 35.1x the fastest (754 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 35.1x the fastest

Fastest dep_graph_hybrid (754 ns) to slowest dep_graph_dense (26.46 us): 35.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dep_graph_hybrid** at 754.2 ns median (-3.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 35.08x (fastest 754.2 ns, slowest 26458.3 ns)

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
| dep_graph_csr | 2870ns | 2857ns | 2848ns | 2864ns | 2913ns | base |
| dep_graph_dense | 28657ns | 28566ns | 28499ns | 28570ns | 29074ns | +898.34% |
| dep_graph_hybrid | 2853ns | 2841ns | 2829ns | 2844ns | 2901ns | -0.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 783ns | 776ns | 800ns | base | 0.163 |
| dep_graph_dense | 26543ns | 26407ns | 26922ns | +3287.80% | 0.005 |
| dep_graph_hybrid | 757ns | 752ns | 768ns | -3.39% | 0.169 |

## Performance model

- Peak throughput: **0.170 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.164 | 96.5% |
| dep_graph_dense | 0.005 | 2.8% |
| dep_graph_hybrid | 0.170 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2870ns | 2870ns | base |
| dep_graph_dense | 28657ns | 28657ns | +898.34% |
| dep_graph_hybrid | 2853ns | 2853ns | -0.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 779ns | base | --- | [779, 780] | --- | --- | --- | --- |
| dep_graph_dense | 26458ns | +25678.3ns (+3295.5%) | [+25670, +25686]ns | [26448, 26468] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 754ns | -24.6ns (-3.2%) | [-25, -24]ns | [754, 755] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 779ns | +3317.0% | -1.4% |
| 1 | 2 | 792ns | +3237.4% | -4.8% |
| 1 | 3 | 778ns | +3301.5% | -3.1% |
| 1 | 4 | 777ns | +3304.7% | -1.7% |
| 1 | 5 | 781ns | +3284.0% | -3.5% |
| 1 | 6 | 779ns | +3298.4% | -3.2% |
| 1 | 7 | 778ns | +3299.9% | -3.0% |
| 1 | 8 | 782ns | +3275.6% | -3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.276 | moderate- |
| dep_graph_dense | -0.222 | moderate- |
| dep_graph_hybrid | -0.178 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 52.9ns | 783.5ns | 6.7% | HIGH |
| dep_graph_dense | 8.0ns | 26543.0ns | 0.0% |  |
| dep_graph_hybrid | 58.2ns | 756.9ns | 7.7% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 775.5-799.9 ns)
    775.5 |####################################
    776.7 |########################################
    777.9 |###############################
    779.2 |############################
    780.4 |#############
    781.6 |###########
    782.8 |###
    784.0 |###
    785.3 |
    786.5 |
    787.7 |###
    788.9 |
    790.1 |####################
    791.3 |####################
    792.6 |#############
    793.8 |##########
    795.0 |#
    796.2 |
    797.4 |
    798.6 |
  (14 below, 4 above range)

dep_graph_dense (n=160, range 26407.0-26922.0 ns)
  26407.0 |########################################
  26432.7 |##############################
  26458.5 |########################
  26484.2 |################
  26510.0 |#########
  26535.7 |#######
  26561.5 |########
  26587.2 |###
  26613.0 |###
  26638.8 |###
  26664.5 |#
  26690.3 |
  26716.0 |
  26741.8 |
  26767.5 |
  26793.3 |
  26819.0 |
  26844.8 |
  26870.5 |
  26896.3 |
  (15 below, 8 above range)

dep_graph_hybrid (n=160, range 752.0-768.1 ns)
    752.0 |############################
    752.8 |######################################
    753.6 |########################################
    754.4 |######################
    755.2 |################
    756.0 |####
    756.8 |#
    757.6 |
    758.4 |
    759.2 |#
    760.0 |
    760.8 |
    761.7 |##
    762.5 |#
    763.3 |
    764.1 |
    764.9 |####
    765.7 |######
    766.5 |########
    767.3 |######
  (11 below, 16 above range)

```
