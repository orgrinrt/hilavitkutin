# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_const dominates: 212% faster than the next best (shr_loop_invariant)

shr_const (1 ns) leads shr_loop_invariant (2 ns) by 212%, a clear separation rather than a photo finish. CV 77.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### shr_per_iter is an outlier: 8.9x slower than the field

shr_per_iter (7 ns) is 8.9x the fastest (1 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 77.4%)

shr_const wins on median (1 ns) yet has the highest variance (CV 77.4%), while shr_per_iter is the steadiest (CV 23.4%, 7 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (1 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 8.9x the fastest

Fastest shr_const (1 ns) to slowest shr_per_iter (7 ns): 8.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_const is inconsistent: worst-20% is 6.7x its best-20%

shr_const's best 20% of batches run at 0 ns but its worst 20% at 2 ns (6.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 0.8 ns median
- 2 variants significantly slower than baseline
- Spread: 8.87x (fastest 0.8 ns, slowest 7.1 ns)

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
| shr_const | 2175ns | 2109ns | 2075ns | 2138ns | 2387ns | base |
| shr_loop_invariant | 2283ns | 2247ns | 2105ns | 2290ns | 2437ns | +4.94% |
| shr_per_iter | 2268ns | 2245ns | 2089ns | 2274ns | 2430ns | +4.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 1ns | 0ns | 2ns | base | 239.953 |
| shr_loop_invariant | 3ns | 1ns | 4ns | +140.01% | 99.976 |
| shr_per_iter | 7ns | 5ns | 10ns | +568.42% | 35.898 |

## Performance model

- Peak throughput: **853.333 Gops/s** (shr_const; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 320.000 | 37.5% |
| shr_loop_invariant | 102.400 | 12.0% |
| shr_per_iter | 36.056 | 4.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2175ns | 2175ns | base |
| shr_loop_invariant | 2283ns | 2283ns | +4.94% |
| shr_per_iter | 2268ns | 2268ns | +4.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 1ns | base | --- | [1, 1] | --- | --- | --- | --- |
| shr_loop_invariant | 2ns | +1.6ns (+206.2%) | [+1, +2]ns | [2, 2] | YES | 0.0000 | 0.0000 | 15 |
| shr_per_iter | 7ns | +6.3ns (+787.5%) | [+6, +7]ns | [7, 8] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 1ns | +262.5% | +787.5% |
| 1 | 2 | 1ns | +108.3% | +541.7% |
| 1 | 3 | 1ns | +75.0% | +541.7% |
| 1 | 4 | 1ns | +108.3% | +437.5% |
| 1 | 5 | 1ns | +212.5% | +837.5% |
| 1 | 6 | 1ns | +212.5% | +812.5% |
| 1 | 7 | 1ns | +212.5% | +787.5% |
| 1 | 8 | 1ns | +212.5% | +787.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.392 | moderate+ |
| shr_loop_invariant | 0.000 | ok |
| shr_per_iter | -0.336 | moderate- |

**Consistency summary:**

- **shr_loop_invariant**: won 0/8, lost 8/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 2.3ns | 1.1ns | 213.5% | HIGH |
| shr_loop_invariant | 4.4ns | 2.6ns | 172.0% | HIGH |
| shr_per_iter | 2.7ns | 7.1ns | 38.2% | HIGH |

## Distribution (algo ns)

```
shr_const (n=160, range 0.3-2.0 ns)
      0.3 |
      0.4 |#########################
      0.5 |
      0.6 |
      0.6 |
      0.7 |########################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |
      1.2 |##################################
      1.2 |
      1.3 |
      1.4 |
      1.5 |
      1.6 |
      1.7 |####################
      1.8 |
      1.8 |
      1.9 |
  (8 below, 16 above range)

shr_loop_invariant (n=160, range 1.2-4.2 ns)
      1.2 |
      1.4 |
      1.5 |
      1.7 |################################
      1.8 |
      2.0 |#####################################
      2.1 |
      2.3 |
      2.4 |########################################
      2.6 |
      2.7 |
      2.9 |###################################
      3.0 |
      3.2 |#################
      3.3 |
      3.5 |
      3.6 |################
      3.8 |
      3.9 |
      4.1 |####
  (23 below, 13 above range)

shr_per_iter (n=160, range 4.8-9.5 ns)
      4.8 |########################
      5.1 |
      5.3 |###################
      5.5 |
      5.8 |###########
      6.0 |################################
      6.2 |
      6.5 |##############################
      6.7 |
      6.9 |######################
      7.2 |
      7.4 |########################################
      7.7 |
      7.9 |########################
      8.1 |######################
      8.4 |
      8.6 |###############
      8.8 |
      9.1 |#############
      9.3 |
  (10 below, 15 above range)

```

## Diagnostics

- **shr_const**: CV=58.1% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 6.7x (possible bimodal distribution)
- **shr_const**: bridge=262.5% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=42.4% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=168.0% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=23.3% (high variance, measurements may be unstable)
- **shr_per_iter**: bridge=35.2% of algo (FFI overhead may distort results)
