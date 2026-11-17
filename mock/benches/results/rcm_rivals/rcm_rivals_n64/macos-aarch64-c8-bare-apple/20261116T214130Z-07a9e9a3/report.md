# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.8% (fastest 179.76 us to slowest 181.12 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### riv_rcm shows alternating (throttle bounce) (autocorr -0.59)

riv_rcm's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.36 us) is smaller than the fastest variant's own run-to-run std-dev (6.91 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_spec vs stability leader riv_rowm (+1% speed for 3.4x steadier)

riv_spec is fastest (179.76 us, CV 3.8%); riv_rowm gives up 0.8% median for 3.4x lower variance (CV 1.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: riv_spec** at 179760.4 ns median (-0.4% vs baseline)
- Spread: 1.01x (fastest 179760.4 ns, slowest 181122.9 ns)

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
| riv_rcm | 185307ns | 184323ns | 182864ns | 184125ns | 188301ns | base |
| riv_rowm | 185407ns | 184852ns | 182710ns | 184444ns | 188201ns | +0.05% |
| riv_scr | 188023ns | 185308ns | 183524ns | 185049ns | 194735ns | +1.47% |
| riv_snake | 186893ns | 184544ns | 183144ns | 184171ns | 192849ns | +0.86% |
| riv_spec | 186558ns | 183681ns | 183251ns | 183646ns | 192578ns | +0.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 181438ns | 179143ns | 184397ns | base | 0.000 |
| riv_rowm | 181629ns | 179329ns | 184165ns | +0.11% | 0.000 |
| riv_scr | 183999ns | 179806ns | 190796ns | +1.41% | 0.000 |
| riv_snake | 182842ns | 179164ns | 188861ns | +0.77% | 0.000 |
| riv_spec | 182582ns | 179135ns | 188633ns | +0.63% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 99.3% |
| riv_rowm | 0.000 | 98.9% |
| riv_scr | 0.000 | 99.0% |
| riv_snake | 0.000 | 99.4% |
| riv_spec | 0.000 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 185307ns | 185307ns | base |
| riv_rowm | 185407ns | 185407ns | +0.05% |
| riv_scr | 188023ns | 188023ns | +1.47% |
| riv_snake | 186893ns | 186893ns | +0.86% |
| riv_spec | 186558ns | 186558ns | +0.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 180448ns | base | --- | [179717, 181471] | --- | --- | --- | --- |
| riv_rowm | 181123ns | no significant difference | [-1015, +2896]ns | [180083, 182817] | no | 0.4807 | 0.4807 | 0 |
| riv_scr | 180971ns | no significant difference | [-108, +1860]ns | [180698, 181938] | no | 0.3850 | 0.0963 | 0 |
| riv_snake | 180206ns | no significant difference | [-1504, +1862]ns | [179681, 181338] | no | 0.4807 | 0.4807 | 0 |
| riv_spec | 179760ns | no significant difference | [-1354, +571]ns | [179390, 181031] | no | 0.4807 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 181267ns | -0.9% | +0.6% | +1.4% | -0.8% |
| 1 | 2 | 179475ns | +1.0% | +0.8% | +0.1% | +0.8% |
| 1 | 3 | 181712ns | -0.0% | -0.5% | -0.3% | -0.3% |
| 2 | 1 | 180792ns | -0.7% | +3.1% | -0.9% | -0.6% |
| 2 | 2 | 179588ns | +0.8% | +0.3% | +1.8% | -0.2% |
| 2 | 3 | 181633ns | -0.8% | -0.5% | -1.1% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.591 | HIGH- (thermal bounce) |
| riv_rowm | -0.426 | moderate- |
| riv_scr | -0.372 | moderate- |
| riv_snake | -0.529 | HIGH- (thermal bounce) |
| riv_spec | 0.371 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 2/6
- **riv_scr**: won 2/6, lost 4/6
- **riv_snake**: won 3/6, lost 3/6
- **riv_spec**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 4194345.1ns | 181438.2ns | 2311.7% | HIGH |
| riv_rowm | 4185246.5ns | 181628.9ns | 2304.3% | HIGH |
| riv_scr | 4187706.9ns | 183999.3ns | 2275.9% | HIGH |
| riv_snake | 4185658.1ns | 182842.1ns | 2289.2% | HIGH |
| riv_spec | 4188389.6ns | 182582.4ns | 2294.0% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 179143.1-184397.2 ns)
  179143.1 |##########################
  179405.8 |##########################
  179668.5 |##########################
  179931.2 |
  180193.9 |##########################
  180456.6 |#############
  180719.3 |#############
  180982.0 |
  181244.7 |##########################
  181507.4 |########################################
  181770.1 |
  182032.8 |
  182295.5 |
  182558.3 |
  182821.0 |
  183083.7 |
  183346.4 |
  183609.1 |#############
  183871.8 |
  184134.5 |
  (1 below, 1 above range)

riv_rowm (n=18, range 179329.2-184164.6 ns)
  179329.2 |
  179570.9 |########################################
  179812.7 |####################
  180054.5 |####################
  180296.2 |####################
  180538.0 |
  180779.8 |####################
  181021.6 |########################################
  181263.3 |####################
  181505.1 |####################
  181746.9 |
  181988.6 |####################
  182230.4 |
  182472.2 |
  182713.9 |
  182955.7 |
  183197.5 |
  183439.3 |####################
  183681.0 |####################
  183922.8 |
  (2 below, 3 above range)

riv_scr (n=18, range 179805.6-190795.8 ns)
  179805.6 |################
  180355.1 |########################################
  180904.6 |################################
  181454.1 |########
  182003.6 |########
  182553.1 |
  183102.6 |
  183652.2 |
  184201.7 |
  184751.2 |
  185300.7 |
  185850.2 |
  186399.7 |########
  186949.2 |
  187498.8 |
  188048.3 |########
  188597.8 |
  189147.3 |########
  189696.8 |
  190246.3 |
  (1 below, 1 above range)

riv_snake (n=18, range 179163.9-188861.1 ns)
  179163.9 |########################################
  179648.8 |########################################
  180133.6 |########################################
  180618.5 |#############
  181103.3 |##########################
  181588.2 |
  182073.1 |
  182557.9 |#############
  183042.8 |#############
  183527.6 |#############
  184012.5 |
  184497.4 |
  184982.2 |
  185467.1 |
  185951.9 |
  186436.8 |
  186921.7 |
  187406.5 |
  187891.4 |
  188376.2 |
  (2 below, 1 above range)

riv_spec (n=18, range 179134.7-188632.6 ns)
  179134.7 |########################################
  179609.6 |########################################
  180084.5 |########
  180559.4 |########
  181034.3 |########
  181509.2 |
  181984.1 |
  182459.0 |
  182933.9 |
  183408.8 |
  183883.7 |
  184358.6 |########
  184833.5 |
  185308.4 |
  185783.3 |
  186258.1 |
  186733.0 |########
  187207.9 |
  187682.8 |
  188157.7 |
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=2316.7% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=2307.4% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=2310.0% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=2318.9% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=2325.0% of algo (FFI overhead may distort results)
