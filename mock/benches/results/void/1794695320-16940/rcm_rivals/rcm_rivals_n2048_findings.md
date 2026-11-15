# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.1% (fastest 7.80 ms to slowest 7.88 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### riv_rowm shows alternating (throttle bounce) (autocorr -0.71)

riv_rowm's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (86.27 us) is smaller than the fastest variant's own run-to-run std-dev (496.11 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_spec vs stability leader riv_snake (+0% speed for 4.2x steadier)

riv_spec is fastest (7.80 ms, CV 6.4%); riv_snake gives up 0.4% median for 4.2x lower variance (CV 1.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: riv_spec** at 7797335.4 ns median (-0.9% vs baseline)
- Spread: 1.01x (fastest 7797335.4 ns, slowest 7883608.3 ns)

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
| riv_rcm | 7922422ns | 7874917ns | 7728517ns | 7860768ns | 8111855ns | base |
| riv_rowm | 7879850ns | 7814410ns | 7736843ns | 7810162ns | 8055884ns | -0.54% |
| riv_scr | 8233345ns | 7891562ns | 7710681ns | 7835547ns | 9091373ns | +3.92% |
| riv_snake | 7853955ns | 7833969ns | 7720093ns | 7807008ns | 7991308ns | -0.86% |
| riv_spec | 7996742ns | 7804056ns | 7655144ns | 7786337ns | 8483149ns | +0.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 7915364ns | 7721343ns | 8104828ns | base | 0.000 |
| riv_rowm | 7873055ns | 7730175ns | 8049319ns | -0.53% | 0.000 |
| riv_scr | 8226319ns | 7704179ns | 9084336ns | +3.93% | 0.000 |
| riv_snake | 7847046ns | 7713200ns | 7984503ns | -0.86% | 0.000 |
| riv_spec | 7989640ns | 7647882ns | 8475896ns | +0.94% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 97.2% |
| riv_rowm | 0.000 | 98.0% |
| riv_scr | 0.000 | 97.0% |
| riv_snake | 0.000 | 97.7% |
| riv_spec | 0.000 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 7922422ns | 7922422ns | base |
| riv_rowm | 7879850ns | 7879850ns | -0.54% |
| riv_scr | 8233345ns | 8233345ns | +3.92% |
| riv_snake | 7853955ns | 7853955ns | -0.86% |
| riv_spec | 7996742ns | 7996742ns | +0.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 7868250ns | base | --- | [7791681, 8048083] | --- | --- | --- | --- |
| riv_rowm | 7807917ns | no significant difference | [-105285, +10365]ns | [7778023, 7975373] | no | 0.6409 | 0.2379 | 0 |
| riv_scr | 7883608ns | no significant difference | [-203902, +216062]ns | [7758165, 7983912] | no | 0.8145 | 0.8145 | 0 |
| riv_snake | 7827127ns | no significant difference | [-175452, +54515]ns | [7751910, 7894217] | no | 0.6409 | 0.4807 | 0 |
| riv_spec | 7797335ns | no significant difference | [-163102, +124700]ns | [7712092, 7985804] | no | 0.6409 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 7850838ns | +0.0% | -0.7% | -1.0% | -1.7% |
| 1 | 2 | 7815554ns | -0.5% | -1.2% | +0.7% | -1.3% |
| 1 | 3 | 8052388ns | -1.0% | -3.8% | -2.9% | -3.7% |
| 2 | 1 | 8043779ns | -3.5% | +3.9% | -3.0% | -3.6% |
| 2 | 2 | 7749783ns | +1.5% | +2.8% | +1.2% | +2.9% |
| 2 | 3 | 7944404ns | -2.1% | -0.6% | -1.4% | +5.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.203 | moderate- |
| riv_rowm | -0.713 | HIGH- (thermal bounce) |
| riv_scr | 0.018 | ok |
| riv_snake | -0.466 | moderate- |
| riv_spec | 0.284 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 1/6
- **riv_scr**: won 4/6, lost 2/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 134237527.1ns | 7915364.1ns | 1695.9% | HIGH |
| riv_rowm | 134258427.8ns | 7873054.9ns | 1705.3% | HIGH |
| riv_scr | 134808392.6ns | 8226318.8ns | 1638.7% | HIGH |
| riv_snake | 134316327.1ns | 7847045.8ns | 1711.7% | HIGH |
| riv_spec | 136103327.8ns | 7989639.6ns | 1703.5% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 7721343.1-8104827.8 ns)
  7721343.1 |#############
  7740517.3 |#############
  7759691.5 |
  7778865.8 |########################################
  7798040.0 |##########################
  7817214.2 |
  7836388.5 |#############
  7855562.7 |
  7874737.0 |#############
  7893911.2 |
  7913085.4 |
  7932259.7 |#############
  7951433.9 |
  7970608.1 |
  7989782.4 |#############
  8008956.6 |
  8028130.8 |#############
  8047305.1 |#############
  8066479.3 |#############
  8085653.5 |
  (1 below, 3 above range)

riv_rowm (n=18, range 7730175.0-8049319.4 ns)
  7730175.0 |
  7746132.2 |########################################
  7762089.4 |#############
  7778046.7 |##########################
  7794003.9 |##########################
  7809961.1 |#############
  7825918.3 |
  7841875.6 |#############
  7857832.8 |#############
  7873790.0 |
  7889747.2 |
  7905704.4 |
  7921661.7 |
  7937618.9 |
  7953576.1 |
  7969533.3 |########################################
  7985490.5 |
  8001447.8 |#############
  8017405.0 |
  8033362.2 |
  (1 below, 2 above range)

riv_scr (n=18, range 7704179.1-9084336.1 ns)
  7704179.1 |########################################
  7773187.0 |####################
  7842194.8 |##############################
  7911202.7 |####################
  7980210.5 |##########
  8049218.4 |
  8118226.2 |
  8187234.1 |
  8256241.9 |##########
  8325249.8 |##########
  8394257.6 |
  8463265.5 |
  8532273.3 |
  8601281.2 |
  8670289.0 |
  8739296.9 |
  8808304.7 |
  8877312.6 |
  8946320.4 |
  9015328.3 |
  (2 below, 2 above range)

riv_snake (n=18, range 7713200.0-7984502.8 ns)
  7713200.0 |########################################
  7726765.1 |########################################
  7740330.3 |
  7753895.4 |
  7767460.6 |####################
  7781025.7 |####################
  7794590.8 |####################
  7808156.0 |
  7821721.1 |########################################
  7835286.3 |####################
  7848851.4 |
  7862416.5 |########################################
  7875981.7 |
  7889546.8 |
  7903111.9 |####################
  7916677.1 |
  7930242.2 |
  7943807.4 |
  7957372.5 |
  7970937.6 |########################################
  (1 below, 2 above range)

riv_spec (n=18, range 7647881.9-8475895.8 ns)
  7647881.9 |#############
  7689282.6 |########################################
  7730683.3 |########################################
  7772084.0 |
  7813484.7 |#############
  7854885.4 |
  7896286.1 |##########################
  7937686.8 |#############
  7979087.5 |#############
  8020488.2 |
  8061888.9 |
  8103289.6 |
  8144690.3 |
  8186091.0 |
  8227491.7 |#############
  8268892.4 |
  8310293.1 |
  8351693.7 |
  8393094.4 |#############
  8434495.1 |#############
  (2 below, 1 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1704.6% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1718.2% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1701.8% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1713.6% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1720.2% of algo (FFI overhead may distort results)
