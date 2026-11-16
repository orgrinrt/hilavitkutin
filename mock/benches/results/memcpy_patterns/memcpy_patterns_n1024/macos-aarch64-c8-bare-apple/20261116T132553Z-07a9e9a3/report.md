# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 3 variants sit between 1.42 us and 1.42 us - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (3 ns, 0.19%)

memcpy_naive differs from baseline memcpy_intrinsic by 3 ns (0.19%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_unrolled** at 1420.4 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 1420.4 ns, slowest 1424.8 ns)

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
| memcpy_intrinsic | 3526ns | 3523ns | 3510ns | 3523ns | 3553ns | base |
| memcpy_naive | 3533ns | 3528ns | 3512ns | 3529ns | 3568ns | +0.21% |
| memcpy_unrolled | 3524ns | 3520ns | 3512ns | 3520ns | 3549ns | -0.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 1424ns | 1414ns | 1441ns | base | 0.719 |
| memcpy_naive | 1428ns | 1416ns | 1446ns | +0.23% | 0.717 |
| memcpy_unrolled | 1422ns | 1413ns | 1434ns | -0.18% | 0.720 |

## Performance model

- Peak throughput: **0.725 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.720 | 99.4% |
| memcpy_naive | 0.719 | 99.2% |
| memcpy_unrolled | 0.721 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 3526ns | 3526ns | base |
| memcpy_naive | 3533ns | 3533ns | +0.21% |
| memcpy_unrolled | 3524ns | 3524ns | -0.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 1422ns | base | --- | [1420, 1424] | --- | --- | --- | --- |
| memcpy_naive | 1425ns | +2.7ns (+0.2%) | [+1, +4]ns | [1424, 1426] | YES | 0.0002 | 0.0001 | 4 |
| memcpy_unrolled | 1420ns | -1.6ns (-0.1%) | [-2, -0]ns | [1420, 1422] | YES | 0.0046 | 0.0046 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 1425ns | -0.1% | -0.2% |
| 1 | 2 | 1419ns | +0.6% | +0.0% |
| 1 | 3 | 1418ns | +0.8% | +0.1% |
| 1 | 4 | 1423ns | +0.3% | +0.1% |
| 1 | 5 | 1425ns | +0.1% | -0.4% |
| 1 | 6 | 1422ns | +0.1% | -0.1% |
| 1 | 7 | 1416ns | +0.0% | +0.1% |
| 1 | 8 | 1433ns | -0.6% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.308 | moderate- |
| memcpy_naive | 0.272 | moderate+ |
| memcpy_unrolled | -0.398 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 1/8, lost 4/8
- **memcpy_unrolled**: won 4/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 4.2ns | 1424.3ns | 0.3% |  |
| memcpy_naive | 4.1ns | 1427.6ns | 0.3% |  |
| memcpy_unrolled | 4.1ns | 1421.8ns | 0.3% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 1414.0-1440.9 ns)
   1414.0 |#############
   1415.4 |########################
   1416.7 |#################################
   1418.1 |########################################
   1419.4 |######################
   1420.7 |###############
   1422.1 |##########################
   1423.4 |######################
   1424.8 |###############
   1426.1 |###########
   1427.5 |##########################
   1428.8 |######
   1430.1 |###########
   1431.5 |########
   1432.8 |########
   1434.2 |##
   1435.5 |##
   1436.9 |######
   1438.2 |######
   1439.5 |####
  (13 below, 8 above range)

memcpy_naive (n=160, range 1416.3-1445.9 ns)
   1416.3 |############
   1417.8 |################
   1419.2 |######
   1420.7 |###################################
   1422.2 |########################################
   1423.7 |###################################
   1425.2 |#############################
   1426.6 |#############################
   1428.1 |##########
   1429.6 |################
   1431.1 |##################
   1432.6 |############
   1434.0 |############
   1435.5 |####
   1437.0 |##
   1438.5 |####
   1440.0 |######
   1441.4 |
   1442.9 |####
   1444.4 |
  (13 below, 5 above range)

memcpy_unrolled (n=160, range 1413.4-1433.9 ns)
   1413.4 |###########
   1414.4 |##########################
   1415.4 |#################
   1416.4 |#############
   1417.5 |############################
   1418.5 |#################
   1419.5 |########################################
   1420.5 |#############
   1421.6 |########################################
   1422.6 |########
   1423.6 |##########################
   1424.6 |#############
   1425.7 |##
   1426.7 |########################
   1427.7 |########
   1428.7 |####
   1429.8 |
   1430.8 |####
   1431.8 |####
   1432.8 |
  (13 below, 9 above range)

```
