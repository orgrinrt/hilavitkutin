# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.5% (fastest 33.79 ms to slowest 33.97 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### rcm_scr shows alternating (throttle bounce) (autocorr -0.87)

rcm_scr's per-pass series has lag-1 autocorrelation -0.87, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (177.15 us) is smaller than the fastest variant's own run-to-run std-dev (255.36 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_half** at 33789368.8 ns median (-0.2% vs baseline)
- Spread: 1.01x (fastest 33789368.8 ns, slowest 33966522.9 ns)

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
| rcm_adj | 33961461ns | 33881596ns | 33509850ns | 33790672ns | 34443451ns | base |
| rcm_half | 33824973ns | 33798098ns | 33540162ns | 33733917ns | 34103962ns | -0.40% |
| rcm_rev | 34009816ns | 33976083ns | 33598007ns | 33889639ns | 34395988ns | +0.14% |
| rcm_scr | 33902564ns | 33810558ns | 33523326ns | 33737440ns | 34339869ns | -0.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 33952016ns | 33500401ns | 34434394ns | base | 0.000 |
| rcm_half | 33815398ns | 33530589ns | 34094909ns | -0.40% | 0.000 |
| rcm_rev | 34000933ns | 33589986ns | 34387003ns | +0.14% | 0.000 |
| rcm_scr | 33892445ns | 33512347ns | 34329940ns | -0.18% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_adj; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.9% |
| rcm_half | 0.000 | 99.1% |
| rcm_rev | 0.000 | 98.6% |
| rcm_scr | 0.000 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 33961461ns | 33961461ns | base |
| rcm_half | 33824973ns | 33824973ns | -0.40% |
| rcm_rev | 34009816ns | 34009816ns | +0.14% |
| rcm_scr | 33902564ns | 33902564ns | -0.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 33872250ns | base | --- | [33588077, 34036783] | --- | --- | --- | --- |
| rcm_half | 33789369ns | no significant difference | [-480615, +210908]ns | [33628850, 33908283] | no | 0.7210 | 0.4807 | 0 |
| rcm_rev | 33966523ns | no significant difference | [-317279, +487615]ns | [33726583, 34133950] | no | 0.7210 | 0.4807 | 0 |
| rcm_scr | 33801581ns | no significant difference | [-258840, +297283]ns | [33605358, 34151917] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 34397725ns | -2.0% | -2.0% | -0.7% |
| 1 | 2 | 34391479ns | -1.4% | -1.4% | -2.5% |
| 1 | 3 | 33830246ns | +0.2% | +0.2% | +1.8% |
| 2 | 1 | 33811271ns | -0.1% | +1.8% | -0.9% |
| 2 | 2 | 33583125ns | +1.4% | +1.4% | +1.7% |
| 2 | 3 | 33593029ns | +0.2% | +1.3% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.493 | moderate+ |
| rcm_half | -0.574 | HIGH- (thermal bounce) |
| rcm_rev | 0.087 | ok |
| rcm_scr | -0.875 | HIGH- (thermal bounce) |

**Consistency summary:**

- **rcm_half**: won 3/6, lost 3/6
- **rcm_rev**: won 2/6, lost 4/6
- **rcm_scr**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 716350155.5ns | 33952016.2ns | 2109.9% | HIGH |
| rcm_half | 716357392.1ns | 33815397.9ns | 2118.4% | HIGH |
| rcm_rev | 716291781.2ns | 34000933.1ns | 2106.7% | HIGH |
| rcm_scr | 716301065.0ns | 33892445.4ns | 2113.5% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 33500401.4-34434393.8 ns)
  33500401.4 |##########################
  33547101.0 |########################################
  33593800.6 |
  33640500.3 |
  33687199.9 |
  33733899.5 |
  33780599.1 |##########################
  33827298.7 |#############
  33873998.3 |#############
  33920698.0 |#############
  33967397.6 |#############
  34014097.2 |##########################
  34060796.8 |
  34107496.4 |
  34154196.1 |
  34200895.7 |
  34247595.3 |
  34294294.9 |
  34340994.5 |
  34387694.1 |##########################
  (1 below, 2 above range)

rcm_half (n=18, range 33530588.9-34094909.0 ns)
  33530588.9 |########################################
  33558804.9 |#############
  33587020.9 |
  33615236.9 |
  33643452.9 |
  33671668.9 |##########################
  33699884.9 |#############
  33728100.9 |
  33756316.9 |#############
  33784532.9 |#############
  33812748.9 |
  33840964.9 |#############
  33869181.0 |#############
  33897397.0 |##########################
  33925613.0 |
  33953829.0 |
  33982045.0 |
  34010261.0 |#############
  34038477.0 |
  34066693.0 |##########################
  (1 below, 1 above range)

rcm_rev (n=18, range 33589986.1-34387002.8 ns)
  33589986.1 |####################
  33629836.9 |
  33669687.8 |########################################
  33709538.6 |
  33749389.4 |####################
  33789240.3 |
  33829091.1 |
  33868941.9 |########################################
  33908792.8 |####################
  33948643.6 |####################
  33988494.4 |####################
  34028345.3 |########################################
  34068196.1 |
  34108046.9 |
  34147897.8 |
  34187748.6 |####################
  34227599.4 |
  34267450.3 |####################
  34307301.1 |
  34347151.9 |
  (2 below, 3 above range)

rcm_scr (n=18, range 33512347.2-34329940.3 ns)
  33512347.2 |########################################
  33553226.9 |####################
  33594106.5 |####################
  33634986.2 |####################
  33675865.8 |
  33716745.5 |####################
  33757625.1 |####################
  33798504.8 |########################################
  33839384.4 |
  33880264.1 |
  33921143.8 |
  33962023.4 |####################
  34002903.1 |
  34043782.7 |
  34084662.4 |
  34125542.0 |########################################
  34166421.7 |
  34207301.3 |
  34248181.0 |####################
  34289060.6 |
  (2 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2114.6% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2119.9% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2108.7% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2119.0% of algo (FFI overhead may distort results)
