# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: rcm_rev** at 4666443.6 ns median (-2.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 4666443.6 ns, slowest 4949999.6 ns)

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
| Counter frequency | 2112757625 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 4816671ns | 4788051ns | 4617668ns | 4758568ns | 5003325ns | base |
| rcm_half | 4939999ns | 4953944ns | 4794210ns | 4922416ns | 5039267ns | +2.56% |
| rcm_rev | 4666212ns | 4670643ns | 4516193ns | 4648824ns | 4767304ns | -3.12% |
| rcm_scr | 4913644ns | 4897694ns | 4807527ns | 4870695ns | 5031125ns | +2.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 4812631ns | 4613576ns | 4999262ns | base | 0.000 |
| rcm_half | 4935980ns | 4790158ns | 5035166ns | +2.56% | 0.000 |
| rcm_rev | 4662217ns | 4512067ns | 4763365ns | -3.13% | 0.000 |
| rcm_scr | 4909718ns | 4803548ns | 5027274ns | +2.02% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 94.3% |
| rcm_half | 0.000 | 91.2% |
| rcm_rev | 0.000 | 96.7% |
| rcm_scr | 0.000 | 92.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 4816671ns | 4816671ns | base |
| rcm_half | 4939999ns | 4939999ns | +2.56% |
| rcm_rev | 4666212ns | 4666212ns | -3.12% |
| rcm_scr | 4913644ns | 4913644ns | +2.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 4784192ns | base | --- | [4685185, 4910609] | --- | --- | --- | --- |
| rcm_half | 4950000ns | +139137.2ns (+2.9%) | [+19831, +209105]ns | [4882796, 4979794] | YES | 0.0463 | 0.0309 | 0 |
| rcm_rev | 4666444ns | -81320.0ns (-1.7%) | [-321475, -34892]ns | [4608269, 4707715] | YES | 0.0463 | 0.0309 | 0 |
| rcm_scr | 4893520ns | no significant difference | [-21757, +221698]ns | [4835049, 4924024] | no | 0.0963 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 4776381ns | +4.8% | -4.7% | +0.7% |
| 1 | 2 | 5087352ns | -1.3% | -8.3% | -4.8% |
| 1 | 3 | 4853421ns | +1.9% | -6.6% | -0.3% |
| 2 | 1 | 4841106ns | +0.1% | -2.0% | +1.1% |
| 2 | 2 | 4632094ns | +3.5% | +1.6% | +6.7% |
| 2 | 3 | 4774612ns | +4.0% | -1.6% | +2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.013 | ok |
| rcm_half | 0.330 | moderate+ |
| rcm_rev | -0.173 | ok |
| rcm_scr | 0.457 | moderate+ |

**Consistency summary:**

- **rcm_half**: won 1/6, lost 4/6
- **rcm_rev**: won 5/6, lost 1/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 31158842.1ns | 4812630.7ns | 647.4% | HIGH |
| rcm_half | 30915480.9ns | 4935980.1ns | 626.3% | HIGH |
| rcm_rev | 31001379.4ns | 4662216.5ns | 664.9% | HIGH |
| rcm_scr | 30956893.6ns | 4909717.9ns | 630.5% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 4613575.6-4999261.7 ns)
  4613575.6 |########################################
  4632859.9 |####################
  4652144.2 |####################
  4671428.5 |
  4690712.8 |
  4709997.1 |####################
  4729281.5 |
  4748565.8 |####################
  4767850.1 |########################################
  4787134.4 |####################
  4806418.7 |
  4825703.0 |####################
  4844987.3 |########################################
  4864271.6 |
  4883555.9 |
  4902840.2 |
  4922124.5 |
  4941408.8 |
  4960693.1 |####################
  4979977.4 |########################################
  (1 below, 2 above range)

rcm_half (n=18, range 4790157.9-5035165.8 ns)
  4790157.9 |#############
  4802408.3 |#############
  4814658.7 |#############
  4826909.1 |
  4839159.5 |#############
  4851409.9 |
  4863660.3 |
  4875910.7 |
  4888161.1 |
  4900411.4 |
  4912661.8 |########################################
  4924912.2 |
  4937162.6 |#############
  4949413.0 |#############
  4961663.4 |##########################
  4973913.8 |##########################
  4986164.2 |
  4998414.6 |#############
  5010665.0 |#############
  5022915.4 |#############
  (1 below, 1 above range)

rcm_rev (n=18, range 4512066.8-4763364.7 ns)
  4512066.8 |
  4524631.7 |##########################
  4537196.6 |
  4549761.5 |#############
  4562326.4 |
  4574891.3 |
  4587456.2 |
  4600021.1 |#############
  4612586.0 |#############
  4625150.9 |
  4637715.8 |
  4650280.7 |#############
  4662845.5 |########################################
  4675410.4 |#############
  4687975.3 |#############
  4700540.2 |##########################
  4713105.1 |
  4725670.0 |
  4738234.9 |#############
  4750799.8 |
  (1 below, 3 above range)

rcm_scr (n=18, range 4803548.3-5027273.6 ns)
  4803548.3 |########################################
  4814734.5 |
  4825920.8 |########################################
  4837107.1 |########################################
  4848293.3 |
  4859479.6 |####################
  4870665.9 |
  4881852.1 |####################
  4893038.4 |########################################
  4904224.7 |####################
  4915410.9 |####################
  4926597.2 |####################
  4937783.5 |####################
  4948969.7 |
  4960156.0 |####################
  4971342.3 |
  4982528.5 |
  4993714.8 |
  5004901.1 |
  5016087.3 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=647.2% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=620.7% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=662.5% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=629.7% of algo (FFI overhead may distort results)
