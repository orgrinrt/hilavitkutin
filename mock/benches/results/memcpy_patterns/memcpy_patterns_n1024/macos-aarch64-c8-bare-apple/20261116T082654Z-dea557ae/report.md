# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (1.45 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.1% of the fastest

All 3 variants sit between 1.45 us and 1.45 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_unrolled's edge over baseline is significant but tiny (2 ns, 0.12%)

memcpy_unrolled differs from baseline memcpy_intrinsic by 2 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 1450.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 1450.8 ns, slowest 1452.3 ns)

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
| memcpy_intrinsic | 3583ns | 3579ns | 3568ns | 3579ns | 3610ns | base |
| memcpy_naive | 3586ns | 3578ns | 3569ns | 3579ns | 3625ns | +0.09% |
| memcpy_unrolled | 3593ns | 3585ns | 3573ns | 3586ns | 3635ns | +0.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 1451ns | 1441ns | 1463ns | base | 0.706 |
| memcpy_naive | 1453ns | 1442ns | 1467ns | +0.10% | 0.705 |
| memcpy_unrolled | 1453ns | 1443ns | 1468ns | +0.16% | 0.705 |

## Performance model

- Peak throughput: **0.710 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.706 | 99.4% |
| memcpy_naive | 0.706 | 99.3% |
| memcpy_unrolled | 0.705 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 3583ns | 3583ns | base |
| memcpy_naive | 3586ns | 3586ns | +0.09% |
| memcpy_unrolled | 3593ns | 3593ns | +0.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 1451ns | base | --- | [1450, 1452] | --- | --- | --- | --- |
| memcpy_naive | 1451ns | no significant difference | [-0, +2]ns | [1450, 1453] | no | 0.1480 | 0.1480 | 5 |
| memcpy_unrolled | 1452ns | +1.7ns (+0.1%) | [+1, +3]ns | [1450, 1453] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 1449ns | +0.2% | +0.1% |
| 1 | 2 | 1454ns | -0.5% | -0.2% |
| 1 | 3 | 1449ns | +0.2% | -0.1% |
| 1 | 4 | 1455ns | -0.2% | -0.7% |
| 1 | 5 | 1454ns | -0.1% | +0.2% |
| 1 | 6 | 1447ns | +0.5% | +0.9% |
| 1 | 7 | 1451ns | +0.1% | +0.1% |
| 1 | 8 | 1444ns | +0.3% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.207 | moderate- |
| memcpy_naive | -0.096 | ok |
| memcpy_unrolled | 0.313 | moderate+ |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 5/8
- **memcpy_unrolled**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.6ns | 1451.1ns | 0.3% |  |
| memcpy_naive | 3.8ns | 1452.6ns | 0.3% |  |
| memcpy_unrolled | 3.8ns | 1453.4ns | 0.3% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 1441.4-1462.7 ns)
   1441.4 |#########
   1442.5 |#########
   1443.5 |########################################
   1444.6 |############
   1445.7 |#####################
   1446.7 |###########################
   1447.8 |#####################
   1448.9 |########################
   1449.9 |#################################
   1451.0 |#################################
   1452.1 |########################
   1453.1 |####################################
   1454.2 |#################################
   1455.3 |########################
   1456.3 |############
   1457.4 |###
   1458.5 |############
   1459.5 |###############
   1460.6 |###############
   1461.6 |######
  (17 below, 7 above range)

memcpy_naive (n=160, range 1442.3-1467.1 ns)
   1442.3 |################
   1443.6 |########################
   1444.8 |##################
   1446.0 |#############################
   1447.3 |##########################
   1448.5 |################################
   1449.8 |########################
   1451.0 |#############################
   1452.2 |##################
   1453.5 |########################################
   1454.7 |##########################
   1456.0 |##################
   1457.2 |################
   1458.4 |#############
   1459.7 |########
   1460.9 |#####
   1462.2 |#####
   1463.4 |
   1464.6 |##
   1465.9 |
  (14 below, 13 above range)

memcpy_unrolled (n=160, range 1442.9-1468.2 ns)
   1442.9 |###########
   1444.2 |##################
   1445.4 |##############
   1446.7 |########################################
   1448.0 |#########################
   1449.2 |##################
   1450.5 |################
   1451.8 |#########################
   1453.0 |#######################
   1454.3 |#######################
   1455.5 |#######################
   1456.8 |#########
   1458.1 |################
   1459.3 |###########
   1460.6 |#########
   1461.9 |#########
   1463.1 |#########
   1464.4 |#######
   1465.7 |####
   1466.9 |##
  (16 below, 7 above range)

```
