# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (rcm_rev, rcm_adj) are a dead heat (<1%)

rcm_rev (4.06 ms) and rcm_adj (4.07 ms) differ by 0.27%, inside the noise, even though the wider field spreads 3.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### rcm_adj shows alternating (throttle bounce) (autocorr -0.55)

rcm_adj's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 3.0% of the fastest

All 4 variants sit between 4.06 ms and 4.18 ms - a 3.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: rcm_rev** at 4062560.4 ns median (-0.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.03x (fastest 4062560.4 ns, slowest 4184973.0 ns)

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
| rcm_adj | 4087135ns | 4079262ns | 4060582ns | 4076388ns | 4116531ns | base |
| rcm_half | 4153074ns | 4140156ns | 4102775ns | 4135735ns | 4204232ns | +1.61% |
| rcm_rev | 4071013ns | 4067625ns | 4028747ns | 4061117ns | 4106990ns | -0.39% |
| rcm_scr | 4202571ns | 4190112ns | 4169040ns | 4187633ns | 4241742ns | +2.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 4082028ns | 4055565ns | 4111433ns | base | 0.001 |
| rcm_half | 4147980ns | 4097411ns | 4199297ns | +1.62% | 0.000 |
| rcm_rev | 4065834ns | 4023519ns | 4101949ns | -0.40% | 0.001 |
| rcm_scr | 4197510ns | 4163864ns | 4236925ns | +2.83% | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 98.8% |
| rcm_half | 0.000 | 97.3% |
| rcm_rev | 0.001 | 99.0% |
| rcm_scr | 0.000 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 4087135ns | 4087135ns | base |
| rcm_half | 4153074ns | 4153074ns | +1.61% |
| rcm_rev | 4071013ns | 4071013ns | -0.39% |
| rcm_scr | 4202571ns | 4202571ns | +2.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 4073619ns | base | --- | [4067973, 4078573] | --- | --- | --- | --- |
| rcm_half | 4134885ns | +58941.6ns (+1.4%) | [+40440, +79029]ns | [4120879, 4157306] | YES | 0.0002 | 0.0001 | 0 |
| rcm_rev | 4062560ns | no significant difference | [-31617, +6788]ns | [4047590, 4075890] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 4184973ns | +114654.1ns (+2.8%) | [+96158, +134646]ns | [4176498, 4208198] | YES | 0.0002 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 4074271ns | +1.5% | +0.0% | +4.8% |
| 1 | 2 | 4066858ns | +3.0% | -0.2% | +3.2% |
| 1 | 3 | 4075996ns | +1.7% | -1.3% | +2.6% |
| 2 | 1 | 4072083ns | +1.1% | +0.3% | +3.3% |
| 2 | 2 | 4076000ns | +1.4% | -0.6% | +2.6% |
| 2 | 3 | 4072967ns | +1.5% | -0.0% | +2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.546 | HIGH- (thermal bounce) |
| rcm_half | 0.024 | ok |
| rcm_rev | -0.497 | moderate- |
| rcm_scr | 0.029 | ok |

**Consistency summary:**

- **rcm_half**: won 0/6, lost 6/6
- **rcm_rev**: won 3/6, lost 1/6
- **rcm_scr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 89539500.2ns | 4082027.8ns | 2193.5% | HIGH |
| rcm_half | 89634615.5ns | 4147979.8ns | 2160.9% | HIGH |
| rcm_rev | 89525240.5ns | 4065834.5ns | 2201.9% | HIGH |
| rcm_scr | 89533051.2ns | 4197510.4ns | 2133.0% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 4055565.3-4111432.6 ns)
  4055565.3 |
  4058358.7 |
  4061152.0 |##########################
  4063945.4 |
  4066738.8 |##########################
  4069532.1 |##########################
  4072325.5 |##########################
  4075118.9 |########################################
  4077912.2 |#############
  4080705.6 |
  4083499.0 |#############
  4086292.3 |
  4089085.7 |
  4091879.1 |
  4094672.4 |
  4097465.8 |
  4100259.2 |
  4103052.5 |##########################
  4105845.9 |
  4108639.3 |
  (2 below, 1 above range)

rcm_half (n=18, range 4097411.1-4199296.5 ns)
  4097411.1 |
  4102505.4 |####################
  4107599.6 |
  4112693.9 |####################
  4117788.2 |########################################
  4122882.5 |####################
  4127976.7 |####################
  4133071.0 |########################################
  4138165.3 |
  4143259.5 |########################################
  4148353.8 |
  4153448.1 |########################################
  4158542.4 |
  4163636.6 |
  4168730.9 |
  4173825.2 |
  4178919.4 |####################
  4184013.7 |
  4189108.0 |####################
  4194202.2 |
  (2 below, 2 above range)

rcm_rev (n=18, range 4023519.5-4101948.6 ns)
  4023519.5 |####################
  4027440.9 |
  4031362.4 |####################
  4035283.8 |####################
  4039205.3 |
  4043126.8 |####################
  4047048.2 |########################################
  4050969.7 |
  4054891.1 |####################
  4058812.6 |####################
  4062734.0 |########################################
  4066655.5 |
  4070577.0 |####################
  4074498.4 |########################################
  4078419.9 |
  4082341.3 |####################
  4086262.8 |####################
  4090184.2 |
  4094105.7 |
  4098027.2 |
  (1 below, 2 above range)

rcm_scr (n=18, range 4163863.9-4236925.0 ns)
  4163863.9 |########################################
  4167516.9 |####################
  4171170.0 |####################
  4174823.0 |####################
  4178476.1 |########################################
  4182129.1 |####################
  4185782.2 |####################
  4189435.3 |####################
  4193088.3 |
  4196741.4 |####################
  4200394.4 |
  4204047.5 |####################
  4207700.5 |####################
  4211353.6 |
  4215006.7 |####################
  4218659.7 |
  4222312.8 |####################
  4225965.8 |
  4229618.9 |
  4233271.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2196.3% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2163.7% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2202.4% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2137.8% of algo (FFI overhead may distort results)
