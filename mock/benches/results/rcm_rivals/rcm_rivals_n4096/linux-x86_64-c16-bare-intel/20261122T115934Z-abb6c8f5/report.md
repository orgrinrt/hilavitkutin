# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### riv_rowm shows alternating (throttle bounce) (autocorr -0.54)

riv_rowm's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (269.85 us) is smaller than the fastest variant's own run-to-run std-dev (1.39 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_rowm vs stability leader riv_snake (+0% speed for 8.6x steadier)

riv_rowm is fastest (37.13 ms, CV 3.7%); riv_snake gives up 0.4% median for 8.6x lower variance (CV 0.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.7% of the fastest

All 5 variants sit between 37.13 ms and 37.40 ms - a 0.7% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_rowm** at 37134746.7 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 37134746.7 ns, slowest 37404597.6 ns)

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
| riv_rcm | 37320938ns | 37155281ns | 37002435ns | 37129921ns | 37766714ns | base |
| riv_rowm | 37471492ns | 37138767ns | 37027405ns | 37108695ns | 38237732ns | +0.40% |
| riv_scr | 37483068ns | 37263729ns | 37086878ns | 37232365ns | 38057218ns | +0.43% |
| riv_snake | 37267747ns | 37285182ns | 36985533ns | 37254618ns | 37428547ns | -0.14% |
| riv_spec | 37441054ns | 37408431ns | 37090851ns | 37345456ns | 37759554ns | +0.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 37316923ns | 36998395ns | 37762794ns | base | 0.000 |
| riv_rowm | 37467484ns | 37023364ns | 38233697ns | +0.40% | 0.000 |
| riv_scr | 37478987ns | 37082690ns | 38053073ns | +0.43% | 0.000 |
| riv_snake | 37263751ns | 36981384ns | 37424569ns | -0.14% | 0.000 |
| riv_spec | 37436940ns | 37086702ns | 37755372ns | +0.32% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_snake; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 99.5% |
| riv_rowm | 0.000 | 99.6% |
| riv_scr | 0.000 | 99.3% |
| riv_snake | 0.000 | 99.2% |
| riv_spec | 0.000 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 37320938ns | 37320938ns | base |
| riv_rowm | 37471492ns | 37471492ns | +0.40% |
| riv_scr | 37483068ns | 37483068ns | +0.43% |
| riv_snake | 37267747ns | 37267747ns | -0.14% |
| riv_spec | 37441054ns | 37441054ns | +0.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 37151389ns | base | --- | [37089779, 37198562] | --- | --- | --- | --- |
| riv_rowm | 37134747ns | no significant difference | [-81838, +37277]ns | [37060906, 37191990] | no | 0.4807 | 0.4807 | 0 |
| riv_scr | 37259796ns | no significant difference | [-45140, +174863]ns | [37174006, 37331055] | no | 0.4807 | 0.4807 | 0 |
| riv_snake | 37281344ns | no significant difference | [-41321, +200780]ns | [37193158, 37344808] | no | 0.4807 | 0.4807 | 0 |
| riv_spec | 37404598ns | +283797.3ns (+0.8%) | [+112221, +379056]ns | [37221774, 37544485] | YES | 0.0302 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 37139603ns | -0.2% | +0.5% | +0.3% | +1.1% |
| 1 | 2 | 38267212ns | -2.6% | -2.5% | -2.6% | -2.8% |
| 1 | 3 | 37163175ns | -0.2% | +0.3% | -0.0% | +1.0% |
| 2 | 1 | 37209787ns | -0.2% | -0.2% | +0.4% | +1.0% |
| 2 | 2 | 37070374ns | +0.2% | +0.5% | +0.9% | +0.9% |
| 2 | 3 | 37003261ns | +0.1% | +0.5% | -0.0% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.169 | ok |
| riv_rowm | -0.541 | HIGH- (thermal bounce) |
| riv_scr | 0.111 | ok |
| riv_snake | -0.303 | moderate- |
| riv_spec | -0.153 | ok |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 2/6
- **riv_scr**: won 2/6, lost 4/6
- **riv_snake**: won 1/6, lost 3/6
- **riv_spec**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 184572418.1ns | 37316923.1ns | 494.6% | HIGH |
| riv_rowm | 184481103.0ns | 37467483.8ns | 492.4% | HIGH |
| riv_scr | 184609632.6ns | 37478986.7ns | 492.6% | HIGH |
| riv_snake | 184584691.6ns | 37263750.6ns | 495.3% | HIGH |
| riv_spec | 184703141.5ns | 37436939.6ns | 493.4% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 36998394.8-37762793.9 ns)
  36998394.8 |####################
  37036614.8 |####################
  37074834.7 |##########
  37113054.7 |##############################
  37151274.6 |########################################
  37189494.6 |####################
  37227714.5 |
  37265934.5 |
  37304154.4 |##########
  37342374.4 |
  37380594.4 |
  37418814.3 |
  37457034.3 |
  37495254.2 |
  37533474.2 |
  37571694.1 |
  37609914.1 |
  37648134.0 |
  37686354.0 |
  37724573.9 |
  (1 below, 2 above range)

riv_rowm (n=18, range 37023364.4-38233696.7 ns)
  37023364.4 |########################################
  37083881.0 |########################
  37144397.6 |########################
  37204914.2 |########
  37265430.9 |########################
  37325947.5 |
  37386464.1 |
  37446980.7 |
  37507497.3 |
  37568013.9 |
  37628530.5 |
  37689047.2 |
  37749563.8 |
  37810080.4 |
  37870597.0 |
  37931113.6 |
  37991630.2 |
  38052146.8 |
  38112663.5 |
  38173180.1 |
  (2 below, 1 above range)

riv_scr (n=18, range 37082689.9-38053072.6 ns)
  37082689.9 |
  37131209.1 |##############################
  37179728.2 |####################
  37228247.3 |########################################
  37276766.5 |####################
  37325285.6 |##########
  37373804.7 |
  37422323.9 |
  37470843.0 |##########
  37519362.1 |
  37567881.3 |
  37616400.4 |
  37664919.5 |
  37713438.7 |
  37761957.8 |
  37810476.9 |
  37858996.1 |##########
  37907515.2 |
  37956034.3 |
  38004553.5 |
  (2 below, 2 above range)

riv_snake (n=18, range 36981384.5-37424568.6 ns)
  36981384.5 |##########################
  37003543.7 |
  37025702.9 |
  37047862.1 |
  37070021.3 |
  37092180.5 |
  37114339.7 |#############
  37136498.9 |#############
  37158658.1 |
  37180817.3 |
  37202976.5 |
  37225135.7 |##########################
  37247294.9 |##########################
  37269454.2 |
  37291613.4 |#############
  37313772.6 |#############
  37335931.8 |########################################
  37358091.0 |
  37380250.2 |#############
  37402409.4 |
  (1 below, 3 above range)

riv_spec (n=18, range 37086701.7-37755372.0 ns)
  37086701.7 |##########################
  37120135.2 |
  37153568.7 |
  37187002.2 |##########################
  37220435.8 |#############
  37253869.3 |
  37287302.8 |#############
  37320736.3 |#############
  37354169.8 |
  37387603.4 |##########################
  37421036.9 |#############
  37454470.4 |
  37487903.9 |
  37521337.4 |########################################
  37554770.9 |#############
  37588204.4 |
  37621638.0 |#############
  37655071.5 |
  37688505.0 |
  37721938.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=496.2% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=496.4% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=493.7% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=494.3% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=492.6% of algo (FFI overhead may distort results)
