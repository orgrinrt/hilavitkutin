# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.9% (fastest 15.68 ms to slowest 15.81 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (137.73 us) is smaller than the fastest variant's own run-to-run std-dev (241.56 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (riv_rcm)

The baseline riv_rcm is the fastest (15.68 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (riv_rcm) is the fastest** at 15675704.2 ns median
- Spread: 1.01x (fastest 15675704.2 ns, slowest 15813431.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | ordering theories on a grid: RCM vs spectral vs greedy vs row-major |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345679 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 15765597ns | 15684969ns | 15520643ns | 15654429ns | 16054825ns | base |
| riv_rowm | 15825780ns | 15723319ns | 15510206ns | 15679600ns | 16202838ns | +0.38% |
| riv_scr | 15878644ns | 15788890ns | 15511501ns | 15740350ns | 16269656ns | +0.72% |
| riv_snake | 15867841ns | 15821558ns | 15533611ns | 15745631ns | 16218271ns | +0.65% |
| riv_spec | 15797620ns | 15691196ns | 15590828ns | 15692667ns | 16058447ns | +0.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 15756477ns | 15510828ns | 16046122ns | base | 0.000 |
| riv_rowm | 15816270ns | 15500699ns | 16193271ns | +0.38% | 0.000 |
| riv_scr | 15870157ns | 15503250ns | 16261915ns | +0.72% | 0.000 |
| riv_snake | 15859183ns | 15524912ns | 16210462ns | +0.65% | 0.000 |
| riv_spec | 15788631ns | 15580726ns | 16049721ns | +0.20% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 98.9% |
| riv_rowm | 0.000 | 98.6% |
| riv_scr | 0.000 | 98.2% |
| riv_snake | 0.000 | 98.0% |
| riv_spec | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 15765597ns | 15765597ns | base |
| riv_rowm | 15825780ns | 15825780ns | +0.38% |
| riv_scr | 15878644ns | 15878644ns | +0.72% |
| riv_snake | 15867841ns | 15867841ns | +0.65% |
| riv_spec | 15797620ns | 15797620ns | +0.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 15675704ns | base | --- | [15612952, 15852285] | --- | --- | --- | --- |
| riv_rowm | 15715169ns | no significant difference | [-52598, +78565]ns | [15571204, 15801150] | no | 0.9614 | 0.4807 | 0 |
| riv_scr | 15778760ns | no significant difference | [-27754, +309721]ns | [15626244, 16044985] | no | 0.3850 | 0.0963 | 0 |
| riv_snake | 15813431ns | no significant difference | [-108398, +254488]ns | [15624794, 15982556] | no | 1.0000 | 0.8145 | 0 |
| riv_spec | 15682160ns | no significant difference | [-90815, +135104]ns | [15634585, 15875673] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 15540233ns | -0.1% | +1.8% | +0.7% | +0.7% |
| 1 | 2 | 15908812ns | -1.4% | -1.3% | -0.6% | -1.4% |
| 1 | 3 | 15730338ns | -0.2% | -1.0% | +0.0% | +0.7% |
| 2 | 1 | 16116225ns | -1.4% | +0.1% | -1.7% | -2.1% |
| 2 | 2 | 15638804ns | +0.9% | +1.1% | +0.8% | -0.3% |
| 2 | 3 | 15610038ns | +1.1% | +0.1% | +3.9% | +3.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.305 | moderate- |
| riv_rowm | 0.230 | moderate+ |
| riv_scr | -0.258 | moderate- |
| riv_snake | -0.120 | ok |
| riv_spec | -0.329 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 2/6
- **riv_scr**: won 2/6, lost 3/6
- **riv_snake**: won 2/6, lost 3/6
- **riv_spec**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 268300613.4ns | 15756476.6ns | 1702.8% | HIGH |
| riv_rowm | 269423097.9ns | 15816270.1ns | 1703.5% | HIGH |
| riv_scr | 268929135.2ns | 15870156.7ns | 1694.6% | HIGH |
| riv_snake | 268262960.6ns | 15859182.9ns | 1691.5% | HIGH |
| riv_spec | 268141437.3ns | 15788630.6ns | 1698.3% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 15510827.8-16046121.5 ns)
  15510827.8 |####################
  15537592.5 |####################
  15564357.2 |
  15591121.9 |########################################
  15617886.5 |########################################
  15644651.2 |####################
  15671415.9 |########################################
  15698180.6 |
  15724945.3 |####################
  15751710.0 |
  15778474.7 |
  15805239.3 |
  15832004.0 |########################################
  15858768.7 |
  15885533.4 |####################
  15912298.1 |
  15939062.8 |
  15965827.5 |
  15992592.1 |
  16019356.8 |
  (2 below, 3 above range)

riv_rowm (n=18, range 15500698.6-16193270.8 ns)
  15500698.6 |####################
  15535327.2 |##########
  15569955.8 |##########
  15604584.4 |
  15639213.0 |
  15673841.7 |##############################
  15708470.3 |##########
  15743098.9 |
  15777727.5 |########################################
  15812356.1 |##########
  15846984.7 |
  15881613.3 |##########
  15916241.9 |##########
  15950870.6 |
  15985499.2 |
  16020127.8 |
  16054756.4 |
  16089385.0 |
  16124013.6 |
  16158642.2 |
  (2 below, 1 above range)

riv_scr (n=18, range 15503250.0-16261914.6 ns)
  15503250.0 |########################################
  15541183.2 |####################
  15579116.5 |
  15617049.7 |########################################
  15654982.9 |
  15692916.1 |########################################
  15730849.4 |####################
  15768782.6 |
  15806715.8 |########################################
  15844649.1 |
  15882582.3 |
  15920515.5 |####################
  15958448.8 |
  15996382.0 |####################
  16034315.2 |
  16072248.4 |####################
  16110181.7 |####################
  16148114.9 |####################
  16186048.1 |
  16223981.4 |
  (1 below, 2 above range)

riv_snake (n=18, range 15524912.5-16210462.5 ns)
  15524912.5 |
  15559190.0 |########################################
  15593467.5 |####################
  15627745.0 |####################
  15662022.5 |
  15696300.0 |
  15730577.5 |########################################
  15764855.0 |
  15799132.5 |########################################
  15833410.0 |########################################
  15867687.5 |
  15901965.0 |####################
  15936242.5 |
  15970520.0 |
  16004797.5 |
  16039075.0 |####################
  16073352.5 |
  16107630.0 |
  16141907.5 |
  16176185.0 |
  (2 below, 4 above range)

riv_spec (n=18, range 15580726.4-16049720.9 ns)
  15580726.4 |##########
  15604176.1 |##########
  15627625.8 |########################################
  15651075.5 |##########
  15674525.3 |##########
  15697975.0 |
  15721424.7 |
  15744874.4 |
  15768324.2 |##########
  15791773.9 |
  15815223.6 |
  15838673.3 |####################
  15862123.1 |
  15885572.8 |##########
  15909022.5 |##########
  15932472.2 |
  15955922.0 |
  15979371.7 |
  16002821.4 |
  16026271.1 |
  (2 below, 3 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1711.2% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1707.4% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1700.2% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1695.7% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1709.6% of algo (FFI overhead may distort results)
