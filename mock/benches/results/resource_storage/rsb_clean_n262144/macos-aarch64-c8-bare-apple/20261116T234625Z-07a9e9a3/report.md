# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (43 ns) is smaller than the fastest variant's own run-to-run std-dev (115 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.7% of the fastest

All 6 variants sit between 6.51 us and 6.55 us - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v4_erased's edge over baseline is significant but tiny (15 ns, 0.23%)

v4_erased differs from baseline v0_blob by 15 ns (0.23%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 6508.2 ns median (-0.4% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.01x (fastest 6508.2 ns, slowest 6550.8 ns)

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
| v0_blob | 7436ns | 7351ns | 6873ns | 7367ns | 8209ns | base |
| v1_snapshot | 7384ns | 7403ns | 6827ns | 7388ns | 7928ns | -0.70% |
| v2_decomposed | 7396ns | 7376ns | 6919ns | 7374ns | 7939ns | -0.55% |
| v3_shapebound | 7576ns | 7596ns | 7016ns | 7605ns | 8052ns | +1.88% |
| v4_erased | 7366ns | 7370ns | 6805ns | 7358ns | 7952ns | -0.95% |
| v5_handletable | 7393ns | 7380ns | 6758ns | 7394ns | 8023ns | -0.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 6632ns | 6490ns | 7067ns | base | 39.527 |
| v1_snapshot | 6548ns | 6496ns | 6667ns | -1.27% | 40.036 |
| v2_decomposed | 6565ns | 6495ns | 6739ns | -1.00% | 39.928 |
| v3_shapebound | 6558ns | 6487ns | 6742ns | -1.12% | 39.974 |
| v4_erased | 6571ns | 6505ns | 6730ns | -0.91% | 39.892 |
| v5_handletable | 6579ns | 6533ns | 6674ns | -0.80% | 39.845 |

## Performance model

- Peak throughput: **40.411 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 40.121 | 99.3% |
| v1_snapshot | 40.207 | 99.5% |
| v2_decomposed | 40.201 | 99.5% |
| v3_shapebound | 40.279 | 99.7% |
| v4_erased | 40.107 | 99.2% |
| v5_handletable | 40.017 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 7436ns | 7436ns | base |
| v1_snapshot | 7384ns | 7384ns | -0.70% |
| v2_decomposed | 7396ns | 7396ns | -0.55% |
| v3_shapebound | 7576ns | 7576ns | +1.88% |
| v4_erased | 7366ns | 7366ns | -0.95% |
| v5_handletable | 7393ns | 7393ns | -0.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 6534ns | base | --- | [6515, 6546] | --- | --- | --- | --- |
| v1_snapshot | 6520ns | no significant difference | [-15, +6]ns | [6506, 6535] | no | 0.7843 | 0.7843 | 0 |
| v2_decomposed | 6521ns | no significant difference | [-7, +12]ns | [6505, 6546] | no | 0.3941 | 0.3153 | 0 |
| v3_shapebound | 6508ns | no significant difference | [-11, +0]ns | [6493, 6524] | no | 0.1373 | 0.0824 | 0 |
| v4_erased | 6536ns | +15.1ns (+0.2%) | [+5, +20]ns | [6522, 6550] | YES | 0.0334 | 0.0134 | 0 |
| v5_handletable | 6551ns | +40.4ns (+0.6%) | [+19, +44]ns | [6541, 6575] | YES | 0.0017 | 0.0003 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 6495ns | +0.3% | +0.1% | +0.0% | +0.4% | +0.6% |
| 1 | 2 | 6537ns | +0.5% | +1.2% | +3.3% | +0.3% | +0.7% |
| 1 | 3 | 6506ns | -0.1% | -0.1% | -0.2% | -0.0% | +0.6% |
| 1 | 4 | 6711ns | -3.1% | -3.1% | -3.3% | -2.9% | -2.6% |
| 1 | 5 | 6560ns | -0.3% | -1.0% | +0.2% | +1.0% | -0.1% |
| 2 | 1 | 6566ns | -0.5% | -0.9% | -1.1% | +2.9% | +0.8% |
| 2 | 2 | 6517ns | -0.3% | +0.5% | -0.5% | +0.1% | +0.3% |
| 2 | 3 | 6493ns | +0.3% | +0.7% | +0.4% | +0.2% | +0.6% |
| 2 | 4 | 6498ns | +0.3% | +0.4% | +0.2% | +1.0% | +2.0% |
| 2 | 5 | 6498ns | +0.2% | +0.0% | +0.4% | +0.2% | +0.6% |
| 3 | 1 | 6547ns | -0.6% | +0.6% | -0.7% | -0.5% | +0.8% |
| 3 | 2 | 6502ns | -0.1% | -0.0% | -0.2% | +0.6% | +0.7% |
| 3 | 3 | 6573ns | -0.1% | -1.0% | +0.0% | +0.0% | -0.5% |
| 3 | 4 | 6508ns | +0.6% | +0.0% | -0.3% | -0.0% | +0.5% |
| 3 | 5 | 6797ns | -4.3% | -0.3% | -1.4% | -3.6% | -3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.081 | ok |
| v1_snapshot | -0.118 | ok |
| v2_decomposed | -0.125 | ok |
| v3_shapebound | -0.259 | moderate- |
| v4_erased | 0.162 | ok |
| v5_handletable | -0.411 | moderate- |

**Consistency summary:**

- **v1_snapshot**: won 8/15, lost 6/15
- **v2_decomposed**: won 6/15, lost 6/15
- **v3_shapebound**: won 8/15, lost 5/15
- **v4_erased**: won 3/15, lost 8/15
- **v5_handletable**: won 3/15, lost 11/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 490324.1ns | 6632.0ns | 7393.3% | HIGH |
| v1_snapshot | 489357.7ns | 6547.7ns | 7473.8% | HIGH |
| v2_decomposed | 491513.3ns | 6565.3ns | 7486.5% | HIGH |
| v3_shapebound | 487793.0ns | 6557.8ns | 7438.3% | HIGH |
| v4_erased | 486569.6ns | 6571.4ns | 7404.3% | HIGH |
| v5_handletable | 485570.9ns | 6579.1ns | 7380.5% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 6490.0-7067.3 ns)
   6490.0 |########################################
   6518.9 |#######################
   6547.7 |############
   6576.6 |##########
   6605.5 |####
   6634.3 |####
   6663.2 |###
   6692.1 |##
   6720.9 |#
   6749.8 |
   6778.7 |##
   6807.5 |#
   6836.4 |##
   6865.3 |
   6894.1 |
   6923.0 |
   6951.9 |
   6980.7 |
   7009.6 |#
   7038.5 |
  (12 below, 4 above range)

v1_snapshot (n=120, range 6495.7-6667.1 ns)
   6495.7 |########################################
   6504.2 |#########
   6512.8 |###########
   6521.4 |#####
   6529.9 |######
   6538.5 |#########
   6547.1 |####
   6555.6 |##
   6564.2 |###
   6572.8 |##########
   6581.4 |###
   6589.9 |##
   6598.5 |##
   6607.1 |
   6615.6 |#
   6624.2 |##
   6632.8 |
   6641.4 |
   6649.9 |#
   6658.5 |#
  (10 below, 8 above range)

v2_decomposed (n=120, range 6495.5-6739.0 ns)
   6495.5 |########################################
   6507.7 |#####
   6519.8 |######
   6532.0 |##
   6544.2 |##########
   6556.4 |###
   6568.6 |##
   6580.7 |######
   6592.9 |
   6605.1 |#
   6617.3 |#
   6629.4 |#
   6641.6 |#
   6653.8 |##
   6666.0 |
   6678.1 |
   6690.3 |#
   6702.5 |##
   6714.7 |
   6726.9 |
  (9 below, 6 above range)

v3_shapebound (n=120, range 6487.0-6742.3 ns)
   6487.0 |########################################
   6499.8 |#######
   6512.5 |#####
   6525.3 |######
   6538.1 |##
   6550.8 |#
   6563.6 |###
   6576.4 |##
   6589.1 |###
   6601.9 |##
   6614.7 |##
   6627.4 |
   6640.2 |#
   6653.0 |
   6665.7 |
   6678.5 |
   6691.2 |
   6704.0 |
   6716.8 |##
   6729.5 |#
  (10 below, 9 above range)

v4_erased (n=120, range 6504.9-6730.0 ns)
   6504.9 |########################################
   6516.2 |#########
   6527.4 |#########
   6538.7 |#########
   6549.9 |###########
   6561.2 |####
   6572.4 |###
   6583.7 |####
   6594.9 |##
   6606.2 |######
   6617.5 |##
   6628.7 |##
   6640.0 |##
   6651.2 |
   6662.5 |#
   6673.7 |#
   6685.0 |
   6696.2 |
   6707.5 |#
   6718.8 |##
  (11 below, 10 above range)

v5_handletable (n=120, range 6533.2-6673.8 ns)
   6533.2 |########################################
   6540.2 |###########
   6547.3 |#####
   6554.3 |###
   6561.3 |#
   6568.4 |###
   6575.4 |####
   6582.4 |########
   6589.5 |#
   6596.5 |##
   6603.5 |###
   6610.6 |###
   6617.6 |####
   6624.6 |#######
   6631.6 |##
   6638.7 |####
   6645.7 |
   6652.7 |#
   6659.8 |##
   6666.8 |
  (11 below, 8 above range)

```

## Diagnostics

- **v0_blob**: bridge=7430.4% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=7415.2% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=7420.8% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=7426.4% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=7409.3% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=7383.3% of algo (FFI overhead may distort results)
