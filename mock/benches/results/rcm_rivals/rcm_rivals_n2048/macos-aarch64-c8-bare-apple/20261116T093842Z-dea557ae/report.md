# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.6% (fastest 7.85 ms to slowest 7.98 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### riv_rcm shows alternating (throttle bounce) (autocorr -0.58)

riv_rcm's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (123.56 us) is smaller than the fastest variant's own run-to-run std-dev (223.30 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: riv_rowm** at 7854479.2 ns median (-1.5% vs baseline)
- Spread: 1.02x (fastest 7854479.2 ns, slowest 7978043.8 ns)

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
| riv_rcm | 8024422ns | 7979694ns | 7870436ns | 7961107ns | 8196388ns | base |
| riv_rowm | 7958707ns | 7861350ns | 7744004ns | 7852065ns | 8226022ns | -0.82% |
| riv_scr | 8008046ns | 7955119ns | 7802099ns | 7926732ns | 8232991ns | -0.20% |
| riv_snake | 7912883ns | 7918819ns | 7729606ns | 7879287ns | 8054915ns | -1.39% |
| riv_spec | 7998915ns | 7984452ns | 7719926ns | 7925305ns | 8248825ns | -0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 8018158ns | 7863926ns | 8190302ns | base | 0.000 |
| riv_rowm | 7952215ns | 7737643ns | 8219880ns | -0.82% | 0.000 |
| riv_scr | 8001613ns | 7795081ns | 8226938ns | -0.21% | 0.000 |
| riv_snake | 7906478ns | 7722701ns | 8048631ns | -1.39% | 0.000 |
| riv_spec | 7992448ns | 7712786ns | 8242730ns | -0.32% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 96.7% |
| riv_rowm | 0.000 | 98.2% |
| riv_scr | 0.000 | 97.0% |
| riv_snake | 0.000 | 97.5% |
| riv_spec | 0.000 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 8024422ns | 8024422ns | base |
| riv_rowm | 7958707ns | 7958707ns | -0.82% |
| riv_scr | 8008046ns | 8008046ns | -0.20% |
| riv_snake | 7912883ns | 7912883ns | -1.39% |
| riv_spec | 7998915ns | 7998915ns | -0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 7973360ns | base | --- | [7920508, 8064692] | --- | --- | --- | --- |
| riv_rowm | 7854479ns | no significant difference | [-189335, +49925]ns | [7803725, 8040985] | no | 0.6409 | 0.2379 | 0 |
| riv_scr | 7948767ns | no significant difference | [-143694, +98006]ns | [7883583, 8044869] | no | 0.8145 | 0.8145 | 0 |
| riv_snake | 7913358ns | no significant difference | [-181121, +15558]ns | [7817921, 7948408] | no | 0.6409 | 0.4807 | 0 |
| riv_spec | 7978044ns | no significant difference | [-202844, +94410]ns | [7839629, 8115577] | no | 0.6409 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 7956708ns | +4.5% | +2.7% | -0.1% | +0.9% |
| 1 | 2 | 8270262ns | -6.0% | -3.6% | -4.3% | -3.0% |
| 1 | 3 | 7894483ns | +0.2% | +1.1% | -0.3% | -1.4% |
| 2 | 1 | 8053075ns | -3.2% | -1.7% | -4.0% | -0.5% |
| 2 | 2 | 7922196ns | -1.1% | -1.0% | -1.2% | +0.6% |
| 2 | 3 | 8018188ns | -2.6% | -1.8% | -2.1% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.585 | HIGH- (thermal bounce) |
| riv_rowm | -0.186 | ok |
| riv_scr | 0.260 | moderate+ |
| riv_snake | 0.281 | moderate+ |
| riv_spec | -0.329 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 2/6
- **riv_scr**: won 4/6, lost 2/6
- **riv_snake**: won 6/6, lost 0/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 134260579.6ns | 8018158.1ns | 1674.5% | HIGH |
| riv_rowm | 134272650.5ns | 7952215.3ns | 1688.5% | HIGH |
| riv_scr | 134439472.9ns | 8001613.4ns | 1680.2% | HIGH |
| riv_snake | 134286723.8ns | 7906478.2ns | 1698.4% | HIGH |
| riv_spec | 134777338.6ns | 7992447.7ns | 1686.3% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 7863926.4-8190302.1 ns)
  7863926.4 |
  7880245.2 |##########
  7896563.9 |##########
  7912882.7 |########################################
  7929201.5 |##########
  7945520.3 |##########
  7961839.1 |
  7978157.9 |##########
  7994476.6 |
  8010795.4 |####################
  8027114.2 |
  8043433.0 |##########
  8059751.8 |
  8076070.6 |####################
  8092389.4 |##########
  8108708.1 |
  8125026.9 |
  8141345.7 |
  8157664.5 |
  8173983.3 |
  (1 below, 2 above range)

riv_rowm (n=18, range 7737643.1-8219879.9 ns)
  7737643.1 |##########################
  7761754.9 |#############
  7785866.7 |##########################
  7809978.6 |#############
  7834090.4 |########################################
  7858202.3 |
  7882314.1 |
  7906426.0 |#############
  7930537.8 |#############
  7954649.6 |
  7978761.5 |
  8002873.3 |#############
  8026985.2 |
  8051097.0 |#############
  8075208.8 |
  8099320.7 |
  8123432.5 |#############
  8147544.4 |
  8171656.2 |
  8195768.0 |
  (1 below, 3 above range)

riv_scr (n=18, range 7795080.5-8226937.5 ns)
  7795080.5 |#############
  7816673.4 |#############
  7838266.2 |#############
  7859859.1 |#############
  7881451.9 |#############
  7903044.8 |#############
  7924637.6 |#############
  7946230.5 |########################################
  7967823.3 |##########################
  7989416.2 |
  8011009.0 |
  8032601.9 |
  8054194.7 |
  8075787.6 |
  8097380.4 |#############
  8118973.3 |
  8140566.1 |
  8162159.0 |#############
  8183751.8 |
  8205344.7 |
  (1 below, 3 above range)

riv_snake (n=18, range 7722701.4-8048631.2 ns)
  7722701.4 |##########
  7738997.9 |##########
  7755294.4 |
  7771590.8 |
  7787887.3 |##########
  7804183.8 |##########
  7820480.3 |##########
  7836776.8 |##########
  7853073.3 |
  7869369.8 |##########
  7885666.3 |
  7901962.8 |####################
  7918259.3 |
  7934555.8 |########################################
  7950852.3 |
  7967148.8 |
  7983445.3 |##########
  7999741.8 |
  8016038.2 |
  8032334.7 |
  (1 below, 3 above range)

riv_spec (n=18, range 7712786.1-8242729.9 ns)
  7712786.1 |####################
  7739283.3 |
  7765780.5 |####################
  7792277.7 |
  7818774.9 |####################
  7845272.0 |####################
  7871769.2 |
  7898266.4 |########################################
  7924763.6 |
  7951260.8 |####################
  7977758.0 |####################
  8004255.2 |########################################
  8030752.4 |####################
  8057249.5 |
  8083746.7 |
  8110243.9 |
  8136741.1 |
  8163238.3 |
  8189735.5 |####################
  8216232.7 |####################
  (2 below, 3 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1682.6% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1708.0% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1689.7% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1695.0% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1680.5% of algo (FFI overhead may distort results)
