# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_const dominates: 49% faster than the next best (shr_loop_invariant)

shr_const (136 ns) leads shr_loop_invariant (203 ns) by 49%, a clear separation rather than a photo finish. CV 98.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### shr_per_iter is an outlier: 9.8x slower than the field

shr_per_iter (1.33 us) is 9.8x the fastest (136 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 98.6%)

shr_const wins on median (136 ns) yet has the highest variance (CV 98.6%), while shr_per_iter is the steadiest (CV 27.8%, 1.33 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (136 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.8x the fastest

Fastest shr_const (136 ns) to slowest shr_per_iter (1.33 us): 9.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_loop_invariant is inconsistent: worst-20% is 3.5x its best-20%

shr_loop_invariant's best 20% of batches run at 172 ns but its worst 20% at 609 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (shr_const) is the fastest** at 136.1 ns median
- 2 variants significantly slower than baseline
- Spread: 9.80x (fastest 136.1 ns, slowest 1332.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Shift amount: const vs loop-invariant runtime vs per-iter independent |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6a92bf08e314750d |
| Counter frequency | 2112788246 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2574ns | 1687ns | 1670ns | 1900ns | 5500ns | base |
| shr_loop_invariant | 2800ns | 2087ns | 1715ns | 2047ns | 6144ns | +8.79% |
| shr_per_iter | 3255ns | 3218ns | 2634ns | 3183ns | 4093ns | +26.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 209ns | 129ns | 452ns | base | 78.465 |
| shr_loop_invariant | 279ns | 172ns | 609ns | +33.62% | 58.724 |
| shr_per_iter | 1343ns | 1089ns | 1672ns | +543.14% | 12.200 |

## Performance model

- Peak throughput: **126.661 Gops/s** (shr_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 120.426 | 95.1% |
| shr_loop_invariant | 80.889 | 63.9% |
| shr_per_iter | 12.292 | 9.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2574ns | 2574ns | base |
| shr_loop_invariant | 2800ns | 2800ns | +8.79% |
| shr_per_iter | 3255ns | 3255ns | +26.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 136ns | base | --- | [133, 144] | --- | --- | --- | --- |
| shr_loop_invariant | 203ns | +65.3ns (+48.0%) | [+54, +71]ns | [202, 203] | YES | 0.0000 | 0.0000 | 0 |
| shr_per_iter | 1333ns | +1168.7ns (+859.0%) | [+1125, +1194]ns | [1332, 1334] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 140ns | +22.7% | +855.5% |
| 1 | 2 | 215ns | -5.5% | +521.8% |
| 1 | 3 | 135ns | +50.1% | +889.1% |
| 1 | 4 | 130ns | +40.5% | +736.5% |
| 1 | 5 | 135ns | +50.1% | +888.4% |
| 1 | 6 | 134ns | +62.2% | +893.4% |
| 1 | 7 | 131ns | +38.8% | +915.2% |
| 1 | 8 | 130ns | +64.2% | +739.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | -0.053 | ok |
| shr_loop_invariant | -0.374 | moderate- |
| shr_per_iter | -0.208 | moderate- |

**Consistency summary:**

- **shr_loop_invariant**: won 1/8, lost 7/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 30.4ns | 208.8ns | 14.6% | HIGH |
| shr_loop_invariant | 33.2ns | 279.0ns | 11.9% | HIGH |
| shr_per_iter | 24.3ns | 1342.9ns | 1.8% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 129.4-451.6 ns)
    129.4 |########################################
    145.5 |###
    161.6 |##
    177.7 |###
    193.8 |#
    209.9 |#
    226.0 |
    242.1 |##
    258.2 |
    274.3 |
    290.5 |
    306.6 |
    322.7 |
    338.8 |
    354.9 |
    371.0 |##
    387.1 |
    403.2 |#
    419.3 |
    435.4 |#
  (11 below, 12 above range)

shr_loop_invariant (n=160, range 172.3-608.8 ns)
    172.3 |################
    194.1 |########################################
    216.0 |##########
    237.8 |
    259.6 |
    281.4 |
    303.3 |
    325.1 |
    346.9 |
    368.7 |
    390.6 |
    412.4 |
    434.2 |
    456.0 |
    477.8 |#
    499.7 |
    521.5 |
    543.3 |#
    565.1 |#
    587.0 |#
  (18 below, 18 above range)

shr_per_iter (n=160, range 1089.4-1671.7 ns)
   1089.4 |######
   1118.5 |
   1147.6 |
   1176.8 |
   1205.9 |
   1235.0 |
   1264.1 |
   1293.2 |
   1322.3 |########################################
   1351.4 |#
   1380.6 |
   1409.7 |
   1438.8 |
   1467.9 |
   1497.0 |
   1526.1 |
   1555.3 |
   1584.4 |
   1613.5 |
   1642.6 |
  (20 below, 7 above range)

```

## Diagnostics

- **shr_const**: CV=64.2% (high variance, measurements may be unstable)
- **shr_const**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **shr_const**: bridge=14.6% of algo (FFI overhead may distort results)
- **shr_loop_invariant**: CV=62.2% (high variance, measurements may be unstable)
- **shr_loop_invariant**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **shr_loop_invariant**: bridge=12.2% of algo (FFI overhead may distort results)
- **shr_per_iter**: CV=27.6% (high variance, measurements may be unstable)
