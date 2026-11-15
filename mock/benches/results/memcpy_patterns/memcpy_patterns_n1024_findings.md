# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 3.5% of the fastest

All 3 variants sit between 1.47 us and 1.52 us - a 3.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (9 ns, 0.57%)

memcpy_naive differs from baseline memcpy_intrinsic by 9 ns (0.57%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_unrolled** at 1470.2 ns median (-2.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 1470.2 ns, slowest 1522.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 3681ns | 3746ns | 3504ns | 3681ns | 3856ns | base |
| memcpy_naive | 3728ns | 3764ns | 3510ns | 3728ns | 3943ns | +1.28% |
| memcpy_unrolled | 3659ns | 3657ns | 3508ns | 3657ns | 3818ns | -0.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 1484ns | 1411ns | 1561ns | base | 0.690 |
| memcpy_naive | 1507ns | 1417ns | 1603ns | +1.55% | 0.679 |
| memcpy_unrolled | 1476ns | 1417ns | 1538ns | -0.54% | 0.694 |

## Performance model

- Peak throughput: **0.726 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.681 | 93.8% |
| memcpy_naive | 0.673 | 92.7% |
| memcpy_unrolled | 0.697 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 3681ns | 3681ns | base |
| memcpy_naive | 3728ns | 3728ns | +1.28% |
| memcpy_unrolled | 3659ns | 3659ns | -0.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 1505ns | base | --- | [1469, 1517] | --- | --- | --- | --- |
| memcpy_naive | 1522ns | +8.6ns (+0.6%) | [+5, +11]ns | [1517, 1525] | YES | 0.0000 | 0.0000 | 1 |
| memcpy_unrolled | 1470ns | no significant difference | [-13, +5]ns | [1442, 1511] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 1477ns | -3.4% | +2.6% |
| 1 | 2 | 1430ns | +5.8% | -0.5% |
| 1 | 3 | 1422ns | +6.9% | +2.7% |
| 1 | 4 | 1523ns | +0.3% | -3.1% |
| 1 | 5 | 1529ns | +0.0% | -6.1% |
| 1 | 6 | 1528ns | +0.2% | -6.2% |
| 1 | 7 | 1414ns | +0.9% | +7.5% |
| 1 | 8 | 1435ns | +6.3% | +6.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | 0.243 | moderate+ |
| memcpy_naive | -0.179 | ok |
| memcpy_unrolled | 0.077 | ok |

**Consistency summary:**

- **memcpy_naive**: won 1/8, lost 6/8
- **memcpy_unrolled**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 4.2ns | 1484.3ns | 0.3% |  |
| memcpy_naive | 4.0ns | 1507.3ns | 0.3% |  |
| memcpy_unrolled | 3.9ns | 1476.4ns | 0.3% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 1411.0-1561.5 ns)
   1411.0 |####################################
   1418.5 |###############
   1426.0 |#########
   1433.6 |######
   1441.1 |###
   1448.6 |###
   1456.1 |#
   1463.7 |#########
   1471.2 |######
   1478.7 |####
   1486.2 |
   1493.7 |###
   1501.3 |#######
   1508.8 |############
   1516.3 |########################################
   1523.8 |###################################
   1531.4 |##########
   1538.9 |####
   1546.4 |
   1553.9 |
  (14 below, 10 above range)

memcpy_naive (n=160, range 1417.1-1602.7 ns)
   1417.1 |########################
   1426.4 |########
   1435.7 |###
   1445.0 |#
   1454.3 |#
   1463.5 |##
   1472.8 |#
   1482.1 |####
   1491.4 |#
   1500.7 |##
   1509.9 |################
   1519.2 |########################################
   1528.5 |##############################
   1537.8 |####
   1547.1 |##
   1556.3 |
   1565.6 |##
   1574.9 |
   1584.2 |
   1593.5 |#
  (15 below, 19 above range)

memcpy_unrolled (n=160, range 1417.3-1537.7 ns)
   1417.3 |#####################
   1423.3 |#############################
   1429.3 |#########################
   1435.4 |####################
   1441.4 |#######
   1447.4 |###
   1453.4 |###
   1459.4 |#####
   1465.5 |#######
   1471.5 |#
   1477.5 |#
   1483.5 |###
   1489.6 |###
   1495.6 |###
   1501.6 |#
   1507.6 |#########
   1513.6 |########################################
   1519.7 |#########################
   1525.7 |#########################
   1531.7 |##############
  (13 below, 7 above range)

```
