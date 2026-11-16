# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.8% (fastest 14 ns to slowest 15 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (14 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_trait's comparison is tie-heavy (11% tied pairs)

11% of paired samples for dispatch_trait are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_direct (+0% speed for 1.0x steadier)

dispatch_const_ptr is fastest (14 ns, CV 16.8%); dispatch_direct gives up 0.0% median for 1.0x lower variance (CV 16.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_trait is inconsistent: worst-20% is 1.6x its best-20%

dispatch_trait's best 20% of batches run at 11 ns but its worst 20% at 18 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 14.2 ns median
- Spread: 1.03x (fastest 14.2 ns, slowest 14.6 ns)

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
| dispatch_const_ptr | 2105ns | 2102ns | 2096ns | 2102ns | 2125ns | base |
| dispatch_direct | 2105ns | 2101ns | 2096ns | 2101ns | 2124ns | -0.02% |
| dispatch_trait | 2104ns | 2102ns | 2097ns | 2102ns | 2118ns | -0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 14ns | 11ns | 18ns | base | 17.906 |
| dispatch_direct | 14ns | 11ns | 18ns | +1.04% | 17.722 |
| dispatch_trait | 14ns | 11ns | 18ns | +1.30% | 17.677 |

## Performance model

- Peak throughput: **23.102 Gops/s** (dispatch_const_ptr; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 18.028 | 78.0% |
| dispatch_direct | 18.028 | 78.0% |
| dispatch_trait | 17.534 | 75.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2105ns | 2105ns | base |
| dispatch_direct | 2105ns | 2105ns | -0.02% |
| dispatch_trait | 2104ns | 2104ns | -0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 14ns | base | --- | [14, 15] | --- | --- | --- | --- |
| dispatch_direct | 14ns | no significant difference | [-0, +0]ns | [14, 15] | no | 1.0000 | 1.0000 | 13 |
| dispatch_trait | 15ns | no significant difference | [+0, +1]ns | [14, 15] | no | 0.3069 | 0.1534 | **18** (11%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 13ns | +0.0% | +1.9% |
| 1 | 2 | 14ns | -3.6% | +1.4% |
| 1 | 3 | 15ns | +1.4% | -7.2% |
| 1 | 4 | 15ns | -1.4% | +0.0% |
| 1 | 5 | 14ns | -2.8% | +7.0% |
| 1 | 6 | 14ns | +8.7% | +2.9% |
| 1 | 7 | 14ns | +3.3% | +7.6% |
| 1 | 8 | 14ns | -1.4% | -7.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.273 | moderate+ |
| dispatch_direct | 0.018 | ok |
| dispatch_trait | -0.011 | ok |

**Consistency summary:**

- **dispatch_direct**: won 4/8, lost 3/8
- **dispatch_trait**: won 2/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 4.0ns | 14.3ns | 28.2% | HIGH |
| dispatch_direct | 4.0ns | 14.4ns | 27.5% | HIGH |
| dispatch_trait | 4.0ns | 14.5ns | 27.6% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 11.1-17.8 ns)
     11.1 |##############
     11.4 |############################
     11.7 |
     12.1 |######################
     12.4 |#########################
     12.7 |########################################
     13.1 |#################
     13.4 |
     13.8 |######################
     14.1 |##################################
     14.4 |##################################
     14.8 |######################
     15.1 |#################
     15.4 |
     15.8 |######################
     16.1 |###########
     16.4 |###########
     16.8 |
     17.1 |#########################
     17.4 |#####################################
  (13 below, 11 above range)

dispatch_direct (n=160, range 11.3-17.8 ns)
     11.3 |
     11.6 |#############
     12.0 |################
     12.3 |########################
     12.6 |##################################
     12.9 |
     13.2 |########################
     13.6 |########################################
     13.9 |##########################
     14.2 |
     14.5 |################
     14.9 |################
     15.2 |#####################
     15.5 |#############################
     15.8 |
     16.1 |##########################
     16.5 |#####################
     16.8 |##################
     17.1 |
     17.4 |########
  (18 below, 16 above range)

dispatch_trait (n=160, range 11.1-18.1 ns)
     11.1 |#####
     11.5 |###############################
     11.8 |##############
     12.2 |####################
     12.5 |
     12.9 |#####################################
     13.2 |########################################
     13.6 |######################
     13.9 |##############
     14.3 |#########################
     14.6 |
     15.0 |#########################
     15.3 |############################
     15.7 |############################
     16.0 |####################
     16.4 |####################
     16.7 |
     17.1 |######################
     17.4 |##############
     17.8 |########
  (14 below, 13 above range)

```

## Diagnostics

- **dispatch_const_ptr**: bridge=26.1% of algo (FFI overhead may distort results)
- **dispatch_direct**: bridge=26.1% of algo (FFI overhead may distort results)
- **dispatch_trait**: bridge=25.3% of algo (FFI overhead may distort results)
