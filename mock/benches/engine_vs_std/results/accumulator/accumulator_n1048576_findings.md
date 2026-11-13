# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine has the worst median (1.03 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std at 436.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std dominates: 37% faster than the next best (std_par)

std (436.52 us) leads std_par (598.84 us) by 37%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std beats baseline by 58% (significant)

std is -596.84 us (58%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### engine is an outlier: 2.4x slower than the field

engine (1.03 ms) is 2.4x the fastest (436.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_par shows alternating (throttle bounce) (autocorr -0.50)

std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: std** at 436522.6 ns median (-57.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.36x (fastest 436522.6 ns, slowest 1032105.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | accumulator, a transform feeding the append surface |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000003 |
| Counter frequency | 2300371984 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 1024761ns | 1034030ns | 911124ns | 974330ns | 1090993ns | base |
| engine_par | 1023191ns | 990232ns | 813279ns | 909724ns | 1160770ns | -0.15% |
| std | 437327ns | 438340ns | 423222ns | 431299ns | 445374ns | -57.32% |
| std_par | 601979ns | 600510ns | 527087ns | 571931ns | 643238ns | -41.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 1022729ns | 909613ns | 1088526ns | base | 1.025 |
| engine_par | 1020685ns | 811626ns | 1157308ns | -0.20% | 1.027 |
| std | 435410ns | 421865ns | 443014ns | -57.43% | 2.408 |
| std_par | 600141ns | 525336ns | 641461ns | -41.32% | 1.747 |

## Performance model

- Peak throughput: **2.486 Gops/s** (std; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.016 | 40.9% |
| engine_par | 1.061 | 42.7% |
| std | 2.402 | 96.6% |
| std_par | 1.751 | 70.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 1024761ns | 1024761ns | base |
| engine_par | 1023191ns | 1023191ns | -0.15% |
| std | 437327ns | 437327ns | -57.32% |
| std_par | 601979ns | 601979ns | -41.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 1032105ns | base | --- | [932361, 1100889] | --- | --- | --- | --- |
| engine_par | 988135ns | no significant difference | [-133561, +194231]ns | [823036, 1257532] | no | 1.0000 | 1.0000 | 0 |
| std | 436523ns | -596840.4ns (-57.8%) | [-666356, -482891]ns | [423439, 444929] | YES | 0.0117 | 0.0078 | 0 |
| std_par | 598836ns | -426451.2ns (-41.3%) | [-520298, -287660]ns | [557118, 644701] | YES | 0.0117 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | engine_par | std | std_par |
|---|---|---|---|---|---|
| 1 | 1 | 953343ns | -8.4% | -54.0% | -32.8% |
| 1 | 2 | 1085178ns | +9.1% | -60.1% | -48.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| std | -0.500 | moderate- |
| std_par | -0.500 | moderate- |

**Consistency summary:**

- **engine_par**: won 1/2, lost 1/2
- **std**: won 2/2, lost 0/2
- **std_par**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 4797226.7ns | 1022729.5ns | 469.1% | HIGH |
| engine_par | 4977838.5ns | 1020685.3ns | 487.7% | HIGH |
| std | 4717397.3ns | 435409.8ns | 1083.4% | HIGH |
| std_par | 4801192.9ns | 600140.9ns | 800.0% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 909612.8-1088525.7 ns)
  909612.8 |########################################
  918558.4 |
  927504.1 |########################################
  936449.7 |
  945395.4 |
  954341.0 |
  963286.7 |
  972232.3 |########################################
  981178.0 |
  990123.6 |
  999069.3 |
  1008014.9 |########################################
  1016960.6 |
  1025906.2 |
  1034851.8 |
  1043797.5 |
  1052743.1 |########################################
  1061688.8 |########################################
  1070634.4 |
  1079580.1 |
  (0 below, 2 above range)

engine_par (n=8, range 811626.1-1157308.2 ns)
  811626.1 |########################################
  828910.2 |
  846194.3 |
  863478.4 |
  880762.5 |
  898046.6 |
  915330.7 |####################
  932614.9 |
  949899.0 |
  967183.1 |####################
  984467.2 |####################
  1001751.3 |
  1019035.4 |
  1036319.5 |
  1053603.6 |
  1070887.7 |
  1088171.8 |
  1105455.9 |####################
  1122740.0 |
  1140024.1 |
  (0 below, 2 above range)

std (n=8, range 421864.7-443014.3 ns)
  421864.7 |########################################
  422922.2 |########################################
  423979.7 |
  425037.1 |
  426094.6 |
  427152.1 |
  428209.6 |
  429267.1 |
  430324.5 |
  431382.0 |########################################
  432439.5 |
  433497.0 |########################################
  434554.5 |
  435612.0 |
  436669.4 |
  437726.9 |########################################
  438784.4 |########################################
  439841.9 |
  440899.4 |
  441956.8 |
  (0 below, 2 above range)

std_par (n=8, range 525335.9-641461.2 ns)
  525335.9 |########################################
  531142.2 |
  536948.4 |
  542754.7 |
  548561.0 |
  554367.2 |########################################
  560173.5 |
  565979.7 |
  571786.0 |########################################
  577592.3 |########################################
  583398.5 |
  589204.8 |
  595011.1 |
  600817.3 |
  606623.6 |
  612429.8 |########################################
  618236.1 |
  624042.4 |
  629848.6 |
  635654.9 |########################################
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: bridge=466.4% of algo (FFI overhead may distort results)
- **engine_par**: bridge=498.0% of algo (FFI overhead may distort results)
- **std**: bridge=1080.9% of algo (FFI overhead may distort results)
- **std_par**: bridge=801.7% of algo (FFI overhead may distort results)
