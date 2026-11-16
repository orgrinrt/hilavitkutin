# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### layout_aos is fastest but the noisiest (CV 52.9%)

layout_aos wins on median (8 ns) yet has the highest variance (CV 52.9%), while layout_soa is the steadiest (CV 33.0%, 8 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (8 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader layout_aos vs stability leader layout_soa (+5% speed for 1.6x steadier)

layout_aos is fastest (8 ns, CV 52.9%); layout_soa gives up 5.3% median for 1.6x lower variance (CV 33.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### layout_aos is inconsistent: worst-20% is 3.0x its best-20%

layout_aos's best 20% of batches run at 5 ns but its worst 20% at 14 ns (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (layout_aos) is the fastest** at 7.5 ns median
- Spread: 1.05x (fastest 7.5 ns, slowest 7.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | AoS vs SoA (column-store) layout under partial-field iteration |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eadbeef01234567 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2110ns | 2111ns | 2080ns | 2108ns | 2145ns | base |
| layout_soa | 2120ns | 2105ns | 2083ns | 2108ns | 2192ns | +0.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 8ns | 5ns | 14ns | base | 122.819 |
| layout_soa | 8ns | 5ns | 12ns | -2.62% | 126.128 |

## Performance model

- Peak throughput: **221.405 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 136.533 | 61.7% |
| layout_soa | 129.620 | 58.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2110ns | 2110ns | base |
| layout_soa | 2120ns | 2120ns | +0.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 8ns | base | --- | [7, 8] | --- | --- | --- | --- |
| layout_soa | 8ns | no significant difference | [-0, +0]ns | [8, 8] | no | 0.7399 | 0.7399 | 15 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 13ns | -42.1% |
| 1 | 2 | 8ns | +66.7% |
| 1 | 3 | 8ns | +2.5% |
| 1 | 4 | 7ns | -5.5% |
| 1 | 5 | 7ns | +8.7% |
| 1 | 6 | 8ns | -5.3% |
| 1 | 7 | 8ns | +10.7% |
| 1 | 8 | 7ns | -2.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.009 | ok |
| layout_soa | -0.014 | ok |

**Consistency summary:**

- **layout_soa**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 2.9ns | 8.3ns | 34.8% | HIGH |
| layout_soa | 2.9ns | 8.1ns | 35.6% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 4.6-13.8 ns)
      4.6 |################
      5.1 |##########
      5.5 |################
      6.0 |####################
      6.5 |#########################
      6.9 |################
      7.4 |#####################
      7.8 |########################################
      8.3 |
      8.8 |####################
      9.2 |############
      9.7 |#######
     10.1 |#######
     10.6 |##########
     11.1 |#########
     11.5 |
     12.0 |###
     12.4 |#####
     12.9 |#########
     13.4 |###
  (12 below, 7 above range)

layout_soa (n=160, range 5.0-12.2 ns)
      5.0 |##########
      5.4 |################
      5.7 |#######################
      6.1 |################
      6.4 |###########################
      6.8 |############
      7.2 |#############################
      7.5 |
      7.9 |#####################
      8.2 |########################################
      8.6 |#########################
      9.0 |##########
      9.3 |######
      9.7 |################
     10.0 |
     10.4 |######
     10.8 |########
     11.1 |##
     11.5 |######
     11.8 |##
  (11 below, 15 above range)

```

## Diagnostics

- **layout_aos**: CV=47.6% (high variance, measurements may be unstable)
- **layout_aos**: bridge=38.7% of algo (FFI overhead may distort results)
- **layout_soa**: CV=32.1% (high variance, measurements may be unstable)
- **layout_soa**: bridge=36.7% of algo (FFI overhead may distort results)
