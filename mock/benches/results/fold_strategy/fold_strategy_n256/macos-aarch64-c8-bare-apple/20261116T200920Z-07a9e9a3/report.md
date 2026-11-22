# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fold_paired) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fold_paired has the worst median (15 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest fold_quad at 15 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.7% (fastest 15 ns to slowest 15 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### fold_quad is fastest but the noisiest (CV 17.1%)

fold_quad wins on median (15 ns) yet has the highest variance (CV 17.1%), while fold_sequential is the steadiest (CV 15.5%, 15 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader fold_quad vs stability leader fold_sequential (+0% speed for 1.1x steadier)

fold_quad is fastest (15 ns, CV 17.1%); fold_sequential gives up 0.0% median for 1.1x lower variance (CV 15.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### fold_quad is inconsistent: worst-20% is 1.6x its best-20%

fold_quad's best 20% of batches run at 12 ns but its worst 20% at 19 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: fold_quad** at 15.0 ns median (-2.6% vs baseline)
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
| fold_paired | 2131ns | 2121ns | 2098ns | 2123ns | 2188ns | base |
| fold_quad | 2126ns | 2107ns | 2096ns | 2113ns | 2194ns | -0.22% |
| fold_sequential | 2129ns | 2108ns | 2099ns | 2119ns | 2186ns | -0.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 16ns | 12ns | 19ns | base | 16.385 |
| fold_quad | 16ns | 12ns | 19ns | -0.53% | 16.472 |
| fold_sequential | 15ns | 12ns | 19ns | -1.61% | 16.652 |

## Performance model

- Peak throughput: **21.103 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 16.623 | 78.8% |
| fold_quad | 17.067 | 80.9% |
| fold_sequential | 17.067 | 80.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2131ns | 2131ns | base |
| fold_quad | 2126ns | 2126ns | -0.22% |
| fold_sequential | 2129ns | 2129ns | -0.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 15ns | base | --- | [15, 16] | --- | --- | --- | --- |
| fold_quad | 15ns | no significant difference | [-0, +0]ns | [15, 15] | no | 0.8091 | 0.8091 | 6 |
| fold_sequential | 15ns | no significant difference | [-0, +0]ns | [15, 15] | no | 0.4349 | 0.2174 | 12 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 15ns | -1.3% | -3.9% |
| 1 | 2 | 16ns | -10.0% | -4.0% |
| 1 | 3 | 16ns | -3.7% | -6.2% |
| 1 | 4 | 15ns | +1.3% | -3.9% |
| 1 | 5 | 15ns | +0.0% | +0.0% |
| 1 | 6 | 15ns | +2.7% | +0.0% |
| 1 | 7 | 15ns | -1.3% | +2.6% |
| 1 | 8 | 15ns | +1.3% | +2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | 0.341 | moderate+ |
| fold_quad | -0.250 | moderate- |
| fold_sequential | 0.044 | ok |

**Consistency summary:**

- **fold_quad**: won 4/8, lost 3/8
- **fold_sequential**: won 4/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.4ns | 15.6ns | 15.6% | HIGH |
| fold_quad | 2.5ns | 15.5ns | 16.3% | HIGH |
| fold_sequential | 2.6ns | 15.4ns | 17.2% | HIGH |

## Distribution (algo ns)

```
fold_paired (n=160, range 12.4-19.4 ns)
     12.4 |###############
     12.8 |###############
     13.1 |#################
     13.5 |####################
     13.8 |
     14.2 |################################
     14.5 |####################
     14.9 |################################
     15.2 |########################################
     15.6 |############
     15.9 |###################################
     16.3 |
     16.6 |###########################
     17.0 |#######
     17.3 |############
     17.6 |###############
     18.0 |#######
     18.3 |
     18.7 |#####
     19.0 |#####
  (13 below, 19 above range)

fold_quad (n=160, range 12.1-19.4 ns)
     12.1 |
     12.5 |##################
     12.9 |##################
     13.2 |###############
     13.6 |####################################
     13.9 |#################################
     14.3 |####################################
     14.7 |########################################
     15.0 |
     15.4 |#################################
     15.7 |###############
     16.1 |############
     16.5 |########################
     16.8 |###########################
     17.2 |###############
     17.5 |######
     17.9 |
     18.3 |########################
     18.6 |########################################
     19.0 |######
  (17 below, 11 above range)

fold_sequential (n=160, range 12.4-18.9 ns)
     12.4 |####################
     12.8 |############################
     13.1 |#########################
     13.4 |
     13.7 |###############################
     14.1 |###############################
     14.4 |########################################
     14.7 |######################
     15.0 |
     15.4 |########################################
     15.7 |############################
     16.0 |######################
     16.3 |
     16.7 |######################
     17.0 |#################
     17.3 |####################
     17.6 |###########
     18.0 |
     18.3 |#################
     18.6 |##############
  (11 below, 11 above range)

```

## Diagnostics

- **fold_paired**: bridge=16.2% of algo (FFI overhead may distort results)
- **fold_quad**: bridge=16.7% of algo (FFI overhead may distort results)
- **fold_sequential**: bridge=16.7% of algo (FFI overhead may distort results)
