# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.7% (fastest 4.09 ms to slowest 4.16 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (67.67 us) is smaller than the fastest variant's own run-to-run std-dev (74.10 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_rev** at 4092608.3 ns median (-0.4% vs baseline)
- Spread: 1.02x (fastest 4092608.3 ns, slowest 4160281.2 ns)

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
| rcm_adj | 4170425ns | 4114031ns | 4022851ns | 4101226ns | 4348010ns | base |
| rcm_half | 4162202ns | 4109698ns | 4025232ns | 4099781ns | 4324319ns | -0.20% |
| rcm_rev | 4097223ns | 4097894ns | 4011529ns | 4076639ns | 4170946ns | -1.76% |
| rcm_scr | 4178051ns | 4166292ns | 4100079ns | 4156199ns | 4249815ns | +0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 4164279ns | 4017167ns | 4341548ns | base | 0.000 |
| rcm_half | 4156195ns | 4019626ns | 4318483ns | -0.19% | 0.000 |
| rcm_rev | 4091227ns | 4005656ns | 4164808ns | -1.75% | 0.001 |
| rcm_scr | 4171947ns | 4093683ns | 4243560ns | +0.18% | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 97.5% |
| rcm_half | 0.000 | 97.6% |
| rcm_rev | 0.001 | 97.9% |
| rcm_scr | 0.000 | 96.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 4170425ns | 4170425ns | base |
| rcm_half | 4162202ns | 4162202ns | -0.20% |
| rcm_rev | 4097223ns | 4097223ns | -1.76% |
| rcm_scr | 4178051ns | 4178051ns | +0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 4107946ns | base | --- | [4063240, 4163665] | --- | --- | --- | --- |
| rcm_half | 4103719ns | no significant difference | [-45710, +105960]ns | [4075196, 4210100] | no | 1.0000 | 1.0000 | 0 |
| rcm_rev | 4092608ns | no significant difference | [-70452, +28435]ns | [4038040, 4116590] | no | 1.0000 | 1.0000 | 0 |
| rcm_scr | 4160281ns | no significant difference | [-21242, +109671]ns | [4127527, 4185008] | no | 1.0000 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 4157054ns | -1.2% | +0.2% | -0.5% |
| 1 | 2 | 4040112ns | +0.8% | +0.9% | +3.4% |
| 1 | 3 | 4515358ns | -9.7% | -11.1% | -8.1% |
| 2 | 1 | 4057117ns | +4.8% | +0.9% | +2.9% |
| 2 | 2 | 4081929ns | +0.8% | -0.5% | +3.3% |
| 2 | 3 | 4101396ns | -0.5% | -0.2% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.411 | moderate- |
| rcm_half | -0.161 | ok |
| rcm_rev | -0.090 | ok |
| rcm_scr | -0.423 | moderate- |

**Consistency summary:**

- **rcm_half**: won 3/6, lost 3/6
- **rcm_rev**: won 3/6, lost 3/6
- **rcm_scr**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 90706074.5ns | 4164279.2ns | 2178.2% | HIGH |
| rcm_half | 89553854.4ns | 4156194.7ns | 2154.7% | HIGH |
| rcm_rev | 89686028.5ns | 4091227.3ns | 2192.2% | HIGH |
| rcm_scr | 89449028.2ns | 4171946.5ns | 2144.1% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 4017166.7-4341547.9 ns)
  4017166.7 |
  4033385.7 |#############
  4049604.8 |##########################
  4065823.9 |########################################
  4082042.9 |
  4098262.0 |#############
  4114481.0 |#############
  4130700.1 |#############
  4146919.2 |##########################
  4163138.2 |#############
  4179357.3 |
  4195576.4 |
  4211795.4 |
  4228014.5 |
  4244233.5 |
  4260452.6 |
  4276671.7 |
  4292890.7 |#############
  4309109.8 |
  4325328.9 |#############
  (2 below, 2 above range)

rcm_half (n=18, range 4019626.4-4318483.3 ns)
  4019626.4 |#############
  4034569.2 |#############
  4049512.1 |
  4064454.9 |########################################
  4079397.8 |##########################
  4094340.6 |##########################
  4109283.5 |#############
  4124226.3 |
  4139169.2 |#############
  4154112.0 |
  4169054.9 |#############
  4183997.7 |
  4198940.6 |
  4213883.4 |
  4228826.3 |
  4243769.1 |#############
  4258711.9 |
  4273654.8 |
  4288597.6 |
  4303540.5 |#############
  (1 below, 3 above range)

rcm_rev (n=18, range 4005655.6-4164808.3 ns)
  4005655.6 |####################
  4013613.2 |####################
  4021570.8 |
  4029528.5 |########################################
  4037486.1 |####################
  4045443.8 |
  4053401.4 |####################
  4061359.0 |
  4069316.7 |
  4077274.3 |####################
  4085232.0 |########################################
  4093189.6 |####################
  4101147.2 |####################
  4109104.9 |####################
  4117062.5 |####################
  4125020.1 |########################################
  4132977.8 |
  4140935.4 |
  4148893.1 |
  4156850.7 |
  (1 below, 2 above range)

rcm_scr (n=18, range 4093683.3-4243560.4 ns)
  4093683.3 |####################
  4101177.2 |
  4108671.0 |####################
  4116164.9 |########################################
  4123658.7 |
  4131152.6 |####################
  4138646.5 |
  4146140.3 |####################
  4153634.2 |########################################
  4161128.0 |####################
  4168621.9 |####################
  4176115.7 |####################
  4183609.6 |########################################
  4191103.4 |
  4198597.3 |
  4206091.1 |
  4213585.0 |####################
  4221078.8 |
  4228572.7 |####################
  4236066.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2178.5% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2178.9% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2184.8% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2149.8% of algo (FFI overhead may distort results)
