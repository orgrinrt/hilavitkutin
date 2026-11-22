# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### riv_snake is fastest but the noisiest (CV 15.8%)

riv_snake wins on median (208.76 us) yet has the highest variance (CV 15.8%), while riv_rowm is the steadiest (CV 7.1%, 215.62 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (riv_snake, riv_spec) are a dead heat (<1%)

riv_snake (208.76 us) and riv_spec (209.90 us) differ by 0.54%, inside the noise, even though the wider field spreads 3.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### riv_snake shows alternating (throttle bounce) (autocorr -0.54)

riv_snake's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6.86 us) is smaller than the fastest variant's own run-to-run std-dev (32.96 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_snake vs stability leader riv_rowm (+3% speed for 2.2x steadier)

riv_snake is fastest (208.76 us, CV 15.8%); riv_rowm gives up 3.3% median for 2.2x lower variance (CV 7.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: riv_snake** at 208764.6 ns median (-2.7% vs baseline)
- Spread: 1.03x (fastest 208764.6 ns, slowest 215625.0 ns)

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
| riv_rcm | 222626ns | 218219ns | 201950ns | 215222ns | 244071ns | base |
| riv_rowm | 217551ns | 219660ns | 196353ns | 213351ns | 234449ns | -2.28% |
| riv_scr | 219622ns | 215623ns | 196585ns | 210742ns | 244461ns | -1.35% |
| riv_snake | 221038ns | 212908ns | 199274ns | 210506ns | 247719ns | -0.71% |
| riv_spec | 220191ns | 214052ns | 197226ns | 211235ns | 245106ns | -1.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 218573ns | 198444ns | 239542ns | base | 0.000 |
| riv_rowm | 213442ns | 193144ns | 229809ns | -2.35% | 0.000 |
| riv_scr | 215634ns | 193139ns | 239794ns | -1.34% | 0.000 |
| riv_snake | 216939ns | 195832ns | 243283ns | -0.75% | 0.000 |
| riv_spec | 216011ns | 193903ns | 240178ns | -1.17% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_scr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 90.0% |
| riv_rowm | 0.000 | 89.6% |
| riv_scr | 0.000 | 91.2% |
| riv_snake | 0.000 | 92.5% |
| riv_spec | 0.000 | 92.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 222626ns | 222626ns | base |
| riv_rowm | 217551ns | 217551ns | -2.28% |
| riv_scr | 219622ns | 219622ns | -1.35% |
| riv_snake | 221038ns | 221038ns | -0.71% |
| riv_spec | 220191ns | 220191ns | -1.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 214640ns | base | --- | [206858, 221196] | --- | --- | --- | --- |
| riv_rowm | 215625ns | no significant difference | [-9869, +5425]ns | [199842, 221283] | no | 0.8145 | 0.8145 | 0 |
| riv_scr | 211825ns | no significant difference | [-13602, +2033]ns | [199798, 218373] | no | 0.4758 | 0.2379 | 0 |
| riv_snake | 208765ns | no significant difference | [-17442, +8288]ns | [202898, 214519] | no | 0.8145 | 0.8145 | 0 |
| riv_spec | 209898ns | no significant difference | [-12835, +6638]ns | [202223, 220871] | no | 0.4758 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 213396ns | -6.7% | -4.5% | -4.0% | -3.3% |
| 1 | 2 | 201558ns | +7.8% | +1.7% | +0.5% | +19.1% |
| 1 | 3 | 233488ns | -10.3% | -8.4% | -7.9% | -4.3% |
| 2 | 1 | 211812ns | +4.6% | -8.1% | -4.0% | -1.0% |
| 2 | 2 | 215883ns | -0.9% | +1.0% | -3.0% | -2.7% |
| 2 | 3 | 216308ns | +2.2% | -1.3% | -3.4% | -3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.538 | HIGH- (thermal bounce) |
| riv_rowm | -0.276 | moderate- |
| riv_scr | -0.434 | moderate- |
| riv_snake | -0.544 | HIGH- (thermal bounce) |
| riv_spec | -0.040 | ok |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 3/6
- **riv_scr**: won 4/6, lost 2/6
- **riv_snake**: won 5/6, lost 1/6
- **riv_spec**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 4296815.7ns | 218573.4ns | 1965.8% | HIGH |
| riv_rowm | 4291980.8ns | 213442.1ns | 2010.8% | HIGH |
| riv_scr | 4295094.9ns | 215634.0ns | 1991.8% | HIGH |
| riv_snake | 4308985.2ns | 216938.9ns | 1986.3% | HIGH |
| riv_spec | 4307152.1ns | 216011.1ns | 1993.9% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 198444.4-239542.4 ns)
  198444.4 |#############
  200499.3 |#############
  202554.2 |#############
  204609.1 |#############
  206664.0 |#############
  208718.9 |#############
  210773.8 |#############
  212828.7 |#############
  214883.6 |########################################
  216938.5 |
  218993.4 |#############
  221048.3 |#############
  223103.2 |
  225158.1 |#############
  227213.0 |
  229267.9 |
  231322.8 |
  233377.7 |#############
  235432.6 |
  237487.5 |
  (1 below, 2 above range)

riv_rowm (n=18, range 193144.4-229809.0 ns)
  193144.4 |#############
  194977.7 |#############
  196810.9 |
  198644.1 |########################################
  200477.4 |
  202310.6 |
  204143.8 |
  205977.0 |#############
  207810.3 |#############
  209643.5 |
  211476.7 |
  213310.0 |#############
  215143.2 |
  216976.4 |##########################
  218809.7 |#############
  220642.9 |##########################
  222476.1 |
  224309.3 |
  226142.6 |#############
  227975.8 |##########################
  (1 below, 1 above range)

riv_scr (n=18, range 193138.9-239793.8 ns)
  193138.9 |########################################
  195471.6 |####################
  197804.4 |
  200137.1 |
  202469.9 |####################
  204802.6 |####################
  207135.4 |
  209468.1 |########################################
  211800.8 |########################################
  214133.6 |####################
  216466.3 |########################################
  218799.1 |
  221131.8 |####################
  223464.6 |
  225797.3 |
  228130.0 |
  230462.8 |
  232795.5 |
  235128.3 |
  237461.0 |
  (2 below, 3 above range)

riv_snake (n=18, range 195831.9-243283.3 ns)
  195831.9 |
  198204.5 |
  200577.1 |########################################
  202949.6 |##########################
  205322.2 |#############
  207694.8 |########################################
  210067.4 |#############
  212439.9 |##########################
  214812.5 |#############
  217185.1 |
  219557.6 |#############
  221930.2 |
  224302.8 |
  226675.3 |
  229047.9 |
  231420.5 |
  233793.1 |
  236165.6 |
  238538.2 |
  240910.8 |
  (2 below, 2 above range)

riv_spec (n=18, range 193902.8-240178.5 ns)
  193902.8 |########################################
  196216.6 |####################
  198530.4 |####################
  200844.2 |
  203157.9 |####################
  205471.7 |####################
  207785.5 |########################################
  210099.3 |########################################
  212413.1 |
  214726.9 |
  217040.6 |########################################
  219354.4 |
  221668.2 |####################
  223982.0 |
  226295.8 |
  228609.6 |
  230923.3 |
  233237.1 |
  235550.9 |####################
  237864.7 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1999.0% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1987.7% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=2025.4% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=2054.3% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=2045.4% of algo (FFI overhead may distort results)
