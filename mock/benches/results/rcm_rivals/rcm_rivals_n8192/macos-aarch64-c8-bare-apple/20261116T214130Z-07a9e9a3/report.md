# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (32.48 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_snake at 32.13 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.1% (fastest 32.13 ms to slowest 32.48 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### riv_spec shows alternating (throttle bounce) (autocorr -0.53)

riv_spec's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (352.79 us) is smaller than the fastest variant's own run-to-run std-dev (2.17 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_snake vs stability leader riv_scr (+0% speed for 3.3x steadier)

riv_snake is fastest (32.13 ms, CV 6.8%); riv_scr gives up 0.5% median for 3.3x lower variance (CV 2.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: riv_snake** at 32127895.8 ns median (-1.1% vs baseline)
- Spread: 1.01x (fastest 32127895.8 ns, slowest 32480687.5 ns)

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
| riv_rcm | 32585088ns | 32490052ns | 31759914ns | 32307142ns | 33414596ns | base |
| riv_rowm | 32937414ns | 32338781ns | 31639556ns | 32218437ns | 34664809ns | +1.08% |
| riv_scr | 32504610ns | 32295221ns | 31611622ns | 32256189ns | 33323737ns | -0.25% |
| riv_snake | 32968587ns | 32138012ns | 31541571ns | 31946305ns | 35215520ns | +1.18% |
| riv_spec | 32314422ns | 32145906ns | 31518789ns | 32003933ns | 33177972ns | -0.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 32574991ns | 31749972ns | 33405024ns | base | 0.000 |
| riv_rowm | 32927120ns | 31629826ns | 34654708ns | +1.08% | 0.000 |
| riv_scr | 32494066ns | 31602393ns | 33312764ns | -0.25% | 0.000 |
| riv_snake | 32958522ns | 31531619ns | 35205893ns | +1.18% | 0.000 |
| riv_spec | 32304310ns | 31508331ns | 33168035ns | -0.83% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 97.0% |
| riv_rowm | 0.000 | 97.5% |
| riv_scr | 0.000 | 97.6% |
| riv_snake | 0.000 | 98.1% |
| riv_spec | 0.000 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 32585088ns | 32585088ns | base |
| riv_rowm | 32937414ns | 32937414ns | +1.08% |
| riv_scr | 32504610ns | 32504610ns | -0.25% |
| riv_snake | 32968587ns | 32968587ns | +1.18% |
| riv_spec | 32314422ns | 32314422ns | -0.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 32480688ns | base | --- | [32047390, 32844858] | --- | --- | --- | --- |
| riv_rowm | 32327742ns | no significant difference | [-711298, +239469]ns | [31951225, 32936581] | no | 1.0000 | 0.8145 | 0 |
| riv_scr | 32286246ns | no significant difference | [-700558, +606310]ns | [32106900, 32915871] | no | 1.0000 | 1.0000 | 0 |
| riv_snake | 32127896ns | no significant difference | [-520442, +781396]ns | [31688006, 33276608] | no | 1.0000 | 1.0000 | 0 |
| riv_spec | 32135521ns | no significant difference | [-714127, +229242]ns | [31769210, 32523758] | no | 0.9515 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 32575133ns | -1.2% | -1.4% | +3.6% | +0.5% |
| 1 | 2 | 33937462ns | -2.8% | -5.1% | -6.7% | -5.4% |
| 1 | 3 | 32386242ns | -1.0% | -2.2% | -1.0% | -0.5% |
| 2 | 1 | 31896467ns | +1.2% | +3.7% | +1.3% | +0.5% |
| 2 | 2 | 32595046ns | -0.9% | -0.7% | -2.6% | +1.0% |
| 2 | 3 | 32624617ns | -0.1% | -0.1% | +4.0% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.086 | ok |
| riv_rowm | -0.510 | HIGH- (thermal bounce) |
| riv_scr | -0.333 | moderate- |
| riv_snake | -0.265 | moderate- |
| riv_spec | -0.534 | HIGH- (thermal bounce) |

**Consistency summary:**

- **riv_rowm**: won 5/6, lost 1/6
- **riv_scr**: won 4/6, lost 1/6
- **riv_snake**: won 3/6, lost 3/6
- **riv_spec**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 539141655.5ns | 32574991.4ns | 1655.1% | HIGH |
| riv_rowm | 537988807.6ns | 32927119.7ns | 1633.9% | HIGH |
| riv_scr | 537796445.6ns | 32494065.5ns | 1655.1% | HIGH |
| riv_snake | 539472672.9ns | 32958522.0ns | 1636.8% | HIGH |
| riv_spec | 538531703.7ns | 32304310.4ns | 1667.1% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 31749972.2-33405024.3 ns)
  31749972.2 |####################
  31832724.8 |########################################
  31915477.4 |####################
  31998230.0 |
  32080982.6 |####################
  32163735.2 |####################
  32246487.9 |####################
  32329240.5 |####################
  32411993.1 |
  32494745.7 |####################
  32577498.3 |########################################
  32660250.9 |
  32743003.5 |####################
  32825756.1 |
  32908508.7 |####################
  32991261.3 |
  33074013.9 |
  33156766.5 |
  33239519.1 |########################################
  33322271.7 |
  (1 below, 2 above range)

riv_rowm (n=18, range 31629826.4-34654707.6 ns)
  31629826.4 |########################################
  31781070.4 |#############
  31932314.5 |#############
  32083558.6 |#############
  32234802.6 |########################################
  32386046.7 |
  32537290.7 |##########################
  32688534.8 |
  32839778.9 |########################################
  32991022.9 |
  33142267.0 |#############
  33293511.1 |
  33444755.1 |
  33595999.2 |#############
  33747243.3 |
  33898487.3 |
  34049731.4 |
  34200975.4 |
  34352219.5 |
  34503463.6 |
  (1 below, 1 above range)

riv_scr (n=18, range 31602393.1-33312763.9 ns)
  31602393.1 |#############
  31687911.6 |#############
  31773430.1 |
  31858948.7 |
  31944467.2 |
  32029985.8 |##########################
  32115504.3 |########################################
  32201022.9 |#############
  32286541.4 |#############
  32372059.9 |
  32457578.5 |#############
  32543097.0 |#############
  32628615.6 |
  32714134.1 |
  32799652.6 |#############
  32885171.2 |
  32970689.7 |
  33056208.3 |#############
  33141726.8 |
  33227245.3 |#############
  (1 below, 3 above range)

riv_snake (n=18, range 31531619.5-35205893.0 ns)
  31531619.5 |########################################
  31715333.1 |########################################
  31899046.8 |#############
  32082760.5 |#############
  32266474.2 |##########################
  32450187.9 |
  32633901.5 |#############
  32817615.2 |
  33001328.9 |
  33185042.6 |
  33368756.2 |
  33552469.9 |
  33736183.6 |##########################
  33919897.3 |#############
  34103611.0 |
  34287324.6 |
  34471038.3 |
  34654752.0 |
  34838465.7 |
  35022179.4 |
  (2 below, 2 above range)

riv_spec (n=18, range 31508330.6-33168035.4 ns)
  31508330.6 |
  31591315.8 |########################################
  31674301.1 |####################
  31757286.3 |####################
  31840271.5 |
  31923256.8 |####################
  32006242.0 |####################
  32089227.3 |########################################
  32172212.5 |########################################
  32255197.7 |
  32338183.0 |####################
  32421168.2 |
  32504153.5 |
  32587138.7 |
  32670123.9 |####################
  32753109.2 |####################
  32836094.4 |####################
  32919079.7 |
  33002064.9 |
  33085050.2 |
  (2 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1655.5% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1662.0% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1665.4% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1673.4% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1672.2% of algo (FFI overhead may distort results)
