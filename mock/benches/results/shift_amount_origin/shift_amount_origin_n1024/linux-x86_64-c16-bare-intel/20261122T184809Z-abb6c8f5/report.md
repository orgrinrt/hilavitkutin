# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 4.9x slower than the field

shr_per_iter (95 ns) is 4.9x the fastest (19 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (19 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.9x the fastest

Fastest shr_const (19 ns) to slowest shr_per_iter (95 ns): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_loop_invariant is inconsistent: worst-20% is 3.7x its best-20%

shr_loop_invariant's best 20% of batches run at 16 ns but its worst 20% at 59 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 19.2 ns median
- 2 variants significantly slower than baseline
- Spread: 4.93x (fastest 19.2 ns, slowest 94.6 ns)

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
| shr_const | 2405ns | 1895ns | 1550ns | 1848ns | 4934ns | base |
| shr_loop_invariant | 2544ns | 1891ns | 1551ns | 1846ns | 5631ns | +5.76% |
| shr_per_iter | 2691ns | 1976ns | 1660ns | 1982ns | 5848ns | +11.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 24ns | 16ns | 49ns | base | 42.349 |
| shr_loop_invariant | 27ns | 16ns | 59ns | +10.16% | 38.443 |
| shr_per_iter | 128ns | 79ns | 276ns | +429.53% | 7.997 |

## Performance model

- Peak throughput: **65.327 Gops/s** (shr_const; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 53.333 | 81.6% |
| shr_loop_invariant | 52.513 | 80.4% |
| shr_per_iter | 10.825 | 16.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2405ns | 2405ns | base |
| shr_loop_invariant | 2544ns | 2544ns | +5.76% |
| shr_per_iter | 2691ns | 2691ns | +11.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 19ns | base | --- | [19, 19] | --- | --- | --- | --- |
| shr_loop_invariant | 20ns | +0.4ns (+2.1%) | [+0, +0]ns | [16, 20] | YES | 0.0171 | 0.0171 | 1 |
| shr_per_iter | 95ns | +75.5ns (+393.2%) | [+75, +76]ns | [94, 95] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 19ns | -15.6% | +393.8% |
| 1 | 2 | 19ns | +3.1% | +392.5% |
| 1 | 3 | 19ns | +2.9% | +391.4% |
| 1 | 4 | 16ns | +26.1% | +504.1% |
| 1 | 5 | 16ns | +3.2% | +393.0% |
| 1 | 6 | 16ns | +2.5% | +390.2% |
| 1 | 7 | 19ns | -15.6% | +393.0% |
| 1 | 8 | 19ns | -15.6% | +392.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.397 | moderate+ |
| shr_loop_invariant | 0.389 | moderate+ |
| shr_per_iter | 0.289 | moderate+ |

**Consistency summary:**

- **shr_loop_invariant**: won 3/8, lost 5/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 30.3ns | 24.2ns | 125.1% | HIGH |
| shr_loop_invariant | 32.0ns | 26.6ns | 120.3% | HIGH |
| shr_per_iter | 32.3ns | 128.0ns | 25.2% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 15.7-49.2 ns)
     15.7 |################
     17.3 |##
     19.0 |########################################
     20.7 |
     22.4 |
     24.0 |
     25.7 |
     27.4 |
     29.1 |
     30.7 |
     32.4 |
     34.1 |
     35.8 |
     37.4 |
     39.1 |
     40.8 |
     42.5 |
     44.1 |
     45.8 |
     47.5 |
  (9 below, 19 above range)

shr_loop_invariant (n=160, range 16.1-59.5 ns)
     16.1 |########################################
     18.3 |################################
     20.4 |
     22.6 |
     24.8 |
     27.0 |
     29.1 |
     31.3 |
     33.5 |
     35.6 |
     37.8 |
     40.0 |#
     42.1 |
     44.3 |
     46.5 |#
     48.6 |##
     50.8 |####
     53.0 |##
     55.1 |
     57.3 |###
  (24 below, 12 above range)

shr_per_iter (n=160, range 79.4-276.0 ns)
     79.4 |
     89.2 |########################################
     99.1 |
    108.9 |
    118.7 |
    128.5 |
    138.4 |
    148.2 |
    158.0 |
    167.8 |
    177.7 |
    187.5 |
    197.3 |
    207.2 |
    217.0 |
    226.8 |
    236.6 |#
    246.5 |
    256.3 |#
    266.1 |
  (28 below, 17 above range)

```

## Diagnostics

- **shr_const**: CV=63.7% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **shr_const**: bridge=122.9% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=66.7% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=122.3% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=61.8% (high variance, measurements may be unstable)
- **shr_per_iter**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **shr_per_iter**: bridge=25.3% of algo (FFI overhead may distort results)
