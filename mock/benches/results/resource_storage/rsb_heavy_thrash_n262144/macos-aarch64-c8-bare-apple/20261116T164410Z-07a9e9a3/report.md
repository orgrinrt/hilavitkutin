# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.8% of the fastest

All 6 variants sit between 6.50 us and 6.56 us - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v1_snapshot's edge over baseline is significant but tiny (6 ns, 0.10%)

v1_snapshot differs from baseline v0_blob by 6 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 6504.2 ns median (-0.0% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.01x (fastest 6504.2 ns, slowest 6557.1 ns)

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
| v0_blob | 12124ns | 12113ns | 11788ns | 12129ns | 12444ns | base |
| v1_snapshot | 12143ns | 12133ns | 11841ns | 12147ns | 12434ns | +0.16% |
| v2_decomposed | 12113ns | 12171ns | 11740ns | 12143ns | 12394ns | -0.09% |
| v3_shapebound | 12090ns | 12126ns | 11739ns | 12119ns | 12356ns | -0.28% |
| v4_erased | 12097ns | 12122ns | 11694ns | 12120ns | 12429ns | -0.22% |
| v5_handletable | 12039ns | 12084ns | 11505ns | 12083ns | 12438ns | -0.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 6521ns | 6498ns | 6584ns | base | 40.197 |
| v1_snapshot | 6543ns | 6505ns | 6665ns | +0.33% | 40.067 |
| v2_decomposed | 6523ns | 6506ns | 6559ns | +0.02% | 40.189 |
| v3_shapebound | 6524ns | 6497ns | 6600ns | +0.03% | 40.184 |
| v4_erased | 6540ns | 6514ns | 6619ns | +0.29% | 40.081 |
| v5_handletable | 6572ns | 6550ns | 6631ns | +0.78% | 39.886 |

## Performance model

- Peak throughput: **40.347 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 40.298 | 99.9% |
| v1_snapshot | 40.263 | 99.8% |
| v2_decomposed | 40.239 | 99.7% |
| v3_shapebound | 40.303 | 99.9% |
| v4_erased | 40.211 | 99.7% |
| v5_handletable | 39.979 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 12124ns | 12124ns | base |
| v1_snapshot | 12143ns | 12143ns | +0.16% |
| v2_decomposed | 12113ns | 12113ns | -0.09% |
| v3_shapebound | 12090ns | 12090ns | -0.28% |
| v4_erased | 12097ns | 12097ns | -0.22% |
| v5_handletable | 12039ns | 12039ns | -0.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 6505ns | base | --- | [6504, 6508] | --- | --- | --- | --- |
| v1_snapshot | 6511ns | +6.4ns (+0.1%) | [+4, +8]ns | [6509, 6513] | YES | 0.0016 | 0.0013 | 0 |
| v2_decomposed | 6515ns | +7.8ns (+0.1%) | [+5, +10]ns | [6512, 6516] | YES | 0.0000 | 0.0000 | 0 |
| v3_shapebound | 6504ns | no significant difference | [-4, +2]ns | [6503, 6506] | no | 0.6483 | 0.6483 | 0 |
| v4_erased | 6519ns | +14.1ns (+0.2%) | [+11, +18]ns | [6518, 6521] | YES | 0.0000 | 0.0000 | 0 |
| v5_handletable | 6557ns | +51.1ns (+0.8%) | [+48, +54]ns | [6555, 6564] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 6502ns | +0.1% | +0.3% | +0.1% | +0.5% | +1.0% |
| 1 | 2 | 6509ns | -0.0% | +0.0% | -0.1% | +0.4% | +0.7% |
| 1 | 3 | 6507ns | +0.3% | +0.3% | +0.1% | +0.2% | +0.7% |
| 1 | 4 | 6510ns | -0.0% | +0.2% | -0.1% | +0.1% | +0.7% |
| 1 | 5 | 6502ns | +0.1% | +0.2% | +0.0% | +0.2% | +1.0% |
| 2 | 1 | 6513ns | +0.0% | -0.0% | -0.2% | +0.1% | +0.8% |
| 2 | 2 | 6505ns | +0.1% | +0.3% | +0.0% | +0.2% | +0.8% |
| 2 | 3 | 6513ns | -0.0% | -0.0% | -0.2% | +0.0% | +0.6% |
| 2 | 4 | 6506ns | +0.2% | +0.1% | +0.5% | +0.2% | +0.8% |
| 2 | 5 | 6505ns | +0.1% | +0.2% | -0.0% | +0.4% | +0.9% |
| 3 | 1 | 6500ns | +0.2% | +0.2% | +0.0% | +0.3% | +1.1% |
| 3 | 2 | 6504ns | +0.1% | +0.1% | +0.0% | +0.2% | +1.0% |
| 3 | 3 | 6603ns | -1.4% | -1.3% | -1.4% | -1.3% | -0.8% |
| 3 | 4 | 6510ns | +0.3% | +0.0% | +0.0% | +0.4% | +0.8% |
| 3 | 5 | 6509ns | +0.2% | +0.1% | -0.1% | +0.1% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.077 | ok |
| v1_snapshot | 0.007 | ok |
| v2_decomposed | -0.422 | moderate- |
| v3_shapebound | -0.273 | moderate- |
| v4_erased | 0.117 | ok |
| v5_handletable | 0.123 | ok |

**Consistency summary:**

- **v1_snapshot**: won 1/15, lost 8/15
- **v2_decomposed**: won 1/15, lost 8/15
- **v3_shapebound**: won 5/15, lost 3/15
- **v4_erased**: won 1/15, lost 13/15
- **v5_handletable**: won 1/15, lost 14/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 484774.7ns | 6521.4ns | 7433.6% | HIGH |
| v1_snapshot | 484995.3ns | 6542.6ns | 7412.8% | HIGH |
| v2_decomposed | 484132.0ns | 6522.7ns | 7422.3% | HIGH |
| v3_shapebound | 484691.9ns | 6523.6ns | 7429.8% | HIGH |
| v4_erased | 484792.9ns | 6540.4ns | 7412.3% | HIGH |
| v5_handletable | 484595.0ns | 6572.4ns | 7373.2% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 6498.2-6584.4 ns)
   6498.2 |########################################
   6502.6 |######################################
   6506.9 |########
   6511.2 |#####
   6515.5 |################
   6519.8 |########
   6524.1 |########
   6528.4 |#
   6532.7 |#
   6537.0 |
   6541.3 |
   6545.6 |
   6550.0 |#
   6554.3 |#
   6558.6 |#
   6562.9 |#
   6567.2 |####
   6571.5 |
   6575.8 |
   6580.1 |#
  (12 below, 7 above range)

