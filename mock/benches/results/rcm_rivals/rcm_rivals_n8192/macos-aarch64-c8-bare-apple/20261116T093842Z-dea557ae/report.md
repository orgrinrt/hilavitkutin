# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (riv_rcm, riv_snake) are a dead heat (<1%)

riv_rcm (39.43 ms) and riv_snake (39.56 ms) differ by 0.32%, inside the noise, even though the wider field spreads 3.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### riv_snake shows alternating (throttle bounce) (autocorr -0.56)

riv_snake's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.33 ms) is smaller than the fastest variant's own run-to-run std-dev (3.68 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (riv_rcm)

The baseline riv_rcm is the fastest (39.43 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader riv_rcm vs stability leader riv_rowm (+3% speed for 1.2x steadier)

riv_rcm is fastest (39.43 ms, CV 9.3%); riv_rowm gives up 3.4% median for 1.2x lower variance (CV 7.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### riv_scr is inconsistent: worst-20% is 2.0x its best-20%

riv_scr's best 20% of batches run at 32.85 ms but its worst 20% at 66.79 ms (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 3.4% of the fastest

All 5 variants sit between 39.43 ms and 40.76 ms - a 3.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (riv_rcm) is the fastest** at 39434445.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 39434445.9 ns, slowest 40760983.3 ns)

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
| riv_rcm | 38489722ns | 39442667ns | 31954340ns | 38322511ns | 42008231ns | base |
| riv_rowm | 39434893ns | 40767308ns | 33257024ns | 39874958ns | 41863731ns | +2.46% |
| riv_scr | 47142088ns | 39674579ns | 32856592ns | 38798531ns | 66800171ns | +22.48% |
| riv_snake | 40400519ns | 39567731ns | 31892957ns | 36841374ns | 49993019ns | +4.96% |
| riv_spec | 39930192ns | 39841865ns | 31968032ns | 37780577ns | 47135693ns | +3.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 38482537ns | 31946249ns | 42001719ns | base | 0.000 |
| riv_rowm | 39428038ns | 33248993ns | 41856692ns | +2.46% | 0.000 |
| riv_scr | 47133481ns | 32847933ns | 66788449ns | +22.48% | 0.000 |
| riv_snake | 40392114ns | 31885624ns | 49982732ns | +4.96% | 0.000 |
| riv_spec | 39922368ns | 31960415ns | 47127066ns | +3.74% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_snake; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 80.9% |
| riv_rowm | 0.000 | 78.2% |
| riv_scr | 0.000 | 80.4% |
| riv_snake | 0.000 | 80.6% |
| riv_spec | 0.000 | 80.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 38489722ns | 38489722ns | base |
| riv_rowm | 39434893ns | 39434893ns | +2.46% |
| riv_scr | 47142088ns | 47142088ns | +22.48% |
| riv_snake | 40400519ns | 40400519ns | +4.96% |
| riv_spec | 39930192ns | 39930192ns | +3.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 39434446ns | base | --- | [36875806, 40202652] | --- | --- | --- | --- |
| riv_rowm | 40760983ns | +876227.0ns (+2.2%) | [+118510, +1570187]ns | [38535646, 41402546] | YES (adj: no) | 0.1235 | 0.0309 | 0 |
| riv_scr | 39668235ns | no significant difference | [-449812, +1260935]ns | [38500379, 41685508] | no | 1.0000 | 1.0000 | 0 |
| riv_snake | 39559656ns | no significant difference | [-2821571, +2227956]ns | [32766681, 42463954] | no | 0.6409 | 0.4807 | 0 |
| riv_spec | 39835233ns | no significant difference | [-713144, +1935183]ns | [34769108, 41176671] | no | 0.6409 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 42595338ns | -0.0% | -1.9% | -1.7% | -3.2% |
| 1 | 2 | 40511200ns | +2.4% | +0.7% | -21.2% | -20.7% |
| 1 | 3 | 35249321ns | +15.9% | +124.2% | +38.3% | +20.0% |
| 2 | 1 | 39020046ns | +4.2% | -1.2% | +2.8% | +1.2% |
| 2 | 2 | 39466550ns | -2.4% | -2.4% | +34.5% | +1.1% |
| 2 | 3 | 31946050ns | +2.7% | +5.0% | +1.1% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.079 | ok |
| riv_rowm | 0.253 | moderate+ |
| riv_scr | -0.171 | ok |
| riv_snake | -0.562 | HIGH- (thermal bounce) |
| riv_spec | -0.441 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 1/6, lost 4/6
- **riv_scr**: won 3/6, lost 3/6
- **riv_snake**: won 2/6, lost 4/6
- **riv_spec**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 545552249.5ns | 38482537.0ns | 1417.7% | HIGH |
| riv_rowm | 546951720.1ns | 39428038.4ns | 1387.2% | HIGH |
| riv_scr | 565523486.3ns | 47133480.8ns | 1199.8% | HIGH |
| riv_snake | 549416954.4ns | 40392114.3ns | 1360.2% | HIGH |
| riv_spec | 547435969.7ns | 39922368.1ns | 1371.3% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 31946248.6-42001718.8 ns)
  31946248.6 |#############
  32449022.1 |
  32951795.6 |
  33454569.2 |
  33957342.7 |
  34460116.2 |
  34962889.7 |##########################
  35465663.2 |
  35968436.7 |
  36471210.2 |
  36973983.7 |
  37476757.2 |
  37979530.7 |
  38482304.2 |##########################
  38985077.7 |########################################
  39487851.2 |########################################
  39990624.7 |
  40493398.2 |#############
  40996171.7 |
  41498945.2 |#############
  (2 below, 3 above range)

riv_rowm (n=18, range 33248993.1-41856692.4 ns)
  33248993.1 |
  33679378.0 |
  34109763.0 |#############
  34540148.0 |
  34970532.9 |
  35400917.9 |
  35831302.9 |
  36261687.8 |
  36692072.8 |
  37122457.8 |
  37552842.7 |
  37983227.7 |#############
  38413612.6 |##########################
  38843997.6 |
  39274382.6 |#############
  39704767.5 |
  40135152.5 |#############
  40565537.5 |##########################
  40995922.4 |########################################
  41426307.4 |########################################
  (2 below, 2 above range)

riv_scr (n=18, range 32847933.3-66788449.3 ns)
  32847933.3 |################
  34544959.1 |
  36241984.9 |
  37939010.7 |########################################
  39636036.5 |########################
  41333062.3 |########################
  43030088.1 |
  44727113.9 |
  46424139.7 |
  48121165.5 |
  49818191.3 |
  51515217.1 |
  53212242.9 |
  54909268.7 |
  56606294.5 |
  58303320.3 |
  60000346.1 |
  61697371.9 |
  63394397.7 |
  65091423.5 |
  (2 below, 3 above range)

riv_snake (n=18, range 31885623.6-49982731.9 ns)
  31885623.6 |########################################
  32790479.0 |##########
  33695334.4 |##########
  34600189.8 |
  35505045.3 |
  36409900.7 |
  37314756.1 |##########
  38219611.5 |
  39124466.9 |####################
  40029322.3 |##########
  40934177.8 |
  41839033.2 |####################
  42743888.6 |##########
  43648744.0 |
  44553599.4 |
  45458454.8 |
  46363310.3 |
  47268165.7 |
  48173021.1 |##########
  49077876.5 |
  (1 below, 3 above range)

riv_spec (n=18, range 31960415.3-47127066.0 ns)
  31960415.3 |########################################
  32718747.8 |
  33477080.3 |
  34235412.9 |
  34993745.4 |
  35752077.9 |
  36510410.5 |
  37268743.0 |##########
  38027075.5 |
  38785408.1 |####################
  39543740.6 |########################################
  40302073.2 |
  41060405.7 |##############################
  41818738.2 |##########
  42577070.8 |
  43335403.3 |
  44093735.8 |
  44852068.4 |
  45610400.9 |
  46368733.4 |
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1382.4% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1341.2% of algo (FFI overhead may distort results)
- **riv_scr**: CV=46.3% (high variance, measurements may be unstable)
- **riv_scr**: bridge=1382.6% of algo (FFI overhead may distort results)
- **riv_snake**: CV=20.3% (high variance, measurements may be unstable)
- **riv_snake**: bridge=1376.0% of algo (FFI overhead may distort results)
- **riv_spec**: CV=20.4% (high variance, measurements may be unstable)
- **riv_spec**: bridge=1375.5% of algo (FFI overhead may distort results)
