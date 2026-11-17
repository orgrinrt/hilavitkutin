# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.7% (fastest 35 ns to slowest 36 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: memcpy_naive** at 35.4 ns median (-1.1% vs baseline)
- Spread: 1.02x (fastest 35.4 ns, slowest 36.0 ns)

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
| memcpy_intrinsic | 2137ns | 2128ns | 2122ns | 2129ns | 2176ns | base |
| memcpy_naive | 2141ns | 2130ns | 2122ns | 2130ns | 2192ns | +0.19% |
| memcpy_unrolled | 2153ns | 2129ns | 2122ns | 2133ns | 2243ns | +0.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 36ns | 29ns | 41ns | base | 1.799 |
| memcpy_naive | 36ns | 30ns | 42ns | +0.44% | 1.791 |
| memcpy_unrolled | 36ns | 29ns | 42ns | +0.57% | 1.789 |

## Performance model

- Peak throughput: **2.213 Gops/s** (memcpy_intrinsic; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 1.788 | 80.8% |
| memcpy_naive | 1.808 | 81.7% |
| memcpy_unrolled | 1.778 | 80.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 2137ns | 2137ns | base |
| memcpy_naive | 2141ns | 2141ns | +0.19% |
| memcpy_unrolled | 2153ns | 2153ns | +0.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 36ns | base | --- | [35, 37] | --- | --- | --- | --- |
| memcpy_naive | 35ns | no significant difference | [-0, +1]ns | [35, 36] | no | 1.0000 | 1.0000 | 8 |
| memcpy_unrolled | 36ns | no significant difference | [-0, +1]ns | [35, 37] | no | 0.5153 | 0.2576 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 38ns | -4.5% | -4.0% |
| 1 | 2 | 35ns | +0.6% | +2.4% |
| 1 | 3 | 35ns | -2.8% | +0.0% |
| 1 | 4 | 35ns | -4.1% | +1.9% |
| 1 | 5 | 37ns | -5.1% | -4.6% |
| 1 | 6 | 35ns | +5.9% | -1.1% |
| 1 | 7 | 36ns | +1.8% | +5.3% |
| 1 | 8 | 36ns | -2.3% | +5.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.253 | moderate- |
| memcpy_naive | 0.421 | moderate+ |
| memcpy_unrolled | 0.297 | moderate+ |

**Consistency summary:**

- **memcpy_naive**: won 5/8, lost 3/8
- **memcpy_unrolled**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.5ns | 35.6ns | 9.8% | HIGH |
| memcpy_naive | 3.8ns | 35.7ns | 10.6% | HIGH |
| memcpy_unrolled | 3.6ns | 35.8ns | 10.0% | HIGH |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 28.9-41.5 ns)
     28.9 |########
     29.5 |##############
     30.2 |#####
     30.8 |###########
     31.4 |########
     32.1 |####################
     32.7 |##############
     33.3 |####################
     33.9 |##############
     34.6 |########################################
     35.2 |#####################################
     35.8 |######################
     36.5 |####################
     37.1 |###############################
     37.7 |############################
     38.3 |#################
     39.0 |#################
     39.6 |#################
     40.2 |############################
     40.9 |#####
  (15 below, 11 above range)

memcpy_naive (n=160, range 30.1-41.9 ns)
     30.1 |#########
     30.7 |########################
     31.3 |#########
     31.9 |########################
     32.5 |####################################
     33.1 |###############
     33.6 |########################################
     34.2 |#########
     34.8 |#################################
     35.4 |#####################
     36.0 |###############
     36.6 |####################################
     37.2 |###############
     37.8 |##################
     38.4 |##################
     38.9 |############
     39.5 |#################################
     40.1 |###############
     40.7 |######
     41.3 |#########
  (15 below, 13 above range)

memcpy_unrolled (n=160, range 29.3-41.5 ns)
     29.3 |###########
     29.9 |########
     30.5 |########
     31.1 |####################
     31.7 |#####
     32.3 |##############
     32.9 |#################
     33.6 |#################
     34.2 |############################
     34.8 |#################
     35.4 |###############################
     36.0 |#####
     36.6 |########################################
     37.2 |##############
     37.8 |##################################
     38.5 |#########################
     39.1 |#########################
     39.7 |############################
     40.3 |##############
     40.9 |#####
  (17 below, 12 above range)

```

## Diagnostics

- **memcpy_naive**: bridge=10.5% of algo (FFI overhead may distort results)
