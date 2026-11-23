# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_index dominates: 20% faster than the next best (bc_unchecked)

bc_index (484 ns) leads bc_unchecked (580 ns) by 20%, a clear separation rather than a photo finish. CV 84.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bc_index is fastest but the noisiest (CV 84.9%)

bc_index wins on median (484 ns) yet has the highest variance (CV 84.9%), while bc_unchecked is the steadiest (CV 68.6%, 580 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (96 ns) is smaller than the fastest variant's own run-to-run std-dev (411 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (484 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### bc_index is inconsistent: worst-20% is 3.2x its best-20%

bc_index's best 20% of batches run at 474 ns but its worst 20% at 1.50 us (3.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### bc_iter's edge over baseline is significant but tiny (0 ns, 0.09%)

bc_iter differs from baseline bc_index by 0 ns (0.09%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (bc_index) is the fastest** at 484.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.20x (fastest 484.5 ns, slowest 580.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bounds-check elision: iter() vs explicit index vs get_unchecked |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5a3ec07491bf2d68 |
| Counter frequency | 2112744983 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 3092ns | 2045ns | 2009ns | 2353ns | 6392ns | base |
| bc_iter | 2994ns | 2461ns | 2014ns | 2454ns | 5595ns | -3.16% |
| bc_unchecked | 3066ns | 2463ns | 2237ns | 2465ns | 5699ns | -0.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 728ns | 474ns | 1500ns | base | 5.629 |
| bc_iter | 704ns | 475ns | 1308ns | -3.31% | 5.822 |
| bc_unchecked | 720ns | 527ns | 1335ns | -1.02% | 5.687 |

## Performance model

- Peak throughput: **8.633 Gops/s** (bc_index; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 8.454 | 97.9% |
| bc_iter | 7.058 | 81.8% |
| bc_unchecked | 7.063 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 3092ns | 3092ns | base |
| bc_iter | 2994ns | 2994ns | -3.16% |
| bc_unchecked | 3066ns | 3066ns | -0.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 484ns | base | --- | [475, 580] | --- | --- | --- | --- |
| bc_iter | 580ns | +0.5ns (+0.1%) | [+0, +10]ns | [580, 580] | YES | 0.0000 | 0.0000 | 11 |
| bc_unchecked | 580ns | no significant difference | [+0, +105]ns | [580, 580] | no (adj: YES) | 0.0192 | 0.0192 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 1112ns | -47.8% | -47.8% |
| 1 | 2 | 475ns | +22.3% | +22.2% |
| 1 | 3 | 475ns | +22.3% | +22.2% |
| 1 | 4 | 475ns | +0.0% | +22.2% |
| 1 | 5 | 476ns | +21.8% | -0.4% |
| 1 | 6 | 475ns | +0.1% | +22.2% |
| 1 | 7 | 580ns | +0.0% | -0.0% |
| 1 | 8 | 580ns | +0.1% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.034 | ok |
| bc_iter | -0.375 | moderate- |
| bc_unchecked | -0.160 | ok |

**Consistency summary:**

- **bc_iter**: won 1/8, lost 3/8
- **bc_unchecked**: won 2/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 29.8ns | 727.7ns | 4.1% |  |
| bc_iter | 29.0ns | 703.6ns | 4.1% |  |
| bc_unchecked | 29.9ns | 720.2ns | 4.1% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 474.5-1500.5 ns)
    474.5 |########################################
    525.8 |
    577.1 |#################
    628.4 |##
    679.7 |
    731.0 |
    782.3 |
    833.6 |
    884.9 |#
    936.2 |
    987.5 |
   1038.8 |
   1090.1 |
   1141.4 |
   1192.7 |#
   1244.0 |
   1295.3 |####
   1346.6 |#
   1397.9 |#
   1449.2 |#
  (7 below, 13 above range)

bc_iter (n=160, range 475.3-1307.5 ns)
    475.3 |#
    517.0 |
    558.6 |########################################
    600.2 |
    641.8 |
    683.4 |
    725.0 |
    766.6 |
    808.2 |
    849.8 |
    891.4 |
    933.0 |
    974.7 |
   1016.3 |
   1057.9 |
   1099.5 |
   1141.1 |
   1182.7 |
   1224.3 |
   1265.9 |
  (30 below, 17 above range)

bc_unchecked (n=160, range 527.1-1334.5 ns)
    527.1 |
    567.5 |########################################
    607.8 |
    648.2 |
    688.6 |
    729.0 |
    769.3 |
    809.7 |
    850.1 |
    890.4 |
    930.8 |
    971.2 |
   1011.6 |
   1051.9 |
   1092.3 |
   1132.7 |
   1173.0 |
   1213.4 |
   1253.8 |
   1294.2 |
  (16 below, 18 above range)

```

## Diagnostics

- **bc_index**: CV=56.5% (high variance, measurements may be unstable)
- **bc_index**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **bc_iter**: CV=57.4% (high variance, measurements may be unstable)
- **bc_unchecked**: CV=55.2% (high variance, measurements may be unstable)
