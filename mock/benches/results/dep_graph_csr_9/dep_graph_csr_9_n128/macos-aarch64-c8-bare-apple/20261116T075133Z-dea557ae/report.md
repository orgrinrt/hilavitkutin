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

- **Fastest: dep_graph_hybrid** at 754.2 ns median (-3.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 35.08x (fastest 754.2 ns, slowest 26460.0 ns)

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
| dep_graph_csr | 2869ns | 2855ns | 2836ns | 2866ns | 2908ns | base |
| dep_graph_dense | 28704ns | 28552ns | 28494ns | 28563ns | 29339ns | +900.63% |
| dep_graph_hybrid | 2851ns | 2831ns | 2817ns | 2844ns | 2909ns | -0.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 784ns | 776ns | 794ns | base | 0.163 |
| dep_graph_dense | 26599ns | 26410ns | 27183ns | +3291.08% | 0.005 |
| dep_graph_hybrid | 762ns | 752ns | 785ns | -2.88% | 0.168 |

## Performance model

- Peak throughput: **0.170 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.164 | 96.4% |
| dep_graph_dense | 0.005 | 2.8% |
| dep_graph_hybrid | 0.170 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2869ns | 2869ns | base |
| dep_graph_dense | 28704ns | 28704ns | +900.63% |
| dep_graph_hybrid | 2851ns | 2851ns | -0.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 780ns | base | --- | [779, 789] | --- | --- | --- | --- |
| dep_graph_dense | 26460ns | +25679.6ns (+3292.3%) | [+25656, +25690]ns | [26447, 26472] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 754ns | -24.6ns (-3.2%) | [-25, -24]ns | [754, 755] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 778ns | +3300.5% | -3.0% |
| 1 | 2 | 778ns | +3299.1% | -3.2% |
| 1 | 3 | 790ns | +3243.4% | -4.7% |
| 1 | 4 | 793ns | +3231.3% | -3.0% |
| 1 | 5 | 792ns | +3234.7% | -3.3% |
| 1 | 6 | 778ns | +3319.1% | -3.1% |
| 1 | 7 | 779ns | +3318.7% | -3.2% |
| 1 | 8 | 791ns | +3239.3% | -3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.178 | ok |
| dep_graph_dense | 0.252 | moderate+ |
| dep_graph_hybrid | 0.061 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 53.3ns | 784.4ns | 6.8% | HIGH |
| dep_graph_dense | 8.9ns | 26598.6ns | 0.0% |  |
| dep_graph_hybrid | 58.6ns | 761.8ns | 7.7% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 775.9-794.0 ns)
    775.9 |##########################
    776.8 |###################################
    777.7 |###################################
    778.6 |########################
    779.5 |############################
    780.4 |######
    781.3 |
    782.2 |
    783.1 |
    784.0 |
    784.9 |##
    785.9 |
    786.8 |####
    787.7 |##
    788.6 |##
    789.5 |##
    790.4 |####################
    791.3 |########################
    792.2 |#####################################
    793.1 |########################################
  (17 below, 11 above range)

dep_graph_dense (n=160, range 26409.6-27183.3 ns)
  26409.6 |########################################
  26448.3 |#######################
  26487.0 |########
  26525.7 |##########
  26564.4 |####
  26603.1 |###
  26641.7 |#
  26680.4 |
  26719.1 |
  26757.8 |
  26796.5 |
  26835.1 |
  26873.8 |
  26912.5 |
  26951.2 |
  26989.9 |
  27028.6 |
  27067.2 |
  27105.9 |
  27144.6 |
  (18 below, 16 above range)

dep_graph_hybrid (n=160, range 751.8-784.9 ns)
    751.8 |########################################
    753.4 |#########################
    755.1 |######
    756.8 |##
    758.4 |
    760.1 |
    761.7 |#
    763.4 |
    765.0 |########
    766.7 |#######
    768.4 |###########
    770.0 |#####
    771.7 |#
    773.3 |
    775.0 |
    776.7 |
    778.3 |#
    780.0 |#
    781.6 |
    783.3 |
  (14 below, 7 above range)

```
