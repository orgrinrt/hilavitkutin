# Branchful if/else vs branchless cmov/csel under data-dependent input

2 variants, 160 samples per variant.
Baseline: **branch_cmov**

## Highlights

Baseline for all deltas below: **branch_cmov**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### branch_cmov dominates: 14% faster than the next best (branch_if)

branch_cmov (22.47 us) leads branch_if (25.58 us) by 14%, a clear separation rather than a photo finish. CV 10.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### branch_cmov is fastest but the noisiest (CV 10.7%)

branch_cmov wins on median (22.47 us) yet has the highest variance (CV 10.7%), while branch_if is the steadiest (CV 9.7%, 25.58 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (branch_cmov)

The baseline branch_cmov is the fastest (22.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (branch_cmov) is the fastest** at 22472.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.14x (fastest 22472.9 ns, slowest 25581.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branchful if/else vs branchless cmov/csel under data-dependent input |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x76543210fedcba98 |
| Counter frequency | 2112670986 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| branch_cmov | 25966ns | 24026ns | 23873ns | 25532ns | 29362ns | base |
| branch_if | 28458ns | 27135ns | 26455ns | 27552ns | 33178ns | +9.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| branch_cmov | 24282ns | 22324ns | 27458ns | base | 0.675 |
| branch_if | 26811ns | 24902ns | 31274ns | +10.42% | 0.611 |

## Performance model

- Peak throughput: **0.734 Gops/s** (branch_cmov; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| branch_cmov | 0.729 | 99.3% |
| branch_if | 0.640 | 87.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| branch_cmov | 25966ns | 25966ns | base |
| branch_if | 28458ns | 28458ns | +9.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| branch_cmov | 22473ns | base | --- | [22444, 22552] | --- | --- | --- | --- |
| branch_if | 25582ns | +2730.2ns (+12.1%) | [+2621, +2913]ns | [25492, 25696] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | branch_cmov | branch_if |
|---|---|---|---|
| 1 | 1 | 27354ns | -5.5% |
| 1 | 2 | 22408ns | +40.3% |
| 1 | 3 | 22385ns | +14.0% |
| 1 | 4 | 27347ns | +13.4% |
| 1 | 5 | 22392ns | +11.7% |
| 1 | 6 | 27357ns | -6.4% |
| 1 | 7 | 22423ns | +11.9% |
| 1 | 8 | 22411ns | +10.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| branch_cmov | -0.477 | moderate- |
| branch_if | -0.299 | moderate- |

**Consistency summary:**

- **branch_if**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| branch_cmov | 21.3ns | 24282.0ns | 0.1% |  |
| branch_if | 21.5ns | 26811.0ns | 0.1% |  |

## Distribution (algo ns)

```
branch_cmov (n=160, range 22323.7-27458.3 ns)
  22323.7 |########################################
  22580.5 |#
  22837.2 |
  23093.9 |
  23350.7 |
  23607.4 |
  23864.1 |
  24120.8 |
  24377.6 |
  24634.3 |
  24891.0 |
  25147.8 |
  25404.5 |
  25661.2 |
  25917.9 |
  26174.7 |
  26431.4 |
  26688.1 |
  26944.9 |
  27201.6 |#######################
  (17 below, 13 above range)

branch_if (n=160, range 24901.7-31273.8 ns)
  24901.7 |#####################################
  25220.3 |#####################################
  25538.9 |########################################
  25857.5 |############
  26176.1 |#####
  26494.8 |
  26813.4 |
  27132.0 |
  27450.6 |
  27769.2 |
  28087.8 |
  28406.4 |
  28725.0 |#
  29043.6 |#
  29362.2 |###
  29680.8 |#
  29999.4 |#
  30318.0 |
  30636.6 |##########
  30955.2 |#########
  (16 below, 19 above range)

```
