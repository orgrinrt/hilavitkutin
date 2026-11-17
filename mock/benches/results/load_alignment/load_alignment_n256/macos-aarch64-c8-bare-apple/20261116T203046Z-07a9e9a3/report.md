# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.3% (fastest 16 ns to slowest 16 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### load_byte_pack shows alternating (throttle bounce) (autocorr -0.54)

load_byte_pack's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (16 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### load_unaligned's comparison is tie-heavy (11% tied pairs)

11% of paired samples for load_unaligned are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### load_byte_pack is inconsistent: worst-20% is 1.5x its best-20%

load_byte_pack's best 20% of batches run at 12 ns but its worst 20% at 18 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 15.6 ns median
- Spread: 1.01x (fastest 15.6 ns, slowest 15.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x2c8af49106b7ed53 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2110ns | 2105ns | 2099ns | 2105ns | 2136ns | base |
| load_byte_pack | 2121ns | 2107ns | 2100ns | 2108ns | 2181ns | +0.52% |
| load_unaligned | 2122ns | 2104ns | 2100ns | 2106ns | 2191ns | +0.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 15ns | 12ns | 18ns | base | 16.549 |
| load_byte_pack | 16ns | 12ns | 18ns | +0.73% | 16.429 |
| load_unaligned | 16ns | 12ns | 19ns | +1.28% | 16.340 |

## Performance model

- Peak throughput: **20.834 Gops/s** (load_byte_pack; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 16.410 | 78.8% |
| load_byte_pack | 16.203 | 77.8% |
| load_unaligned | 16.203 | 77.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2110ns | 2110ns | base |
| load_byte_pack | 2121ns | 2121ns | +0.52% |
| load_unaligned | 2122ns | 2122ns | +0.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 16ns | base | --- | [15, 16] | --- | --- | --- | --- |
| load_byte_pack | 16ns | no significant difference | [-0, +0]ns | [15, 16] | no | 0.5598 | 0.5598 | 16 |
| load_unaligned | 16ns | no significant difference | [+0, +1]ns | [15, 16] | no | 0.5598 | 0.4502 | **18** (11%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 16ns | +0.0% | -2.6% |
| 1 | 2 | 16ns | +1.2% | -3.7% |
| 1 | 3 | 15ns | +1.3% | +5.2% |
| 1 | 4 | 15ns | +2.6% | +0.0% |
| 1 | 5 | 16ns | -1.3% | -1.3% |
| 1 | 6 | 16ns | -2.5% | +0.0% |
| 1 | 7 | 16ns | +2.8% | +0.0% |
| 1 | 8 | 15ns | -2.6% | +6.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.191 | ok |
| load_byte_pack | -0.537 | HIGH- (thermal bounce) |
| load_unaligned | -0.036 | ok |

**Consistency summary:**

- **load_byte_pack**: won 3/8, lost 4/8
- **load_unaligned**: won 3/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.7ns | 15.5ns | 17.6% | HIGH |
| load_byte_pack | 2.6ns | 15.6ns | 16.8% | HIGH |
| load_unaligned | 2.6ns | 15.7ns | 16.5% | HIGH |

## Distribution (algo ns)

```
load_aligned (n=160, range 12.3-18.4 ns)
     12.3 |##########################
     12.6 |#############
     12.9 |
     13.2 |##################
     13.5 |##########################
     13.8 |
     14.1 |################################
     14.4 |##################
     14.7 |##################
     15.0 |
     15.3 |#####################################
     15.6 |########################################
     15.9 |##########################
     16.2 |
     16.6 |#####################
     16.9 |##################################
     17.2 |
     17.5 |##########
     17.8 |########################
     18.1 |##################################
  (8 below, 8 above range)

load_byte_pack (n=160, range 12.3-18.5 ns)
     12.3 |#####
     12.6 |#############
     12.9 |
     13.2 |#####################
     13.5 |##########
     13.8 |
     14.1 |########################################
     14.5 |########################
     14.8 |################
     15.1 |
     15.4 |##################################
     15.7 |##################################
     16.0 |##########################
     16.3 |
     16.6 |##################################
     16.9 |################################
     17.3 |################
     17.6 |
     17.9 |########################
     18.2 |##################
  (14 below, 14 above range)

load_unaligned (n=160, range 12.5-18.7 ns)
     12.5 |######################
     12.8 |#####
     13.1 |#################
     13.4 |
     13.7 |#################
     14.0 |############################
     14.3 |##################################
     14.7 |
     15.0 |############################
     15.3 |##################################
     15.6 |########################################
     15.9 |#####################################
     16.2 |
     16.5 |##################################
     16.8 |####################
     17.2 |
     17.5 |########################################
     17.8 |###########
     18.1 |##############
     18.4 |
  (10 below, 15 above range)

```

## Diagnostics

- **load_aligned**: bridge=16.0% of algo (FFI overhead may distort results)
- **load_byte_pack**: bridge=15.8% of algo (FFI overhead may distort results)
- **load_unaligned**: bridge=15.8% of algo (FFI overhead may distort results)
