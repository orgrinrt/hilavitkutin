# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### minmax_branchless is fastest but the noisiest (CV 120.1%)

minmax_branchless wins on median (28 ns) yet has the highest variance (CV 120.1%), while minmax_intrinsic is the steadiest (CV 89.6%, 34 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### minmax_intrinsic shows alternating (throttle bounce) (autocorr -0.63)

minmax_intrinsic's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (33 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### minmax_branchless's comparison is tie-heavy (22% tied pairs)

22% of paired samples for minmax_branchless are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### minmax_branch is inconsistent: worst-20% is 3.9x its best-20%

minmax_branch's best 20% of batches run at 28 ns but its worst 20% at 109 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: minmax_branchless** at 27.8 ns median (-17.5% vs baseline)
- Spread: 1.22x (fastest 27.8 ns, slowest 33.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5d49a3f1b7c06e82 |
| Counter frequency | 2112775109 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2625ns | 1916ns | 1566ns | 1825ns | 6086ns | base |
| minmax_branchless | 2583ns | 1581ns | 1567ns | 1792ns | 5973ns | -1.60% |
| minmax_intrinsic | 2661ns | 1917ns | 1568ns | 1901ns | 6034ns | +1.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 47ns | 28ns | 109ns | base | 5.494 |
| minmax_branchless | 46ns | 28ns | 108ns | -0.32% | 5.512 |
| minmax_intrinsic | 47ns | 28ns | 105ns | +0.14% | 5.486 |

## Performance model

- Peak throughput: **9.269 Gops/s** (minmax_branchless; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 7.596 | 82.0% |
| minmax_branchless | 9.209 | 99.3% |
| minmax_intrinsic | 7.574 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2625ns | 2625ns | base |
| minmax_branchless | 2583ns | 2583ns | -1.60% |
| minmax_intrinsic | 2661ns | 2661ns | +1.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 34ns | base | --- | [28, 34] | --- | --- | --- | --- |
| minmax_branchless | 28ns | no significant difference | [-0, +0]ns | [28, 34] | no (adj: YES) | 0.0179 | 0.0089 | **36** (22%, HIGH) |
| minmax_intrinsic | 34ns | no significant difference | [-0, +6]ns | [34, 34] | no | 0.2948 | 0.2948 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 34ns | -18.3% | -17.6% |
| 1 | 2 | 28ns | +0.0% | +22.0% |
| 1 | 3 | 28ns | +0.0% | +0.0% |
| 1 | 4 | 28ns | +0.0% | +22.4% |
| 1 | 5 | 34ns | -18.2% | -18.2% |
| 1 | 6 | 28ns | +22.4% | +22.4% |
| 1 | 7 | 28ns | +0.0% | +22.4% |
| 1 | 8 | 34ns | +0.0% | -18.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.274 | moderate- |
| minmax_branchless | -0.208 | moderate- |
| minmax_intrinsic | -0.626 | HIGH- (thermal bounce) |

**Consistency summary:**

- **minmax_branchless**: won 2/8, lost 1/8
- **minmax_intrinsic**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 31.7ns | 46.6ns | 68.1% | HIGH |
| minmax_branchless | 31.3ns | 46.4ns | 67.3% | HIGH |
| minmax_intrinsic | 32.3ns | 46.7ns | 69.2% | HIGH |

## Distribution (algo ns)

```
minmax_branch (n=160, range 27.6-108.9 ns)
     27.6 |########################################
     31.7 |##############################
     35.8 |
     39.8 |
     43.9 |
     47.9 |
     52.0 |
     56.1 |
     60.1 |
     64.2 |
     68.2 |
     72.3 |
     76.4 |
     80.4 |
     84.5 |#
     88.5 |#
     92.6 |
     96.7 |##
    100.7 |####
    104.8 |
  (18 below, 16 above range)

minmax_branchless (n=160, range 27.6-108.2 ns)
     27.6 |########################################
     31.6 |###################
     35.7 |
     39.7 |
     43.7 |#
     47.8 |
     51.8 |
     55.8 |
     59.9 |
     63.9 |
     67.9 |
     71.9 |
     76.0 |
     80.0 |
     84.0 |#
     88.1 |
     92.1 |##
     96.1 |#
    100.1 |####
    104.2 |
  (24 below, 14 above range)

minmax_intrinsic (n=160, range 27.7-105.1 ns)
     27.7 |##########################
     31.5 |########################################
     35.4 |
     39.3 |#
     43.1 |
     47.0 |
     50.9 |
     54.7 |
     58.6 |
     62.5 |
     66.4 |
     70.2 |
     74.1 |
     78.0 |
     81.8 |
     85.7 |
     89.6 |
     93.5 |#####
     97.3 |##
    101.2 |###
  (13 below, 12 above range)

```

## Diagnostics

- **minmax_branch**: CV=69.9% (high variance, measurements may be unstable)
- **minmax_branch**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **minmax_branch**: bridge=68.8% of algo (FFI overhead may distort results)
- **minmax_branchless**: CV=71.9% (high variance, measurements may be unstable)
- **minmax_branchless**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **minmax_branchless**: bridge=69.1% of algo (FFI overhead may distort results)
- **minmax_intrinsic**: CV=64.9% (high variance, measurements may be unstable)
- **minmax_intrinsic**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **minmax_intrinsic**: bridge=68.9% of algo (FFI overhead may distort results)
