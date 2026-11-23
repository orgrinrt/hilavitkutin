# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (28 ns) is smaller than the fastest variant's own run-to-run std-dev (1.75 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader v4_erased vs stability leader v0_blob (+0% speed for 1.1x steadier)

v4_erased is fastest (25.93 us, CV 6.7%); v0_blob gives up 0.1% median for 1.1x lower variance (CV 6.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.1% of the fastest

All 6 variants sit between 25.93 us and 25.96 us - a 0.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### v3_shapebound's edge over baseline is significant but tiny (-22 ns, 0.09%)

v3_shapebound differs from baseline v0_blob by -22 ns (0.09%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v4_erased** at 25933.0 ns median (-0.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 25933.0 ns, slowest 25961.3 ns)

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
| Counter frequency | 2112759259 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 25658ns | 26043ns | 23176ns | 26044ns | 26980ns | base |
| v1_snapshot | 25432ns | 26039ns | 22262ns | 26037ns | 26786ns | -0.88% |
| v2_decomposed | 24863ns | 26056ns | 21297ns | 25374ns | 26895ns | -3.10% |
| v3_shapebound | 25004ns | 26036ns | 21304ns | 25756ns | 26446ns | -2.55% |
| v4_erased | 25558ns | 26030ns | 22747ns | 26028ns | 26959ns | -0.39% |
| v5_handletable | 25290ns | 26036ns | 21529ns | 25982ns | 26976ns | -1.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 25568ns | 23099ns | 26885ns | base | 10.253 |
| v1_snapshot | 25339ns | 22180ns | 26687ns | -0.90% | 10.346 |
| v2_decomposed | 24773ns | 21220ns | 26800ns | -3.11% | 10.582 |
| v3_shapebound | 24915ns | 21225ns | 26352ns | -2.55% | 10.522 |
| v4_erased | 25463ns | 22669ns | 26851ns | -0.41% | 10.295 |
| v5_handletable | 25204ns | 21453ns | 26884ns | -1.42% | 10.401 |

## Performance model

- Peak throughput: **12.353 Gops/s** (v2_decomposed; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 10.102 | 81.8% |
| v1_snapshot | 10.105 | 81.8% |
| v2_decomposed | 10.097 | 81.7% |
| v3_shapebound | 10.104 | 81.8% |
| v4_erased | 10.108 | 81.8% |
| v5_handletable | 10.101 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 25658ns | 25658ns | base |
| v1_snapshot | 25432ns | 25432ns | -0.88% |
| v2_decomposed | 24863ns | 24863ns | -3.10% |
| v3_shapebound | 25004ns | 25004ns | -2.55% |
| v4_erased | 25558ns | 25558ns | -0.39% |
| v5_handletable | 25290ns | 25290ns | -1.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 25950ns | base | --- | [25940, 25963] | --- | --- | --- | --- |
| v1_snapshot | 25943ns | no significant difference | [-33, +12]ns | [25935, 25951] | no | 0.6483 | 0.6483 | 0 |
| v2_decomposed | 25961ns | no significant difference | [-12, +26]ns | [25944, 25981] | no | 0.5143 | 0.3153 | 0 |
| v3_shapebound | 25944ns | -22.3ns (-0.1%) | [-58, -0]ns | [25934, 25952] | YES (adj: no) | 0.2060 | 0.0548 | 0 |
| v4_erased | 25933ns | no significant difference | [-40, +2]ns | [25923, 25940] | no | 0.2060 | 0.0824 | 0 |
| v5_handletable | 25952ns | no significant difference | [-42, +22]ns | [25936, 25958] | no | 0.5143 | 0.4114 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 27238ns | -4.8% | -21.9% | -4.7% | -4.8% | -4.8% |
| 1 | 2 | 25956ns | -0.0% | +0.3% | -0.0% | -0.0% | -18.1% |
| 1 | 3 | 25929ns | +0.0% | -18.2% | +1.7% | +0.0% | +0.1% |
| 1 | 4 | 25916ns | +0.0% | +1.3% | +0.0% | +0.1% | +0.2% |
| 1 | 5 | 25964ns | -0.1% | -0.1% | +0.0% | -0.1% | -0.1% |
| 2 | 1 | 25984ns | -9.1% | -0.1% | -18.3% | +5.7% | -18.2% |
| 2 | 2 | 25955ns | -0.1% | +0.2% | +0.0% | -18.2% | +0.0% |
| 2 | 3 | 22479ns | -5.7% | +15.8% | +15.5% | +15.4% | +15.5% |
| 2 | 4 | 21233ns | +22.2% | -0.0% | +22.3% | +22.0% | +22.2% |
| 2 | 5 | 25942ns | +5.1% | +5.7% | -18.3% | +0.0% | +0.1% |
| 3 | 1 | 25938ns | +3.3% | +0.2% | -0.0% | +0.0% | +0.1% |
| 3 | 2 | 25938ns | +0.0% | -18.0% | -18.0% | -0.0% | +5.0% |
| 3 | 3 | 25966ns | -18.4% | +0.1% | -18.1% | -0.2% | -0.1% |
| 3 | 4 | 25967ns | -0.0% | -1.4% | -0.1% | -18.1% | -17.8% |
| 3 | 5 | 25938ns | +0.1% | +0.1% | -0.0% | +0.1% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.379 | moderate+ |
| v1_snapshot | -0.119 | ok |
| v2_decomposed | -0.353 | moderate- |
| v3_shapebound | -0.033 | ok |
| v4_erased | -0.275 | moderate- |
| v5_handletable | -0.210 | moderate- |

**Consistency summary:**

- **v1_snapshot**: won 4/15, lost 3/15
- **v2_decomposed**: won 5/15, lost 8/15
- **v3_shapebound**: won 5/15, lost 3/15
- **v4_erased**: won 4/15, lost 3/15
- **v5_handletable**: won 5/15, lost 7/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 607521.7ns | 25568.1ns | 2376.1% | HIGH |
| v1_snapshot | 602629.2ns | 25338.9ns | 2378.3% | HIGH |
| v2_decomposed | 588872.8ns | 24772.6ns | 2377.1% | HIGH |
| v3_shapebound | 594799.2ns | 24914.9ns | 2387.3% | HIGH |
| v4_erased | 605321.5ns | 25463.2ns | 2377.2% | HIGH |
| v5_handletable | 597826.4ns | 25204.3ns | 2371.9% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 23098.6-26885.2 ns)
  23098.6 |
  23287.9 |
  23477.3 |
  23666.6 |
  23855.9 |
  24045.2 |
  24234.6 |
  24423.9 |
  24613.2 |
  24802.6 |
  24991.9 |
  25181.2 |
  25370.5 |
  25559.9 |
  25749.2 |#####################
  25938.5 |########################################
  26127.9 |#
  26317.2 |
  26506.5 |
  26695.8 |
  (14 below, 14 above range)

v1_snapshot (n=120, range 22180.2-26687.2 ns)
  22180.2 |
  22405.5 |
  22630.9 |
  22856.2 |
  23081.6 |
  23306.9 |
  23532.3 |
  23757.6 |
  23983.0 |
  24208.3 |
  24433.7 |
  24659.0 |
  24884.4 |
  25109.7 |
  25335.1 |
  25560.4 |
  25785.8 |########################################
  26011.1 |#
  26236.5 |
  26461.9 |
  (18 below, 12 above range)

v2_decomposed (n=120, range 21220.3-26799.5 ns)
  21220.3 |################
  21499.2 |#
  21778.2 |
  22057.2 |
  22336.1 |
  22615.1 |
  22894.0 |
  23173.0 |
  23452.0 |
  23730.9 |#
  24009.9 |
  24288.9 |
  24567.8 |
  24846.8 |
  25125.7 |
  25404.7 |
  25683.7 |######################
  25962.6 |########################################
  26241.6 |###
  26520.6 |
  (13 below, 12 above range)

v3_shapebound (n=120, range 21224.9-26351.6 ns)
  21224.9 |#####
  21481.2 |
  21737.6 |
  21993.9 |
  22250.2 |
  22506.6 |
  22762.9 |
  23019.3 |
  23275.6 |
  23531.9 |
  23788.3 |
  24044.6 |
  24300.9 |
  24557.3 |
  24813.6 |
  25070.0 |
  25326.3 |
  25582.6 |
  25839.0 |########################################
  26095.3 |##
  (15 below, 6 above range)

v4_erased (n=120, range 22668.5-26850.8 ns)
  22668.5 |
  22877.6 |
  23086.8 |
  23295.9 |
  23505.0 |
  23714.1 |
  23923.2 |
  24132.3 |
  24341.4 |
  24550.5 |
  24759.6 |
  24968.8 |
  25177.9 |
  25387.0 |
  25596.1 |
  25805.2 |########################################
  26014.3 |#
  26223.4 |
  26432.5 |
  26641.6 |
  (17 below, 12 above range)

v5_handletable (n=120, range 21452.6-26884.1 ns)
  21452.6 |
  21724.2 |
  21995.8 |#
  22267.3 |
  22538.9 |#
  22810.5 |
  23082.1 |
  23353.6 |
  23625.2 |
  23896.8 |
  24168.4 |
  24439.9 |
  24711.5 |
  24983.1 |
  25254.7 |
  25526.2 |
  25797.8 |########################################
  26069.4 |#
  26341.0 |
  26612.5 |
  (20 below, 14 above range)

```

## Diagnostics

- **v0_blob**: bridge=2393.2% of algo (FFI overhead may distort results)
- **v1_snapshot**: bridge=2393.8% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2391.9% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2393.6% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2394.6% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2393.1% of algo (FFI overhead may distort results)
