# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 3 variants sit between 474 ns and 476 ns - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (-3 ns, 0.58%)

memcpy_naive differs from baseline memcpy_intrinsic by -3 ns (0.58%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_naive** at 474.2 ns median (-0.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 474.2 ns, slowest 475.8 ns)

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
| memcpy_intrinsic | 2606ns | 2602ns | 2592ns | 2603ns | 2632ns | base |
| memcpy_naive | 2604ns | 2600ns | 2587ns | 2601ns | 2632ns | -0.09% |
| memcpy_unrolled | 2613ns | 2607ns | 2590ns | 2608ns | 2653ns | +0.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 476ns | 465ns | 487ns | base | 0.538 |
| memcpy_naive | 474ns | 463ns | 485ns | -0.37% | 0.540 |
| memcpy_unrolled | 476ns | 463ns | 493ns | +0.16% | 0.538 |

## Performance model

- Peak throughput: **0.553 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.539 | 97.4% |
| memcpy_naive | 0.540 | 97.6% |
| memcpy_unrolled | 0.538 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2606ns | 2606ns | base |
| memcpy_naive | 2604ns | 2604ns | -0.09% |
| memcpy_unrolled | 2613ns | 2613ns | +0.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 475ns | base | --- | [474, 477] | --- | --- | --- | --- |
| memcpy_naive | 474ns | -2.8ns (-0.6%) | [-5, -0]ns | [472, 475] | YES | 0.0415 | 0.0208 | 2 |
| memcpy_unrolled | 476ns | no significant difference | [-2, +1]ns | [473, 476] | no | 0.8103 | 0.8103 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 479ns | -2.6% | +0.5% |
| 1 | 2 | 473ns | -1.4% | -2.0% |
| 1 | 3 | 482ns | -1.2% | -2.1% |
| 1 | 4 | 475ns | -0.1% | +1.9% |
| 1 | 5 | 473ns | +0.5% | -0.1% |
| 1 | 6 | 471ns | +2.3% | +0.9% |
| 1 | 7 | 478ns | -0.7% | -1.2% |
| 1 | 8 | 469ns | +0.9% | +1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.333 | moderate- |
| memcpy_naive | 0.298 | moderate+ |
| memcpy_unrolled | -0.347 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 5/8, lost 3/8
- **memcpy_unrolled**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.5ns | 475.5ns | 0.7% |  |
| memcpy_naive | 3.5ns | 473.7ns | 0.7% |  |
| memcpy_unrolled | 3.7ns | 476.2ns | 0.8% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 465.5-486.8 ns)
    465.5 |##########
    466.6 |####################
    467.6 |###############
    468.7 |##############################
    469.8 |##########
    470.8 |#######
    471.9 |################################
    473.0 |###############
    474.0 |################################
    475.1 |##
    476.2 |###########################
    477.2 |######################
    478.3 |########################################
    479.4 |######################
    480.4 |##########
    481.5 |#################
    482.6 |#######
    483.6 |##########
    484.7 |##
    485.8 |#####
  (13 below, 11 above range)

memcpy_naive (n=160, range 462.9-485.4 ns)
    462.9 |############
    464.0 |############
    465.1 |#####################
    466.3 |###########################
    467.4 |########################
    468.5 |#####################
    469.6 |##################
    470.8 |#####################
    471.9 |##################
    473.0 |#####################
    474.1 |####################################
    475.3 |########################################
    476.4 |####################################
    477.5 |#########
    478.7 |###########################
    479.8 |######
    480.9 |#####################
    482.0 |##############################
    483.2 |######
    484.3 |#########
  (14 below, 8 above range)

memcpy_unrolled (n=160, range 462.8-493.5 ns)
    462.8 |########
    464.3 |############
    465.9 |##########
    467.4 |################
    468.9 |##################
    470.5 |################
    472.0 |########################################
    473.5 |##
    475.1 |#####################################
    476.6 |############
    478.1 |########################################
    479.7 |################
    481.2 |############
    482.7 |#####################
    484.3 |##############
    485.8 |########
    487.3 |
    488.9 |##
    490.4 |##
    491.9 |######
  (15 below, 2 above range)

```
