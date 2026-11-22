# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_const dominates: 24% faster than the next best (shr_loop_invariant)

shr_const (48 ns) leads shr_loop_invariant (60 ns) by 24%, a clear separation rather than a photo finish. CV 100.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### shr_per_iter is an outlier: 7.1x slower than the field

shr_per_iter (342 ns) is 7.1x the fastest (48 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 100.5%)

shr_const wins on median (48 ns) yet has the highest variance (CV 100.5%), while shr_per_iter is the steadiest (CV 71.4%, 342 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (48 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 7.1x the fastest

Fastest shr_const (48 ns) to slowest shr_per_iter (342 ns): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_const is inconsistent: worst-20% is 3.8x its best-20%

shr_const's best 20% of batches run at 40 ns but its worst 20% at 155 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 48.5 ns median
- 2 variants significantly slower than baseline
- Spread: 7.06x (fastest 48.5 ns, slowest 342.2 ns)

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
| shr_const | 2517ns | 1609ns | 1581ns | 1759ns | 5728ns | base |
| shr_loop_invariant | 2739ns | 1946ns | 1614ns | 1973ns | 6164ns | +8.82% |
| shr_per_iter | 2771ns | 2227ns | 2023ns | 2229ns | 5145ns | +10.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 67ns | 40ns | 155ns | base | 61.266 |
| shr_loop_invariant | 85ns | 50ns | 190ns | +26.82% | 48.310 |
| shr_per_iter | 425ns | 311ns | 786ns | +535.26% | 9.644 |

## Performance model

- Peak throughput: **101.136 Gops/s** (shr_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 84.454 | 83.5% |
| shr_loop_invariant | 68.381 | 67.6% |
| shr_per_iter | 11.970 | 11.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2517ns | 2517ns | base |
| shr_loop_invariant | 2739ns | 2739ns | +8.82% |
| shr_per_iter | 2771ns | 2771ns | +10.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 48ns | base | --- | [41, 50] | --- | --- | --- | --- |
| shr_loop_invariant | 60ns | +16.4ns (+33.7%) | [+11, +19]ns | [60, 60] | YES | 0.0000 | 0.0000 | 0 |
| shr_per_iter | 342ns | +299.5ns (+617.5%) | [+293, +301]ns | [342, 342] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 41ns | +20.3% | +742.1% |
| 1 | 2 | 41ns | +52.4% | +737.7% |
| 1 | 3 | 41ns | +47.9% | +588.0% |
| 1 | 4 | 41ns | +47.7% | +742.9% |
| 1 | 5 | 50ns | -0.9% | +590.0% |
| 1 | 6 | 50ns | +20.7% | +587.9% |
| 1 | 7 | 41ns | +47.2% | +740.8% |
| 1 | 8 | 50ns | +19.9% | +585.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.045 | ok |
| shr_loop_invariant | -0.261 | moderate- |
| shr_per_iter | -0.160 | ok |

**Consistency summary:**

- **shr_loop_invariant**: won 1/8, lost 7/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 30.8ns | 66.9ns | 46.0% | HIGH |
| shr_loop_invariant | 34.2ns | 84.8ns | 40.3% | HIGH |
| shr_per_iter | 29.3ns | 424.7ns | 6.9% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 40.5-155.5 ns)
     40.5 |########################################
     46.2 |#########################
     52.0 |
     57.7 |###
     63.5 |
     69.2 |
     75.0 |
     80.7 |
     86.5 |
     92.2 |
     98.0 |
    103.7 |
    109.5 |
    115.2 |
    121.0 |
    126.7 |
    132.5 |
    138.2 |
    144.0 |#
    149.7 |##
  (7 below, 16 above range)

shr_loop_invariant (n=160, range 49.8-190.4 ns)
     49.8 |##
     56.9 |########################################
     63.9 |
     70.9 |
     77.9 |
     85.0 |
     92.0 |
     99.0 |
    106.1 |
    113.1 |
    120.1 |
    127.1 |
    134.2 |
    141.2 |
    148.2 |
    155.3 |
    162.3 |#
    169.3 |
    176.3 |#
    183.4 |#
  (25 below, 17 above range)

shr_per_iter (n=160, range 311.2-785.7 ns)
    311.2 |
    334.9 |########################################
    358.6 |
    382.4 |
    406.1 |
    429.8 |
    453.5 |
    477.3 |
    501.0 |
    524.7 |
    548.4 |
    572.2 |
    595.9 |
    619.6 |
    643.4 |
    667.1 |
    690.8 |
    714.5 |
    738.3 |
    762.0 |
  (16 below, 17 above range)

```

## Diagnostics

- **shr_const**: CV=72.9% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **shr_const**: bridge=40.9% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=64.2% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=40.6% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=57.5% (high variance, measurements may be unstable)
