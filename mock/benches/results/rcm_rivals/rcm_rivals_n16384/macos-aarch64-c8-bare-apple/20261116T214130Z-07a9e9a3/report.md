# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.41 ms) is smaller than the fastest variant's own run-to-run std-dev (8.97 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_rowm vs stability leader riv_scr (+4% speed for 1.4x steadier)

riv_rowm is fastest (66.22 ms, CV 13.5%); riv_scr gives up 3.6% median for 1.4x lower variance (CV 9.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: riv_rowm** at 66220145.9 ns median (-1.1% vs baseline)
- Spread: 1.04x (fastest 66220145.9 ns, slowest 68627295.8 ns)

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
| riv_rcm | 72116490ns | 66987056ns | 62555136ns | 66219338ns | 85742895ns | base |
| riv_rowm | 69672487ns | 66232379ns | 62227532ns | 65361235ns | 79861843ns | -3.39% |
| riv_scr | 70310735ns | 68643379ns | 63130743ns | 67400621ns | 78265900ns | -2.50% |
| riv_snake | 71655700ns | 67112919ns | 62871111ns | 66423734ns | 83895944ns | -0.64% |
| riv_spec | 72546416ns | 67504748ns | 62576138ns | 66446712ns | 86681111ns | +0.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 72105232ns | 62544290ns | 85730626ns | base | 0.000 |
| riv_rowm | 69661219ns | 62217075ns | 79849326ns | -3.39% | 0.000 |
| riv_scr | 70298542ns | 63119681ns | 78253419ns | -2.51% | 0.000 |
| riv_snake | 71643778ns | 62857331ns | 83882649ns | -0.64% | 0.000 |
| riv_spec | 72534848ns | 62565637ns | 86667298ns | +0.60% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 92.9% |
| riv_rowm | 0.000 | 94.0% |
| riv_scr | 0.000 | 90.7% |
| riv_snake | 0.000 | 92.7% |
| riv_spec | 0.000 | 92.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 72116490ns | 72116490ns | base |
| riv_rowm | 69672487ns | 69672487ns | -3.39% |
| riv_scr | 70310735ns | 70310735ns | -2.50% |
| riv_snake | 71655700ns | 71655700ns | -0.64% |
| riv_spec | 72546416ns | 72546416ns | +0.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 66976575ns | base | --- | [65052754, 76442190] | --- | --- | --- | --- |
| riv_rowm | 66220146ns | no significant difference | [-12194923, +2772567]ns | [63430640, 69352975] | no | 0.9515 | 0.2379 | 0 |
| riv_scr | 68627296ns | no significant difference | [-4778456, +4062125]ns | [65609181, 72319392] | no | 0.9614 | 0.4807 | 0 |
| riv_snake | 67102238ns | no significant difference | [-11900333, +7483148]ns | [65165412, 73960150] | no | 1.0000 | 1.0000 | 0 |
| riv_spec | 67494867ns | no significant difference | [-4412065, +3672046]ns | [64242177, 77143829] | no | 1.0000 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 67808150ns | -4.9% | -0.1% | -4.5% | +3.4% |
| 1 | 2 | 84682662ns | -25.3% | -24.9% | -20.6% | -8.9% |
| 1 | 3 | 68665767ns | +0.6% | +5.6% | -4.5% | +0.2% |
| 2 | 1 | 66117946ns | +4.2% | +6.0% | +2.6% | -4.9% |
| 2 | 2 | 63340508ns | -1.1% | +5.0% | +18.9% | +2.4% |
| 2 | 3 | 65731862ns | +1.2% | +6.5% | +1.1% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.030 | ok |
| riv_rowm | -0.181 | ok |
| riv_scr | -0.322 | moderate- |
| riv_snake | -0.105 | ok |
| riv_spec | 0.341 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 3/6
- **riv_scr**: won 1/6, lost 4/6
- **riv_snake**: won 3/6, lost 3/6
- **riv_spec**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 1078408016.2ns | 72105232.0ns | 1495.6% | HIGH |
| riv_rowm | 1080623723.4ns | 69661219.2ns | 1551.3% | HIGH |
| riv_scr | 1082956333.8ns | 70298542.1ns | 1540.5% | HIGH |
| riv_snake | 1081314872.9ns | 71643778.0ns | 1509.3% | HIGH |
| riv_spec | 1083453532.6ns | 72534848.2ns | 1493.7% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 62544290.3-85730625.7 ns)
  62544290.3 |########################################
  63703607.0 |
  64862923.8 |########################################
  66022240.6 |##########################
  67181557.4 |##########################
  68340874.1 |#############
  69500190.9 |
  70659507.7 |
  71818824.4 |
  72978141.2 |#############
  74137458.0 |
  75296774.8 |
  76456091.5 |
  77615408.3 |
  78774725.1 |#############
  79934041.9 |
  81093358.6 |
  82252675.4 |
  83411992.2 |
  84571308.9 |#############
  (1 below, 3 above range)

riv_rowm (n=18, range 62217075.0-79849326.4 ns)
  62217075.0 |##########################
  63098687.6 |########################################
  63980300.1 |#############
  64861912.7 |
  65743525.3 |##########################
  66625137.8 |
  67506750.4 |
  68388363.0 |########################################
  69269975.6 |#############
  70151588.1 |
  71033200.7 |
  71914813.3 |
  72796425.8 |
  73678038.4 |
  74559651.0 |
  75441263.5 |#############
  76322876.1 |
  77204488.7 |
  78086101.2 |
  78967713.8 |
  (2 below, 3 above range)

riv_scr (n=18, range 63119680.6-78253418.8 ns)
  63119680.6 |####################
  63876367.5 |####################
  64633054.4 |####################
  65389741.3 |
  66146428.2 |########################################
  66903115.1 |####################
  67659802.0 |####################
  68416488.9 |
  69173175.8 |####################
  69929862.7 |########################################
  70686549.7 |
  71443236.6 |
  72199923.5 |########################################
  72956610.4 |####################
  73713297.3 |
  74469984.2 |
  75226671.1 |
  75983358.0 |
  76740044.9 |
  77496731.8 |
  (2 below, 3 above range)

riv_snake (n=18, range 62857330.6-83882649.3 ns)
  62857330.6 |##########################
  63908596.5 |#############
  64959862.4 |#############
  66011128.4 |########################################
  67062394.3 |##########################
  68113660.3 |#############
  69164926.2 |
  70216192.1 |
  71267458.1 |
  72318724.0 |#############
  73369989.9 |
  74421255.9 |##########################
  75472521.8 |
  76523787.8 |
  77575053.7 |
  78626319.6 |#############
  79677585.6 |
  80728851.5 |
  81780117.4 |
  82831383.4 |
  (2 below, 2 above range)

riv_spec (n=18, range 62565637.5-86667297.9 ns)
  62565637.5 |####################
  63770720.5 |########################################
  64975803.5 |##########
  66180886.5 |##########
  67385969.6 |
  68591052.6 |##########
  69796135.6 |####################
  71001218.6 |
  72206301.7 |
  73411384.7 |
  74616467.7 |
  75821550.7 |
  77026633.7 |##########
  78231716.8 |##########
  79436799.8 |##########
  80641882.8 |
  81846965.8 |
  83052048.9 |
  84257131.9 |##########
  85462214.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1607.6% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1626.7% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1574.3% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1606.2% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1599.0% of algo (FFI overhead may distort results)
