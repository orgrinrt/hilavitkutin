# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.9% of the fastest

All 6 variants sit between 6.50 us and 6.56 us - a 0.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v1_snapshot's edge over baseline is significant but tiny (9 ns, 0.14%)

v1_snapshot differs from baseline v0_blob by 9 ns (0.14%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 6502.0 ns median (-0.2% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.01x (fastest 6502.0 ns, slowest 6558.5 ns)

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
| v0_blob | 12095ns | 12151ns | 11449ns | 12129ns | 12638ns | base |
| v1_snapshot | 12145ns | 12256ns | 11441ns | 12192ns | 12707ns | +0.41% |
| v2_decomposed | 12095ns | 12118ns | 11427ns | 12119ns | 12688ns | +0.00% |
| v3_shapebound | 12094ns | 12141ns | 11444ns | 12131ns | 12634ns | -0.00% |
| v4_erased | 12190ns | 12184ns | 11551ns | 12227ns | 12719ns | +0.79% |
| v5_handletable | 12077ns | 12032ns | 11470ns | 12089ns | 12648ns | -0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 6537ns | 6489ns | 6642ns | base | 40.101 |
| v1_snapshot | 6574ns | 6495ns | 6777ns | +0.57% | 39.874 |
| v2_decomposed | 6555ns | 6497ns | 6680ns | +0.27% | 39.992 |
| v3_shapebound | 6525ns | 6487ns | 6607ns | -0.19% | 40.176 |
| v4_erased | 6549ns | 6504ns | 6651ns | +0.19% | 40.025 |
| v5_handletable | 6583ns | 6540ns | 6675ns | +0.70% | 39.821 |

## Performance model

- Peak throughput: **40.414 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 40.257 | 99.6% |
| v1_snapshot | 40.181 | 99.4% |
| v2_decomposed | 40.205 | 99.5% |
| v3_shapebound | 40.317 | 99.8% |
| v4_erased | 40.175 | 99.4% |
| v5_handletable | 39.970 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 12095ns | 12095ns | base |
| v1_snapshot | 12145ns | 12145ns | +0.41% |
| v2_decomposed | 12095ns | 12095ns | +0.00% |
| v3_shapebound | 12094ns | 12094ns | -0.00% |
| v4_erased | 12190ns | 12190ns | +0.79% |
| v5_handletable | 12077ns | 12077ns | -0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 6512ns | base | --- | [6497, 6533] | --- | --- | --- | --- |
| v1_snapshot | 6524ns | +9.2ns (+0.1%) | [+4, +26]ns | [6514, 6546] | YES | 0.0061 | 0.0024 | 0 |
| v2_decomposed | 6520ns | +9.2ns (+0.1%) | [+4, +15]ns | [6514, 6546] | YES | 0.0098 | 0.0078 | 0 |
| v3_shapebound | 6502ns | no significant difference | [-13, +1]ns | [6496, 6512] | no | 0.0985 | 0.0985 | 1 |
| v4_erased | 6525ns | +11.8ns (+0.2%) | [+6, +17]ns | [6510, 6537] | YES | 0.0074 | 0.0045 | 0 |
| v5_handletable | 6558ns | +50.3ns (+0.8%) | [+47, +53]ns | [6546, 6573] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 6530ns | -0.4% | -0.1% | -0.2% | -0.3% | +0.3% |
| 1 | 2 | 6535ns | +0.4% | +0.1% | +0.5% | +0.5% | +0.3% |
| 1 | 3 | 6496ns | +0.0% | +0.1% | -0.0% | +0.2% | +0.7% |
| 1 | 4 | 6494ns | +0.7% | +0.2% | +0.1% | +0.5% | +0.8% |
| 1 | 5 | 6526ns | -0.5% | -0.2% | -0.5% | -0.2% | +0.3% |
| 2 | 1 | 6554ns | +0.9% | -0.6% | -0.8% | +0.3% | +0.5% |
| 2 | 2 | 6496ns | +0.1% | +0.2% | +0.2% | +0.3% | +1.0% |
| 2 | 3 | 6508ns | -0.2% | +0.1% | +0.0% | +0.0% | +0.9% |
| 2 | 4 | 6497ns | +0.2% | +2.5% | -0.1% | +0.1% | +0.7% |
| 2 | 5 | 6495ns | +0.1% | +0.3% | +0.7% | +0.2% | +0.9% |
| 3 | 1 | 6554ns | +0.1% | -0.1% | -1.0% | -0.3% | +0.6% |
| 3 | 2 | 6493ns | +0.4% | +0.1% | -0.0% | +0.5% | +1.8% |
| 3 | 3 | 6493ns | +4.2% | +0.2% | +0.2% | +0.4% | +0.8% |
| 3 | 4 | 6502ns | +0.0% | +2.6% | +0.0% | +0.2% | +0.6% |
| 3 | 5 | 6536ns | +0.3% | +0.3% | +0.0% | +1.2% | +1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.116 | ok |
| v1_snapshot | -0.249 | moderate- |
| v2_decomposed | -0.095 | ok |
| v3_shapebound | -0.143 | ok |
| v4_erased | -0.202 | moderate- |
| v5_handletable | 0.001 | ok |

**Consistency summary:**

- **v1_snapshot**: won 3/15, lost 8/15
- **v2_decomposed**: won 3/15, lost 11/15
- **v3_shapebound**: won 5/15, lost 4/15
- **v4_erased**: won 3/15, lost 11/15
- **v5_handletable**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 485507.0ns | 6537.1ns | 7426.9% | HIGH |
| v1_snapshot | 486641.1ns | 6574.3ns | 7402.2% | HIGH |
| v2_decomposed | 485957.0ns | 6555.0ns | 7413.6% | HIGH |
| v3_shapebound | 484646.7ns | 6524.9ns | 7427.6% | HIGH |
| v4_erased | 486492.3ns | 6549.5ns | 7428.0% | HIGH |
| v5_handletable | 485232.2ns | 6583.0ns | 7371.0% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 6489.2-6642.5 ns)
   6489.2 |########################################
   6496.9 |########
   6504.6 |###
   6512.2 |#####
   6519.9 |###
   6527.5 |######
   6535.2 |###########
   6542.9 |###
   6550.5 |##
   6558.2 |
   6565.9 |#####
   6573.5 |###
   6581.2 |###
   6588.8 |##
   6596.5 |##
   6604.2 |###
   6611.8 |
   6619.5 |####
   6627.1 |
   6634.8 |#
  (12 below, 7 above range)

v1_snapshot (n=120, range 6495.0-6777.1 ns)
   6495.0 |########################################
   6509.1 |############
   6523.2 |#####
   6537.3 |#########
   6551.4 |#########
   6565.5 |#######
   6579.6 |#
   6593.8 |########
   6607.9 |##
   6622.0 |####
   6636.1 |#
   6650.2 |#
   6664.3 |#
   6678.4 |#
   6692.5 |#
   6706.6 |
   6720.7 |
   6734.8 |##
   6748.9 |
   6763.0 |
  (11 below, 8 above range)

v2_decomposed (n=120, range 6496.6-6680.4 ns)
   6496.6 |########################################
   6505.7 |###########
   6514.9 |############
   6524.1 |#
   6533.3 |######
   6542.5 |########
   6551.7 |#######
   6560.9 |###
   6570.1 |##
   6579.3 |#####
   6588.5 |###
   6597.7 |##
   6606.8 |#####
   6616.0 |#
   6625.2 |###
   6634.4 |###
   6643.6 |
   6652.8 |#
   6662.0 |
   6671.2 |#
  (12 below, 11 above range)

v3_shapebound (n=120, range 6486.5-6607.0 ns)
   6486.5 |########################################
   6492.6 |###################
   6498.6 |######
   6504.6 |#######
   6510.6 |#########
   6516.7 |#
   6522.7 |#####
   6528.7 |#####
   6534.7 |#####
   6540.7 |##
   6546.8 |#
   6552.8 |##
   6558.8 |###
   6564.8 |#####
   6570.9 |###
   6576.9 |##
   6582.9 |#####
   6588.9 |###
   6595.0 |#
   6601.0 |
  (11 below, 7 above range)

v4_erased (n=120, range 6503.7-6651.0 ns)
   6503.7 |########################################
   6511.1 |#######
   6518.5 |####
   6525.8 |##########
   6533.2 |
   6540.5 |###
   6547.9 |####
   6555.3 |######
   6562.6 |#####
   6570.0 |######
   6577.4 |##
   6584.7 |##
   6592.1 |##
   6599.4 |##
   6606.8 |##
   6614.2 |
   6621.5 |##
   6628.9 |##
   6636.2 |#
   6643.6 |
  (12 below, 10 above range)

v5_handletable (n=120, range 6540.4-6675.4 ns)
   6540.4 |########################################
   6547.1 |###
   6553.9 |###
   6560.6 |#####
   6567.4 |####
   6574.1 |
   6580.9 |#####
   6587.7 |####
   6594.4 |###
   6601.2 |
   6607.9 |#
   6614.7 |######
   6621.4 |##
   6628.2 |#
   6634.9 |####
   6641.7 |#
   6648.4 |
   6655.2 |
   6661.9 |
   6668.7 |#
  (12 below, 7 above range)

```

## Diagnostics

- **v0_blob**: bridge=7425.2% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=7421.4% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=7421.6% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=7429.5% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=7404.0% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=7373.6% of algo (FFI overhead may distort results)
