# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_const dominates: 23% faster than the next best (shr_loop_invariant)

shr_const (50 ns) leads shr_loop_invariant (61 ns) by 23%, a clear separation rather than a photo finish. CV 99.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### shr_per_iter is an outlier: 6.9x slower than the field

shr_per_iter (342 ns) is 6.9x the fastest (50 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 99.2%)

shr_const wins on median (50 ns) yet has the highest variance (CV 99.2%), while shr_per_iter is the steadiest (CV 81.4%, 342 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### shr_loop_invariant shows alternating (throttle bounce) (autocorr -0.54)

shr_loop_invariant's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (50 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 6.9x the fastest

Fastest shr_const (50 ns) to slowest shr_per_iter (342 ns): 6.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_const is inconsistent: worst-20% is 4.0x its best-20%

shr_const's best 20% of batches run at 42 ns but its worst 20% at 167 ns (4.0x) - a bimodal or bursty profile the median hides.

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
| Counter frequency | 2112849135 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2745ns | 1926ns | 1620ns | 1986ns | 6145ns | base |
| shr_loop_invariant | 2868ns | 1939ns | 1781ns | 2132ns | 6160ns | +4.48% |
| shr_per_iter | 2962ns | 2219ns | 1817ns | 2220ns | 6331ns | +7.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 73ns | 42ns | 167ns | base | 55.995 |
| shr_loop_invariant | 86ns | 55ns | 186ns | +18.15% | 47.394 |
| shr_per_iter | 456ns | 280ns | 973ns | +523.55% | 8.980 |

## Performance model

- Peak throughput: **98.233 Gops/s** (shr_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 82.414 | 83.9% |
| shr_loop_invariant | 67.038 | 68.2% |
| shr_per_iter | 11.970 | 12.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2745ns | 2745ns | base |
| shr_loop_invariant | 2868ns | 2868ns | +4.48% |
| shr_per_iter | 2962ns | 2962ns | +7.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 50ns | base | --- | [50, 50] | --- | --- | --- | --- |
| shr_loop_invariant | 61ns | +11.9ns (+24.0%) | [+10, +14]ns | [60, 63] | YES | 0.0000 | 0.0000 | 1 |
| shr_per_iter | 342ns | +292.8ns (+589.1%) | [+293, +293]ns | [342, 342] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 50ns | +47.1% | +587.8% |
| 1 | 2 | 50ns | -1.9% | +589.0% |
| 1 | 3 | 61ns | +3.3% | +357.8% |
| 1 | 4 | 50ns | +26.6% | +464.0% |
| 1 | 5 | 50ns | +20.4% | +590.4% |
| 1 | 6 | 50ns | +20.3% | +590.3% |
| 1 | 7 | 41ns | +46.9% | +741.8% |
| 1 | 8 | 41ns | +46.9% | +741.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.290 | moderate+ |
| shr_loop_invariant | -0.545 | HIGH- (thermal bounce) |
| shr_per_iter | 0.289 | moderate+ |

**Consistency summary:**

- **shr_loop_invariant**: won 1/8, lost 7/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 33.6ns | 73.1ns | 45.9% | HIGH |
| shr_loop_invariant | 35.8ns | 86.4ns | 41.4% | HIGH |
| shr_per_iter | 32.3ns | 456.1ns | 7.1% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 41.7-166.6 ns)
     41.7 |
     47.9 |########################################
     54.2 |
     60.4 |####
     66.7 |
     72.9 |
     79.2 |
     85.4 |
     91.7 |
     97.9 |
    104.2 |
    110.4 |
    116.6 |
    122.9 |
    129.1 |
    135.4 |##
    141.6 |
    147.9 |#
    154.1 |
    160.4 |#
  (28 below, 13 above range)

shr_loop_invariant (n=160, range 54.8-186.4 ns)
     54.8 |########################################
     61.4 |################
     68.0 |#########
     74.5 |#
     81.1 |
     87.7 |
     94.3 |
    100.9 |
    107.4 |
    114.0 |
    120.6 |
    127.2 |
    133.8 |
    140.4 |
    146.9 |
    153.5 |#
    160.1 |#
    166.7 |#
    173.3 |##
    179.8 |#
  (14 below, 16 above range)

shr_per_iter (n=160, range 280.0-972.9 ns)
    280.0 |#####
    314.7 |########################################
    349.3 |#
    383.9 |
    418.6 |
    453.2 |
    487.9 |
    522.5 |
    557.2 |
    591.8 |
    626.4 |
    661.1 |
    695.7 |
    730.4 |
    765.0 |
    799.7 |
    834.3 |
    868.9 |
    903.6 |
    938.2 |#
  (19 below, 21 above range)

```

## Diagnostics

- **shr_const**: CV=67.4% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **shr_const**: bridge=47.3% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=61.0% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=39.8% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=61.1% (high variance, measurements may be unstable)
- **shr_per_iter**: worst_20/best_20 = 3.5x (possible bimodal distribution)
