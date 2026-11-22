# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: riv_snake** at 20167139.6 ns median (-3.7% vs baseline)
- Spread: 1.05x (fastest 20167139.6 ns, slowest 21190270.8 ns)

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
| riv_rcm | 20580211ns | 20961535ns | 18360700ns | 20605141ns | 21652573ns | base |
| riv_rowm | 20430647ns | 20443860ns | 17250157ns | 20196494ns | 22372120ns | -0.73% |
| riv_scr | 20764772ns | 20488742ns | 19298808ns | 20395322ns | 22051928ns | +0.90% |
| riv_snake | 20136033ns | 20174738ns | 18742489ns | 19908039ns | 21174797ns | -2.16% |
| riv_spec | 20862728ns | 21197304ns | 18532692ns | 20639029ns | 22363293ns | +1.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 20573103ns | 18354699ns | 21645498ns | base | 0.000 |
| riv_rowm | 20423247ns | 17243667ns | 22364426ns | -0.73% | 0.000 |
| riv_scr | 20757697ns | 19292510ns | 22044190ns | +0.90% | 0.000 |
| riv_snake | 20128827ns | 18735910ns | 21167524ns | -2.16% | 0.000 |
| riv_spec | 20855798ns | 18526249ns | 22355803ns | +1.37% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 82.3% |
| riv_rowm | 0.000 | 84.4% |
| riv_scr | 0.000 | 84.2% |
| riv_snake | 0.000 | 85.5% |
| riv_spec | 0.000 | 81.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 20580211ns | 20580211ns | base |
| riv_rowm | 20430647ns | 20430647ns | -0.73% |
| riv_scr | 20764772ns | 20764772ns | +0.90% |
| riv_snake | 20136033ns | 20136033ns | -2.16% |
| riv_spec | 20862728ns | 20862728ns | +1.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 20952848ns | base | --- | [20125988, 21313717] | --- | --- | --- | --- |
| riv_rowm | 20436977ns | no significant difference | [-1284850, +687667]ns | [19811465, 21442335] | no | 1.0000 | 0.8145 | 0 |
| riv_scr | 20481292ns | no significant difference | [-818588, +1040404]ns | [20122927, 21501185] | no | 1.0000 | 1.0000 | 0 |
| riv_snake | 20167140ns | no significant difference | [-1903129, +486415]ns | [19499029, 20654625] | no | 0.9515 | 0.2379 | 0 |
| riv_spec | 21190271ns | no significant difference | [-630523, +1240560]ns | [19854512, 21936969] | no | 0.9614 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 21433725ns | -6.3% | -5.8% | -9.5% | +0.3% |
| 1 | 2 | 20820550ns | -1.2% | -1.5% | +1.4% | +3.4% |
| 1 | 3 | 21146654ns | +2.2% | +4.5% | -3.8% | +6.9% |
| 2 | 1 | 18988071ns | -15.1% | +14.0% | +5.3% | +3.1% |
| 2 | 2 | 20391488ns | +0.8% | -0.1% | -2.0% | -8.0% |
| 2 | 3 | 21085146ns | -6.0% | -2.1% | -3.5% | -3.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.076 | ok |
| riv_rowm | -0.432 | moderate- |
| riv_scr | 0.147 | ok |
| riv_snake | -0.388 | moderate- |
| riv_spec | 0.307 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 2/6
- **riv_scr**: won 3/6, lost 2/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 276196303.7ns | 20573102.8ns | 1342.5% | HIGH |
| riv_rowm | 276252665.7ns | 20423246.5ns | 1352.6% | HIGH |
| riv_scr | 276492821.1ns | 20757697.5ns | 1332.0% | HIGH |
| riv_snake | 276452351.9ns | 20128826.9ns | 1373.4% | HIGH |
| riv_spec | 276414185.6ns | 20855797.7ns | 1325.4% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 18354698.6-21645497.9 ns)
  18354698.6 |
  18519238.6 |
  18683778.5 |
  18848318.5 |#############
  19012858.5 |
  19177398.4 |
  19341938.4 |
  19506478.4 |
  19671018.3 |
  19835558.3 |##########################
  20000098.3 |
  20164638.2 |
  20329178.2 |##########################
  20493718.2 |#############
  20658258.1 |#############
  20822798.1 |
  20987338.1 |##########################
  21151878.0 |##########################
  21316418.0 |########################################
  21480958.0 |
  (2 below, 2 above range)

riv_rowm (n=18, range 17243666.7-22364426.4 ns)
  17243666.7 |
  17499704.7 |
  17755742.6 |
  18011780.6 |
  18267818.6 |
  18523856.6 |
  18779894.6 |
  19035932.6 |
  19291970.6 |########################################
  19548008.5 |
  19804046.5 |#############
  20060084.5 |##########################
  20316122.5 |########################################
  20572160.5 |
  20828198.5 |
  21084236.5 |##########################
  21340274.5 |
  21596312.4 |#############
  21852350.4 |##########################
  22108388.4 |#############
  (2 below, 1 above range)

riv_scr (n=18, range 19292509.7-22044189.6 ns)
  19292509.7 |
  19430093.7 |
  19567677.7 |
  19705261.7 |#############
  19842845.7 |
  19980429.7 |########################################
  20118013.7 |##########################
  20255597.7 |#############
  20393181.7 |##########################
  20530765.7 |#############
  20668349.6 |
  20805933.6 |
  20943517.6 |#############
  21081101.6 |
  21218685.6 |
  21356269.6 |#############
  21493853.6 |
  21631437.6 |#############
  21769021.6 |
  21906605.6 |#############
  (1 below, 3 above range)

riv_snake (n=18, range 18735909.7-21167523.6 ns)
  18735909.7 |#############
  18857490.4 |
  18979071.1 |#############
  19100651.8 |#############
  19222232.5 |
  19343813.2 |#############
  19465393.9 |
  19586974.6 |#############
  19708555.3 |
  19830136.0 |#############
  19951716.7 |##########################
  20073297.4 |
  20194878.1 |
  20316458.7 |########################################
  20438039.4 |#############
  20559620.1 |
  20681200.8 |
  20802781.5 |#############
  20924362.2 |
  21045942.9 |#############
  (1 below, 3 above range)

riv_spec (n=18, range 18526248.6-22355802.8 ns)
  18526248.6 |
  18717726.3 |####################
  18909204.0 |
  19100681.7 |
  19292159.4 |
  19483637.1 |####################
  19675114.8 |########################################
  19866592.6 |
  20058070.3 |
  20249548.0 |####################
  20441025.7 |####################
  20632503.4 |
  20823981.1 |####################
  21015458.8 |
  21206936.5 |
  21398414.2 |########################################
  21589891.9 |########################################
  21781369.6 |
  21972847.3 |
  22164325.1 |########################################
  (2 below, 3 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1317.8% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1350.9% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1350.4% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1370.0% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1302.2% of algo (FFI overhead may distort results)
