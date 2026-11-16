# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.8% (fastest 36 ns to slowest 37 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (memcpy_intrinsic)

The baseline memcpy_intrinsic is the fastest (36 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader memcpy_intrinsic vs stability leader memcpy_naive (+2% speed for 1.0x steadier)

memcpy_intrinsic is fastest (36 ns, CV 11.4%); memcpy_naive gives up 1.8% median for 1.0x lower variance (CV 11.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (memcpy_intrinsic) is the fastest** at 36.5 ns median
- Spread: 1.02x (fastest 36.5 ns, slowest 37.1 ns)

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
| memcpy_intrinsic | 2167ns | 2158ns | 2148ns | 2159ns | 2213ns | base |
| memcpy_naive | 2161ns | 2156ns | 2150ns | 2156ns | 2186ns | -0.30% |
| memcpy_unrolled | 2169ns | 2157ns | 2150ns | 2157ns | 2224ns | +0.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 37ns | 31ns | 43ns | base | 1.731 |
| memcpy_naive | 37ns | 32ns | 43ns | +1.00% | 1.714 |
| memcpy_unrolled | 37ns | 31ns | 43ns | +0.74% | 1.718 |

## Performance model

- Peak throughput: **2.061 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 1.756 | 85.2% |
| memcpy_naive | 1.725 | 83.7% |
| memcpy_unrolled | 1.725 | 83.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2167ns | 2167ns | base |
| memcpy_naive | 2161ns | 2161ns | -0.30% |
| memcpy_unrolled | 2169ns | 2169ns | +0.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 36ns | base | --- | [36, 38] | --- | --- | --- | --- |
| memcpy_naive | 37ns | no significant difference | [-0, +1]ns | [36, 38] | no | 0.6812 | 0.6812 | 12 |
| memcpy_unrolled | 37ns | no significant difference | [-0, +1]ns | [37, 38] | no | 0.5833 | 0.2917 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 37ns | +0.0% | -1.2% |
| 1 | 2 | 36ns | -1.1% | +4.1% |
| 1 | 3 | 36ns | +1.9% | -0.6% |
| 1 | 4 | 36ns | +1.9% | +4.7% |
| 1 | 5 | 38ns | -2.6% | -1.6% |
| 1 | 6 | 38ns | -1.6% | +1.2% |
| 1 | 7 | 36ns | +3.6% | +3.6% |
| 1 | 8 | 36ns | +5.8% | +1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | 0.222 | moderate+ |
| memcpy_naive | 0.402 | moderate+ |
| memcpy_unrolled | -0.223 | moderate- |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 4/8
- **memcpy_unrolled**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 4.3ns | 37.0ns | 11.5% | HIGH |
| memcpy_naive | 4.3ns | 37.3ns | 11.5% | HIGH |
| memcpy_unrolled | 4.2ns | 37.2ns | 11.2% | HIGH |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 31.3-43.1 ns)
     31.3 |#################
     31.9 |##############
     32.4 |####################
     33.0 |########
     33.6 |#################
     34.2 |##############
     34.8 |########################################
     35.4 |###############################
     36.0 |###############################
     36.6 |##################################
     37.2 |####################
     37.8 |######################
     38.4 |##############
     39.0 |###########
     39.5 |######################
     40.1 |#################
     40.7 |##############
     41.3 |########
     41.9 |##############
     42.5 |##############
  (12 below, 12 above range)

memcpy_naive (n=160, range 31.5-43.5 ns)
     31.5 |#############
     32.1 |##
     32.7 |#############
     33.3 |##########
     33.9 |################
     34.5 |########################################
     35.1 |########################
     35.7 |########################
     36.3 |##########################
     36.9 |################################
     37.5 |#############
     38.1 |##################
     38.7 |#############
     39.3 |#############
     39.9 |########################
     40.5 |#####
     41.1 |#############
     41.7 |##################
     42.3 |########
     42.9 |########################
  (15 below, 12 above range)

memcpy_unrolled (n=160, range 31.1-43.3 ns)
     31.1 |##########
     31.7 |##########
     32.3 |########
     32.9 |##########################
     33.5 |################
     34.1 |################
     34.7 |################
     35.3 |#####################
     35.9 |##################
     36.5 |########################################
     37.2 |##########################
     37.8 |##################
     38.4 |##################
     39.0 |################
     39.6 |################
     40.2 |#####################
     40.8 |#############
     41.4 |##########
     42.0 |########################
     42.6 |########
  (12 below, 14 above range)

```

## Diagnostics

- **memcpy_intrinsic**: bridge=11.5% of algo (FFI overhead may distort results)
- **memcpy_naive**: bridge=11.3% of algo (FFI overhead may distort results)
- **memcpy_unrolled**: bridge=11.3% of algo (FFI overhead may distort results)
