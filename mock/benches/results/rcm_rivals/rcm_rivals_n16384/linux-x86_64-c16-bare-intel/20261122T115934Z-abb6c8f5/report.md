# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.7% (fastest 137.24 ms to slowest 138.16 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (921.36 us) is smaller than the fastest variant's own run-to-run std-dev (2.56 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: riv_rowm** at 137238881.0 ns median (-0.1% vs baseline)
- Spread: 1.01x (fastest 137238881.0 ns, slowest 138160245.9 ns)

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
| Counter frequency | 2112741408 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 137509187ns | 137371864ns | 136973801ns | 137274956ns | 138128226ns | base |
| riv_rowm | 137957099ns | 137243418ns | 136767274ns | 137159514ns | 139748391ns | +0.33% |
| riv_scr | 138294147ns | 137390864ns | 136694905ns | 137360328ns | 140494496ns | +0.57% |
| riv_snake | 138208994ns | 138165238ns | 136337028ns | 137641160ns | 139996728ns | +0.51% |
| riv_spec | 137502186ns | 137428380ns | 136414993ns | 137370243ns | 138243696ns | -0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 137504554ns | 136969253ns | 138123498ns | base | 0.000 |
| riv_rowm | 137952426ns | 136762484ns | 139743643ns | +0.33% | 0.000 |
| riv_scr | 138286083ns | 136670389ns | 140489698ns | +0.57% | 0.000 |
| riv_snake | 138204212ns | 136332230ns | 139991899ns | +0.51% | 0.000 |
| riv_spec | 137497429ns | 136410166ns | 138238914ns | -0.01% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_snake; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 99.2% |
| riv_rowm | 0.000 | 99.3% |
| riv_scr | 0.000 | 99.2% |
| riv_snake | 0.000 | 98.7% |
| riv_spec | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 137509187ns | 137509187ns | base |
| riv_rowm | 137957099ns | 137957099ns | +0.33% |
| riv_scr | 138294147ns | 138294147ns | +0.57% |
| riv_snake | 138208994ns | 138208994ns | +0.51% |
| riv_spec | 137502186ns | 137502186ns | -0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 137367282ns | base | --- | [137151856, 137642803] | --- | --- | --- | --- |
| riv_rowm | 137238881ns | no significant difference | [-604725, +123239]ns | [137053393, 137338148] | no | 0.4758 | 0.2379 | 0 |
| riv_scr | 137386115ns | no significant difference | [-346056, +1333737]ns | [137073777, 138888187] | no | 0.8145 | 0.8145 | 0 |
| riv_snake | 138160246ns | no significant difference | [-204382, +1153523]ns | [137121498, 138759830] | no | 0.4758 | 0.2379 | 0 |
| riv_spec | 137423790ns | no significant difference | [-337605, +265828]ns | [137245348, 137750692] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 137570984ns | -0.2% | -0.7% | -0.8% | -0.2% |
| 1 | 2 | 137967534ns | -0.5% | +0.9% | +0.3% | -0.1% |
| 1 | 3 | 137128127ns | -0.1% | +1.0% | +0.9% | +1.0% |
| 2 | 1 | 136964500ns | +1.5% | +0.1% | +0.1% | -0.6% |
| 2 | 2 | 137417043ns | -0.4% | -0.2% | +1.4% | +0.2% |
| 2 | 3 | 137197329ns | +0.0% | +1.0% | -0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.071 | ok |
| riv_rowm | -0.486 | moderate- |
| riv_scr | -0.235 | moderate- |
| riv_snake | -0.497 | moderate- |
| riv_spec | -0.443 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 1/6
- **riv_scr**: won 2/6, lost 4/6
- **riv_snake**: won 1/6, lost 4/6
- **riv_spec**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 738316311.4ns | 137504553.7ns | 536.9% | HIGH |
| riv_rowm | 738298658.7ns | 137952426.3ns | 535.2% | HIGH |
| riv_scr | 737900394.7ns | 138286082.5ns | 533.6% | HIGH |
| riv_snake | 737609252.2ns | 138204212.0ns | 533.7% | HIGH |
| riv_spec | 737008514.9ns | 137497429.1ns | 536.0% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 136969253.2-138123498.0 ns)
  136969253.2 |########################################
  137026965.4 |
  137084677.7 |####################
  137142389.9 |########################################
  137200102.2 |####################
  137257814.4 |
  137315526.6 |####################
  137373238.9 |####################
  137430951.1 |####################
  137488663.3 |####################
  137546375.6 |####################
  137604087.8 |
  137661800.1 |####################
  137719512.3 |
  137777224.5 |####################
  137834936.8 |
  137892649.0 |
  137950361.3 |####################
  138008073.5 |####################
  138065785.7 |
  (2 below, 1 above range)

riv_rowm (n=18, range 136762483.5-139743643.2 ns)
  136762483.5 |###########
  136911541.5 |#################
  137060599.5 |#####
  137209657.5 |########################################
  137358715.5 |#####
  137507773.4 |
  137656831.4 |
  137805889.4 |
  137954947.4 |
  138104005.4 |
  138253063.3 |
  138402121.3 |
  138551179.3 |
  138700237.3 |
  138849295.3 |#####
  138998353.2 |#####
  139147411.2 |
  139296469.2 |
  139445527.2 |
  139594585.2 |
  (1 below, 1 above range)

riv_scr (n=18, range 136670388.5-140489697.5 ns)
  136670388.5 |################
  136861354.0 |
  137052319.4 |########################################
  137243284.9 |
  137434250.3 |################
  137625215.8 |
  137816181.2 |
  138007146.7 |
  138198112.1 |
  138389077.6 |################
  138580043.0 |
  138771008.5 |
  138961973.9 |
  139152939.4 |########
  139343904.8 |
  139534870.3 |
  139725835.7 |
  139916801.2 |
  140107766.6 |
  140298732.1 |
  (2 below, 4 above range)

riv_snake (n=18, range 136332230.1-139991899.1 ns)
  136332230.1 |##########################
  136515213.5 |
  136698197.0 |
  136881180.4 |#############
  137064163.9 |########################################
  137247147.3 |#############
  137430130.8 |
  137613114.2 |
  137796097.7 |
  137979081.1 |#############
  138162064.6 |########################################
  138345048.0 |#############
  138528031.5 |
  138711014.9 |
  138893998.4 |
  139076981.8 |#############
  139259965.3 |#############
  139442948.7 |#############
  139625932.2 |
  139808915.6 |
  (1 below, 2 above range)

riv_spec (n=18, range 136410166.4-138238913.7 ns)
  136410166.4 |
  136501603.8 |
  136593041.2 |
  136684478.5 |
  136775915.9 |
  136867353.2 |
  136958790.6 |
  137050228.0 |##########
  137141665.3 |##########
  137233102.7 |########################################
  137324540.0 |##########
  137415977.4 |
  137507414.8 |####################
  137598852.1 |####################
  137690289.5 |
  137781726.8 |####################
  137873164.2 |
  137964601.6 |
  138056038.9 |
  138147476.3 |
  (2 below, 3 above range)

```

## Diagnostics

- **riv_rcm**: bridge=537.5% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=537.8% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=537.0% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=534.3% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=536.2% of algo (FFI overhead may distort results)
