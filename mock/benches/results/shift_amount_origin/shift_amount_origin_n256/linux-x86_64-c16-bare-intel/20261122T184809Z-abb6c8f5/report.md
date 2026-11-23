# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_loop_invariant dominates: 15% faster than the next best (shr_const)

shr_loop_invariant (11 ns) leads shr_const (12 ns) by 15%, a clear separation rather than a photo finish. CV 108.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### shr_per_iter is an outlier: 2.4x slower than the field

shr_per_iter (26 ns) is 2.4x the fastest (11 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_loop_invariant is fastest but the noisiest (CV 108.7%)

shr_loop_invariant wins on median (11 ns) yet has the highest variance (CV 108.7%), while shr_const is the steadiest (CV 86.7%, 12 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### shr_const shows alternating (throttle bounce) (autocorr -0.66)

shr_const's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### shr_per_iter is inconsistent: worst-20% is 3.8x its best-20%

shr_per_iter's best 20% of batches run at 21 ns but its worst 20% at 81 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: shr_loop_invariant** at 10.9 ns median (-12.8% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.41x (fastest 10.9 ns, slowest 26.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Shift amount: const vs loop-invariant runtime vs per-iter independent |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6a92bf08e314750d |
| Counter frequency | 2112797885 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2635ns | 1890ns | 1544ns | 1962ns | 5744ns | base |
| shr_loop_invariant | 2570ns | 1551ns | 1546ns | 1883ns | 5655ns | -2.46% |
| shr_per_iter | 2762ns | 1903ns | 1555ns | 2086ns | 5997ns | +4.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 17ns | 10ns | 37ns | base | 14.790 |
| shr_loop_invariant | 18ns | 11ns | 39ns | +3.63% | 14.271 |
| shr_per_iter | 38ns | 21ns | 81ns | +117.52% | 6.799 |

## Performance model

- Peak throughput: **25.214 Gops/s** (shr_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 20.480 | 81.2% |
| shr_loop_invariant | 23.486 | 93.1% |
| shr_per_iter | 9.734 | 38.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2635ns | 2635ns | base |
| shr_loop_invariant | 2570ns | 2570ns | -2.46% |
| shr_per_iter | 2762ns | 2762ns | +4.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 12ns | base | --- | [12, 12] | --- | --- | --- | --- |
| shr_loop_invariant | 11ns | no significant difference | [-2, +0]ns | [11, 11] | no | 0.1328 | 0.1328 | 0 |
| shr_per_iter | 26ns | +13.9ns (+111.2%) | [+14, +14]ns | [26, 26] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 12ns | -12.8% | +111.2% |
| 1 | 2 | 12ns | -12.8% | +72.0% |
| 1 | 3 | 10ns | +7.8% | +112.7% |
| 1 | 4 | 12ns | -12.8% | +111.2% |
| 1 | 5 | 10ns | +6.8% | +114.6% |
| 1 | 6 | 12ns | -12.1% | +112.9% |
| 1 | 7 | 10ns | +6.9% | +110.8% |
| 1 | 8 | 12ns | -12.8% | +111.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | -0.659 | HIGH- (thermal bounce) |
| shr_loop_invariant | -0.375 | moderate- |
| shr_per_iter | -0.600 | HIGH- (thermal bounce) |

**Consistency summary:**

- **shr_loop_invariant**: won 5/8, lost 3/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 33.2ns | 17.3ns | 191.8% | HIGH |
| shr_loop_invariant | 32.3ns | 17.9ns | 179.9% | HIGH |
| shr_per_iter | 33.8ns | 37.7ns | 89.6% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 10.2-37.3 ns)
     10.2 |##############
     11.5 |########################################
     12.9 |
     14.2 |
     15.6 |
     16.9 |
     18.3 |
     19.7 |
     21.0 |
     22.4 |
     23.7 |#
     25.1 |#
     26.5 |#
     27.8 |#
     29.2 |
     30.5 |
     31.9 |#
     33.2 |#
     34.6 |
     36.0 |#
  (14 below, 16 above range)

shr_loop_invariant (n=160, range 10.8-39.2 ns)
     10.8 |########################################
     12.3 |
     13.7 |
     15.1 |
     16.5 |
     17.9 |
     19.3 |
     20.7 |
     22.2 |
     23.6 |
     25.0 |
     26.4 |
     27.8 |
     29.2 |
     30.7 |
     32.1 |
     33.5 |###
     34.9 |
     36.3 |####
     37.7 |
  (18 below, 13 above range)

shr_per_iter (n=160, range 21.5-81.3 ns)
     21.5 |################################
     24.5 |########################################
     27.5 |
     30.5 |
     33.5 |
     36.4 |
     39.4 |
     42.4 |
     45.4 |
     48.4 |###
     51.4 |
     54.4 |
     57.4 |
     60.4 |
     63.4 |
     66.3 |###
     69.3 |#
     72.3 |###
     75.3 |##
     78.3 |###
  (7 below, 14 above range)

```

## Diagnostics

- **shr_const**: CV=62.6% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **shr_const**: bridge=191.2% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=66.1% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=179.8% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=62.8% (high variance, measurements may be unstable)
- **shr_per_iter**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **shr_per_iter**: bridge=89.7% of algo (FFI overhead may distort results)
