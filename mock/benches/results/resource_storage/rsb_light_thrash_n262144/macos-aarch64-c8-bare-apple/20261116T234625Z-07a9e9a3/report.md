# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (57 ns) is smaller than the fastest variant's own run-to-run std-dev (60 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.9% of the fastest

All 6 variants sit between 6.50 us and 6.56 us - a 0.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v2_decomposed's edge over baseline is significant but tiny (7 ns, 0.11%)

v2_decomposed differs from baseline v0_blob by 7 ns (0.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v1_snapshot** at 6503.0 ns median (-0.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.01x (fastest 6503.0 ns, slowest 6559.6 ns)

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
| v0_blob | 7510ns | 7494ns | 6918ns | 7517ns | 8081ns | base |
| v1_snapshot | 7556ns | 7639ns | 6801ns | 7612ns | 8146ns | +0.61% |
| v2_decomposed | 7584ns | 7555ns | 7038ns | 7592ns | 8107ns | +0.98% |
| v3_shapebound | 7588ns | 7636ns | 6911ns | 7632ns | 8132ns | +1.03% |
| v4_erased | 7535ns | 7469ns | 6910ns | 7535ns | 8163ns | +0.33% |
| v5_handletable | 7575ns | 7597ns | 6882ns | 7620ns | 8137ns | +0.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 6547ns | 6489ns | 6671ns | base | 40.042 |
| v1_snapshot | 6536ns | 6495ns | 6636ns | -0.17% | 40.110 |
| v2_decomposed | 6559ns | 6495ns | 6707ns | +0.18% | 39.969 |
| v3_shapebound | 6551ns | 6488ns | 6728ns | +0.06% | 40.018 |
| v4_erased | 6567ns | 6505ns | 6725ns | +0.30% | 39.920 |
| v5_handletable | 6592ns | 6535ns | 6727ns | +0.69% | 39.767 |

## Performance model

- Peak throughput: **40.404 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 40.246 | 99.6% |
| v1_snapshot | 40.311 | 99.8% |
| v2_decomposed | 40.218 | 99.5% |
| v3_shapebound | 40.303 | 99.8% |
| v4_erased | 40.243 | 99.6% |
| v5_handletable | 39.963 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 7510ns | 7510ns | base |
| v1_snapshot | 7556ns | 7556ns | +0.61% |
| v2_decomposed | 7584ns | 7584ns | +0.98% |
| v3_shapebound | 7588ns | 7588ns | +1.03% |
| v4_erased | 7535ns | 7535ns | +0.33% |
| v5_handletable | 7575ns | 7575ns | +0.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 6514ns | base | --- | [6496, 6536] | --- | --- | --- | --- |
| v1_snapshot | 6503ns | no significant difference | [-6, +6]ns | [6500, 6513] | no | 0.5794 | 0.4635 | 1 |
| v2_decomposed | 6518ns | +7.2ns (+0.1%) | [+2, +10]ns | [6504, 6543] | YES | 0.0456 | 0.0274 | 1 |
| v3_shapebound | 6504ns | no significant difference | [-8, +2]ns | [6493, 6515] | no | 0.7140 | 0.7140 | 1 |
| v4_erased | 6514ns | +15.2ns (+0.2%) | [+9, +18]ns | [6509, 6539] | YES | 0.0334 | 0.0134 | 0 |
| v5_handletable | 6560ns | +47.0ns (+0.7%) | [+39, +52]ns | [6545, 6572] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 6494ns | +0.0% | +0.1% | +0.2% | +0.2% | +0.7% |
| 1 | 2 | 6613ns | -0.3% | -0.4% | +1.7% | -0.2% | -0.4% |
| 1 | 3 | 6521ns | -0.2% | -0.4% | -0.5% | -0.1% | +0.4% |
| 1 | 4 | 6617ns | -1.8% | -1.6% | -1.6% | -1.6% | -1.2% |
| 1 | 5 | 6524ns | -0.4% | -0.1% | -0.2% | +1.3% | +0.8% |
| 2 | 1 | 6542ns | -0.6% | -0.5% | -0.8% | -0.5% | +1.7% |
| 2 | 2 | 6493ns | +1.0% | +0.1% | +0.2% | +0.4% | +1.3% |
| 2 | 3 | 6505ns | -0.1% | -0.1% | -0.2% | +0.1% | +0.8% |
| 2 | 4 | 6510ns | -0.2% | -0.2% | -0.3% | +0.4% | +0.5% |
| 2 | 5 | 6494ns | +0.3% | +0.4% | +0.5% | +0.3% | +0.8% |
| 3 | 1 | 6559ns | -0.2% | +0.5% | -1.0% | -0.7% | +0.6% |
| 3 | 2 | 6492ns | +0.2% | +0.1% | +0.2% | +0.3% | +0.8% |
| 3 | 3 | 6490ns | +0.1% | +1.2% | +0.0% | +2.8% | +1.2% |
| 3 | 4 | 6565ns | -1.0% | -0.8% | -1.1% | -0.9% | -0.2% |
| 3 | 5 | 6497ns | +0.0% | +4.2% | +1.0% | +1.7% | +1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.254 | moderate- |
| v1_snapshot | -0.158 | ok |
| v2_decomposed | -0.091 | ok |
| v3_shapebound | -0.130 | ok |
| v4_erased | -0.379 | moderate- |
| v5_handletable | -0.036 | ok |

**Consistency summary:**

- **v1_snapshot**: won 8/15, lost 3/15
- **v2_decomposed**: won 8/15, lost 5/15
- **v3_shapebound**: won 8/15, lost 6/15
- **v4_erased**: won 5/15, lost 8/15
- **v5_handletable**: won 3/15, lost 12/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 485747.4ns | 6546.8ns | 7419.6% | HIGH |
| v1_snapshot | 484752.2ns | 6535.6ns | 7417.1% | HIGH |
| v2_decomposed | 486194.2ns | 6558.7ns | 7413.0% | HIGH |
| v3_shapebound | 490309.1ns | 6550.7ns | 7484.8% | HIGH |
| v4_erased | 486346.8ns | 6566.7ns | 7406.2% | HIGH |
| v5_handletable | 485991.2ns | 6592.0ns | 7372.4% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 6489.3-6671.2 ns)
   6489.3 |########################################
   6498.4 |####
   6507.5 |######
   6516.6 |##
   6525.7 |###
   6534.8 |#####
   6543.9 |##
   6553.0 |#####
   6562.1 |
   6571.2 |####
   6580.2 |##
   6589.3 |##
   6598.4 |
   6607.5 |##
   6616.6 |#
   6625.7 |
   6634.8 |###
   6643.9 |
   6653.0 |
   6662.1 |
  (9 below, 11 above range)

v1_snapshot (n=120, range 6495.2-6635.9 ns)
   6495.2 |########################################
   6502.3 |###########
   6509.3 |##
   6516.3 |###
   6523.4 |
   6530.4 |#
   6537.4 |##
   6544.5 |######
   6551.5 |#####
   6558.5 |##
   6565.5 |
   6572.6 |
   6579.6 |#
   6586.6 |
   6593.7 |#
   6600.7 |###
   6607.7 |
   6614.8 |
   6621.8 |
   6628.8 |
  (14 below, 10 above range)

v2_decomposed (n=120, range 6495.4-6706.7 ns)
   6495.4 |########################################
   6505.9 |#######
   6516.5 |##########
   6527.1 |##
   6537.6 |####
   6548.2 |#####
   6558.8 |#
   6569.3 |#######
   6579.9 |#####
   6590.5 |####
   6601.0 |###
   6611.6 |##
   6622.1 |
   6632.7 |###
   6643.3 |####
   6653.8 |##
   6664.4 |#
   6675.0 |
   6685.5 |##
   6696.1 |
  (12 below, 7 above range)

v3_shapebound (n=120, range 6488.1-6727.5 ns)
   6488.1 |########################################
   6500.1 |#######
   6512.0 |#########
   6524.0 |##
   6536.0 |###
   6547.9 |###
   6559.9 |#######
   6571.9 |#
   6583.9 |#
   6595.8 |
   6607.8 |
   6619.8 |###
   6631.8 |
   6643.7 |##
   6655.7 |
   6667.7 |#
   6679.6 |
   6691.6 |
   6703.6 |
   6715.6 |
  (14 below, 6 above range)

v4_erased (n=120, range 6504.6-6724.8 ns)
   6504.6 |########################################
   6515.6 |###
   6526.6 |###
   6537.6 |##
   6548.6 |####
   6559.6 |####
   6570.6 |####
   6581.7 |
   6592.7 |
   6603.7 |#######
   6614.7 |#
   6625.7 |#
   6636.7 |##
   6647.7 |
   6658.7 |#
   6669.7 |
   6680.7 |
   6691.7 |
   6702.7 |
   6713.7 |
  (12 below, 11 above range)

v5_handletable (n=120, range 6535.0-6726.6 ns)
   6535.0 |########################################
   6544.6 |#######
   6554.2 |########
   6563.7 |#########
   6573.3 |#####
   6582.9 |#####
   6592.5 |####
   6602.0 |######
   6611.6 |#
   6621.2 |#
   6630.8 |###
   6640.4 |##
   6649.9 |###
   6659.5 |##
   6669.1 |###
   6678.7 |
   6688.2 |##
   6697.8 |#
   6707.4 |###
   6717.0 |#
  (9 below, 6 above range)

```

## Diagnostics

- **v0_blob**: bridge=7422.6% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=7428.1% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=7415.0% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=7429.0% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=7417.2% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=7380.6% of algo (FFI overhead may distort results)
