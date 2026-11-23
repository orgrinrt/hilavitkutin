# fold_heavy, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 67% faster than the next best (kernel_nway)

sequential (19.11 us) leads kernel_nway (31.93 us) by 67%, a clear separation rather than a photo finish. CV 118.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 4.4x slower than the field

std_parallel (84.89 us) is 4.4x the fastest (19.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 118.0%)

sequential wins on median (19.11 us) yet has the highest variance (CV 118.0%), while std_parallel is the steadiest (CV 8.1%, 84.89 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.62)

sequential's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.4x the fastest

Fastest sequential (19.11 us) to slowest std_parallel (84.89 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 18.48 us but its worst 20% at 65.98 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 19107.0 ns median (-50.9% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 4.44x (fastest 19107.0 ns, slowest 84891.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112746474 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 41297ns | 40188ns | 22749ns | 36824ns | 57282ns | base |
| head_tail_dyn | 47015ns | 46572ns | 24688ns | 45497ns | 60454ns | +13.84% |
| kernel_nway | 40226ns | 33219ns | 21785ns | 34294ns | 58346ns | -2.59% |
| kernel_seq | 45619ns | 46947ns | 29277ns | 44619ns | 55290ns | +10.47% |
| nway | 75605ns | 73654ns | 64928ns | 72088ns | 86218ns | +83.07% |
| sequential | 36774ns | 19248ns | 18632ns | 22988ns | 66525ns | -10.95% |
| std_parallel | 85361ns | 85482ns | 74991ns | 83515ns | 93316ns | +106.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 40557ns | 22439ns | 56260ns | base | 0.101 |
| head_tail_dyn | 45974ns | 24398ns | 58812ns | +13.36% | 0.089 |
| kernel_nway | 39457ns | 21493ns | 57169ns | -2.71% | 0.104 |
| kernel_seq | 44713ns | 28912ns | 53663ns | +10.25% | 0.092 |
| nway | 75069ns | 64487ns | 85580ns | +85.09% | 0.055 |
| sequential | 36475ns | 18483ns | 65976ns | -10.06% | 0.112 |
| std_parallel | 84528ns | 74444ns | 92080ns | +108.42% | 0.048 |

## Performance model

- Peak throughput: **0.222 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.105 | 47.5% |
| head_tail_dyn | 0.089 | 40.4% |
| kernel_nway | 0.128 | 57.9% |
| kernel_seq | 0.088 | 39.9% |
| nway | 0.056 | 25.3% |
| sequential | 0.214 | 96.7% |
| std_parallel | 0.048 | 21.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 41297ns | 41297ns | base |
| head_tail_dyn | 47015ns | 47015ns | +13.84% |
| kernel_nway | 40226ns | 40226ns | -2.59% |
| kernel_seq | 45619ns | 45619ns | +10.47% |
| nway | 75605ns | 75605ns | +83.07% |
| sequential | 36774ns | 36774ns | -10.95% |
| std_parallel | 85361ns | 85361ns | +106.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 38948ns | base | --- | [31019, 52308] | --- | --- | --- | --- |
| head_tail_dyn | 45768ns | no significant difference | [-805, +10650]ns | [42488, 53113] | no | 0.3568 | 0.2379 | 0 |
| kernel_nway | 31931ns | no significant difference | [-1711, +644]ns | [27854, 52652] | no | 0.5768 | 0.4807 | 0 |
| kernel_seq | 46294ns | no significant difference | [-3256, +11711]ns | [40759, 51036] | no | 0.8145 | 0.8145 | 0 |
| nway | 73148ns | +30826.0ns (+79.1%) | [+23504, +42091]ns | [68877, 79392] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 19107ns | no significant difference | [-13661, +4219]ns | [18626, 51426] | no | 0.3568 | 0.2379 | 0 |
| std_parallel | 84891ns | +42831.9ns (+110.0%) | [+31417, +51417]ns | [80121, 88329] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 31019ns | +39.2% | -7.8% | +41.3% | +134.5% | -24.4% | +170.1% |
| 1 | 2 | 45778ns | +11.9% | +4.7% | -0.4% | +50.9% | -24.2% | +85.8% |
| 1 | 3 | 54959ns | -17.2% | -1.4% | -8.8% | +44.5% | -65.2% | +54.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.012 | ok |
| head_tail_dyn | -0.613 | HIGH- (thermal bounce) |
| kernel_nway | -0.053 | ok |
| kernel_seq | -0.040 | ok |
| nway | -0.397 | moderate- |
| sequential | -0.618 | HIGH- (thermal bounce) |
| std_parallel | -0.236 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 41058.6ns | 40557.3ns | 101.2% | HIGH |
| head_tail_dyn | 45860.9ns | 45974.4ns | 99.8% | HIGH |
| kernel_nway | 39993.3ns | 39456.8ns | 101.4% | HIGH |
| kernel_seq | 45142.2ns | 44713.0ns | 101.0% | HIGH |
| nway | 76147.1ns | 75069.3ns | 101.4% | HIGH |
| sequential | 37275.5ns | 36475.2ns | 102.2% | HIGH |
| std_parallel | 85023.3ns | 84527.9ns | 100.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 22439.1-56260.1 ns)
  22439.1 |
  24130.1 |
  25821.2 |
  27512.2 |########################################
  29203.3 |####################
  30894.3 |########################################
  32585.4 |####################
  34276.4 |
  35967.5 |####################
  37658.5 |
  39349.6 |####################
  41040.6 |
  42731.7 |
  44422.7 |####################
  46113.8 |####################
  47804.8 |
  49495.9 |####################
  51186.9 |
  52878.0 |
  54569.0 |########################################
  (2 below, 3 above range)

head_tail_dyn (n=18, range 24398.3-58812.1 ns)
  24398.3 |
  26119.0 |
  27839.7 |
  29560.4 |
  31281.1 |
  33001.8 |
  34722.5 |##########
  36443.1 |
  38163.8 |##########
  39884.5 |##########
  41605.2 |
  43325.9 |########################################
  45046.6 |##########
  46767.3 |##########
  48488.0 |##########
  50208.7 |
  51929.4 |####################
  53650.1 |
  55370.8 |##########
  57091.5 |
  (2 below, 3 above range)

kernel_nway (n=18, range 21493.0-57168.9 ns)
  21493.0 |
  23276.8 |
  25060.6 |#############
  26844.4 |########################################
  28628.2 |##########################
  30412.0 |#############
  32195.8 |#############
  33979.6 |
  35763.4 |
  37547.2 |
  39331.0 |
  41114.7 |
  42898.5 |
  44682.3 |#############
  46466.1 |
  48249.9 |#############
  50033.7 |
  51817.5 |#############
  53601.3 |##########################
  55385.1 |
  (2 below, 3 above range)

kernel_seq (n=18, range 28912.3-53663.5 ns)
  28912.3 |
  30149.8 |
  31387.4 |
  32624.9 |#############
  33862.5 |
  35100.1 |
  36337.6 |
  37575.2 |
  38812.7 |########################################
  40050.3 |
  41287.9 |##########################
  42525.4 |
  43763.0 |#############
  45000.5 |#############
  46238.1 |#############
  47475.7 |##########################
  48713.2 |
  49950.8 |#############
  51188.3 |##########################
  52425.9 |#############
  (1 below, 2 above range)

nway (n=18, range 64487.2-85580.1 ns)
  64487.2 |####################
  65541.9 |
  66596.5 |
  67651.2 |##########
  68705.8 |########################################
  69760.4 |
  70815.1 |
  71869.7 |##########
  72924.4 |##########
  73979.0 |
  75033.7 |
  76088.3 |##########
  77142.9 |##########
  78197.6 |
  79252.2 |##############################
  80306.9 |
  81361.5 |
  82416.2 |
  83470.8 |
  84525.4 |
  (1 below, 3 above range)

sequential (n=18, range 18482.8-65975.9 ns)
  18482.8 |########################################
  20857.5 |
  23232.1 |
  25606.8 |
  27981.5 |####
  30356.1 |
  32730.8 |
  35105.4 |
  37480.1 |
  39854.7 |
  42229.4 |
  44604.0 |####
  46978.7 |
  49353.3 |####
  51728.0 |
  54102.6 |
  56477.3 |####
  58852.0 |
  61226.6 |####
  63601.3 |
  (1 below, 3 above range)

std_parallel (n=18, range 74444.5-92080.0 ns)
  74444.5 |
  75326.2 |
  76208.0 |
  77089.8 |####################
  77971.6 |########################################
  78853.4 |
  79735.1 |
  80616.9 |
  81498.7 |########################################
  82380.5 |####################
  83262.2 |
  84144.0 |####################
  85025.8 |########################################
  85907.6 |
  86789.4 |
  87671.1 |########################################
  88552.9 |########################################
  89434.7 |
  90316.5 |
  91198.2 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=32.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=102.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=27.3% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.4% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=37.0% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=104.7% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=22.0% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=101.1% of algo (FFI overhead may distort results)
- **nway**: bridge=102.0% of algo (FFI overhead may distort results)
- **sequential**: CV=61.8% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=99.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=98.9% of algo (FFI overhead may distort results)
