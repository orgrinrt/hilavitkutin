# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### popcount_std dominates: 1050% faster than the next best (popcount_handrolled)

popcount_std (0 ns) leads popcount_handrolled (5 ns) by 1050%, a clear separation rather than a photo finish. CV 102.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 83% (significant)

popcount_std is -4 ns (83%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_swar is an outlier: 11.5x slower than the field

popcount_swar (5 ns) is 11.5x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 102.5%)

popcount_std wins on median (0 ns) yet has the highest variance (CV 102.5%), while popcount_swar is the steadiest (CV 28.5%, 5 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### popcount_swar's comparison is tie-heavy (13% tied pairs)

13% of paired samples for popcount_swar are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Wide spread: slowest is 11.5x the fastest

Fastest popcount_std (0 ns) to slowest popcount_swar (5 ns): 11.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar is inconsistent: worst-20% is 2.4x its best-20%

popcount_swar's best 20% of batches run at 3 ns but its worst 20% at 6 ns (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 0.4 ns median (-91.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 11.50x (fastest 0.4 ns, slowest 4.6 ns)

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
| popcount_handrolled | 2113ns | 2099ns | 2092ns | 2102ns | 2168ns | base |
| popcount_std | 2104ns | 2095ns | 2088ns | 2095ns | 2149ns | -0.44% |
| popcount_swar | 2105ns | 2100ns | 2095ns | 2100ns | 2129ns | -0.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 5ns | 3ns | 7ns | base | 13.775 |
| popcount_std | 0ns | 0ns | 1ns | -90.44% | 144.023 |
| popcount_swar | 4ns | 3ns | 6ns | -5.29% | 14.543 |

## Performance model

- Peak throughput: **24.764 Gops/s** (popcount_swar; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 13.913 | 56.2% |
| popcount_std | 160.000 | 646.1% |
| popcount_swar | 13.913 | 56.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2113ns | 2113ns | base |
| popcount_std | 2104ns | 2104ns | -0.44% |
| popcount_swar | 2105ns | 2105ns | -0.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 5ns | base | --- | [4, 5] | --- | --- | --- | --- |
| popcount_std | 0ns | -3.8ns (-82.6%) | [-4, -4]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 5ns | no significant difference | [-0, +0]ns | [4, 5] | no | 0.3964 | 0.3964 | **21** (13%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 5ns | -91.3% | -8.7% |
| 1 | 2 | 4ns | -90.5% | +0.0% |
| 1 | 3 | 4ns | -100.0% | +13.6% |
| 1 | 4 | 4ns | -90.9% | +4.5% |
| 1 | 5 | 5ns | -91.3% | -4.3% |
| 1 | 6 | 5ns | -92.0% | -4.0% |
| 1 | 7 | 5ns | -92.6% | -11.1% |
| 1 | 8 | 4ns | -90.5% | +4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | 0.100 | ok |
| popcount_std | -0.161 | ok |
| popcount_swar | -0.052 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.7ns | 4.6ns | 79.9% | HIGH |
| popcount_std | 2.6ns | 0.4ns | 587.8% | HIGH |
| popcount_swar | 3.7ns | 4.4ns | 84.4% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 2.9-6.7 ns)
      2.9 |########################
      3.1 |
      3.3 |##########################
      3.5 |
      3.7 |########################################
      3.8 |
      4.0 |########################################
      4.2 |
      4.4 |####################################
      4.6 |
      4.8 |
      5.0 |######################################
      5.2 |
      5.4 |######################################
      5.6 |
      5.7 |##################
      5.9 |
      6.1 |########
      6.3 |
      6.5 |
  (8 below, 18 above range)

popcount_std (n=160, range 0.0-1.1 ns)
      0.0 |############################
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |########################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.8 |###############
      0.8 |
      0.9 |
      0.9 |
      1.0 |
      1.0 |
  (0 below, 17 above range)

popcount_swar (n=160, range 2.6-6.2 ns)
      2.6 |
      2.8 |##################
      2.9 |
      3.1 |#######################
      3.3 |
      3.5 |
      3.7 |########################
      3.9 |
      4.0 |###########################
      4.2 |
      4.4 |
      4.6 |##########################
      4.8 |
      4.9 |########################################
      5.1 |
      5.3 |#######################
      5.5 |
      5.7 |###############
      5.9 |
      6.0 |###
  (16 below, 13 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=29.3% (high variance, measurements may be unstable)
- **popcount_handrolled**: bridge=80.4% of algo (FFI overhead may distort results)
- **popcount_std**: CV=92.3% (high variance, measurements may be unstable)
- **popcount_std**: bridge=625.0% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=29.8% (high variance, measurements may be unstable)
- **popcount_swar**: bridge=80.4% of algo (FFI overhead may distort results)
