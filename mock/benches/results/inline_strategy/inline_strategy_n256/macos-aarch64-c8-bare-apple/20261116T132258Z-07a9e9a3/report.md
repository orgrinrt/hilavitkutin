# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (inline_always, inline_default) are a dead heat (<1%)

inline_always (268 ns) and inline_default (268 ns) differ by 0.15%, inside the noise, even though the wider field spreads 18.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (268 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (inline_always) is the fastest** at 267.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.18x (fastest 267.9 ns, slowest 316.7 ns)

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
| inline_always | 2370ns | 2365ns | 2360ns | 2365ns | 2395ns | base |
| inline_default | 2371ns | 2366ns | 2361ns | 2367ns | 2392ns | +0.03% |
| inline_never | 2395ns | 2390ns | 2385ns | 2391ns | 2419ns | +1.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 268ns | 264ns | 273ns | base | 0.954 |
| inline_default | 269ns | 264ns | 275ns | +0.15% | 0.952 |
| inline_never | 316ns | 311ns | 322ns | +17.91% | 0.809 |

## Performance model

- Peak throughput: **0.969 Gops/s** (inline_default; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.956 | 98.6% |
| inline_default | 0.954 | 98.5% |
| inline_never | 0.808 | 83.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 2370ns | 2370ns | base |
| inline_default | 2371ns | 2371ns | +0.03% |
| inline_never | 2395ns | 2395ns | +1.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 268ns | base | --- | [268, 269] | --- | --- | --- | --- |
| inline_default | 268ns | no significant difference | [-0, +0]ns | [268, 269] | no | 0.7399 | 0.7399 | 15 |
| inline_never | 317ns | +47.9ns (+17.9%) | [+47, +49]ns | [316, 317] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 269ns | -0.1% | +17.8% |
| 1 | 2 | 267ns | +0.4% | +18.3% |
| 1 | 3 | 270ns | -0.4% | +17.5% |
| 1 | 4 | 269ns | -0.3% | +17.6% |
| 1 | 5 | 268ns | +0.2% | +18.3% |
| 1 | 6 | 268ns | +0.1% | +18.4% |
| 1 | 7 | 268ns | +0.1% | +18.5% |
| 1 | 8 | 268ns | +0.6% | +18.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.264 | moderate- |
| inline_default | -0.175 | ok |
| inline_never | 0.295 | moderate+ |

**Consistency summary:**

- **inline_default**: won 2/8, lost 4/8
- **inline_never**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 2.4ns | 268.4ns | 0.9% |  |
| inline_default | 2.3ns | 268.8ns | 0.9% |  |
| inline_never | 2.9ns | 316.5ns | 0.9% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 264.4-272.5 ns)
    264.4 |##################
    264.8 |######
    265.3 |############
    265.7 |###############
    266.1 |##############################
    266.5 |#################################
    266.9 |##################
    267.3 |#################################
    267.7 |########################################
    268.1 |########################
    268.5 |#########
    268.9 |########################
    269.3 |#####################
    269.7 |########################
    270.1 |#####################
    270.5 |##################
    270.9 |#####################
    271.3 |#####################
    271.7 |#########
    272.1 |#########
  (13 below, 12 above range)

inline_default (n=160, range 264.2-274.8 ns)
    264.2 |######
    264.7 |##############
    265.3 |########
    265.8 |#############################
    266.3 |################
    266.8 |##########
    267.4 |##############
    267.9 |########################################
    268.4 |##############
    269.0 |#######################
    269.5 |###############################
    270.0 |##############
    270.5 |##############
    271.1 |############
    271.6 |##################
    272.1 |##########
    272.6 |##
    273.2 |##
    273.7 |########
    274.2 |####
  (16 below, 2 above range)

inline_never (n=160, range 310.7-322.0 ns)
    310.7 |########
    311.3 |######
    311.8 |##############
    312.4 |##############
    313.0 |########
    313.5 |##############
    314.1 |##########
    314.6 |####
    315.2 |######
    315.8 |########################################
    316.3 |##########
    316.9 |#############################
    317.5 |##############
    318.0 |##############
    318.6 |################
    319.2 |#########################
    319.7 |############
    320.3 |############
    320.8 |######
    321.4 |######
  (15 below, 13 above range)

```
