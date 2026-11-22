# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 2.1x slower than the field

shr_per_iter (26 ns) is 2.1x the fastest (12 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const shows alternating (throttle bounce) (autocorr -0.51)

shr_const's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (12 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### shr_loop_invariant is inconsistent: worst-20% is 3.8x its best-20%

shr_loop_invariant's best 20% of batches run at 11 ns but its worst 20% at 42 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 12.4 ns median
- 2 variants significantly slower than baseline
- Spread: 2.11x (fastest 12.4 ns, slowest 26.2 ns)

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
| Counter frequency | 2112788246 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2651ns | 1893ns | 1548ns | 2026ns | 5632ns | base |
| shr_loop_invariant | 2748ns | 1898ns | 1551ns | 2062ns | 6006ns | +3.66% |
| shr_per_iter | 2666ns | 1910ns | 1561ns | 1911ns | 6038ns | +0.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 17ns | 10ns | 37ns | base | 14.690 |
| shr_loop_invariant | 19ns | 11ns | 42ns | +10.15% | 13.337 |
| shr_per_iter | 36ns | 22ns | 82ns | +109.16% | 7.023 |

## Performance model

- Peak throughput: **25.191 Gops/s** (shr_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 20.645 | 82.0% |
| shr_loop_invariant | 19.248 | 76.4% |
| shr_per_iter | 9.771 | 38.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2651ns | 2651ns | base |
| shr_loop_invariant | 2748ns | 2748ns | +3.66% |
| shr_per_iter | 2666ns | 2666ns | +0.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 12ns | base | --- | [12, 12] | --- | --- | --- | --- |
| shr_loop_invariant | 13ns | +0.8ns (+6.5%) | [+1, +1]ns | [13, 13] | YES | 0.0000 | 0.0000 | 0 |
| shr_per_iter | 26ns | +13.8ns (+110.9%) | [+14, +14]ns | [26, 26] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 10ns | +29.1% | +109.7% |
| 1 | 2 | 12ns | -12.0% | +110.4% |
| 1 | 3 | 10ns | +5.8% | +109.7% |
| 1 | 4 | 10ns | +27.9% | +153.8% |
| 1 | 5 | 10ns | +30.4% | +112.7% |
| 1 | 6 | 12ns | +7.2% | +72.8% |
| 1 | 7 | 10ns | +7.8% | +157.8% |
| 1 | 8 | 13ns | +5.6% | +108.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | -0.514 | HIGH- (thermal bounce) |
| shr_loop_invariant | -0.135 | ok |
| shr_per_iter | -0.385 | moderate- |

**Consistency summary:**

- **shr_loop_invariant**: won 1/8, lost 7/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 33.6ns | 17.4ns | 192.6% | HIGH |
| shr_loop_invariant | 35.0ns | 19.2ns | 182.1% | HIGH |
| shr_per_iter | 32.8ns | 36.5ns | 90.1% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 10.2-36.6 ns)
     10.2 |########################################
     11.5 |#####################################
     12.8 |
     14.1 |
     15.5 |
     16.8 |
     18.1 |#
     19.4 |
     20.8 |
     22.1 |
     23.4 |
     24.7 |
     26.0 |
     27.4 |###
     28.7 |##
     30.0 |###
     31.3 |##
     32.7 |##
     34.0 |####
     35.3 |
  (12 below, 16 above range)

shr_loop_invariant (n=160, range 10.9-41.7 ns)
     10.9 |####################
     12.4 |########################################
     14.0 |
     15.5 |
     17.1 |
     18.6 |
     20.1 |
     21.7 |#
     23.2 |
     24.8 |
     26.3 |
     27.9 |
     29.4 |
     30.9 |#
     32.5 |#
     34.0 |#
     35.6 |##
     37.1 |
     38.7 |###
     40.2 |##
  (6 below, 13 above range)

shr_per_iter (n=160, range 21.5-82.0 ns)
     21.5 |#########################
     24.6 |########################################
     27.6 |
     30.6 |
     33.6 |
     36.7 |
     39.7 |
     42.7 |
     45.7 |
     48.8 |
     51.8 |
     54.8 |
     57.8 |
     60.9 |
     63.9 |
     66.9 |#
     69.9 |#
     73.0 |##
     76.0 |##
     79.0 |#
  (17 below, 15 above range)

```

## Diagnostics

- **shr_const**: CV=61.1% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **shr_const**: bridge=192.7% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=63.0% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=179.7% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=65.0% (high variance, measurements may be unstable)
- **shr_per_iter**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **shr_per_iter**: bridge=90.5% of algo (FFI overhead may distort results)
