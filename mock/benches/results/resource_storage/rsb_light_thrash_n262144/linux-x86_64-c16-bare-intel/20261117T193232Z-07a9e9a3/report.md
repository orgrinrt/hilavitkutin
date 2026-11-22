# Resource storage layouts under light surrounding work

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### v2_decomposed dominates: 12% faster than the next best (v1_snapshot)

v2_decomposed (23.15 us) leads v1_snapshot (25.93 us) by 12%, a clear separation rather than a photo finish. CV 10.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### v2_decomposed is fastest but the noisiest (CV 10.7%)

v2_decomposed wins on median (23.15 us) yet has the highest variance (CV 10.7%), while v0_blob is the steadiest (CV 6.3%, 25.95 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### v1_snapshot's edge over baseline is significant but tiny (-14 ns, 0.05%)

v1_snapshot differs from baseline v0_blob by -14 ns (0.05%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v2_decomposed** at 23154.8 ns median (-10.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.12x (fastest 23154.8 ns, slowest 25956.4 ns)

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
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 25753ns | 26140ns | 23275ns | 26146ns | 27053ns | base |
| v1_snapshot | 25199ns | 26131ns | 21389ns | 25952ns | 26749ns | -2.15% |
| v2_decomposed | 24094ns | 23336ns | 21385ns | 23975ns | 27157ns | -6.45% |
| v3_shapebound | 25554ns | 26155ns | 21681ns | 26183ns | 27541ns | -0.77% |
| v4_erased | 25103ns | 26124ns | 21385ns | 25594ns | 27346ns | -2.53% |
| v5_handletable | 25342ns | 26141ns | 21532ns | 25869ns | 27569ns | -1.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 25562ns | 23105ns | 26853ns | base | 10.255 |
| v1_snapshot | 25005ns | 21232ns | 26539ns | -2.18% | 10.484 |
| v2_decomposed | 23912ns | 21226ns | 26951ns | -6.45% | 10.963 |
| v3_shapebound | 25359ns | 21509ns | 27340ns | -0.79% | 10.337 |
| v4_erased | 24910ns | 21218ns | 27143ns | -2.55% | 10.524 |
| v5_handletable | 25152ns | 21370ns | 27370ns | -1.60% | 10.422 |

## Performance model

- Peak throughput: **12.355 Gops/s** (v4_erased; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.103 | 81.8% |
| v1_snapshot | 10.111 | 81.8% |
| v2_decomposed | 11.321 | 91.6% |
| v3_shapebound | 10.099 | 81.7% |
| v4_erased | 10.111 | 81.8% |
| v5_handletable | 10.103 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 25753ns | 25753ns | base |
| v1_snapshot | 25199ns | 25199ns | -2.15% |
| v2_decomposed | 24094ns | 24094ns | -6.45% |
| v3_shapebound | 25554ns | 25554ns | -0.77% |
| v4_erased | 25103ns | 25103ns | -2.53% |
| v5_handletable | 25342ns | 25342ns | -1.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 25947ns | base | --- | [25941, 25956] | --- | --- | --- | --- |
| v1_snapshot | 25925ns | -13.8ns (-0.1%) | [-25, -5]ns | [25917, 25937] | YES (adj: no) | 0.0669 | 0.0134 | 0 |
| v2_decomposed | 23155ns | no significant difference | [-1300, +11]ns | [22274, 25956] | no | 0.6857 | 0.4114 | 0 |
| v3_shapebound | 25956ns | no significant difference | [-18, +18]ns | [25947, 25963] | no | 0.9273 | 0.9273 | 0 |
| v4_erased | 25928ns | no significant difference | [-75, +2]ns | [25921, 25935] | no | 0.3008 | 0.1203 | 0 |
| v5_handletable | 25948ns | no significant difference | [-20, +22]ns | [25938, 25956] | no | 0.9273 | 0.7843 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 21251ns | -0.1% | -0.1% | -0.2% | +22.1% | +22.2% |
| 1 | 2 | 25940ns | -0.0% | +0.2% | +0.1% | -18.2% | -1.9% |
| 1 | 3 | 27010ns | -4.0% | -17.5% | -3.9% | -4.0% | -4.0% |
| 1 | 4 | 25944ns | -0.0% | +0.0% | -0.0% | -0.1% | +0.1% |
| 1 | 5 | 25950ns | -0.0% | +0.1% | +0.0% | -0.0% | -0.1% |
| 2 | 1 | 25952ns | -0.1% | -18.0% | +5.0% | -0.1% | +5.6% |
| 2 | 2 | 25948ns | +0.3% | +5.7% | +0.1% | -18.1% | -18.2% |
| 2 | 3 | 25949ns | +0.4% | +0.2% | -18.0% | -18.2% | -14.0% |
| 2 | 4 | 21255ns | +17.8% | +2.5% | +22.1% | +22.1% | +0.3% |
| 2 | 5 | 25963ns | +3.7% | -18.2% | -17.6% | +4.9% | +0.2% |
| 3 | 1 | 25954ns | -18.1% | +0.0% | +0.1% | +4.9% | +5.3% |
| 3 | 2 | 25963ns | -0.1% | -13.1% | -0.1% | -0.1% | -0.1% |
| 3 | 3 | 25952ns | -0.1% | -14.2% | -0.1% | -18.2% | -17.2% |
| 3 | 4 | 26839ns | -20.8% | -20.7% | -3.3% | -3.3% | -3.3% |
| 3 | 5 | 25927ns | -0.0% | +0.2% | +5.7% | +0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.039 | ok |
| v1_snapshot | -0.267 | moderate- |
| v2_decomposed | -0.276 | moderate- |
| v3_shapebound | -0.111 | ok |
| v4_erased | 0.067 | ok |
| v5_handletable | 0.161 | ok |

**Consistency summary:**

- **v1_snapshot**: won 7/15, lost 4/15
- **v2_decomposed**: won 7/15, lost 6/15
- **v3_shapebound**: won 5/15, lost 5/15
- **v4_erased**: won 8/15, lost 4/15
- **v5_handletable**: won 6/15, lost 5/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 607689.7ns | 25562.2ns | 2377.3% | HIGH |
| v1_snapshot | 596559.3ns | 25005.2ns | 2385.7% | HIGH |
| v2_decomposed | 564057.5ns | 23912.3ns | 2358.9% | HIGH |
| v3_shapebound | 599261.6ns | 25359.1ns | 2363.1% | HIGH |
| v4_erased | 590862.8ns | 24910.2ns | 2372.0% | HIGH |
| v5_handletable | 593702.7ns | 25152.3ns | 2360.4% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 23104.7-26853.0 ns)
  23104.7 |
  23292.1 |
  23479.6 |
  23667.0 |
  23854.4 |
  24041.8 |
  24229.2 |
  24416.6 |
  24604.0 |
  24791.4 |
  24978.9 |
  25166.3 |
  25353.7 |
  25541.1 |
  25728.5 |###
  25915.9 |########################################
  26103.3 |#
  26290.7 |
  26478.2 |
  26665.6 |#
  (15 below, 13 above range)

v1_snapshot (n=120, range 21232.3-26539.1 ns)
  21232.3 |######
  21497.6 |
  21763.0 |
  22028.3 |
  22293.6 |
  22559.0 |
  22824.3 |
  23089.7 |
  23355.0 |
  23620.4 |
  23885.7 |
  24151.0 |
  24416.4 |
  24681.7 |
  24947.1 |
  25212.4 |
  25477.8 |
  25743.1 |########################################
  26008.4 |######
  26273.8 |#
  (12 below, 8 above range)

v2_decomposed (n=120, range 21226.0-26951.4 ns)
  21226.0 |##########################
  21512.3 |
  21798.5 |#
  22084.8 |############
  22371.1 |#
  22657.3 |
  22943.6 |##
  23229.9 |
  23516.1 |
  23802.4 |
  24088.7 |
  24374.9 |
  24661.2 |
  24947.5 |
  25233.8 |
  25520.0 |
  25806.3 |########################################
  26092.6 |##
  26378.8 |
  26665.1 |
  (12 below, 12 above range)

v3_shapebound (n=120, range 21508.8-27339.9 ns)
  21508.8 |
  21800.3 |
  22091.9 |#
  22383.5 |
  22675.0 |
  22966.6 |
  23258.1 |
  23549.7 |
  23841.2 |
  24132.8 |
  24424.3 |
  24715.9 |
  25007.5 |
  25299.0 |
  25590.6 |
  25882.1 |########################################
  26173.7 |
  26465.2 |
  26756.8 |##
  27048.3 |######
  (20 below, 11 above range)

v4_erased (n=120, range 21218.0-27143.1 ns)
  21218.0 |###########
  21514.2 |#
  21810.5 |
  22106.8 |#
  22403.0 |
  22699.3 |
  22995.5 |
  23291.8 |
  23588.0 |
  23884.3 |
  24180.6 |
  24476.8 |
  24773.1 |
  25069.3 |
  25365.6 |
  25661.8 |########################################
  25958.1 |#####
  26254.4 |#
  26550.6 |
  26846.9 |#
  (12 below, 17 above range)

v5_handletable (n=120, range 21370.2-27370.0 ns)
  21370.2 |#
  21670.2 |
  21970.2 |
  22270.2 |###
  22570.1 |
  22870.1 |
  23170.1 |
  23470.1 |
  23770.1 |
  24070.1 |
  24370.1 |
  24670.1 |
  24970.1 |
  25270.0 |
  25570.0 |
  25870.0 |########################################
  26170.0 |
  26470.0 |
  26770.0 |
  27070.0 |######
  (19 below, 12 above range)

```

## Diagnostics

- **v0_blob**: bridge=2393.5% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2395.4% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2209.4% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2392.6% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2395.0% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2393.4% of algo (FFI overhead may distort results)
