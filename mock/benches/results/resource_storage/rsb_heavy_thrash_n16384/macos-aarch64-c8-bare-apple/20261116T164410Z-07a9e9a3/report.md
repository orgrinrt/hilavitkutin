# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 1.2% of the fastest

All 6 variants sit between 436 ns and 441 ns - a 1.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v1_snapshot's edge over baseline is significant but tiny (-1 ns, 0.27%)

v1_snapshot differs from baseline v0_blob by -1 ns (0.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 435.9 ns median (-0.4% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 435.9 ns, slowest 441.3 ns)

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
| v0_blob | 5161ns | 5150ns | 5129ns | 5151ns | 5222ns | base |
| v1_snapshot | 5174ns | 5151ns | 5130ns | 5153ns | 5279ns | +0.25% |
| v2_decomposed | 5152ns | 5150ns | 5127ns | 5150ns | 5185ns | -0.16% |
| v3_shapebound | 5149ns | 5147ns | 5128ns | 5148ns | 5176ns | -0.22% |
| v4_erased | 5155ns | 5152ns | 5131ns | 5153ns | 5185ns | -0.12% |
| v5_handletable | 5168ns | 5156ns | 5135ns | 5159ns | 5229ns | +0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 438ns | 435ns | 443ns | base | 37.398 |
| v1_snapshot | 440ns | 434ns | 453ns | +0.33% | 37.274 |
| v2_decomposed | 437ns | 435ns | 440ns | -0.29% | 37.507 |
| v3_shapebound | 436ns | 434ns | 440ns | -0.41% | 37.551 |
| v4_erased | 437ns | 435ns | 440ns | -0.21% | 37.475 |
| v5_handletable | 443ns | 439ns | 450ns | +1.07% | 37.003 |

## Performance model

- Peak throughput: **37.783 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 37.449 | 99.1% |
| v1_snapshot | 37.535 | 99.3% |
| v2_decomposed | 37.535 | 99.3% |
| v3_shapebound | 37.587 | 99.5% |
| v4_erased | 37.492 | 99.2% |
| v5_handletable | 37.127 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 5161ns | 5161ns | base |
| v1_snapshot | 5174ns | 5174ns | +0.25% |
| v2_decomposed | 5152ns | 5152ns | -0.16% |
| v3_shapebound | 5149ns | 5149ns | -0.22% |
| v4_erased | 5155ns | 5155ns | -0.12% |
| v5_handletable | 5168ns | 5168ns | +0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 438ns | base | --- | [437, 438] | --- | --- | --- | --- |
| v1_snapshot | 436ns | -1.2ns (-0.3%) | [-2, -0]ns | [436, 437] | YES | 0.0010 | 0.0008 | 3 |
| v2_decomposed | 436ns | -1.0ns (-0.2%) | [-2, -0]ns | [436, 437] | YES | 0.0005 | 0.0003 | 2 |
| v3_shapebound | 436ns | -1.4ns (-0.3%) | [-2, -1]ns | [436, 436] | YES | 0.0000 | 0.0000 | 1 |
| v4_erased | 437ns | no significant difference | [-1, +0]ns | [437, 438] | no | 0.1172 | 0.1172 | 2 |
| v5_handletable | 441ns | +4.1ns (+0.9%) | [+3, +5]ns | [441, 442] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 438ns | -0.3% | -0.3% | -0.3% | +0.1% | +1.0% |
| 1 | 2 | 438ns | -0.3% | -0.5% | -0.5% | -0.3% | +1.1% |
| 1 | 3 | 437ns | -0.1% | -0.3% | -0.5% | -0.2% | +0.9% |
| 1 | 4 | 436ns | +0.0% | +0.0% | -0.2% | +0.3% | +1.1% |
| 1 | 5 | 438ns | -0.3% | -0.3% | -0.7% | -0.2% | +0.7% |
| 2 | 1 | 438ns | -0.3% | -0.4% | -0.4% | +0.1% | +0.6% |
| 2 | 2 | 450ns | -2.9% | -2.8% | -3.3% | -3.0% | -0.9% |
| 2 | 3 | 438ns | +1.5% | -0.3% | -0.3% | +0.0% | +0.8% |
| 2 | 4 | 437ns | -0.1% | +0.2% | +0.0% | +0.5% | +1.3% |
| 2 | 5 | 437ns | -0.1% | +0.0% | +0.0% | -0.1% | +0.9% |
| 3 | 1 | 437ns | -0.4% | -0.4% | -0.4% | -0.1% | +0.9% |
| 3 | 2 | 437ns | +0.1% | -0.1% | -0.0% | +0.1% | +1.4% |
| 3 | 3 | 438ns | -0.4% | -0.2% | -0.5% | -0.3% | +0.7% |
| 3 | 4 | 437ns | -0.2% | +0.0% | -0.3% | -0.0% | +0.7% |
| 3 | 5 | 437ns | -0.1% | +0.1% | +0.1% | -0.2% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.007 | ok |
| v1_snapshot | -0.074 | ok |
| v2_decomposed | 0.035 | ok |
| v3_shapebound | -0.095 | ok |
| v4_erased | -0.243 | moderate- |
| v5_handletable | -0.260 | moderate- |

**Consistency summary:**

- **v1_snapshot**: won 10/15, lost 1/15
- **v2_decomposed**: won 10/15, lost 2/15
- **v3_shapebound**: won 11/15, lost 1/15
- **v4_erased**: won 7/15, lost 4/15
- **v5_handletable**: won 1/15, lost 14/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 30392.8ns | 438.1ns | 6937.4% | HIGH |
| v1_snapshot | 30425.7ns | 439.6ns | 6922.0% | HIGH |
| v2_decomposed | 30344.4ns | 436.8ns | 6946.6% | HIGH |
| v3_shapebound | 30341.7ns | 436.3ns | 6954.2% | HIGH |
| v4_erased | 30331.6ns | 437.2ns | 6937.8% | HIGH |
| v5_handletable | 30453.4ns | 442.8ns | 6877.8% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 435.1-443.1 ns)
    435.1 |##########
    435.5 |##########################
    435.9 |##################
    436.3 |##################
    436.7 |##########################
    437.1 |########################
    437.5 |########################################
    437.9 |################
    438.3 |################
    438.7 |#############################
    439.1 |#############
    439.5 |#############
    439.9 |#####
    440.3 |
    440.7 |#####
    441.1 |
    441.5 |
    441.9 |
    442.3 |
    442.7 |
  (12 below, 9 above range)

