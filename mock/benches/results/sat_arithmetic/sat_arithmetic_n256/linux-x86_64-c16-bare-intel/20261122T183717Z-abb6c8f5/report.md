# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (140 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 81 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 72% faster than the next best (sat_saturate)

sat_wrap (81 ns) leads sat_saturate (140 ns) by 72%, a clear separation rather than a photo finish. CV 85.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 42% (significant)

sat_wrap is -58 ns (42%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_checked shows warm-up / thermal drift (autocorr +0.63)

sat_checked's per-pass series has lag-1 autocorrelation +0.63, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (59 ns) is smaller than the fastest variant's own run-to-run std-dev (69 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### sat_saturate is inconsistent: worst-20% is 3.8x its best-20%

sat_saturate's best 20% of batches run at 115 ns but its worst 20% at 430 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### sat_saturate's edge over baseline is significant but tiny (1 ns, 1.04%)

sat_saturate differs from baseline sat_checked by 1 ns (1.04%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: sat_wrap** at 81.2 ns median (-42.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.72x (fastest 81.2 ns, slowest 139.9 ns)

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
| Counter frequency | 2112769626 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 2689ns | 2016ns | 1648ns | 1943ns | 5971ns | base |
| sat_saturate | 2754ns | 2018ns | 1650ns | 1979ns | 6183ns | +2.42% |
| sat_wrap | 2797ns | 1959ns | 1601ns | 2176ns | 5855ns | +3.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 186ns | 114ns | 412ns | base | 1.373 |
| sat_saturate | 191ns | 115ns | 430ns | +2.72% | 1.337 |
| sat_wrap | 116ns | 66ns | 241ns | -38.01% | 2.215 |

## Performance model

- Peak throughput: **3.853 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.829 | 47.5% |
| sat_saturate | 1.830 | 47.5% |
| sat_wrap | 3.153 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2689ns | 2689ns | base |
| sat_saturate | 2754ns | 2754ns | +2.42% |
| sat_wrap | 2797ns | 2797ns | +3.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 140ns | base | --- | [140, 140] | --- | --- | --- | --- |
| sat_saturate | 140ns | +1.5ns (+1.0%) | [+1, +22]ns | [140, 140] | YES | 0.0109 | 0.0109 | 1 |
| sat_wrap | 81ns | -58.3ns (-41.7%) | [-59, -48]ns | [81, 81] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 115ns | +1.5% | -29.0% |
| 1 | 2 | 115ns | +22.3% | -41.9% |
| 1 | 3 | 116ns | +21.3% | -29.8% |
| 1 | 4 | 114ns | +22.2% | -29.1% |
| 1 | 5 | 140ns | +0.0% | -42.1% |
| 1 | 6 | 142ns | -19.1% | -53.1% |
| 1 | 7 | 142ns | -1.0% | -42.7% |
| 1 | 8 | 141ns | -18.6% | -52.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | 0.633 | HIGH+ (drift/warm-up) |
| sat_saturate | -0.281 | moderate- |
| sat_wrap | -0.475 | moderate- |

**Consistency summary:**

- **sat_saturate**: won 3/8, lost 4/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 31.7ns | 186.4ns | 17.0% | HIGH |
| sat_saturate | 32.4ns | 191.5ns | 16.9% | HIGH |
| sat_wrap | 33.9ns | 115.5ns | 29.4% | HIGH |

## Distribution (algo ns)

```
sat_checked (n=160, range 114.1-412.1 ns)
    114.1 |##########################
    129.0 |########################################
    143.9 |#
    158.8 |
    173.7 |
    188.6 |
    203.5 |#
    218.4 |
    233.3 |
    248.2 |
    263.1 |
    278.0 |#
    292.9 |
    307.8 |#
    322.7 |
    337.6 |
    352.5 |#
    367.4 |
    382.3 |#
    397.2 |#
  (14 below, 17 above range)

sat_saturate (n=160, range 114.5-429.8 ns)
    114.5 |##############
    130.3 |########################################
    146.1 |#
    161.8 |
    177.6 |
    193.4 |
    209.1 |
    224.9 |
    240.6 |
    256.4 |
    272.2 |
    287.9 |
    303.7 |
    319.5 |#
    335.2 |
    351.0 |
    366.8 |#
    382.5 |###
    398.3 |
    414.1 |#
  (16 below, 16 above range)

sat_wrap (n=160, range 66.4-241.1 ns)
     66.4 |##########
     75.2 |########################################
     83.9 |
     92.6 |
    101.4 |
    110.1 |#
    118.8 |
    127.6 |
    136.3 |#
    145.0 |
    153.7 |
    162.5 |
    171.2 |##
    179.9 |##
    188.7 |##
    197.4 |#
    206.1 |#
    214.9 |
    223.6 |##
    232.3 |#
  (17 below, 16 above range)

```

## Diagnostics

- **sat_checked**: CV=64.2% (high variance, measurements may be unstable)
- **sat_checked**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sat_checked**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **sat_checked**: bridge=17.0% of algo (FFI overhead may distort results)
- **sat_saturate**: CV=64.9% (high variance, measurements may be unstable)
- **sat_saturate**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **sat_saturate**: bridge=17.0% of algo (FFI overhead may distort results)
- **sat_wrap**: CV=59.8% (high variance, measurements may be unstable)
- **sat_wrap**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sat_wrap**: bridge=29.4% of algo (FFI overhead may distort results)
