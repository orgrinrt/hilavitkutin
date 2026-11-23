# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 72% faster than the next best (sat_checked)

sat_wrap (295 ns) leads sat_checked (507 ns) by 72%, a clear separation rather than a photo finish. CV 80.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 41% (significant)

sat_wrap is -210 ns (41%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap is fastest but the noisiest (CV 80.3%)

sat_wrap wins on median (295 ns) yet has the highest variance (CV 80.3%), while sat_saturate is the steadiest (CV 68.9%, 509 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sat_wrap shows warm-up / thermal drift (autocorr +0.63)

sat_wrap's per-pass series has lag-1 autocorrelation +0.63, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (214 ns) is smaller than the fastest variant's own run-to-run std-dev (237 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### sat_checked is inconsistent: worst-20% is 3.0x its best-20%

sat_checked's best 20% of batches run at 412 ns but its worst 20% at 1.23 us (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sat_wrap** at 295.1 ns median (-41.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.72x (fastest 295.1 ns, slowest 508.6 ns)

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
| sat_checked | 2952ns | 2385ns | 1949ns | 2325ns | 5838ns | base |
| sat_saturate | 2947ns | 2385ns | 1967ns | 2386ns | 5613ns | -0.17% |
| sat_wrap | 2956ns | 2175ns | 2169ns | 2178ns | 6075ns | +0.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 625ns | 412ns | 1228ns | base | 1.639 |
| sat_saturate | 625ns | 418ns | 1183ns | -0.04% | 1.639 |
| sat_wrap | 399ns | 295ns | 817ns | -36.07% | 2.563 |

## Performance model

- Peak throughput: **3.472 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 2.021 | 58.2% |
| sat_saturate | 2.013 | 58.0% |
| sat_wrap | 3.471 | 100.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2952ns | 2952ns | base |
| sat_saturate | 2947ns | 2947ns | -0.17% |
| sat_wrap | 2956ns | 2956ns | +0.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 507ns | base | --- | [505, 509] | --- | --- | --- | --- |
| sat_saturate | 509ns | no significant difference | [-2, +9]ns | [508, 510] | no | 0.3041 | 0.3041 | 0 |
| sat_wrap | 295ns | -210.2ns (-41.5%) | [-213, -207]ns | [295, 295] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 415ns | +22.5% | -28.9% |
| 1 | 2 | 508ns | -0.1% | -42.0% |
| 1 | 3 | 413ns | +23.5% | -28.6% |
| 1 | 4 | 510ns | -0.0% | -42.2% |
| 1 | 5 | 508ns | +0.2% | -41.9% |
| 1 | 6 | 510ns | -18.3% | -42.1% |
| 1 | 7 | 417ns | +22.1% | -29.3% |
| 1 | 8 | 511ns | -18.2% | -42.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.481 | moderate- |
| sat_saturate | -0.212 | moderate- |
| sat_wrap | 0.625 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **sat_saturate**: won 2/8, lost 4/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 29.4ns | 624.9ns | 4.7% |  |
| sat_saturate | 29.3ns | 624.6ns | 4.7% |  |
| sat_wrap | 32.2ns | 399.5ns | 8.1% | HIGH |

## Distribution (algo ns)

```
sat_checked (n=160, range 412.4-1228.1 ns)
    412.4 |##############
    453.2 |
    494.0 |########################################
    534.7 |
    575.5 |
    616.3 |
    657.1 |
    697.9 |
    738.7 |
    779.5 |
    820.3 |
    861.0 |
    901.8 |
    942.6 |
    983.4 |
   1024.2 |
   1065.0 |
   1105.8 |
   1146.6 |
   1187.4 |
  (15 below, 21 above range)

sat_saturate (n=160, range 418.2-1182.6 ns)
    418.2 |#
    456.4 |
    494.6 |########################################
    532.8 |
    571.1 |
    609.3 |
    647.5 |
    685.7 |
    723.9 |
    762.2 |
    800.4 |
    838.6 |
    876.8 |
    915.0 |
    953.2 |
    991.5 |
   1029.7 |
   1067.9 |
   1106.1 |
   1144.3 |
  (28 below, 19 above range)

sat_wrap (n=160, range 294.9-817.2 ns)
    294.9 |########################################
    321.1 |
    347.2 |
    373.3 |
    399.4 |
    425.5 |
    451.6 |
    477.7 |
    503.8 |
    530.0 |
    556.1 |
    582.2 |
    608.3 |
    634.4 |
    660.5 |
    686.6 |
    712.7 |
    738.9 |
    765.0 |
    791.1 |
  (16 below, 22 above range)

```

## Diagnostics

- **sat_checked**: CV=57.3% (high variance, measurements may be unstable)
- **sat_saturate**: CV=56.1% (high variance, measurements may be unstable)
- **sat_wrap**: CV=59.3% (high variance, measurements may be unstable)
- **sat_wrap**: autocorrelation=0.63 (measurement drift or warm-up artifact)
