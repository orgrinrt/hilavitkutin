# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_index is fastest but the noisiest (CV 5.7%)

bc_index wins on median (93 ns) yet has the highest variance (CV 5.7%), while bc_iter is the steadiest (CV 5.2%, 94 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (bc_index, bc_iter) are a dead heat (<1%)

bc_index (93 ns) and bc_iter (94 ns) differ by 0.96%, inside the noise, even though the wider field spreads 55.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (bc_index)

The baseline bc_index is the fastest (93 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader bc_index vs stability leader bc_iter (+1% speed for 1.1x steadier)

bc_index is fastest (93 ns, CV 5.7%); bc_iter gives up 1.0% median for 1.1x lower variance (CV 5.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bc_iter's edge over baseline is significant but tiny (1 ns, 0.96%)

bc_iter differs from baseline bc_index by 1 ns (0.96%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (bc_index) is the fastest** at 93.3 ns median
- 2 variants significantly slower than baseline
- Spread: 1.55x (fastest 93.3 ns, slowest 144.6 ns)

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
| bc_index | 2173ns | 2159ns | 2148ns | 2168ns | 2214ns | base |
| bc_iter | 2182ns | 2190ns | 2152ns | 2182ns | 2212ns | +0.42% |
| bc_unchecked | 2239ns | 2244ns | 2206ns | 2236ns | 2280ns | +3.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 94ns | 87ns | 102ns | base | 43.583 |
| bc_iter | 95ns | 89ns | 103ns | +1.23% | 43.053 |
| bc_unchecked | 145ns | 139ns | 154ns | +54.77% | 28.160 |

## Performance model

- Peak throughput: **46.905 Gops/s** (bc_index; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 43.901 | 93.6% |
| bc_iter | 43.482 | 92.7% |
| bc_unchecked | 28.326 | 60.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2173ns | 2173ns | base |
| bc_iter | 2182ns | 2182ns | +0.42% |
| bc_unchecked | 2239ns | 2239ns | +3.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 93ns | base | --- | [92, 95] | --- | --- | --- | --- |
| bc_iter | 94ns | +0.9ns (+1.0%) | [+0, +1]ns | [93, 95] | YES | 0.0007 | 0.0007 | 5 |
| bc_unchecked | 145ns | +50.8ns (+54.4%) | [+50, +51]ns | [144, 145] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 95ns | -0.4% | +52.2% |
| 1 | 2 | 96ns | -2.4% | +52.5% |
| 1 | 3 | 95ns | -0.0% | +51.5% |
| 1 | 4 | 91ns | +3.9% | +58.6% |
| 1 | 5 | 93ns | +0.0% | +56.4% |
| 1 | 6 | 94ns | -0.3% | +53.8% |
| 1 | 7 | 94ns | +0.5% | +53.0% |
| 1 | 8 | 92ns | +2.9% | +57.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | 0.189 | ok |
| bc_iter | -0.127 | ok |
| bc_unchecked | -0.455 | moderate- |

**Consistency summary:**

- **bc_iter**: won 3/8, lost 3/8
- **bc_unchecked**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 2.3ns | 94.0ns | 2.5% |  |
| bc_iter | 2.3ns | 95.1ns | 2.4% |  |
| bc_unchecked | 2.4ns | 145.5ns | 1.6% |  |

## Distribution (algo ns)

```
bc_index (n=160, range 87.3-101.7 ns)
     87.3 |################
     88.0 |#############
     88.8 |#####
     89.5 |################################
     90.2 |##################
     90.9 |################
     91.6 |########################################
     92.3 |################################
     93.1 |##########
     93.8 |#####################
     94.5 |#####################
     95.2 |########################
     95.9 |########
     96.6 |##########################
     97.4 |################
     98.1 |#####
     98.8 |################
     99.5 |################
    100.2 |##################
    100.9 |#####
  (14 below, 10 above range)

bc_iter (n=160, range 89.0-102.6 ns)
     89.0 |########
     89.7 |####################
     90.4 |############################
     91.1 |############################
     91.7 |######################
     92.4 |########################################
     93.1 |####################
     93.8 |############################
     94.5 |#################
     95.1 |############################
     95.8 |########
     96.5 |############################
     97.2 |###########
     97.9 |##############
     98.6 |######################
     99.2 |#####
     99.9 |###########
    100.6 |####################
    101.3 |########
    102.0 |###########
  (13 below, 12 above range)

bc_unchecked (n=160, range 139.0-154.4 ns)
    139.0 |############
    139.7 |##############
    140.5 |######################
    141.3 |##############
    142.1 |############
    142.8 |####################
    143.6 |##################
    144.4 |########################################
    145.1 |########################
    145.9 |####################
    146.7 |##############
    147.5 |##########
    148.2 |##############
    149.0 |####################
    149.8 |########
    150.5 |########
    151.3 |
    152.1 |######
    152.9 |####
    153.6 |
  (14 below, 6 above range)

```
