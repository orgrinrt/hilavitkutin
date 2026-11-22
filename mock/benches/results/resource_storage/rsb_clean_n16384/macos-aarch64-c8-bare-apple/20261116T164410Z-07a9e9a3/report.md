# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.2% of the fastest

All 6 variants sit between 437 ns and 442 ns - a 1.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v2_decomposed's edge over baseline is significant but tiny (-1 ns, 0.19%)

v2_decomposed differs from baseline v0_blob by -1 ns (0.19%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 436.8 ns median (-0.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 436.8 ns, slowest 442.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under clean morsel loop (algo only) |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 505ns | 503ns | 488ns | 504ns | 529ns | base |
| v1_snapshot | 507ns | 507ns | 487ns | 506ns | 532ns | +0.39% |
| v2_decomposed | 502ns | 499ns | 485ns | 500ns | 526ns | -0.65% |
| v3_shapebound | 500ns | 498ns | 482ns | 498ns | 526ns | -0.99% |
| v4_erased | 502ns | 498ns | 482ns | 501ns | 524ns | -0.73% |
| v5_handletable | 512ns | 509ns | 493ns | 510ns | 535ns | +1.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 439ns | 436ns | 446ns | base | 37.302 |
| v1_snapshot | 440ns | 435ns | 451ns | +0.24% | 37.212 |
| v2_decomposed | 439ns | 435ns | 446ns | -0.15% | 37.360 |
| v3_shapebound | 439ns | 435ns | 448ns | -0.12% | 37.348 |
| v4_erased | 440ns | 436ns | 449ns | +0.10% | 37.267 |
| v5_handletable | 444ns | 440ns | 452ns | +1.06% | 36.912 |

## Performance model

- Peak throughput: **37.703 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 37.423 | 99.3% |
| v1_snapshot | 37.441 | 99.3% |
| v2_decomposed | 37.466 | 99.4% |
| v3_shapebound | 37.509 | 99.5% |
| v4_erased | 37.432 | 99.3% |
| v5_handletable | 37.051 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 505ns | 505ns | base |
| v1_snapshot | 507ns | 507ns | +0.39% |
| v2_decomposed | 502ns | 502ns | -0.65% |
| v3_shapebound | 500ns | 500ns | -0.99% |
| v4_erased | 502ns | 502ns | -0.73% |
| v5_handletable | 512ns | 512ns | +1.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 438ns | base | --- | [438, 438] | --- | --- | --- | --- |
| v1_snapshot | 438ns | no significant difference | [-1, +1]ns | [437, 438] | no | 0.8925 | 0.7140 | 1 |
| v2_decomposed | 437ns | -0.8ns (-0.2%) | [-1, -0]ns | [437, 438] | YES | 0.0095 | 0.0057 | 1 |
| v3_shapebound | 437ns | -1.2ns (-0.3%) | [-2, -0]ns | [436, 437] | YES | 0.0029 | 0.0012 | 2 |
| v4_erased | 438ns | no significant difference | [-1, +1]ns | [437, 438] | no | 0.9267 | 0.9267 | 2 |
| v5_handletable | 442ns | +4.6ns (+1.1%) | [+4, +5]ns | [442, 443] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 436ns | +0.4% | +2.0% | -0.0% | +0.2% | +2.8% |
| 1 | 2 | 438ns | +0.1% | -0.2% | -0.2% | -0.1% | +0.9% |
| 1 | 3 | 438ns | +4.6% | -0.1% | -0.3% | -0.2% | +0.5% |
| 1 | 4 | 437ns | +1.7% | +0.3% | -0.1% | +0.1% | +2.0% |
| 1 | 5 | 441ns | -0.7% | -1.0% | -1.3% | -0.6% | +0.1% |
| 2 | 1 | 438ns | -0.3% | -0.1% | -0.1% | +0.7% | +0.9% |
| 2 | 2 | 438ns | -0.3% | -0.3% | -0.2% | -0.1% | +0.9% |
| 2 | 3 | 438ns | -0.4% | -0.2% | -0.0% | -0.2% | +1.1% |
| 2 | 4 | 440ns | -0.8% | -0.7% | -0.4% | +0.0% | +0.5% |
| 2 | 5 | 445ns | -1.9% | -1.2% | -2.1% | -1.9% | -0.8% |
| 3 | 1 | 438ns | -0.3% | -0.6% | -0.5% | -0.3% | +1.0% |
| 3 | 2 | 437ns | +3.2% | +0.1% | +1.2% | +2.9% | +4.4% |
| 3 | 3 | 438ns | +0.2% | -0.0% | -0.3% | +0.1% | +0.8% |
| 3 | 4 | 438ns | -0.5% | -0.5% | -0.3% | -0.1% | +0.8% |
| 3 | 5 | 438ns | -0.3% | -0.2% | -0.5% | -0.2% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.117 | ok |
| v1_snapshot | 0.122 | ok |
| v2_decomposed | -0.148 | ok |
| v3_shapebound | -0.213 | moderate- |
| v4_erased | -0.120 | ok |
| v5_handletable | -0.151 | ok |

**Consistency summary:**

- **v1_snapshot**: won 9/15, lost 5/15
- **v2_decomposed**: won 10/15, lost 3/15
- **v3_shapebound**: won 12/15, lost 1/15
- **v4_erased**: won 7/15, lost 4/15
- **v5_handletable**: won 1/15, lost 13/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 30421.0ns | 439.2ns | 6926.1% | HIGH |
| v1_snapshot | 30558.5ns | 440.3ns | 6940.6% | HIGH |
| v2_decomposed | 30412.8ns | 438.5ns | 6934.9% | HIGH |
| v3_shapebound | 30400.7ns | 438.7ns | 6929.9% | HIGH |
| v4_erased | 30441.3ns | 439.6ns | 6924.1% | HIGH |
| v5_handletable | 30500.3ns | 443.9ns | 6871.6% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 435.6-446.1 ns)
    435.6 |###############
    436.1 |#################
    436.6 |##############################
    437.1 |###################################
    437.7 |########################################
    438.2 |###################################
    438.7 |#######
    439.2 |############
    439.8 |#######
    440.3 |#####
    440.8 |#####
    441.3 |#######
    441.9 |##
    442.4 |
    442.9 |##
    443.4 |
    444.0 |##
    444.5 |###############
    445.0 |#######
    445.5 |#######
  (11 below, 7 above range)

v1_snapshot (n=120, range 434.8-451.4 ns)
    434.8 |########################
    435.6 |###############################
    436.4 |########################################
    437.3 |###############################
    438.1 |####################
    438.9 |########
    439.8 |########
    440.6 |##
    441.4 |####
    442.2 |####
    443.1 |######
    443.9 |###############
    444.7 |######
    445.6 |####
    446.4 |##
    447.2 |
    448.0 |##
    448.9 |
    449.7 |####
    450.5 |####
  (10 below, 10 above range)

v2_decomposed (n=120, range 435.0-445.5 ns)
    435.0 |####################
    435.5 |######################
    436.1 |########################################
    436.6 |##########################
    437.1 |###############
    437.6 |####################
    438.2 |##########################
    438.7 |###########
    439.2 |########
    439.7 |######
    440.3 |##
    440.8 |
    441.3 |####
    441.9 |##
    442.4 |
    442.9 |####
    443.4 |##
    444.0 |##
    444.5 |######
    445.0 |#############
  (9 below, 5 above range)

v3_shapebound (n=120, range 434.5-448.2 ns)
    434.5 |##############
    435.2 |########################################
    435.9 |##################################
    436.6 |####################################
    437.3 |######################
    438.0 |################
    438.7 |################
    439.3 |####
    440.0 |######
    440.7 |##
    441.4 |
    442.1 |
    442.8 |##
    443.4 |######
    444.1 |######
    444.8 |##
    445.5 |
    446.2 |####
    446.9 |
    447.6 |
  (11 below, 4 above range)

v4_erased (n=120, range 435.5-448.6 ns)
    435.5 |########################
    436.2 |########################################
    436.8 |###############################
    437.5 |###################################
    438.1 |#############
    438.8 |###############
    439.4 |######
    440.1 |######
    440.7 |########
    441.4 |
    442.1 |####
    442.7 |
    443.4 |####
    444.0 |########
    444.7 |
    445.3 |######
    446.0 |####
    446.6 |
    447.3 |####
    447.9 |
  (12 below, 11 above range)

v5_handletable (n=120, range 440.2-451.9 ns)
    440.2 |########################
    440.7 |########################################
    441.3 |####################
    441.9 |##########################
    442.5 |###################
    443.1 |###########
    443.7 |#######
    444.3 |#####
    444.8 |###
    445.4 |#######
    446.0 |#####
    446.6 |#
    447.2 |#
    447.8 |###
    448.4 |#
    448.9 |#
    449.5 |###
    450.1 |
    450.7 |#
    451.3 |#
  (9 below, 10 above range)

```

## Diagnostics

- **v0_blob**: bridge=6930.2% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=6936.6% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=6938.5% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=6946.1% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=6930.8% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6875.5% of algo (FFI overhead may distort results)
