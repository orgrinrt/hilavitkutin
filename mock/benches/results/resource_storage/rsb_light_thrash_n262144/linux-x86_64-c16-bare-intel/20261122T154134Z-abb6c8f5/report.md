# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (v0_blob) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline v0_blob has the worst median (25.97 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest v1_snapshot at 25.94 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (32 ns) is smaller than the fastest variant's own run-to-run std-dev (1.96 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader v1_snapshot vs stability leader v0_blob (+0% speed for 1.2x steadier)

v1_snapshot is fastest (25.94 us, CV 7.5%); v0_blob gives up 0.1% median for 1.2x lower variance (CV 6.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.1% of the fastest

All 6 variants sit between 25.94 us and 25.97 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v1_snapshot's edge over baseline is significant but tiny (-36 ns, 0.14%)

v1_snapshot differs from baseline v0_blob by -36 ns (0.14%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v1_snapshot** at 25936.6 ns median (-0.1% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.00x (fastest 25936.6 ns, slowest 25968.6 ns)

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
| Counter frequency | 2112842590 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 26249ns | 26168ns | 24025ns | 26574ns | 27499ns | base |
| v1_snapshot | 25521ns | 26143ns | 21945ns | 26141ns | 27235ns | -2.77% |
| v2_decomposed | 25148ns | 26146ns | 21686ns | 25538ns | 27440ns | -4.19% |
| v3_shapebound | 25213ns | 26141ns | 21701ns | 25707ns | 27246ns | -3.95% |
| v4_erased | 25706ns | 26134ns | 22356ns | 26167ns | 27672ns | -2.07% |
| v5_handletable | 25601ns | 26146ns | 22423ns | 26149ns | 27138ns | -2.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 26055ns | 23845ns | 27299ns | base | 10.061 |
| v1_snapshot | 25324ns | 21782ns | 27031ns | -2.80% | 10.351 |
| v2_decomposed | 24961ns | 21521ns | 27235ns | -4.20% | 10.502 |
| v3_shapebound | 25026ns | 21542ns | 27042ns | -3.95% | 10.475 |
| v4_erased | 25514ns | 22185ns | 27470ns | -2.08% | 10.275 |
| v5_handletable | 25417ns | 22260ns | 26943ns | -2.45% | 10.314 |

## Performance model

- Peak throughput: **12.181 Gops/s** (v2_decomposed; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.095 | 82.9% |
| v1_snapshot | 10.107 | 83.0% |
| v2_decomposed | 10.101 | 82.9% |
| v3_shapebound | 10.104 | 83.0% |
| v4_erased | 10.107 | 83.0% |
| v5_handletable | 10.099 | 82.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 26249ns | 26249ns | base |
| v1_snapshot | 25521ns | 25521ns | -2.77% |
| v2_decomposed | 25148ns | 25148ns | -4.19% |
| v3_shapebound | 25213ns | 25213ns | -3.95% |
| v4_erased | 25706ns | 25706ns | -2.07% |
| v5_handletable | 25601ns | 25601ns | -2.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 25969ns | base | --- | [25944, 26343] | --- | --- | --- | --- |
| v1_snapshot | 25937ns | -36.1ns (-0.1%) | [-204, -12]ns | [25928, 25943] | YES | 0.0022 | 0.0013 | 0 |
| v2_decomposed | 25954ns | -51.7ns (-0.2%) | [-974, -7]ns | [25940, 25967] | YES | 0.0167 | 0.0134 | 0 |
| v3_shapebound | 25944ns | -652.7ns (-2.5%) | [-1287, -15]ns | [25937, 25953] | YES | 0.0022 | 0.0013 | 0 |
| v4_erased | 25938ns | -35.4ns (-0.1%) | [-232, -15]ns | [25930, 25947] | YES | 0.0022 | 0.0007 | 0 |
| v5_handletable | 25956ns | no significant difference | [-999, +12]ns | [25950, 25966] | no | 0.6483 | 0.6483 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 21221ns | +22.2% | +0.0% | +22.5% | +0.1% | +22.3% |
| 1 | 2 | 25953ns | -0.0% | +0.1% | -0.0% | -0.1% | +0.0% |
| 1 | 3 | 27239ns | -4.8% | -4.5% | -4.7% | -4.8% | -4.6% |
| 1 | 4 | 25968ns | -0.1% | -13.9% | -0.1% | -0.1% | -0.0% |
| 1 | 5 | 26142ns | -18.8% | -0.5% | -18.8% | -18.9% | -0.8% |
| 2 | 1 | 25944ns | +3.7% | -8.7% | -13.9% | +0.0% | +0.1% |
| 2 | 2 | 25942ns | -0.1% | +0.0% | +5.3% | -0.0% | +0.1% |
| 2 | 3 | 27245ns | -4.8% | -4.4% | -7.5% | -4.8% | -4.8% |
| 2 | 4 | 26629ns | +2.2% | -20.2% | -2.3% | -2.6% | -20.3% |
| 2 | 5 | 27240ns | -4.8% | -4.7% | -4.3% | -7.9% | -4.8% |
| 3 | 1 | 25934ns | +0.0% | -14.1% | +0.0% | +5.0% | +6.7% |
| 3 | 2 | 25926ns | -0.0% | +0.1% | +0.2% | +5.0% | -18.1% |
| 3 | 3 | 27240ns | -22.1% | -4.5% | -18.1% | -4.7% | -4.7% |
| 3 | 4 | 25939ns | +0.0% | +0.0% | -18.2% | +0.0% | +0.1% |
| 3 | 5 | 27234ns | -0.0% | -0.0% | -4.7% | -4.8% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.021 | ok |
| v1_snapshot | -0.163 | ok |
| v2_decomposed | -0.324 | moderate- |
| v3_shapebound | 0.182 | ok |
| v4_erased | -0.029 | ok |
| v5_handletable | -0.247 | moderate- |

**Consistency summary:**

- **v1_snapshot**: won 6/15, lost 3/15
- **v2_decomposed**: won 9/15, lost 1/15
- **v3_shapebound**: won 10/15, lost 3/15
- **v4_erased**: won 7/15, lost 2/15
- **v5_handletable**: won 7/15, lost 4/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 611601.3ns | 26054.7ns | 2347.4% | HIGH |
| v1_snapshot | 601951.3ns | 25324.4ns | 2377.0% | HIGH |
| v2_decomposed | 587320.5ns | 24961.4ns | 2352.9% | HIGH |
| v3_shapebound | 590488.7ns | 25026.1ns | 2359.5% | HIGH |
| v4_erased | 603848.2ns | 25513.9ns | 2366.7% | HIGH |
| v5_handletable | 604086.3ns | 25416.6ns | 2376.7% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 23845.4-27299.0 ns)
  23845.4 |
  24018.1 |
  24190.8 |
  24363.5 |
  24536.1 |
  24708.8 |
  24881.5 |
  25054.2 |
  25226.8 |
  25399.5 |
  25572.2 |
  25744.9 |##
  25917.5 |########################################
  26090.2 |##
  26262.9 |##
  26435.6 |#
  26608.3 |
  26780.9 |
  26953.6 |
  27126.3 |#############################
  (11 below, 7 above range)

v1_snapshot (n=120, range 21781.6-27030.5 ns)
  21781.6 |
  22044.0 |
  22306.5 |
  22568.9 |
  22831.4 |
  23093.8 |
  23356.3 |
  23618.7 |
  23881.2 |
  24143.6 |
  24406.1 |
  24668.5 |
  24931.0 |
  25193.4 |
  25455.9 |
  25718.3 |########################################
  25980.7 |#
  26243.2 |
  26505.6 |
  26768.1 |
  (21 below, 15 above range)

v2_decomposed (n=120, range 21521.0-27234.8 ns)
  21521.0 |
  21806.7 |
  22092.4 |#########
  22378.1 |##
  22663.8 |
  22949.5 |
  23235.1 |
  23520.8 |
  23806.5 |
  24092.2 |
  24377.9 |
  24663.6 |
  24949.3 |
  25235.0 |
  25520.6 |
  25806.3 |########################################
  26092.0 |
  26377.7 |
  26663.4 |##
  26949.1 |#####
  (17 below, 11 above range)

v3_shapebound (n=120, range 21542.4-27042.1 ns)
  21542.4 |
  21817.3 |
  22092.3 |##########
  22367.3 |#
  22642.3 |
  22917.3 |
  23192.3 |
  23467.3 |
  23742.3 |
  24017.3 |
  24292.3 |
  24567.2 |
  24842.2 |
  25117.2 |
  25392.2 |
  25667.2 |####################
  25942.2 |########################################
  26217.2 |
  26492.2 |#
  26767.2 |
  (17 below, 16 above range)

v4_erased (n=120, range 22185.4-27469.8 ns)
  22185.4 |
  22449.6 |
  22713.8 |
  22978.1 |
  23242.3 |
  23506.5 |
  23770.7 |
  24034.9 |
  24299.2 |
  24563.4 |
  24827.6 |
  25091.8 |
  25356.0 |
  25620.3 |
  25884.5 |########################################
  26148.7 |
  26412.9 |#
  26677.1 |
  26941.4 |
  27205.6 |##########
  (18 below, 5 above range)

v5_handletable (n=120, range 22260.4-26943.0 ns)
  22260.4 |
  22494.6 |
  22728.7 |
  22962.8 |
  23196.9 |
  23431.1 |
  23665.2 |
  23899.3 |
  24133.4 |
  24367.6 |
  24601.7 |
  24835.8 |
  25070.0 |
  25304.1 |
  25538.2 |
  25772.3 |########################################
  26006.5 |####
  26240.6 |#
  26474.7 |
  26708.9 |
  (18 below, 14 above range)

```

## Diagnostics

- **v0_blob**: bridge=2391.5% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2394.4% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2392.6% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2393.7% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2394.4% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2392.8% of algo (FFI overhead may distort results)
