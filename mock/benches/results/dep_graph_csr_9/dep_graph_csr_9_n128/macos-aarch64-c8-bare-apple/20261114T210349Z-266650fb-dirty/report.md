# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 32.9x slower than the field

dep_graph_dense (26.53 us) is 32.9x the fastest (806 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (dep_graph_csr, dep_graph_hybrid) are a dead heat (<1%)

dep_graph_csr (806 ns) and dep_graph_hybrid (808 ns) differ by 0.31%, inside the noise, even though the wider field spreads 3192.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (dep_graph_csr)

The baseline dep_graph_csr is the fastest (806 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 32.9x the fastest

Fastest dep_graph_csr (806 ns) to slowest dep_graph_dense (26.53 us): 32.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (dep_graph_csr) is the fastest** at 805.8 ns median
- 1 variant significantly slower than baseline
- Spread: 32.92x (fastest 805.8 ns, slowest 26527.7 ns)

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
| dep_graph_csr | 2964ns | 2936ns | 2838ns | 2952ns | 3127ns | base |
| dep_graph_dense | 28859ns | 28640ns | 28519ns | 28692ns | 29700ns | +873.53% |
| dep_graph_hybrid | 3021ns | 3023ns | 2892ns | 3015ns | 3168ns | +1.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 811ns | 777ns | 856ns | base | 0.158 |
| dep_graph_dense | 26740ns | 26428ns | 27523ns | +3195.59% | 0.005 |
| dep_graph_hybrid | 807ns | 770ns | 853ns | -0.49% | 0.159 |

## Performance model

- Peak throughput: **0.166 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.159 | 95.5% |
| dep_graph_dense | 0.005 | 2.9% |
| dep_graph_hybrid | 0.158 | 95.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 2964ns | 2964ns | base |
| dep_graph_dense | 28859ns | 28859ns | +873.53% |
| dep_graph_hybrid | 3021ns | 3021ns | +1.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 806ns | base | --- | [793, 833] | --- | --- | --- | --- |
| dep_graph_dense | 26528ns | +25713.3ns (+3191.0%) | [+25695, +25751]ns | [26504, 26552] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 808ns | no significant difference | [-21, +16]ns | [808, 809] | no | 0.8126 | 0.8126 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 777ns | +3322.8% | +4.2% |
| 1 | 2 | 782ns | +3320.9% | +1.2% |
| 1 | 3 | 835ns | +3075.7% | -5.8% |
| 1 | 4 | 780ns | +3307.8% | +11.5% |
| 1 | 5 | 834ns | +3080.5% | -2.8% |
| 1 | 6 | 834ns | +3084.6% | -6.8% |
| 1 | 7 | 807ns | +3179.5% | +0.2% |
| 1 | 8 | 792ns | +3240.3% | +2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.140 | ok |
| dep_graph_dense | 0.074 | ok |
| dep_graph_hybrid | -0.165 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 55.9ns | 811.4ns | 6.9% | HIGH |
| dep_graph_dense | 8.1ns | 26740.1ns | 0.0% |  |
| dep_graph_hybrid | 61.4ns | 807.4ns | 7.6% | HIGH |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 776.8-856.0 ns)
    776.8 |#######################
    780.8 |#######
    784.7 |#
    788.7 |###
    792.7 |######
    796.6 |
    800.6 |
    804.5 |#######
    808.5 |
    812.5 |
    816.4 |
    820.4 |
    824.3 |##
    828.3 |
    832.3 |########################################
    836.2 |##
    840.2 |
    844.1 |
    848.1 |
    852.1 |
  (17 below, 11 above range)

dep_graph_dense (n=160, range 26427.7-27523.1 ns)
  26427.7 |#####################################
  26482.5 |########################################
  26537.2 |#####################
  26592.0 |#########
  26646.8 |###
  26701.5 |#
  26756.3 |#
  26811.1 |###
  26865.9 |##
  26920.6 |######
  26975.4 |####
  27030.2 |####
  27085.0 |#
  27139.7 |
  27194.5 |#
  27249.3 |##
  27304.1 |#
  27358.8 |######
  27413.6 |###
  27468.4 |#
  (15 below, 11 above range)

dep_graph_hybrid (n=160, range 769.5-852.8 ns)
    769.5 |#
    773.7 |##
    777.8 |#########
    782.0 |######
    786.2 |###
    790.3 |###
    794.5 |#
    798.7 |#
    802.8 |########
    807.0 |########################################
    811.2 |#############
    815.3 |###
    819.5 |##
    823.7 |
    827.8 |
    832.0 |
    836.2 |
    840.3 |
    844.5 |
    848.7 |
  (13 below, 18 above range)

```
