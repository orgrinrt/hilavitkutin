# fold_heavy, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn dominates: 32% faster than the next best (kernel_nway)

head_tail_dyn (35.08 us) leads kernel_nway (46.14 us) by 32%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn beats baseline by 34% (significant)

head_tail_dyn is -18.14 us (34%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 2.1x slower than the field

std_parallel (72.10 us) is 2.1x the fastest (35.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: head_tail_dyn** at 35081.2 ns median (-34.1% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.06x (fastest 35081.2 ns, slowest 72102.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 53772ns | 53819ns | 51082ns | 53560ns | 55435ns | base |
| head_tail_dyn | 35766ns | 35575ns | 33624ns | 35325ns | 37499ns | -33.49% |
| kernel_nway | 46870ns | 46775ns | 43486ns | 46181ns | 49594ns | -12.84% |
| kernel_seq | 53475ns | 53221ns | 52539ns | 53097ns | 54510ns | -0.55% |
| nway | 60326ns | 59196ns | 56962ns | 58978ns | 64028ns | +12.19% |
| sequential | 57576ns | 57583ns | 56857ns | 57450ns | 58124ns | +7.07% |
| std_parallel | 72784ns | 72475ns | 67506ns | 71247ns | 77730ns | +35.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 53278ns | 50593ns | 54956ns | base | 1.230 |
| head_tail_dyn | 35237ns | 33218ns | 36915ns | -33.86% | 1.860 |
| kernel_nway | 46192ns | 42839ns | 48858ns | -13.30% | 1.419 |
| kernel_seq | 53249ns | 52362ns | 54219ns | -0.05% | 1.231 |
| nway | 59739ns | 56374ns | 63441ns | +12.13% | 1.097 |
| sequential | 57397ns | 56688ns | 57955ns | +7.73% | 1.142 |
| std_parallel | 72382ns | 67115ns | 77320ns | +35.86% | 0.905 |

## Performance model

- Peak throughput: **1.973 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.230 | 62.4% |
| head_tail_dyn | 1.868 | 94.7% |
| kernel_nway | 1.421 | 72.0% |
| kernel_seq | 1.235 | 62.6% |
| nway | 1.118 | 56.7% |
| sequential | 1.142 | 57.9% |
| std_parallel | 0.909 | 46.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 53772ns | 53772ns | base |
| head_tail_dyn | 35766ns | 35766ns | -33.49% |
| kernel_nway | 46870ns | 46870ns | -12.84% |
| kernel_seq | 53475ns | 53475ns | -0.55% |
| nway | 60326ns | 60326ns | +12.19% |
| sequential | 57576ns | 57576ns | +7.07% |
| std_parallel | 72784ns | 72784ns | +35.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 53273ns | base | --- | [52788, 54121] | --- | --- | --- | --- |
| head_tail_dyn | 35081ns | -18137.5ns (-34.0%) | [-18558, -16994]ns | [34535, 35965] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 46135ns | -7300.0ns (-13.7%) | [-9744, -5979]ns | [44627, 47225] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 53048ns | no significant difference | [-910, +750]ns | [52679, 53490] | no | 0.4807 | 0.4807 | 0 |
| nway | 58606ns | +5079.2ns (+9.5%) | [+4004, +7254]ns | [58025, 59979] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 57392ns | +4341.7ns (+8.1%) | [+2971, +4690]ns | [57048, 57704] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 72102ns | +18047.9ns (+33.9%) | [+16310, +21183]ns | [69502, 73756] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 54296ns | -35.4% | -17.4% | -1.1% | +9.9% | +5.1% | +31.5% |
| 1 | 2 | 53448ns | -33.7% | -12.5% | -1.0% | +9.0% | +7.4% | +34.8% |
| 1 | 3 | 53035ns | -33.9% | -12.3% | -0.5% | +9.5% | +8.8% | +36.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.026 | ok |
| head_tail_dyn | -0.665 | HIGH- (thermal bounce) |
| kernel_nway | -0.239 | moderate- |
| kernel_seq | -0.095 | ok |
| nway | -0.106 | ok |
| sequential | -0.002 | ok |
| std_parallel | -0.054 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 112036.6ns | 53278.0ns | 210.3% | HIGH |
| head_tail_dyn | 98316.0ns | 35237.0ns | 279.0% | HIGH |
| kernel_nway | 110360.9ns | 46191.7ns | 238.9% | HIGH |
| kernel_seq | 106199.5ns | 53248.8ns | 199.4% | HIGH |
| nway | 119094.7ns | 59739.1ns | 199.4% | HIGH |
| sequential | 114910.6ns | 57397.0ns | 200.2% | HIGH |
| std_parallel | 136925.9ns | 72381.9ns | 189.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 50593.1-54956.3 ns)
  50593.1 |
  50811.2 |
  51029.4 |
  51247.5 |
  51465.7 |#############
  51683.9 |##########################
  51902.0 |
  52120.2 |
  52338.3 |
  52556.5 |#############
  52774.7 |#############
  52992.8 |########################################
  53211.0 |#############
  53429.1 |#############
  53647.3 |##########################
  53865.5 |
  54083.6 |
  54301.8 |#############
  54519.9 |
  54738.1 |##########################
  (1 below, 2 above range)

head_tail_dyn (n=18, range 33218.1-36914.6 ns)
  33218.1 |########################################
  33402.9 |
  33587.7 |
  33772.5 |
  33957.4 |
  34142.2 |####################
  34327.0 |
  34511.8 |
  34696.7 |########################################
  34881.5 |########################################
  35066.3 |########################################
  35251.1 |
  35436.0 |####################
  35620.8 |
  35805.6 |####################
  35990.4 |####################
  36175.3 |
  36360.1 |
  36544.9 |
  36729.7 |####################
  (2 below, 3 above range)

kernel_nway (n=18, range 42838.9-48857.6 ns)
  42838.9 |
  43139.8 |##########################
  43440.8 |#############
  43741.7 |
  44042.6 |#############
  44343.6 |
  44644.5 |
  44945.5 |#############
  45246.4 |#############
  45547.3 |
  45848.3 |##########################
  46149.2 |##########################
  46450.1 |#############
  46751.1 |
  47052.0 |#############
  47352.9 |########################################
  47653.9 |
  47954.8 |
  48255.8 |
  48556.7 |
  (1 below, 2 above range)

kernel_seq (n=18, range 52362.5-54219.5 ns)
  52362.5 |#############
  52455.3 |#############
  52548.2 |##########################
  52641.0 |
  52733.9 |########################################
  52826.7 |
  52919.6 |
  53012.4 |##########################
  53105.3 |
  53198.1 |#############
  53291.0 |#############
  53383.8 |#############
  53476.7 |#############
  53569.5 |
  53662.4 |
  53755.2 |
  53848.1 |
  53940.9 |#############
  54033.8 |
  54126.6 |
  (1 below, 3 above range)

nway (n=18, range 56373.6-63441.0 ns)
  56373.6 |
  56727.0 |##########
  57080.3 |
  57433.7 |
  57787.1 |########################################
  58140.4 |####################
  58493.8 |####################
  58847.2 |
  59200.6 |##########
  59553.9 |##########
  59907.3 |##########
  60260.7 |
  60614.0 |
  60967.4 |
  61320.8 |##########
  61674.1 |##########
  62027.5 |
  62380.9 |
  62734.2 |
  63087.6 |
  (2 below, 2 above range)

sequential (n=18, range 56687.5-57954.8 ns)
  56687.5 |##########################
  56750.9 |
  56814.2 |
  56877.6 |
  56941.0 |
  57004.3 |########################################
  57067.7 |#############
  57131.1 |
  57194.4 |
  57257.8 |#############
  57321.2 |#############
  57384.5 |##########################
  57447.9 |
  57511.3 |
  57574.6 |
  57638.0 |#############
  57701.4 |##########################
  57764.7 |
  57828.1 |
  57891.5 |
  (1 below, 4 above range)

std_parallel (n=18, range 67115.3-77320.1 ns)
  67115.3 |####################
  67625.5 |
  68135.8 |####################
  68646.0 |####################
  69156.2 |########################################
  69666.5 |####################
  70176.7 |####################
  70687.0 |
  71197.2 |
  71707.5 |####################
  72217.7 |########################################
  72727.9 |########################################
  73238.2 |
  73748.4 |
  74258.7 |####################
  74768.9 |########################################
  75279.2 |
  75789.4 |
  76299.6 |
  76809.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=209.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=278.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=234.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=199.3% of algo (FFI overhead may distort results)
- **nway**: bridge=201.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=200.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=191.6% of algo (FFI overhead may distort results)
