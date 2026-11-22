# DependencyGraph backing: dense vs CSR vs hybrid (Topic 9 axis B gate)

3 variants, 160 samples per variant.
Baseline: **dep_graph_csr**

## Highlights

Baseline for all deltas below: **dep_graph_csr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dep_graph_dense is an outlier: 66.1x slower than the field

dep_graph_dense (104.01 us) is 66.1x the fastest (1.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 66.1x the fastest

Fastest dep_graph_hybrid (1.57 us) to slowest dep_graph_dense (104.01 us): 66.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: dep_graph_hybrid** at 1573.3 ns median (-3.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 66.11x (fastest 1573.3 ns, slowest 104007.5 ns)

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
| dep_graph_csr | 3734ns | 3704ns | 3690ns | 3716ns | 3829ns | base |
| dep_graph_dense | 106691ns | 106207ns | 105995ns | 106264ns | 108669ns | +2757.66% |
| dep_graph_hybrid | 3666ns | 3651ns | 3638ns | 3655ns | 3725ns | -1.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dep_graph_csr | 1642ns | 1618ns | 1690ns | base | 0.156 |
| dep_graph_dense | 104503ns | 103875ns | 106454ns | +6266.29% | 0.002 |
| dep_graph_hybrid | 1580ns | 1567ns | 1611ns | -3.73% | 0.162 |

## Performance model

- Peak throughput: **0.163 Gops/s** (dep_graph_hybrid; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dep_graph_csr | 0.157 | 96.1% |
| dep_graph_dense | 0.002 | 1.5% |
| dep_graph_hybrid | 0.163 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dep_graph_csr | 3734ns | 3734ns | base |
| dep_graph_dense | 106691ns | 106691ns | +2757.66% |
| dep_graph_hybrid | 3666ns | 3666ns | -1.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dep_graph_csr | 1630ns | base | --- | [1629, 1632] | --- | --- | --- | --- |
| dep_graph_dense | 104008ns | +102372.9ns (+6280.5%) | [+102331, +102420]ns | [103970, 104053] | YES | 0.0000 | 0.0000 | 0 |
| dep_graph_hybrid | 1573ns | -54.8ns (-3.4%) | [-57, -53]ns | [1572, 1575] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dep_graph_csr | dep_graph_dense | dep_graph_hybrid |
|---|---|---|---|---|
| 1 | 1 | 1642ns | +6445.1% | -4.4% |
| 1 | 2 | 1629ns | +6285.2% | -3.5% |
| 1 | 3 | 1636ns | +6265.8% | -3.8% |
| 1 | 4 | 1628ns | +6281.4% | -3.4% |
| 1 | 5 | 1625ns | +6297.9% | -3.1% |
| 1 | 6 | 1625ns | +6301.2% | -3.1% |
| 1 | 7 | 1631ns | +6271.3% | -2.7% |
| 1 | 8 | 1665ns | +6143.9% | -5.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dep_graph_csr | 0.043 | ok |
| dep_graph_dense | -0.008 | ok |
| dep_graph_hybrid | 0.037 | ok |

**Consistency summary:**

- **dep_graph_dense**: won 0/8, lost 8/8
- **dep_graph_hybrid**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dep_graph_csr | 3.6ns | 1641.5ns | 0.2% |  |
| dep_graph_dense | 11.7ns | 104502.8ns | 0.0% |  |
| dep_graph_hybrid | 9.8ns | 1580.3ns | 0.6% |  |

## Distribution (algo ns)

```
dep_graph_csr (n=160, range 1618.5-1690.4 ns)
   1618.5 |################################
   1622.1 |###########################
   1625.7 |##############################
   1629.3 |########################################
   1632.9 |##########
   1636.5 |######
   1640.1 |###
   1643.7 |#
   1647.3 |###
   1650.8 |#########
   1654.4 |####
   1658.0 |##########
   1661.6 |######
   1665.2 |######
   1668.8 |###
   1672.4 |####
   1676.0 |###
   1679.6 |###
   1683.2 |###
   1686.8 |#
  (15 below, 8 above range)

dep_graph_dense (n=160, range 103874.6-106454.0 ns)
  103874.6 |########################################
  104003.6 |###############
  104132.5 |######
  104261.5 |####
  104390.5 |
  104519.4 |##
  104648.4 |
  104777.4 |#
  104906.3 |#
  105035.3 |
  105164.3 |
  105293.2 |
  105422.2 |##
  105551.2 |
  105680.2 |
  105809.1 |
  105938.1 |#
  106067.1 |
  106196.0 |
  106325.0 |
  (14 below, 17 above range)

dep_graph_hybrid (n=160, range 1567.1-1610.5 ns)
   1567.1 |########################################
   1569.2 |########################################
   1571.4 |########################################
   1573.6 |########################################
   1575.8 |#################
   1577.9 |###
   1580.1 |###
   1582.3 |###
   1584.5 |######
   1586.6 |###
   1588.8 |#
   1591.0 |###
   1593.2 |#####
   1595.3 |#####
   1597.5 |##########
   1599.7 |###
   1601.8 |######
   1604.0 |#####
   1606.2 |#
   1608.4 |###
  (12 below, 7 above range)

```
