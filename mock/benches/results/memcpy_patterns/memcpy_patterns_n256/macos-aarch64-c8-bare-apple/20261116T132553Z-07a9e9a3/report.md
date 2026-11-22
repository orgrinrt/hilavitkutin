# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (19 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader memcpy_naive vs stability leader memcpy_unrolled (+1% speed for 3.0x steadier)

memcpy_naive is fastest (461 ns, CV 4.1%); memcpy_unrolled gives up 0.8% median for 3.0x lower variance (CV 1.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.8% of the fastest

All 3 variants sit between 461 ns and 465 ns - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (-3 ns, 0.58%)

memcpy_naive differs from baseline memcpy_intrinsic by -3 ns (0.58%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_naive** at 460.8 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 460.8 ns, slowest 464.6 ns)

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
| memcpy_intrinsic | 2580ns | 2562ns | 2553ns | 2564ns | 2658ns | base |
| memcpy_naive | 2573ns | 2562ns | 2549ns | 2563ns | 2630ns | -0.27% |
| memcpy_unrolled | 2567ns | 2564ns | 2556ns | 2564ns | 2585ns | -0.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 466ns | 452ns | 485ns | base | 0.550 |
| memcpy_naive | 463ns | 453ns | 478ns | -0.62% | 0.553 |
| memcpy_unrolled | 464ns | 455ns | 473ns | -0.36% | 0.552 |

## Performance model

- Peak throughput: **0.566 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.553 | 97.6% |
| memcpy_naive | 0.556 | 98.1% |
| memcpy_unrolled | 0.551 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2580ns | 2580ns | base |
| memcpy_naive | 2573ns | 2573ns | -0.27% |
| memcpy_unrolled | 2567ns | 2567ns | -0.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 463ns | base | --- | [462, 465] | --- | --- | --- | --- |
| memcpy_naive | 461ns | -2.7ns (-0.6%) | [-4, -1]ns | [459, 462] | YES | 0.0001 | 0.0001 | 1 |
| memcpy_unrolled | 465ns | no significant difference | [-1, +1]ns | [463, 466] | no | 0.9362 | 0.9362 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 459ns | +0.9% | +1.4% |
| 1 | 2 | 461ns | +0.2% | -0.5% |
| 1 | 3 | 488ns | -4.8% | -5.8% |
| 1 | 4 | 459ns | +0.5% | -0.2% |
| 1 | 5 | 453ns | +0.8% | +3.6% |
| 1 | 6 | 468ns | -2.1% | +0.0% |
| 1 | 7 | 464ns | -1.5% | +0.3% |
| 1 | 8 | 469ns | -0.7% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.221 | moderate- |
| memcpy_naive | 0.115 | ok |
| memcpy_unrolled | 0.280 | moderate+ |

**Consistency summary:**

- **memcpy_naive**: won 4/8, lost 4/8
- **memcpy_unrolled**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 4.1ns | 465.8ns | 0.9% |  |
| memcpy_naive | 4.2ns | 462.9ns | 0.9% |  |
| memcpy_unrolled | 4.1ns | 464.1ns | 0.9% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 452.1-484.6 ns)
    452.1 |#################
    453.8 |###########
    455.4 |########
    457.0 |######################
    458.6 |######################
    460.2 |############################
    461.9 |########################################
    463.5 |####################
    465.1 |#################
    466.7 |#############
    468.4 |########################
    470.0 |###############
    471.6 |###########
    473.2 |###########
    474.9 |########
    476.5 |####
    478.1 |
    479.7 |
    481.4 |####
    483.0 |######
  (15 below, 15 above range)

memcpy_naive (n=160, range 452.9-478.5 ns)
    452.9 |#########
    454.2 |#########################
    455.5 |#########################
    456.7 |##############################
    458.0 |#####################################
    459.3 |##################
    460.6 |################################
    461.9 |#####################
    463.1 |##################
    464.4 |########################################
    465.7 |#########################
    467.0 |#####################
    468.3 |#########
    469.5 |#########
    470.8 |##
    472.1 |
    473.4 |###########
    474.7 |
    475.9 |
    477.2 |
  (13 below, 2 above range)

memcpy_unrolled (n=160, range 455.1-473.0 ns)
    455.1 |##########
    456.0 |########
    456.9 |########
    457.8 |########################################
    458.7 |#####
    459.6 |##########################
    460.5 |#############
    461.4 |##########
    462.3 |########################
    463.2 |##########################
    464.1 |##########
    465.0 |##################################
    465.9 |#############################
    466.8 |##################
    467.7 |#####################################
    468.5 |#####################
    469.4 |########
    470.3 |##
    471.2 |##
    472.1 |########
  (13 below, 17 above range)

```
