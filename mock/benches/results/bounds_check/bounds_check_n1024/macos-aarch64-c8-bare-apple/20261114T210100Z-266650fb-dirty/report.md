# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_iter is fastest but the noisiest (CV 14.2%)

bc_iter wins on median (38 ns) yet has the highest variance (CV 14.2%), while bc_index is the steadiest (CV 13.5%, 40 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bc_iter vs stability leader bc_index (+4% speed for 1.1x steadier)

bc_iter is fastest (38 ns, CV 14.2%); bc_index gives up 4.4% median for 1.1x lower variance (CV 13.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bc_iter** at 38.3 ns median (-4.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.07x (fastest 38.3 ns, slowest 40.8 ns)

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
| bc_index | 2395ns | 2423ns | 2256ns | 2423ns | 2450ns | base |
| bc_iter | 2244ns | 2148ns | 2094ns | 2229ns | 2439ns | -6.31% |
| bc_unchecked | 2263ns | 2212ns | 2098ns | 2258ns | 2443ns | -5.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 40ns | 33ns | 48ns | base | 25.477 |
| bc_iter | 38ns | 31ns | 47ns | -4.42% | 26.655 |
| bc_unchecked | 41ns | 33ns | 49ns | +1.85% | 25.014 |

## Performance model

- Peak throughput: **32.703 Gops/s** (bc_iter; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 25.600 | 78.3% |
| bc_iter | 26.736 | 81.8% |
| bc_unchecked | 25.098 | 76.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2395ns | 2395ns | base |
| bc_iter | 2244ns | 2244ns | -6.31% |
| bc_unchecked | 2263ns | 2263ns | -5.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 40ns | base | --- | [39, 41] | --- | --- | --- | --- |
| bc_iter | 38ns | -1.7ns (-4.3%) | [-2, -1]ns | [38, 39] | YES | 0.0001 | 0.0000 | 7 |
| bc_unchecked | 41ns | no significant difference | [-0, +2]ns | [40, 42] | no | 0.2979 | 0.2979 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 42ns | -11.4% | -7.9% |
| 1 | 2 | 39ns | -7.1% | -2.3% |
| 1 | 3 | 39ns | +4.7% | +12.2% |
| 1 | 4 | 42ns | -7.9% | -13.6% |
| 1 | 5 | 41ns | -3.5% | -0.6% |
| 1 | 6 | 37ns | +5.0% | +16.3% |
| 1 | 7 | 39ns | -10.2% | +7.9% |
| 1 | 8 | 42ns | -11.8% | +6.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.095 | ok |
| bc_iter | 0.119 | ok |
| bc_unchecked | -0.105 | ok |

**Consistency summary:**

- **bc_iter**: won 6/8, lost 2/8
- **bc_unchecked**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 3.0ns | 40.2ns | 7.6% | HIGH |
| bc_iter | 2.9ns | 38.4ns | 7.5% | HIGH |
| bc_unchecked | 2.6ns | 40.9ns | 6.4% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 32.9-48.2 ns)
     32.9 |########
     33.7 |#####
     34.4 |########################
     35.2 |##################
     36.0 |#####################
     36.7 |########
     37.5 |########################
     38.3 |##################
     39.0 |########################################
     39.8 |##################################
     40.6 |##################
     41.3 |################
     42.1 |##################################
     42.9 |################
     43.6 |################
     44.4 |##########
     45.1 |##########
     45.9 |##
     46.7 |########
     47.4 |#############
  (15 below, 14 above range)

bc_iter (n=160, range 31.3-46.8 ns)
     31.3 |###
     32.1 |#####################
     32.9 |###########################
     33.6 |#####################
     34.4 |########################
     35.2 |########################
     35.9 |#####################
     36.7 |#########
     37.5 |########################################
     38.3 |#################################
     39.0 |########################################
     39.8 |####################################
     40.6 |#####################
     41.4 |###############
     42.1 |############
     42.9 |###
     43.7 |############
     44.4 |#########
     45.2 |############
     46.0 |#########
  (16 below, 14 above range)

bc_unchecked (n=160, range 33.1-49.2 ns)
     33.1 |#######
     33.9 |##############
     34.7 |#######
     35.5 |########################################
     36.3 |####################################
     37.1 |#########################
     37.9 |####################################
     38.7 |##############
     39.5 |#####################
     40.3 |########################################
     41.1 |#############################
     41.9 |#############################
     42.8 |#############################
     43.6 |########################################
     44.4 |##############
     45.2 |#####################
     46.0 |#############################
     46.8 |##################
     47.6 |#######
     48.4 |#######
  (16 below, 15 above range)

```
