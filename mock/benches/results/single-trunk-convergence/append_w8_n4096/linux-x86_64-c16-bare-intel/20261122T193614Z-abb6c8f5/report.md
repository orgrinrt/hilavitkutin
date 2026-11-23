# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 5.0x slower than the field

std_parallel (70.29 us) is 5.0x the fastest (14.10 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 80.3%)

sequential wins on median (14.10 us) yet has the highest variance (CV 80.3%), while nway is the steadiest (CV 8.1%, 57.25 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.66)

kernel_nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail, head_tail_dyn, kernel_seq, kernel_nway} vs {nway, std_parallel} (210% apart)

The field splits into a fast tier {sequential, head_tail, head_tail_dyn, kernel_seq, kernel_nway} and a slow tier {nway, std_parallel} with a 210% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.0x the fastest

Fastest sequential (14.10 us) to slowest std_parallel (70.29 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 9.77 us but its worst 20% at 34.14 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 14103.4 ns median (-6.0% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 4.98x (fastest 14103.4 ns, slowest 70289.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112862021 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 16041ns | 15456ns | 12018ns | 15051ns | 19537ns | base |
| head_tail_dyn | 17926ns | 16380ns | 12998ns | 15200ns | 24479ns | +11.75% |
| kernel_nway | 19306ns | 19152ns | 12936ns | 18372ns | 23893ns | +20.36% |
| kernel_seq | 16928ns | 16047ns | 12789ns | 15778ns | 20722ns | +5.53% |
| nway | 55919ns | 57865ns | 48012ns | 55922ns | 59870ns | +248.61% |
| sequential | 20137ns | 14309ns | 9918ns | 13841ns | 34690ns | +25.54% |
| std_parallel | 84852ns | 70496ns | 63215ns | 69210ns | 119134ns | +428.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15209ns | 11570ns | 18129ns | base | 0.269 |
| head_tail_dyn | 17047ns | 12561ns | 23222ns | +12.09% | 0.240 |
| kernel_nway | 18324ns | 12482ns | 22264ns | +20.49% | 0.224 |
| kernel_seq | 15976ns | 12276ns | 19038ns | +5.04% | 0.256 |
| nway | 55233ns | 47470ns | 58981ns | +263.17% | 0.074 |
| sequential | 19828ns | 9770ns | 34143ns | +30.37% | 0.207 |
| std_parallel | 84358ns | 63003ns | 118132ns | +454.67% | 0.049 |

## Performance model

- Peak throughput: **0.419 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.273 | 65.1% |
| head_tail_dyn | 0.269 | 64.2% |
| kernel_nway | 0.222 | 52.9% |
| kernel_seq | 0.265 | 63.3% |
| nway | 0.072 | 17.1% |
| sequential | 0.290 | 69.3% |
| std_parallel | 0.058 | 13.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 16041ns | 16041ns | base |
| head_tail_dyn | 17926ns | 17926ns | +11.75% |
| kernel_nway | 19306ns | 19306ns | +20.36% |
| kernel_seq | 16928ns | 16928ns | +5.53% |
| nway | 55919ns | 55919ns | +248.61% |
| sequential | 20137ns | 20137ns | +25.54% |
| std_parallel | 84852ns | 84852ns | +428.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 15011ns | base | --- | [13625, 16755] | --- | --- | --- | --- |
| head_tail_dyn | 15230ns | no significant difference | [-375, +1079]ns | [13003, 17223] | no | 0.7210 | 0.4807 | 0 |
| kernel_nway | 18472ns | +3006.8ns (+20.0%) | [+1809, +4166]ns | [16410, 19871] | YES | 0.0026 | 0.0013 | 0 |
| kernel_seq | 15441ns | no significant difference | [-1080, +2652]ns | [14608, 17031] | no | 0.9774 | 0.8145 | 0 |
| nway | 57248ns | +41395.0ns (+275.8%) | [+36766, +44434]ns | [53238, 57679] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 14103ns | no significant difference | [-3030, +12220]ns | [9884, 28106] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 70289ns | +54009.9ns (+359.8%) | [+51240, +72871]ns | [65665, 88455] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 12587ns | +3.9% | +39.8% | +25.8% | +356.6% | -20.4% | +497.5% |
| 1 | 2 | 16007ns | -7.4% | +22.0% | -8.7% | +259.2% | +8.4% | +327.7% |
| 1 | 3 | 16434ns | +4.8% | +5.5% | -4.7% | +193.0% | +40.4% | +312.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.112 | ok |
| head_tail_dyn | -0.006 | ok |
| kernel_nway | -0.659 | HIGH- (thermal bounce) |
| kernel_seq | -0.656 | HIGH- (thermal bounce) |
| nway | -0.168 | ok |
| sequential | -0.003 | ok |
| std_parallel | -0.122 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 68355.1ns | 15208.8ns | 449.4% | HIGH |
| head_tail_dyn | 73199.3ns | 17047.4ns | 429.4% | HIGH |
| kernel_nway | 84069.9ns | 18324.4ns | 458.8% | HIGH |
| kernel_seq | 77510.9ns | 15975.8ns | 485.2% | HIGH |
| nway | 111160.8ns | 55233.4ns | 201.3% | HIGH |
| sequential | 49933.7ns | 19828.2ns | 251.8% | HIGH |
| std_parallel | 104988.6ns | 84357.9ns | 124.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11570.2-18129.0 ns)
  11570.2 |
  11898.1 |#############
  12226.1 |
  12554.0 |
  12882.0 |#############
  13209.9 |#############
  13537.9 |
  13865.8 |########################################
  14193.7 |
  14521.7 |#############
  14849.6 |
  15177.6 |#############
  15505.5 |#############
  15833.4 |#############
  16161.4 |
  16489.3 |#############
  16817.3 |##########################
  17145.2 |
  17473.2 |
  17801.1 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 12561.4-23221.8 ns)
  12561.4 |########################################
  13094.5 |##########
  13627.5 |
  14160.5 |##########
  14693.5 |##########
  15226.5 |##########
  15759.6 |
  16292.6 |####################
  16825.6 |####################
  17358.6 |
  17891.6 |
  18424.7 |
  18957.7 |##########
  19490.7 |
  20023.7 |##########
  20556.7 |
  21089.8 |
  21622.8 |
  22155.8 |
  22688.8 |
  (2 below, 2 above range)

kernel_nway (n=18, range 12482.0-22263.8 ns)
  12482.0 |
  12971.1 |
  13460.2 |
  13949.3 |
  14438.4 |####################
  14927.4 |
  15416.5 |########################################
  15905.6 |####################
  16394.7 |####################
  16883.8 |####################
  17372.9 |####################
  17862.0 |
  18351.0 |########################################
  18840.1 |####################
  19329.2 |####################
  19818.3 |########################################
  20307.4 |
  20796.5 |
  21285.6 |####################
  21774.7 |####################
  (1 below, 2 above range)

kernel_seq (n=18, range 12275.7-19038.3 ns)
  12275.7 |#############
  12613.8 |
  12952.0 |#############
  13290.1 |
  13628.2 |#############
  13966.3 |
  14304.5 |#############
  14642.6 |##########################
  14980.7 |
  15318.9 |########################################
  15657.0 |#############
  15995.1 |#############
  16333.2 |#############
  16671.4 |
  17009.5 |
  17347.6 |########################################
  17685.8 |
  18023.9 |
  18362.0 |
  18700.1 |
  (1 below, 2 above range)

nway (n=18, range 47469.7-58980.8 ns)
  47469.7 |######
  48045.3 |
  48620.8 |######
  49196.4 |
  49771.9 |######
  50347.5 |
  50923.0 |
  51498.6 |
  52074.1 |
  52649.7 |
  53225.2 |
  53800.8 |
  54376.4 |
  54951.9 |
  55527.5 |
  56103.0 |####################
  56678.6 |######
  57254.1 |########################################
  57829.7 |######
  58405.2 |######
  (2 below, 1 above range)

sequential (n=18, range 9770.2-34142.7 ns)
   9770.2 |########################################
  10988.9 |
  12207.5 |
  13426.1 |
  14644.7 |
  15863.3 |
  17082.0 |#####
  18300.6 |
  19519.2 |#####
  20737.8 |
  21956.4 |
  23175.1 |
  24393.7 |#####
  25612.3 |
  26830.9 |
  28049.6 |#####
  29268.2 |
  30486.8 |#####
  31705.4 |#####
  32924.0 |#####
  (1 below, 2 above range)

std_parallel (n=18, range 63002.9-118132.4 ns)
  63002.9 |########################################
  65759.3 |##############################
  68515.8 |####################
  71272.3 |##########
  74028.8 |
  76785.2 |##########
  79541.7 |
  82298.2 |
  85054.7 |
  87811.2 |####################
  90567.6 |
  93324.1 |
  96080.6 |
  98837.1 |
  101593.5 |
  104350.0 |##########
  107106.5 |
  109863.0 |
  112619.4 |
  115375.9 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=451.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=36.0% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=462.0% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=22.3% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=455.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=522.6% of algo (FFI overhead may distort results)
- **nway**: bridge=184.8% of algo (FFI overhead may distort results)
- **sequential**: CV=57.1% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=248.5% of algo (FFI overhead may distort results)
- **std_parallel**: CV=33.3% (high variance, measurements may be unstable)
- **std_parallel**: bridge=123.0% of algo (FFI overhead may distort results)
