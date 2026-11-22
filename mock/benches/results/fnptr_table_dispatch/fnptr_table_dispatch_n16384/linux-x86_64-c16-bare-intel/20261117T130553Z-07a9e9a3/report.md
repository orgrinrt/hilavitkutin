# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 18% faster than the next best (fpt_table_16)

fpt_match_16 (13.26 us) leads fpt_table_16 (15.61 us) by 18%, a clear separation rather than a photo finish. CV 9.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_match_16 is fastest but the noisiest (CV 9.5%)

fpt_match_16 wins on median (13.26 us) yet has the highest variance (CV 9.5%), while fpt_table_16 is the steadiest (CV 7.3%, 15.61 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (13.26 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 13260.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.18x (fastest 13260.9 ns, slowest 15611.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 2112762479 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 15084ns | 15172ns | 13148ns | 15194ns | 16687ns | base |
| fpt_table_16 | 17111ns | 17511ns | 15028ns | 17428ns | 18243ns | +13.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 13211ns | 11450ns | 14752ns | base | 1.240 |
| fpt_table_16 | 15251ns | 13344ns | 16323ns | +15.45% | 1.074 |

## Performance model

- Peak throughput: **1.431 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 1.236 | 86.3% |
| fpt_table_16 | 1.049 | 73.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 15084ns | 15084ns | base |
| fpt_table_16 | 17111ns | 17111ns | +13.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 13261ns | base | --- | [13066, 13412] | --- | --- | --- | --- |
| fpt_table_16 | 15612ns | +2161.6ns (+16.3%) | [+1955, +2411]ns | [15462, 15744] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 10712ns | +46.6% |
| 1 | 2 | 13326ns | +19.9% |
| 1 | 3 | 14294ns | +11.0% |
| 1 | 4 | 12883ns | +14.1% |
| 1 | 5 | 13004ns | +16.0% |
| 1 | 6 | 13274ns | +20.9% |
| 1 | 7 | 13885ns | -8.7% |
| 1 | 8 | 12800ns | +23.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.059 | ok |
| fpt_table_16 | -0.351 | moderate- |

**Consistency summary:**

- **fpt_table_16**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 27.6ns | 13210.5ns | 0.2% |  |
| fpt_table_16 | 24.2ns | 15250.9ns | 0.2% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 11450.0-14751.6 ns)
  11450.0 |##
  11615.1 |
  11780.1 |##
  11945.2 |
  12110.3 |
  12275.4 |
  12440.5 |#####################################
  12605.5 |########################
  12770.6 |#####################################
  12935.7 |##################################
  13100.8 |#############################
  13265.9 |########################################
  13430.9 |################
  13596.0 |##################
  13761.1 |#####################
  13926.2 |#############################
  14091.3 |##########################
  14256.3 |##########
  14421.4 |#############
  14586.5 |########
  (18 below, 10 above range)

fpt_table_16 (n=160, range 13344.3-16323.4 ns)
  13344.3 |
  13493.3 |##
  13642.2 |
  13791.2 |
  13940.1 |
  14089.1 |
  14238.0 |######
  14387.0 |######
  14535.9 |########
  14684.9 |##########
  14833.8 |################
  14982.8 |#####################
  15131.7 |################
  15280.7 |######
  15429.6 |###############################
  15578.6 |#####################
  15727.6 |########################################
  15876.5 |########################################
  16025.5 |###########################
  16174.4 |##############
  (19 below, 13 above range)

```
