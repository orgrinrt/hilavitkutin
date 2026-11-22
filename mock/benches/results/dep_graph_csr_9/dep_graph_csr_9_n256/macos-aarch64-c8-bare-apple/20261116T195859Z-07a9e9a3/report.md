# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 66.0x slower than the field

dep_graph_dense (103.92 us) is 66.0x the fastest (1.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 66.0x the fastest

Fastest dep_graph_hybrid (1.57 us) to slowest dep_graph_dense (103.92 us): 66.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dep_graph_hybrid** at 1573.8 ns median (-3.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 66.03x (fastest 1573.8 ns, slowest 103921.0 ns)

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
| dep_graph_csr | 3713ns | 3705ns | 3698ns | 3705ns | 3749ns | base |
| dep_graph_dense | 106464ns | 106158ns | 106031ns | 106187ns | 107727ns | +2767.53% |
| dep_graph_hybrid | 3689ns | 3661ns | 3653ns | 3664ns | 3797ns | -0.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 1627ns | 1616ns | 1644ns | base | 0.157 |
| dep_graph_dense | 104242ns | 103861ns | 105464ns | +6307.83% | 0.002 |
| dep_graph_hybrid | 1585ns | 1568ns | 1635ns | -2.56% | 0.161 |

## Performance model

- Peak throughput: **0.163 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.158 | 96.5% |
| dep_graph_dense | 0.002 | 1.5% |
| dep_graph_hybrid | 0.163 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 3713ns | 3713ns | base |
| dep_graph_dense | 106464ns | 106464ns | +2767.53% |
| dep_graph_hybrid | 3689ns | 3689ns | -0.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 1625ns | base | --- | [1624, 1627] | --- | --- | --- | --- |
| dep_graph_dense | 103921ns | +102292.7ns (+6294.9%) | [+102283, +102341]ns | [103910, 103964] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 1574ns | -50.4ns (-3.1%) | [-51, -49]ns | [1572, 1575] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 1626ns | +6295.6% | -3.3% |
| 1 | 2 | 1628ns | +6283.4% | -3.4% |
| 1 | 3 | 1625ns | +6292.5% | -3.2% |
| 1 | 4 | 1625ns | +6293.5% | -3.2% |
| 1 | 5 | 1625ns | +6299.4% | +0.5% |
| 1 | 6 | 1625ns | +6294.7% | -3.3% |
| 1 | 7 | 1625ns | +6295.9% | -3.2% |
| 1 | 8 | 1624ns | +6300.2% | -3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.395 | moderate+ |
| dep_graph_dense | -0.142 | ok |
| dep_graph_hybrid | -0.173 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 4.0ns | 1626.8ns | 0.2% |  |
| dep_graph_dense | 8.0ns | 104242.0ns | 0.0% |  |
| dep_graph_hybrid | 8.6ns | 1585.2ns | 0.5% |  |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 1615.9-1644.0 ns)
   1615.9 |################
   1617.3 |###################################
   1618.7 |#####################
   1620.1 |##################
   1621.5 |######
   1622.9 |##############
   1624.3 |#################################
   1625.7 |########################################
   1627.1 |################
   1628.5 |#####################
   1629.9 |#############################
   1631.3 |########
   1632.7 |############
   1634.1 |####
   1635.5 |
   1636.9 |##
   1638.4 |##
   1639.8 |
   1641.2 |
   1642.6 |##
  (14 below, 10 above range)

dep_graph_dense (n=160, range 103861.4-105463.8 ns)
  103861.4 |########################################
  103941.5 |##########
  104021.6 |########
  104101.7 |#####
  104181.9 |##
  104262.0 |##
  104342.1 |
  104422.2 |
  104502.4 |
  104582.5 |
  104662.6 |
  104742.7 |
  104822.8 |
  104903.0 |
  104983.1 |
  105063.2 |
  105143.3 |
  105223.5 |
  105303.6 |
  105383.7 |
  (15 below, 13 above range)

dep_graph_hybrid (n=160, range 1567.6-1634.6 ns)
   1567.6 |#######################
   1571.0 |########################################
   1574.3 |######################
   1577.7 |#####
   1581.0 |#
   1584.4 |
   1587.7 |#
   1591.1 |#
   1594.4 |#
   1597.8 |
   1601.1 |#
   1604.5 |
   1607.8 |
   1611.2 |
   1614.5 |
   1617.9 |
   1621.2 |#
   1624.5 |
   1627.9 |#####
   1631.2 |
  (14 below, 12 above range)

```
