# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### v3_shapebound shows warm-up / thermal drift (autocorr +0.64)

v3_shapebound's per-pass series has lag-1 autocorrelation +0.64, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6 ns) is smaller than the fastest variant's own run-to-run std-dev (9 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.3% of the fastest

All 6 variants sit between 436 ns and 441 ns - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v1_snapshot's edge over baseline is significant but tiny (-1 ns, 0.23%)

v1_snapshot differs from baseline v0_blob by -1 ns (0.23%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 435.6 ns median (-0.3% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 435.6 ns, slowest 441.2 ns)

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
| v0_blob | 499ns | 497ns | 482ns | 497ns | 523ns | base |
| v1_snapshot | 503ns | 498ns | 481ns | 499ns | 537ns | +0.77% |
| v2_decomposed | 508ns | 501ns | 484ns | 501ns | 553ns | +1.83% |
| v3_shapebound | 503ns | 500ns | 485ns | 500ns | 531ns | +0.90% |
| v4_erased | 507ns | 500ns | 484ns | 500ns | 555ns | +1.70% |
| v5_handletable | 510ns | 504ns | 489ns | 506ns | 545ns | +2.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 439ns | 435ns | 450ns | base | 37.307 |
| v1_snapshot | 440ns | 434ns | 456ns | +0.12% | 37.263 |
| v2_decomposed | 442ns | 434ns | 465ns | +0.60% | 37.085 |
| v3_shapebound | 439ns | 433ns | 454ns | -0.00% | 37.307 |
| v4_erased | 440ns | 435ns | 456ns | +0.27% | 37.206 |
| v5_handletable | 443ns | 439ns | 454ns | +0.96% | 36.952 |

## Performance model

- Peak throughput: **37.800 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 37.501 | 99.2% |
| v1_snapshot | 37.561 | 99.4% |
| v2_decomposed | 37.552 | 99.3% |
| v3_shapebound | 37.612 | 99.5% |
| v4_erased | 37.509 | 99.2% |
| v5_handletable | 37.135 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 499ns | 499ns | base |
| v1_snapshot | 503ns | 503ns | +0.77% |
| v2_decomposed | 508ns | 508ns | +1.83% |
| v3_shapebound | 503ns | 503ns | +0.90% |
| v4_erased | 507ns | 507ns | +1.70% |
| v5_handletable | 510ns | 510ns | +2.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 437ns | base | --- | [437, 437] | --- | --- | --- | --- |
| v1_snapshot | 436ns | -1.0ns (-0.2%) | [-2, -0]ns | [436, 436] | YES | 0.0063 | 0.0032 | 1 |
| v2_decomposed | 436ns | no significant difference | [-1, +0]ns | [436, 437] | no | 0.1426 | 0.1141 | 4 |
| v3_shapebound | 436ns | -1.0ns (-0.2%) | [-2, -0]ns | [435, 436] | YES | 0.0063 | 0.0038 | 4 |
| v4_erased | 437ns | no significant difference | [-0, +1]ns | [436, 438] | no | 0.5793 | 0.5793 | 3 |
| v5_handletable | 441ns | +4.2ns (+1.0%) | [+4, +5]ns | [441, 442] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 436ns | -0.0% | -0.1% | -0.2% | -0.1% | +2.2% |
| 1 | 2 | 436ns | +0.1% | -0.0% | +0.0% | +0.3% | +1.3% |
| 1 | 3 | 437ns | -0.3% | +1.4% | +2.4% | +0.0% | +0.8% |
| 1 | 4 | 449ns | +0.3% | -0.7% | +0.6% | -2.5% | -1.3% |
| 1 | 5 | 438ns | -0.5% | -0.3% | +1.3% | +0.8% | +0.8% |
| 2 | 1 | 438ns | +0.0% | +0.7% | +0.3% | +1.2% | +0.8% |
| 2 | 2 | 438ns | +3.9% | -0.4% | -0.6% | -0.1% | +0.8% |
| 2 | 3 | 437ns | -0.2% | -0.2% | -0.5% | -0.2% | +0.9% |
| 2 | 4 | 437ns | -0.3% | -0.1% | -0.3% | -0.1% | +1.1% |
| 2 | 5 | 436ns | +0.0% | -0.1% | -0.2% | +1.3% | +0.8% |
| 3 | 1 | 441ns | -1.0% | -1.0% | -1.3% | -0.9% | +0.2% |
| 3 | 2 | 436ns | -0.0% | -0.1% | -0.0% | +0.1% | +1.5% |
| 3 | 3 | 438ns | -0.6% | +0.1% | -0.7% | +0.1% | +0.7% |
| 3 | 4 | 436ns | +0.1% | +0.1% | -0.3% | +0.4% | +1.0% |
| 3 | 5 | 436ns | -0.5% | +0.9% | -0.2% | -0.0% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.049 | ok |
| v1_snapshot | -0.107 | ok |
| v2_decomposed | 0.187 | ok |
| v3_shapebound | 0.642 | HIGH+ (drift/warm-up) |
| v4_erased | 0.152 | ok |
| v5_handletable | -0.090 | ok |

**Consistency summary:**

- **v1_snapshot**: won 7/15, lost 4/15
- **v2_decomposed**: won 6/15, lost 4/15
- **v3_shapebound**: won 9/15, lost 4/15
- **v4_erased**: won 4/15, lost 7/15
- **v5_handletable**: won 1/15, lost 14/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 30404.6ns | 439.2ns | 6923.2% | HIGH |
| v1_snapshot | 30484.4ns | 439.7ns | 6933.2% | HIGH |
| v2_decomposed | 30524.4ns | 441.8ns | 6909.1% | HIGH |
| v3_shapebound | 30510.4ns | 439.2ns | 6947.3% | HIGH |
| v4_erased | 30428.4ns | 440.4ns | 6910.0% | HIGH |
| v5_handletable | 30458.7ns | 443.4ns | 6869.7% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 434.9-449.8 ns)
    434.9 |########################
    435.7 |########################################
    436.4 |###############################
    437.1 |####################################
    437.9 |#################
    438.6 |#############
    439.4 |#
    440.1 |#
    440.9 |
    441.6 |
    442.4 |
    443.1 |#
    443.8 |
    444.6 |#
    445.3 |#
    446.1 |#
    446.8 |
    447.6 |###
    448.3 |
    449.1 |#
  (8 below, 9 above range)

v1_snapshot (n=120, range 433.7-456.1 ns)
    433.7 |##################
    434.9 |################################
    436.0 |########################################
    437.1 |##############
    438.2 |###
    439.3 |
    440.4 |##
    441.6 |
    442.7 |
    443.8 |
    444.9 |##
    446.0 |
    447.1 |
    448.2 |##
    449.4 |##
    450.5 |##
    451.6 |###
    452.7 |##
    453.8 |#
    454.9 |#
  (9 below, 4 above range)

v2_decomposed (n=120, range 434.4-464.7 ns)
    434.4 |########################################
    435.9 |###############################
    437.4 |#########
    439.0 |###
    440.5 |###
    442.0 |##
    443.5 |##
    445.0 |#
    446.5 |###
    448.1 |#
    449.6 |
    451.1 |
    452.6 |
    454.1 |
    455.6 |
    457.2 |##
    458.7 |
    460.2 |#
    461.7 |#
    463.2 |
  (12 below, 9 above range)

v3_shapebound (n=120, range 433.4-454.3 ns)
    433.4 |##############
    434.5 |########################################
    435.5 |####################
    436.6 |###########
    437.6 |####
    438.6 |#
    439.7 |#
    440.7 |#
    441.8 |##
    442.8 |#
    443.9 |#
    444.9 |#
    445.9 |#
    447.0 |#
    448.0 |##
    449.1 |###
    450.1 |##
    451.1 |#
    452.2 |#
    453.2 |
  (11 below, 9 above range)

v4_erased (n=120, range 434.8-455.7 ns)
    434.8 |#######################
    435.8 |########################################
    436.8 |#########################
    437.9 |#########
    438.9 |######
    440.0 |####
    441.0 |
    442.1 |##
    443.1 |
    444.2 |
    445.2 |#
    446.3 |##
    447.3 |
    448.4 |
    449.4 |#
    450.5 |##
    451.5 |
    452.6 |#
    453.6 |
    454.6 |
  (10 below, 11 above range)

v5_handletable (n=120, range 439.2-453.8 ns)
    439.2 |##################
    439.9 |##############################
    440.6 |########################################
    441.4 |###########################
    442.1 |#############
    442.8 |#############
    443.6 |#
    444.3 |#
    445.0 |####
    445.8 |
    446.5 |
    447.2 |
    448.0 |#
    448.7 |
    449.4 |#
    450.2 |
    450.9 |
    451.6 |#
    452.4 |
    453.1 |
  (9 below, 9 above range)

```

## Diagnostics

- **v0_blob**: bridge=6922.1% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=6934.4% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=6933.3% of algo (FFI overhead may distort results)
- **v3_shapebound**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **v3_shapebound**: bridge=6944.0% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=6925.9% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=6869.5% of algo (FFI overhead may distort results)
