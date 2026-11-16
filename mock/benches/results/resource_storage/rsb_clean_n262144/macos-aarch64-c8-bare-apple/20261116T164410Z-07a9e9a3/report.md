# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.7% of the fastest

All 6 variants sit between 6.51 us and 6.55 us - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v1_snapshot's edge over baseline is significant but tiny (8 ns, 0.12%)

v1_snapshot differs from baseline v0_blob by 8 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 6506.1 ns median (-0.0% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.01x (fastest 6506.1 ns, slowest 6550.4 ns)

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
| v0_blob | 7500ns | 7512ns | 7174ns | 7513ns | 7788ns | base |
| v1_snapshot | 7456ns | 7486ns | 7084ns | 7483ns | 7745ns | -0.59% |
| v2_decomposed | 7394ns | 7471ns | 6880ns | 7437ns | 7780ns | -1.41% |
| v3_shapebound | 7453ns | 7498ns | 7054ns | 7489ns | 7742ns | -0.63% |
| v4_erased | 7427ns | 7446ns | 7053ns | 7446ns | 7745ns | -0.97% |
| v5_handletable | 7461ns | 7488ns | 7041ns | 7491ns | 7788ns | -0.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 6519ns | 6499ns | 6572ns | base | 40.209 |
| v1_snapshot | 6533ns | 6504ns | 6611ns | +0.21% | 40.124 |
| v2_decomposed | 6535ns | 6506ns | 6613ns | +0.24% | 40.113 |
| v3_shapebound | 6522ns | 6498ns | 6582ns | +0.04% | 40.192 |
| v4_erased | 6536ns | 6514ns | 6595ns | +0.26% | 40.107 |
| v5_handletable | 6574ns | 6542ns | 6662ns | +0.83% | 39.878 |

## Performance model

- Peak throughput: **40.343 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 40.285 | 99.9% |
| v1_snapshot | 40.247 | 99.8% |
| v2_decomposed | 40.236 | 99.7% |
| v3_shapebound | 40.292 | 99.9% |
| v4_erased | 40.200 | 99.6% |
| v5_handletable | 40.020 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 7500ns | 7500ns | base |
| v1_snapshot | 7456ns | 7456ns | -0.59% |
| v2_decomposed | 7394ns | 7394ns | -1.41% |
| v3_shapebound | 7453ns | 7453ns | -0.63% |
| v4_erased | 7427ns | 7427ns | -0.97% |
| v5_handletable | 7461ns | 7461ns | -0.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 6507ns | base | --- | [6504, 6510] | --- | --- | --- | --- |
| v1_snapshot | 6513ns | +8.1ns (+0.1%) | [+4, +12]ns | [6512, 6518] | YES | 0.0011 | 0.0009 | 1 |
| v2_decomposed | 6515ns | +8.9ns (+0.1%) | [+6, +12]ns | [6513, 6518] | YES | 0.0000 | 0.0000 | 0 |
| v3_shapebound | 6506ns | no significant difference | [-2, +4]ns | [6505, 6511] | no | 0.6483 | 0.6483 | 0 |
| v4_erased | 6521ns | +14.3ns (+0.2%) | [+11, +18]ns | [6520, 6523] | YES | 0.0000 | 0.0000 | 0 |
| v5_handletable | 6550ns | +43.1ns (+0.7%) | [+40, +47]ns | [6549, 6553] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 6515ns | -0.1% | -0.0% | -0.0% | +0.2% | +0.5% |
| 1 | 2 | 6508ns | +0.1% | +0.3% | -0.1% | +0.2% | +0.6% |
| 1 | 3 | 6506ns | +0.1% | +0.0% | +0.0% | +0.1% | +0.9% |
| 1 | 4 | 6504ns | +0.3% | +0.5% | -0.0% | +0.4% | +0.9% |
| 1 | 5 | 6502ns | +0.1% | +0.2% | +0.1% | +0.3% | +0.7% |
| 2 | 1 | 6504ns | +0.3% | +0.2% | +0.0% | +0.2% | +0.7% |
| 2 | 2 | 6505ns | +0.3% | +0.1% | +0.7% | +0.2% | +0.7% |
| 2 | 3 | 6514ns | -0.1% | +0.3% | +0.6% | +0.4% | +0.5% |
| 2 | 4 | 6510ns | -0.0% | +0.0% | -0.0% | +0.3% | +0.8% |
| 2 | 5 | 6502ns | +0.3% | +0.2% | +0.2% | +0.4% | +0.7% |
| 3 | 1 | 6516ns | +0.1% | -0.0% | -0.2% | +0.1% | +0.5% |
| 3 | 2 | 6517ns | +0.1% | -0.0% | -0.2% | +0.0% | +0.5% |
| 3 | 3 | 6509ns | +0.1% | +1.0% | +0.0% | +0.1% | +2.2% |
| 3 | 4 | 6508ns | +0.0% | +0.0% | +0.2% | +0.3% | +0.6% |
| 3 | 5 | 6502ns | +0.1% | +0.3% | -0.0% | +0.3% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.212 | moderate+ |
| v1_snapshot | -0.108 | ok |
| v2_decomposed | -0.318 | moderate- |
| v3_shapebound | 0.334 | moderate+ |
| v4_erased | -0.045 | ok |
| v5_handletable | -0.152 | ok |

**Consistency summary:**

- **v1_snapshot**: won 1/15, lost 8/15
- **v2_decomposed**: won 0/15, lost 9/15
- **v3_shapebound**: won 2/15, lost 4/15
- **v4_erased**: won 0/15, lost 13/15
- **v5_handletable**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 484473.6ns | 6519.5ns | 7431.2% | HIGH |
| v1_snapshot | 484809.7ns | 6533.3ns | 7420.6% | HIGH |
| v2_decomposed | 484923.6ns | 6535.1ns | 7420.3% | HIGH |
| v3_shapebound | 484401.3ns | 6522.3ns | 7426.9% | HIGH |
| v4_erased | 484298.1ns | 6536.1ns | 7409.5% | HIGH |
| v5_handletable | 485176.2ns | 6573.6ns | 7380.6% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 6498.6-6572.2 ns)
   6498.6 |########################################
   6502.3 |#########################
   6506.0 |##################
   6509.7 |############
   6513.4 |#######
   6517.0 |##########
   6520.7 |##########
   6524.4 |#
   6528.1 |#####
   6531.8 |#####
   6535.4 |#
   6539.1 |
   6542.8 |
   6546.5 |
   6550.2 |##
   6553.8 |
   6557.5 |##
   6561.2 |#
   6564.9 |
   6568.6 |
  (10 below, 8 above range)

