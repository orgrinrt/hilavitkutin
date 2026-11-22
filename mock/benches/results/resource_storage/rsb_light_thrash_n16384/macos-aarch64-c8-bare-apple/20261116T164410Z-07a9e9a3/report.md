# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (8 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.3% of the fastest

All 6 variants sit between 436 ns and 442 ns - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v1_snapshot's edge over baseline is significant but tiny (-1 ns, 0.23%)

v1_snapshot differs from baseline v0_blob by -1 ns (0.23%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 436.2 ns median (-0.3% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 436.2 ns, slowest 441.8 ns)

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
| v0_blob | 585ns | 584ns | 567ns | 583ns | 611ns | base |
| v1_snapshot | 583ns | 580ns | 568ns | 581ns | 603ns | -0.48% |
| v2_decomposed | 586ns | 585ns | 568ns | 584ns | 609ns | +0.11% |
| v3_shapebound | 587ns | 585ns | 567ns | 584ns | 615ns | +0.25% |
| v4_erased | 586ns | 583ns | 569ns | 584ns | 609ns | +0.13% |
| v5_handletable | 588ns | 587ns | 571ns | 588ns | 608ns | +0.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 439ns | 435ns | 447ns | base | 37.326 |
| v1_snapshot | 437ns | 435ns | 440ns | -0.48% | 37.505 |
| v2_decomposed | 438ns | 435ns | 446ns | -0.17% | 37.391 |
| v3_shapebound | 438ns | 434ns | 447ns | -0.23% | 37.414 |
| v4_erased | 438ns | 435ns | 446ns | -0.12% | 37.370 |
| v5_handletable | 443ns | 440ns | 451ns | +0.97% | 36.969 |

## Performance model

- Peak throughput: **37.760 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 37.441 | 99.2% |
| v1_snapshot | 37.535 | 99.4% |
| v2_decomposed | 37.492 | 99.3% |
| v3_shapebound | 37.556 | 99.5% |
| v4_erased | 37.492 | 99.3% |
| v5_handletable | 37.085 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 585ns | 585ns | base |
| v1_snapshot | 583ns | 583ns | -0.48% |
| v2_decomposed | 586ns | 586ns | +0.11% |
| v3_shapebound | 587ns | 587ns | +0.25% |
| v4_erased | 586ns | 586ns | +0.13% |
| v5_handletable | 588ns | 588ns | +0.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 438ns | base | --- | [437, 438] | --- | --- | --- | --- |
| v1_snapshot | 436ns | -1.0ns (-0.2%) | [-2, -1]ns | [436, 437] | YES | 0.0004 | 0.0003 | 4 |
| v2_decomposed | 437ns | no significant difference | [-1, +0]ns | [436, 437] | no | 0.0634 | 0.0507 | 4 |
| v3_shapebound | 436ns | -1.2ns (-0.3%) | [-2, -1]ns | [436, 437] | YES | 0.0001 | 0.0000 | 0 |
| v4_erased | 437ns | no significant difference | [-1, +0]ns | [437, 437] | no | 0.0985 | 0.0985 | 1 |
| v5_handletable | 442ns | +4.4ns (+1.0%) | [+4, +5]ns | [441, 442] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 437ns | -0.0% | +0.0% | +3.1% | +3.3% | +4.9% |
| 1 | 2 | 437ns | +0.1% | +0.0% | -0.4% | -0.2% | +0.9% |
| 1 | 3 | 438ns | -0.4% | -0.2% | -0.0% | -0.2% | +0.6% |
| 1 | 4 | 439ns | -0.6% | -0.4% | -0.4% | -0.4% | +1.0% |
| 1 | 5 | 438ns | -0.3% | -0.3% | -0.4% | -0.1% | +1.2% |
| 2 | 1 | 438ns | -0.4% | -0.1% | -0.1% | -0.2% | +1.3% |
| 2 | 2 | 438ns | +0.0% | -0.3% | -0.7% | -0.4% | +1.0% |
| 2 | 3 | 438ns | -0.2% | +4.7% | -0.1% | -0.2% | +0.9% |
| 2 | 4 | 438ns | -0.1% | -0.1% | -0.4% | +0.0% | +0.8% |
| 2 | 5 | 438ns | -0.3% | -0.3% | -0.7% | -0.4% | +0.7% |
| 3 | 1 | 437ns | -0.1% | -0.2% | -0.3% | +0.3% | +1.0% |
| 3 | 2 | 438ns | -0.3% | +0.1% | -0.0% | -0.1% | +0.6% |
| 3 | 3 | 438ns | -0.3% | -0.4% | -0.3% | -0.1% | +0.9% |
| 3 | 4 | 437ns | -0.1% | -0.2% | -0.5% | -0.2% | +0.9% |
| 3 | 5 | 438ns | -0.4% | -0.3% | -0.6% | -0.4% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.086 | ok |
| v1_snapshot | -0.006 | ok |
| v2_decomposed | -0.104 | ok |
| v3_shapebound | -0.093 | ok |
| v4_erased | -0.059 | ok |
| v5_handletable | -0.048 | ok |

**Consistency summary:**

- **v1_snapshot**: won 11/15, lost 0/15
- **v2_decomposed**: won 11/15, lost 2/15
- **v3_shapebound**: won 12/15, lost 1/15
- **v4_erased**: won 12/15, lost 2/15
- **v5_handletable**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 30375.8ns | 438.9ns | 6920.2% | HIGH |
| v1_snapshot | 30321.8ns | 436.8ns | 6941.1% | HIGH |
| v2_decomposed | 30398.2ns | 438.2ns | 6937.3% | HIGH |
| v3_shapebound | 30393.2ns | 437.9ns | 6940.5% | HIGH |
| v4_erased | 30391.6ns | 438.4ns | 6932.0% | HIGH |
| v5_handletable | 30471.2ns | 443.2ns | 6875.6% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 435.3-446.9 ns)
    435.3 |################################
    435.9 |################################
    436.4 |###################################
    437.0 |##############################
    437.6 |########################################
    438.2 |###################################
    438.8 |######################
    439.4 |############
    439.9 |###############
    440.5 |##
    441.1 |
    441.7 |##
    442.3 |
    442.8 |##
    443.4 |
    444.0 |
    444.6 |
    445.2 |
    445.7 |
    446.3 |
  (8 below, 7 above range)

