# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_trait is fastest but the noisiest (CV 53.1%)

dispatch_trait wins on median (2 ns) yet has the highest variance (CV 53.1%), while dispatch_direct is the steadiest (CV 47.9%, 2 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (1 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_trait's comparison is tie-heavy (16% tied pairs)

16% of paired samples for dispatch_trait are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### dispatch_direct is inconsistent: worst-20% is 5.1x its best-20%

dispatch_direct's best 20% of batches run at 1 ns but its worst 20% at 3 ns (5.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dispatch_trait** at 1.7 ns median (-10.5% vs baseline)
- Spread: 1.24x (fastest 1.7 ns, slowest 2.1 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2096ns | 2092ns | 2087ns | 2093ns | 2116ns | base |
| dispatch_direct | 2095ns | 2093ns | 2088ns | 2093ns | 2106ns | -0.07% |
| dispatch_trait | 2102ns | 2094ns | 2090ns | 2094ns | 2134ns | +0.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 2ns | 1ns | 4ns | base | 32.477 |
| dispatch_direct | 2ns | 1ns | 3ns | +2.00% | 31.841 |
| dispatch_trait | 2ns | 1ns | 3ns | -4.69% | 34.077 |

## Performance model

- Peak throughput: **93.091 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 33.684 | 36.2% |
| dispatch_direct | 30.476 | 32.7% |
| dispatch_trait | 37.647 | 40.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2096ns | 2096ns | base |
| dispatch_direct | 2095ns | 2095ns | -0.07% |
| dispatch_trait | 2102ns | 2102ns | +0.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2ns | base | --- | [2, 2] | --- | --- | --- | --- |
| dispatch_direct | 2ns | no significant difference | [+0, +0]ns | [2, 2] | no | 0.4913 | 0.4470 | **20** (12%, HIGH) |
| dispatch_trait | 2ns | no significant difference | [-0, +0]ns | [2, 2] | no | 0.4913 | 0.4913 | **25** (16%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 2ns | +10.5% | +0.0% |
| 1 | 2 | 1ns | +108.3% | +75.0% |
| 1 | 3 | 1ns | +0.0% | +44.8% |
| 1 | 4 | 2ns | -19.0% | -19.0% |
| 1 | 5 | 2ns | -9.5% | +0.0% |
| 1 | 6 | 1ns | +75.0% | +0.0% |
| 1 | 7 | 2ns | +9.5% | -19.0% |
| 1 | 8 | 2ns | -26.1% | -17.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.058 | ok |
| dispatch_direct | -0.121 | ok |
| dispatch_trait | -0.114 | ok |

**Consistency summary:**

- **dispatch_direct**: won 3/8, lost 4/8
- **dispatch_trait**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.5ns | 2.0ns | 126.2% | HIGH |
| dispatch_direct | 2.5ns | 2.0ns | 122.5% | HIGH |
| dispatch_trait | 2.6ns | 1.9ns | 137.1% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 0.8-3.5 ns)
      0.8 |
      0.9 |
      1.1 |########################################
      1.2 |
      1.4 |
      1.5 |
      1.6 |#########################
      1.8 |
      1.9 |
      2.0 |####################################
      2.2 |
      2.3 |
      2.4 |##################
      2.6 |
      2.7 |
      2.8 |###########
      3.0 |
      3.1 |
      3.3 |###############
      3.4 |
  (24 below, 10 above range)

dispatch_direct (n=160, range 0.7-3.5 ns)
      0.7 |##############################
      0.8 |
      1.0 |
      1.1 |##############################
      1.2 |
      1.4 |
      1.5 |
      1.7 |########################################
      1.8 |
      2.0 |
      2.1 |###################################
      2.2 |
      2.4 |########################################
      2.5 |
      2.7 |
      2.8 |####################
      2.9 |
      3.1 |
      3.2 |############
      3.4 |
  (10 below, 14 above range)

dispatch_trait (n=160, range 0.7-3.2 ns)
      0.7 |########################
      0.8 |
      0.9 |
      1.1 |
      1.2 |########################################
      1.3 |
      1.4 |
      1.6 |
      1.7 |##############################
      1.8 |
      1.9 |
      2.1 |########################################
      2.2 |
      2.3 |
      2.4 |##########################
      2.6 |
      2.7 |
      2.8 |#################
      2.9 |
      3.1 |
  (10 below, 16 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=50.7% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **dispatch_const_ptr**: bridge=131.6% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=50.0% (high variance, measurements may be unstable)
- **dispatch_direct**: worst_20/best_20 = 5.1x (possible bimodal distribution)
- **dispatch_direct**: bridge=119.0% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=48.1% (high variance, measurements may be unstable)
- **dispatch_trait**: worst_20/best_20 = 4.6x (possible bimodal distribution)
- **dispatch_trait**: bridge=147.1% of algo (FFI overhead may distort results)
