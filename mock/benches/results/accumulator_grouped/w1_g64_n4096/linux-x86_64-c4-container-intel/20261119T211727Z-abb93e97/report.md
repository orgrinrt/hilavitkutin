# W = 1, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (35.06 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest per_core at 28.11 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### per_core beats baseline by 24% (significant)

per_core is -8.27 us (24%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader per_core vs stability leader sequential (+10% speed for 1.1x steadier)

per_core is fastest (28.11 us, CV 8.0%); sequential gives up 9.7% median for 1.1x lower variance (CV 7.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: per_core** at 28109.8 ns median (-19.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.25x (fastest 28109.8 ns, slowest 35061.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 1, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 36456ns | 35760ns | 32562ns | 35377ns | 40023ns | base |
| per_core | 28979ns | 28571ns | 26277ns | 28027ns | 31759ns | -20.51% |
| sequential | 32161ns | 31607ns | 29589ns | 31170ns | 34934ns | -11.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 35867ns | 31893ns | 39509ns | base | 0.114 |
| per_core | 28317ns | 25680ns | 30907ns | -21.05% | 0.145 |
| sequential | 31588ns | 29137ns | 34305ns | -11.93% | 0.130 |

## Performance model

- Peak throughput: **0.159 Gops/s** (per_core; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.117 | 73.2% |
| per_core | 0.146 | 91.4% |
| sequential | 0.133 | 83.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 36456ns | 36456ns | base |
| per_core | 28979ns | 28979ns | -20.51% |
| sequential | 32161ns | 32161ns | -11.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 35061ns | base | --- | [34316, 36924] | --- | --- | --- | --- |
| per_core | 28110ns | -8274.4ns (-23.6%) | [-9868, -4336]ns | [26579, 29112] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 30830ns | -4428.5ns (-12.6%) | [-5540, -2200]ns | [29687, 33150] | YES | 0.0013 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 36180ns | -26.6% | -10.9% |
| 1 | 2 | 35037ns | -22.3% | -14.5% |
| 1 | 3 | 33993ns | -11.0% | -5.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.000 | ok |
| per_core | -0.080 | ok |
| sequential | -0.660 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 36151.4ns | 35867.2ns | 100.8% | HIGH |
| per_core | 32083.1ns | 28317.1ns | 113.3% | HIGH |
| sequential | 32194.8ns | 31587.8ns | 101.9% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 31893.2-39509.1 ns)
  31893.2 |#############
  32274.0 |
  32654.8 |##########################
  33035.6 |
  33416.4 |
  33797.2 |#############
  34178.0 |
  34558.8 |##########################
  34939.6 |########################################
  35320.4 |#############
  35701.2 |#############
  36082.0 |
  36462.8 |#############
  36843.5 |
  37224.3 |##########################
  37605.1 |
  37985.9 |#############
  38366.7 |
  38747.5 |
  39128.3 |
  (1 below, 2 above range)

per_core (n=18, range 25680.4-30907.4 ns)
  25680.4 |
  25941.7 |########################################
  26203.1 |####################
  26464.4 |########################################
  26725.8 |####################
  26987.1 |
  27248.5 |
  27509.8 |####################
  27771.2 |
  28032.5 |########################################
  28293.9 |####################
  28555.2 |####################
  28816.6 |####################
  29077.9 |####################
  29339.3 |
  29600.6 |
  29862.0 |
  30123.3 |####################
  30384.7 |
  30646.0 |
  (1 below, 3 above range)

sequential (n=18, range 29137.4-34304.6 ns)
  29137.4 |####################
  29395.7 |########################################
  29654.1 |####################
  29912.4 |
  30170.8 |
  30429.2 |########################################
  30687.5 |########################################
  30945.9 |
  31204.3 |####################
  31462.6 |
  31721.0 |
  31979.3 |
  32237.7 |
  32496.1 |####################
  32754.4 |
  33012.8 |########################################
  33271.1 |
  33529.5 |
  33787.9 |
  34046.2 |
  (2 below, 4 above range)

```

## Diagnostics

- **grouped**: bridge=103.4% of algo (FFI overhead may distort results)
- **per_core**: bridge=97.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=105.0% of algo (FFI overhead may distort results)
