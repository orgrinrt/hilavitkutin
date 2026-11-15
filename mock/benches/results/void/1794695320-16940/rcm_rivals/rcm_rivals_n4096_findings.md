# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### riv_spec is fastest but the noisiest (CV 25.8%)

riv_spec wins on median (15.87 ms) yet has the highest variance (CV 25.8%), while riv_scr is the steadiest (CV 1.1%, 15.91 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (119.32 us) is smaller than the fastest variant's own run-to-run std-dev (4.09 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_spec vs stability leader riv_scr (+0% speed for 23.2x steadier)

riv_spec is fastest (15.87 ms, CV 25.8%); riv_scr gives up 0.2% median for 23.2x lower variance (CV 1.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.8% of the fastest

All 5 variants sit between 15.87 ms and 15.99 ms - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_spec** at 15870537.5 ns median (-0.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 15870537.5 ns, slowest 15989858.3 ns)

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
| riv_rcm | 15965288ns | 15904723ns | 15740018ns | 15878890ns | 16207520ns | base |
| riv_rowm | 16117522ns | 15977233ns | 15685036ns | 15940125ns | 16599860ns | +0.95% |
| riv_scr | 15946217ns | 15917752ns | 15748012ns | 15883836ns | 16138890ns | -0.12% |
| riv_snake | 16182249ns | 15998823ns | 15844350ns | 15967358ns | 16673536ns | +1.36% |
| riv_spec | 16983873ns | 15878715ns | 15675832ns | 15889743ns | 19279090ns | +6.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 15956362ns | 15730804ns | 16198960ns | base | 0.000 |
| riv_rowm | 16108882ns | 15675571ns | 16591753ns | +0.96% | 0.000 |
| riv_scr | 15937515ns | 15740138ns | 16130074ns | -0.12% | 0.000 |
| riv_snake | 16173873ns | 15835919ns | 16665599ns | +1.36% | 0.000 |
| riv_spec | 16975198ns | 15667518ns | 19270501ns | +6.39% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 98.6% |
| riv_rowm | 0.000 | 98.1% |
| riv_scr | 0.000 | 98.5% |
| riv_snake | 0.000 | 98.0% |
| riv_spec | 0.000 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 15965288ns | 15965288ns | base |
| riv_rowm | 16117522ns | 16117522ns | +0.95% |
| riv_scr | 15946217ns | 15946217ns | -0.12% |
| riv_snake | 16182249ns | 16182249ns | +1.36% |
| riv_spec | 16983873ns | 16983873ns | +6.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 15895162ns | base | --- | [15819121, 16110369] | --- | --- | --- | --- |
| riv_rowm | 15967973ns | no significant difference | [-115144, +259975]ns | [15871212, 16111742] | no | 0.8145 | 0.8145 | 0 |
| riv_scr | 15909019ns | no significant difference | [-115092, +131540]ns | [15838871, 15963762] | no | 0.8145 | 0.8145 | 0 |
| riv_snake | 15989858ns | +119452.1ns (+0.8%) | [+18806, +298925]ns | [15880817, 16281019] | YES | 0.0302 | 0.0075 | 0 |
| riv_spec | 15870538ns | no significant difference | [-106352, +199388]ns | [15820556, 16124565] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 15879588ns | +0.6% | -0.3% | +0.3% | +1.1% |
| 1 | 2 | 15915725ns | +0.2% | -0.4% | +0.8% | +0.6% |
| 1 | 3 | 15900183ns | +0.1% | +0.1% | -0.1% | +4.9% |
| 2 | 1 | 16101221ns | -0.9% | +0.3% | +0.2% | +0.6% |
| 2 | 2 | 16245192ns | -1.4% | -2.0% | +3.4% | -3.2% |
| 2 | 3 | 15870392ns | +3.7% | -0.1% | +0.1% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.031 | ok |
| riv_rowm | 0.091 | ok |
| riv_scr | 0.073 | ok |
| riv_snake | -0.191 | ok |
| riv_spec | 0.116 | ok |

**Consistency summary:**

- **riv_rowm**: won 2/6, lost 3/6
- **riv_scr**: won 4/6, lost 2/6
- **riv_snake**: won 1/6, lost 4/6
- **riv_spec**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 268603572.5ns | 15956362.5ns | 1683.4% | HIGH |
| riv_rowm | 270685976.2ns | 16108882.4ns | 1680.4% | HIGH |
| riv_scr | 268486377.8ns | 15937514.8ns | 1684.6% | HIGH |
| riv_snake | 269669731.2ns | 16173872.9ns | 1667.3% | HIGH |
| riv_spec | 270417608.3ns | 16975197.9ns | 1593.0% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 15730804.2-16198960.4 ns)
  15730804.2 |####################
  15754212.0 |
  15777619.8 |########################################
  15801027.6 |########################################
  15824435.4 |
  15847843.2 |####################
  15871251.0 |########################################
  15894658.9 |########################################
  15918066.7 |####################
  15941474.5 |
  15964882.3 |
  15988290.1 |
  16011697.9 |
  16035105.7 |
  16058513.5 |
  16081921.4 |####################
  16105329.2 |####################
  16128737.0 |
  16152144.8 |
  16175552.6 |####################
  (1 below, 3 above range)

riv_rowm (n=18, range 15675570.9-16591752.8 ns)
  15675570.9 |
  15721380.0 |##########################
  15767189.1 |#############
  15812998.2 |#############
  15858807.2 |#############
  15904616.3 |#############
  15950425.4 |########################################
  15996234.5 |##########################
  16042043.6 |#############
  16087852.7 |
  16133661.8 |#############
  16179470.9 |
  16225280.0 |
  16271089.1 |
  16316898.2 |
  16362707.3 |
  16408516.4 |#############
  16454325.5 |#############
  16500134.6 |
  16545943.7 |
  (1 below, 2 above range)

riv_scr (n=18, range 15740137.5-16130074.3 ns)
  15740137.5 |
  15759634.3 |
  15779131.2 |
  15798628.0 |####################
  15818124.9 |##########
  15837621.7 |####################
  15857118.5 |##########
  15876615.4 |
  15896112.2 |##########
  15915609.1 |########################################
  15935105.9 |
  15954602.7 |
  15974099.6 |
  15993596.4 |##########
  16013093.3 |
  16032590.1 |
  16052087.0 |##########
  16071583.8 |
  16091080.6 |
  16110577.5 |
  (2 below, 3 above range)

riv_snake (n=18, range 15835919.5-16665598.6 ns)
  15835919.5 |##########################
  15877403.4 |##########################
  15918887.4 |########################################
  15960371.3 |
  16001855.3 |#############
  16043339.2 |#############
  16084823.2 |#############
  16126307.2 |#############
  16167791.1 |
  16209275.1 |
  16250759.0 |
  16292243.0 |
  16333726.9 |
  16375210.9 |
  16416694.9 |#############
  16458178.8 |
  16499662.8 |#############
  16541146.7 |#############
  16582630.7 |
  16624114.6 |
  (2 below, 2 above range)

riv_spec (n=18, range 15667518.0-19270500.7 ns)
  15667518.0 |##########################
  15847667.2 |########################################
  16027816.3 |##########################
  16207965.4 |
  16388114.6 |
  16568263.7 |#############
  16748412.8 |
  16928562.0 |
  17108711.1 |
  17288860.2 |
  17469009.4 |
  17649158.5 |
  17829307.6 |
  18009456.8 |
  18189605.9 |
  18369755.0 |
  18549904.2 |
  18730053.3 |
  18910202.4 |
  19090351.6 |
  (1 below, 1 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1689.1% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1681.3% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1687.6% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1679.6% of algo (FFI overhead may distort results)
- **riv_spec**: CV=24.1% (high variance, measurements may be unstable)
- **riv_spec**: bridge=1693.2% of algo (FFI overhead may distort results)
