# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_const_ptr is fastest but the noisiest (CV 93.7%)

dispatch_const_ptr wins on median (16 ns) yet has the highest variance (CV 93.7%), while dispatch_direct is the steadiest (CV 89.2%, 16 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### dispatch_direct shows warm-up / thermal drift (autocorr +0.59)

dispatch_direct's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (15 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (16 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_direct's comparison is tie-heavy (18% tied pairs)

18% of paired samples for dispatch_direct are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_direct (+1% speed for 1.1x steadier)

dispatch_const_ptr is fastest (16 ns, CV 93.7%); dispatch_direct gives up 0.6% median for 1.1x lower variance (CV 89.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_const_ptr is inconsistent: worst-20% is 3.9x its best-20%

dispatch_const_ptr's best 20% of batches run at 13 ns but its worst 20% at 51 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.6% of the fastest

All 3 variants sit between 16 ns and 16 ns - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_trait's edge over baseline is significant but tiny (0 ns, 1.25%)

dispatch_trait differs from baseline dispatch_const_ptr by 0 ns (1.25%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 16.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 16.0 ns, slowest 16.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Static dispatch shapes: direct vs sealed-trait vs const fn pointer |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x7eedfacedeadbeef |
| Counter frequency | 2112808726 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2622ns | 1895ns | 1552ns | 1889ns | 5889ns | base |
| dispatch_direct | 2727ns | 1901ns | 1554ns | 2052ns | 5927ns | +4.01% |
| dispatch_trait | 2633ns | 1900ns | 1553ns | 1885ns | 5957ns | +0.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 23ns | 13ns | 51ns | base | 2.843 |
| dispatch_direct | 23ns | 13ns | 50ns | +2.37% | 2.777 |
| dispatch_trait | 22ns | 13ns | 50ns | -0.67% | 2.862 |

## Performance model

- Peak throughput: **4.868 Gops/s** (dispatch_const_ptr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 4.000 | 82.2% |
| dispatch_direct | 3.975 | 81.7% |
| dispatch_trait | 3.975 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2622ns | 2622ns | base |
| dispatch_direct | 2727ns | 2727ns | +4.01% |
| dispatch_trait | 2633ns | 2633ns | +0.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 16ns | base | --- | [14, 16] | --- | --- | --- | --- |
| dispatch_direct | 16ns | no significant difference | [+0, +0]ns | [16, 16] | no | 0.6003 | 0.6003 | **29** (18%, HIGH) |
| dispatch_trait | 16ns | +0.2ns (+1.3%) | [+0, +0]ns | [16, 16] | YES | 0.0003 | 0.0001 | **18** (11%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 13ns | +0.8% | +3.0% |
| 1 | 2 | 14ns | -5.0% | +16.5% |
| 1 | 3 | 13ns | +0.0% | +0.0% |
| 1 | 4 | 13ns | +22.3% | +22.3% |
| 1 | 5 | 13ns | +22.0% | +22.7% |
| 1 | 6 | 16ns | +0.3% | -0.3% |
| 1 | 7 | 16ns | +0.6% | -18.0% |
| 1 | 8 | 16ns | +0.0% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.527 | HIGH+ (drift/warm-up) |
| dispatch_direct | 0.594 | HIGH+ (drift/warm-up) |
| dispatch_trait | -0.489 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 1/8, lost 5/8
- **dispatch_trait**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 33.0ns | 22.5ns | 146.8% | HIGH |
| dispatch_direct | 34.7ns | 23.0ns | 150.7% | HIGH |
| dispatch_trait | 33.2ns | 22.4ns | 148.3% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 13.1-50.6 ns)
     13.1 |########################################
     15.0 |#################################
     16.9 |
     18.8 |
     20.6 |
     22.5 |
     24.4 |
     26.3 |
     28.1 |#
     30.0 |
     31.9 |
     33.8 |
     35.6 |
     37.5 |#
     39.4 |#
     41.3 |##
     43.1 |####
     45.0 |
     46.9 |#
     48.7 |#
  (17 below, 17 above range)

dispatch_direct (n=160, range 13.2-49.9 ns)
     13.2 |###############
     15.0 |########################################
     16.8 |
     18.7 |#
     20.5 |
     22.3 |
     24.2 |
     26.0 |
     27.8 |
     29.7 |
     31.5 |
     33.4 |
     35.2 |#
     37.0 |#
     38.9 |
     40.7 |##
     42.5 |
     44.4 |##
     46.2 |#
     48.1 |##
  (13 below, 15 above range)

dispatch_trait (n=160, range 13.2-50.5 ns)
     13.2 |####################
     15.1 |########################################
     16.9 |
     18.8 |
     20.7 |#
     22.5 |
     24.4 |
     26.2 |
     28.1 |
     30.0 |
     31.8 |
     33.7 |
     35.6 |
     37.4 |
     39.3 |#
     41.2 |#
     43.0 |#
     44.9 |#
     46.7 |##
     48.6 |
  (7 below, 16 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=66.6% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **dispatch_const_ptr**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **dispatch_const_ptr**: bridge=148.1% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=62.3% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_direct**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **dispatch_direct**: bridge=148.4% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=65.1% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **dispatch_trait**: bridge=148.1% of algo (FFI overhead may distort results)
