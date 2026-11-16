# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (27 ns) is smaller than the fastest variant's own run-to-run std-dev (139 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bp_balanced)

The baseline bp_balanced is the fastest (20.52 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.1% of the fastest

All 3 variants sit between 20.52 us and 20.55 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_low's edge over baseline is significant but tiny (6 ns, 0.03%)

bp_skew_low differs from baseline bp_balanced by 6 ns (0.03%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (bp_balanced) is the fastest** at 20518.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 20518.8 ns, slowest 20545.8 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 22658ns | 22622ns | 22583ns | 22620ns | 22847ns | base |
| bp_skew_high | 22784ns | 22633ns | 22583ns | 22637ns | 23428ns | +0.56% |
| bp_skew_low | 22764ns | 22640ns | 22585ns | 22645ns | 23300ns | +0.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 20563ns | 20498ns | 20734ns | base | 0.797 |
| bp_skew_high | 20678ns | 20500ns | 21263ns | +0.56% | 0.792 |
| bp_skew_low | 20657ns | 20500ns | 21142ns | +0.46% | 0.793 |

## Performance model

- Peak throughput: **0.799 Gops/s** (bp_balanced; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.798 | 99.9% |
| bp_skew_high | 0.798 | 99.8% |
| bp_skew_low | 0.797 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 22658ns | 22658ns | base |
| bp_skew_high | 22784ns | 22784ns | +0.56% |
| bp_skew_low | 22764ns | 22764ns | +0.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 20519ns | base | --- | [20509, 20531] | --- | --- | --- | --- |
| bp_skew_high | 20542ns | no significant difference | [-1, +8]ns | [20512, 20555] | no | 0.2668 | 0.2668 | 1 |
| bp_skew_low | 20546ns | +5.6ns (+0.0%) | [+0, +19]ns | [20523, 20561] | YES (adj: no) | 0.0521 | 0.0261 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 20512ns | +0.1% | -0.0% |
| 1 | 2 | 20509ns | +0.1% | +0.0% |
| 1 | 3 | 20511ns | +0.1% | +1.4% |
| 1 | 4 | 20516ns | -0.1% | +0.3% |
| 1 | 5 | 20542ns | -0.0% | -0.1% |
| 1 | 6 | 20521ns | +4.8% | +0.2% |
| 1 | 7 | 20538ns | -0.1% | -0.0% |
| 1 | 8 | 20526ns | -0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | 0.246 | moderate+ |
| bp_skew_high | -0.159 | ok |
| bp_skew_low | -0.076 | ok |

**Consistency summary:**

- **bp_skew_high**: won 1/8, lost 2/8
- **bp_skew_low**: won 0/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 2.6ns | 20562.7ns | 0.0% |  |
| bp_skew_high | 2.9ns | 20678.2ns | 0.0% |  |
| bp_skew_low | 2.7ns | 20657.4ns | 0.0% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 20498.1-20734.1 ns)
  20498.1 |########################################
  20509.9 |######
  20521.7 |########
  20533.5 |##
  20545.3 |##########
  20557.1 |########
  20568.9 |####
  20580.7 |##
  20592.5 |####
  20604.3 |###
  20616.1 |#
  20627.9 |
  20639.7 |
  20651.5 |
  20663.3 |
  20675.1 |
  20686.9 |
  20698.7 |
  20710.5 |
  20722.3 |#
  (12 below, 8 above range)

bp_skew_high (n=160, range 20500.0-21262.9 ns)
  20500.0 |########################################
  20538.2 |######################
  20576.3 |######
  20614.5 |####
  20652.6 |#
  20690.7 |
  20728.9 |#
  20767.0 |#
  20805.2 |
  20843.3 |
  20881.5 |
  20919.6 |
  20957.7 |
  20995.9 |
  21034.0 |
  21072.2 |
  21110.3 |
  21148.5 |
  21186.6 |
  21224.8 |#
  (15 below, 19 above range)

bp_skew_low (n=160, range 20500.1-21142.0 ns)
  20500.1 |########################################
  20532.2 |############
  20564.3 |###############
  20596.4 |######
  20628.5 |##
  20660.6 |#
  20692.7 |#
  20724.8 |##
  20756.9 |
  20789.0 |
  20821.0 |
  20853.1 |
  20885.2 |
  20917.3 |
  20949.4 |
  20981.5 |
  21013.6 |#
  21045.7 |
  21077.8 |
  21109.9 |
  (16 below, 17 above range)

```
