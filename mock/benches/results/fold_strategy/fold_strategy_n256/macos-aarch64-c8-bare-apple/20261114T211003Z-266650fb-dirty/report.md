# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_paired dominates: 16% faster than the next best (fold_quad)

fold_paired (13 ns) leads fold_quad (15 ns) by 16%, a clear separation rather than a photo finish. CV 31.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_paired is fastest but the noisiest (CV 31.7%)

fold_paired wins on median (13 ns) yet has the highest variance (CV 31.7%), while fold_sequential is the steadiest (CV 16.3%, 16 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fold_sequential shows warm-up / thermal drift (autocorr +0.54)

fold_sequential's per-pass series has lag-1 autocorrelation +0.54, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (fold_paired)

The baseline fold_paired is the fastest (13 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### fold_paired is inconsistent: worst-20% is 2.4x its best-20%

fold_paired's best 20% of batches run at 8 ns but its worst 20% at 20 ns (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fold_paired) is the fastest** at 13.3 ns median
- 2 variants significantly slower than baseline
- Spread: 1.24x (fastest 13.3 ns, slowest 16.4 ns)

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
| fold_paired | 2298ns | 2402ns | 2083ns | 2326ns | 2431ns | base |
| fold_quad | 2249ns | 2240ns | 2086ns | 2246ns | 2421ns | -2.15% |
| fold_sequential | 2322ns | 2415ns | 2114ns | 2354ns | 2435ns | +1.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 14ns | 8ns | 20ns | base | 18.798 |
| fold_quad | 15ns | 9ns | 21ns | +13.58% | 16.551 |
| fold_sequential | 17ns | 13ns | 21ns | +22.65% | 15.326 |

## Performance model

- Peak throughput: **30.682 Gops/s** (fold_paired; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 19.248 | 62.7% |
| fold_quad | 16.623 | 54.2% |
| fold_sequential | 15.562 | 50.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2298ns | 2298ns | base |
| fold_quad | 2249ns | 2249ns | -2.15% |
| fold_sequential | 2322ns | 2322ns | +1.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 13ns | base | --- | [12, 15] | --- | --- | --- | --- |
| fold_quad | 15ns | +0.8ns (+6.4%) | [+0, +1]ns | [15, 16] | YES | 0.0095 | 0.0095 | 7 |
| fold_sequential | 16ns | +2.9ns (+21.8%) | [+2, +4]ns | [16, 17] | YES | 0.0000 | 0.0000 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 9ns | +92.6% | +61.7% |
| 1 | 2 | 15ns | +2.6% | +2.6% |
| 1 | 3 | 16ns | +4.1% | +2.5% |
| 1 | 4 | 9ns | +88.3% | +68.1% |
| 1 | 5 | 17ns | -3.8% | -2.3% |
| 1 | 6 | 17ns | -3.0% | +7.2% |
| 1 | 7 | 10ns | +0.0% | +66.3% |
| 1 | 8 | 11ns | -13.2% | +57.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.188 | ok |
| fold_quad | 0.430 | moderate+ |
| fold_sequential | 0.543 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **fold_quad**: won 3/8, lost 4/8
- **fold_sequential**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.8ns | 13.6ns | 20.7% | HIGH |
| fold_quad | 2.8ns | 15.5ns | 17.9% | HIGH |
| fold_sequential | 3.1ns | 16.7ns | 18.4% | HIGH |

## Distribution (algo ns)

```
fold_paired (n=160, range 8.3-20.0 ns)
      8.3 |##############################
      8.9 |#####################
      9.5 |########################################
     10.1 |########################
     10.7 |##############################
     11.3 |#########
     11.8 |#########
     12.4 |########################
     13.0 |#####################
     13.6 |##################
     14.2 |#####################
     14.7 |#################################
     15.3 |###########################
     15.9 |##################
     16.5 |############
     17.1 |##################
     17.6 |############
     18.2 |############
     18.8 |############
     19.4 |############
  (13 below, 13 above range)

fold_quad (n=160, range 9.4-21.5 ns)
      9.4 |###############
     10.0 |##########
     10.6 |####################
     11.2 |##
     11.8 |###############
     12.4 |###############
     13.1 |#####
     13.7 |###################################
     14.3 |##########
     14.9 |########################################
     15.5 |###############
     16.1 |#################
     16.7 |#####################################
     17.3 |#######
     17.9 |##############################
     18.5 |#####
     19.1 |##########
     19.7 |#######
     20.3 |######################
     20.9 |##########
  (15 below, 13 above range)

fold_sequential (n=160, range 13.3-20.8 ns)
     13.3 |###########
     13.6 |#########################
     14.0 |###########
     14.4 |###############################
     14.8 |###############################
     15.1 |##############
     15.5 |##################################
     15.9 |############################
     16.3 |
     16.6 |########################################
     17.0 |##############
     17.4 |###########
     17.8 |##################################
     18.1 |#########################
     18.5 |####################
     18.9 |########
     19.3 |#################
     19.6 |#####
     20.0 |
     20.4 |###########
  (14 below, 14 above range)

```

## Diagnostics

- **fold_paired**: CV=31.0% (high variance, measurements may be unstable)
- **fold_paired**: bridge=20.3% of algo (FFI overhead may distort results)
- **fold_quad**: CV=27.0% (high variance, measurements may be unstable)
- **fold_quad**: bridge=18.8% of algo (FFI overhead may distort results)
- **fold_sequential**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **fold_sequential**: bridge=17.6% of algo (FFI overhead may distort results)
