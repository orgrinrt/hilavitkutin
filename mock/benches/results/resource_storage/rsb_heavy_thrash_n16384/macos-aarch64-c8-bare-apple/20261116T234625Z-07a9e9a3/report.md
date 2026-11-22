# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.4% of the fastest

All 6 variants sit between 435 ns and 441 ns - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v3_shapebound's edge over baseline is significant but tiny (-2 ns, 0.46%)

v3_shapebound differs from baseline v0_blob by -2 ns (0.46%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 434.8 ns median (-0.5% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 434.8 ns, slowest 440.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under L1-evicting heavy-memory mix-in |
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
| v0_blob | 5157ns | 5141ns | 5125ns | 5143ns | 5231ns | base |
| v1_snapshot | 5184ns | 5146ns | 5124ns | 5151ns | 5343ns | +0.53% |
| v2_decomposed | 5177ns | 5146ns | 5124ns | 5149ns | 5313ns | +0.39% |
| v3_shapebound | 5148ns | 5140ns | 5123ns | 5141ns | 5197ns | -0.16% |
| v4_erased | 5162ns | 5143ns | 5121ns | 5143ns | 5260ns | +0.10% |
| v5_handletable | 5163ns | 5144ns | 5126ns | 5144ns | 5256ns | +0.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 439ns | 435ns | 451ns | base | 37.310 |
| v1_snapshot | 440ns | 434ns | 458ns | +0.16% | 37.252 |
| v2_decomposed | 438ns | 434ns | 447ns | -0.29% | 37.419 |
| v3_shapebound | 436ns | 433ns | 441ns | -0.81% | 37.614 |
| v4_erased | 438ns | 434ns | 445ns | -0.33% | 37.435 |
| v5_handletable | 442ns | 439ns | 450ns | +0.71% | 37.049 |

## Performance model

- Peak throughput: **37.856 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 37.509 | 99.1% |
| v1_snapshot | 37.595 | 99.3% |
| v2_decomposed | 37.595 | 99.3% |
| v3_shapebound | 37.682 | 99.5% |
| v4_erased | 37.561 | 99.2% |
| v5_handletable | 37.169 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 5157ns | 5157ns | base |
| v1_snapshot | 5184ns | 5184ns | +0.53% |
| v2_decomposed | 5177ns | 5177ns | +0.39% |
| v3_shapebound | 5148ns | 5148ns | -0.16% |
| v4_erased | 5162ns | 5162ns | +0.10% |
| v5_handletable | 5163ns | 5163ns | +0.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 437ns | base | --- | [436, 437] | --- | --- | --- | --- |
| v1_snapshot | 436ns | no significant difference | [-1, +0]ns | [435, 436] | no | 0.3071 | 0.3071 | 4 |
| v2_decomposed | 436ns | no significant difference | [-1, +0]ns | [436, 436] | no | 0.1734 | 0.1388 | 3 |
| v3_shapebound | 435ns | -2.0ns (-0.5%) | [-3, -2]ns | [434, 435] | YES | 0.0000 | 0.0000 | 4 |
| v4_erased | 436ns | -0.7ns (-0.2%) | [-1, -0]ns | [436, 437] | YES | 0.0280 | 0.0168 | 1 |
| v5_handletable | 441ns | +4.2ns (+1.0%) | [+3, +5]ns | [440, 441] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 437ns | -0.4% | -0.6% | -0.5% | +0.3% | +1.0% |
| 1 | 2 | 437ns | -0.3% | +0.8% | -0.2% | -0.3% | +1.1% |
| 1 | 3 | 435ns | +0.1% | +0.0% | -0.1% | +0.3% | +1.1% |
| 1 | 4 | 437ns | -0.4% | +0.0% | -0.6% | -0.5% | +0.9% |
| 1 | 5 | 436ns | -0.3% | +0.7% | -0.3% | -0.3% | +1.2% |
| 2 | 1 | 437ns | +2.5% | -0.3% | -0.0% | +0.5% | +0.8% |
| 2 | 2 | 440ns | +1.6% | -1.0% | -1.1% | -0.8% | +0.4% |
| 2 | 3 | 438ns | -0.7% | -0.5% | -0.9% | -0.4% | +0.5% |
| 2 | 4 | 437ns | -0.4% | -0.2% | -0.5% | -0.2% | +0.4% |
| 2 | 5 | 437ns | -0.3% | -0.1% | -0.4% | -0.3% | +1.0% |
| 3 | 1 | 437ns | -0.3% | -0.1% | -0.7% | -0.2% | +1.5% |
| 3 | 2 | 436ns | +0.8% | -0.2% | -0.3% | +0.1% | +6.4% |
| 3 | 3 | 437ns | -0.3% | +0.6% | -0.8% | -0.1% | +0.9% |
| 3 | 4 | 436ns | -0.2% | -0.3% | -0.4% | -0.3% | +0.9% |
| 3 | 5 | 435ns | -0.1% | +0.2% | -0.1% | +2.2% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.156 | ok |
| v1_snapshot | 0.320 | moderate+ |
| v2_decomposed | -0.429 | moderate- |
| v3_shapebound | 0.149 | ok |
| v4_erased | -0.130 | ok |
| v5_handletable | 0.046 | ok |

**Consistency summary:**

- **v1_snapshot**: won 11/15, lost 4/15
- **v2_decomposed**: won 9/15, lost 4/15
- **v3_shapebound**: won 13/15, lost 0/15
- **v4_erased**: won 10/15, lost 4/15
- **v5_handletable**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 30352.0ns | 439.1ns | 6911.9% | HIGH |
| v1_snapshot | 30466.4ns | 439.8ns | 6927.0% | HIGH |
| v2_decomposed | 30437.3ns | 437.8ns | 6951.6% | HIGH |
| v3_shapebound | 30295.5ns | 435.6ns | 6955.1% | HIGH |
| v4_erased | 30378.5ns | 437.7ns | 6941.0% | HIGH |
| v5_handletable | 30434.0ns | 442.2ns | 6881.9% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 434.5-450.6 ns)
    434.5 |####################
    435.3 |#######################
    436.1 |########################################
    436.9 |########################################
    437.7 |#######################
    438.5 |##########
    439.3 |#####
    440.1 |#
    440.9 |
    441.7 |
    442.6 |#
    443.4 |
    444.2 |
    445.0 |
    445.8 |
    446.6 |
    447.4 |
    448.2 |
    449.0 |
    449.8 |
  (13 below, 8 above range)

v1_snapshot (n=120, range 433.8-457.5 ns)
    433.8 |########################################
    435.0 |######################################
    436.2 |#######################
    437.4 |######
    438.6 |
    439.8 |###
    440.9 |###
    442.1 |##
    443.3 |
    444.5 |
    445.7 |
    446.9 |
    448.1 |
    449.2 |##
    450.4 |
    451.6 |#
    452.8 |
    454.0 |#
    455.2 |#
    456.3 |#
  (9 below, 10 above range)

v2_decomposed (n=120, range 434.0-447.2 ns)
    434.0 |##########################
    434.7 |########################################
    435.3 |#######################
    436.0 |############################
    436.6 |##################
    437.3 |#####
    437.9 |########
    438.6 |######
    439.3 |#
    439.9 |#
    440.6 |#####
    441.2 |#
    441.9 |#
    442.6 |#
    443.2 |#
    443.9 |###
    444.5 |
    445.2 |
    445.9 |#
    446.5 |
  (7 below, 7 above range)

v3_shapebound (n=120, range 432.8-440.9 ns)
    432.8 |######################
    433.2 |#########################
    433.6 |###################################
    434.0 |###############
    434.4 |########################################
    434.8 |###################################
    435.2 |##############################
    435.6 |#####################################
    436.0 |#######
    436.5 |#######
    436.9 |##########
    437.3 |#####
    437.7 |##
    438.1 |##
    438.5 |
    438.9 |
    439.3 |
    439.7 |
    440.1 |
    440.5 |
  (8 below, 2 above range)

v4_erased (n=120, range 434.2-445.5 ns)
    434.2 |######################
    434.8 |########################################
    435.4 |#################
    435.9 |##############################
    436.5 |############################
    437.1 |#######
    437.6 |##########################
    438.2 |#####
    438.7 |
    439.3 |#
    439.9 |#
    440.4 |#####
    441.0 |
    441.5 |
    442.1 |
    442.7 |
    443.2 |###
    443.8 |
    444.4 |
    444.9 |
  (11 below, 8 above range)

v5_handletable (n=120, range 438.6-449.9 ns)
    438.6 |########################
    439.1 |#################
    439.7 |###################
    440.3 |########################################
    440.8 |######################
    441.4 |###################
    442.0 |######################
    442.6 |#######
    443.1 |#######
    443.7 |#########
    444.3 |#
    444.8 |#
    445.4 |
    446.0 |
    446.5 |
    447.1 |
    447.7 |
    448.2 |
    448.8 |
    449.4 |
  (10 below, 8 above range)

```

## Diagnostics

- **v0_blob**: bridge=6928.9% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=6946.7% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=6946.9% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=6960.1% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=6938.0% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6881.2% of algo (FFI overhead may distort results)
