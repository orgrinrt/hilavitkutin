# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (5.16 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### inline_never is inconsistent: worst-20% is 1.9x its best-20%

inline_never's best 20% of batches run at 5.14 us but its worst 20% at 9.89 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 1.7% of the fastest

All 3 variants sit between 5.16 us and 5.25 us - a 1.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (inline_always) is the fastest** at 5164.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 5164.0 ns, slowest 5252.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x39f847b2c651de80 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 7343ns | 7307ns | 7265ns | 7317ns | 7498ns | base |
| inline_default | 7356ns | 7312ns | 7241ns | 7324ns | 7564ns | +0.17% |
| inline_never | 8384ns | 7384ns | 7223ns | 7420ns | 12439ns | +14.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 5195ns | 5141ns | 5319ns | base | 0.788 |
| inline_default | 5203ns | 5126ns | 5352ns | +0.15% | 0.787 |
| inline_never | 6172ns | 5141ns | 9890ns | +18.80% | 0.664 |

## Performance model

- Peak throughput: **0.799 Gops/s** (inline_default; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.793 | 99.3% |
| inline_default | 0.792 | 99.1% |
| inline_never | 0.780 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 7343ns | 7343ns | base |
| inline_default | 7356ns | 7356ns | +0.17% |
| inline_never | 8384ns | 8384ns | +14.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 5164ns | base | --- | [5162, 5172] | --- | --- | --- | --- |
| inline_default | 5171ns | no significant difference | [-4, +5]ns | [5164, 5180] | no | 0.9370 | 0.9370 | 0 |
| inline_never | 5252ns | +72.9ns (+1.4%) | [+61, +103]ns | [5230, 5282] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 5165ns | +0.2% | +1.1% |
| 1 | 2 | 5163ns | -0.9% | +1.2% |
| 1 | 3 | 5167ns | -0.1% | +3.9% |
| 1 | 4 | 5162ns | +0.2% | -0.8% |
| 1 | 5 | 5158ns | +0.4% | +1.2% |
| 1 | 6 | 5162ns | +0.2% | +0.4% |
| 1 | 7 | 5223ns | -0.3% | +5.3% |
| 1 | 8 | 5163ns | +0.6% | +7.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.160 | ok |
| inline_default | 0.221 | moderate+ |
| inline_never | 0.217 | moderate+ |

**Consistency summary:**

- **inline_default**: won 2/8, lost 5/8
- **inline_never**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 3.7ns | 5195.3ns | 0.1% |  |
| inline_default | 2.8ns | 5203.2ns | 0.1% |  |
| inline_never | 3.6ns | 6172.2ns | 0.1% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 5140.7-5318.8 ns)
   5140.7 |
   5149.6 |########################################
   5158.5 |########################################
   5167.4 |###############
   5176.3 |#
   5185.2 |#########
   5194.1 |###
   5203.0 |####
   5212.0 |#####
   5220.9 |
   5229.8 |#
   5238.7 |#
   5247.6 |######
   5256.5 |
   5265.4 |##
   5274.3 |
   5283.2 |
   5292.1 |##
   5301.0 |
   5309.9 |
  (5 below, 10 above range)

inline_default (n=160, range 5126.1-5351.8 ns)
   5126.1 |
   5137.3 |
   5148.6 |########################################
   5159.9 |######################
   5171.2 |###########
   5182.5 |##########
   5193.8 |#######
   5205.1 |###
   5216.3 |##
   5227.6 |####
   5238.9 |#########
   5250.2 |###
   5261.5 |
   5272.8 |##
   5284.1 |
   5295.3 |#
   5306.6 |#
   5317.9 |
   5329.2 |
   5340.5 |
  (12 below, 10 above range)

inline_never (n=160, range 5141.2-9890.2 ns)
   5141.2 |########################################
   5378.6 |############
   5616.1 |
   5853.5 |
   6091.0 |
   6328.4 |
   6565.9 |
   6803.3 |
   7040.8 |
   7278.2 |
   7515.7 |
   7753.1 |
   7990.6 |
   8228.0 |
   8465.5 |
   8702.9 |
   8940.4 |
   9177.8 |
   9415.3 |
   9652.7 |
  (20 below, 4 above range)

```

## Diagnostics

- **inline_never**: CV=130.6% (high variance, measurements may be unstable)
