# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 35.1x slower than the field

dep_graph_dense (26.46 us) is 35.1x the fastest (753 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 35.1x the fastest

Fastest dep_graph_hybrid (753 ns) to slowest dep_graph_dense (26.46 us): 35.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dep_graph_hybrid** at 753.3 ns median (-3.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 35.12x (fastest 753.3 ns, slowest 26456.7 ns)

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
| dep_graph_csr | 2859ns | 2853ns | 2845ns | 2853ns | 2890ns | base |
| dep_graph_dense | 28575ns | 28558ns | 28510ns | 28562ns | 28677ns | +899.57% |
| dep_graph_hybrid | 2838ns | 2835ns | 2828ns | 2835ns | 2856ns | -0.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 780ns | 775ns | 791ns | base | 0.164 |
| dep_graph_dense | 26469ns | 26416ns | 26558ns | +3294.32% | 0.005 |
| dep_graph_hybrid | 754ns | 752ns | 759ns | -3.29% | 0.170 |

## Performance model

- Peak throughput: **0.170 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.165 | 96.7% |
| dep_graph_dense | 0.005 | 2.8% |
| dep_graph_hybrid | 0.170 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2859ns | 2859ns | base |
| dep_graph_dense | 28575ns | 28575ns | +899.57% |
| dep_graph_hybrid | 2838ns | 2838ns | -0.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 778ns | base | --- | [777, 778] | --- | --- | --- | --- |
| dep_graph_dense | 26457ns | +25673.8ns (+3302.1%) | [+25663, +25689]ns | [26444, 26469] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 753ns | -24.6ns (-3.2%) | [-25, -24]ns | [753, 754] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 778ns | +3299.8% | -3.1% |
| 1 | 2 | 778ns | +3301.4% | -3.1% |
| 1 | 3 | 778ns | +3302.9% | -3.0% |
| 1 | 4 | 777ns | +3302.8% | -3.1% |
| 1 | 5 | 779ns | +3295.5% | -3.3% |
| 1 | 6 | 778ns | +3303.6% | -3.2% |
| 1 | 7 | 780ns | +3289.8% | -3.4% |
| 1 | 8 | 776ns | +3311.5% | -2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.499 | moderate- |
| dep_graph_dense | -0.061 | ok |
| dep_graph_hybrid | -0.120 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 52.4ns | 779.8ns | 6.7% | HIGH |
| dep_graph_dense | 7.6ns | 26469.4ns | 0.0% |  |
| dep_graph_hybrid | 57.5ns | 754.2ns | 7.6% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 774.7-791.3 ns)
    774.7 |##########################
    775.5 |###################################
    776.3 |###############################
    777.2 |########################################
    778.0 |####################################
    778.8 |#####################
    779.7 |#####################
    780.5 |#####
    781.3 |######
    782.2 |#
    783.0 |#
    783.8 |
    784.7 |
    785.5 |
    786.3 |
    787.2 |
    788.0 |
    788.8 |#
    789.6 |
    790.5 |
  (12 below, 10 above range)

dep_graph_dense (n=160, range 26415.9-26558.0 ns)
  26415.9 |########################################
  26423.0 |############################
  26430.1 |######################
  26437.2 |########################
  26444.3 |############
  26451.4 |##############
  26458.5 |##############
  26465.6 |################
  26472.7 |##################
  26479.8 |##################
  26486.9 |##################
  26494.0 |##########
  26501.2 |########
  26508.3 |##
  26515.4 |##########
  26522.5 |######
  26529.6 |##########
  26536.7 |######
  26543.8 |##
  26550.9 |####
  (12 below, 7 above range)

dep_graph_hybrid (n=160, range 751.9-758.7 ns)
    751.9 |###################
    752.2 |##########################
    752.6 |##########################
    752.9 |
    753.2 |##########################
    753.6 |########################################
    753.9 |######################
    754.3 |#############
    754.6 |
    755.0 |############
    755.3 |#########
    755.6 |#
    756.0 |##
    756.3 |
    756.7 |#
    757.0 |
    757.4 |
    757.7 |
    758.0 |
    758.4 |
  (11 below, 3 above range)

```
