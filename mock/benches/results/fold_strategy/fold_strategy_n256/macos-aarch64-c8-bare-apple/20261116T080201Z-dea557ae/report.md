# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader fold_quad vs stability leader fold_sequential (+4% speed for 1.2x steadier)

fold_quad is fastest (10 ns, CV 22.7%); fold_sequential gives up 4.2% median for 1.2x lower variance (CV 19.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### fold_paired is inconsistent: worst-20% is 2.1x its best-20%

fold_paired's best 20% of batches run at 7 ns but its worst 20% at 15 ns (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 9.6 ns median (-4.0% vs baseline)
- Spread: 1.04x (fastest 9.6 ns, slowest 10.0 ns)

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
| fold_paired | 2136ns | 2129ns | 2121ns | 2129ns | 2172ns | base |
| fold_quad | 2132ns | 2128ns | 2117ns | 2129ns | 2158ns | -0.18% |
| fold_sequential | 2137ns | 2131ns | 2126ns | 2131ns | 2166ns | +0.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 11ns | 7ns | 15ns | base | 23.974 |
| fold_quad | 10ns | 7ns | 13ns | -5.76% | 25.439 |
| fold_sequential | 10ns | 7ns | 13ns | -5.95% | 25.492 |

## Performance model

- Peak throughput: **35.371 Gops/s** (fold_paired; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 25.600 | 72.4% |
| fold_quad | 26.667 | 75.4% |
| fold_sequential | 25.600 | 72.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2136ns | 2136ns | base |
| fold_quad | 2132ns | 2132ns | -0.18% |
| fold_sequential | 2137ns | 2137ns | +0.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 10ns | base | --- | [10, 11] | --- | --- | --- | --- |
| fold_quad | 10ns | no significant difference | [-1, +0]ns | [10, 10] | no | 0.6208 | 0.6208 | 13 |
| fold_sequential | 10ns | no significant difference | [-1, +0]ns | [10, 10] | no | 0.6208 | 0.5124 | 11 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 14ns | -33.8% | -26.8% |
| 1 | 2 | 10ns | +4.2% | +6.2% |
| 1 | 3 | 9ns | +13.0% | +8.7% |
| 1 | 4 | 10ns | -7.7% | +1.9% |
| 1 | 5 | 11ns | -12.5% | -12.5% |
| 1 | 6 | 10ns | -4.0% | -4.0% |
| 1 | 7 | 10ns | +2.1% | +6.2% |
| 1 | 8 | 9ns | +4.5% | +9.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.042 | ok |
| fold_quad | -0.058 | ok |
| fold_sequential | -0.152 | ok |

**Consistency summary:**

- **fold_quad**: won 4/8, lost 4/8
- **fold_sequential**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.7ns | 10.7ns | 24.9% | HIGH |
| fold_quad | 2.6ns | 10.1ns | 25.9% | HIGH |
| fold_sequential | 2.7ns | 10.0ns | 26.9% | HIGH |

## Distribution (algo ns)

```
fold_paired (n=160, range 7.2-15.4 ns)
      7.2 |####
      7.6 |########
      8.1 |##############
      8.5 |########################################
      8.9 |#####################
      9.3 |##################
      9.7 |###########################
     10.1 |#########################
     10.5 |################
     10.9 |#######################
     11.3 |##############
     11.7 |##########
     12.1 |############
     12.5 |################
     12.9 |####
     13.3 |
     13.8 |######
     14.2 |########
     14.6 |####
     15.0 |##
  (17 below, 10 above range)

fold_quad (n=160, range 7.3-13.3 ns)
      7.3 |########################
      7.6 |##################################
      7.9 |
      8.2 |##########################
      8.5 |########################
      8.8 |
      9.1 |########################################
      9.4 |########################################
      9.7 |##########################
     10.0 |
     10.3 |##########################
     10.6 |################################
     10.9 |#####################
     11.2 |
     11.5 |################################
     11.8 |########
     12.1 |
     12.4 |#############
     12.7 |#####
     13.0 |##########
  (10 below, 13 above range)

fold_sequential (n=160, range 7.4-12.8 ns)
      7.4 |#################
      7.7 |######################
      8.0 |
      8.2 |####################
      8.5 |
      8.8 |########################################
      9.0 |####################
      9.3 |
      9.6 |##########################
      9.9 |#################
     10.1 |
     10.4 |##########################
     10.7 |############################
     10.9 |
     11.2 |############################
     11.5 |########################
     11.7 |
     12.0 |###########
     12.3 |#################
     12.5 |
  (10 below, 14 above range)

```

## Diagnostics

- **fold_paired**: CV=31.3% (high variance, measurements may be unstable)
- **fold_paired**: bridge=25.0% of algo (FFI overhead may distort results)
- **fold_quad**: CV=21.7% (high variance, measurements may be unstable)
- **fold_quad**: bridge=26.0% of algo (FFI overhead may distort results)
- **fold_sequential**: bridge=29.0% of algo (FFI overhead may distort results)
