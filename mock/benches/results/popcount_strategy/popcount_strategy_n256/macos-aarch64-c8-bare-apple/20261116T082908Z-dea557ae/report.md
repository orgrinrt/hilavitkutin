# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### popcount_std dominates: 992% faster than the next best (popcount_handrolled)

popcount_std (4 ns) leads popcount_handrolled (40 ns) by 992%, a clear separation rather than a photo finish. CV 30.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 92% (significant)

popcount_std is -37 ns (92%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_swar is an outlier: 11.0x slower than the field

popcount_swar (41 ns) is 11.0x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 30.2%)

popcount_std wins on median (4 ns) yet has the highest variance (CV 30.2%), while popcount_swar is the steadiest (CV 12.3%, 41 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 11.0x the fastest

Fastest popcount_std (4 ns) to slowest popcount_swar (41 ns): 11.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 2.3x its best-20%

popcount_std's best 20% of batches run at 2 ns but its worst 20% at 5 ns (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 3.7 ns median (-90.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 11.03x (fastest 3.7 ns, slowest 40.8 ns)

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
| popcount_handrolled | 2145ns | 2141ns | 2131ns | 2141ns | 2173ns | base |
| popcount_std | 2131ns | 2122ns | 2114ns | 2123ns | 2171ns | -0.66% |
| popcount_swar | 2166ns | 2141ns | 2136ns | 2142ns | 2265ns | +0.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 42ns | 36ns | 50ns | base | 6.151 |
| popcount_std | 4ns | 2ns | 5ns | -90.91% | 67.691 |
| popcount_swar | 42ns | 36ns | 50ns | +0.74% | 6.105 |

## Performance model

- Peak throughput: **109.372 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 6.337 | 5.8% |
| popcount_std | 69.189 | 63.3% |
| popcount_swar | 6.275 | 5.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2145ns | 2145ns | base |
| popcount_std | 2131ns | 2131ns | -0.66% |
| popcount_swar | 2166ns | 2166ns | +0.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 40ns | base | --- | [40, 42] | --- | --- | --- | --- |
| popcount_std | 4ns | -37.1ns (-91.8%) | [-38, -36]ns | [3, 4] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 41ns | no significant difference | [-0, +1]ns | [40, 42] | no | 0.2885 | 0.2885 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 40ns | -91.7% | +2.5% |
| 1 | 2 | 42ns | -89.4% | -2.2% |
| 1 | 3 | 41ns | -90.9% | +4.7% |
| 1 | 4 | 40ns | -90.7% | +3.0% |
| 1 | 5 | 42ns | -91.1% | -3.5% |
| 1 | 6 | 41ns | -91.0% | -0.5% |
| 1 | 7 | 40ns | -90.8% | +2.0% |
| 1 | 8 | 41ns | -91.0% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.425 | moderate- |
| popcount_std | -0.482 | moderate- |
| popcount_swar | -0.128 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.3ns | 41.6ns | 7.9% | HIGH |
| popcount_std | 2.6ns | 3.8ns | 69.7% | HIGH |
| popcount_swar | 3.2ns | 41.9ns | 7.6% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 35.7-49.6 ns)
     35.7 |####################
     36.4 |###############
     37.1 |#############
     37.8 |########################################
     38.5 |###############
     39.2 |###############################
     39.8 |######################
     40.5 |#############
     41.2 |###############
     41.9 |#################
     42.6 |#################
     43.3 |######
     44.0 |###########
     44.7 |#################
     45.4 |
     46.1 |####################
     46.8 |##
     47.5 |########
     48.2 |######
     48.9 |##
  (11 below, 15 above range)

popcount_std (n=160, range 2.3-5.4 ns)
      2.3 |
      2.5 |###################
      2.7 |
      2.8 |############################
      3.0 |
      3.1 |
      3.3 |######################################
      3.4 |
      3.6 |########################################
      3.7 |
      3.9 |
      4.0 |################################
      4.2 |
      4.4 |
      4.5 |#################
      4.7 |
      4.8 |
      5.0 |################
      5.1 |
      5.3 |################
  (11 below, 8 above range)

popcount_swar (n=160, range 36.1-49.7 ns)
     36.1 |#########
     36.8 |#######
     37.5 |########################################
     38.1 |##############################
     38.8 |###########
     39.5 |#######################
     40.2 |########################################
     40.9 |################
     41.5 |##############
     42.2 |###########
     42.9 |###########
     43.6 |##################
     44.3 |#########
     44.9 |#########
     45.6 |##################
     46.3 |#########
     47.0 |#########
     47.7 |################
     48.4 |####
     49.0 |##
  (15 below, 11 above range)

```

## Diagnostics

- **popcount_std**: CV=29.6% (high variance, measurements may be unstable)
- **popcount_std**: bridge=67.6% of algo (FFI overhead may distort results)
