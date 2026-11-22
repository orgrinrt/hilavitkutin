# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (inline_always, inline_default) are a dead heat (<1%)

inline_always (268 ns) and inline_default (268 ns) differ by 0.00%, inside the noise, even though the wider field spreads 18.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (268 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (inline_always) is the fastest** at 268.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.18x (fastest 268.3 ns, slowest 316.7 ns)

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
| inline_always | 2374ns | 2367ns | 2361ns | 2368ns | 2403ns | base |
| inline_default | 2371ns | 2367ns | 2359ns | 2367ns | 2396ns | -0.09% |
| inline_never | 2401ns | 2393ns | 2387ns | 2394ns | 2438ns | +1.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 269ns | 264ns | 274ns | base | 0.953 |
| inline_default | 269ns | 265ns | 273ns | +0.00% | 0.953 |
| inline_never | 317ns | 311ns | 323ns | +17.87% | 0.808 |

## Performance model

- Peak throughput: **0.968 Gops/s** (inline_always; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.954 | 98.6% |
| inline_default | 0.954 | 98.6% |
| inline_never | 0.808 | 83.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 2374ns | 2374ns | base |
| inline_default | 2371ns | 2371ns | -0.09% |
| inline_never | 2401ns | 2401ns | +1.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 268ns | base | --- | [268, 269] | --- | --- | --- | --- |
| inline_default | 268ns | no significant difference | [-1, +0]ns | [268, 269] | no | 0.8716 | 0.8716 | 7 |
| inline_never | 317ns | +47.9ns (+17.9%) | [+47, +48]ns | [316, 318] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 269ns | +0.0% | +17.8% |
| 1 | 2 | 270ns | -0.6% | +17.5% |
| 1 | 3 | 268ns | +0.3% | +17.8% |
| 1 | 4 | 268ns | +0.1% | +19.1% |
| 1 | 5 | 268ns | -0.2% | +18.2% |
| 1 | 6 | 269ns | +0.1% | +17.8% |
| 1 | 7 | 270ns | -0.5% | +17.4% |
| 1 | 8 | 268ns | +0.6% | +18.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.193 | ok |
| inline_default | -0.128 | ok |
| inline_never | -0.305 | moderate- |

**Consistency summary:**

- **inline_default**: won 3/8, lost 2/8
- **inline_never**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 2.4ns | 268.8ns | 0.9% |  |
| inline_default | 2.4ns | 268.8ns | 0.9% |  |
| inline_never | 3.0ns | 316.8ns | 1.0% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 264.4-273.7 ns)
    264.4 |##############
    264.9 |#######
    265.4 |################
    265.8 |#########
    266.3 |#########################
    266.8 |#######################
    267.2 |##############
    267.7 |##################
    268.2 |############################
    268.6 |############################
    269.1 |##############
    269.5 |########################################
    270.0 |##############
    270.5 |#########
    270.9 |####
    271.4 |#######
    271.9 |#########
    272.3 |#######
    272.8 |#######
    273.3 |##############
  (14 below, 13 above range)

inline_default (n=160, range 264.8-273.4 ns)
    264.8 |##########
    265.3 |##########
    265.7 |#############
    266.1 |################
    266.5 |##########################
    267.0 |#################################
    267.4 |##############################
    267.8 |########################################
    268.3 |########################################
    268.7 |##############################
    269.1 |####################################
    269.5 |####################################
    270.0 |#################################
    270.4 |##############################
    270.8 |####################
    271.3 |
    271.7 |################
    272.1 |#############
    272.5 |##########
    273.0 |
  (16 below, 10 above range)

inline_never (n=160, range 310.8-323.5 ns)
    310.8 |#####################
    311.4 |####
    312.0 |################
    312.7 |##############
    313.3 |#######
    313.9 |#########
    314.6 |########################################
    315.2 |##################
    315.8 |##################
    316.5 |################################
    317.1 |#########################
    317.8 |############################
    318.4 |##############
    319.0 |#####################
    319.7 |################
    320.3 |#########
    320.9 |##
    321.6 |#######
    322.2 |##
    322.8 |####
  (12 below, 14 above range)

```
