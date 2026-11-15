# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (18.87 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_snake at 18.47 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (396.21 us) is smaller than the fastest variant's own run-to-run std-dev (985.77 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_snake vs stability leader riv_scr (+0% speed for 3.5x steadier)

riv_snake is fastest (18.47 ms, CV 5.3%); riv_scr gives up 0.2% median for 3.5x lower variance (CV 1.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 2.1% of the fastest

All 5 variants sit between 18.47 ms and 18.87 ms - a 2.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_snake** at 18471685.7 ns median (-2.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.02x (fastest 18471685.7 ns, slowest 18867897.0 ns)

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
| Counter frequency | 2112802731 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 19945552ns | 18871927ns | 18457515ns | 18982469ns | 22134196ns | base |
| riv_rowm | 19242145ns | 18630398ns | 18194054ns | 18611903ns | 20711554ns | -3.53% |
| riv_scr | 18522617ns | 18505596ns | 18182982ns | 18472102ns | 18768208ns | -7.13% |
| riv_snake | 18909626ns | 18475636ns | 18302965ns | 18502953ns | 19822965ns | -5.19% |
| riv_spec | 19854801ns | 18706887ns | 18408488ns | 18751477ns | 22232945ns | -0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 19941631ns | 18453575ns | 22130270ns | base | 0.000 |
| riv_rowm | 19238053ns | 18189941ns | 20707331ns | -3.53% | 0.000 |
| riv_scr | 18518660ns | 18178861ns | 18764275ns | -7.14% | 0.000 |
| riv_snake | 18905571ns | 18299052ns | 19818780ns | -5.20% | 0.000 |
| riv_spec | 19850738ns | 18404561ns | 22228934ns | -0.46% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_scr; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 96.3% |
| riv_rowm | 0.000 | 97.6% |
| riv_scr | 0.000 | 98.3% |
| riv_snake | 0.000 | 98.4% |
| riv_spec | 0.000 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 19945552ns | 19945552ns | base |
| riv_rowm | 19242145ns | 19242145ns | -3.53% |
| riv_scr | 18522617ns | 18522617ns | -7.13% |
| riv_snake | 18909626ns | 18909626ns | -5.19% |
| riv_spec | 19854801ns | 19854801ns | -0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 18867897ns | base | --- | [18653873, 21143753] | --- | --- | --- | --- |
| riv_rowm | 18626169ns | -326281.1ns (-1.7%) | [-1058838, -1710]ns | [18452958, 20080581] | YES (adj: no) | 0.1283 | 0.0963 | 0 |
| riv_scr | 18501703ns | -490307.7ns (-2.6%) | [-2346975, -261814]ns | [18400575, 18598671] | YES | 0.0052 | 0.0013 | 0 |
| riv_snake | 18471686ns | no significant difference | [-2117402, +3315]ns | [18429038, 18763808] | no | 0.1283 | 0.0963 | 0 |
| riv_spec | 18702697ns | no significant difference | [-1992357, +1671068]ns | [18628047, 21457248] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 18380464ns | +10.3% | +0.1% | +8.5% | +20.9% |
| 1 | 2 | 19014140ns | -2.7% | -2.2% | -3.2% | -1.7% |
| 1 | 3 | 18721654ns | +6.2% | -1.4% | -1.3% | +20.1% |
| 2 | 1 | 18632082ns | -2.7% | -2.4% | -1.8% | +1.2% |
| 2 | 2 | 22612490ns | -18.6% | -17.9% | -18.4% | -18.6% |
| 2 | 3 | 21549699ns | -5.9% | -13.4% | -12.9% | -13.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.301 | moderate+ |
| riv_rowm | -0.376 | moderate- |
| riv_scr | -0.112 | ok |
| riv_snake | -0.049 | ok |
| riv_spec | -0.285 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 2/6
- **riv_scr**: won 5/6, lost 0/6
- **riv_snake**: won 5/6, lost 1/6
- **riv_spec**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 92443717.5ns | 19941631.3ns | 463.6% | HIGH |
| riv_rowm | 92221258.7ns | 19238053.5ns | 479.4% | HIGH |
| riv_scr | 92007729.7ns | 18518660.4ns | 496.8% | HIGH |
| riv_snake | 92142234.6ns | 18905570.5ns | 487.4% | HIGH |
| riv_spec | 92471488.8ns | 19850738.4ns | 465.8% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 18453574.5-22130270.2 ns)
  18453574.5 |##############################
  18637409.3 |########################################
  18821244.1 |
  19005078.9 |##########
  19188913.7 |
  19372748.4 |
  19556583.2 |
  19740418.0 |####################
  19924252.8 |
  20108087.6 |
  20291922.3 |
  20475757.1 |
  20659591.9 |##########
  20843426.7 |
  21027261.5 |
  21211096.2 |
  21394931.0 |##########
  21578765.8 |
  21762600.6 |
  21946435.4 |
  (2 below, 4 above range)

riv_rowm (n=18, range 18189941.3-20707331.3 ns)
  18189941.3 |
  18315810.8 |##############################
  18441680.3 |########################################
  18567549.8 |
  18693419.3 |####################
  18819288.8 |
  18945158.3 |
  19071027.8 |
  19196897.3 |##########
  19322766.8 |
  19448636.3 |
  19574505.8 |
  19700375.3 |
  19826244.8 |##########
  19952114.3 |
  20077983.8 |
  20203853.3 |####################
  20329722.8 |
  20455592.3 |
  20581461.8 |
  (2 below, 3 above range)

riv_scr (n=18, range 18178861.0-18764274.8 ns)
  18178861.0 |#############
  18208131.7 |#############
  18237402.4 |
  18266673.1 |
  18295943.8 |
  18325214.5 |
  18354485.1 |#############
  18383755.8 |##########################
  18413026.5 |
  18442297.2 |########################################
  18471567.9 |
  18500838.6 |
  18530109.3 |##########################
  18559380.0 |#############
  18588650.7 |########################################
  18617921.4 |#############
  18647192.1 |#############
  18676462.7 |
  18705733.4 |
  18735004.1 |
  (1 below, 1 above range)

riv_snake (n=18, range 18299052.0-19818780.2 ns)
  18299052.0 |##########
  18375038.4 |##############################
  18451024.8 |########################################
  18527011.2 |
  18602997.6 |##########
  18678984.0 |##########
  18754970.4 |##############################
  18830956.8 |
  18906943.2 |
  18982929.7 |
  19058916.1 |
  19134902.5 |
  19210888.9 |
  19286875.3 |
  19362861.7 |
  19438848.1 |
  19514834.5 |
  19590820.9 |
  19666807.3 |
  19742793.8 |
  (2 below, 3 above range)

riv_spec (n=18, range 18404560.7-22228934.3 ns)
  18404560.7 |########################
  18595779.3 |########################################
  18786998.0 |########
  18978216.7 |
  19169435.4 |
  19360654.1 |########
  19551872.8 |
  19743091.4 |
  19934310.1 |
  20125528.8 |
  20316747.5 |
  20507966.2 |
  20699184.8 |
  20890403.5 |
  21081622.2 |
  21272840.9 |########
  21464059.6 |
  21655278.3 |
  21846496.9 |
  22037715.6 |########
  (2 below, 4 above range)

```

## Diagnostics

- **riv_rcm**: bridge=488.2% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=494.2% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=497.6% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=498.5% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=492.6% of algo (FFI overhead may distort results)
