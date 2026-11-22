# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bp_skew_high is fastest but the noisiest (CV 9.2%)

bp_skew_high wins on median (18.04 us) yet has the highest variance (CV 9.2%), while bp_balanced is the steadiest (CV 6.0%, 18.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (229 ns) is smaller than the fastest variant's own run-to-run std-dev (1.66 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bp_skew_high vs stability leader bp_balanced (+1% speed for 1.5x steadier)

bp_skew_high is fastest (18.04 us, CV 9.2%); bp_balanced gives up 1.3% median for 1.5x lower variance (CV 6.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.3% of the fastest

All 3 variants sit between 18.04 us and 18.27 us - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_high's edge over baseline is significant but tiny (-8 ns, 0.04%)

bp_skew_high differs from baseline bp_balanced by -8 ns (0.04%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_high** at 18039.5 ns median (-1.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 18039.5 ns, slowest 18268.9 ns)

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
| Counter frequency | 2112755873 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 19708ns | 20156ns | 17867ns | 20156ns | 20205ns | base |
| bp_skew_high | 18412ns | 19906ns | 16485ns | 18434ns | 20274ns | -6.58% |
| bp_skew_low | 19275ns | 20158ns | 16489ns | 19855ns | 20320ns | -2.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 17857ns | 16189ns | 18292ns | base | 0.918 |
| bp_skew_high | 16682ns | 14937ns | 18364ns | -6.57% | 0.982 |
| bp_skew_low | 17464ns | 14942ns | 18409ns | -2.20% | 0.938 |

## Performance model

- Peak throughput: **1.097 Gops/s** (bp_skew_high; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.897 | 81.8% |
| bp_skew_high | 0.908 | 82.8% |
| bp_skew_low | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 19708ns | 19708ns | base |
| bp_skew_high | 18412ns | 18412ns | -6.58% |
| bp_skew_low | 19275ns | 19275ns | -2.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 18269ns | base | --- | [18268, 18270] | --- | --- | --- | --- |
| bp_skew_high | 18040ns | -7.8ns (-0.0%) | [-13, -1]ns | [14952, 18259] | YES | 0.0342 | 0.0171 | 1 |
| bp_skew_low | 18269ns | no significant difference | [-1, +1]ns | [18268, 18269] | no | 0.8740 | 0.8740 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 18270ns | -18.2% | -0.0% |
| 1 | 2 | 18271ns | -0.0% | -18.2% |
| 1 | 3 | 18269ns | -18.2% | +0.0% |
| 1 | 4 | 14948ns | +0.0% | -0.0% |
| 1 | 5 | 18269ns | -0.0% | +0.0% |
| 1 | 6 | 18269ns | -18.2% | -0.0% |
| 1 | 7 | 18269ns | +0.0% | +0.0% |
| 1 | 8 | 18269ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.160 | ok |
| bp_skew_high | -0.375 | moderate- |
| bp_skew_low | -0.375 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 3/8, lost 0/8
- **bp_skew_low**: won 1/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 23.4ns | 17856.5ns | 0.1% |  |
| bp_skew_high | 21.9ns | 16682.5ns | 0.1% |  |
| bp_skew_low | 22.8ns | 17464.2ns | 0.1% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 16189.1-18291.6 ns)
  16189.1 |
  16294.3 |
  16399.4 |
  16504.5 |
  16609.6 |
  16714.8 |
  16819.9 |
  16925.0 |
  17030.1 |
  17135.3 |
  17240.4 |
  17345.5 |
  17450.6 |
  17555.8 |
  17660.9 |
  17766.0 |
  17871.1 |
  17976.3 |
  18081.4 |
  18186.5 |########################################
  (20 below, 5 above range)

bp_skew_high (n=160, range 14937.4-18364.1 ns)
  14937.4 |#####################################
  15108.8 |
  15280.1 |
  15451.4 |
  15622.8 |
  15794.1 |
  15965.4 |
  16136.8 |
  16308.1 |
  16479.4 |
  16650.8 |
  16822.1 |#
  16993.4 |
  17164.8 |
  17336.1 |
  17507.4 |
  17678.8 |
  17850.1 |
  18021.4 |
  18192.8 |########################################
  (8 below, 9 above range)

bp_skew_low (n=160, range 14941.6-18408.6 ns)
  14941.6 |#######
  15114.9 |
  15288.3 |
  15461.6 |
  15635.0 |
  15808.3 |
  15981.7 |
  16155.0 |
  16328.4 |
  16501.7 |
  16675.1 |
  16848.4 |
  17021.8 |
  17195.1 |
  17368.5 |
  17541.8 |
  17715.2 |
  17888.6 |
  18061.9 |
  18235.3 |########################################
  (17 below, 4 above range)

```
