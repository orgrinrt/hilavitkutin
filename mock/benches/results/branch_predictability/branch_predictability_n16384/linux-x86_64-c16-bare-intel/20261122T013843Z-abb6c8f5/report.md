# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (1.61 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bp_skew_low vs stability leader bp_skew_high (+0% speed for 1.5x steadier)

bp_skew_low is fastest (18.27 us, CV 8.8%); bp_skew_high gives up 0.0% median for 1.5x lower variance (CV 6.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.0% of the fastest

All 3 variants sit between 18.27 us and 18.27 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_high's edge over baseline is significant but tiny (1 ns, 0.00%)

bp_skew_high differs from baseline bp_balanced by 1 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_low** at 18267.2 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 18267.2 ns, slowest 18268.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x1b41e7c35fa28d06 |
| Counter frequency | 2112577410 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 19624ns | 20152ns | 16483ns | 19167ns | 24136ns | base |
| bp_skew_high | 19705ns | 20155ns | 17865ns | 20155ns | 20194ns | +0.41% |
| bp_skew_low | 18788ns | 20154ns | 16487ns | 19092ns | 20179ns | -4.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 17783ns | 14939ns | 21865ns | base | 0.921 |
| bp_skew_high | 17853ns | 16185ns | 18281ns | +0.40% | 0.918 |
| bp_skew_low | 17021ns | 14939ns | 18271ns | -4.28% | 0.963 |

## Performance model

- Peak throughput: **1.097 Gops/s** (bp_balanced; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.897 | 81.8% |
| bp_skew_high | 0.897 | 81.8% |
| bp_skew_low | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 19624ns | 19624ns | base |
| bp_skew_high | 19705ns | 19705ns | +0.41% |
| bp_skew_low | 18788ns | 18788ns | -4.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 18268ns | base | --- | [18258, 18268] | --- | --- | --- | --- |
| bp_skew_high | 18268ns | +0.6ns (+0.0%) | [+0, +2]ns | [18268, 18268] | YES (adj: no) | 0.0778 | 0.0389 | 1 |
| bp_skew_low | 18267ns | no significant difference | [-1, +1]ns | [18257, 18268] | no | 1.0000 | 1.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 18268ns | +0.0% | +0.0% |
| 1 | 2 | 18269ns | -0.0% | -18.2% |
| 1 | 3 | 18269ns | -0.0% | -18.2% |
| 1 | 4 | 14947ns | +22.2% | +22.2% |
| 1 | 5 | 18268ns | -0.0% | -0.0% |
| 1 | 6 | 14947ns | +22.2% | +22.2% |
| 1 | 7 | 18268ns | -0.0% | -0.0% |
| 1 | 8 | 14942ns | -0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.475 | moderate- |
| bp_skew_high | -0.018 | ok |
| bp_skew_low | 0.058 | ok |

**Consistency summary:**

- **bp_skew_high**: won 0/8, lost 2/8
- **bp_skew_low**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 23.4ns | 17782.9ns | 0.1% |  |
| bp_skew_high | 23.5ns | 17853.5ns | 0.1% |  |
| bp_skew_low | 22.4ns | 17020.9ns | 0.1% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 14939.1-21864.8 ns)
  14939.1 |#############
  15285.4 |
  15631.7 |
  15978.0 |
  16324.2 |
  16670.5 |
  17016.8 |
  17363.1 |
  17709.4 |
  18055.7 |########################################
  18401.9 |
  18748.2 |
  19094.5 |
  19440.8 |
  19787.1 |
  20133.4 |
  20479.6 |
  20825.9 |
  21172.2 |
  21518.5 |
  (25 below, 2 above range)

bp_skew_high (n=160, range 16185.3-18280.9 ns)
  16185.3 |
  16290.1 |
  16394.8 |
  16499.6 |
  16604.4 |
  16709.2 |
  16814.0 |
  16918.7 |
  17023.5 |
  17128.3 |
  17233.1 |
  17337.9 |
  17442.7 |
  17547.4 |
  17652.2 |
  17757.0 |
  17861.8 |
  17966.6 |
  18071.3 |
  18176.1 |########################################
  (20 below, 3 above range)

bp_skew_low (n=160, range 14939.2-18270.7 ns)
  14939.2 |################
  15105.8 |
  15272.4 |
  15439.0 |
  15605.5 |
  15772.1 |
  15938.7 |
  16105.3 |
  16271.8 |
  16438.4 |
  16605.0 |
  16771.6 |
  16938.1 |
  17104.7 |
  17271.3 |
  17437.9 |
  17604.4 |
  17771.0 |
  17937.6 |
  18104.2 |########################################
  (22 below, 8 above range)

```

## Diagnostics

- **bp_balanced**: CV=40.6% (high variance, measurements may be unstable)
