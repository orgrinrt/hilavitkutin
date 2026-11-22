# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (riv_snake, riv_rcm) are a dead heat (<1%)

riv_snake (63.93 ms) and riv_rcm (64.51 ms) differ by 0.89%, inside the noise, even though the wider field spreads 5.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3.61 ms) is smaller than the fastest variant's own run-to-run std-dev (12.92 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_snake vs stability leader riv_rcm (+1% speed for 4.6x steadier)

riv_snake is fastest (63.93 ms, CV 20.2%); riv_rcm gives up 0.9% median for 4.6x lower variance (CV 4.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### riv_scr is inconsistent: worst-20% is 1.9x its best-20%

riv_scr's best 20% of batches run at 63.51 ms but its worst 20% at 119.53 ms (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: riv_snake** at 63934743.8 ns median (-0.9% vs baseline)
- Spread: 1.06x (fastest 63934743.8 ns, slowest 67546320.8 ns)

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
| riv_rcm | 65411769ns | 64513679ns | 63586725ns | 64291910ns | 68004078ns | base |
| riv_rowm | 67360756ns | 64931096ns | 63346142ns | 64735044ns | 73306633ns | +2.98% |
| riv_scr | 82861838ns | 65610515ns | 63521899ns | 64856792ns | 119539377ns | +26.68% |
| riv_snake | 68440431ns | 63942960ns | 63414324ns | 63917403ns | 77738028ns | +4.63% |
| riv_spec | 77736230ns | 67556888ns | 63467757ns | 66524420ns | 101688182ns | +18.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 65403198ns | 63578939ns | 67994640ns | base | 0.000 |
| riv_rowm | 67351169ns | 63337476ns | 73295965ns | +2.98% | 0.000 |
| riv_scr | 82851916ns | 63511886ns | 119526381ns | +26.68% | 0.000 |
| riv_snake | 68431703ns | 63404824ns | 77728821ns | +4.63% | 0.000 |
| riv_spec | 77726461ns | 63459544ns | 101676155ns | +18.84% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 98.2% |
| riv_rowm | 0.000 | 97.6% |
| riv_scr | 0.000 | 96.5% |
| riv_snake | 0.000 | 99.1% |
| riv_spec | 0.000 | 93.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 65411769ns | 65411769ns | base |
| riv_rowm | 67360756ns | 67360756ns | +2.98% |
| riv_scr | 82861838ns | 82861838ns | +26.68% |
| riv_snake | 68440431ns | 68440431ns | +4.63% |
| riv_spec | 77736230ns | 77736230ns | +18.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 64506048ns | base | --- | [63887423, 65382992] | --- | --- | --- | --- |
| riv_rowm | 64921404ns | no significant difference | [-1100244, +6241031]ns | [63474533, 70042250] | no | 1.0000 | 1.0000 | 0 |
| riv_scr | 65602860ns | no significant difference | [-479554, +24363535]ns | [63823092, 85368721] | no | 0.9614 | 0.4807 | 0 |
| riv_snake | 63934744ns | no significant difference | [-1608371, +857148]ns | [63758342, 65043906] | no | 1.0000 | 0.8145 | 0 |
| riv_spec | 67546321ns | no significant difference | [-151975, +10014521]ns | [64275000, 76344921] | no | 0.9515 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 64382392ns | -1.4% | -0.9% | -0.4% | -1.2% |
| 1 | 2 | 66905471ns | -3.2% | +27.6% | -4.7% | +17.7% |
| 1 | 3 | 64692712ns | -1.9% | +60.6% | -1.7% | +38.2% |
| 2 | 1 | 63923717ns | +1.8% | +3.3% | +2.6% | +5.5% |
| 2 | 2 | 65030738ns | +7.5% | +0.2% | -1.7% | -0.5% |
| 2 | 3 | 63908817ns | +6.2% | +1.2% | -0.1% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.229 | moderate- |
| riv_rowm | 0.363 | moderate+ |
| riv_scr | 0.087 | ok |
| riv_snake | -0.309 | moderate- |
| riv_spec | 0.149 | ok |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 3/6
- **riv_scr**: won 1/6, lost 5/6
- **riv_snake**: won 5/6, lost 1/6
- **riv_spec**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 1076990104.4ns | 65403198.2ns | 1646.7% | HIGH |
| riv_rowm | 1079561919.2ns | 67351169.4ns | 1602.9% | HIGH |
| riv_scr | 1083704722.9ns | 82851916.4ns | 1308.0% | HIGH |
| riv_snake | 1078848838.9ns | 68431703.5ns | 1576.5% | HIGH |
| riv_spec | 1083290305.6ns | 77726460.9ns | 1393.7% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 63578938.9-67994640.3 ns)
  63578938.9 |########################################
  63799724.0 |########################################
  64020509.0 |
  64241294.1 |#############
  64462079.2 |##########################
  64682864.2 |#############
  64903649.3 |#############
  65124434.4 |#############
  65345219.5 |#############
  65566004.5 |
  65786789.6 |
  66007574.7 |#############
  66228359.7 |
  66449144.8 |
  66669929.9 |
  66890714.9 |#############
  67111500.0 |
  67332285.1 |
  67553070.1 |
  67773855.2 |
  (1 below, 2 above range)

