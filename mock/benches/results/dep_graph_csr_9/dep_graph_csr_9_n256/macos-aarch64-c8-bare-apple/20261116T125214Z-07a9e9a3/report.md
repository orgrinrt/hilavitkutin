# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 66.1x slower than the field

dep_graph_dense (103.92 us) is 66.1x the fastest (1.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 66.1x the fastest

Fastest dep_graph_hybrid (1.57 us) to slowest dep_graph_dense (103.92 us): 66.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dep_graph_hybrid** at 1572.7 ns median (-3.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 66.08x (fastest 1572.7 ns, slowest 103917.7 ns)

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
| dep_graph_csr | 3710ns | 3703ns | 3697ns | 3704ns | 3744ns | base |
| dep_graph_dense | 106206ns | 106155ns | 105976ns | 106157ns | 106584ns | +2762.41% |
| dep_graph_hybrid | 3672ns | 3658ns | 3651ns | 3658ns | 3734ns | -1.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 1626ns | 1615ns | 1644ns | base | 0.157 |
| dep_graph_dense | 103993ns | 103853ns | 104317ns | +6293.76% | 0.002 |
| dep_graph_hybrid | 1578ns | 1568ns | 1602ns | -3.00% | 0.162 |

## Performance model

- Peak throughput: **0.163 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.157 | 96.4% |
| dep_graph_dense | 0.002 | 1.5% |
| dep_graph_hybrid | 0.163 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 3710ns | 3710ns | base |
| dep_graph_dense | 106206ns | 106206ns | +2762.41% |
| dep_graph_hybrid | 3672ns | 3672ns | -1.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 1625ns | base | --- | [1623, 1626] | --- | --- | --- | --- |
| dep_graph_dense | 103918ns | +102293.6ns (+6293.4%) | [+102267, +102314]ns | [103901, 103944] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 1573ns | -50.4ns (-3.1%) | [-52, -50]ns | [1572, 1574] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 1627ns | +6288.5% | -3.3% |
| 1 | 2 | 1624ns | +6295.7% | -3.3% |
| 1 | 3 | 1624ns | +6302.0% | -3.1% |
| 1 | 4 | 1626ns | +6294.8% | -3.3% |
| 1 | 5 | 1625ns | +6297.0% | -3.1% |
| 1 | 6 | 1625ns | +6296.8% | -3.2% |
| 1 | 7 | 1625ns | +6292.9% | -3.2% |
| 1 | 8 | 1626ns | +6293.4% | -3.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.215 | moderate- |
| dep_graph_dense | -0.014 | ok |
| dep_graph_hybrid | -0.181 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 4.1ns | 1626.5ns | 0.2% |  |
| dep_graph_dense | 7.6ns | 103992.5ns | 0.0% |  |
| dep_graph_hybrid | 8.4ns | 1577.7ns | 0.5% |  |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 1615.0-1644.0 ns)
   1615.0 |#################
   1616.5 |########################################
   1617.9 |###############
   1619.4 |#############
   1620.8 |###############
   1622.3 |####################
   1623.7 |###############
   1625.2 |#################################
   1626.6 |############################
   1628.1 |##########################
   1629.5 |########################
   1631.0 |####################
   1632.4 |###############
   1633.9 |####
   1635.3 |
   1636.8 |
   1638.2 |
   1639.7 |
   1641.1 |
   1642.6 |##
  (16 below, 12 above range)

dep_graph_dense (n=160, range 103852.8-104316.8 ns)
  103852.8 |######################################
  103876.0 |########################################
  103899.2 |#####################
  103922.4 |#################
  103945.6 |############
  103968.8 |#################
  103992.0 |########
  104015.2 |#####
  104038.4 |##########
  104061.6 |########
  104084.8 |####
  104108.0 |##
  104131.2 |#####
  104154.4 |##
  104177.6 |##
  104200.8 |
  104224.0 |##
  104247.2 |
  104270.4 |#
  104293.6 |#
  (12 below, 5 above range)

dep_graph_hybrid (n=160, range 1567.5-1602.4 ns)
   1567.5 |####################
   1569.3 |#####################################
   1571.0 |################################
   1572.8 |#########################
   1574.5 |########################################
   1576.2 |###############
   1578.0 |##########
   1579.7 |#
   1581.5 |#
   1583.2 |
   1585.0 |#
   1586.7 |
   1588.5 |##
   1590.2 |
   1591.9 |
   1593.7 |##
   1595.4 |
   1597.2 |#
   1598.9 |#
   1600.7 |
  (17 below, 7 above range)

```
