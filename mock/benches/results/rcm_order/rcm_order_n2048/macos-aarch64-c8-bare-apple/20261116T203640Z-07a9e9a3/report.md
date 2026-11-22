# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (rcm_rev, rcm_adj) are a dead heat (<1%)

rcm_rev (3.97 ms) and rcm_adj (4.01 ms) differ by 0.91%, inside the noise, even though the wider field spreads 3.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole field within 3.9% of the fastest

All 4 variants sit between 3.97 ms and 4.13 ms - a 3.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: rcm_rev** at 3974031.2 ns median (-0.9% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.04x (fastest 3974031.2 ns, slowest 4130033.3 ns)

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
| rcm_adj | 4048236ns | 4015362ns | 3963222ns | 4006378ns | 4153528ns | base |
| rcm_half | 4107170ns | 4090175ns | 4018938ns | 4073320ns | 4202062ns | +1.46% |
| rcm_rev | 4014263ns | 3979765ns | 3918001ns | 3978160ns | 4116547ns | -0.84% |
| rcm_scr | 4160450ns | 4135048ns | 4074122ns | 4126036ns | 4255235ns | +2.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 4042517ns | 3957535ns | 4147764ns | base | 0.001 |
| rcm_half | 4101564ns | 4013832ns | 4196116ns | +1.46% | 0.000 |
| rcm_rev | 4008683ns | 3912546ns | 4110921ns | -0.84% | 0.001 |
| rcm_scr | 4154757ns | 4069068ns | 4249103ns | +2.78% | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 97.6% |
| rcm_half | 0.001 | 95.8% |
| rcm_rev | 0.001 | 98.5% |
| rcm_scr | 0.000 | 94.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 4048236ns | 4048236ns | base |
| rcm_half | 4107170ns | 4107170ns | +1.46% |
| rcm_rev | 4014263ns | 4014263ns | -0.84% |
| rcm_scr | 4160450ns | 4160450ns | +2.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 4010060ns | base | --- | [3980340, 4062933] | --- | --- | --- | --- |
| rcm_half | 4085094ns | +64264.7ns (+1.6%) | [+3042, +116604]ns | [4034879, 4150885] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| rcm_rev | 3974031ns | no significant difference | [-130396, +56746]ns | [3958110, 4051612] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 4130033ns | +112508.4ns (+2.8%) | [+80075, +191146]ns | [4093954, 4203975] | YES (adj: no) | 0.0927 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 3968958ns | +3.8% | -0.2% | +3.1% |
| 1 | 2 | 4032633ns | +3.0% | -1.2% | +4.7% |
| 1 | 3 | 3993004ns | +1.9% | +1.0% | +4.8% |
| 2 | 1 | 4018167ns | +1.7% | -0.8% | +2.2% |
| 2 | 2 | 4035092ns | +1.9% | -3.2% | +1.8% |
| 2 | 3 | 3976746ns | +1.5% | -0.3% | +2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.463 | moderate- |
| rcm_half | -0.108 | ok |
| rcm_rev | 0.111 | ok |
| rcm_scr | 0.095 | ok |

**Consistency summary:**

- **rcm_half**: won 0/6, lost 6/6
- **rcm_rev**: won 5/6, lost 1/6
- **rcm_scr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 89404831.5ns | 4042517.4ns | 2211.6% | HIGH |
| rcm_half | 89392934.0ns | 4101563.7ns | 2179.5% | HIGH |
| rcm_rev | 89316182.4ns | 4008682.9ns | 2228.1% | HIGH |
| rcm_scr | 89408444.9ns | 4154756.7ns | 2152.0% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 3957534.7-4147763.9 ns)
  3957534.7 |
  3967046.2 |########################################
  3976557.6 |########################################
  3986069.1 |####################
  3995580.5 |########################################
  4005092.0 |
  4014603.5 |####################
  4024114.9 |####################
  4033626.4 |########################################
  4043137.8 |
  4052649.3 |
  4062160.8 |
  4071672.2 |
  4081183.7 |
  4090695.1 |####################
  4100206.6 |
  4109718.1 |
  4119229.5 |
  4128741.0 |####################
  4138252.4 |
  (2 below, 3 above range)

rcm_half (n=18, range 4013831.9-4196116.0 ns)
  4013831.9 |########################################
  4022946.1 |#############
  4032060.3 |#############
  4041174.5 |
  4050288.7 |
  4059402.9 |#############
  4068517.1 |#############
  4077631.3 |#############
  4086745.5 |#############
  4095859.7 |
  4104974.0 |#############
  4114088.2 |#############
  4123202.4 |
  4132316.6 |
  4141430.8 |#############
  4150545.0 |#############
  4159659.2 |
  4168773.4 |
  4177887.6 |
  4187001.8 |#############
  (1 below, 3 above range)

rcm_rev (n=18, range 3912545.8-4110920.8 ns)
  3912545.8 |
  3922464.6 |
  3932383.3 |#############
  3942302.1 |#############
  3952220.8 |########################################
  3962139.6 |##########################
  3972058.3 |
  3981977.1 |##########################
  3991895.8 |
  4001814.6 |
  4011733.3 |
  4021652.1 |#############
  4031570.8 |#############
  4041489.6 |
  4051408.3 |
  4061327.1 |#############
  4071245.8 |
  4081164.6 |
  4091083.3 |
  4101002.1 |#############
  (2 below, 3 above range)

rcm_scr (n=18, range 4069068.1-4249103.5 ns)
  4069068.1 |########################################
  4078069.8 |####################
  4087071.6 |####################
  4096073.4 |########################################
  4105075.1 |########################################
  4114076.9 |
  4123078.7 |
  4132080.5 |
  4141082.2 |
  4150084.0 |########################################
  4159085.8 |
  4168087.5 |
  4177089.3 |########################################
  4186091.1 |
  4195092.8 |
  4204094.6 |
  4213096.4 |
  4222098.2 |########################################
  4231099.9 |####################
  4240101.7 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2226.7% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2186.2% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2247.3% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2162.9% of algo (FFI overhead may distort results)