v1_snapshot (n=120, range 6505.3-6665.2 ns)
   6505.3 |########################################
   6513.3 |#####
   6521.3 |#######
   6529.3 |####
   6537.3 |##
   6545.3 |#
   6553.3 |
   6561.3 |
   6569.3 |
   6577.3 |##
   6585.3 |
   6593.2 |
   6601.2 |
   6609.2 |#
   6617.2 |#
   6625.2 |
   6633.2 |
   6641.2 |
   6649.2 |
   6657.2 |
  (12 below, 7 above range)

v2_decomposed (n=120, range 6506.5-6559.4 ns)
   6506.5 |########################################
   6509.1 |################################
   6511.8 |####################
   6514.4 |################################
   6517.1 |##############
   6519.7 |########
   6522.4 |####
   6525.0 |##
   6527.6 |####################
   6530.3 |##########
   6532.9 |######
   6535.6 |
   6538.2 |##
   6540.9 |######
   6543.5 |##
   6546.2 |##
   6548.8 |
   6551.5 |####
   6554.1 |
   6556.8 |
  (11 below, 7 above range)

v3_shapebound (n=120, range 6497.2-6599.7 ns)
   6497.2 |########################################
   6502.3 |#########################
   6507.4 |######
   6512.6 |##
   6517.7 |######
   6522.8 |#
   6527.9 |#
   6533.1 |
   6538.2 |##
   6543.3 |#
   6548.4 |#
   6553.6 |
   6558.7 |
   6563.8 |
   6568.9 |
   6574.1 |###
   6579.2 |
   6584.3 |
   6589.4 |#
   6594.6 |
  (8 below, 6 above range)

v4_erased (n=120, range 6513.8-6619.0 ns)
   6513.8 |########################################
   6519.0 |###########
   6524.3 |###
   6529.5 |######
   6534.8 |#######
   6540.1 |##########
   6545.3 |#
   6550.6 |
   6555.8 |
   6561.1 |
   6566.4 |
   6571.6 |
   6576.9 |
   6582.1 |
   6587.4 |
   6592.7 |#
   6597.9 |
   6603.2 |
   6608.4 |
   6613.7 |
  (11 below, 7 above range)

v5_handletable (n=120, range 6549.6-6630.9 ns)
   6549.6 |########################################
   6553.7 |#############################
   6557.8 |######
   6561.8 |#########
   6565.9 |##############
   6570.0 |############
   6574.0 |##
   6578.1 |######
   6582.2 |#
   6586.2 |
   6590.3 |#####
   6594.4 |##
   6598.4 |##
   6602.5 |#
   6606.5 |
   6610.6 |
   6614.7 |
   6618.7 |##
   6622.8 |#
   6626.9 |#
  (10 below, 5 above range)

```

## Diagnostics

- **v0_blob**: bridge=7439.1% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=7432.5% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=7428.6% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=7440.5% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=7423.1% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=7383.8% of algo (FFI overhead may distort results)