v1_snapshot (n=120, range 434.5-440.0 ns)
    434.5 |##
    434.8 |##################
    435.1 |##########
    435.3 |##
    435.6 |########################################
    435.9 |################
    436.2 |################################
    436.4 |#############################
    436.7 |#############
    437.0 |#####################
    437.3 |#############################
    437.5 |#############################
    437.8 |##
    438.1 |##########
    438.4 |##
    438.6 |##
    438.9 |##
    439.2 |##
    439.5 |#####
    439.7 |##
  (11 below, 5 above range)

v2_decomposed (n=120, range 434.7-445.5 ns)
    434.7 |#####################
    435.2 |############################
    435.8 |########################################
    436.3 |############################
    436.8 |################################
    437.4 |#####################
    437.9 |################################
    438.5 |##############
    439.0 |###########
    439.5 |##
    440.1 |####
    440.6 |
    441.2 |##
    441.7 |##
    442.3 |
    442.8 |
    443.3 |
    443.9 |
    444.4 |
    445.0 |
  (9 below, 8 above range)

v3_shapebound (n=120, range 433.9-446.7 ns)
    433.9 |##################
    434.5 |###############
    435.2 |########################################
    435.8 |#############
    436.5 |############################
    437.1 |####################
    437.8 |##################
    438.4 |########
    439.0 |###
    439.7 |
    440.3 |#
    441.0 |
    441.6 |
    442.2 |
    442.9 |#
    443.5 |
    444.2 |
    444.8 |
    445.5 |
    446.1 |
  (10 below, 9 above range)

v4_erased (n=120, range 434.8-446.1 ns)
    434.8 |#################
    435.4 |##############################
    435.9 |########################################
    436.5 |########################################
    437.1 |################################
    437.6 |############
    438.2 |#########################
    438.8 |#####
    439.3 |######################
    439.9 |##
    440.4 |#####
    441.0 |#####
    441.6 |
    442.1 |
    442.7 |
    443.3 |
    443.8 |
    444.4 |
    444.9 |
    445.5 |
  (13 below, 12 above range)

v5_handletable (n=120, range 439.6-450.8 ns)
    439.6 |#################
    440.2 |################################
    440.7 |####################
    441.3 |########################################
    441.8 |#############
    442.4 |##############################
    443.0 |###################
    443.5 |###########
    444.1 |#
    444.7 |
    445.2 |#
    445.8 |
    446.3 |#
    446.9 |
    447.5 |
    448.0 |
    448.6 |#
    449.1 |
    449.7 |
    450.3 |
  (8 below, 11 above range)

```

## Diagnostics

- **v0_blob**: bridge=6928.8% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=6946.6% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=6940.4% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=6952.4% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=6936.7% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6881.7% of algo (FFI overhead may distort results)
