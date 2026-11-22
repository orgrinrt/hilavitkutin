# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### fold_quad is inconsistent: worst-20% is 1.6x its best-20%

fold_quad's best 20% of batches run at 12 ns but its worst 20% at 20 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 2.7% of the fastest

All 3 variants sit between 15 ns and 15 ns - a 2.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: fold_sequential** at 15.0 ns median (-1.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.03x (fastest 15.0 ns, slowest 15.4 ns)

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
| fold_paired | 2103ns | 2101ns | 2096ns | 2101ns | 2118ns | base |
| fold_quad | 2104ns | 2100ns | 2095ns | 2101ns | 2121ns | +0.02% |
| fold_sequential | 2106ns | 2103ns | 2099ns | 2103ns | 2122ns | +0.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 16ns | 13ns | 19ns | base | 16.474 |
| fold_quad | 16ns | 12ns | 20ns | +2.46% | 16.079 |
| fold_sequential | 15ns | 12ns | 18ns | -3.23% | 17.023 |

## Performance model

- Peak throughput: **21.445 Gops/s** (fold_sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 16.842 | 78.5% |
| fold_quad | 16.623 | 77.5% |
| fold_sequential | 17.067 | 79.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2103ns | 2103ns | base |
| fold_quad | 2104ns | 2104ns | +0.02% |
| fold_sequential | 2106ns | 2106ns | +0.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 15ns | base | --- | [15, 16] | --- | --- | --- | --- |
| fold_quad | 15ns | no significant difference | [+0, +0]ns | [15, 16] | no (adj: YES) | 0.0395 | 0.0395 | 12 |
| fold_sequential | 15ns | -0.4ns (-2.6%) | [-1, -0]ns | [15, 15] | YES | 0.0082 | 0.0041 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 15ns | +11.0% | +0.0% |
| 1 | 2 | 15ns | +9.6% | +2.7% |
| 1 | 3 | 16ns | -6.4% | -10.3% |
| 1 | 4 | 15ns | +3.9% | -2.6% |
| 1 | 5 | 15ns | +5.3% | +3.9% |
| 1 | 6 | 15ns | +1.4% | +1.4% |
| 1 | 7 | 15ns | +1.3% | -1.3% |
| 1 | 8 | 16ns | -1.3% | -6.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | 0.076 | ok |
| fold_quad | -0.179 | ok |
| fold_sequential | -0.015 | ok |

**Consistency summary:**

- **fold_quad**: won 2/8, lost 6/8
- **fold_sequential**: won 4/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.6ns | 15.5ns | 16.5% | HIGH |
| fold_quad | 2.6ns | 15.9ns | 16.4% | HIGH |
| fold_sequential | 2.7ns | 15.0ns | 17.8% | HIGH |

## Distribution (algo ns)

```
fold_paired (n=160, range 12.5-19.4 ns)
     12.5 |
     12.9 |######################
     13.2 |##################################
     13.5 |############################
     13.9 |#####################################
     14.2 |
     14.6 |########################################
     14.9 |######################
     15.3 |#####################################
     15.6 |#####################################
     16.0 |#################
     16.3 |
     16.6 |########
     17.0 |####################
     17.3 |##############
     17.7 |##############
     18.0 |#########################
     18.4 |
     18.7 |###########
     19.1 |#####
  (15 below, 13 above range)

fold_quad (n=160, range 12.4-19.9 ns)
     12.4 |#################
     12.8 |###########
     13.2 |######################
     13.5 |##############
     13.9 |#########################
     14.3 |########################################
     14.7 |##################################
     15.0 |############################
     15.4 |
     15.8 |######################
     16.2 |######################
     16.5 |############################
     16.9 |##############
     17.3 |###############################
     17.7 |########
     18.0 |####################
     18.4 |
     18.8 |##
     19.2 |##############
     19.5 |##############
  (13 below, 16 above range)

fold_sequential (n=160, range 11.9-18.3 ns)
     11.9 |###########
     12.3 |################
     12.6 |
     12.9 |###########
     13.2 |#########################
     13.5 |################################
     13.8 |
     14.2 |#####################
     14.5 |#########################
     14.8 |########################################
     15.1 |##############
     15.4 |
     15.8 |#######################
     16.1 |##################
     16.4 |##################
     16.7 |
     17.0 |############################
     17.4 |##############
     17.7 |#########
     18.0 |##
  (13 below, 13 above range)

```

## Diagnostics

- **fold_paired**: bridge=16.4% of algo (FFI overhead may distort results)
- **fold_quad**: bridge=16.2% of algo (FFI overhead may distort results)
- **fold_sequential**: bridge=19.3% of algo (FFI overhead may distort results)
