# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_trait shows alternating (throttle bounce) (autocorr -0.74)

dispatch_trait's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (10 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_trait is inconsistent: worst-20% is 2.1x its best-20%

dispatch_trait's best 20% of batches run at 7 ns but its worst 20% at 15 ns (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 10.4 ns median
- Spread: 1.04x (fastest 10.4 ns, slowest 10.8 ns)

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
| dispatch_const_ptr | 2107ns | 2094ns | 2084ns | 2099ns | 2156ns | base |
| dispatch_direct | 2115ns | 2094ns | 2087ns | 2100ns | 2190ns | +0.37% |
| dispatch_trait | 2127ns | 2127ns | 2087ns | 2121ns | 2183ns | +0.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 11ns | 8ns | 14ns | base | 23.832 |
| dispatch_direct | 11ns | 8ns | 15ns | +4.62% | 22.780 |
| dispatch_trait | 11ns | 7ns | 15ns | +0.96% | 23.605 |

## Performance model

- Peak throughput: **36.088 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 24.615 | 68.2% |
| dispatch_direct | 23.704 | 65.7% |
| dispatch_trait | 23.704 | 65.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2107ns | 2107ns | base |
| dispatch_direct | 2115ns | 2115ns | +0.37% |
| dispatch_trait | 2127ns | 2127ns | +0.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 10ns | base | --- | [10, 11] | --- | --- | --- | --- |
| dispatch_direct | 11ns | no significant difference | [+0, +1]ns | [10, 11] | no | 0.0918 | 0.0459 | 15 |
| dispatch_trait | 11ns | no significant difference | [-0, +0]ns | [10, 11] | no | 0.8078 | 0.8078 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 10ns | +4.0% | +12.5% |
| 1 | 2 | 10ns | +6.0% | +2.0% |
| 1 | 3 | 11ns | -5.7% | +5.7% |
| 1 | 4 | 11ns | -2.3% | -9.5% |
| 1 | 5 | 10ns | +32.7% | +7.7% |
| 1 | 6 | 11ns | -1.9% | +1.9% |
| 1 | 7 | 11ns | -3.8% | -1.9% |
| 1 | 8 | 10ns | +3.8% | +3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.238 | moderate+ |
| dispatch_direct | -0.078 | ok |
| dispatch_trait | -0.744 | HIGH- (thermal bounce) |

**Consistency summary:**

- **dispatch_direct**: won 4/8, lost 4/8
- **dispatch_trait**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.9ns | 10.7ns | 26.6% | HIGH |
| dispatch_direct | 2.9ns | 11.2ns | 25.5% | HIGH |
| dispatch_trait | 2.9ns | 10.8ns | 26.9% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 7.6-14.3 ns)
      7.6 |##############
      8.0 |#########
      8.3 |
      8.6 |#####################
      9.0 |##############################
      9.3 |############################
      9.7 |
     10.0 |##############################
     10.3 |########################################
     10.7 |################
     11.0 |##################
     11.3 |
     11.7 |#####################
     12.0 |################
     12.3 |###########
     12.7 |################
     13.0 |##################
     13.3 |
     13.7 |###########
     14.0 |#########
  (15 below, 11 above range)

dispatch_direct (n=160, range 7.7-15.4 ns)
      7.7 |##################
      8.1 |#####################
      8.5 |########################
      8.9 |##############################
      9.3 |###########################
      9.6 |##################
     10.0 |########################################
     10.4 |
     10.8 |#################################
     11.2 |#################################
     11.6 |########################
     11.9 |#########
     12.3 |#####################
     12.7 |##################
     13.1 |##################
     13.5 |############
     13.9 |###############
     14.3 |###############
     14.6 |#####################
     15.0 |##################
  (14 below, 8 above range)

dispatch_trait (n=160, range 7.1-14.8 ns)
      7.1 |##########
      7.5 |##############################
      7.9 |##########################
      8.3 |##########################
      8.6 |#######################
      9.0 |################
      9.4 |################
      9.8 |####################################
     10.2 |#############
     10.6 |####################################
     10.9 |########################################
     11.3 |#######################
     11.7 |########################################
     12.1 |
     12.5 |################
     12.9 |#############
     13.3 |#######################
     13.6 |################
     14.0 |##########
     14.4 |#################################
  (12 below, 12 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=22.3% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: bridge=27.9% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=24.5% (high variance, measurements may be unstable)
- **dispatch_direct**: bridge=26.9% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=25.3% (high variance, measurements may be unstable)
- **dispatch_trait**: bridge=26.9% of algo (FFI overhead may distort results)
