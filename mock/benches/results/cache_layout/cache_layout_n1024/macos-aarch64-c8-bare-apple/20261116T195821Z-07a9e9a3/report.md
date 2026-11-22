# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### layout_aos is fastest but the noisiest (CV 14.0%)

layout_aos wins on median (12 ns) yet has the highest variance (CV 14.0%), while layout_soa is the steadiest (CV 13.7%, 13 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (2 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (layout_aos)

The baseline layout_aos is the fastest (12 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader layout_aos vs stability leader layout_soa (+6% speed for 1.0x steadier)

layout_aos is fastest (12 ns, CV 14.0%); layout_soa gives up 6.4% median for 1.0x lower variance (CV 13.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (layout_aos) is the fastest** at 12.5 ns median
- Spread: 1.06x (fastest 12.5 ns, slowest 13.3 ns)

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
| layout_aos | 2101ns | 2098ns | 2093ns | 2098ns | 2119ns | base |
| layout_soa | 2113ns | 2102ns | 2095ns | 2106ns | 2154ns | +0.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 13ns | 10ns | 15ns | base | 81.683 |
| layout_soa | 13ns | 10ns | 15ns | +2.87% | 79.407 |

## Performance model

- Peak throughput: **101.418 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 81.920 | 80.8% |
| layout_soa | 76.992 | 75.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2101ns | 2101ns | base |
| layout_soa | 2113ns | 2113ns | +0.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 12ns | base | --- | [12, 13] | --- | --- | --- | --- |
| layout_soa | 13ns | no significant difference | [+0, +1]ns | [12, 13] | no | 0.0692 | 0.0692 | 13 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 12ns | +8.4% |
| 1 | 2 | 12ns | +0.0% |
| 1 | 3 | 13ns | +1.6% |
| 1 | 4 | 12ns | +6.4% |
| 1 | 5 | 12ns | +4.8% |
| 1 | 6 | 13ns | -4.5% |
| 1 | 7 | 13ns | +3.1% |
| 1 | 8 | 12ns | +5.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.041 | ok |
| layout_soa | -0.250 | moderate- |

**Consistency summary:**

- **layout_soa**: won 1/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 3.2ns | 12.5ns | 25.3% | HIGH |
| layout_soa | 3.0ns | 12.9ns | 23.4% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 10.1-15.0 ns)
     10.1 |
     10.3 |################
     10.6 |#######################
     10.8 |
     11.1 |#############################
     11.3 |
     11.6 |#####################
     11.8 |
     12.1 |##############
     12.3 |########################################
     12.6 |
     12.8 |#########################
     13.0 |
     13.3 |###################################
     13.5 |
     13.8 |#############################
     14.0 |################
     14.3 |
     14.5 |##################
     14.8 |######
  (16 below, 12 above range)

layout_soa (n=160, range 10.3-15.5 ns)
     10.3 |#######
     10.6 |###############
     10.8 |
     11.1 |###########
     11.4 |
     11.6 |##################
     11.9 |##############
     12.1 |
     12.4 |############
     12.6 |
     12.9 |##############
     13.2 |########################################
     13.4 |
     13.7 |###############
     13.9 |
     14.2 |##########
     14.4 |#################
     14.7 |
     14.9 |##############
     15.2 |##
  (13 below, 11 above range)

```

## Diagnostics

- **layout_aos**: bridge=26.4% of algo (FFI overhead may distort results)
- **layout_soa**: bridge=21.8% of algo (FFI overhead may distort results)
