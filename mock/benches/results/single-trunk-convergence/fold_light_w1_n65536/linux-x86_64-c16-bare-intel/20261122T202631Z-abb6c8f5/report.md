# fold_light, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 87% faster than the next best (kernel_nway)

sequential (14.89 us) leads kernel_nway (27.87 us) by 87%, a clear separation rather than a photo finish. CV 107.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 37% (significant)

sequential is -12.89 us (37%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 3.9x slower than the field

std_parallel (58.55 us) is 3.9x the fastest (14.89 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 107.4%)

sequential wins on median (14.89 us) yet has the highest variance (CV 107.4%), while std_parallel is the steadiest (CV 13.5%, 58.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.64)

sequential's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.9x the fastest

Fastest sequential (14.89 us) to slowest std_parallel (58.55 us): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.9x its best-20%

sequential's best 20% of batches run at 11.95 us but its worst 20% at 46.34 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 14892.8 ns median (-57.5% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.93x (fastest 14892.8 ns, slowest 58552.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 1 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112795948 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 34671ns | 35723ns | 22071ns | 33147ns | 43257ns | base |
| head_tail_dyn | 32506ns | 31740ns | 25125ns | 29769ns | 40302ns | -6.24% |
| kernel_nway | 32827ns | 29150ns | 24130ns | 28587ns | 43534ns | -5.32% |
| kernel_seq | 30874ns | 29651ns | 23866ns | 28184ns | 38414ns | -10.95% |
| nway | 54288ns | 52242ns | 43182ns | 50076ns | 66159ns | +56.58% |
| sequential | 25376ns | 15097ns | 12109ns | 15393ns | 46983ns | -26.81% |
| std_parallel | 61718ns | 59094ns | 51543ns | 58431ns | 71737ns | +78.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 33709ns | 21609ns | 41577ns | base | 1.944 |
| head_tail_dyn | 31650ns | 24435ns | 39124ns | -6.11% | 2.071 |
| kernel_nway | 31884ns | 23601ns | 42100ns | -5.41% | 2.055 |
| kernel_seq | 30055ns | 23310ns | 37026ns | -10.84% | 2.181 |
| nway | 53698ns | 42753ns | 65382ns | +59.30% | 1.220 |
| sequential | 25032ns | 11948ns | 46338ns | -25.74% | 2.618 |
| std_parallel | 60960ns | 51097ns | 70475ns | +80.84% | 1.075 |

## Performance model

- Peak throughput: **5.485 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.870 | 34.1% |
| head_tail_dyn | 2.093 | 38.2% |
| kernel_nway | 2.352 | 42.9% |
| kernel_seq | 2.253 | 41.1% |
| nway | 1.266 | 23.1% |
| sequential | 4.401 | 80.2% |
| std_parallel | 1.119 | 20.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 34671ns | 34671ns | base |
| head_tail_dyn | 32506ns | 32506ns | -6.24% |
| kernel_nway | 32827ns | 32827ns | -5.32% |
| kernel_seq | 30874ns | 30874ns | -10.95% |
| nway | 54288ns | 54288ns | +56.58% |
| sequential | 25376ns | 25376ns | -26.81% |
| std_parallel | 61718ns | 61718ns | +78.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 35042ns | base | --- | [29760, 37885] | --- | --- | --- | --- |
| head_tail_dyn | 31314ns | -2967.0ns (-8.5%) | [-5010, -723]ns | [26003, 36647] | YES | 0.0309 | 0.0309 | 0 |
| kernel_nway | 27870ns | -3999.6ns (-11.4%) | [-5109, -1342]ns | [25068, 34637] | YES | 0.0151 | 0.0075 | 0 |
| kernel_seq | 29093ns | -5806.9ns (-16.6%) | [-6809, -2117]ns | [25076, 33022] | YES | 0.0309 | 0.0309 | 0 |
| nway | 51763ns | +18497.8ns (+52.8%) | [+15224, +22293]ns | [45845, 56294] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 14893ns | -12890.8ns (-36.8%) | [-16630, -2059]ns | [12204, 38493] | YES | 0.0309 | 0.0309 | 0 |
| std_parallel | 58552ns | +25227.9ns (+72.0%) | [+20974, +31284]ns | [56166, 64938] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 33299ns | -21.9% | -17.8% | -25.1% | +67.0% | -49.8% | +112.7% |
| 1 | 2 | 38303ns | -9.6% | -9.6% | -12.2% | +37.9% | -17.6% | +54.8% |
| 1 | 3 | 30527ns | +7.2% | -20.0% | -8.4% | +49.5% | -58.9% | +81.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.584 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.301 | moderate- |
| kernel_nway | -0.614 | HIGH- (thermal bounce) |
| kernel_seq | -0.588 | HIGH- (thermal bounce) |
| nway | -0.040 | ok |
| sequential | -0.638 | HIGH- (thermal bounce) |
| std_parallel | -0.049 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 34243.3ns | 33708.9ns | 101.6% | HIGH |
| head_tail_dyn | 32940.4ns | 31649.9ns | 104.1% | HIGH |
| kernel_nway | 32101.1ns | 31884.3ns | 100.7% | HIGH |
| kernel_seq | 30565.5ns | 30055.0ns | 101.7% | HIGH |
| nway | 57310.5ns | 53697.9ns | 106.7% | HIGH |
| sequential | 25975.2ns | 25031.6ns | 103.8% | HIGH |
| std_parallel | 64550.3ns | 60959.6ns | 105.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 21609.2-41576.8 ns)
  21609.2 |
  22607.5 |
  23605.9 |
  24604.3 |########################################
  25602.7 |
  26601.1 |
  27599.5 |
  28597.8 |#############
  29596.2 |
  30594.6 |#############
  31593.0 |#############
  32591.4 |
  33589.8 |#############
  34588.1 |##########################
  35586.5 |########################################
  36584.9 |
  37583.3 |
  38581.7 |#############
  39580.1 |#############
  40578.4 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 24435.0-39123.8 ns)
  24435.0 |########################################
  25169.5 |####################
  25903.9 |
  26638.4 |########################################
  27372.8 |####################
  28107.2 |
  28841.7 |
  29576.1 |
  30310.6 |####################
  31045.0 |
  31779.4 |####################
  32513.9 |
  33248.3 |########################################
  33982.8 |
  34717.2 |####################
  35451.6 |
  36186.1 |
  36920.5 |
  37655.0 |####################
  38389.4 |####################
  (2 below, 3 above range)

