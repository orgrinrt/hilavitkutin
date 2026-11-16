# Min/max: u64::min intrinsic vs if/else branch vs branchless bit trick

3 variants, 160 samples per variant.
Baseline: **minmax_branch**

## Highlights

Baseline for all deltas below: **minmax_branch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (8 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (minmax_branch)

The baseline minmax_branch is the fastest (378 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.2% of the fastest

All 3 variants sit between 378 ns and 383 ns - a 1.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### minmax_branchless's edge over baseline is significant but tiny (5 ns, 1.22%)

minmax_branchless differs from baseline minmax_branch by 5 ns (1.22%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (minmax_branch) is the fastest** at 378.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 378.3 ns, slowest 382.9 ns)

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
| minmax_branch | 2456ns | 2450ns | 2444ns | 2451ns | 2481ns | base |
| minmax_branchless | 2457ns | 2454ns | 2447ns | 2454ns | 2476ns | +0.05% |
| minmax_intrinsic | 2455ns | 2451ns | 2445ns | 2451ns | 2480ns | -0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| minmax_branch | 379ns | 373ns | 387ns | base | 43.199 |
| minmax_branchless | 383ns | 378ns | 388ns | +1.00% | 42.773 |
| minmax_intrinsic | 379ns | 374ns | 385ns | -0.01% | 43.204 |

## Performance model

- Peak throughput: **43.900 Gops/s** (minmax_branch; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| minmax_branch | 43.310 | 98.7% |
| minmax_branchless | 42.789 | 97.5% |
| minmax_intrinsic | 43.252 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| minmax_branch | 2456ns | 2456ns | base |
| minmax_branchless | 2457ns | 2457ns | +0.05% |
| minmax_intrinsic | 2455ns | 2455ns | -0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| minmax_branch | 378ns | base | --- | [378, 379] | --- | --- | --- | --- |
| minmax_branchless | 383ns | +4.6ns (+1.2%) | [+3, +5]ns | [382, 384] | YES | 0.0000 | 0.0000 | 6 |
| minmax_intrinsic | 379ns | no significant difference | [-0, +1]ns | [378, 380] | no | 0.2638 | 0.2638 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | minmax_branch | minmax_branchless | minmax_intrinsic |
|---|---|---|---|---|
| 1 | 1 | 378ns | +1.4% | +0.2% |
| 1 | 2 | 378ns | +1.3% | +0.7% |
| 1 | 3 | 381ns | +0.6% | -0.7% |
| 1 | 4 | 378ns | +1.5% | +0.4% |
| 1 | 5 | 378ns | +1.6% | -0.3% |
| 1 | 6 | 378ns | +1.3% | +0.4% |
| 1 | 7 | 378ns | +0.7% | +0.1% |
| 1 | 8 | 380ns | +0.8% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| minmax_branch | -0.174 | ok |
| minmax_branchless | 0.212 | moderate+ |
| minmax_intrinsic | -0.367 | moderate- |

**Consistency summary:**

- **minmax_branchless**: won 0/8, lost 8/8
- **minmax_intrinsic**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| minmax_branch | 2.8ns | 379.3ns | 0.7% |  |
| minmax_branchless | 2.8ns | 383.0ns | 0.7% |  |
| minmax_intrinsic | 2.7ns | 379.2ns | 0.7% |  |

## Distribution (algo ns)

```
minmax_branch (n=160, range 373.2-387.4 ns)
    373.2 |##############
    373.9 |####
    374.6 |####
    375.4 |##########################
    376.1 |####################
    376.8 |################
    377.5 |########################################
    378.2 |########################
    378.9 |########################
    379.6 |########
    380.3 |##########################
    381.0 |################
    381.8 |########
    382.5 |##########
    383.2 |########
    383.9 |############
    384.6 |##
    385.3 |############
    386.0 |######
    386.7 |
  (16 below, 4 above range)

minmax_branchless (n=160, range 378.2-388.2 ns)
    378.2 |#####################
    378.7 |###########################
    379.2 |#########
    379.7 |##################
    380.2 |############
    380.7 |###########################
    381.2 |###########################
    381.7 |#################################
    382.2 |########################
    382.7 |#####################
    383.2 |###########################
    383.7 |####################################
    384.2 |##################
    384.7 |###############
    385.2 |########################
    385.7 |########################################
    386.2 |###############
    386.7 |#########
    387.2 |######
    387.7 |######
  (10 below, 12 above range)

minmax_intrinsic (n=160, range 374.0-385.1 ns)
    374.0 |###########
    374.6 |####################
    375.1 |##############
    375.7 |###############################
    376.2 |####################
    376.8 |######################
    377.3 |########################################
    377.9 |######################
    378.5 |##############
    379.0 |#########################
    379.6 |#########################
    380.1 |####################
    380.7 |#################
    381.2 |###########
    381.8 |##############
    382.3 |#####
    382.9 |###############################
    383.5 |#####
    384.0 |##############
    384.6 |##############
  (13 below, 13 above range)

```
