# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 11 ns to slowest 11 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### load_aligned is fastest but the noisiest (CV 26.2%)

load_aligned wins on median (11 ns) yet has the highest variance (CV 26.2%), while load_unaligned is the steadiest (CV 25.9%, 11 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (11 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader load_aligned vs stability leader load_unaligned (+0% speed for 1.0x steadier)

load_aligned is fastest (11 ns, CV 26.2%); load_unaligned gives up 0.0% median for 1.0x lower variance (CV 25.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### load_unaligned is inconsistent: worst-20% is 2.0x its best-20%

load_unaligned's best 20% of batches run at 8 ns but its worst 20% at 15 ns (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 10.8 ns median
- Spread: 1.00x (fastest 10.8 ns, slowest 10.8 ns)

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
| load_aligned | 2156ns | 2136ns | 2128ns | 2136ns | 2243ns | base |
| load_byte_pack | 2151ns | 2134ns | 2127ns | 2137ns | 2218ns | -0.20% |
| load_unaligned | 2161ns | 2134ns | 2126ns | 2137ns | 2269ns | +0.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 11ns | 8ns | 16ns | base | 22.370 |
| load_byte_pack | 11ns | 8ns | 16ns | +0.23% | 22.319 |
| load_unaligned | 11ns | 8ns | 15ns | -3.22% | 23.114 |

## Performance model

- Peak throughput: **33.533 Gops/s** (load_unaligned; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 23.704 | 70.7% |
| load_byte_pack | 23.704 | 70.7% |
| load_unaligned | 23.704 | 70.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2156ns | 2156ns | base |
| load_byte_pack | 2151ns | 2151ns | -0.20% |
| load_unaligned | 2161ns | 2161ns | +0.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 11ns | base | --- | [11, 12] | --- | --- | --- | --- |
| load_byte_pack | 11ns | no significant difference | [-1, +0]ns | [10, 11] | no | 0.2236 | 0.2236 | 8 |
| load_unaligned | 11ns | no significant difference | [-1, +0]ns | [10, 11] | no | 0.0523 | 0.0261 | 12 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 10ns | +6.0% | +8.0% |
| 1 | 2 | 12ns | -11.1% | -12.8% |
| 1 | 3 | 11ns | -1.9% | +1.9% |
| 1 | 4 | 16ns | -25.0% | -2.6% |
| 1 | 5 | 11ns | +56.5% | -3.7% |
| 1 | 6 | 10ns | +5.8% | +3.8% |
| 1 | 7 | 11ns | -7.4% | -7.4% |
| 1 | 8 | 11ns | -9.3% | -3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.219 | moderate- |
| load_byte_pack | 0.102 | ok |
| load_unaligned | -0.111 | ok |

**Consistency summary:**

- **load_byte_pack**: won 5/8, lost 3/8
- **load_unaligned**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.6ns | 11.4ns | 22.3% | HIGH |
| load_byte_pack | 2.6ns | 11.5ns | 22.4% | HIGH |
| load_unaligned | 2.7ns | 11.1ns | 24.5% | HIGH |

## Distribution (algo ns)

```
load_aligned (n=160, range 8.0-15.8 ns)
      8.0 |###############
      8.4 |
      8.7 |####################
      9.1 |####################
      9.5 |#################
      9.9 |###############
     10.3 |##########################
     10.7 |########################################
     11.1 |#############
     11.5 |########################
     11.9 |#############
     12.3 |###############
     12.7 |#############
     13.1 |####################
     13.5 |######
     13.9 |###########
     14.2 |######
     14.6 |####
     15.0 |########
     15.4 |####
  (14 below, 12 above range)

load_byte_pack (n=160, range 8.1-15.9 ns)
      8.1 |###
      8.5 |########################
      8.9 |##############################
      9.3 |########################################
      9.7 |#################################
     10.1 |########################################
     10.5 |###########################
     10.9 |##############################
     11.3 |
     11.7 |###########################
     12.0 |###########################
     12.4 |###############
     12.8 |#####################
     13.2 |#########
     13.6 |###############
     14.0 |##################
     14.4 |#########
     14.8 |######
     15.2 |###############
     15.6 |###
  (17 below, 13 above range)

load_unaligned (n=160, range 7.6-15.4 ns)
      7.6 |#####################
      8.0 |###########################
      8.4 |
      8.8 |##################
      9.2 |########################
      9.6 |###########################
     10.0 |####################################
     10.3 |#################################
     10.7 |####################################
     11.1 |########################################
     11.5 |##############################
     11.9 |###########################
     12.3 |###############
     12.7 |#########
     13.0 |###############
     13.4 |############
     13.8 |
     14.2 |############
     14.6 |###############
     15.0 |############
  (13 below, 11 above range)

```

## Diagnostics

- **load_aligned**: CV=24.8% (high variance, measurements may be unstable)
- **load_aligned**: bridge=23.1% of algo (FFI overhead may distort results)
- **load_byte_pack**: CV=24.6% (high variance, measurements may be unstable)
- **load_byte_pack**: bridge=23.1% of algo (FFI overhead may distort results)
- **load_unaligned**: CV=25.3% (high variance, measurements may be unstable)
- **load_unaligned**: bridge=25.0% of algo (FFI overhead may distort results)