v1_snapshot (n=120, range 6504.2-6611.2 ns)
   6504.2 |########################################
   6509.5 |##############################
   6514.9 |##############
   6520.2 |####
   6525.6 |#########
   6530.9 |##########
   6536.3 |####
   6541.6 |
   6547.0 |##
   6552.3 |####
   6557.7 |
   6563.0 |
   6568.4 |#
   6573.7 |
   6579.1 |##
   6584.4 |##
   6589.8 |##
   6595.1 |
   6600.5 |#
   6605.8 |#
  (12 below, 9 above range)

v2_decomposed (n=120, range 6506.1-6613.5 ns)
   6506.1 |######################################
   6511.5 |########################################
   6516.8 |######
   6522.2 |#########
   6527.6 |########
   6532.9 |#########
   6538.3 |##
   6543.7 |####
   6549.1 |####
   6554.4 |
   6559.8 |##
   6565.2 |#
   6570.5 |#
   6575.9 |##
   6581.3 |
   6586.6 |
   6592.0 |#
   6597.4 |#
   6602.7 |#
   6608.1 |##
  (12 below, 7 above range)

v3_shapebound (n=120, range 6497.8-6582.4 ns)
   6497.8 |######################################
   6502.1 |########################################
   6506.3 |################
   6510.5 |########
   6514.8 |######
   6519.0 |###########
   6523.2 |###########
   6527.5 |###
   6531.7 |####
   6535.9 |######
   6540.1 |
   6544.4 |###
   6548.6 |#
   6552.8 |
   6557.1 |#
   6561.3 |#
   6565.5 |#
   6569.8 |
   6574.0 |#
   6578.2 |###
  (11 below, 9 above range)

v4_erased (n=120, range 6513.5-6595.4 ns)
   6513.5 |########################################
   6517.6 |###################################
   6521.7 |#################
   6525.8 |####
   6529.9 |#####
   6534.0 |######
   6538.1 |#########
   6542.2 |##
   6546.3 |#####
   6550.4 |####
   6554.4 |##
   6558.5 |
   6562.6 |#
   6566.7 |
   6570.8 |
   6574.9 |##
   6579.0 |
   6583.1 |
   6587.2 |
   6591.3 |##
  (9 below, 8 above range)

v5_handletable (n=120, range 6542.1-6662.3 ns)
   6542.1 |########################################
   6548.1 |###############################
   6554.1 |#######
   6560.1 |######
   6566.1 |#######
   6572.1 |##
   6578.1 |
   6584.1 |###
   6590.1 |#
   6596.2 |##
   6602.2 |#
   6608.2 |####
   6614.2 |#
   6620.2 |##
   6626.2 |#
   6632.2 |#
   6638.2 |
   6644.2 |
   6650.3 |
   6656.3 |#
  (8 below, 9 above range)

```

## Diagnostics

- **v0_blob**: bridge=7436.6% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=7430.0% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=7428.9% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=7437.7% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=7420.3% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=7390.6% of algo (FFI overhead may distort results)
