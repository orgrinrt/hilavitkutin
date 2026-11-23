# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Speed leader rcm_rev vs stability leader rcm_adj (+4% speed for 2.2x steadier)

rcm_rev is fastest (4.60 ms, CV 5.3%); rcm_adj gives up 3.7% median for 2.2x lower variance (CV 2.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: rcm_rev** at 4600331.9 ns median (-3.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 4600331.9 ns, slowest 5016251.2 ns)

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
| Counter frequency | 2112776360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 4767317ns | 4776227ns | 4579044ns | 4748734ns | 4889328ns | base |
| rcm_half | 5035686ns | 5020320ns | 4867797ns | 4978728ns | 5205068ns | +5.63% |
| rcm_rev | 4679754ns | 4604340ns | 4437734ns | 4574626ns | 4958456ns | -1.84% |
| rcm_scr | 4869546ns | 4765962ns | 4662592ns | 4738499ns | 5169593ns | +2.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 4763329ns | 4575046ns | 4885326ns | base | 0.000 |
| rcm_half | 5031514ns | 4863270ns | 5200908ns | +5.63% | 0.000 |
| rcm_rev | 4675765ns | 4433681ns | 4954445ns | -1.84% | 0.000 |
| rcm_scr | 4865855ns | 4658941ns | 5165815ns | +2.15% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 92.9% |
| rcm_half | 0.000 | 88.4% |
| rcm_rev | 0.000 | 96.4% |
| rcm_scr | 0.000 | 93.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 4767317ns | 4767317ns | base |
| rcm_half | 5035686ns | 5035686ns | +5.63% |
| rcm_rev | 4679754ns | 4679754ns | -1.84% |
| rcm_scr | 4869546ns | 4869546ns | +2.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 4772379ns | base | --- | [4694733, 4831541] | --- | --- | --- | --- |
| rcm_half | 5016251ns | +322452.5ns (+6.8%) | [+169160, +375487]ns | [4909553, 5141645] | YES | 0.0004 | 0.0001 | 0 |
| rcm_rev | 4600332ns | -125191.5ns (-2.6%) | [-217163, -50010]ns | [4514516, 4709611] | YES | 0.0463 | 0.0309 | 0 |
| rcm_scr | 4762220ns | no significant difference | [-88000, +181634]ns | [4679878, 4933515] | no | 0.2379 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 4569771ns | +6.8% | -3.2% | +2.0% |
| 1 | 2 | 4904737ns | +0.9% | -5.4% | -3.1% |
| 1 | 3 | 4833637ns | +8.5% | +6.9% | +11.3% |
| 2 | 1 | 4740115ns | +8.7% | -0.8% | +4.4% |
| 2 | 2 | 4675504ns | +7.3% | -3.6% | +2.0% |
| 2 | 3 | 4828403ns | +1.2% | -5.8% | -3.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.298 | moderate- |
| rcm_half | 0.214 | moderate+ |
| rcm_rev | 0.070 | ok |
| rcm_scr | 0.052 | ok |

**Consistency summary:**

- **rcm_half**: won 0/6, lost 6/6
- **rcm_rev**: won 5/6, lost 1/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 30955490.5ns | 4763329.3ns | 649.9% | HIGH |
| rcm_half | 30951649.0ns | 5031513.6ns | 615.2% | HIGH |
| rcm_rev | 30874883.1ns | 4675764.8ns | 660.3% | HIGH |
| rcm_scr | 30837999.7ns | 4865855.4ns | 633.8% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 4575045.8-4885325.6 ns)
  4575045.8 |
  4590559.8 |#############
  4606073.8 |
  4621587.7 |
  4637101.7 |
  4652615.7 |
  4668129.7 |##########################
  4683643.7 |
  4699157.7 |#############
  4714671.7 |#############
  4730185.7 |#############
  4745699.7 |#############
  4761213.7 |
  4776727.7 |
  4792241.7 |#############
  4807755.7 |#############
  4823269.6 |########################################
  4838783.6 |
  4854297.6 |
  4869811.6 |
  (2 below, 4 above range)

rcm_half (n=18, range 4863270.4-5200907.8 ns)
  4863270.4 |####################
  4880152.3 |########################################
  4897034.2 |####################
  4913916.0 |
  4930797.9 |
  4947679.8 |########################################
  4964561.6 |
  4981443.5 |
  4998325.4 |
  5015207.3 |########################################
  5032089.1 |####################
  5048971.0 |
  5065852.9 |
  5082734.7 |
  5099616.6 |
  5116498.5 |########################################
  5133380.3 |
  5150262.2 |####################
  5167144.1 |####################
  5184025.9 |
  (2 below, 3 above range)

rcm_rev (n=18, range 4433681.0-4954445.2 ns)
  4433681.0 |
  4459719.2 |########################################
  4485757.4 |####################
  4511795.6 |####################
  4537833.8 |########################################
  4563872.0 |####################
  4589910.2 |####################
  4615948.4 |####################
  4641986.7 |
  4668024.9 |####################
  4694063.1 |########################################
  4720101.3 |
  4746139.5 |
  4772177.7 |
  4798215.9 |####################
  4824254.1 |
  4850292.4 |
  4876330.6 |
  4902368.8 |
  4928407.0 |
  (2 below, 3 above range)

rcm_scr (n=18, range 4658941.1-5165815.2 ns)
  4658941.1 |########################################
  4684284.8 |
  4709628.5 |########
  4734972.2 |################
  4760315.9 |################
  4785659.6 |########
  4811003.3 |
  4836347.0 |
  4861690.7 |
  4887034.4 |
  4912378.1 |########
  4937721.8 |########
  4963065.5 |########
  4988409.3 |
  5013753.0 |
  5039096.7 |
  5064440.4 |
  5089784.1 |
  5115127.8 |
  5140471.5 |
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=644.8% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=612.2% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=668.1% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=644.5% of algo (FFI overhead may distort results)
