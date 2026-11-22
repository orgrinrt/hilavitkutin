# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### popcount_std dominates: 919% faster than the next best (popcount_handrolled)

popcount_std (4 ns) leads popcount_handrolled (38 ns) by 919%, a clear separation rather than a photo finish. CV 33.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 91% (significant)

popcount_std is -34 ns (91%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_swar is an outlier: 10.2x slower than the field

popcount_swar (38 ns) is 10.2x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 33.1%)

popcount_std wins on median (4 ns) yet has the highest variance (CV 33.1%), while popcount_handrolled is the steadiest (CV 13.4%, 38 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 10.2x the fastest

Fastest popcount_std (4 ns) to slowest popcount_swar (38 ns): 10.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 2.6x its best-20%

popcount_std's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 3.7 ns median (-90.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 10.24x (fastest 3.7 ns, slowest 37.9 ns)

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
| popcount_handrolled | 2118ns | 2113ns | 2109ns | 2113ns | 2142ns | base |
| popcount_std | 2102ns | 2096ns | 2091ns | 2096ns | 2130ns | -0.76% |
| popcount_swar | 2121ns | 2116ns | 2109ns | 2116ns | 2147ns | +0.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 38ns | 32ns | 46ns | base | 6.652 |
| popcount_std | 4ns | 2ns | 6ns | -90.16% | 67.635 |
| popcount_swar | 39ns | 32ns | 46ns | +0.47% | 6.621 |

## Performance model

- Peak throughput: **119.070 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 6.790 | 5.7% |
| popcount_std | 69.189 | 58.1% |
| popcount_swar | 6.755 | 5.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2118ns | 2118ns | base |
| popcount_std | 2102ns | 2102ns | -0.76% |
| popcount_swar | 2121ns | 2121ns | +0.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 38ns | base | --- | [37, 39] | --- | --- | --- | --- |
| popcount_std | 4ns | -34.2ns (-90.7%) | [-35, -33]ns | [3, 4] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 38ns | no significant difference | [-0, +1]ns | [37, 39] | no | 0.5703 | 0.5703 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 38ns | -88.5% | +0.1% |
| 1 | 2 | 38ns | -90.3% | +1.3% |
| 1 | 3 | 38ns | -88.9% | -1.1% |
| 1 | 4 | 38ns | -90.2% | -2.1% |
| 1 | 5 | 38ns | -91.2% | +0.0% |
| 1 | 6 | 38ns | -89.5% | +3.3% |
| 1 | 7 | 37ns | -91.2% | +1.1% |
| 1 | 8 | 39ns | -92.1% | -3.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.140 | ok |
| popcount_std | 0.027 | ok |
| popcount_swar | 0.036 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.5ns | 38.5ns | 9.1% | HIGH |
| popcount_std | 3.0ns | 3.8ns | 79.6% | HIGH |
| popcount_swar | 3.4ns | 38.7ns | 8.7% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 32.1-46.3 ns)
     32.1 |##########
     32.8 |########################################
     33.5 |##############
     34.3 |#####################
     35.0 |####################################
     35.7 |####################################
     36.4 |################################
     37.1 |####################################
     37.8 |################################
     38.5 |################################
     39.2 |#####################
     39.9 |##############
     40.6 |####################################
     41.3 |#########################
     42.0 |##########
     42.8 |#####################
     43.5 |###
     44.2 |##################
     44.9 |#######
     45.6 |##################
  (17 below, 13 above range)

popcount_std (n=160, range 2.2-5.6 ns)
      2.2 |
      2.3 |
      2.5 |#######################
      2.7 |
      2.8 |#############################
      3.0 |
      3.2 |################################
      3.3 |
      3.5 |
      3.7 |########################################
      3.9 |
      4.0 |####################################
      4.2 |
      4.4 |
      4.5 |#######################
      4.7 |
      4.9 |###########################
      5.1 |
      5.2 |##############
      5.4 |
  (22 below, 13 above range)

popcount_swar (n=160, range 32.3-46.4 ns)
     32.3 |#############
     33.0 |#############
     33.7 |##################
     34.4 |########################################
     35.1 |##########################
     35.8 |#####
     36.5 |#####################################
     37.2 |##########################
     37.9 |########
     38.6 |#############################
     39.3 |################
     40.1 |########
     40.8 |#############
     41.5 |##################################
     42.2 |########
     42.9 |################
     43.6 |##########
     44.3 |##########
     45.0 |#############
     45.7 |#####
  (15 below, 12 above range)

```

## Diagnostics

- **popcount_std**: CV=32.4% (high variance, measurements may be unstable)
- **popcount_std**: bridge=78.4% of algo (FFI overhead may distort results)
