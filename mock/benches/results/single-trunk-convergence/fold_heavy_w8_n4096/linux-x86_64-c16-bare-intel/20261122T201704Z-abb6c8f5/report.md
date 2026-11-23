# fold_heavy, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 63% faster than the next best (head_tail)

sequential (18.78 us) leads head_tail (30.66 us) by 63%, a clear separation rather than a photo finish. CV 117.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 6.0x slower than the field

std_parallel (111.73 us) is 6.0x the fastest (18.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 117.0%)

sequential wins on median (18.78 us) yet has the highest variance (CV 117.0%), while nway is the steadiest (CV 14.3%, 72.11 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail shows alternating (throttle bounce) (autocorr -0.54)

head_tail's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.0x the fastest

Fastest sequential (18.78 us) to slowest std_parallel (111.73 us): 6.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.4x its best-20%

sequential's best 20% of batches run at 18.46 us but its worst 20% at 63.36 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 18778.2 ns median (-38.7% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 5.95x (fastest 18778.2 ns, slowest 111733.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112765502 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 33020ns | 31568ns | 26598ns | 30881ns | 39439ns | base |
| head_tail_dyn | 47772ns | 44700ns | 34420ns | 43973ns | 60147ns | +44.68% |
| kernel_nway | 34717ns | 33240ns | 25753ns | 31755ns | 43642ns | +5.14% |
| kernel_seq | 55603ns | 53717ns | 31696ns | 51301ns | 74010ns | +68.39% |
| nway | 72927ns | 72831ns | 60615ns | 70240ns | 83112ns | +120.86% |
| sequential | 34643ns | 18925ns | 18608ns | 20475ns | 63913ns | +4.92% |
| std_parallel | 113252ns | 112196ns | 83204ns | 105217ns | 140329ns | +242.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 32209ns | 26154ns | 38231ns | base | 0.127 |
| head_tail_dyn | 46915ns | 33935ns | 58632ns | +45.66% | 0.087 |
| kernel_nway | 33689ns | 24886ns | 42149ns | +4.60% | 0.122 |
| kernel_seq | 54581ns | 31341ns | 72286ns | +69.46% | 0.075 |
| nway | 72230ns | 60055ns | 82338ns | +124.26% | 0.057 |
| sequential | 34355ns | 18462ns | 63361ns | +6.66% | 0.119 |
| std_parallel | 112680ns | 82864ns | 139627ns | +249.84% | 0.036 |

## Performance model

- Peak throughput: **0.222 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.134 | 60.2% |
| head_tail_dyn | 0.093 | 41.9% |
| kernel_nway | 0.126 | 56.7% |
| kernel_seq | 0.077 | 34.8% |
| nway | 0.057 | 25.6% |
| sequential | 0.218 | 98.3% |
| std_parallel | 0.037 | 16.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 33020ns | 33020ns | base |
| head_tail_dyn | 47772ns | 47772ns | +44.68% |
| kernel_nway | 34717ns | 34717ns | +5.14% |
| kernel_seq | 55603ns | 55603ns | +68.39% |
| nway | 72927ns | 72927ns | +120.86% |
| sequential | 34643ns | 34643ns | +4.92% |
| std_parallel | 113252ns | 113252ns | +242.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 30656ns | base | --- | [28939, 34215] | --- | --- | --- | --- |
| head_tail_dyn | 44022ns | +13106.2ns (+42.8%) | [+8502, +16261]ns | [42329, 51968] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 32570ns | no significant difference | [-753, +1789]ns | [29267, 36566] | no | 0.8145 | 0.8145 | 0 |
| kernel_seq | 53060ns | +22626.3ns (+73.8%) | [+18030, +27255]ns | [47006, 60160] | YES | 0.0002 | 0.0001 | 0 |
| nway | 72112ns | +37879.2ns (+123.6%) | [+35402, +42464]ns | [65789, 74071] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 18778ns | no significant difference | [-11652, +15760]ns | [18626, 45714] | no | 0.5768 | 0.4807 | 0 |
| std_parallel | 111733ns | +73649.5ns (+240.2%) | [+62354, +90091]ns | [96444, 122728] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 27518ns | +31.2% | -4.9% | +58.8% | +119.6% | -5.5% | +348.3% |
| 1 | 2 | 34494ns | +32.9% | +8.9% | +78.7% | +112.2% | -45.7% | +212.4% |
| 1 | 3 | 30509ns | +50.0% | +3.2% | +81.8% | +142.0% | -38.7% | +246.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.545 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.171 | ok |
| kernel_nway | -0.523 | HIGH- (thermal bounce) |
| kernel_seq | -0.389 | moderate- |
| nway | -0.143 | ok |
| sequential | -0.163 | ok |
| std_parallel | -0.109 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 33822.1ns | 32208.8ns | 105.0% | HIGH |
| head_tail_dyn | 46568.3ns | 46915.5ns | 99.3% | HIGH |
| kernel_nway | 34798.4ns | 33689.3ns | 103.3% | HIGH |
| kernel_seq | 55299.7ns | 54581.0ns | 101.3% | HIGH |
| nway | 72990.2ns | 72229.9ns | 101.1% | HIGH |
| sequential | 35266.5ns | 34355.3ns | 102.7% | HIGH |
| std_parallel | 110283.4ns | 112680.3ns | 97.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 26154.3-38231.0 ns)
  26154.3 |#############
  26758.2 |#############
  27362.0 |#############
  27965.8 |
  28569.7 |#############
  29173.5 |########################################
  29777.3 |#############
  30381.2 |
  30985.0 |#############
  31588.8 |#############
  32192.7 |
  32796.5 |
  33400.3 |
  34004.2 |########################################
  34608.0 |#############
  35211.8 |
  35815.7 |#############
  36419.5 |
  37023.3 |
  37627.2 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 33934.6-58632.0 ns)
  33934.6 |#############
  35169.5 |#############
  36404.4 |#############
  37639.2 |
  38874.1 |
  40109.0 |
  41343.8 |#############
  42578.7 |########################################
  43813.6 |##########################
  45048.4 |
  46283.3 |#############
  47518.2 |#############
  48753.1 |
  49987.9 |
  51222.8 |##########################
  52457.7 |#############
  53692.5 |#############
  54927.4 |
  56162.3 |
  57397.1 |
  (1 below, 2 above range)

