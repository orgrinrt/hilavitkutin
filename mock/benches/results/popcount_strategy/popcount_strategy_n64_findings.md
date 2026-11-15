# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### popcount_std dominates: 1150% faster than the next best (popcount_handrolled)

popcount_std (0 ns) leads popcount_handrolled (5 ns) by 1150%, a clear separation rather than a photo finish. CV 111.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 92% (significant)

popcount_std is -5 ns (92%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_swar is an outlier: 12.5x slower than the field

popcount_swar (5 ns) is 12.5x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 111.4%)

popcount_std wins on median (0 ns) yet has the highest variance (CV 111.4%), while popcount_handrolled is the steadiest (CV 30.6%, 5 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### popcount_std shows alternating (throttle bounce) (autocorr -0.68)

popcount_std's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12.5x the fastest

Fastest popcount_std (0 ns) to slowest popcount_swar (5 ns): 12.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar is inconsistent: worst-20% is 2.5x its best-20%

popcount_swar's best 20% of batches run at 3 ns but its worst 20% at 8 ns (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 0.4 ns median (-92.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 12.50x (fastest 0.4 ns, slowest 5.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Popcount lowering: std count_ones vs Kernighan loop vs SWAR |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x24680123456789ab |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 2273ns | 2247ns | 2086ns | 2287ns | 2421ns | base |
| popcount_std | 2328ns | 2395ns | 2162ns | 2353ns | 2419ns | +2.41% |
| popcount_swar | 2160ns | 2108ns | 2075ns | 2124ns | 2350ns | -5.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 5ns | 3ns | 7ns | base | 12.296 |
| popcount_std | 1ns | 0ns | 1ns | -89.65% | 118.794 |
| popcount_swar | 5ns | 3ns | 8ns | -0.25% | 12.327 |

## Performance model

- Peak throughput: **20.237 Gops/s** (popcount_swar; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 12.800 | 63.2% |
| popcount_std | 160.000 | 790.6% |
| popcount_swar | 12.800 | 63.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2273ns | 2273ns | base |
| popcount_std | 2328ns | 2328ns | +2.41% |
| popcount_swar | 2160ns | 2160ns | -5.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 5ns | base | --- | [5, 5] | --- | --- | --- | --- |
| popcount_std | 0ns | -4.6ns (-92.0%) | [-5, -4]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 5ns | no significant difference | [-0, +0]ns | [5, 5] | no | 0.9354 | 0.9354 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 6ns | -93.5% | -25.8% |
| 1 | 2 | 5ns | -84.6% | -3.8% |
| 1 | 3 | 5ns | -92.6% | -31.5% |
| 1 | 4 | 5ns | -82.6% | +8.7% |
| 1 | 5 | 5ns | -92.6% | +24.1% |
| 1 | 6 | 5ns | -82.6% | +50.0% |
| 1 | 7 | 5ns | -91.7% | -4.2% |
| 1 | 8 | 5ns | -91.3% | -14.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.004 | ok |
| popcount_std | -0.675 | HIGH- (thermal bounce) |
| popcount_swar | 0.290 | moderate+ |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.9ns | 5.2ns | 75.1% | HIGH |
| popcount_std | 2.8ns | 0.5ns | 523.7% | HIGH |
| popcount_swar | 3.5ns | 5.2ns | 66.4% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 3.2-7.4 ns)
      3.2 |#######################
      3.4 |
      3.6 |#########################
      3.9 |
      4.1 |###################################
      4.3 |
      4.5 |#################################
      4.7 |
      4.9 |#####################################
      5.1 |
      5.3 |#####################################
      5.5 |
      5.8 |#############################
      6.0 |
      6.2 |########################################
      6.4 |
      6.6 |############
      6.8 |
      7.0 |####
      7.2 |
  (10 below, 17 above range)

popcount_std (n=160, range 0.0-1.2 ns)
      0.0 |###############################
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.4 |########################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.8 |###################################
      0.8 |
      0.9 |
      0.9 |
      1.0 |
      1.1 |
      1.1 |
  (0 below, 21 above range)

popcount_swar (n=160, range 3.2-7.9 ns)
      3.2 |##########################
      3.4 |
      3.6 |##############################
      3.9 |
      4.1 |########################################
      4.4 |
      4.6 |################################
      4.8 |##############################
      5.1 |
      5.3 |##############################
      5.6 |
      5.8 |############################
      6.0 |##########
      6.3 |
      6.5 |####################
      6.7 |
      7.0 |####
      7.2 |
      7.5 |################
      7.7 |########
  (12 below, 11 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=29.4% (high variance, measurements may be unstable)
- **popcount_handrolled**: bridge=84.0% of algo (FFI overhead may distort results)
- **popcount_std**: CV=82.7% (high variance, measurements may be unstable)
- **popcount_std**: bridge=725.0% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=33.3% (high variance, measurements may be unstable)
- **popcount_swar**: bridge=66.0% of algo (FFI overhead may distort results)
