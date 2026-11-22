# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### popcount_std dominates: 1200% faster than the next best (popcount_handrolled)

popcount_std (0 ns) leads popcount_handrolled (5 ns) by 1200%, a clear separation rather than a photo finish. CV 121.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 96% (significant)

popcount_std is -5 ns (96%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_swar is an outlier: 15.5x slower than the field

popcount_swar (6 ns) is 15.5x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 121.0%)

popcount_std wins on median (0 ns) yet has the highest variance (CV 121.0%), while popcount_swar is the steadiest (CV 26.8%, 6 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### popcount_std shows warm-up / thermal drift (autocorr +0.88)

popcount_std's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 15.5x the fastest

Fastest popcount_std (0 ns) to slowest popcount_swar (6 ns): 15.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_handrolled is inconsistent: worst-20% is 2.6x its best-20%

popcount_handrolled's best 20% of batches run at 3 ns but its worst 20% at 8 ns (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 0.4 ns median (-92.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 15.50x (fastest 0.4 ns, slowest 6.2 ns)

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
| popcount_handrolled | 2129ns | 2127ns | 2116ns | 2127ns | 2149ns | base |
| popcount_std | 2125ns | 2120ns | 2114ns | 2120ns | 2151ns | -0.19% |
| popcount_swar | 2137ns | 2127ns | 2118ns | 2129ns | 2180ns | +0.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 5ns | 3ns | 8ns | base | 12.015 |
| popcount_std | 1ns | 0ns | 1ns | -90.59% | 127.681 |
| popcount_swar | 6ns | 4ns | 9ns | +19.59% | 10.046 |

## Performance model

- Peak throughput: **21.649 Gops/s** (popcount_handrolled; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 12.308 | 56.9% |
| popcount_std | 160.000 | 739.1% |
| popcount_swar | 10.323 | 47.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2129ns | 2129ns | base |
| popcount_std | 2125ns | 2125ns | -0.19% |
| popcount_swar | 2137ns | 2137ns | +0.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 5ns | base | --- | [5, 5] | --- | --- | --- | --- |
| popcount_std | 0ns | -5.0ns (-96.2%) | [-5, -5]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 6ns | +0.9ns (+17.3%) | [+1, +2]ns | [6, 7] | YES | 0.0001 | 0.0001 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 7ns | -94.5% | -8.2% |
| 1 | 2 | 7ns | -94.0% | -7.5% |
| 1 | 3 | 5ns | -91.3% | +34.8% |
| 1 | 4 | 5ns | -91.3% | +40.2% |
| 1 | 5 | 5ns | -92.0% | +34.0% |
| 1 | 6 | 5ns | -91.7% | +39.6% |
| 1 | 7 | 4ns | -90.9% | +46.6% |
| 1 | 8 | 5ns | -92.6% | -7.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | 0.385 | moderate+ |
| popcount_std | 0.875 | HIGH+ (drift/warm-up) |
| popcount_swar | 0.018 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.4ns | 5.3ns | 64.1% | HIGH |
| popcount_std | 2.3ns | 0.5ns | 451.0% | HIGH |
| popcount_swar | 3.0ns | 6.4ns | 46.3% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 3.0-7.8 ns)
      3.0 |
      3.2 |#####################
      3.4 |
      3.7 |#####################
      3.9 |
      4.2 |#####################
      4.4 |#################################
      4.7 |
      4.9 |########################################
      5.1 |
      5.4 |#############################
      5.6 |###############################
      5.9 |
      6.1 |##############
      6.3 |
      6.6 |#########################
      6.8 |
      7.1 |#####################
      7.3 |################
      7.6 |
  (15 below, 14 above range)

popcount_std (n=160, range 0.0-1.3 ns)
      0.0 |############################
      0.1 |
      0.1 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |########################################
      0.4 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.8 |################
      0.8 |
      0.9 |
      1.0 |
      1.0 |
      1.1 |
      1.1 |########
      1.2 |
  (0 below, 10 above range)

popcount_swar (n=160, range 4.0-8.7 ns)
      4.0 |##########
      4.3 |
      4.5 |##############
      4.7 |
      5.0 |########################
      5.2 |####################
      5.4 |
      5.7 |########################################
      5.9 |
      6.1 |##################################
      6.4 |
      6.6 |####################################
      6.8 |
      7.1 |######################
      7.3 |##########################
      7.5 |
      7.7 |######################
      8.0 |
      8.2 |##################
      8.4 |
  (13 below, 14 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=32.2% (high variance, measurements may be unstable)
- **popcount_handrolled**: bridge=63.5% of algo (FFI overhead may distort results)
- **popcount_std**: CV=96.6% (high variance, measurements may be unstable)
- **popcount_std**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **popcount_std**: bridge=525.0% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=26.0% (high variance, measurements may be unstable)
- **popcount_swar**: bridge=46.8% of algo (FFI overhead may distort results)
