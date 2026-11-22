# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_index is fastest but the noisiest (CV 13.5%)

bc_index wins on median (36 ns) yet has the highest variance (CV 13.5%), while bc_unchecked is the steadiest (CV 13.1%, 38 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (5 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (36 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader bc_index vs stability leader bc_unchecked (+6% speed for 1.0x steadier)

bc_index is fastest (36 ns, CV 13.5%); bc_unchecked gives up 5.9% median for 1.0x lower variance (CV 13.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (bc_index) is the fastest** at 35.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 35.8 ns, slowest 37.9 ns)

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
| bc_index | 2124ns | 2112ns | 2104ns | 2116ns | 2166ns | base |
| bc_iter | 2119ns | 2111ns | 2105ns | 2113ns | 2153ns | -0.21% |
| bc_unchecked | 2126ns | 2115ns | 2108ns | 2118ns | 2167ns | +0.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 36ns | 30ns | 44ns | base | 28.244 |
| bc_iter | 37ns | 31ns | 44ns | +1.82% | 27.739 |
| bc_unchecked | 38ns | 31ns | 45ns | +4.44% | 27.045 |

## Performance model

- Peak throughput: **34.165 Gops/s** (bc_index; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 28.603 | 83.7% |
| bc_iter | 28.287 | 82.8% |
| bc_unchecked | 27.018 | 79.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2124ns | 2124ns | base |
| bc_iter | 2119ns | 2119ns | -0.21% |
| bc_unchecked | 2126ns | 2126ns | +0.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 36ns | base | --- | [35, 37] | --- | --- | --- | --- |
| bc_iter | 36ns | no significant difference | [+0, +1]ns | [35, 38] | no (adj: YES) | 0.0272 | 0.0272 | 10 |
| bc_unchecked | 38ns | +1.7ns (+4.7%) | [+1, +2]ns | [37, 39] | YES | 0.0000 | 0.0000 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 37ns | +3.4% | +6.4% |
| 1 | 2 | 37ns | +4.5% | +3.8% |
| 1 | 3 | 35ns | +1.1% | +4.7% |
| 1 | 4 | 35ns | +1.1% | +7.1% |
| 1 | 5 | 36ns | +0.1% | +4.9% |
| 1 | 6 | 36ns | -1.1% | +7.0% |
| 1 | 7 | 35ns | +2.3% | +8.9% |
| 1 | 8 | 35ns | +6.0% | +7.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | 0.139 | ok |
| bc_iter | 0.229 | moderate+ |
| bc_unchecked | 0.007 | ok |

**Consistency summary:**

- **bc_iter**: won 1/8, lost 7/8
- **bc_unchecked**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.4ns | 36.3ns | 6.6% | HIGH |
| bc_iter | 2.4ns | 36.9ns | 6.4% | HIGH |
| bc_unchecked | 2.4ns | 37.9ns | 6.3% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 30.0-43.5 ns)
     30.0 |####################
     30.6 |#################
     31.3 |########
     32.0 |######################
     32.7 |#####################################
     33.4 |###########
     34.0 |########################################
     34.7 |####################
     35.4 |###############################
     36.1 |#########################
     36.7 |####################
     37.4 |####################
     38.1 |###############################
     38.8 |##
     39.5 |############################
     40.1 |####################
     40.8 |##
     41.5 |###########
     42.2 |##
     42.8 |########
  (11 below, 15 above range)

bc_iter (n=160, range 30.6-44.2 ns)
     30.6 |##############
     31.2 |########
     31.9 |#################
     32.6 |#########################
     33.3 |#########################
     34.0 |########################################
     34.6 |###########
     35.3 |##################################
     36.0 |##############
     36.7 |######################
     37.4 |###############################
     38.0 |###########
     38.7 |#########################
     39.4 |##############
     40.1 |##############
     40.8 |####################
     41.4 |##############
     42.1 |##
     42.8 |############################
     43.5 |##
  (14 below, 13 above range)

bc_unchecked (n=160, range 31.0-45.0 ns)
     31.0 |##############
     31.7 |##
     32.4 |####################
     33.1 |#################
     33.8 |########
     34.5 |###############################
     35.2 |#########################
     35.9 |##############
     36.6 |########################################
     37.3 |#################
     38.0 |####################
     38.7 |########################################
     39.4 |#########################
     40.1 |#########################
     40.8 |##############
     41.5 |##
     42.2 |#################
     42.9 |############################
     43.6 |########
     44.3 |
  (15 below, 14 above range)

```
