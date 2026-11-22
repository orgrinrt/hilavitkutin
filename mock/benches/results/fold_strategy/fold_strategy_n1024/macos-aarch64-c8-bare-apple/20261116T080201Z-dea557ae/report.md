# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 28% faster than the next best (fold_paired)

fold_quad (54 ns) leads fold_paired (69 ns) by 28%, a clear separation rather than a photo finish. CV 6.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 22% (significant)

fold_quad is -15 ns (22%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_quad is fastest but the noisiest (CV 6.6%)

fold_quad wins on median (54 ns) yet has the highest variance (CV 6.6%), while fold_sequential is the steadiest (CV 2.7%, 108 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fold_quad shows alternating (throttle bounce) (autocorr -0.70)

fold_quad's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: fold_quad** at 54.2 ns median (-21.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.00x (fastest 54.2 ns, slowest 108.3 ns)

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
| fold_paired | 2181ns | 2177ns | 2171ns | 2178ns | 2199ns | base |
| fold_quad | 2163ns | 2157ns | 2150ns | 2157ns | 2191ns | -0.83% |
| fold_sequential | 2239ns | 2233ns | 2228ns | 2234ns | 2266ns | +2.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 70ns | 65ns | 75ns | base | 14.721 |
| fold_quad | 54ns | 50ns | 60ns | -21.65% | 18.789 |
| fold_sequential | 109ns | 105ns | 113ns | +55.99% | 9.437 |

## Performance model

- Peak throughput: **20.595 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 14.798 | 71.9% |
| fold_quad | 18.893 | 91.7% |
| fold_sequential | 9.455 | 45.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2181ns | 2181ns | base |
| fold_quad | 2163ns | 2163ns | -0.83% |
| fold_sequential | 2239ns | 2239ns | +2.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 69ns | base | --- | [68, 70] | --- | --- | --- | --- |
| fold_quad | 54ns | -15.0ns (-21.7%) | [-15, -14]ns | [54, 55] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 108ns | +39.2ns (+56.6%) | [+38, +40]ns | [108, 109] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 69ns | -22.6% | +56.5% |
| 1 | 2 | 70ns | -20.2% | +53.3% |
| 1 | 3 | 70ns | -23.8% | +56.3% |
| 1 | 4 | 70ns | -22.3% | +54.4% |
| 1 | 5 | 69ns | -21.6% | +56.1% |
| 1 | 6 | 70ns | -22.6% | +54.1% |
| 1 | 7 | 68ns | -20.4% | +59.9% |
| 1 | 8 | 67ns | -19.5% | +60.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | 0.344 | moderate+ |
| fold_quad | -0.701 | HIGH- (thermal bounce) |
| fold_sequential | -0.604 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.7ns | 69.6ns | 3.9% |  |
| fold_quad | 2.5ns | 54.5ns | 4.7% |  |
| fold_sequential | 2.8ns | 108.5ns | 2.6% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 64.7-75.4 ns)
     64.7 |#################
     65.2 |#################
     65.8 |##################################
     66.3 |#########################
     66.8 |########
     67.4 |#####################################
     67.9 |#########################
     68.4 |#################
     69.0 |####################
     69.5 |########################################
     70.0 |##############
     70.6 |##############
     71.1 |######################
     71.6 |#########################
     72.2 |#################
     72.7 |#################
     73.2 |##############
     73.8 |########
     74.3 |########
     74.8 |########
  (12 below, 10 above range)

fold_quad (n=160, range 49.7-59.9 ns)
     49.7 |#########
     50.2 |##
     50.7 |##############################
     51.3 |##################
     51.8 |##############
     52.3 |###########
     52.8 |##############
     53.3 |##################
     53.8 |########################################
     54.3 |##################
     54.8 |###########
     55.3 |##################
     55.8 |################
     56.3 |#######################
     56.9 |###########
     57.4 |#######
     57.9 |###########
     58.4 |####
     58.9 |####
     59.4 |#########
  (16 below, 17 above range)

fold_sequential (n=160, range 104.5-112.7 ns)
    104.5 |##############
    104.9 |##################
    105.3 |##############
    105.7 |#####################
    106.1 |#####################
    106.6 |####################################
    107.0 |#########################
    107.4 |#############################
    107.8 |########################################
    108.2 |####################################
    108.6 |#########################
    109.0 |#####################
    109.4 |#############################
    109.8 |#########################
    110.2 |#########################
    110.6 |################################
    111.0 |##################
    111.4 |##################
    111.9 |##########
    112.3 |##################
  (14 below, 13 above range)

```