v1_snapshot (n=120, range 434.5-453.5 ns)
    434.5 |#################
    435.4 |#################################
    436.4 |########################################
    437.3 |##############
    438.3 |########
    439.2 |#
    440.2 |
    441.1 |
    442.1 |
    443.0 |
    444.0 |
    444.9 |
    445.9 |
    446.8 |
    447.8 |
    448.7 |
    449.7 |##
    450.6 |#
    451.6 |
    452.5 |
  (10 below, 7 above range)

v2_decomposed (n=120, range 434.6-439.8 ns)
    434.6 |#############
    434.9 |########
    435.1 |#####################
    435.4 |################
    435.7 |#####################
    435.9 |#############
    436.2 |########################################
    436.4 |#############
    436.7 |########################
    437.0 |#############################
    437.2 |########
    437.5 |#####################
    437.7 |##################
    438.0 |##########
    438.3 |########
    438.5 |##
    438.8 |#####
    439.0 |##########
    439.3 |
    439.6 |
  (8 below, 5 above range)

v3_shapebound (n=120, range 433.6-440.2 ns)
    433.6 |####################
    434.0 |####################
    434.3 |#############
    434.6 |##########################
    434.9 |##########################
    435.3 |####################################
    435.6 |##########################
    435.9 |#######################
    436.2 |##############################
    436.6 |########################################
    436.9 |#######################
    437.2 |#######################
    437.5 |#######################
    437.9 |###
    438.2 |######
    438.5 |################
    438.9 |###
    439.2 |
    439.5 |
    439.8 |
  (9 below, 2 above range)

v4_erased (n=120, range 434.7-440.3 ns)
    434.7 |
    435.0 |#############
    435.3 |##########################
    435.5 |##########################
    435.8 |####################
    436.1 |######
    436.4 |####################
    436.7 |########################################
    436.9 |#################################
    437.2 |####################
    437.5 |#######################
    437.8 |########################################
    438.1 |####################
    438.3 |######
    438.6 |######
    438.9 |################
    439.2 |##########
    439.5 |##########
    439.7 |
    440.0 |
  (12 below, 6 above range)

v5_handletable (n=120, range 439.5-449.6 ns)
    439.5 |#############
    440.0 |##########
    440.5 |########################################
    441.0 |########################
    441.5 |##################
    442.0 |#######
    442.5 |#########
    443.0 |#######
    443.5 |############
    444.0 |#######
    444.5 |###
    445.0 |###
    445.5 |
    446.0 |###
    446.6 |
    447.1 |
    447.6 |
    448.1 |
    448.6 |
    449.1 |#
  (8 below, 6 above range)

```

## Diagnostics

- **v0_blob**: bridge=6935.3% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=6951.2% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=6953.3% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=6960.0% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=6939.7% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6893.1% of algo (FFI overhead may distort results)
