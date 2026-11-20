# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (55 ns) is smaller than the fastest variant's own run-to-run std-dev (2.07 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (v0_blob)

The baseline v0_blob is the fastest (25.92 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader v0_blob vs stability leader v3_shapebound (+0% speed for 1.6x steadier)

v0_blob is fastest (25.92 us, CV 8.0%); v3_shapebound gives up 0.1% median for 1.6x lower variance (CV 4.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.2% of the fastest

All 6 variants sit between 25.92 us and 25.98 us - a 0.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v3_shapebound's edge over baseline is significant but tiny (32 ns, 0.12%)

v3_shapebound differs from baseline v0_blob by 32 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (v0_blob) is the fastest** at 25920.6 ns median
- 2 variants significantly slower than baseline
- Spread: 1.00x (fastest 25920.6 ns, slowest 25975.2 ns)

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
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 24807ns | 26012ns | 21318ns | 25455ns | 26351ns | base |
| v1_snapshot | 25514ns | 26037ns | 22902ns | 26033ns | 26570ns | +2.85% |
| v2_decomposed | 25301ns | 26069ns | 21632ns | 25910ns | 27141ns | +1.99% |
| v3_shapebound | 25823ns | 26049ns | 24282ns | 26051ns | 26678ns | +4.09% |
| v4_erased | 25499ns | 26038ns | 21931ns | 26041ns | 27441ns | +2.79% |
| v5_handletable | 24366ns | 26029ns | 21302ns | 24753ns | 26270ns | -1.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 24715ns | 21241ns | 26245ns | base | 10.607 |
| v1_snapshot | 25417ns | 22817ns | 26461ns | +2.84% | 10.314 |
| v2_decomposed | 25210ns | 21548ns | 27044ns | +2.00% | 10.398 |
| v3_shapebound | 25728ns | 24194ns | 26576ns | +4.10% | 10.189 |
| v4_erased | 25408ns | 21852ns | 27345ns | +2.80% | 10.318 |
| v5_handletable | 24277ns | 21226ns | 26177ns | -1.77% | 10.798 |

## Performance model

- Peak throughput: **12.350 Gops/s** (v5_handletable; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.113 | 81.9% |
| v1_snapshot | 10.106 | 81.8% |
| v2_decomposed | 10.092 | 81.7% |
| v3_shapebound | 10.100 | 81.8% |
| v4_erased | 10.104 | 81.8% |
| v5_handletable | 10.106 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 24807ns | 24807ns | base |
| v1_snapshot | 25514ns | 25514ns | +2.85% |
| v2_decomposed | 25301ns | 25301ns | +1.99% |
| v3_shapebound | 25823ns | 25823ns | +4.09% |
| v4_erased | 25499ns | 25499ns | +2.79% |
| v5_handletable | 24366ns | 24366ns | -1.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 25921ns | base | --- | [25904, 25939] | --- | --- | --- | --- |
| v1_snapshot | 25939ns | no significant difference | [-12, +35]ns | [25927, 25944] | no | 0.6537 | 0.5230 | 0 |
| v2_decomposed | 25975ns | +73.9ns (+0.3%) | [+47, +167]ns | [25964, 25989] | YES | 0.0022 | 0.0004 | 1 |
| v3_shapebound | 25955ns | +32.0ns (+0.1%) | [+8, +65]ns | [25946, 25965] | YES | 0.0111 | 0.0045 | 0 |
| v4_erased | 25945ns | no significant difference | [-5, +75]ns | [25931, 25955] | no | 0.2844 | 0.1706 | 0 |
| v5_handletable | 25940ns | no significant difference | [-35, +17]ns | [25911, 25954] | no | 0.7843 | 0.7843 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 25902ns | +0.2% | -16.1% | +0.1% | -8.3% | -17.9% |
| 1 | 2 | 25975ns | -0.1% | -15.6% | -0.1% | -0.1% | -18.2% |
| 1 | 3 | 22425ns | +15.7% | +15.9% | +15.8% | -5.5% | +15.8% |
| 1 | 4 | 25897ns | +0.1% | +0.5% | +0.2% | +0.3% | +0.3% |
| 1 | 5 | 25942ns | -0.0% | +4.9% | -0.0% | -18.2% | -18.0% |
| 2 | 1 | 21266ns | -0.1% | +0.9% | -0.0% | +22.1% | -0.1% |
| 2 | 2 | 26075ns | -0.5% | -0.4% | -0.3% | -0.6% | -0.4% |
| 2 | 3 | 23655ns | +9.7% | +10.1% | +9.9% | +9.6% | -10.4% |
| 2 | 4 | 21217ns | +22.2% | +22.3% | +22.6% | +22.4% | +22.5% |
| 2 | 5 | 25969ns | -0.1% | -3.2% | -0.1% | +6.7% | +0.0% |
| 3 | 1 | 25941ns | +0.1% | +0.0% | +0.1% | -0.1% | +0.1% |
| 3 | 2 | 25952ns | -0.1% | +0.1% | +5.2% | -0.0% | -0.0% |
| 3 | 3 | 25992ns | -0.1% | +4.7% | -0.1% | -0.2% | -0.2% |
| 3 | 4 | 25961ns | -0.1% | +0.3% | -0.1% | +4.9% | +0.1% |
| 3 | 5 | 21222ns | +0.0% | +22.5% | +22.3% | +22.3% | +3.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | -0.202 | moderate- |
| v1_snapshot | -0.084 | ok |
| v2_decomposed | 0.123 | ok |
| v3_shapebound | -0.035 | ok |
| v4_erased | -0.088 | ok |
| v5_handletable | 0.079 | ok |

**Consistency summary:**

- **v1_snapshot**: won 5/15, lost 5/15
- **v2_decomposed**: won 4/15, lost 9/15
- **v3_shapebound**: won 3/15, lost 8/15
- **v4_erased**: won 6/15, lost 7/15
- **v5_handletable**: won 6/15, lost 5/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 588519.6ns | 24715.0ns | 2381.2% | HIGH |
| v1_snapshot | 606617.8ns | 25417.1ns | 2386.7% | HIGH |
| v2_decomposed | 596532.8ns | 25210.3ns | 2366.2% | HIGH |
| v3_shapebound | 612830.8ns | 25728.1ns | 2382.0% | HIGH |
| v4_erased | 600942.2ns | 25407.6ns | 2365.2% | HIGH |
| v5_handletable | 578941.1ns | 24276.8ns | 2384.7% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 21240.5-26245.4 ns)
  21240.5 |########
  21490.8 |
  21741.0 |
  21991.3 |
  22241.5 |####
  22491.8 |
  22742.0 |
  22992.2 |
  23242.5 |
  23492.7 |
  23743.0 |
  23993.2 |
  24243.5 |
  24493.7 |
  24743.9 |
  24994.2 |
  25244.4 |
  25494.7 |
  25744.9 |########################################
  25995.2 |######
  (13 below, 4 above range)

v1_snapshot (n=120, range 22816.9-26461.4 ns)
  22816.9 |#
  22999.1 |
  23181.4 |
  23363.6 |
  23545.8 |
  23728.0 |
  23910.3 |
  24092.5 |
  24274.7 |
  24456.9 |
  24639.1 |
  24821.4 |
  25003.6 |
  25185.8 |
  25368.0 |
  25550.3 |
  25732.5 |###########
  25914.7 |########################################
  26096.9 |
  26279.2 |#
  (14 below, 10 above range)

v2_decomposed (n=120, range 21547.7-27044.1 ns)
  21547.7 |
  21822.5 |
  22097.3 |####
  22372.1 |
  22647.0 |
  22921.8 |
  23196.6 |
  23471.4 |
  23746.2 |
  24021.1 |
  24295.9 |#
  24570.7 |
  24845.5 |
  25120.3 |
  25395.1 |
  25670.0 |#####
  25944.8 |########################################
  26219.6 |
  26494.4 |
  26769.2 |#
  (17 below, 16 above range)

v3_shapebound (n=120, range 24193.6-26576.4 ns)
  24193.6 |
  24312.7 |
  24431.9 |
  24551.0 |
  24670.2 |
  24789.3 |
  24908.4 |
  25027.6 |
  25146.7 |
  25265.9 |
  25385.0 |
  25504.1 |
  25623.3 |
  25742.4 |
  25861.6 |########################################
  25980.7 |###########
  26099.8 |
  26219.0 |
  26338.1 |#
  26457.3 |
  (9 below, 10 above range)

v4_erased (n=120, range 21852.3-27345.1 ns)
  21852.3 |
  22127.0 |#
  22401.6 |
  22676.3 |
  22950.9 |
  23225.5 |
  23500.2 |
  23774.8 |
  24049.4 |
  24324.1 |
  24598.7 |
  24873.3 |#
  25148.0 |
  25422.6 |
  25697.3 |########################################
  25971.9 |##########
  26246.5 |
  26521.2 |#
  26795.8 |
  27070.4 |######
  (18 below, 10 above range)

v5_handletable (n=120, range 21225.6-26176.5 ns)
  21225.6 |###############
  21473.2 |#
  21720.7 |
  21968.3 |
  22215.8 |###
  22463.4 |
  22710.9 |
  22958.5 |
  23206.0 |
  23453.5 |
  23701.1 |
  23948.6 |
  24196.2 |
  24443.7 |
  24691.3 |
  24938.8 |
  25186.4 |
  25433.9 |
  25681.4 |###
  25929.0 |########################################
  (13 below, 5 above range)

```

## Diagnostics

- **v0_blob**: bridge=2395.4% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2394.0% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2390.6% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2392.8% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2393.3% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2394.0% of algo (FFI overhead may distort results)
