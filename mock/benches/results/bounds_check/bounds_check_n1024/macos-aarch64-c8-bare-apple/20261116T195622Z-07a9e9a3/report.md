# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_index is fastest but the noisiest (CV 13.3%)

bc_index wins on median (36 ns) yet has the highest variance (CV 13.3%), while bc_iter is the steadiest (CV 12.5%, 37 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (36 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader bc_index vs stability leader bc_iter (+3% speed for 1.1x steadier)

bc_index is fastest (36 ns, CV 13.3%); bc_iter gives up 3.1% median for 1.1x lower variance (CV 12.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 4.2% of the fastest

All 3 variants sit between 36 ns and 38 ns - a 4.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (bc_index) is the fastest** at 36.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 36.0 ns, slowest 37.5 ns)

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
| bc_index | 2120ns | 2110ns | 2105ns | 2111ns | 2164ns | base |
| bc_iter | 2118ns | 2110ns | 2103ns | 2110ns | 2159ns | -0.08% |
| bc_unchecked | 2127ns | 2116ns | 2110ns | 2119ns | 2167ns | +0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 37ns | 30ns | 43ns | base | 28.012 |
| bc_iter | 37ns | 30ns | 43ns | +0.99% | 27.736 |
| bc_unchecked | 38ns | 32ns | 45ns | +3.66% | 27.024 |

## Performance model

- Peak throughput: **34.034 Gops/s** (bc_index; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 28.444 | 83.6% |
| bc_iter | 27.601 | 81.1% |
| bc_unchecked | 27.307 | 80.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2120ns | 2120ns | base |
| bc_iter | 2118ns | 2118ns | -0.08% |
| bc_unchecked | 2127ns | 2127ns | +0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 36ns | base | --- | [35, 37] | --- | --- | --- | --- |
| bc_iter | 37ns | no significant difference | [+0, +1]ns | [36, 38] | no | 0.1401 | 0.1401 | 11 |
| bc_unchecked | 38ns | +1.2ns (+3.5%) | [+1, +2]ns | [37, 39] | YES | 0.0000 | 0.0000 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 37ns | +2.9% | +0.7% |
| 1 | 2 | 36ns | +3.6% | +4.7% |
| 1 | 3 | 37ns | +0.1% | +7.0% |
| 1 | 4 | 36ns | +3.5% | +1.8% |
| 1 | 5 | 37ns | +0.7% | +3.4% |
| 1 | 6 | 36ns | +0.6% | +0.0% |
| 1 | 7 | 35ns | +5.9% | +5.4% |
| 1 | 8 | 36ns | +1.8% | +7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.019 | ok |
| bc_iter | -0.358 | moderate- |
| bc_unchecked | -0.340 | moderate- |

**Consistency summary:**

- **bc_iter**: won 0/8, lost 8/8
- **bc_unchecked**: won 0/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.2ns | 36.6ns | 6.1% | HIGH |
| bc_iter | 2.4ns | 36.9ns | 6.6% | HIGH |
| bc_unchecked | 2.4ns | 37.9ns | 6.4% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 30.1-43.5 ns)
     30.1 |########
     30.8 |#################
     31.4 |#####
     32.1 |###############################
     32.8 |#########################
     33.4 |###########
     34.1 |############################
     34.8 |########################################
     35.4 |######################
     36.1 |######################
     36.8 |#################
     37.4 |##################################
     38.1 |#####
     38.8 |###########
     39.4 |######################
     40.1 |#####
     40.8 |#########################
     41.4 |#########################
     42.1 |########
     42.8 |##############
  (13 below, 12 above range)

bc_iter (n=160, range 30.4-43.5 ns)
     30.4 |#########
     31.0 |###############
     31.7 |#####################
     32.3 |##################
     33.0 |###############
     33.6 |##############################
     34.3 |##################
     35.0 |##############################
     35.6 |#####################
     36.3 |###############
     36.9 |########################################
     37.6 |##################
     38.2 |########################################
     38.9 |######
     39.5 |##################
     40.2 |#################################
     40.9 |###############
     41.5 |##################
     42.2 |######
     42.8 |############
  (13 below, 15 above range)

bc_unchecked (n=160, range 31.6-44.8 ns)
     31.6 |#########################
     32.3 |########################################
     33.0 |#####################
     33.6 |#########################
     34.3 |##############
     34.9 |#############################
     35.6 |################################
     36.2 |#############################
     36.9 |########################################
     37.5 |##############
     38.2 |########################################
     38.9 |##########
     39.5 |####################################
     40.2 |#############################
     40.8 |##########
     41.5 |########################################
     42.1 |##########
     42.8 |##################
     43.5 |
     44.1 |#########################
  (11 below, 13 above range)

```
