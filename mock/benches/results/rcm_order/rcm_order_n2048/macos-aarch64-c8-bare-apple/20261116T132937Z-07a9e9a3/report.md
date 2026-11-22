# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (rcm_rev, rcm_adj) are a dead heat (<1%)

rcm_rev (3.97 ms) and rcm_adj (3.98 ms) differ by 0.12%, inside the noise, even though the wider field spreads 3.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### rcm_half shows alternating (throttle bounce) (autocorr -0.81)

rcm_half's per-pass series has lag-1 autocorrelation -0.81, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 3.4% of the fastest

All 4 variants sit between 3.97 ms and 4.11 ms - a 3.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: rcm_rev** at 3973393.8 ns median (-0.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.03x (fastest 3973393.8 ns, slowest 4106950.0 ns)

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
| rcm_adj | 4028583ns | 3983292ns | 3965622ns | 3980722ns | 4131856ns | base |
| rcm_half | 4093297ns | 4073446ns | 4021867ns | 4060020ns | 4178926ns | +1.61% |
| rcm_rev | 4008939ns | 3978592ns | 3947279ns | 3972600ns | 4094279ns | -0.49% |
| rcm_scr | 4143463ns | 4112554ns | 4076675ns | 4103302ns | 4237098ns | +2.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 4023182ns | 3960696ns | 4126314ns | base | 0.001 |
| rcm_half | 4088010ns | 4016718ns | 4173444ns | +1.61% | 0.001 |
| rcm_rev | 4003695ns | 3942350ns | 4089227ns | -0.48% | 0.001 |
| rcm_scr | 4138086ns | 4071243ns | 4231610ns | +2.86% | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 99.1% |
| rcm_half | 0.001 | 96.9% |
| rcm_rev | 0.001 | 99.2% |
| rcm_scr | 0.000 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 4028583ns | 4028583ns | base |
| rcm_half | 4093297ns | 4093297ns | +1.61% |
| rcm_rev | 4008939ns | 4008939ns | -0.49% |
| rcm_scr | 4143463ns | 4143463ns | +2.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 3978217ns | base | --- | [3969627, 4001242] | --- | --- | --- | --- |
| rcm_half | 4067865ns | +77881.2ns (+2.0%) | [+39494, +126433]ns | [4025938, 4120642] | YES | 0.0463 | 0.0309 | 0 |
| rcm_rev | 3973394ns | no significant difference | [-33596, +25877]ns | [3954952, 3997823] | no | 1.0000 | 1.0000 | 0 |
| rcm_scr | 4106950ns | +114154.2ns (+2.9%) | [+85612, +151225]ns | [4080398, 4132250] | YES | 0.0039 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 3969946ns | +2.9% | +0.4% | +3.4% |
| 1 | 2 | 3977488ns | +2.0% | -0.6% | +2.5% |
| 1 | 3 | 3998708ns | +3.2% | -1.3% | +2.6% |
| 2 | 1 | 3978946ns | +1.0% | +0.3% | +3.8% |
| 2 | 2 | 3979979ns | +3.3% | -0.1% | +3.4% |
| 2 | 3 | 3974138ns | +2.6% | -0.1% | +4.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.080 | ok |
| rcm_half | -0.809 | HIGH- (thermal bounce) |
| rcm_rev | -0.180 | ok |
| rcm_scr | 0.185 | ok |

**Consistency summary:**

- **rcm_half**: won 0/6, lost 6/6
- **rcm_rev**: won 3/6, lost 2/6
- **rcm_scr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 89269253.2ns | 4023181.7ns | 2218.9% | HIGH |
| rcm_half | 89250796.7ns | 4088009.7ns | 2183.2% | HIGH |
| rcm_rev | 89256290.5ns | 4003694.9ns | 2229.3% | HIGH |
| rcm_scr | 89323546.8ns | 4138086.1ns | 2158.6% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 3960695.8-4126313.9 ns)
  3960695.8 |##############################
  3968976.7 |########################################
  3977257.6 |##############################
  3985538.5 |##########
  3993819.4 |##########
  4002100.3 |##########
  4010381.2 |##########
  4018662.2 |
  4026943.1 |
  4035224.0 |
  4043504.9 |
  4051785.8 |
  4060066.7 |
  4068347.6 |
  4076628.5 |
  4084909.4 |
  4093190.3 |
  4101471.2 |
  4109752.1 |
  4118033.0 |
  (1 below, 3 above range)

rcm_half (n=18, range 4016718.1-4173443.8 ns)
  4016718.1 |##########################
  4024554.4 |########################################
  4032390.6 |#############
  4040226.9 |#############
  4048063.2 |
  4055899.5 |#############
  4063735.8 |
  4071572.1 |#############
  4079408.3 |#############
  4087244.6 |
  4095080.9 |
  4102917.2 |
  4110753.5 |##########################
  4118589.8 |#############
  4126426.1 |#############
  4134262.3 |
  4142098.6 |
  4149934.9 |#############
  4157771.2 |
  4165607.5 |
  (1 below, 2 above range)

rcm_rev (n=18, range 3942350.0-4089227.1 ns)
  3942350.0 |########################################
  3949693.9 |##########################
  3957037.7 |##########################
  3964381.6 |
  3971725.4 |##########################
  3979069.3 |#############
  3986413.1 |##########################
  3993757.0 |
  4001100.8 |#############
  4008444.7 |#############
  4015788.5 |
  4023132.4 |
  4030476.2 |
  4037820.1 |
  4045164.0 |
  4052507.8 |
  4059851.7 |
  4067195.5 |
  4074539.4 |
  4081883.2 |
  (1 below, 3 above range)

rcm_scr (n=18, range 4071243.0-4231609.7 ns)
  4071243.0 |########################################
  4079261.4 |##########
  4087279.7 |
  4095298.0 |##########
  4103316.4 |########################################
  4111334.7 |##########
  4119353.0 |
  4127371.4 |####################
  4135389.7 |
  4143408.0 |
  4151426.4 |##########
  4159444.7 |
  4167463.1 |
  4175481.4 |
  4183499.7 |
  4191518.1 |##########
  4199536.4 |
  4207554.7 |
  4215573.1 |
  4223591.4 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2243.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2193.4% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2245.4% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2172.7% of algo (FFI overhead may distort results)
