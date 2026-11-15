# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 0 ns to slowest 0 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### layout_aos is fastest but the noisiest (CV 115.3%)

layout_aos wins on median (0 ns) yet has the highest variance (CV 115.3%), while layout_soa is the steadiest (CV 110.0%, 0 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (0 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### layout_soa's comparison is tie-heavy (29% tied pairs)

29% of paired samples for layout_soa are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader layout_aos vs stability leader layout_soa (+0% speed for 1.0x steadier)

layout_aos is fastest (0 ns, CV 115.3%); layout_soa gives up 0.0% median for 1.0x lower variance (CV 110.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (layout_aos) is the fastest** at 0.4 ns median
- Spread: 1.00x (fastest 0.4 ns, slowest 0.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | AoS vs SoA (column-store) layout under partial-field iteration |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eadbeef01234567 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2220ns | 2152ns | 2075ns | 2203ns | 2416ns | base |
| layout_soa | 2330ns | 2401ns | 2107ns | 2372ns | 2424ns | +4.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 0ns | 0ns | 1ns | base | 128.805 |
| layout_soa | 0ns | 0ns | 1ns | -3.65% | 133.681 |

## Performance model

- Peak throughput: **0.000 Gops/s** (; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 160.000 | 0.0% |
| layout_soa | 160.000 | 0.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2220ns | 2220ns | base |
| layout_soa | 2330ns | 2330ns | +4.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| layout_soa | 0ns | no significant difference | [+0, +0]ns | [0, 0] | no | 0.5123 | 0.5123 | **46** (29%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 0ns | +100.0% |
| 1 | 2 | 0ns | +100.0% |
| 1 | 3 | 1ns | -33.3% |
| 1 | 4 | 0ns | -100.0% |
| 1 | 5 | 0ns | +0.0% |
| 1 | 6 | 1ns | +0.0% |
| 1 | 7 | 0ns | +0.0% |
| 1 | 8 | 0ns | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.284 | moderate- |
| layout_soa | -0.005 | ok |

**Consistency summary:**

- **layout_soa**: won 2/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 2.9ns | 0.5ns | 577.1% | HIGH |
| layout_soa | 3.1ns | 0.5ns | 652.6% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 0.0-1.2 ns)
      0.0 |#####################################
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.4 |########################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |#########################
      0.8 |
      0.9 |
      1.0 |
      1.0 |
      1.1 |
      1.1 |
  (0 below, 23 above range)

layout_soa (n=160, range 0.0-1.1 ns)
      0.0 |############################
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |########################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |###################
      0.8 |
      0.9 |
      0.9 |
      1.0 |
      1.0 |
      1.1 |
  (0 below, 19 above range)

```

## Diagnostics

- **layout_aos**: CV=92.8% (high variance, measurements may be unstable)
- **layout_aos**: bridge=725.0% of algo (FFI overhead may distort results)
- **layout_soa**: CV=91.9% (high variance, measurements may be unstable)
- **layout_soa**: bridge=725.0% of algo (FFI overhead may distort results)
