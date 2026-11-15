# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### minmax_branch is fastest but the noisiest (CV 85.3%)

minmax_branch wins on median (185 ns) yet has the highest variance (CV 85.3%), while minmax_intrinsic is the steadiest (CV 83.1%, 188 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (158 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (185 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader minmax_branch vs stability leader minmax_intrinsic (+1% speed for 1.0x steadier)

minmax_branch is fastest (185 ns, CV 85.3%); minmax_intrinsic gives up 1.4% median for 1.0x lower variance (CV 83.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### minmax_branch is inconsistent: worst-20% is 3.5x its best-20%

minmax_branch's best 20% of batches run at 150 ns but its worst 20% at 531 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 1.4% of the fastest

All 3 variants sit between 185 ns and 188 ns - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### minmax_branchless's edge over baseline is significant but tiny (-3 ns, 1.83%)

minmax_branchless differs from baseline minmax_branch by -3 ns (1.83%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 185.4 ns median
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 185.4 ns, slowest 188.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5d49a3f1b7c06e82 |
| Counter frequency | 2112457735 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2773ns | 2067ns | 1688ns | 2190ns | 5606ns | base |
| minmax_branchless | 2909ns | 2063ns | 2045ns | 2068ns | 6296ns | +4.92% |
| minmax_intrinsic | 2822ns | 2067ns | 1853ns | 2069ns | 6050ns | +1.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 258ns | 150ns | 531ns | base | 15.881 |
| minmax_branchless | 256ns | 170ns | 555ns | -0.81% | 16.011 |
| minmax_intrinsic | 255ns | 162ns | 546ns | -1.13% | 16.063 |

## Performance model

- Peak throughput: **27.385 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 22.093 | 80.7% |
| minmax_branchless | 21.799 | 79.6% |
| minmax_intrinsic | 21.781 | 79.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2773ns | 2773ns | base |
| minmax_branchless | 2909ns | 2909ns | +4.92% |
| minmax_intrinsic | 2822ns | 2822ns | +1.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 185ns | base | --- | [184, 192] | --- | --- | --- | --- |
| minmax_branchless | 188ns | -3.4ns (-1.8%) | [-4, -2]ns | [180, 188] | YES | 0.0110 | 0.0055 | 0 |
| minmax_intrinsic | 188ns | no significant difference | [-1, +0]ns | [185, 191] | no | 0.1124 | 0.1124 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 193ns | -2.5% | -0.1% |
| 1 | 2 | 184ns | -7.9% | +0.1% |
| 1 | 3 | 152ns | +17.7% | +22.1% |
| 1 | 4 | 151ns | +19.4% | -0.2% |
| 1 | 5 | 185ns | -8.4% | -0.0% |
| 1 | 6 | 192ns | -1.7% | -0.4% |
| 1 | 7 | 183ns | -1.7% | -0.4% |
| 1 | 8 | 371ns | -49.0% | -47.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | 0.045 | ok |
| minmax_branchless | -0.358 | moderate- |
| minmax_intrinsic | -0.077 | ok |

**Consistency summary:**

- **minmax_branchless**: won 6/8, lost 2/8
- **minmax_intrinsic**: won 4/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 31.9ns | 257.9ns | 12.4% | HIGH |
| minmax_branchless | 33.6ns | 255.8ns | 13.2% | HIGH |
| minmax_intrinsic | 32.5ns | 255.0ns | 12.7% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 149.6-530.6 ns)
    149.6 |#####################
    168.6 |########################################
    187.7 |############################
    206.7 |
    225.8 |
    244.8 |
    263.9 |
    282.9 |
    302.0 |
    321.0 |
    340.1 |
    359.1 |#############
    378.2 |
    397.2 |
    416.3 |
    435.3 |
    454.4 |
    473.4 |#
    492.5 |
    511.5 |
  (5 below, 17 above range)

minmax_branchless (n=160, range 169.7-555.0 ns)
    169.7 |########################################
    189.0 |#######
    208.2 |
    227.5 |
    246.8 |
    266.0 |
    285.3 |
    304.6 |
    323.8 |
    343.1 |
    362.4 |
    381.6 |
    400.9 |
    420.2 |
    439.4 |
    458.7 |
    478.0 |
    497.2 |
    516.5 |
    535.8 |
  (29 below, 20 above range)

minmax_intrinsic (n=160, range 161.9-546.1 ns)
    161.9 |#####
    181.1 |########################################
    200.3 |
    219.5 |
    238.7 |
    257.9 |
    277.2 |
    296.4 |
    315.6 |
    334.8 |
    354.0 |
    373.2 |
    392.4 |
    411.6 |
    430.8 |
    450.0 |
    469.3 |
    488.5 |
    507.7 |
    526.9 |#
  (17 below, 20 above range)

```

## Diagnostics

- **minmax_branch**: CV=61.3% (high variance, measurements may be unstable)
- **minmax_branch**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **minmax_branch**: bridge=12.9% of algo (FFI overhead may distort results)
- **minmax_branchless**: CV=62.4% (high variance, measurements may be unstable)
- **minmax_branchless**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **minmax_branchless**: bridge=12.7% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: CV=61.3% (high variance, measurements may be unstable)
- **minmax_intrinsic**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **minmax_intrinsic**: bridge=12.7% of algo (FFI overhead may distort results)
