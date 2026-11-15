# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 2.1x slower than the field

shr_per_iter (26 ns) is 2.1x the fastest (12 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (12 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader shr_const vs stability leader shr_loop_invariant (+6% speed for 1.0x steadier)

shr_const is fastest (12 ns, CV 93.9%); shr_loop_invariant gives up 6.5% median for 1.0x lower variance (CV 90.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### shr_const is inconsistent: worst-20% is 3.9x its best-20%

shr_const's best 20% of batches run at 10 ns but its worst 20% at 40 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 12.4 ns median
- 2 variants significantly slower than baseline
- Spread: 2.14x (fastest 12.4 ns, slowest 26.5 ns)

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
| Counter frequency | 2112849135 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2717ns | 1889ns | 1545ns | 1957ns | 6169ns | base |
| shr_loop_invariant | 2628ns | 1889ns | 1545ns | 1876ns | 5965ns | -3.28% |
| shr_per_iter | 2851ns | 1904ns | 1737ns | 2051ns | 6363ns | +4.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 18ns | 10ns | 40ns | base | 14.389 |
| shr_loop_invariant | 18ns | 11ns | 41ns | +2.92% | 13.980 |
| shr_per_iter | 39ns | 24ns | 87ns | +121.42% | 6.499 |

## Performance model

- Peak throughput: **25.245 Gops/s** (shr_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 20.645 | 81.8% |
| shr_loop_invariant | 19.394 | 76.8% |
| shr_per_iter | 9.660 | 38.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2717ns | 2717ns | base |
| shr_loop_invariant | 2628ns | 2628ns | -3.28% |
| shr_per_iter | 2851ns | 2851ns | +4.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 12ns | base | --- | [12, 12] | --- | --- | --- | --- |
| shr_loop_invariant | 13ns | +0.7ns (+5.6%) | [+0, +1]ns | [13, 13] | YES | 0.0261 | 0.0261 | 1 |
| shr_per_iter | 26ns | +15.9ns (+128.2%) | [+14, +16]ns | [26, 26] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 10ns | +30.4% | +159.8% |
| 1 | 2 | 12ns | -12.1% | +113.3% |
| 1 | 3 | 10ns | +6.9% | +159.8% |
| 1 | 4 | 10ns | +30.4% | +159.3% |
| 1 | 5 | 12ns | -12.1% | +74.6% |
| 1 | 6 | 12ns | -12.1% | +112.9% |
| 1 | 7 | 12ns | +6.5% | +113.7% |
| 1 | 8 | 12ns | +6.9% | +114.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.058 | ok |
| shr_loop_invariant | -0.138 | ok |
| shr_per_iter | -0.136 | ok |

**Consistency summary:**

- **shr_loop_invariant**: won 3/8, lost 5/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 34.5ns | 17.8ns | 193.7% | HIGH |
| shr_loop_invariant | 33.1ns | 18.3ns | 180.6% | HIGH |
| shr_per_iter | 35.2ns | 39.4ns | 89.3% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 10.1-40.0 ns)
     10.1 |##############
     11.6 |########################################
     13.1 |
     14.6 |
     16.1 |
     17.6 |
     19.1 |
     20.6 |
     22.1 |
     23.6 |
     25.0 |
     26.5 |
     28.0 |
     29.5 |#
     31.0 |#
     32.5 |
     34.0 |
     35.5 |##
     37.0 |#
     38.5 |#
  (17 below, 17 above range)

shr_loop_invariant (n=160, range 10.8-41.3 ns)
     10.8 |#####################
     12.3 |########################################
     13.9 |
     15.4 |
     16.9 |
     18.4 |
     20.0 |
     21.5 |
     23.0 |
     24.5 |
     26.1 |
     27.6 |
     29.1 |#
     30.6 |
     32.2 |
     33.7 |#
     35.2 |
     36.7 |#
     38.3 |#
     39.8 |#
  (24 below, 17 above range)

shr_per_iter (n=160, range 24.1-87.4 ns)
     24.1 |########################################
     27.3 |
     30.4 |
     33.6 |
     36.7 |
     39.9 |
     43.1 |
     46.2 |
     49.4 |
     52.6 |
     55.7 |
     58.9 |
     62.1 |
     65.2 |
     68.4 |
     71.6 |
     74.7 |
     77.9 |##
     81.1 |
     84.2 |###
  (15 below, 14 above range)

```

## Diagnostics

- **shr_const**: CV=65.5% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **shr_const**: bridge=192.7% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=65.5% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=180.3% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=63.7% (high variance, measurements may be unstable)
- **shr_per_iter**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **shr_per_iter**: bridge=89.1% of algo (FFI overhead may distort results)
