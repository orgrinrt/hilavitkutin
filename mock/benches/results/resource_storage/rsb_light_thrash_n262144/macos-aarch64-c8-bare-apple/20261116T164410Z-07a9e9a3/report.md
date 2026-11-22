# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (46 ns) is smaller than the fastest variant's own run-to-run std-dev (55 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.7% of the fastest

All 6 variants sit between 6.50 us and 6.55 us - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v1_snapshot's edge over baseline is significant but tiny (8 ns, 0.12%)

v1_snapshot differs from baseline v0_blob by 8 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v3_shapebound** at 6504.2 ns median (-0.0% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.01x (fastest 6504.2 ns, slowest 6550.1 ns)

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
| v0_blob | 7538ns | 7553ns | 7169ns | 7556ns | 7855ns | base |
| v1_snapshot | 7586ns | 7624ns | 7213ns | 7614ns | 7876ns | +0.64% |
| v2_decomposed | 7530ns | 7601ns | 7016ns | 7582ns | 7888ns | -0.10% |
| v3_shapebound | 7583ns | 7654ns | 7177ns | 7613ns | 7900ns | +0.60% |
| v4_erased | 7570ns | 7662ns | 7079ns | 7622ns | 7903ns | +0.42% |
| v5_handletable | 7576ns | 7641ns | 7153ns | 7620ns | 7864ns | +0.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 6524ns | 6499ns | 6593ns | base | 40.185 |
| v1_snapshot | 6535ns | 6505ns | 6617ns | +0.18% | 40.112 |
| v2_decomposed | 6529ns | 6505ns | 6580ns | +0.08% | 40.151 |
| v3_shapebound | 6525ns | 6497ns | 6608ns | +0.03% | 40.173 |
| v4_erased | 6529ns | 6513ns | 6568ns | +0.09% | 40.148 |
| v5_handletable | 6557ns | 6543ns | 6586ns | +0.51% | 39.980 |

## Performance model

- Peak throughput: **40.347 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 40.294 | 99.9% |
| v1_snapshot | 40.249 | 99.8% |
| v2_decomposed | 40.222 | 99.7% |
| v3_shapebound | 40.303 | 99.9% |
| v4_erased | 40.208 | 99.7% |
| v5_handletable | 40.021 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 7538ns | 7538ns | base |
| v1_snapshot | 7586ns | 7586ns | +0.64% |
| v2_decomposed | 7530ns | 7530ns | -0.10% |
| v3_shapebound | 7583ns | 7583ns | +0.60% |
| v4_erased | 7570ns | 7570ns | +0.42% |
| v5_handletable | 7576ns | 7576ns | +0.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 6506ns | base | --- | [6504, 6509] | --- | --- | --- | --- |
| v1_snapshot | 6513ns | +8.1ns (+0.1%) | [+5, +10]ns | [6511, 6519] | YES | 0.0001 | 0.0001 | 1 |
| v2_decomposed | 6517ns | +9.3ns (+0.1%) | [+7, +13]ns | [6514, 6523] | YES | 0.0000 | 0.0000 | 0 |
| v3_shapebound | 6504ns | no significant difference | [-5, +2]ns | [6503, 6507] | no | 0.4114 | 0.4114 | 0 |
| v4_erased | 6520ns | +13.1ns (+0.2%) | [+9, +16]ns | [6518, 6522] | YES | 0.0000 | 0.0000 | 1 |
| v5_handletable | 6550ns | +43.6ns (+0.7%) | [+41, +46]ns | [6548, 6554] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 6518ns | +0.3% | +0.0% | -0.1% | +0.0% | +0.5% |
| 1 | 2 | 6502ns | +0.1% | +0.3% | +0.4% | +0.2% | +0.9% |
| 1 | 3 | 6504ns | +0.1% | +0.4% | -0.0% | +0.3% | +0.8% |
| 1 | 4 | 6512ns | +0.1% | +0.1% | -0.1% | +0.1% | +0.6% |
| 1 | 5 | 6519ns | -0.1% | +0.1% | -0.2% | +0.3% | +0.4% |
| 2 | 1 | 6515ns | +0.1% | -0.0% | -0.1% | +0.1% | +0.5% |
| 2 | 2 | 6507ns | +0.6% | +0.1% | +0.2% | +0.2% | +0.6% |
| 2 | 3 | 6505ns | +0.1% | +0.1% | -0.0% | +0.2% | +0.6% |
| 2 | 4 | 6505ns | +0.1% | +0.1% | +0.0% | +0.2% | +0.8% |
| 2 | 5 | 6519ns | -0.0% | +0.1% | -0.3% | -0.0% | +0.6% |
| 3 | 1 | 6512ns | -0.0% | +0.1% | -0.2% | +0.3% | +0.6% |
| 3 | 2 | 6503ns | +0.3% | +0.2% | +0.1% | +0.5% | +0.7% |
| 3 | 3 | 6508ns | +0.1% | +0.0% | -0.1% | +0.2% | +0.7% |
| 3 | 4 | 6504ns | +0.1% | +0.4% | +0.0% | +0.2% | +0.7% |
| 3 | 5 | 6503ns | +0.1% | +0.2% | +0.1% | +0.2% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.125 | ok |
| v1_snapshot | -0.046 | ok |
| v2_decomposed | 0.122 | ok |
| v3_shapebound | -0.048 | ok |
| v4_erased | 0.090 | ok |
| v5_handletable | 0.279 | moderate+ |

**Consistency summary:**

- **v1_snapshot**: won 1/15, lost 10/15
- **v2_decomposed**: won 0/15, lost 9/15
- **v3_shapebound**: won 6/15, lost 3/15
- **v4_erased**: won 0/15, lost 12/15
- **v5_handletable**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 484575.6ns | 6523.5ns | 7428.2% | HIGH |
| v1_snapshot | 484662.5ns | 6535.2ns | 7416.1% | HIGH |
| v2_decomposed | 484267.9ns | 6529.0ns | 7417.2% | HIGH |
| v3_shapebound | 484730.5ns | 6525.3ns | 7428.5% | HIGH |
| v4_erased | 484119.3ns | 6529.4ns | 7414.5% | HIGH |
| v5_handletable | 484148.6ns | 6557.0ns | 7383.7% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 6499.4-6592.7 ns)
   6499.4 |########################################
   6504.0 |########################
   6508.7 |##########
   6513.4 |#####
   6518.0 |#######
   6522.7 |###
   6527.4 |#####
   6532.0 |#
   6536.7 |##
   6541.4 |#
   6546.0 |##
   6550.7 |
   6555.4 |
   6560.0 |
   6564.7 |
   6569.4 |
   6574.0 |#
   6578.7 |
   6583.3 |
   6588.0 |#
  (10 below, 11 above range)

