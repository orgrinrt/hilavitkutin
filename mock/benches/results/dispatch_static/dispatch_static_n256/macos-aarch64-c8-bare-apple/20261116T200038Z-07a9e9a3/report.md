# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 15 ns to slowest 15 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (15 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_direct's comparison is tie-heavy (11% tied pairs)

11% of paired samples for dispatch_direct are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_direct (+0% speed for 1.0x steadier)

dispatch_const_ptr is fastest (15 ns, CV 16.7%); dispatch_direct gives up 0.0% median for 1.0x lower variance (CV 16.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_trait is inconsistent: worst-20% is 1.6x its best-20%

dispatch_trait's best 20% of batches run at 11 ns but its worst 20% at 18 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 14.6 ns median
- Spread: 1.00x (fastest 14.6 ns, slowest 14.6 ns)

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
| dispatch_const_ptr | 2131ns | 2139ns | 2100ns | 2130ns | 2169ns | base |
| dispatch_direct | 2126ns | 2134ns | 2100ns | 2125ns | 2154ns | -0.26% |
| dispatch_trait | 2133ns | 2114ns | 2099ns | 2124ns | 2194ns | +0.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 15ns | 11ns | 18ns | base | 17.444 |
| dispatch_direct | 15ns | 11ns | 18ns | -1.09% | 17.637 |
| dispatch_trait | 14ns | 11ns | 18ns | -1.21% | 17.657 |

## Performance model

- Peak throughput: **23.044 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 17.534 | 76.1% |
| dispatch_direct | 17.534 | 76.1% |
| dispatch_trait | 17.534 | 76.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2131ns | 2131ns | base |
| dispatch_direct | 2126ns | 2126ns | -0.26% |
| dispatch_trait | 2133ns | 2133ns | +0.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 15ns | base | --- | [14, 15] | --- | --- | --- | --- |
| dispatch_direct | 15ns | no significant difference | [-0, +0]ns | [14, 15] | no | 0.6160 | 0.6160 | **17** (11%, HIGH) |
| dispatch_trait | 15ns | no significant difference | [-0, +0]ns | [14, 15] | no | 0.5639 | 0.2819 | 14 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 15ns | -5.4% | +0.0% |
| 1 | 2 | 15ns | +1.4% | -4.1% |
| 1 | 3 | 14ns | +4.2% | +1.4% |
| 1 | 4 | 15ns | -3.9% | -6.6% |
| 1 | 5 | 14ns | +5.8% | +4.3% |
| 1 | 6 | 15ns | -2.7% | -1.4% |
| 1 | 7 | 16ns | -15.8% | -7.6% |
| 1 | 8 | 16ns | -7.7% | -7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.091 | ok |
| dispatch_direct | 0.163 | ok |
| dispatch_trait | -0.400 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 5/8, lost 3/8
- **dispatch_trait**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 4.2ns | 14.7ns | 28.5% | HIGH |
| dispatch_direct | 4.1ns | 14.5ns | 28.2% | HIGH |
| dispatch_trait | 4.0ns | 14.5ns | 27.7% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 11.3-18.1 ns)
     11.3 |
     11.7 |##########
     12.0 |#################
     12.4 |#################
     12.7 |#########################
     13.0 |###################################
     13.4 |
     13.7 |######################
     14.1 |###############
     14.4 |####################
     14.7 |###########################
     15.1 |###############
     15.4 |
     15.8 |########################################
     16.1 |##############################
     16.4 |############
     16.8 |#################
     17.1 |
     17.5 |##########
     17.8 |############
  (16 below, 13 above range)

dispatch_direct (n=160, range 11.3-18.0 ns)
     11.3 |
     11.6 |##########
     11.9 |###############
     12.3 |#################
     12.6 |##############################
     13.0 |
     13.3 |######################
     13.6 |#########################
     14.0 |################################
     14.3 |########################################
     14.6 |
     15.0 |######################
     15.3 |###############
     15.6 |################################
     16.0 |###############
     16.3 |
     16.7 |#################
     17.0 |############
     17.3 |##
     17.7 |####################
  (17 below, 11 above range)

dispatch_trait (n=160, range 11.1-18.1 ns)
     11.1 |######
     11.5 |################
     11.8 |########################################
     12.2 |#################################
     12.5 |
     12.9 |########################################
     13.2 |##########
     13.6 |#################################
     13.9 |####################################
     14.3 |########################################
     14.6 |
     15.0 |####################################
     15.3 |##########
     15.7 |##########################
     16.0 |#################################
     16.4 |##############################
     16.7 |
     17.1 |####################
     17.4 |####################
     17.8 |##########
  (14 below, 13 above range)

```

## Diagnostics

- **dispatch_const_ptr**: bridge=28.8% of algo (FFI overhead may distort results)
- **dispatch_direct**: bridge=25.3% of algo (FFI overhead may distort results)
- **dispatch_trait**: bridge=28.8% of algo (FFI overhead may distort results)