kernel_nway (n=18, range 23601.4-42099.7 ns)
  23601.4 |##########################
  24526.3 |########################################
  25451.2 |
  26376.1 |##########################
  27301.1 |#############
  28226.0 |#############
  29150.9 |
  30075.8 |
  31000.7 |
  31925.6 |
  32850.6 |########################################
  33775.5 |
  34700.4 |#############
  35625.3 |
  36550.2 |
  37475.1 |
  38400.1 |#############
  39325.0 |
  40249.9 |#############
  41174.8 |
  (1 below, 2 above range)

kernel_seq (n=18, range 23310.3-37025.5 ns)
  23310.3 |
  23996.1 |
  24681.8 |########################################
  25367.6 |
  26053.3 |##########
  26739.1 |##########
  27424.9 |
  28110.6 |##########
  28796.4 |##########
  29482.1 |##########
  30167.9 |
  30853.7 |
  31539.4 |##########
  32225.2 |##########
  32911.0 |##########
  33596.7 |
  34282.5 |##########
  34968.2 |
  35654.0 |
  36339.8 |
  (2 below, 3 above range)

nway (n=18, range 42752.9-65382.3 ns)
  42752.9 |
  43884.4 |########################################
  45015.8 |####################
  46147.3 |########################################
  47278.8 |
  48410.2 |
  49541.7 |
  50673.2 |########################################
  51804.6 |####################
  52936.1 |####################
  54067.6 |####################
  55199.1 |####################
  56330.5 |####################
  57462.0 |
  58593.5 |
  59724.9 |
  60856.4 |########################################
  61987.9 |
  63119.3 |
  64250.8 |
  (2 below, 2 above range)

sequential (n=18, range 11948.4-46338.2 ns)
  11948.4 |########################################
  13667.9 |
  15387.4 |#####
  17106.8 |
  18826.3 |
  20545.8 |#####
  22265.3 |
  23984.8 |#####
  25704.3 |
  27423.8 |
  29143.3 |
  30862.8 |
  32582.3 |
  34301.8 |
  36021.3 |
  37740.8 |#####
  39460.3 |#####
  41179.8 |
  42899.2 |#####
  44618.7 |#####
  (2 below, 2 above range)

std_parallel (n=18, range 51096.9-70475.3 ns)
  51096.9 |
  52065.8 |
  53034.7 |##########
  54003.7 |##########
  54972.6 |
  55941.5 |########################################
  56910.4 |##########
  57879.4 |
  58848.3 |##########
  59817.2 |
  60786.1 |
  61755.0 |####################
  62724.0 |
  63692.9 |
  64661.8 |####################
  65630.7 |
  66599.7 |
  67568.6 |##########
  68537.5 |
  69506.4 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=22.8% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.0% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=28.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=109.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: CV=20.3% (high variance, measurements may be unstable)
- **nway**: bridge=103.3% of algo (FFI overhead may distort results)
- **sequential**: CV=63.9% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **sequential**: bridge=106.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=105.9% of algo (FFI overhead may distort results)