kernel_nway (n=18, range 24886.5-42148.5 ns)
  24886.5 |####################
  25749.6 |
  26612.7 |####################
  27475.8 |
  28338.9 |####################
  29202.0 |########################################
  30065.1 |####################
  30928.2 |
  31791.3 |####################
  32654.4 |########################################
  33517.5 |
  34380.6 |####################
  35243.7 |
  36106.8 |########################################
  36969.9 |
  37833.0 |
  38696.1 |####################
  39559.2 |
  40422.3 |
  41285.4 |
  (2 below, 3 above range)

kernel_seq (n=18, range 31341.0-72286.4 ns)
  31341.0 |
  33388.3 |
  35435.6 |
  37482.8 |####################
  39530.1 |####################
  41577.4 |####################
  43624.7 |####################
  45671.9 |
  47719.2 |########################################
  49766.5 |####################
  51813.7 |########################################
  53861.0 |####################
  55908.3 |########################################
  57955.5 |
  60002.8 |
  62050.1 |####################
  64097.4 |####################
  66144.6 |
  68191.9 |
  70239.2 |
  (1 below, 3 above range)

nway (n=18, range 60055.5-82338.2 ns)
  60055.5 |####################
  61169.6 |
  62283.7 |
  63397.9 |
  64512.0 |##########
  65626.2 |##########
  66740.3 |
  67854.4 |
  68968.6 |
  70082.7 |##########
  71196.9 |##############################
  72311.0 |
  73425.1 |########################################
  74539.3 |
  75653.4 |####################
  76767.6 |
  77881.7 |
  78995.8 |
  80110.0 |
  81224.1 |
  (2 below, 2 above range)

sequential (n=18, range 18462.5-63360.9 ns)
  18462.5 |########################################
  20707.4 |
  22952.3 |
  25197.3 |
  27442.2 |
  29687.1 |
  31932.0 |####
  34177.0 |
  36421.9 |
  38666.8 |
  40911.7 |####
  43156.6 |
  45401.6 |
  47646.5 |
  49891.4 |
  52136.3 |
  54381.3 |
  56626.2 |####
  58871.1 |####
  61116.0 |
  (1 below, 3 above range)

std_parallel (n=18, range 82863.9-139627.0 ns)
  82863.9 |####################
  85702.0 |########################################
  88540.2 |
  91378.3 |
  94216.5 |####################
  97054.6 |####################
  99892.8 |####################
  102731.0 |####################
  105569.1 |
  108407.3 |####################
  111245.4 |####################
  114083.6 |########################################
  116921.7 |
  119759.9 |####################
  122598.0 |####################
  125436.2 |####################
  128274.4 |####################
  131112.5 |
  133950.7 |
  136788.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=99.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=22.8% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.0% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=21.6% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=102.6% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=30.5% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=100.1% of algo (FFI overhead may distort results)
- **nway**: bridge=101.5% of algo (FFI overhead may distort results)
- **sequential**: CV=64.0% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **sequential**: bridge=100.4% of algo (FFI overhead may distort results)
- **std_parallel**: CV=21.8% (high variance, measurements may be unstable)
- **std_parallel**: bridge=95.0% of algo (FFI overhead may distort results)
