# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.7% (fastest 15.94 ms to slowest 16.05 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### riv_scr shows alternating (throttle bounce) (autocorr -0.51)

riv_scr's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (106.37 us) is smaller than the fastest variant's own run-to-run std-dev (278.89 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (riv_rcm)

The baseline riv_rcm is the fastest (15.94 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (riv_rcm) is the fastest** at 15940387.5 ns median
- Spread: 1.01x (fastest 15940387.5 ns, slowest 16046760.4 ns)

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
| riv_rcm | 16014794ns | 15949244ns | 15685904ns | 15910794ns | 16335238ns | base |
| riv_rowm | 16008795ns | 15947838ns | 15727331ns | 15920765ns | 16281573ns | -0.04% |
| riv_scr | 16081067ns | 16010833ns | 15757282ns | 15982502ns | 16390807ns | +0.41% |
| riv_snake | 16333809ns | 15978169ns | 15725126ns | 15959948ns | 17198941ns | +1.99% |
| riv_spec | 16086073ns | 16056088ns | 15720265ns | 15971442ns | 16440924ns | +0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 16006598ns | 15677599ns | 16327431ns | base | 0.000 |
| riv_rowm | 16000585ns | 15718783ns | 16273242ns | -0.04% | 0.000 |
| riv_scr | 16072702ns | 15748938ns | 16382374ns | +0.41% | 0.000 |
| riv_snake | 16325883ns | 15716156ns | 17191737ns | +1.99% | 0.000 |
| riv_spec | 16077819ns | 15711483ns | 16433278ns | +0.44% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rcm; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 98.4% |
| riv_rowm | 0.000 | 98.3% |
| riv_scr | 0.000 | 98.0% |
| riv_snake | 0.000 | 98.2% |
| riv_spec | 0.000 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 16014794ns | 16014794ns | base |
| riv_rowm | 16008795ns | 16008795ns | -0.04% |
| riv_scr | 16081067ns | 16081067ns | +0.41% |
| riv_snake | 16333809ns | 16333809ns | +1.99% |
| riv_spec | 16086073ns | 16086073ns | +0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 15940388ns | base | --- | [15818802, 16135462] | --- | --- | --- | --- |
| riv_rowm | 15940915ns | no significant difference | [-250150, +188077]ns | [15837404, 16074250] | no | 1.0000 | 0.8145 | 0 |
| riv_scr | 16001467ns | no significant difference | [-253990, +292742]ns | [15940819, 16072579] | no | 1.0000 | 1.0000 | 0 |
| riv_snake | 15969996ns | no significant difference | [-221517, +251117]ns | [15907390, 16086719] | no | 1.0000 | 0.8145 | 0 |
| riv_spec | 16046760ns | no significant difference | [-44621, +204810]ns | [15853921, 16137546] | no | 0.9515 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 15747371ns | +0.3% | +2.0% | +1.7% | +0.4% |
| 1 | 2 | 16057908ns | -0.9% | -0.4% | -1.5% | -1.1% |
| 1 | 3 | 16354104ns | -3.6% | -2.2% | -2.6% | -1.2% |
| 2 | 1 | 16244850ns | -1.3% | -2.3% | -1.7% | -1.1% |
| 2 | 2 | 15835146ns | +1.6% | +1.9% | +18.7% | +1.6% |
| 2 | 3 | 16093758ns | -1.0% | -0.8% | +0.0% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.024 | ok |
| riv_rowm | 0.073 | ok |
| riv_scr | -0.511 | HIGH- (thermal bounce) |
| riv_snake | -0.164 | ok |
| riv_spec | 0.245 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 2/6
- **riv_scr**: won 4/6, lost 2/6
- **riv_snake**: won 3/6, lost 2/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 268561066.2ns | 16006597.9ns | 1677.8% | HIGH |
| riv_rowm | 268534918.3ns | 16000585.0ns | 1678.3% | HIGH |
| riv_scr | 268503288.9ns | 16072702.3ns | 1670.6% | HIGH |
| riv_snake | 269105548.4ns | 16325882.9ns | 1648.3% | HIGH |
| riv_spec | 269196368.7ns | 16077819.0ns | 1674.3% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 15677598.6-16327431.2 ns)
  15677598.6 |########################################
  15710090.2 |
  15742581.9 |####################
  15775073.5 |####################
  15807565.1 |####################
  15840056.8 |########################################
  15872548.4 |
  15905040.0 |####################
  15937531.7 |####################
  15970023.3 |
  16002514.9 |
  16035006.6 |####################
  16067498.2 |####################
  16099989.8 |####################
  16132481.5 |####################
  16164973.1 |
  16197464.7 |
  16229956.4 |####################
  16262448.0 |
  16294939.6 |
  (1 below, 3 above range)

riv_rowm (n=18, range 15718783.3-16273241.7 ns)
  15718783.3 |
  15746506.2 |########################################
  15774229.2 |####################
  15801952.1 |####################
  15829675.0 |####################
  15857397.9 |####################
  15885120.8 |####################
  15912843.8 |####################
  15940566.7 |####################
  15968289.6 |
  15996012.5 |
  16023735.4 |########################################
  16051458.3 |####################
  16079181.2 |####################
  16106904.2 |####################
  16134627.1 |
  16162350.0 |
  16190072.9 |####################
  16217795.8 |
  16245518.8 |
  (1 below, 2 above range)

riv_scr (n=18, range 15748937.5-16382374.3 ns)
  15748937.5 |
  15780609.4 |####################
  15812281.2 |
  15843953.0 |##########
  15875624.9 |
  15907296.7 |##########
  15938968.6 |##########
  15970640.4 |########################################
  16002312.2 |##########
  16033984.1 |##########
  16065655.9 |####################
  16097327.7 |
  16128999.6 |##########
  16160671.4 |
  16192343.3 |
  16224015.1 |
  16255686.9 |
  16287358.8 |
  16319030.6 |
  16350702.4 |
  (1 below, 3 above range)

riv_snake (n=18, range 15716155.6-17191736.8 ns)
  15716155.6 |
  15789934.6 |##########
  15863713.7 |########################################
  15937492.8 |##############################
  16011271.8 |##############################
  16085050.9 |##########
  16158829.9 |
  16232609.0 |
  16306388.1 |##########
  16380167.1 |##########
  16453946.2 |
  16527725.2 |
  16601504.3 |
  16675283.4 |
  16749062.4 |
  16822841.5 |
  16896620.6 |
  16970399.6 |
  17044178.7 |
  17117957.7 |
  (2 below, 2 above range)

riv_spec (n=18, range 15711483.3-16433277.8 ns)
  15711483.3 |####################
  15747573.1 |
  15783662.8 |########################################
  15819752.5 |####################
  15855842.2 |####################
  15891931.9 |####################
  15928021.7 |
  15964111.4 |####################
  16000201.1 |####################
  16036290.8 |####################
  16072380.6 |########################################
  16108470.3 |####################
  16144560.0 |####################
  16180649.7 |
  16216739.4 |
  16252829.2 |
  16288918.9 |
  16325008.6 |
  16361098.3 |
  16397188.1 |
  (1 below, 4 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1684.5% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1684.1% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1678.2% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1681.3% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1673.5% of algo (FFI overhead may distort results)
