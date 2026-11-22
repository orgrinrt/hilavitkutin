# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### popcount_std dominates: 924% faster than the next best (popcount_handrolled)

popcount_std (4 ns) leads popcount_handrolled (38 ns) by 924%, a clear separation rather than a photo finish. CV 31.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 90% (significant)

popcount_std is -34 ns (90%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_swar is an outlier: 10.3x slower than the field

popcount_swar (38 ns) is 10.3x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 31.7%)

popcount_std wins on median (4 ns) yet has the highest variance (CV 31.7%), while popcount_handrolled is the steadiest (CV 13.0%, 38 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### popcount_handrolled shows alternating (throttle bounce) (autocorr -0.65)

popcount_handrolled's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.3x the fastest

Fastest popcount_std (4 ns) to slowest popcount_swar (38 ns): 10.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 2.4x its best-20%

popcount_std's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 3.7 ns median (-90.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 10.30x (fastest 3.7 ns, slowest 38.1 ns)

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
| popcount_handrolled | 2130ns | 2116ns | 2107ns | 2117ns | 2191ns | base |
| popcount_std | 2116ns | 2097ns | 2090ns | 2101ns | 2187ns | -0.67% |
| popcount_swar | 2146ns | 2116ns | 2108ns | 2119ns | 2263ns | +0.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 39ns | 33ns | 46ns | base | 6.620 |
| popcount_std | 4ns | 2ns | 6ns | -90.03% | 66.418 |
| popcount_swar | 39ns | 32ns | 47ns | +0.88% | 6.563 |

## Performance model

- Peak throughput: **109.960 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 6.755 | 6.1% |
| popcount_std | 69.189 | 62.9% |
| popcount_swar | 6.719 | 6.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2130ns | 2130ns | base |
| popcount_std | 2116ns | 2116ns | -0.67% |
| popcount_swar | 2146ns | 2146ns | +0.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 38ns | base | --- | [37, 39] | --- | --- | --- | --- |
| popcount_std | 4ns | -34.2ns (-90.2%) | [-35, -33]ns | [4, 4] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 38ns | no significant difference | [-0, +1]ns | [37, 40] | no | 0.1733 | 0.1733 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 39ns | -90.5% | +1.0% |
| 1 | 2 | 37ns | -91.2% | +0.0% |
| 1 | 3 | 38ns | -90.4% | -1.7% |
| 1 | 4 | 39ns | -90.4% | +5.8% |
| 1 | 5 | 37ns | -90.5% | -1.2% |
| 1 | 6 | 39ns | -89.8% | -0.5% |
| 1 | 7 | 37ns | -89.4% | +5.7% |
| 1 | 8 | 39ns | -89.2% | -5.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.654 | HIGH- (thermal bounce) |
| popcount_std | 0.252 | moderate+ |
| popcount_swar | -0.530 | HIGH- (thermal bounce) |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.5ns | 38.7ns | 9.1% | HIGH |
| popcount_std | 3.1ns | 3.9ns | 79.8% | HIGH |
| popcount_swar | 3.5ns | 39.0ns | 9.0% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 32.6-46.1 ns)
     32.6 |####################
     33.3 |####################################
     34.0 |##############################
     34.6 |####################
     35.3 |################
     36.0 |#######################
     36.7 |########################################
     37.3 |####################################
     38.0 |##########
     38.7 |########################################
     39.4 |####################
     40.0 |################
     40.7 |##############################
     41.4 |##########
     42.1 |#######################
     42.7 |#######################
     43.4 |###
     44.1 |##########################
     44.7 |##########
     45.4 |######
  (15 below, 12 above range)

popcount_std (n=160, range 2.3-5.5 ns)
      2.3 |
      2.5 |################
      2.6 |
      2.8 |##########################
      3.0 |
      3.1 |
      3.3 |########################################
      3.4 |
      3.6 |###################################
      3.8 |
      3.9 |
      4.1 |#############################
      4.2 |
      4.4 |
      4.6 |########################################
      4.7 |
      4.9 |#########
      5.0 |
      5.2 |
      5.4 |############
  (12 below, 12 above range)

popcount_swar (n=160, range 32.4-47.0 ns)
     32.4 |###########
     33.1 |##############
     33.9 |##################
     34.6 |################
     35.3 |########################################
     36.0 |##############
     36.8 |############################
     37.5 |##############
     38.2 |#########
     39.0 |################
     39.7 |#####################
     40.4 |##################
     41.1 |##################
     41.9 |#####################
     42.6 |################
     43.3 |##
     44.1 |####
     44.8 |###########
     45.5 |###########
     46.2 |##
  (13 below, 14 above range)

```

## Diagnostics

- **popcount_std**: CV=30.4% (high variance, measurements may be unstable)
- **popcount_std**: bridge=78.4% of algo (FFI overhead may distort results)
