# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 29% faster than the next best (fold_paired)

fold_quad (52 ns) leads fold_paired (67 ns) by 29%, a clear separation rather than a photo finish. CV 7.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 22% (significant)

fold_quad is -15 ns (22%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 2.1x slower than the field

fold_sequential (108 ns) is 2.1x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_quad is fastest but the noisiest (CV 7.1%)

fold_quad wins on median (52 ns) yet has the highest variance (CV 7.1%), while fold_sequential is the steadiest (CV 2.9%, 108 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: fold_quad** at 52.1 ns median (-22.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.06x (fastest 52.1 ns, slowest 107.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x239fb80547ae61c2 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fold_paired | 2168ns | 2150ns | 2144ns | 2155ns | 2232ns | base |
| fold_quad | 2142ns | 2129ns | 2123ns | 2134ns | 2186ns | -1.19% |
| fold_sequential | 2231ns | 2209ns | 2201ns | 2218ns | 2299ns | +2.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 68ns | 63ns | 74ns | base | 15.122 |
| fold_quad | 53ns | 48ns | 59ns | -21.92% | 19.366 |
| fold_sequential | 107ns | 103ns | 112ns | +58.53% | 9.539 |

## Performance model

- Peak throughput: **21.149 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 15.261 | 72.2% |
| fold_quad | 19.655 | 92.9% |
| fold_sequential | 9.526 | 45.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2168ns | 2168ns | base |
| fold_quad | 2142ns | 2142ns | -1.19% |
| fold_sequential | 2231ns | 2231ns | +2.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 67ns | base | --- | [66, 68] | --- | --- | --- | --- |
| fold_quad | 52ns | -14.6ns (-21.8%) | [-15, -14]ns | [52, 53] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 108ns | +39.6ns (+59.0%) | [+39, +40]ns | [107, 108] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 67ns | -21.5% | +60.4% |
| 1 | 2 | 68ns | -22.8% | +55.9% |
| 1 | 3 | 67ns | -21.8% | +65.9% |
| 1 | 4 | 67ns | -23.3% | +58.6% |
| 1 | 5 | 69ns | -25.0% | +56.8% |
| 1 | 6 | 66ns | -22.3% | +59.8% |
| 1 | 7 | 67ns | -21.0% | +59.6% |
| 1 | 8 | 66ns | -22.9% | +60.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.294 | moderate- |
| fold_quad | -0.086 | ok |
| fold_sequential | -0.365 | moderate- |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.7ns | 67.7ns | 4.0% |  |
| fold_quad | 2.5ns | 52.9ns | 4.8% |  |
| fold_sequential | 2.9ns | 107.4ns | 2.7% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 63.0-73.9 ns)
     63.0 |######
     63.6 |######
     64.1 |########################################
     64.6 |######
     65.2 |##################
     65.7 |#############################
     66.3 |##########
     66.8 |##################
     67.4 |#############################
     67.9 |############
     68.5 |########
     69.0 |################
     69.5 |##################
     70.1 |##########
     70.6 |##
     71.2 |##########
     71.7 |########
     72.3 |######
     72.8 |######
     73.3 |######
  (17 below, 13 above range)

fold_quad (n=160, range 48.4-58.6 ns)
     48.4 |#############
     48.9 |#####################
     49.4 |#####################
     49.9 |########################################
     50.5 |#############################
     51.0 |##########
     51.5 |##########################
     52.0 |################################
     52.5 |########
     53.0 |#############
     53.5 |########
     54.0 |#####################
     54.5 |##################################
     55.1 |#############
     55.6 |#############
     56.1 |##########
     56.6 |#####
     57.1 |##########
     57.6 |#####
     58.1 |#####
  (15 below, 16 above range)

fold_sequential (n=160, range 103.2-111.8 ns)
    103.2 |###############
    103.6 |##########
    104.0 |##########
    104.5 |#########################
    104.9 |###############
    105.3 |######################
    105.8 |#################
    106.2 |############
    106.6 |############
    107.1 |####################
    107.5 |########################################
    107.9 |#################
    108.4 |####################
    108.8 |#########################
    109.2 |####################
    109.7 |#######
    110.1 |###############
    110.5 |##########
    111.0 |#######
    111.4 |#####
  (14 below, 15 above range)

```
