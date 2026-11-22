# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (30 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.1% of the fastest

All 3 variants sit between 5.27 us and 5.27 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_unrolled's edge over baseline is significant but tiny (-4 ns, 0.07%)

memcpy_unrolled differs from baseline memcpy_intrinsic by -4 ns (0.07%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_unrolled** at 5267.1 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 5267.1 ns, slowest 5271.2 ns)

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
| memcpy_intrinsic | 7410ns | 7372ns | 7362ns | 7377ns | 7557ns | base |
| memcpy_naive | 7439ns | 7377ns | 7359ns | 7388ns | 7671ns | +0.39% |
| memcpy_unrolled | 7384ns | 7372ns | 7356ns | 7373ns | 7443ns | -0.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 5297ns | 5261ns | 5410ns | base | 0.773 |
| memcpy_naive | 5314ns | 5261ns | 5479ns | +0.32% | 0.771 |
| memcpy_unrolled | 5276ns | 5257ns | 5320ns | -0.40% | 0.776 |

## Performance model

- Peak throughput: **0.779 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.777 | 99.7% |
| memcpy_naive | 0.777 | 99.7% |
| memcpy_unrolled | 0.778 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 7410ns | 7410ns | base |
| memcpy_naive | 7439ns | 7439ns | +0.39% |
| memcpy_unrolled | 7384ns | 7384ns | -0.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 5271ns | base | --- | [5269, 5272] | --- | --- | --- | --- |
| memcpy_naive | 5271ns | no significant difference | [-0, +3]ns | [5270, 5273] | no | 0.1788 | 0.1788 | 0 |
| memcpy_unrolled | 5267ns | -3.5ns (-0.1%) | [-6, -1]ns | [5265, 5268] | YES | 0.0169 | 0.0084 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 5440ns | +1.1% | -3.3% |
| 1 | 2 | 5268ns | +0.0% | +0.0% |
| 1 | 3 | 5271ns | -0.1% | +0.1% |
| 1 | 4 | 5274ns | -0.0% | -0.1% |
| 1 | 5 | 5269ns | -0.0% | +0.0% |
| 1 | 6 | 5272ns | -0.0% | -0.2% |
| 1 | 7 | 5269ns | -0.0% | -0.2% |
| 1 | 8 | 5267ns | +0.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.033 | ok |
| memcpy_naive | -0.017 | ok |
| memcpy_unrolled | 0.053 | ok |

**Consistency summary:**

- **memcpy_naive**: won 1/8, lost 2/8
- **memcpy_unrolled**: won 4/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.5ns | 5297.2ns | 0.1% |  |
| memcpy_naive | 3.6ns | 5313.9ns | 0.1% |  |
| memcpy_unrolled | 3.5ns | 5275.7ns | 0.1% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 5261.4-5410.0 ns)
   5261.4 |####################################
   5268.8 |########################################
   5276.2 |######
   5283.7 |#####
   5291.1 |
   5298.5 |
   5306.0 |##
   5313.4 |###
   5320.8 |##
   5328.3 |#
   5335.7 |
   5343.1 |
   5350.6 |
   5358.0 |
   5365.4 |
   5372.9 |
   5380.3 |
   5387.7 |
   5395.2 |
   5402.6 |
  (14 below, 14 above range)

memcpy_naive (n=160, range 5260.6-5478.7 ns)
   5260.6 |########################################
   5271.6 |###################
   5282.5 |
   5293.4 |###
   5304.3 |
   5315.2 |#
   5326.1 |#
   5337.0 |#
   5347.9 |
   5358.8 |
   5369.7 |#
   5380.6 |
   5391.5 |
   5402.4 |
   5413.3 |
   5424.2 |#
   5435.1 |
   5446.0 |
   5456.9 |##
   5467.8 |#
  (15 below, 16 above range)

memcpy_unrolled (n=160, range 5257.2-5320.1 ns)
   5257.2 |##############################
   5260.3 |##########################
   5263.5 |#######################
   5266.6 |########################################
   5269.8 |###########
   5272.9 |########
   5276.1 |#######
   5279.2 |##
   5282.4 |###
   5285.5 |##
   5288.7 |###
   5291.8 |#
   5295.0 |##
   5298.1 |
   5301.3 |###
   5304.4 |
   5307.5 |#
   5310.7 |
   5313.8 |##
   5317.0 |
  (11 below, 12 above range)

```
