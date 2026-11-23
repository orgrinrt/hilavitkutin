# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_const dominates: 20% faster than the next best (shr_loop_invariant)

shr_const (50 ns) leads shr_loop_invariant (60 ns) by 20%, a clear separation rather than a photo finish. CV 93.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### shr_per_iter is an outlier: 6.9x slower than the field

shr_per_iter (342 ns) is 6.9x the fastest (50 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 93.7%)

shr_const wins on median (50 ns) yet has the highest variance (CV 93.7%), while shr_per_iter is the steadiest (CV 78.5%, 342 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (50 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.9x the fastest

Fastest shr_const (50 ns) to slowest shr_per_iter (342 ns): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_const is inconsistent: worst-20% is 3.8x its best-20%

shr_const's best 20% of batches run at 43 ns but its worst 20% at 162 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 49.7 ns median
- 2 variants significantly slower than baseline
- Spread: 6.89x (fastest 49.7 ns, slowest 342.2 ns)

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
| shr_const | 2795ns | 1930ns | 1619ns | 2076ns | 6124ns | base |
| shr_loop_invariant | 2715ns | 1939ns | 1586ns | 2031ns | 5897ns | -2.84% |
| shr_per_iter | 2822ns | 2219ns | 1817ns | 2210ns | 5664ns | +0.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 74ns | 43ns | 162ns | base | 55.109 |
| shr_loop_invariant | 86ns | 50ns | 186ns | +15.22% | 47.829 |
| shr_per_iter | 432ns | 280ns | 861ns | +481.89% | 9.471 |

## Performance model

- Peak throughput: **96.348 Gops/s** (shr_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 82.414 | 85.5% |
| shr_loop_invariant | 68.495 | 71.1% |
| shr_per_iter | 11.970 | 12.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2795ns | 2795ns | base |
| shr_loop_invariant | 2715ns | 2715ns | -2.84% |
| shr_per_iter | 2822ns | 2822ns | +0.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 50ns | base | --- | [50, 50] | --- | --- | --- | --- |
| shr_loop_invariant | 60ns | +10.2ns (+20.5%) | [+10, +11]ns | [60, 62] | YES | 0.0000 | 0.0000 | 0 |
| shr_per_iter | 342ns | +292.5ns (+588.5%) | [+292, +293]ns | [342, 342] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 50ns | +3.6% | +585.3% |
| 1 | 2 | 61ns | +1.0% | +356.1% |
| 1 | 3 | 50ns | +20.4% | +589.3% |
| 1 | 4 | 52ns | +17.1% | +440.0% |
| 1 | 5 | 50ns | +27.0% | +588.4% |
| 1 | 6 | 41ns | +26.5% | +739.2% |
| 1 | 7 | 50ns | +22.2% | +588.7% |
| 1 | 8 | 41ns | +28.2% | +743.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.015 | ok |
| shr_loop_invariant | -0.361 | moderate- |
| shr_per_iter | -0.373 | moderate- |

**Consistency summary:**

- **shr_loop_invariant**: won 0/8, lost 8/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 34.2ns | 74.3ns | 46.0% | HIGH |
| shr_loop_invariant | 33.9ns | 85.6ns | 39.6% | HIGH |
| shr_per_iter | 30.0ns | 432.5ns | 6.9% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 42.5-162.0 ns)
     42.5 |#
     48.5 |########################################
     54.5 |##
     60.4 |#######
     66.4 |
     72.4 |
     78.4 |
     84.4 |
     90.3 |
     96.3 |
    102.3 |
    108.3 |
    114.2 |#
    120.2 |#
    126.2 |#
    132.2 |#
    138.1 |#
    144.1 |##
    150.1 |#
    156.1 |#
  (24 below, 16 above range)

shr_loop_invariant (n=160, range 49.8-185.5 ns)
     49.8 |#########
     56.6 |########################################
     63.4 |####
     70.2 |
     77.0 |#
     83.8 |
     90.6 |
     97.3 |
    104.1 |
    110.9 |
    117.7 |
    124.5 |
    131.3 |
    138.0 |#
    144.8 |##
    151.6 |#
    158.4 |#
    165.2 |##
    172.0 |##
    178.8 |
  (21 below, 14 above range)

shr_per_iter (n=160, range 279.9-861.5 ns)
    279.9 |######
    309.0 |
    338.1 |########################################
    367.2 |#
    396.3 |
    425.3 |
    454.4 |
    483.5 |
    512.6 |
    541.6 |
    570.7 |
    599.8 |
    628.9 |
    657.9 |
    687.0 |
    716.1 |
    745.2 |
    774.3 |
    803.3 |
    832.4 |
  (18 below, 20 above range)

```

## Diagnostics

- **shr_const**: CV=62.6% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **shr_const**: bridge=47.7% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=62.4% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=40.5% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=62.1% (high variance, measurements may be unstable)
- **shr_per_iter**: worst_20/best_20 = 3.1x (possible bimodal distribution)
