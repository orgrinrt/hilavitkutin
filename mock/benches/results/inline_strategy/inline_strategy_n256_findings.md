# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (276 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader inline_always vs stability leader inline_default (+7% speed for 1.3x steadier)

inline_always is fastest (276 ns, CV 6.1%); inline_default gives up 6.9% median for 1.3x lower variance (CV 4.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (inline_always) is the fastest** at 276.0 ns median
- 2 variants significantly slower than baseline
- Spread: 1.22x (fastest 276.0 ns, slowest 336.4 ns)

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
| inline_always | 2470ns | 2437ns | 2353ns | 2439ns | 2680ns | base |
| inline_default | 2615ns | 2571ns | 2485ns | 2619ns | 2736ns | +5.90% |
| inline_never | 2548ns | 2551ns | 2379ns | 2536ns | 2754ns | +3.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 281ns | 266ns | 309ns | base | 0.911 |
| inline_default | 298ns | 280ns | 314ns | +5.90% | 0.860 |
| inline_never | 339ns | 314ns | 371ns | +20.58% | 0.755 |

## Performance model

- Peak throughput: **0.964 Gops/s** (inline_always; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.928 | 96.3% |
| inline_default | 0.868 | 90.1% |
| inline_never | 0.761 | 79.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 2470ns | 2470ns | base |
| inline_default | 2615ns | 2615ns | +5.90% |
| inline_never | 2548ns | 2548ns | +3.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 276ns | base | --- | [272, 280] | --- | --- | --- | --- |
| inline_default | 295ns | +19.2ns (+7.0%) | [+17, +21]ns | [290, 308] | YES | 0.0000 | 0.0000 | 2 |
| inline_never | 336ns | +66.2ns (+24.0%) | [+58, +70]ns | [333, 338] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 286ns | +8.5% | +10.8% |
| 1 | 2 | 268ns | +15.6% | +36.1% |
| 1 | 3 | 269ns | +7.8% | +25.9% |
| 1 | 4 | 277ns | +11.1% | +17.3% |
| 1 | 5 | 280ns | +10.3% | +17.6% |
| 1 | 6 | 311ns | -7.0% | +2.3% |
| 1 | 7 | 284ns | +2.1% | +27.4% |
| 1 | 8 | 268ns | +7.0% | +34.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | 0.123 | ok |
| inline_default | 0.161 | ok |
| inline_never | -0.089 | ok |

**Consistency summary:**

- **inline_default**: won 1/8, lost 7/8
- **inline_never**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 2.7ns | 281.2ns | 1.0% |  |
| inline_default | 2.7ns | 297.7ns | 0.9% |  |
| inline_never | 3.3ns | 339.0ns | 1.0% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 265.7-308.6 ns)
    265.7 |########################################
    267.8 |######################################
    270.0 |######################
    272.1 |#################
    274.3 |###########
    276.4 |###########
    278.6 |###########
    280.7 |#############
    282.9 |#############
    285.0 |###################
    287.2 |###############
    289.3 |#########
    291.5 |#####
    293.6 |###
    295.8 |#
    297.9 |
    300.0 |
    302.2 |###
    304.3 |#####
    306.5 |###
  (13 below, 17 above range)

inline_default (n=160, range 279.7-313.6 ns)
    279.7 |########
    281.4 |############
    283.1 |############
    284.8 |######################
    286.5 |##############################
    288.2 |##############################
    289.8 |##############
    291.5 |##########
    293.2 |####
    294.9 |##
    296.6 |####
    298.3 |##
    300.0 |
    301.7 |##
    303.4 |##
    305.1 |####
    306.8 |######################
    308.5 |##############################
    310.2 |########################################
    311.9 |##########################
  (9 below, 13 above range)

inline_never (n=160, range 314.0-370.7 ns)
    314.0 |##############################
    316.8 |#####################################
    319.7 |#################
    322.5 |##########
    325.3 |##########
    328.2 |#################
    331.0 |################################
    333.8 |############
    336.7 |########################################
    339.5 |#########################
    342.3 |############
    345.2 |##
    348.0 |##
    350.8 |##
    353.7 |##
    356.5 |#####
    359.3 |###############
    362.2 |######################
    365.0 |##############################
    367.8 |###########################
  (13 below, 5 above range)

```