v1_snapshot (n=120, range 6505.2-6616.7 ns)
   6505.2 |########################################
   6510.8 |##################
   6516.3 |############
   6521.9 |######
   6527.5 |########
   6533.1 |##
   6538.7 |##
   6544.2 |##
   6549.8 |#
   6555.4 |##
   6561.0 |##
   6566.5 |###
   6572.1 |#
   6577.7 |###
   6583.3 |
   6588.9 |#
   6594.4 |
   6600.0 |#
   6605.6 |#
   6611.2 |
  (10 below, 9 above range)

v2_decomposed (n=120, range 6505.4-6580.4 ns)
   6505.4 |##################
   6509.1 |########################################
   6512.9 |##########
   6516.6 |####################
   6520.4 |#######
   6524.1 |#######
   6527.9 |##########
   6531.6 |###############
   6535.4 |###
   6539.1 |######
   6542.9 |#
   6546.6 |#
   6550.4 |#
   6554.1 |######
   6557.9 |
   6561.6 |
   6565.4 |###
   6569.1 |
   6572.9 |###
   6576.6 |
  (10 below, 8 above range)

v3_shapebound (n=120, range 6497.3-6607.6 ns)
   6497.3 |########################################
   6502.8 |########################
   6508.3 |########
   6513.8 |#######
   6519.4 |######
   6524.9 |##
   6530.4 |###
   6535.9 |#
   6541.4 |#
   6546.9 |###
   6552.5 |#
   6558.0 |##
   6563.5 |
   6569.0 |
   6574.5 |#
   6580.0 |##
   6585.6 |
   6591.1 |##
   6596.6 |#
   6602.1 |
  (13 below, 7 above range)

v4_erased (n=120, range 6513.0-6567.6 ns)
   6513.0 |####################################
   6515.7 |########################################
   6518.5 |#############################
   6521.2 |####################
   6523.9 |###
   6526.7 |#####
   6529.4 |###
   6532.1 |#########
   6534.8 |############
   6537.6 |#######
   6540.3 |#
   6543.0 |###
   6545.8 |###
   6548.5 |
   6551.2 |#######
   6554.0 |
   6556.7 |
   6559.4 |#####
   6562.2 |
   6564.9 |
  (9 below, 7 above range)

v5_handletable (n=120, range 6542.7-6586.1 ns)
   6542.7 |#######################
   6544.9 |########################################
   6547.0 |###########################
   6549.2 |##############
   6551.4 |##################
   6553.5 |#####################
   6555.7 |##########
   6557.9 |####
   6560.0 |####
   6562.2 |##
   6564.4 |########
   6566.5 |##
   6568.7 |################
   6570.9 |########
   6573.1 |####
   6575.2 |##
   6577.4 |####
   6579.6 |
   6581.7 |
   6583.9 |
  (12 below, 7 above range)

```

## Diagnostics

- **v0_blob**: bridge=7436.9% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=7428.1% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=7425.7% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=7440.2% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=7421.1% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=7388.8% of algo (FFI overhead may distort results)
