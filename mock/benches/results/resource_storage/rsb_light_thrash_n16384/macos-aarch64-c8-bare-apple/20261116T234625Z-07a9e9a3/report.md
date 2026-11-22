# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (5 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.2% of the fastest

All 6 variants sit between 436 ns and 441 ns - a 1.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v3_shapebound's edge over baseline is significant but tiny (-1 ns, 0.23%)

v3_shapebound differs from baseline v0_blob by -1 ns (0.23%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 435.9 ns median (-0.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 435.9 ns, slowest 441.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under light surrounding work |
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
| v0_blob | 587ns | 583ns | 569ns | 583ns | 616ns | base |
| v1_snapshot | 584ns | 580ns | 567ns | 582ns | 607ns | -0.52% |
| v2_decomposed | 589ns | 583ns | 569ns | 584ns | 624ns | +0.40% |
| v3_shapebound | 585ns | 580ns | 566ns | 580ns | 617ns | -0.33% |
| v4_erased | 585ns | 579ns | 566ns | 581ns | 618ns | -0.24% |
| v5_handletable | 589ns | 586ns | 571ns | 587ns | 610ns | +0.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 438ns | 435ns | 445ns | base | 37.394 |
| v1_snapshot | 438ns | 434ns | 447ns | -0.05% | 37.414 |
| v2_decomposed | 439ns | 434ns | 450ns | +0.11% | 37.352 |
| v3_shapebound | 438ns | 434ns | 450ns | +0.04% | 37.378 |
| v4_erased | 440ns | 435ns | 452ns | +0.32% | 37.273 |
| v5_handletable | 443ns | 439ns | 451ns | +1.06% | 37.000 |

## Performance model

- Peak throughput: **37.754 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 37.509 | 99.4% |
| v1_snapshot | 37.561 | 99.5% |
| v2_decomposed | 37.556 | 99.5% |
| v3_shapebound | 37.587 | 99.6% |
| v4_erased | 37.501 | 99.3% |
| v5_handletable | 37.135 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 587ns | 587ns | base |
| v1_snapshot | 584ns | 584ns | -0.52% |
| v2_decomposed | 589ns | 589ns | +0.40% |
| v3_shapebound | 585ns | 585ns | -0.33% |
| v4_erased | 585ns | 585ns | -0.24% |
| v5_handletable | 589ns | 589ns | +0.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 437ns | base | --- | [437, 437] | --- | --- | --- | --- |
| v1_snapshot | 436ns | no significant difference | [-1, +0]ns | [436, 436] | no | 0.1901 | 0.1141 | 4 |
| v2_decomposed | 436ns | no significant difference | [-1, +0]ns | [436, 437] | no | 0.3071 | 0.2672 | 3 |
| v3_shapebound | 436ns | -1.0ns (-0.2%) | [-1, -0]ns | [436, 436] | YES | 0.0080 | 0.0032 | 1 |
| v4_erased | 437ns | no significant difference | [-0, +1]ns | [436, 438] | no | 0.3071 | 0.3071 | 4 |
| v5_handletable | 441ns | +4.1ns (+0.9%) | [+4, +5]ns | [441, 442] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 436ns | -0.0% | +0.1% | -0.1% | +0.8% | +1.0% |
| 1 | 2 | 437ns | -0.4% | -0.3% | -0.4% | -0.2% | +0.8% |
| 1 | 3 | 437ns | -0.0% | -0.3% | -0.4% | -0.2% | +0.7% |
| 1 | 4 | 437ns | -0.3% | -0.3% | -0.4% | +1.2% | +1.1% |
| 1 | 5 | 436ns | -0.1% | +0.1% | -0.0% | +0.0% | +1.1% |
| 2 | 1 | 436ns | +0.0% | +0.1% | +0.7% | +0.0% | +1.0% |
| 2 | 2 | 444ns | -1.8% | -1.8% | -1.9% | -1.7% | -0.9% |
| 2 | 3 | 436ns | +1.4% | -0.2% | -0.1% | -0.1% | +0.9% |
| 2 | 4 | 437ns | -0.2% | -0.1% | +0.5% | +2.5% | +1.1% |
| 2 | 5 | 437ns | +0.7% | +0.6% | -0.3% | +0.2% | +0.9% |
| 3 | 1 | 437ns | -0.3% | -0.2% | -0.3% | -0.1% | +1.5% |
| 3 | 2 | 436ns | -0.3% | +0.1% | -0.2% | -0.1% | +1.6% |
| 3 | 3 | 438ns | +0.3% | +2.1% | -0.1% | +2.5% | +3.5% |
| 3 | 4 | 436ns | -0.2% | +0.3% | +0.4% | -0.1% | +0.8% |
| 3 | 5 | 437ns | -0.5% | -0.3% | -0.4% | +0.0% | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.183 | ok |
| v1_snapshot | -0.280 | moderate- |
| v2_decomposed | 0.052 | ok |
| v3_shapebound | -0.052 | ok |
| v4_erased | -0.292 | moderate- |
| v5_handletable | 0.034 | ok |

**Consistency summary:**

- **v1_snapshot**: won 8/15, lost 3/15
- **v2_decomposed**: won 8/15, lost 5/15
- **v3_shapebound**: won 9/15, lost 3/15
- **v4_erased**: won 5/15, lost 5/15
- **v5_handletable**: won 1/15, lost 14/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 30309.0ns | 438.1ns | 6917.5% | HIGH |
| v1_snapshot | 30355.5ns | 437.9ns | 6931.9% | HIGH |
| v2_decomposed | 30462.0ns | 438.6ns | 6944.7% | HIGH |
| v3_shapebound | 30426.8ns | 438.3ns | 6941.4% | HIGH |
| v4_erased | 30434.4ns | 439.6ns | 6923.8% | HIGH |
| v5_handletable | 30444.0ns | 442.8ns | 6875.2% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 434.8-445.2 ns)
    434.8 |####################
    435.3 |###################################
    435.8 |###################################
    436.3 |########################################
    436.8 |#####################################
    437.4 |################################
    437.9 |#####################################
    438.4 |############
    438.9 |#####
    439.5 |#######
    440.0 |
    440.5 |
    441.0 |
    441.6 |
    442.1 |
    442.6 |
    443.1 |
    443.7 |
    444.2 |
    444.7 |
  (9 below, 6 above range)

v1_snapshot (n=120, range 434.0-447.0 ns)
    434.0 |##############
    434.6 |##################################
    435.3 |######################################
    435.9 |########################################
    436.6 |######################
    437.2 |##############
    437.9 |##############
    438.5 |
    439.2 |####
    439.8 |
    440.5 |##
    441.1 |######
    441.8 |##
    442.4 |
    443.1 |
    443.7 |####
    444.4 |
    445.0 |
    445.7 |
    446.3 |##
  (12 below, 10 above range)

v2_decomposed (n=120, range 434.2-449.8 ns)
    434.2 |####################################
    435.0 |########################################
    435.8 |##################################
    436.6 |######################################
    437.4 |#######
    438.1 |#########
    438.9 |#####
    439.7 |
    440.5 |
    441.3 |#
    442.0 |###
    442.8 |#
    443.6 |#
    444.4 |#
    445.2 |#
    445.9 |
    446.7 |
    447.5 |
    448.3 |#
    449.1 |###
  (10 below, 10 above range)

v3_shapebound (n=120, range 434.0-449.9 ns)
    434.0 |########################
    434.8 |########################################
    435.6 |####################################
    436.3 |########################
    437.1 |############
    437.9 |####
    438.7 |
    439.5 |#
    440.3 |
    441.1 |#
    441.9 |
    442.7 |###
    443.5 |#
    444.3 |#
    445.1 |
    445.9 |#
    446.7 |
    447.5 |
    448.3 |#
    449.1 |#
  (9 below, 9 above range)

v4_erased (n=120, range 434.6-451.8 ns)
    434.6 |################
    435.5 |########################################
    436.3 |########################
    437.2 |####################
    438.1 |#####################
    438.9 |######
    439.8 |####
    440.6 |#
    441.5 |
    442.3 |###
    443.2 |###
    444.1 |
    444.9 |
    445.8 |#
    446.6 |###
    447.5 |#
    448.3 |#
    449.2 |#
    450.0 |
    450.9 |#
  (11 below, 10 above range)

v5_handletable (n=120, range 438.8-451.4 ns)
    438.8 |############
    439.5 |#####################################
    440.1 |###############################
    440.7 |########################################
    441.3 |##################
    442.0 |#####################
    442.6 |############
    443.2 |################
    443.9 |####
    444.5 |##
    445.1 |####
    445.8 |
    446.4 |
    447.0 |
    447.6 |##
    448.3 |
    448.9 |
    449.5 |####
    450.2 |
    450.8 |##
  (10 below, 10 above range)

```

## Diagnostics

- **v0_blob**: bridge=6922.2% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=6933.7% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=6934.8% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=6940.7% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=6923.5% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6871.9% of algo (FFI overhead may distort results)
