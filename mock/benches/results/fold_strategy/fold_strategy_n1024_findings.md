# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 26% faster than the next best (fold_paired)

fold_quad (58 ns) leads fold_paired (73 ns) by 26%, a clear separation rather than a photo finish. CV 9.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 22% (significant)

fold_quad is -16 ns (22%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_quad is fastest but the noisiest (CV 9.4%)

fold_quad wins on median (58 ns) yet has the highest variance (CV 9.4%), while fold_sequential is the steadiest (CV 7.0%, 113 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fold_sequential shows alternating (throttle bounce) (autocorr -0.62)

fold_sequential's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: fold_quad** at 58.3 ns median (-20.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.93x (fastest 58.3 ns, slowest 112.7 ns)

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
| fold_paired | 2333ns | 2299ns | 2135ns | 2350ns | 2478ns | base |
| fold_quad | 2353ns | 2443ns | 2124ns | 2395ns | 2456ns | +0.88% |
| fold_sequential | 2358ns | 2352ns | 2190ns | 2349ns | 2552ns | +1.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 74ns | 66ns | 82ns | base | 13.899 |
| fold_quad | 58ns | 51ns | 66ns | -20.94% | 17.581 |
| fold_sequential | 114ns | 105ns | 126ns | +55.40% | 8.944 |

## Performance model

- Peak throughput: **20.257 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 13.970 | 69.0% |
| fold_quad | 17.564 | 86.7% |
| fold_sequential | 9.086 | 44.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2333ns | 2333ns | base |
| fold_quad | 2353ns | 2353ns | +0.88% |
| fold_sequential | 2358ns | 2358ns | +1.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 73ns | base | --- | [72, 75] | --- | --- | --- | --- |
| fold_quad | 58ns | -16.2ns (-22.2%) | [-17, -15]ns | [57, 59] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 113ns | +41.2ns (+56.2%) | [+40, +44]ns | [111, 114] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 76ns | -19.8% | +42.3% |
| 1 | 2 | 76ns | -23.5% | +58.3% |
| 1 | 3 | 77ns | -20.9% | +39.3% |
| 1 | 4 | 71ns | -14.0% | +62.6% |
| 1 | 5 | 75ns | -24.9% | +52.5% |
| 1 | 6 | 72ns | -29.4% | +67.4% |
| 1 | 7 | 69ns | -13.0% | +54.7% |
| 1 | 8 | 67ns | -20.1% | +69.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | 0.404 | moderate+ |
| fold_quad | -0.051 | ok |
| fold_sequential | -0.621 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.9ns | 73.7ns | 4.0% |  |
| fold_quad | 2.8ns | 58.2ns | 4.8% |  |
| fold_sequential | 2.9ns | 114.5ns | 2.6% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 66.0-82.1 ns)
     66.0 |############
     66.8 |########################
     67.6 |########################################
     68.4 |################################
     69.2 |####################
     70.0 |################################
     70.8 |####################
     71.6 |################################
     72.4 |################################
     73.2 |####################################
     74.0 |############################
     74.9 |####################
     75.7 |####################################
     76.5 |########################
     77.3 |################################
     78.1 |################
     78.9 |########################################
     79.7 |############
     80.5 |################
     81.3 |########
  (17 below, 15 above range)

fold_quad (n=160, range 50.5-66.1 ns)
     50.5 |#######
     51.3 |##
     52.1 |##########
     52.9 |############
     53.7 |####################
     54.4 |###########################
     55.2 |####################
     56.0 |####################
     56.8 |#################
     57.6 |########################################
     58.3 |############
     59.1 |###############
     59.9 |###############
     60.7 |#################
     61.5 |##############################
     62.2 |############
     63.0 |#################
     63.8 |##
     64.6 |#####
     65.4 |##########
  (17 below, 17 above range)

fold_sequential (n=160, range 104.9-126.4 ns)
    104.9 |##############################
    106.0 |##############################
    107.0 |#################################
    108.1 |#################################
    109.2 |##############################
    110.3 |##############################
    111.3 |##############################
    112.4 |##############################
    113.5 |####################
    114.5 |################
    115.6 |##########
    116.7 |######
    117.8 |################
    118.8 |#############
    119.9 |################
    121.0 |#############
    122.1 |####################
    123.1 |########################################
    124.2 |###
    125.3 |################
  (13 below, 15 above range)

```
