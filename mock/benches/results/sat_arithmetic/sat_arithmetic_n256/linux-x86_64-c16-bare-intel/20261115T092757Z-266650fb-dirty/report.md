# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 72% faster than the next best (sat_checked)

sat_wrap (81 ns) leads sat_checked (140 ns) by 72%, a clear separation rather than a photo finish. CV 157.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 42% (significant)

sat_wrap is -58 ns (42%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap is fastest but the noisiest (CV 157.7%)

sat_wrap wins on median (81 ns) yet has the highest variance (CV 157.7%), while sat_checked is the steadiest (CV 85.5%, 140 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (58 ns) is smaller than the fastest variant's own run-to-run std-dev (128 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### sat_wrap is inconsistent: worst-20% is 4.3x its best-20%

sat_wrap's best 20% of batches run at 66 ns but its worst 20% at 284 ns (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sat_wrap** at 81.2 ns median (-41.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.72x (fastest 81.2 ns, slowest 139.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Overflow strategy: wrapping_add vs saturating_add vs checked_add |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x52b9de7c318fa064 |
| Counter frequency | 2112797930 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 2776ns | 2016ns | 1649ns | 2045ns | 6099ns | base |
| sat_saturate | 2815ns | 2017ns | 1842ns | 2025ns | 6158ns | +1.39% |
| sat_wrap | 2765ns | 1957ns | 1600ns | 2078ns | 5991ns | -0.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 192ns | 114ns | 419ns | base | 1.335 |
| sat_saturate | 196ns | 127ns | 430ns | +2.10% | 1.308 |
| sat_wrap | 121ns | 66ns | 284ns | -36.95% | 2.118 |

## Performance model

- Peak throughput: **3.854 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.834 | 47.6% |
| sat_saturate | 1.834 | 47.6% |
| sat_wrap | 3.153 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2776ns | 2776ns | base |
| sat_saturate | 2815ns | 2815ns | +1.39% |
| sat_wrap | 2765ns | 2765ns | -0.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 140ns | base | --- | [139, 140] | --- | --- | --- | --- |
| sat_saturate | 140ns | no significant difference | [-0, +2]ns | [139, 140] | no | 0.8126 | 0.8126 | 0 |
| sat_wrap | 81ns | -58.5ns (-41.9%) | [-61, -58]ns | [81, 81] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 143ns | -0.5% | -53.4% |
| 1 | 2 | 140ns | +1.2% | -52.3% |
| 1 | 3 | 139ns | -17.7% | -41.6% |
| 1 | 4 | 114ns | +22.0% | -8.7% |
| 1 | 5 | 114ns | +21.9% | -41.8% |
| 1 | 6 | 140ns | -0.2% | -41.9% |
| 1 | 7 | 140ns | +0.4% | -41.8% |
| 1 | 8 | 114ns | +22.6% | -28.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | 0.075 | ok |
| sat_saturate | -0.170 | ok |
| sat_wrap | -0.131 | ok |

**Consistency summary:**

- **sat_saturate**: won 3/8, lost 5/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 32.5ns | 191.7ns | 17.0% | HIGH |
| sat_saturate | 33.1ns | 195.8ns | 16.9% | HIGH |
| sat_wrap | 33.5ns | 120.9ns | 27.7% | HIGH |

## Distribution (algo ns)

```
sat_checked (n=160, range 113.7-419.2 ns)
    113.7 |###########
    129.0 |########################################
    144.3 |
    159.5 |
    174.8 |
    190.1 |
    205.4 |
    220.6 |
    235.9 |
    251.2 |
    266.5 |
    281.7 |
    297.0 |
    312.3 |
    327.6 |
    342.8 |#
    358.1 |
    373.4 |
    388.7 |
    403.9 |
  (18 below, 17 above range)

sat_saturate (n=160, range 127.1-429.8 ns)
    127.1 |########################################
    142.2 |#####
    157.3 |
    172.5 |
    187.6 |
    202.8 |
    217.9 |
    233.0 |
    248.2 |
    263.3 |
    278.4 |
    293.6 |
    308.7 |
    323.9 |
    339.0 |
    354.1 |
    369.3 |
    384.4 |
    399.5 |
    414.7 |#
  (15 below, 18 above range)

sat_wrap (n=160, range 66.4-283.7 ns)
     66.4 |###################
     77.3 |########################################
     88.2 |#
     99.0 |####
    109.9 |
    120.7 |#
    131.6 |###
    142.5 |
    153.3 |
    164.2 |
    175.1 |
    185.9 |
    196.8 |
    207.6 |##
    218.5 |###
    229.4 |#
    240.2 |##
    251.1 |#
    262.0 |##
    272.8 |#
  (22 below, 7 above range)

```

## Diagnostics

- **sat_checked**: CV=62.2% (high variance, measurements may be unstable)
- **sat_checked**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **sat_checked**: bridge=17.1% of algo (FFI overhead may distort results)
- **sat_saturate**: CV=63.8% (high variance, measurements may be unstable)
- **sat_saturate**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sat_saturate**: bridge=17.0% of algo (FFI overhead may distort results)
- **sat_wrap**: CV=105.9% (high variance, measurements may be unstable)
- **sat_wrap**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **sat_wrap**: bridge=29.3% of algo (FFI overhead may distort results)
