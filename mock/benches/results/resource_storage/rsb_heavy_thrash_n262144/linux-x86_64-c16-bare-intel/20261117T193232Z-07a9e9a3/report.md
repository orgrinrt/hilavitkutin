# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (958 ns) is smaller than the fastest variant's own run-to-run std-dev (2.20 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader v4_erased vs stability leader v5_handletable (+4% speed for 1.1x steadier)

v4_erased is fastest (25.00 us, CV 8.8%); v5_handletable gives up 3.8% median for 1.1x lower variance (CV 7.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 3.8% of the fastest

All 6 variants sit between 25.00 us and 25.96 us - a 3.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v1_snapshot's edge over baseline is significant but tiny (-22 ns, 0.08%)

v1_snapshot differs from baseline v0_blob by -22 ns (0.08%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v4_erased** at 24999.2 ns median (-3.6% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.04x (fastest 24999.2 ns, slowest 25957.6 ns)

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
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 28906ns | 30188ns | 24842ns | 29614ns | 30846ns | base |
| v1_snapshot | 28405ns | 30176ns | 24688ns | 28783ns | 30987ns | -1.73% |
| v2_decomposed | 28735ns | 30222ns | 24734ns | 29109ns | 31616ns | -0.59% |
| v3_shapebound | 29719ns | 30206ns | 25633ns | 30439ns | 31645ns | +2.81% |
| v4_erased | 27796ns | 29073ns | 24694ns | 27938ns | 30472ns | -3.84% |
| v5_handletable | 29517ns | 30213ns | 25216ns | 30254ns | 31606ns | +2.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 24857ns | 21353ns | 26548ns | base | 10.546 |
| v1_snapshot | 24415ns | 21208ns | 26700ns | -1.78% | 10.737 |
| v2_decomposed | 24750ns | 21254ns | 27334ns | -0.43% | 10.592 |
| v3_shapebound | 25580ns | 22021ns | 27352ns | +2.91% | 10.248 |
| v4_erased | 23906ns | 21209ns | 26192ns | -3.83% | 10.966 |
| v5_handletable | 25386ns | 21676ns | 27306ns | +2.13% | 10.326 |

## Performance model

- Peak throughput: **12.360 Gops/s** (v1_snapshot; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.107 | 81.8% |
| v1_snapshot | 10.112 | 81.8% |
| v2_decomposed | 10.099 | 81.7% |
| v3_shapebound | 10.102 | 81.7% |
| v4_erased | 10.486 | 84.8% |
| v5_handletable | 10.104 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 28906ns | 28906ns | base |
| v1_snapshot | 28405ns | 28405ns | -1.73% |
| v2_decomposed | 28735ns | 28735ns | -0.59% |
| v3_shapebound | 29719ns | 29719ns | +2.81% |
| v4_erased | 27796ns | 27796ns | -3.84% |
| v5_handletable | 29517ns | 29517ns | +2.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 25937ns | base | --- | [25930, 25944] | --- | --- | --- | --- |
| v1_snapshot | 25924ns | -21.9ns (-0.1%) | [-35, -9]ns | [25917, 25929] | YES | 0.0002 | 0.0000 | 0 |
| v2_decomposed | 25958ns | +29.0ns (+0.1%) | [+10, +109]ns | [25942, 25977] | YES | 0.0098 | 0.0078 | 0 |
| v3_shapebound | 25949ns | +23.5ns (+0.1%) | [+4, +345]ns | [25940, 25961] | YES | 0.0074 | 0.0045 | 0 |
| v4_erased | 24999ns | -93.1ns (-0.4%) | [-1054, -10]ns | [22328, 25924] | YES | 0.0032 | 0.0013 | 0 |
| v5_handletable | 25943ns | +39.5ns (+0.2%) | [+1, +482]ns | [25935, 25964] | YES | 0.0353 | 0.0353 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 21246ns | -0.2% | +0.1% | +28.2% | +5.0% | +24.3% |
| 1 | 2 | 25940ns | -18.2% | +5.5% | -9.1% | -14.1% | -0.0% |
| 1 | 3 | 25934ns | -0.0% | +5.0% | +0.0% | -0.0% | +0.3% |
| 1 | 4 | 26393ns | -19.5% | -1.7% | +3.3% | -1.8% | -1.6% |
| 1 | 5 | 25932ns | -0.0% | +0.1% | +0.0% | -0.0% | -18.1% |
| 2 | 1 | 21250ns | +22.1% | +22.3% | +23.2% | +22.0% | +22.1% |
| 2 | 2 | 25948ns | -0.0% | +0.1% | -0.1% | -0.0% | +2.5% |
| 2 | 3 | 25960ns | +1.6% | -13.7% | +5.1% | -18.3% | +5.2% |
| 2 | 4 | 25945ns | -0.1% | +4.9% | +0.0% | -18.2% | +1.3% |
| 2 | 5 | 21355ns | +21.4% | +8.6% | +21.5% | +21.5% | +21.5% |
| 3 | 1 | 25937ns | -0.0% | +0.0% | +5.7% | -0.0% | -0.0% |
| 3 | 2 | 25951ns | -0.1% | +0.1% | -0.0% | -13.2% | -18.1% |
| 3 | 3 | 25954ns | -18.2% | -18.0% | +0.0% | -18.3% | -18.1% |
| 3 | 4 | 22300ns | -4.8% | -4.8% | -4.7% | -4.7% | +16.3% |
| 3 | 5 | 25958ns | -18.3% | -18.0% | -18.2% | -0.0% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.264 | moderate- |
| v1_snapshot | 0.358 | moderate+ |
| v2_decomposed | 0.153 | ok |
| v3_shapebound | 0.323 | moderate+ |
| v4_erased | 0.273 | moderate+ |
| v5_handletable | 0.232 | moderate+ |

**Consistency summary:**

- **v1_snapshot**: won 6/15, lost 3/15
- **v2_decomposed**: won 5/15, lost 7/15
- **v3_shapebound**: won 3/15, lost 6/15
- **v4_erased**: won 7/15, lost 3/15
- **v5_handletable**: won 5/15, lost 8/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 590346.5ns | 24857.1ns | 2375.0% | HIGH |
| v1_snapshot | 581605.1ns | 24414.9ns | 2382.2% | HIGH |
| v2_decomposed | 580958.3ns | 24750.0ns | 2347.3% | HIGH |
| v3_shapebound | 603298.9ns | 25580.0ns | 2358.5% | HIGH |
| v4_erased | 565720.3ns | 23906.2ns | 2366.4% | HIGH |
| v5_handletable | 601232.0ns | 25386.4ns | 2368.3% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 21353.2-26548.5 ns)
  21353.2 |
  21613.0 |
  21872.7 |
  22132.5 |#####
  22392.3 |
  22652.0 |
  22911.8 |
  23171.5 |
  23431.3 |
  23691.1 |
  23950.8 |
  24210.6 |
  24470.4 |
  24730.1 |
  24989.9 |
  25249.6 |
  25509.4 |
  25769.2 |########################################
  26028.9 |###
  26288.7 |##
  (20 below, 8 above range)

v1_snapshot (n=120, range 21208.3-26700.0 ns)
  21208.3 |###################
  21482.9 |
  21757.5 |
  22032.1 |
  22306.7 |
  22581.3 |
  22855.8 |
  23130.4 |
  23405.0 |
  23679.6 |
  23954.2 |
  24228.8 |
  24503.3 |
  24777.9 |
  25052.5 |
  25327.1 |
  25601.7 |
  25876.3 |########################################
  26150.8 |###
  26425.4 |
  (13 below, 10 above range)

v2_decomposed (n=120, range 21254.0-27333.6 ns)
  21254.0 |##################
  21558.0 |#
  21861.9 |
  22165.9 |#########
  22469.9 |###
  22773.9 |
  23077.9 |#
  23381.8 |
  23685.8 |
  23989.8 |#
  24293.8 |
  24597.8 |
  24901.7 |
  25205.7 |
  25509.7 |
  25813.7 |########################################
  26117.7 |#####
  26421.6 |#
  26725.6 |
  27029.6 |#######################
  (11 below, 7 above range)

v3_shapebound (n=120, range 22021.4-27352.4 ns)
  22021.4 |
  22288.0 |
  22554.5 |
  22821.1 |
  23087.6 |
  23354.2 |
  23620.7 |
  23887.3 |
  24153.8 |
  24420.4 |
  24686.9 |
  24953.4 |
  25220.0 |
  25486.5 |
  25753.1 |########################################
  26019.6 |##
  26286.2 |##
  26552.7 |
  26819.3 |
  27085.8 |#################
  (19 below, 9 above range)

v4_erased (n=120, range 21209.4-26191.5 ns)
  21209.4 |############################
  21458.5 |#
  21707.7 |
  21956.8 |#
  22205.9 |##################
  22455.0 |#####
  22704.1 |
  22953.2 |
  23202.3 |
  23451.4 |
  23700.5 |##
  23949.6 |
  24198.7 |
  24447.8 |
  24696.9 |#
  24946.0 |
  25195.1 |#
  25444.2 |
  25693.3 |########################################
  25942.4 |######################
  (10 below, 6 above range)

v5_handletable (n=120, range 21676.4-27306.2 ns)
  21676.4 |
  21957.9 |
  22239.4 |#
  22520.9 |
  22802.4 |
  23083.9 |
  23365.4 |
  23646.8 |
  23928.3 |
  24209.8 |
  24491.3 |
  24772.8 |
  25054.3 |
  25335.8 |
  25617.3 |
  25898.8 |########################################
  26180.3 |#
  26461.8 |#
  26743.3 |
  27024.7 |###########
  (19 below, 4 above range)

```

## Diagnostics

- **v0_blob**: bridge=2394.1% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2395.2% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2392.1% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2393.3% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2255.8% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2393.9% of algo (FFI overhead may distort results)
