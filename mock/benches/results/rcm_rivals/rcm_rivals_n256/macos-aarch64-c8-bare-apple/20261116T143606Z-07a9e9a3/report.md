# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: riv_spec** at 1025985.4 ns median (-4.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.08x (fastest 1025985.4 ns, slowest 1112914.6 ns)

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
| riv_rcm | 1077925ns | 1074960ns | 982400ns | 1051430ns | 1165431ns | base |
| riv_rowm | 1089806ns | 1085767ns | 935411ns | 1063526ns | 1206423ns | +1.10% |
| riv_scr | 1157968ns | 1118150ns | 945879ns | 1080494ns | 1380222ns | +7.43% |
| riv_snake | 1063550ns | 1049377ns | 933254ns | 1037394ns | 1167932ns | -1.33% |
| riv_spec | 1015714ns | 1031410ns | 926936ns | 1011971ns | 1065717ns | -5.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 1072488ns | 977362ns | 1159528ns | base | 0.000 |
| riv_rowm | 1084252ns | 930126ns | 1200886ns | +1.10% | 0.000 |
| riv_scr | 1151994ns | 940818ns | 1374114ns | +7.41% | 0.000 |
| riv_snake | 1058167ns | 927478ns | 1162381ns | -1.34% | 0.000 |
| riv_spec | 1010141ns | 921662ns | 1060168ns | -5.81% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 86.2% |
| riv_rowm | 0.000 | 85.3% |
| riv_scr | 0.000 | 82.8% |
| riv_snake | 0.000 | 88.2% |
| riv_spec | 0.000 | 89.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 1077925ns | 1077925ns | base |
| riv_rowm | 1089806ns | 1089806ns | +1.10% |
| riv_scr | 1157968ns | 1157968ns | +7.43% |
| riv_snake | 1063550ns | 1063550ns | -1.33% |
| riv_spec | 1015714ns | 1015714ns | -5.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 1069585ns | base | --- | [1007179, 1109877] | --- | --- | --- | --- |
| riv_rowm | 1080850ns | no significant difference | [-97733, +101538]ns | [1023088, 1135008] | no | 0.8145 | 0.8145 | 0 |
| riv_scr | 1112915ns | no significant difference | [-77817, +204221]ns | [1008856, 1217083] | no | 0.4758 | 0.2379 | 0 |
| riv_snake | 1044483ns | no significant difference | [-84281, +47567]ns | [1019010, 1072727] | no | 0.6409 | 0.4807 | 0 |
| riv_spec | 1025985ns | -64464.6ns (-6.0%) | [-88281, -38479]ns | [979677, 1041408] | YES | 0.0302 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 1033267ns | +18.5% | +0.9% | -0.9% | -6.0% |
| 1 | 2 | 1012696ns | +5.9% | +12.4% | +20.7% | +1.6% |
| 1 | 3 | 1155467ns | -7.9% | +18.5% | -11.7% | -8.5% |
| 2 | 1 | 1104533ns | +0.7% | +5.1% | -5.6% | -6.2% |
| 2 | 2 | 1060683ns | +3.1% | +3.4% | +1.3% | -4.5% |
| 2 | 3 | 1052762ns | -3.7% | -3.8% | -3.3% | -4.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.000 | ok |
| riv_rowm | -0.103 | ok |
| riv_scr | 0.121 | ok |
| riv_snake | -0.430 | moderate- |
| riv_spec | 0.135 | ok |

**Consistency summary:**

- **riv_rowm**: won 2/6, lost 4/6
- **riv_scr**: won 1/6, lost 5/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 17204654.9ns | 1072488.4ns | 1604.2% | HIGH |
| riv_rowm | 17181529.2ns | 1084252.3ns | 1584.6% | HIGH |
| riv_scr | 17212269.9ns | 1151994.2ns | 1494.1% | HIGH |
| riv_snake | 17237532.2ns | 1058167.1ns | 1629.0% | HIGH |
| riv_spec | 17179572.7ns | 1010141.2ns | 1700.7% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 977362.5-1159528.5 ns)
  977362.5 |
  986470.8 |########################################
  995579.1 |########################################
  1004687.4 |####################
  1013795.7 |
  1022904.0 |
  1032012.3 |####################
  1041120.6 |
  1050228.9 |####################
  1059337.2 |####################
  1068445.5 |
  1077553.8 |####################
  1086662.1 |########################################
  1095770.4 |####################
  1104878.7 |
  1113987.0 |####################
  1123095.3 |####################
  1132203.6 |
  1141311.9 |
  1150420.2 |####################
  (1 below, 2 above range)

riv_rowm (n=18, range 930126.4-1200886.1 ns)
  930126.4 |########################################
  943664.4 |
  957202.4 |
  970740.4 |
  984278.3 |####################
  997816.3 |
  1011354.3 |########################################
  1024892.3 |
  1038430.3 |
  1051968.3 |########################################
  1065506.2 |####################
  1079044.2 |####################
  1092582.2 |####################
  1106120.2 |########################################
  1119658.2 |
  1133196.2 |
  1146734.2 |####################
  1160272.1 |
  1173810.1 |####################
  1187348.1 |####################
  (1 below, 2 above range)

riv_scr (n=18, range 940818.0-1374113.9 ns)
  940818.0 |
  962482.8 |
  984147.6 |########################################
  1005812.4 |#############
  1027477.2 |#############
  1049142.0 |
  1070806.8 |#############
  1092471.6 |#############
  1114136.4 |#############
  1135801.2 |#############
  1157466.0 |#############
  1179130.8 |#############
  1200795.6 |
  1222460.3 |#############
  1244125.1 |
  1265789.9 |
  1287454.7 |
  1309119.5 |#############
  1330784.3 |
  1352449.1 |#############
  (2 below, 2 above range)

riv_snake (n=18, range 927477.8-1162380.6 ns)
  927477.8 |
  939222.9 |
  950968.0 |
  962713.2 |#############
  974458.3 |
  986203.5 |
  997948.6 |#############
  1009693.7 |##########################
  1021438.9 |#############
  1033184.0 |##########################
  1044929.2 |########################################
  1056674.3 |
  1068419.4 |##########################
  1080164.6 |#############
  1091909.7 |
  1103654.9 |
  1115400.0 |
  1127145.1 |
  1138890.3 |
  1150635.4 |#############
  (2 below, 2 above range)

riv_spec (n=18, range 921662.5-1060168.1 ns)
  921662.5 |
  928587.7 |#############
  935513.0 |
  942438.3 |#############
  949363.6 |
  956288.9 |#############
  963214.1 |
  970139.4 |#############
  977064.7 |
  983990.0 |#############
  990915.3 |
  997840.5 |#############
  1004765.8 |
  1011691.1 |#############
  1018616.4 |#############
  1025541.7 |#############
  1032466.9 |########################################
  1039392.2 |#############
  1046317.5 |
  1053242.8 |##########################
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1606.5% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1589.5% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1544.2% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1644.6% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1674.0% of algo (FFI overhead may distort results)
