# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 21.3x slower than the field

reg_bound_record (352.84 us) is 21.3x the fastest (16.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_typed_twin, reg_typed) are a dead heat (<1%)

reg_typed_twin (16.54 us) and reg_typed (16.54 us) differ by 0.03%, inside the noise, even though the wider field spreads 2033.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 21.3x the fastest

Fastest reg_typed_twin (16.54 us) to slowest reg_bound_record (352.84 us): 21.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: reg_typed_twin** at 16539.8 ns median (-0.0% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 21.33x (fastest 16539.8 ns, slowest 352839.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Registration: typed binding vs a run-time binding table over a column workload |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71c40e2b95a3d86f |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 20275ns | 20309ns | 19523ns | 20280ns | 21015ns | +2.99% |
| reg_bound_record | 361452ns | 356288ns | 355022ns | 357005ns | 381221ns | +1736.08% |
| reg_bound_strided | 83033ns | 82791ns | 82124ns | 82855ns | 84473ns | +321.78% |
| reg_bound_units | 20666ns | 20683ns | 19831ns | 20675ns | 21477ns | +4.98% |
| reg_typed | 19686ns | 19691ns | 19083ns | 19697ns | 20256ns | base |
| reg_typed_twin | 19695ns | 19667ns | 19165ns | 19693ns | 20231ns | +0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 17368ns | 17123ns | 17928ns | +4.43% | 15.094 |
| reg_bound_record | 358259ns | 351884ns | 378452ns | +2054.11% | 0.732 |
| reg_bound_strided | 79966ns | 79293ns | 81600ns | +380.81% | 3.278 |
| reg_bound_units | 17812ns | 17541ns | 18374ns | +7.10% | 14.718 |
| reg_typed | 16631ns | 16467ns | 16952ns | base | 15.762 |
| reg_typed_twin | 16621ns | 16466ns | 16964ns | -0.06% | 15.772 |

## Performance model

- Peak throughput: **15.920 Gops/s** (reg_typed_twin; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 15.221 | 95.6% |
| reg_bound_record | 0.743 | 4.7% |
| reg_bound_strided | 3.298 | 20.7% |
| reg_bound_units | 14.830 | 93.1% |
| reg_typed | 15.845 | 99.5% |
| reg_typed_twin | 15.849 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 20275ns | 20275ns | +2.99% |
| reg_bound_record | 361452ns | 361452ns | +1736.08% |
| reg_bound_strided | 83033ns | 83033ns | +321.78% |
| reg_bound_units | 20666ns | 20666ns | +4.98% |
| reg_typed | 19686ns | 19686ns | base |
| reg_typed_twin | 19695ns | 19695ns | +0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 16544ns | base | --- | [16529, 16570] | --- | --- | --- | --- |
| reg_bound_columns | 17222ns | +671.9ns (+4.1%) | [+647, +689]ns | [17198, 17248] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 352839ns | +336286.3ns (+2032.7%) | [+335837, +336940]ns | [352392, 353550] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 79492ns | +62912.6ns (+380.3%) | [+62861, +62981]ns | [79443, 79585] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 17677ns | +1115.9ns (+6.7%) | [+1069, +1163]ns | [17661, 17700] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 16540ns | no significant difference | [-29, +13]ns | [16514, 16556] | no | 0.9370 | 0.9370 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 16786ns | +2.4% | +2001.5% | +373.3% | +5.3% | -1.7% |
| 1 | 2 | 16475ns | +4.5% | +2044.0% | +392.8% | +6.6% | -0.0% |
| 1 | 3 | 16505ns | +4.3% | +2033.8% | +383.9% | +7.1% | +0.1% |
| 1 | 4 | 16544ns | +4.4% | +2029.7% | +379.6% | +8.4% | -0.3% |
| 1 | 5 | 16543ns | +3.8% | +2040.4% | +380.7% | +6.0% | +0.7% |
| 1 | 6 | 16710ns | +4.2% | +2039.8% | +375.2% | +5.7% | +0.3% |
| 1 | 7 | 16510ns | +5.5% | +2051.4% | +385.9% | +7.3% | +0.8% |
| 1 | 8 | 16532ns | +3.9% | +2030.1% | +380.1% | +9.4% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | 0.025 | ok |
| reg_bound_record | 0.309 | moderate+ |
| reg_bound_strided | -0.182 | ok |
| reg_bound_units | -0.093 | ok |
| reg_typed | -0.247 | moderate- |
| reg_typed_twin | 0.407 | moderate+ |

**Consistency summary:**

- **reg_bound_columns**: won 0/8, lost 8/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 0/8, lost 8/8
- **reg_typed_twin**: won 3/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 519772.6ns | 17368.0ns | 2992.7% | HIGH |
| reg_bound_record | 521822.4ns | 358259.2ns | 145.7% | HIGH |
| reg_bound_strided | 515871.5ns | 79966.1ns | 645.1% | HIGH |
| reg_bound_units | 517396.7ns | 17811.7ns | 2904.8% | HIGH |
| reg_typed | 514513.9ns | 16631.4ns | 3093.6% | HIGH |
| reg_typed_twin | 514873.4ns | 16620.9ns | 3097.7% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 17123.4-17928.0 ns)
  17123.4 |###########################
  17163.6 |########################################
  17203.8 |###########################
  17244.1 |############
  17284.3 |#####
  17324.5 |###
  17364.8 |#####
  17405.0 |######
  17445.2 |######
  17485.5 |#########
  17525.7 |#
  17565.9 |#####
  17606.2 |###
  17646.4 |###
  17686.6 |#####
  17726.9 |#####
  17767.1 |###
  17807.3 |#
  17847.6 |#
  17887.8 |
  (18 below, 7 above range)

reg_bound_record (n=160, range 351883.8-378451.8 ns)
  351883.8 |########################################
  353212.2 |########
  354540.6 |######
  355869.0 |#####
  357197.4 |#####
  358525.8 |#
  359854.2 |##
  361182.6 |#
  362511.0 |#
  363839.4 |#
  365167.8 |
  366496.2 |
  367824.6 |
  369153.0 |
  370481.4 |
  371809.8 |
  373138.2 |
  374466.6 |
  375795.0 |
  377123.4 |
  (10 below, 4 above range)

reg_bound_strided (n=160, range 79293.5-81600.1 ns)
  79293.5 |########################################
  79408.8 |###########################
  79524.1 |#########
  79639.5 |####
  79754.8 |####
  79870.1 |####
  79985.5 |##
  80100.8 |######
  80216.1 |##
  80331.4 |###
  80446.8 |#
  80562.1 |##
  80677.4 |##
  80792.8 |
  80908.1 |##
  81023.4 |#
  81138.7 |####
  81254.1 |
  81369.4 |
  81484.7 |
  (14 below, 13 above range)

reg_bound_units (n=160, range 17541.3-18373.6 ns)
  17541.3 |#############################
  17582.9 |########################
  17624.6 |########################################
  17666.2 |#################################
  17707.8 |###############
  17749.4 |#####
  17791.0 |#############
  17832.6 |###
  17874.3 |###
  17915.9 |############
  17957.5 |########
  17999.1 |#####
  18040.7 |##########
  18082.3 |###
  18123.9 |############
  18165.6 |###
  18207.2 |#
  18248.8 |###
  18290.4 |#
  18332.0 |
  (20 below, 7 above range)

reg_typed (n=160, range 16466.9-16952.2 ns)
  16466.9 |########################################
  16491.1 |#######################
  16515.4 |################################
  16539.7 |#####################
  16563.9 |##########
  16588.2 |###
  16612.5 |###
  16636.7 |####
  16661.0 |####
  16685.3 |############
  16709.5 |#######
  16733.8 |#######
  16758.1 |###
  16782.3 |############
  16806.6 |###
  16830.9 |####
  16855.1 |#
  16879.4 |###
  16903.7 |#
  16927.9 |###
  (15 below, 13 above range)

reg_typed_twin (n=160, range 16465.9-16964.5 ns)
  16465.9 |########################################
  16490.8 |##############
  16515.7 |###############
  16540.7 |##########
  16565.6 |########
  16590.5 |###
  16615.5 |#
  16640.4 |#############
  16665.3 |########
  16690.2 |##
  16715.2 |#
  16740.1 |
  16765.0 |####
  16790.0 |###
  16814.9 |##
  16839.8 |#
  16864.8 |###
  16889.7 |#
  16914.6 |###
  16939.5 |######
  (15 below, 8 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=2977.5% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=145.4% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=645.0% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=2902.3% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=3098.2% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=3099.1% of algo (FFI overhead may distort results)