riv_rowm (n=18, range 63337476.4-73295965.3 ns)
  63337476.4 |########################################
  63835400.8 |
  64333325.3 |#####
  64831249.7 |#####
  65329174.1 |
  65827098.6 |
  66325023.0 |
  66822947.5 |#####
  67320871.9 |
  67818796.4 |#####
  68316720.8 |
  68814645.3 |
  69312569.7 |
  69810494.2 |###########
  70308418.6 |
  70806343.1 |
  71304267.5 |
  71802191.9 |
  72300116.4 |
  72798040.8 |#####
  (1 below, 3 above range)

riv_scr (n=18, range 63511886.1-119526381.2 ns)
  63511886.1 |########################################
  66312610.9 |
  69113335.6 |
  71914060.4 |
  74714785.2 |
  77515509.9 |
  80316234.7 |
  83116959.4 |###
  85917684.2 |
  88718408.9 |
  91519133.7 |###
  94319858.4 |
  97120583.2 |###
  99921308.0 |
  102722032.7 |###
  105522757.5 |
  108323482.2 |
  111124207.0 |
  113924931.7 |###
  116725656.5 |
  (1 below, 1 above range)

riv_snake (n=18, range 63404823.6-77728820.8 ns)
  63404823.6 |########################################
  64121023.5 |###############
  64837223.3 |
  65553423.2 |#####
  66269623.0 |#####
  66985822.9 |
  67702022.8 |
  68418222.6 |
  69134422.5 |#####
  69850622.4 |
  70566822.2 |
  71283022.1 |
  71999221.9 |
  72715421.8 |
  73431621.7 |
  74147821.5 |
  74864021.4 |
  75580221.3 |
  76296421.1 |
  77012621.0 |
  (2 below, 2 above range)

riv_spec (n=18, range 63459544.4-101676154.9 ns)
  63459544.4 |########################################
  65370375.0 |
  67281205.5 |#############
  69192036.0 |######
  71102866.5 |######
  73013697.0 |######
  74924527.6 |
  76835358.1 |
  78746188.6 |######
  80657019.1 |######
  82567849.7 |
  84478680.2 |######
  86389510.7 |
  88300341.2 |######
  90211171.7 |
  92122002.3 |
  94032832.8 |
  95943663.3 |
  97854493.8 |
  99765324.3 |
  (2 below, 1 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1668.8% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1658.8% of algo (FFI overhead may distort results)
- **riv_scr**: CV=44.9% (high variance, measurements may be unstable)
- **riv_scr**: bridge=1641.6% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1684.1% of algo (FFI overhead may distort results)
- **riv_spec**: CV=40.0% (high variance, measurements may be unstable)
- **riv_spec**: bridge=1596.4% of algo (FFI overhead may distort results)
