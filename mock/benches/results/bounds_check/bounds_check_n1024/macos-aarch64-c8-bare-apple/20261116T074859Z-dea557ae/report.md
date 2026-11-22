# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_index is fastest but the noisiest (CV 14.0%)

bc_index wins on median (35 ns) yet has the highest variance (CV 14.0%), while bc_iter is the steadiest (CV 13.1%, 36 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (35 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader bc_index vs stability leader bc_iter (+5% speed for 1.1x steadier)

bc_index is fastest (35 ns, CV 14.0%); bc_iter gives up 4.6% median for 1.1x lower variance (CV 13.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (bc_index) is the fastest** at 34.6 ns median
- 2 variants significantly slower than baseline
- Spread: 1.14x (fastest 34.6 ns, slowest 39.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bounds-check elision: iter() vs explicit index vs get_unchecked |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5a3ec07491bf2d68 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2112ns | 2101ns | 2091ns | 2101ns | 2164ns | base |
| bc_iter | 2115ns | 2104ns | 2093ns | 2107ns | 2162ns | +0.14% |
| bc_unchecked | 2119ns | 2105ns | 2098ns | 2111ns | 2165ns | +0.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 35ns | 29ns | 43ns | base | 29.209 |
| bc_iter | 36ns | 30ns | 43ns | +3.75% | 28.153 |
| bc_unchecked | 40ns | 33ns | 47ns | +13.58% | 25.715 |

## Performance model

- Peak throughput: **35.695 Gops/s** (bc_index; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 29.595 | 82.9% |
| bc_iter | 28.287 | 79.2% |
| bc_unchecked | 25.990 | 72.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2112ns | 2112ns | base |
| bc_iter | 2115ns | 2115ns | +0.14% |
| bc_unchecked | 2119ns | 2119ns | +0.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 35ns | base | --- | [34, 35] | --- | --- | --- | --- |
| bc_iter | 36ns | +1.2ns (+3.6%) | [+1, +2]ns | [35, 37] | YES | 0.0014 | 0.0014 | 10 |
| bc_unchecked | 39ns | +5.0ns (+14.5%) | [+4, +6]ns | [38, 40] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 33ns | +12.8% | +21.1% |
| 1 | 2 | 36ns | +0.7% | +15.1% |
| 1 | 3 | 36ns | -2.9% | +2.4% |
| 1 | 4 | 34ns | -1.3% | +15.7% |
| 1 | 5 | 34ns | +6.0% | +19.9% |
| 1 | 6 | 34ns | +10.9% | +18.8% |
| 1 | 7 | 33ns | +7.6% | +15.1% |
| 1 | 8 | 38ns | -3.8% | +3.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.233 | moderate- |
| bc_iter | 0.117 | ok |
| bc_unchecked | -0.222 | moderate- |

**Consistency summary:**

- **bc_iter**: won 3/8, lost 5/8
- **bc_unchecked**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.9ns | 35.1ns | 8.2% | HIGH |
| bc_iter | 2.6ns | 36.4ns | 7.0% | HIGH |
| bc_unchecked | 2.4ns | 39.8ns | 6.1% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 28.7-42.6 ns)
     28.7 |######################
     29.4 |#################
     30.1 |#####
     30.8 |####################
     31.5 |###############################
     32.2 |####################
     32.9 |####################
     33.5 |########################################
     34.2 |##############
     34.9 |########################################
     35.6 |############################
     36.3 |###########
     37.0 |#########################
     37.7 |##############
     38.4 |
     39.1 |##############
     39.8 |###########
     40.5 |###########
     41.2 |#################
     41.9 |####################
  (15 below, 10 above range)

bc_iter (n=160, range 30.1-43.3 ns)
     30.1 |########
     30.7 |#################
     31.4 |#####
     32.0 |###############################
     32.7 |###############################
     33.4 |##############
     34.0 |###############################
     34.7 |######################
     35.4 |#################
     36.0 |##############
     36.7 |########################################
     37.3 |####################
     38.0 |###########
     38.7 |####################
     39.3 |######################
     40.0 |#################
     40.6 |#########################
     41.3 |##
     42.0 |#################
     42.6 |
  (15 below, 15 above range)

bc_unchecked (n=160, range 32.9-47.4 ns)
     32.9 |########################
     33.6 |######
     34.3 |########################
     35.1 |###############
     35.8 |########################
     36.5 |########################################
     37.2 |########################
     38.0 |#####################
     38.7 |###########################
     39.4 |########################
     40.2 |#####################
     40.9 |##################
     41.6 |##################
     42.3 |####################################
     43.1 |########################
     43.8 |#########
     44.5 |############
     45.3 |############
     46.0 |############
     46.7 |######
  (12 below, 16 above range)

```
