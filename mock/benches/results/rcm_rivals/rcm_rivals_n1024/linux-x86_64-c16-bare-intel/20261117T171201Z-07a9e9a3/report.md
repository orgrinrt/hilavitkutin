# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (9.73 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_snake at 9.45 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (272.95 us) is smaller than the fastest variant's own run-to-run std-dev (636.18 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 2.9% of the fastest

All 5 variants sit between 9.45 ms and 9.73 ms - a 2.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_snake** at 9454614.2 ns median (-2.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.03x (fastest 9454614.2 ns, slowest 9727561.6 ns)

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
| Counter frequency | 2112768073 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 10232911ns | 9731483ns | 9394988ns | 9815515ns | 11277966ns | base |
| riv_rowm | 9656410ns | 9470681ns | 9266526ns | 9421650ns | 10203493ns | -5.63% |
| riv_scr | 9798665ns | 9556831ns | 9263149ns | 9449649ns | 10589947ns | -4.24% |
| riv_snake | 9709315ns | 9458862ns | 9297473ns | 9413543ns | 10358894ns | -5.12% |
| riv_spec | 10107096ns | 9537912ns | 9441039ns | 9533965ns | 11299820ns | -1.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 10228817ns | 9390540ns | 11273793ns | base | 0.000 |
| riv_rowm | 9652276ns | 9262818ns | 10199462ns | -5.64% | 0.000 |
| riv_scr | 9794655ns | 9259215ns | 10585844ns | -4.24% | 0.000 |
| riv_snake | 9705364ns | 9293669ns | 10354987ns | -5.12% | 0.000 |
| riv_spec | 10102995ns | 9437153ns | 11295829ns | -1.23% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_scr; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 95.2% |
| riv_rowm | 0.000 | 97.8% |
| riv_scr | 0.000 | 96.9% |
| riv_snake | 0.000 | 97.9% |
| riv_spec | 0.000 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 10232911ns | 10232911ns | base |
| riv_rowm | 9656410ns | 9656410ns | -5.63% |
| riv_scr | 9798665ns | 9798665ns | -4.24% |
| riv_snake | 9709315ns | 9709315ns | -5.12% |
| riv_spec | 10107096ns | 10107096ns | -1.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 9727562ns | base | --- | [9617854, 10782813] | --- | --- | --- | --- |
| riv_rowm | 9466571ns | -387598.2ns (-4.0%) | [-1359110, -43866]ns | [9355059, 9507246] | YES (adj: no) | 0.2379 | 0.0963 | 0 |
| riv_scr | 9552966ns | no significant difference | [-1063466, +28580]ns | [9300941, 9704310] | no | 0.2379 | 0.2379 | 0 |
| riv_snake | 9454614ns | no significant difference | [-1321596, +179375]ns | [9346864, 9598136] | no | 0.2379 | 0.2379 | 0 |
| riv_spec | 9532850ns | no significant difference | [-481982, +60497]ns | [9481842, 10571474] | no | 0.2379 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 11715682ns | -19.1% | -18.1% | -19.2% | -7.9% |
| 1 | 2 | 9743714ns | -4.1% | -5.0% | -4.0% | +20.4% |
| 1 | 3 | 9711409ns | -2.1% | -4.2% | +16.5% | -1.6% |
| 2 | 1 | 9391461ns | +0.8% | +3.7% | +2.5% | +0.6% |
| 2 | 2 | 10200594ns | -7.2% | -7.7% | -9.0% | -6.6% |
| 2 | 3 | 9630982ns | -1.6% | +0.4% | -2.9% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.091 | ok |
| riv_rowm | -0.466 | moderate- |
| riv_scr | -0.298 | moderate- |
| riv_snake | -0.148 | ok |
| riv_spec | 0.289 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 5/6, lost 1/6
- **riv_scr**: won 4/6, lost 2/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 46498743.8ns | 10228816.9ns | 454.6% | HIGH |
| riv_rowm | 46355298.0ns | 9652275.8ns | 480.3% | HIGH |
| riv_scr | 46243868.8ns | 9794655.1ns | 472.1% | HIGH |
| riv_snake | 46328673.4ns | 9705364.2ns | 477.4% | HIGH |
| riv_spec | 46371996.1ns | 10102995.2ns | 459.0% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 9390539.7-11273792.9 ns)
  9390539.7 |##########################
  9484702.4 |
  9578865.0 |########################################
  9673027.7 |########################################
  9767190.3 |
  9861353.0 |
  9955515.7 |
  10049678.3 |
  10143841.0 |#############
  10238003.6 |
  10332166.3 |
  10426329.0 |
  10520491.6 |
  10614654.3 |##########################
  10708817.0 |
  10802979.6 |
  10897142.3 |#############
  10991304.9 |#############
  11085467.6 |
  11179630.3 |
  (2 below, 3 above range)

riv_rowm (n=18, range 9262818.3-10199461.8 ns)
  9262818.3 |########################
  9309650.4 |################
  9356482.6 |
  9403314.8 |########
  9450147.0 |########################################
  9496979.1 |################
  9543811.3 |################
  9590643.5 |
  9637475.7 |
  9684307.8 |
  9731140.0 |
  9777972.2 |
  9824804.4 |
  9871636.5 |
  9918468.7 |
  9965300.9 |
  10012133.1 |
  10058965.2 |
  10105797.4 |
  10152629.6 |
  (1 below, 2 above range)

riv_scr (n=18, range 9259214.9-10585844.2 ns)
  9259214.9 |########################################
  9325546.4 |##########
  9391877.8 |##########
  9458209.3 |
  9524540.8 |####################
  9590872.2 |####################
  9657203.7 |##########
  9723535.1 |##########
  9789866.6 |
  9856198.1 |
  9922529.5 |
  9988861.0 |
  10055192.5 |
  10121523.9 |
  10187855.4 |##########
  10254186.9 |
  10320518.3 |
  10386849.8 |
  10453181.3 |
  10519512.7 |
  (2 below, 3 above range)

riv_snake (n=18, range 9293669.2-10354986.7 ns)
  9293669.2 |########################################
  9346735.1 |##########################
  9399800.9 |##########################
  9452866.8 |########################################
  9505932.7 |
  9558998.6 |#############
  9612064.4 |#############
  9665130.3 |
  9718196.2 |
  9771262.1 |
  9824327.9 |
  9877393.8 |
  9930459.7 |
  9983525.6 |#############
  10036591.4 |
  10089657.3 |
  10142723.2 |#############
  10195789.0 |
  10248854.9 |
  10301920.8 |
  (2 below, 2 above range)

riv_spec (n=18, range 9437153.2-11295828.5 ns)
  9437153.2 |########################################
  9530087.0 |##########
  9623020.7 |#####
  9715954.5 |
  9808888.3 |
  9901822.0 |
  9994755.8 |
  10087689.6 |
  10180623.3 |
  10273557.1 |#####
  10366490.9 |
  10459424.6 |
  10552358.4 |
  10645292.2 |
  10738225.9 |#####
  10831159.7 |
  10924093.5 |
  11017027.2 |
  11109961.0 |
  11202894.8 |
  (1 below, 4 above range)

```

## Diagnostics

- **riv_rcm**: bridge=476.9% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=487.6% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=482.4% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=487.6% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=483.8% of algo (FFI overhead may distort results)
