# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 75% faster than the next best (sat_checked)

sat_wrap (67 ns) leads sat_checked (117 ns) by 75%, a clear separation rather than a photo finish. CV 103.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 43% (significant)

sat_wrap is -50 ns (43%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap is fastest but the noisiest (CV 103.7%)

sat_wrap wins on median (67 ns) yet has the highest variance (CV 103.7%), while sat_checked is the steadiest (CV 99.8%, 117 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (50 ns) is smaller than the fastest variant's own run-to-run std-dev (69 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### sat_saturate is inconsistent: worst-20% is 3.6x its best-20%

sat_saturate's best 20% of batches run at 114 ns but its worst 20% at 409 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sat_wrap** at 66.6 ns median (-43.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.75x (fastest 66.6 ns, slowest 116.8 ns)

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
| Counter frequency | 2112805263 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 2728ns | 1670ns | 1654ns | 2048ns | 5845ns | base |
| sat_saturate | 2695ns | 1668ns | 1654ns | 1956ns | 5955ns | -1.22% |
| sat_wrap | 2640ns | 1611ns | 1605ns | 1962ns | 5708ns | -3.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 189ns | 115ns | 402ns | base | 1.353 |
| sat_saturate | 186ns | 114ns | 409ns | -1.59% | 1.375 |
| sat_wrap | 108ns | 66ns | 233ns | -42.68% | 2.361 |

## Performance model

- Peak throughput: **3.857 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 2.193 | 56.9% |
| sat_saturate | 2.192 | 56.8% |
| sat_wrap | 3.844 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2728ns | 2728ns | base |
| sat_saturate | 2695ns | 2695ns | -1.22% |
| sat_wrap | 2640ns | 2640ns | -3.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 117ns | base | --- | [116, 140] | --- | --- | --- | --- |
| sat_saturate | 117ns | no significant difference | [-1, +0]ns | [116, 139] | no | 0.3010 | 0.3010 | 2 |
| sat_wrap | 67ns | -50.1ns (-42.9%) | [-62, -50]ns | [66, 67] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 140ns | -17.0% | -52.5% |
| 1 | 2 | 116ns | -1.5% | -42.9% |
| 1 | 3 | 117ns | +0.1% | -42.9% |
| 1 | 4 | 117ns | +21.1% | -30.3% |
| 1 | 5 | 115ns | +1.4% | -42.3% |
| 1 | 6 | 115ns | -0.3% | -42.1% |
| 1 | 7 | 142ns | -1.7% | -53.3% |
| 1 | 8 | 116ns | -0.1% | -42.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.216 | moderate- |
| sat_saturate | -0.347 | moderate- |
| sat_wrap | -0.165 | ok |

**Consistency summary:**

- **sat_saturate**: won 4/8, lost 3/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 32.0ns | 189.2ns | 16.9% | HIGH |
| sat_saturate | 32.1ns | 186.2ns | 17.2% | HIGH |
| sat_wrap | 32.2ns | 108.4ns | 29.7% | HIGH |

## Distribution (algo ns)

```
sat_checked (n=160, range 114.5-402.2 ns)
    114.5 |########################################
    128.9 |################
    143.3 |#
    157.7 |
    172.1 |
    186.4 |
    200.8 |
    215.2 |
    229.6 |
    244.0 |
    258.4 |
    272.8 |
    287.1 |#
    301.5 |
    315.9 |
    330.3 |#
    344.7 |###
    359.1 |##
    373.5 |##
    387.9 |###
  (14 below, 14 above range)

sat_saturate (n=160, range 114.2-408.5 ns)
    114.2 |########################################
    128.9 |###################
    143.6 |
    158.4 |#
    173.1 |
    187.8 |
    202.5 |
    217.2 |
    231.9 |#
    246.6 |
    261.4 |
    276.1 |
    290.8 |
    305.5 |
    320.2 |
    334.9 |
    349.6 |####
    364.4 |##
    379.1 |###
    393.8 |
  (17 below, 16 above range)

sat_wrap (n=160, range 66.4-232.5 ns)
     66.4 |########################################
     74.7 |#######
     83.0 |
     91.3 |
     99.6 |
    107.9 |
    116.2 |
    124.5 |
    132.8 |
    141.1 |
    149.4 |
    157.8 |
    166.1 |
    174.4 |
    182.7 |##
    191.0 |##
    199.3 |#
    207.6 |##
    215.9 |#
    224.2 |##
  (7 below, 12 above range)

```

## Diagnostics

- **sat_checked**: CV=61.6% (high variance, measurements may be unstable)
- **sat_checked**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sat_checked**: bridge=16.9% of algo (FFI overhead may distort results)
- **sat_saturate**: CV=63.2% (high variance, measurements may be unstable)
- **sat_saturate**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sat_saturate**: bridge=16.9% of algo (FFI overhead may distort results)
- **sat_wrap**: CV=63.7% (high variance, measurements may be unstable)
- **sat_wrap**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sat_wrap**: bridge=29.4% of algo (FFI overhead may distort results)
