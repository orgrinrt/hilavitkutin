# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 35 ns to slowest 35 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (35 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 35.4 ns median
- Spread: 1.00x (fastest 35.4 ns, slowest 35.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 2131ns | 2130ns | 2124ns | 2129ns | 2145ns | base |
| memcpy_naive | 2130ns | 2128ns | 2121ns | 2128ns | 2146ns | -0.05% |
| memcpy_unrolled | 2131ns | 2129ns | 2122ns | 2128ns | 2148ns | -0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 36ns | 30ns | 41ns | base | 1.792 |
| memcpy_naive | 36ns | 30ns | 42ns | -0.00% | 1.792 |
| memcpy_unrolled | 35ns | 30ns | 41ns | -0.97% | 1.809 |

## Performance model

- Peak throughput: **2.167 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 1.808 | 83.4% |
| memcpy_naive | 1.808 | 83.4% |
| memcpy_unrolled | 1.808 | 83.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2131ns | 2131ns | base |
| memcpy_naive | 2130ns | 2130ns | -0.05% |
| memcpy_unrolled | 2131ns | 2131ns | -0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 35ns | base | --- | [35, 37] | --- | --- | --- | --- |
| memcpy_naive | 35ns | no significant difference | [-1, +0]ns | [35, 36] | no | 0.9362 | 0.9362 | 4 |
| memcpy_unrolled | 35ns | no significant difference | [-1, +0]ns | [35, 36] | no | 0.4533 | 0.2267 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 35ns | +0.0% | +0.6% |
| 1 | 2 | 36ns | -1.7% | -3.3% |
| 1 | 3 | 35ns | +1.1% | -4.0% |
| 1 | 4 | 36ns | -1.1% | -1.7% |
| 1 | 5 | 36ns | -4.0% | -1.8% |
| 1 | 6 | 36ns | -0.6% | +1.1% |
| 1 | 7 | 37ns | -2.3% | -2.8% |
| 1 | 8 | 36ns | -1.7% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.189 | ok |
| memcpy_naive | -0.043 | ok |
| memcpy_unrolled | 0.446 | moderate+ |

**Consistency summary:**

- **memcpy_naive**: won 6/8, lost 1/8
- **memcpy_unrolled**: won 6/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.5ns | 35.7ns | 9.8% | HIGH |
| memcpy_naive | 3.4ns | 35.7ns | 9.6% | HIGH |
| memcpy_unrolled | 3.6ns | 35.4ns | 10.1% | HIGH |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 29.8-41.2 ns)
     29.8 |####
     30.3 |##############
     30.9 |#######
     31.5 |#######
     32.0 |#########################
     32.6 |#########
     33.2 |##
     33.8 |########################################
     34.3 |###########
     34.9 |###################################
     35.5 |
     36.0 |##############
     36.6 |#########################
     37.2 |###########
     37.7 |#######################
     38.3 |#########################
     38.9 |###########
     39.5 |############################
     40.0 |
     40.6 |###########
  (14 below, 14 above range)

memcpy_naive (n=160, range 29.6-42.0 ns)
     29.6 |##########
     30.2 |############
     30.8 |###############
     31.5 |##########
     32.1 |#################
     32.7 |###################################
     33.3 |##########
     34.0 |############
     34.6 |########################################
     35.2 |##############################
     35.8 |############
     36.4 |#######
     37.1 |###########################
     37.7 |#################
     38.3 |####################
     38.9 |##########
     39.6 |#################
     40.2 |############
     40.8 |#######
     41.4 |##
  (12 below, 17 above range)

memcpy_unrolled (n=160, range 29.5-41.5 ns)
     29.5 |##########################
     30.1 |########
     30.7 |#####################
     31.3 |##########
     31.9 |##########################
     32.5 |########
     33.1 |##
     33.7 |##################################
     34.3 |################
     34.9 |########################################
     35.5 |################
     36.1 |##################
     36.7 |##################################
     37.3 |
     37.9 |########################################
     38.5 |########
     39.1 |##################
     39.7 |#####
     40.3 |##########
     40.9 |##
  (12 below, 17 above range)

```
