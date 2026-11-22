# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (5 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 1000% faster than the next best (popcount_swar)

popcount_std (0 ns) leads popcount_swar (4 ns) by 1000%, a clear separation rather than a photo finish. CV 97.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 91% (significant)

popcount_std is -4 ns (91%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 11.5x slower than the field

popcount_handrolled (5 ns) is 11.5x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 97.7%)

popcount_std wins on median (0 ns) yet has the highest variance (CV 97.7%), while popcount_swar is the steadiest (CV 27.7%, 4 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### popcount_std shows warm-up / thermal drift (autocorr +0.88)

popcount_std's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 11.5x the fastest

Fastest popcount_std (0 ns) to slowest popcount_handrolled (5 ns): 11.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_handrolled is inconsistent: worst-20% is 2.5x its best-20%

popcount_handrolled's best 20% of batches run at 3 ns but its worst 20% at 7 ns (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 0.4 ns median (-91.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 11.50x (fastest 0.4 ns, slowest 4.6 ns)

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
| popcount_handrolled | 2103ns | 2098ns | 2090ns | 2098ns | 2130ns | base |
| popcount_std | 2097ns | 2093ns | 2089ns | 2094ns | 2115ns | -0.29% |
| popcount_swar | 2099ns | 2096ns | 2092ns | 2097ns | 2114ns | -0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 5ns | 3ns | 7ns | base | 14.161 |
| popcount_std | 0ns | 0ns | 1ns | -89.59% | 135.989 |
| popcount_swar | 4ns | 3ns | 6ns | -1.73% | 14.410 |

## Performance model

- Peak throughput: **24.066 Gops/s** (popcount_handrolled; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 13.913 | 57.8% |
| popcount_std | 160.000 | 664.8% |
| popcount_swar | 14.545 | 60.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2103ns | 2103ns | base |
| popcount_std | 2097ns | 2097ns | -0.29% |
| popcount_swar | 2099ns | 2099ns | -0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 5ns | base | --- | [4, 5] | --- | --- | --- | --- |
| popcount_std | 0ns | -4.2ns (-91.3%) | [-4, -4]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 4ns | no significant difference | [-0, +0]ns | [4, 5] | no | 0.9341 | 0.9341 | 14 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 4ns | -89.9% | +6.3% |
| 1 | 2 | 4ns | -90.9% | +13.6% |
| 1 | 3 | 5ns | -92.0% | -16.0% |
| 1 | 4 | 4ns | -90.5% | +9.5% |
| 1 | 5 | 5ns | -92.0% | -8.0% |
| 1 | 6 | 4ns | -90.5% | +4.8% |
| 1 | 7 | 4ns | -89.9% | +6.3% |
| 1 | 8 | 5ns | -91.3% | -4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.293 | moderate- |
| popcount_std | 0.875 | HIGH+ (drift/warm-up) |
| popcount_swar | -0.505 | HIGH- (thermal bounce) |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.6ns | 4.5ns | 79.0% | HIGH |
| popcount_std | 2.8ns | 0.5ns | 595.8% | HIGH |
| popcount_swar | 3.6ns | 4.4ns | 80.9% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 2.7-6.6 ns)
      2.7 |
      2.9 |#############
      3.1 |
      3.2 |##########################
      3.4 |
      3.6 |########################################
      3.8 |
      4.0 |######################################
      4.2 |
      4.4 |########################
      4.6 |
      4.8 |########################################
      5.0 |
      5.2 |
      5.4 |##########################
      5.6 |
      5.8 |###################
      6.0 |
      6.2 |#################
      6.4 |
  (17 below, 14 above range)

popcount_std (n=160, range 0.0-1.1 ns)
      0.0 |##########################
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |########################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |###################
      0.8 |
      0.9 |
      0.9 |
      1.0 |
  (0 below, 19 above range)

popcount_swar (n=160, range 2.8-6.2 ns)
      2.8 |##############
      2.9 |
      3.1 |
      3.3 |#######################
      3.4 |
      3.6 |########################################
      3.8 |
      4.0 |
      4.1 |########################################
      4.3 |
      4.5 |##################################
      4.7 |
      4.8 |
      5.0 |################################
      5.2 |
      5.3 |#####################
      5.5 |
      5.7 |#######################
      5.9 |
      6.0 |
  (15 below, 18 above range)

```

## Diagnostics

- **popcount_handrolled**: CV=31.1% (high variance, measurements may be unstable)
- **popcount_handrolled**: bridge=71.7% of algo (FFI overhead may distort results)
- **popcount_std**: CV=83.0% (high variance, measurements may be unstable)
- **popcount_std**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **popcount_std**: bridge=725.0% of algo (FFI overhead may distort results)
- **popcount_swar**: CV=27.5% (high variance, measurements may be unstable)
- **popcount_swar**: bridge=84.1% of algo (FFI overhead may distort results)
