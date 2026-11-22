# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_const dominates: 212% faster than the next best (shr_loop_invariant)

shr_const (1 ns) leads shr_loop_invariant (2 ns) by 212%, a clear separation rather than a photo finish. CV 90.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### shr_per_iter is an outlier: 8.4x slower than the field

shr_per_iter (7 ns) is 8.4x the fastest (1 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 90.3%)

shr_const wins on median (1 ns) yet has the highest variance (CV 90.3%), while shr_per_iter is the steadiest (CV 22.4%, 7 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### shr_per_iter shows alternating (throttle bounce) (autocorr -0.54)

shr_per_iter's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (1 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.4x the fastest

Fastest shr_const (1 ns) to slowest shr_per_iter (7 ns): 8.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_const is inconsistent: worst-20% is 12.0x its best-20%

shr_const's best 20% of batches run at 0 ns but its worst 20% at 2 ns (12.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 0.8 ns median
- 2 variants significantly slower than baseline
- Spread: 8.38x (fastest 0.8 ns, slowest 6.7 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2157ns | 2144ns | 2131ns | 2148ns | 2209ns | base |
| shr_loop_invariant | 2158ns | 2138ns | 2131ns | 2142ns | 2232ns | +0.07% |
| shr_per_iter | 2179ns | 2139ns | 2133ns | 2144ns | 2330ns | +1.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 1ns | 0ns | 2ns | base | 250.673 |
| shr_loop_invariant | 3ns | 1ns | 4ns | +160.28% | 96.308 |
| shr_per_iter | 7ns | 5ns | 9ns | +549.39% | 38.601 |

## Performance model

- Peak throughput: **1462.857 Gops/s** (shr_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 320.000 | 21.9% |
| shr_loop_invariant | 102.400 | 7.0% |
| shr_per_iter | 38.209 | 2.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2157ns | 2157ns | base |
| shr_loop_invariant | 2158ns | 2158ns | +0.07% |
| shr_per_iter | 2179ns | 2179ns | +1.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 1ns | base | --- | [1, 1] | --- | --- | --- | --- |
| shr_loop_invariant | 2ns | +1.7ns (+212.5%) | [+1, +2]ns | [2, 3] | YES | 0.0000 | 0.0000 | 15 |
| shr_per_iter | 7ns | +5.7ns (+706.2%) | [+5, +6]ns | [6, 7] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 1ns | +125.0% | +437.5% |
| 1 | 2 | 1ns | +187.5% | +787.5% |
| 1 | 3 | 1ns | +262.5% | +675.0% |
| 1 | 4 | 1ns | +108.3% | +458.3% |
| 1 | 5 | 1ns | +212.5% | +737.5% |
| 1 | 6 | 1ns | +262.5% | +681.2% |
| 1 | 7 | 1ns | +212.5% | +762.5% |
| 1 | 8 | 1ns | +187.5% | +625.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | -0.208 | moderate- |
| shr_loop_invariant | -0.432 | moderate- |
| shr_per_iter | -0.539 | HIGH- (thermal bounce) |

**Consistency summary:**

- **shr_loop_invariant**: won 0/8, lost 8/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 2.4ns | 1.0ns | 232.7% | HIGH |
| shr_loop_invariant | 3.9ns | 2.7ns | 147.6% | HIGH |
| shr_per_iter | 2.7ns | 6.6ns | 40.4% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 0.2-2.1 ns)
      0.2 |
      0.3 |
      0.4 |####################################
      0.5 |
      0.6 |
      0.7 |
      0.8 |########################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |##################################
      1.2 |
      1.3 |
      1.4 |
      1.5 |
      1.6 |##########################
      1.7 |
      1.8 |
      1.9 |
      2.0 |
  (18 below, 18 above range)

shr_loop_invariant (n=160, range 1.3-4.2 ns)
      1.3 |
      1.5 |
      1.6 |#########################
      1.8 |
      1.9 |
      2.1 |########################################
      2.2 |
      2.3 |
      2.5 |#############################
      2.6 |
      2.8 |####################################
      2.9 |
      3.1 |
      3.2 |####################
      3.4 |
      3.5 |
      3.6 |####################
      3.8 |
      3.9 |
      4.1 |########
  (16 below, 10 above range)

shr_per_iter (n=160, range 4.6-8.7 ns)
      4.6 |
      4.8 |########################
      5.0 |
      5.2 |######################
      5.4 |
      5.6 |########################################
      5.8 |
      6.0 |##################################
      6.2 |
      6.5 |
      6.7 |############################
      6.9 |
      7.1 |##########################
      7.3 |
      7.5 |##########################
      7.7 |
      7.9 |####################################
      8.1 |##################
      8.3 |
      8.5 |
  (17 below, 16 above range)

```

## Diagnostics

- **shr_const**: CV=70.7% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 12.0x (possible bimodal distribution)
- **shr_const**: bridge=312.5% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=39.6% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=148.0% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=22.7% (high variance, measurements may be unstable)
- **shr_per_iter**: bridge=37.3% of algo (FFI overhead may distort results)
