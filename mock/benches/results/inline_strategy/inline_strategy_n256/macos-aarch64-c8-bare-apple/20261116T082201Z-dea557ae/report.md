# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (inline_always, inline_default) are a dead heat (<1%)

inline_always (273 ns) and inline_default (274 ns) differ by 0.33%, inside the noise, even though the wider field spreads 18.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (273 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (inline_always) is the fastest** at 272.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.19x (fastest 272.9 ns, slowest 324.2 ns)

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
| inline_always | 2410ns | 2399ns | 2380ns | 2400ns | 2469ns | base |
| inline_default | 2415ns | 2401ns | 2378ns | 2403ns | 2490ns | +0.24% |
| inline_never | 2446ns | 2430ns | 2420ns | 2432ns | 2511ns | +1.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 274ns | 268ns | 284ns | base | 0.935 |
| inline_default | 274ns | 269ns | 281ns | +0.04% | 0.934 |
| inline_never | 324ns | 318ns | 332ns | +18.36% | 0.790 |

## Performance model

- Peak throughput: **0.957 Gops/s** (inline_always; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.938 | 98.0% |
| inline_default | 0.935 | 97.7% |
| inline_never | 0.790 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 2410ns | 2410ns | base |
| inline_default | 2415ns | 2415ns | +0.24% |
| inline_never | 2446ns | 2446ns | +1.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 273ns | base | --- | [272, 274] | --- | --- | --- | --- |
| inline_default | 274ns | no significant difference | [+0, +1]ns | [273, 274] | no | 0.0504 | 0.0504 | 9 |
| inline_never | 324ns | +51.5ns (+18.9%) | [+50, +52]ns | [323, 325] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 272ns | +0.8% | +18.6% |
| 1 | 2 | 274ns | +0.1% | +18.4% |
| 1 | 3 | 273ns | +0.0% | +18.8% |
| 1 | 4 | 269ns | +0.1% | +20.2% |
| 1 | 5 | 273ns | +0.4% | +19.2% |
| 1 | 6 | 272ns | +0.8% | +19.1% |
| 1 | 7 | 275ns | +0.1% | +17.8% |
| 1 | 8 | 273ns | -0.1% | +18.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.016 | ok |
| inline_default | 0.116 | ok |
| inline_never | -0.219 | moderate- |

**Consistency summary:**

- **inline_default**: won 1/8, lost 5/8
- **inline_never**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 3.0ns | 273.9ns | 1.1% |  |
| inline_default | 2.7ns | 274.0ns | 1.0% |  |
| inline_never | 3.1ns | 324.2ns | 1.0% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 267.5-283.5 ns)
    267.5 |##########
    268.3 |######
    269.1 |##########
    269.9 |##########################
    270.7 |##############################
    271.5 |########################
    272.3 |##########################
    273.1 |########################################
    273.9 |####################################
    274.7 |##############
    275.5 |################
    276.3 |##################
    277.1 |##########
    277.9 |##
    278.7 |####
    279.5 |####
    280.3 |####
    281.1 |##
    281.9 |##
    282.7 |
  (16 below, 2 above range)

inline_default (n=160, range 268.7-280.7 ns)
    268.7 |###########
    269.3 |###########
    269.9 |############################
    270.5 |###########
    271.1 |#######################
    271.7 |################
    272.3 |############################
    272.9 |################
    273.5 |#######################
    274.1 |########################################
    274.7 |############################
    275.3 |#######################
    275.9 |####
    276.5 |################
    277.1 |#######
    277.7 |##############
    278.3 |##
    278.9 |##
    279.5 |##
    280.1 |#######
  (13 below, 11 above range)

inline_never (n=160, range 317.8-331.6 ns)
    317.8 |###############################
    318.5 |###########
    319.2 |####################
    319.8 |#################
    320.5 |########################################
    321.2 |#####
    321.9 |#################
    322.6 |#################
    323.3 |#########################
    324.0 |##################################
    324.7 |######################
    325.4 |#####################################
    326.0 |############################
    326.7 |##
    327.4 |############################
    328.1 |##############
    328.8 |###########
    329.5 |##############
    330.2 |##############
    330.9 |##
  (12 below, 9 above range)

```
