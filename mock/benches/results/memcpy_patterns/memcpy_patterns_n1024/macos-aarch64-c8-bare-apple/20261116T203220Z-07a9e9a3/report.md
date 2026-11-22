# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (10 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (1.42 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.3% of the fastest

All 3 variants sit between 1.42 us and 1.43 us - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_unrolled's edge over baseline is significant but tiny (5 ns, 0.34%)

memcpy_unrolled differs from baseline memcpy_intrinsic by 5 ns (0.34%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 1422.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 1422.1 ns, slowest 1426.0 ns)

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
| memcpy_intrinsic | 3528ns | 3524ns | 3513ns | 3524ns | 3555ns | base |
| memcpy_naive | 3533ns | 3527ns | 3517ns | 3527ns | 3569ns | +0.16% |
| memcpy_unrolled | 3558ns | 3530ns | 3520ns | 3531ns | 3677ns | +0.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 1423ns | 1413ns | 1438ns | base | 0.720 |
| memcpy_naive | 1428ns | 1416ns | 1451ns | +0.35% | 0.717 |
| memcpy_unrolled | 1437ns | 1417ns | 1488ns | +0.95% | 0.713 |

## Performance model

- Peak throughput: **0.725 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.720 | 99.3% |
| memcpy_naive | 0.719 | 99.2% |
| memcpy_unrolled | 0.718 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 3528ns | 3528ns | base |
| memcpy_naive | 3533ns | 3533ns | +0.16% |
| memcpy_unrolled | 3558ns | 3558ns | +0.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 1422ns | base | --- | [1420, 1423] | --- | --- | --- | --- |
| memcpy_naive | 1424ns | no significant difference | [-0, +2]ns | [1423, 1426] | no | 0.1506 | 0.1506 | 3 |
| memcpy_unrolled | 1426ns | +4.8ns (+0.3%) | [+3, +8]ns | [1425, 1428] | YES | 0.0000 | 0.0000 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 1421ns | +0.5% | +0.2% |
| 1 | 2 | 1416ns | +0.9% | +0.4% |
| 1 | 3 | 1427ns | -0.2% | -0.4% |
| 1 | 4 | 1423ns | -0.2% | -0.1% |
| 1 | 5 | 1426ns | -0.1% | +0.1% |
| 1 | 6 | 1425ns | -0.1% | +4.8% |
| 1 | 7 | 1420ns | -0.1% | +0.5% |
| 1 | 8 | 1417ns | +0.6% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.046 | ok |
| memcpy_naive | 0.095 | ok |
| memcpy_unrolled | -0.060 | ok |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 3/8
- **memcpy_unrolled**: won 1/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 4.0ns | 1423.2ns | 0.3% |  |
| memcpy_naive | 4.1ns | 1428.2ns | 0.3% |  |
| memcpy_unrolled | 4.1ns | 1436.7ns | 0.3% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 1412.8-1437.6 ns)
   1412.8 |#####################
   1414.0 |#####################
   1415.3 |##############################
   1416.5 |#################################
   1417.7 |####################################
   1419.0 |########################
   1420.2 |###########################
   1421.5 |###########################
   1422.7 |########################################
   1423.9 |########################
   1425.2 |###########################
   1426.4 |########################
   1427.7 |##################
   1428.9 |#####################
   1430.2 |#########
   1431.4 |###############
   1432.6 |######
   1433.9 |######
   1435.1 |######
   1436.4 |###
  (13 below, 8 above range)

memcpy_naive (n=160, range 1416.5-1451.3 ns)
   1416.5 |########################
   1418.2 |##############################
   1419.9 |#################
   1421.7 |########################################
   1423.4 |############################
   1425.2 |##########################
   1426.9 |##########################
   1428.7 |##########################
   1430.4 |#################
   1432.1 |###########
   1433.9 |#
   1435.6 |#####
   1437.4 |#
   1439.1 |
   1440.9 |
   1442.6 |
   1444.3 |#
   1446.1 |
   1447.8 |
   1449.6 |
  (14 below, 9 above range)

memcpy_unrolled (n=160, range 1416.9-1488.1 ns)
   1416.9 |##############################
   1420.5 |##################################
   1424.0 |########################################
   1427.6 |###########################
   1431.1 |############
   1434.7 |#
   1438.3 |##
   1441.8 |######
   1445.4 |#
   1449.0 |#
   1452.5 |
   1456.1 |
   1459.6 |
   1463.2 |
   1466.8 |#####
   1470.3 |#
   1473.9 |#
   1477.4 |##
   1481.0 |##
   1484.6 |##
  (13 below, 12 above range)

```
