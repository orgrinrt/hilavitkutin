# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (379 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.1% of the fastest

All 3 variants sit between 379 ns and 383 ns - a 1.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### minmax_branchless's edge over baseline is significant but tiny (5 ns, 1.22%)

minmax_branchless differs from baseline minmax_branch by 5 ns (1.22%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 378.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 378.6 ns, slowest 382.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5d49a3f1b7c06e82 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| minmax_branch | 2458ns | 2450ns | 2443ns | 2450ns | 2497ns | base |
| minmax_branchless | 2459ns | 2455ns | 2447ns | 2455ns | 2485ns | +0.04% |
| minmax_intrinsic | 2466ns | 2451ns | 2444ns | 2452ns | 2530ns | +0.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 379ns | 373ns | 388ns | base | 43.202 |
| minmax_branchless | 384ns | 378ns | 391ns | +1.16% | 42.705 |
| minmax_intrinsic | 380ns | 374ns | 392ns | +0.33% | 43.059 |

## Performance model

- Peak throughput: **43.894 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 43.281 | 98.6% |
| minmax_branchless | 42.789 | 97.5% |
| minmax_intrinsic | 43.230 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2458ns | 2458ns | base |
| minmax_branchless | 2459ns | 2459ns | +0.04% |
| minmax_intrinsic | 2466ns | 2466ns | +0.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 379ns | base | --- | [378, 379] | --- | --- | --- | --- |
| minmax_branchless | 383ns | +4.6ns (+1.2%) | [+4, +5]ns | [382, 384] | YES | 0.0000 | 0.0000 | 2 |
| minmax_intrinsic | 379ns | no significant difference | [+0, +1]ns | [378, 380] | no | 0.0788 | 0.0788 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 377ns | +1.5% | +0.3% |
| 1 | 2 | 379ns | +1.0% | +0.3% |
| 1 | 3 | 376ns | +2.1% | +0.8% |
| 1 | 4 | 378ns | +1.6% | +0.0% |
| 1 | 5 | 379ns | +1.1% | -0.2% |
| 1 | 6 | 378ns | +1.4% | +2.6% |
| 1 | 7 | 380ns | +0.5% | -0.8% |
| 1 | 8 | 379ns | +0.9% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.173 | ok |
| minmax_branchless | 0.482 | moderate+ |
| minmax_intrinsic | -0.283 | moderate- |

**Consistency summary:**

- **minmax_branchless**: won 0/8, lost 8/8
- **minmax_intrinsic**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.7ns | 379.2ns | 0.7% |  |
| minmax_branchless | 2.8ns | 383.7ns | 0.7% |  |
| minmax_intrinsic | 2.8ns | 380.5ns | 0.7% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 373.3-387.6 ns)
    373.3 |####################
    374.0 |####################
    374.7 |#########################
    375.4 |#####
    376.1 |###################################
    376.9 |#####################################
    377.6 |#######
    378.3 |########################################
    379.0 |###########################
    379.7 |###########################
    380.4 |##########
    381.2 |######################
    381.9 |#################
    382.6 |############
    383.3 |##
    384.0 |####################
    384.8 |###############
    385.5 |#####
    386.2 |##
    386.9 |##
  (14 below, 4 above range)

minmax_branchless (n=160, range 378.3-390.9 ns)
    378.3 |###############
    378.9 |############
    379.6 |####################
    380.2 |################################
    380.8 |###############
    381.5 |####################
    382.1 |########################################
    382.7 |#####################################
    383.4 |######################
    384.0 |#####################################
    384.6 |############
    385.3 |######################
    385.9 |#######
    386.5 |####################
    387.1 |#################
    387.8 |############
    388.4 |#####
    389.0 |
    389.7 |
    390.3 |#######
  (13 below, 4 above range)

minmax_intrinsic (n=160, range 373.6-392.0 ns)
    373.6 |####################
    374.5 |##############################
    375.4 |#####################################
    376.4 |#########################
    377.3 |########################################
    378.2 |###############
    379.1 |#####################################
    380.0 |##############################
    381.0 |#################
    381.9 |###########################
    382.8 |############
    383.7 |######################
    384.6 |#######
    385.6 |#######
    386.5 |#####
    387.4 |##
    388.3 |
    389.2 |#####
    390.2 |#####
    391.1 |##
  (13 below, 7 above range)

```
