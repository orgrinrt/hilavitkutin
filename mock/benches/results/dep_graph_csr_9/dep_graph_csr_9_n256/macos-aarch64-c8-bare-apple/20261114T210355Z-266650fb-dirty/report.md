# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 65.9x slower than the field

dep_graph_dense (104.06 us) is 65.9x the fastest (1.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 65.9x the fastest

Fastest dep_graph_hybrid (1.58 us) to slowest dep_graph_dense (104.06 us): 65.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dep_graph_hybrid** at 1579.8 ns median (-4.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 65.87x (fastest 1579.8 ns, slowest 104061.5 ns)

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
| dep_graph_csr | 3839ns | 3773ns | 3692ns | 3817ns | 4054ns | base |
| dep_graph_dense | 106630ns | 106270ns | 105997ns | 106289ns | 108288ns | +2677.30% |
| dep_graph_hybrid | 3748ns | 3690ns | 3641ns | 3709ns | 3970ns | -2.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 1689ns | 1619ns | 1795ns | base | 0.152 |
| dep_graph_dense | 104421ns | 103880ns | 105996ns | +6082.94% | 0.002 |
| dep_graph_hybrid | 1615ns | 1569ns | 1716ns | -4.35% | 0.158 |

## Performance model

- Peak throughput: **0.163 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.154 | 94.6% |
| dep_graph_dense | 0.002 | 1.5% |
| dep_graph_hybrid | 0.162 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 3839ns | 3839ns | base |
| dep_graph_dense | 106630ns | 106630ns | +2677.30% |
| dep_graph_hybrid | 3748ns | 3748ns | -2.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 1659ns | base | --- | [1651, 1692] | --- | --- | --- | --- |
| dep_graph_dense | 104061ns | +102376.1ns (+6171.0%) | [+102334, +102418]ns | [104030, 104096] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 1580ns | -78.1ns (-4.7%) | [-87, -56]ns | [1576, 1592] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 1627ns | +6293.3% | +3.6% |
| 1 | 2 | 1742ns | +5874.6% | -9.7% |
| 1 | 3 | 1625ns | +6304.5% | -3.3% |
| 1 | 4 | 1629ns | +6285.4% | +3.5% |
| 1 | 5 | 1748ns | +5851.5% | -10.0% |
| 1 | 6 | 1656ns | +6185.3% | -5.0% |
| 1 | 7 | 1651ns | +6201.7% | -3.1% |
| 1 | 8 | 1745ns | +5867.3% | -9.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | -0.456 | moderate- |
| dep_graph_dense | -0.345 | moderate- |
| dep_graph_hybrid | -0.309 | moderate- |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 4.0ns | 1688.9ns | 0.2% |  |
| dep_graph_dense | 11.1ns | 104420.6ns | 0.0% |  |
| dep_graph_hybrid | 9.1ns | 1615.4ns | 0.6% |  |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 1619.3-1794.5 ns)
   1619.3 |########################################
   1628.0 |######################
   1636.8 |####
   1645.6 |#################
   1654.3 |##############
   1663.1 |####
   1671.9 |##
   1680.6 |##
   1689.4 |########
   1698.1 |#
   1706.9 |
   1715.7 |#
   1724.4 |#######
   1733.2 |##########################
   1742.0 |###############################
   1750.7 |#######
   1759.5 |
   1768.2 |#
   1777.0 |#
   1785.8 |
  (17 below, 10 above range)

dep_graph_dense (n=160, range 103879.9-105996.3 ns)
  103879.9 |########################################
  103985.7 |###############################
  104091.5 |#####################
  104197.4 |#########
  104303.2 |#######
  104409.0 |##
  104514.8 |
  104620.6 |
  104726.5 |#
  104832.3 |
  104938.1 |
  105043.9 |#
  105149.7 |#
  105255.5 |
  105361.4 |
  105467.2 |##
  105573.0 |#
  105678.8 |#
  105784.6 |
  105890.5 |#
  (11 below, 8 above range)

dep_graph_hybrid (n=160, range 1568.7-1715.7 ns)
   1568.7 |########################################
   1576.0 |#############
   1583.4 |######
   1590.7 |###
   1598.1 |######
   1605.5 |#
   1612.8 |#
   1620.2 |##
   1627.5 |
   1634.9 |##
   1642.2 |
   1649.6 |
   1656.9 |
   1664.3 |
   1671.6 |##
   1679.0 |############
   1686.3 |#########
   1693.7 |
   1701.0 |
   1708.4 |
  (15 below, 8 above range)

```
