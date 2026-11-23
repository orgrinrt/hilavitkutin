# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (24 ns) is smaller than the fastest variant's own run-to-run std-dev (2.13 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader v2_decomposed vs stability leader v0_blob (+0% speed for 1.3x steadier)

v2_decomposed is fastest (25.93 us, CV 8.2%); v0_blob gives up 0.1% median for 1.3x lower variance (CV 6.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.1% of the fastest

All 6 variants sit between 25.93 us and 25.95 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v2_decomposed's edge over baseline is significant but tiny (-21 ns, 0.08%)

v2_decomposed differs from baseline v0_blob by -21 ns (0.08%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v2_decomposed** at 25925.3 ns median (-0.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.00x (fastest 25925.3 ns, slowest 25948.8 ns)

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
| Counter frequency | 2112746324 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 29759ns | 30201ns | 26932ns | 30206ns | 31244ns | base |
| v1_snapshot | 29769ns | 30218ns | 25548ns | 30499ns | 31802ns | +0.04% |
| v2_decomposed | 28442ns | 30189ns | 24723ns | 28985ns | 30531ns | -4.43% |
| v3_shapebound | 29922ns | 30215ns | 27009ns | 30356ns | 31533ns | +0.55% |
| v4_erased | 28873ns | 30186ns | 24914ns | 29271ns | 31639ns | -2.98% |
| v5_handletable | 29045ns | 30217ns | 24727ns | 29873ns | 30881ns | -2.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 25594ns | 23169ns | 26962ns | base | 10.242 |
| v1_snapshot | 25653ns | 22063ns | 27512ns | +0.23% | 10.219 |
| v2_decomposed | 24454ns | 21237ns | 26262ns | -4.45% | 10.720 |
| v3_shapebound | 25745ns | 23194ns | 27266ns | +0.59% | 10.182 |
| v4_erased | 24868ns | 21424ns | 27367ns | -2.84% | 10.542 |
| v5_handletable | 24965ns | 21235ns | 26590ns | -2.46% | 10.501 |

## Performance model

- Peak throughput: **12.345 Gops/s** (v5_handletable; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.103 | 81.8% |
| v1_snapshot | 10.105 | 81.9% |
| v2_decomposed | 10.112 | 81.9% |
| v3_shapebound | 10.104 | 81.9% |
| v4_erased | 10.111 | 81.9% |
| v5_handletable | 10.102 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 29759ns | 29759ns | base |
| v1_snapshot | 29769ns | 29769ns | +0.04% |
| v2_decomposed | 28442ns | 28442ns | -4.43% |
| v3_shapebound | 29922ns | 29922ns | +0.55% |
| v4_erased | 28873ns | 28873ns | -2.98% |
| v5_handletable | 29045ns | 29045ns | -2.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 25946ns | base | --- | [25937, 25953] | --- | --- | --- | --- |
| v1_snapshot | 25942ns | no significant difference | [-11, +80]ns | [25929, 25976] | no | 0.5825 | 0.5825 | 1 |
| v2_decomposed | 25925ns | -21.4ns (-0.1%) | [-1032, -6]ns | [25915, 25938] | YES | 0.0080 | 0.0032 | 1 |
| v3_shapebound | 25943ns | no significant difference | [-7, +20]ns | [25938, 25960] | no | 0.2133 | 0.1706 | 0 |
| v4_erased | 25926ns | -28.0ns (-0.1%) | [-813, -9]ns | [25921, 25933] | YES | 0.0080 | 0.0024 | 0 |
| v5_handletable | 25949ns | -10.3ns (-0.0%) | [-23, -0]ns | [25938, 25957] | YES (adj: no) | 0.0589 | 0.0353 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 25919ns | +0.0% | -17.9% | +0.1% | -15.9% | -18.1% |
| 1 | 2 | 26398ns | -19.6% | -1.7% | -1.6% | -1.8% | -0.1% |
| 1 | 3 | 25957ns | +8.2% | -0.1% | +5.1% | -18.2% | -18.2% |
| 1 | 4 | 25946ns | -13.2% | +0.8% | +2.2% | -0.1% | +0.1% |
| 1 | 5 | 25956ns | +5.0% | -0.0% | -0.1% | -18.3% | +0.1% |
| 2 | 1 | 25951ns | -14.0% | -15.0% | -0.0% | -0.1% | +0.0% |
| 2 | 2 | 25964ns | +0.0% | -18.2% | +4.9% | +4.9% | -0.1% |
| 2 | 3 | 27289ns | -0.3% | -18.2% | -4.9% | -5.0% | -4.9% |
| 2 | 4 | 25934ns | -0.0% | +0.1% | -0.1% | +5.4% | -16.7% |
| 2 | 5 | 27365ns | -5.2% | -5.3% | -5.2% | -18.5% | -5.2% |
| 3 | 1 | 21226ns | +22.1% | +19.9% | +3.2% | +22.1% | +0.1% |
| 3 | 2 | 25916ns | +5.0% | +0.2% | -18.2% | -13.7% | +0.2% |
| 3 | 3 | 25948ns | +2.5% | +0.0% | +2.5% | -0.0% | -0.1% |
| 3 | 4 | 22309ns | +22.2% | -4.8% | +16.3% | +16.2% | +16.4% |
| 3 | 5 | 25925ns | +0.1% | -0.0% | +0.0% | +6.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.222 | moderate- |
| v1_snapshot | -0.448 | moderate- |
| v2_decomposed | 0.131 | ok |
| v3_shapebound | 0.365 | moderate+ |
| v4_erased | -0.284 | moderate- |
| v5_handletable | -0.339 | moderate- |

**Consistency summary:**

- **v1_snapshot**: won 5/15, lost 7/15
- **v2_decomposed**: won 7/15, lost 3/15
- **v3_shapebound**: won 4/15, lost 6/15
- **v4_erased**: won 7/15, lost 5/15
- **v5_handletable**: won 6/15, lost 2/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 607275.0ns | 25593.9ns | 2372.7% | HIGH |
| v1_snapshot | 598994.0ns | 25653.0ns | 2335.0% | HIGH |
| v2_decomposed | 580541.3ns | 24453.9ns | 2374.0% | HIGH |
| v3_shapebound | 608203.5ns | 25745.0ns | 2362.4% | HIGH |
| v4_erased | 583609.5ns | 24867.8ns | 2346.8% | HIGH |
| v5_handletable | 593958.9ns | 24964.7ns | 2379.2% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 23168.6-26962.4 ns)
  23168.6 |
  23358.3 |
  23548.0 |
  23737.7 |
  23927.4 |
  24117.1 |
  24306.7 |
  24496.4 |
  24686.1 |
  24875.8 |
  25065.5 |
  25255.2 |
  25444.9 |
  25634.6 |
  25824.3 |########################################
  26014.0 |##
  26203.7 |
  26393.4 |
  26583.0 |
  26772.7 |
  (15 below, 16 above range)

v1_snapshot (n=120, range 22062.6-27512.4 ns)
  22062.6 |#####
  22335.1 |######
  22607.6 |
  22880.1 |
  23152.6 |
  23425.1 |
  23697.5 |
  23970.0 |
  24242.5 |
  24515.0 |
  24787.5 |
  25060.0 |
  25332.5 |
  25605.0 |
  25877.5 |########################################
  26150.0 |###
  26422.5 |
  26695.0 |
  26967.4 |##############
  27239.9 |#########
  (8 below, 8 above range)

v2_decomposed (n=120, range 21237.1-26262.3 ns)
  21237.1 |########
  21488.4 |#
  21739.6 |
  21990.9 |
  22242.2 |#####
  22493.4 |
  22744.7 |
  22995.9 |
  23247.2 |
  23498.5 |
  23749.7 |
  24001.0 |#
  24252.2 |
  24503.5 |
  24754.7 |
  25006.0 |
  25257.3 |
  25508.5 |
  25759.8 |########################################
  26011.0 |##
  (14 below, 6 above range)

v3_shapebound (n=120, range 23194.3-27265.9 ns)
  23194.3 |
  23397.9 |
  23601.5 |
  23805.1 |
  24008.6 |
  24212.2 |
  24415.8 |
  24619.4 |
  24823.0 |
  25026.5 |
  25230.1 |
  25433.7 |
  25637.3 |
  25840.9 |########################################
  26044.5 |#
  26248.0 |#
  26451.6 |#
  26655.2 |
  26858.8 |
  27062.4 |###########
  (14 below, 10 above range)

v4_erased (n=120, range 21423.9-27367.3 ns)
  21423.9 |
  21721.1 |
  22018.3 |####
  22315.5 |########
  22612.6 |##
  22909.8 |
  23207.0 |
  23504.1 |
  23801.3 |
  24098.5 |
  24395.6 |
  24692.8 |
  24990.0 |
  25287.1 |
  25584.3 |
  25881.5 |########################################
  26178.7 |
  26475.8 |
  26773.0 |
  27070.2 |############
  (19 below, 8 above range)

v5_handletable (n=120, range 21234.9-26590.1 ns)
  21234.9 |#####
  21502.6 |
  21770.4 |
  22038.2 |
  22305.9 |
  22573.7 |
  22841.4 |
  23109.2 |
  23377.0 |#
  23644.7 |
  23912.5 |
  24180.2 |
  24448.0 |
  24715.7 |
  24983.5 |
  25251.3 |
  25519.0 |
  25786.8 |########################################
  26054.5 |
  26322.3 |#
  (16 below, 10 above range)

```

## Diagnostics

- **v0_blob**: bridge=2393.7% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2393.8% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2395.2% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2393.7% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2395.2% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2393.5% of algo (FFI overhead may distort results)
