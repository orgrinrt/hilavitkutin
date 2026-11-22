# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.3% (fastest 17.05 ms to slowest 17.28 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### rcm_half shows alternating (throttle bounce) (autocorr -0.59)

rcm_half's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: rcm_half** at 17052312.5 ns median (-0.3% vs baseline)
- Spread: 1.01x (fastest 17052312.5 ns, slowest 17276239.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A1-1 dispatch order: column-adjacency dose response across valid topo orders |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345678 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 17219504ns | 17112894ns | 16886124ns | 17059403ns | 17626345ns | base |
| rcm_half | 17123393ns | 17062504ns | 16898322ns | 17023255ns | 17386135ns | -0.56% |
| rcm_rev | 17409047ns | 17285069ns | 17000457ns | 17269538ns | 17822604ns | +1.10% |
| rcm_scr | 17136511ns | 17129204ns | 16850156ns | 17074762ns | 17372312ns | -0.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 17210411ns | 16875760ns | 17617887ns | base | 0.000 |
| rcm_half | 17113223ns | 16888028ns | 17376410ns | -0.56% | 0.000 |
| rcm_rev | 17400221ns | 16990756ns | 17814115ns | +1.10% | 0.000 |
| rcm_scr | 17126947ns | 16839493ns | 17363133ns | -0.48% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_scr; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.5% |
| rcm_half | 0.000 | 98.8% |
| rcm_rev | 0.000 | 97.5% |
| rcm_scr | 0.000 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 17219504ns | 17219504ns | base |
| rcm_half | 17123393ns | 17123393ns | -0.56% |
| rcm_rev | 17409047ns | 17409047ns | +1.10% |
| rcm_scr | 17136511ns | 17136511ns | -0.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 17103908ns | base | --- | [16978369, 17253506] | --- | --- | --- | --- |
| rcm_half | 17052312ns | no significant difference | [-227333, +145963]ns | [16966512, 17162429] | no | 0.7210 | 0.4807 | 0 |
| rcm_rev | 17276240ns | no significant difference | [-56462, +552677]ns | [17212910, 17605592] | no | 0.7137 | 0.2379 | 0 |
| rcm_scr | 17120333ns | no significant difference | [-277446, +175919]ns | [16926215, 17262275] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 17144171ns | -0.7% | +0.7% | +0.7% |
| 1 | 2 | 17174358ns | -0.5% | +3.5% | -1.9% |
| 1 | 3 | 17883208ns | -4.7% | -3.9% | -5.0% |
| 2 | 1 | 17190700ns | -1.6% | +0.5% | -1.5% |
| 2 | 2 | 16956862ns | +3.2% | +3.8% | +1.6% |
| 2 | 3 | 16990400ns | -0.5% | +1.4% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.036 | ok |
| rcm_half | -0.586 | HIGH- (thermal bounce) |
| rcm_rev | -0.559 | HIGH- (thermal bounce) |
| rcm_scr | -0.192 | ok |

**Consistency summary:**

- **rcm_half**: won 5/6, lost 1/6
- **rcm_rev**: won 1/6, lost 5/6
- **rcm_scr**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 358531250.5ns | 17210411.1ns | 2083.2% | HIGH |
| rcm_half | 359192648.9ns | 17113223.2ns | 2098.9% | HIGH |
| rcm_rev | 361423473.6ns | 17400220.6ns | 2077.1% | HIGH |
| rcm_scr | 358436884.3ns | 17126947.0ns | 2092.8% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 16875759.7-17617886.8 ns)
  16875759.7 |#############
  16912866.1 |
  16949972.4 |##########################
  16987078.8 |########################################
  17024185.1 |
  17061291.5 |
  17098397.8 |##########################
  17135504.2 |#############
  17172610.5 |##########################
  17209716.9 |
  17246823.2 |
  17283929.6 |#############
  17321036.0 |
  17358142.3 |
  17395248.7 |#############
  17432355.0 |
  17469461.4 |
  17506567.7 |#############
  17543674.1 |
  17580780.4 |
  (2 below, 2 above range)

rcm_half (n=18, range 16888027.8-17376409.7 ns)
  16888027.8 |########################################
  16912446.9 |####################
  16936866.0 |
  16961285.1 |####################
  16985704.2 |
  17010123.3 |########################################
  17034542.4 |########################################
  17058961.5 |####################
  17083380.6 |####################
  17107799.7 |####################
  17132218.8 |
  17156637.9 |
  17181057.0 |####################
  17205476.1 |
  17229895.2 |
  17254314.2 |
  17278733.3 |
  17303152.4 |
  17327571.5 |
  17351990.6 |
  (2 below, 4 above range)

rcm_rev (n=18, range 16990755.5-17814115.3 ns)
  16990755.5 |#############
  17031923.5 |##########################
  17073091.5 |
  17114259.5 |
  17155427.5 |#############
  17196595.5 |#############
  17237763.5 |########################################
  17278931.4 |#############
  17320099.4 |
  17361267.4 |#############
  17402435.4 |#############
  17443603.4 |
  17484771.4 |
  17525939.4 |
  17567107.4 |#############
  17608275.3 |#############
  17649443.3 |
  17690611.3 |
  17731779.3 |#############
  17772947.3 |
  (1 below, 3 above range)

rcm_scr (n=18, range 16839493.0-17363133.3 ns)
  16839493.0 |##########################
  16865675.0 |
  16891857.1 |#############
  16918039.1 |##########################
  16944221.1 |
  16970403.1 |#############
  16996585.1 |
  17022767.1 |
  17048949.2 |
  17075131.2 |#############
  17101313.2 |#############
  17127495.2 |#############
  17153677.2 |
  17179859.2 |
  17206041.2 |
  17232223.3 |########################################
  17258405.3 |#############
  17284587.3 |
  17310769.3 |
  17336951.3 |#############
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2092.8% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2098.8% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2072.5% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2091.2% of algo (FFI overhead may distort results)
