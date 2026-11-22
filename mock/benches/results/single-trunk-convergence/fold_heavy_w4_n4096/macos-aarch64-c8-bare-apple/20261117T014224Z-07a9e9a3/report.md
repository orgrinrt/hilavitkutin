# fold_heavy, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 10.3x slower than the field

std_parallel (62.36 us) is 10.3x the fastest (6.08 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.3x the fastest

Fastest kernel_seq (6.08 us) to slowest std_parallel (62.36 us): 10.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_seq** at 6081.2 ns median (-13.8% vs baseline)
- 2 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 10.25x (fastest 6081.2 ns, slowest 62356.2 ns)

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
| head_tail | 7789ns | 7677ns | 7251ns | 7651ns | 8264ns | base |
| head_tail_dyn | 7433ns | 7185ns | 6876ns | 7141ns | 8149ns | -4.57% |
| kernel_nway | 21771ns | 21910ns | 17756ns | 21413ns | 24315ns | +179.52% |
| kernel_seq | 6517ns | 6548ns | 5844ns | 6375ns | 7067ns | -16.33% |
| nway | 40644ns | 39858ns | 34838ns | 38836ns | 46260ns | +421.84% |
| sequential | 13166ns | 13125ns | 12678ns | 13048ns | 13586ns | +69.03% |
| std_parallel | 62499ns | 62788ns | 54032ns | 60921ns | 69099ns | +702.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 7128ns | 6586ns | 7584ns | base | 0.575 |
| head_tail_dyn | 6811ns | 6294ns | 7488ns | -4.45% | 0.601 |
| kernel_nway | 21206ns | 17139ns | 23684ns | +197.51% | 0.193 |
| kernel_seq | 5994ns | 5419ns | 6452ns | -15.92% | 0.683 |
| nway | 40145ns | 34278ns | 45790ns | +463.20% | 0.102 |
| sequential | 13018ns | 12524ns | 13435ns | +82.63% | 0.315 |
| std_parallel | 62044ns | 53612ns | 68610ns | +770.42% | 0.066 |

## Performance model

- Peak throughput: **0.756 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.581 | 76.8% |
| head_tail_dyn | 0.625 | 82.7% |
| kernel_nway | 0.192 | 25.5% |
| kernel_seq | 0.674 | 89.1% |
| nway | 0.104 | 13.8% |
| sequential | 0.316 | 41.8% |
| std_parallel | 0.066 | 8.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 7789ns | 7789ns | base |
| head_tail_dyn | 7433ns | 7433ns | -4.57% |
| kernel_nway | 21771ns | 21771ns | +179.52% |
| kernel_seq | 6517ns | 6517ns | -16.33% |
| nway | 40644ns | 40644ns | +421.84% |
| sequential | 13166ns | 13166ns | +69.03% |
| std_parallel | 62499ns | 62499ns | +702.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 7052ns | base | --- | [6867, 7329] | --- | --- | --- | --- |
| head_tail_dyn | 6550ns | -322.9ns (-4.6%) | [-646, -146]ns | [6438, 6773] | YES | 0.0309 | 0.0309 | 0 |
| kernel_nway | 21294ns | +14118.8ns (+200.2%) | [+13135, +15277]ns | [20467, 22362] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 6081ns | -933.3ns (-13.2%) | [-1548, -654]ns | [5596, 6248] | YES | 0.0000 | 0.0000 | 0 |
| nway | 39375ns | +32495.8ns (+460.8%) | [+29629, +33906]ns | [36717, 41146] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12979ns | +5752.0ns (+81.6%) | [+5629, +6221]ns | [12812, 13173] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 62356ns | +55456.2ns (+786.4%) | [+51425, +56967]ns | [57810, 64152] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 7208ns | +1.5% | +221.0% | -24.1% | +453.9% | +79.3% | +777.7% |
| 1 | 2 | 6867ns | -4.7% | +192.8% | -9.1% | +513.5% | +85.3% | +827.7% |
| 1 | 3 | 7171ns | -11.5% | +201.8% | -14.1% | +387.7% | +82.0% | +682.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.660 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.071 | ok |
| kernel_nway | -0.503 | HIGH- (thermal bounce) |
| kernel_seq | -0.228 | moderate- |
| nway | -0.362 | moderate- |
| sequential | -0.572 | HIGH- (thermal bounce) |
| std_parallel | -0.197 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 90661.8ns | 7128.0ns | 1271.9% | HIGH |
| head_tail_dyn | 87079.6ns | 6811.1ns | 1278.5% | HIGH |
| kernel_nway | 108985.9ns | 21206.2ns | 513.9% | HIGH |
| kernel_seq | 91313.9ns | 5993.5ns | 1523.5% | HIGH |
| nway | 109397.2ns | 40145.1ns | 272.5% | HIGH |
| sequential | 91113.0ns | 13018.0ns | 699.9% | HIGH |
| std_parallel | 125658.3ns | 62043.5ns | 202.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 6586.1-7584.1 ns)
   6586.1 |#############
   6636.0 |
   6685.9 |#############
   6735.8 |
   6785.7 |#############
   6835.6 |##########################
   6885.5 |
   6935.4 |#############
   6985.3 |##########################
   7035.2 |
   7085.1 |##########################
   7135.0 |
   7184.9 |
   7234.8 |
   7284.7 |########################################
   7334.6 |#############
   7384.5 |
   7434.4 |
   7484.3 |
   7534.2 |#############
  (1 below, 2 above range)

head_tail_dyn (n=18, range 6294.5-7488.2 ns)
   6294.5 |##########################
   6354.2 |#############
   6413.8 |
   6473.5 |##########################
   6533.2 |########################################
   6592.9 |
   6652.6 |#############
   6712.3 |##########################
   6772.0 |#############
   6831.6 |
   6891.3 |#############
   6951.0 |
   7010.7 |
   7070.4 |
   7130.1 |
   7189.8 |
   7249.5 |
   7309.1 |
   7368.8 |
   7428.5 |
  (2 below, 3 above range)

kernel_nway (n=18, range 17138.9-23684.0 ns)
  17138.9 |
  17466.1 |##########
  17793.4 |
  18120.6 |
  18447.9 |
  18775.2 |##########
  19102.4 |
  19429.7 |
  19756.9 |##########
  20084.2 |
  20411.4 |########################################
  20738.7 |##########
  21066.0 |
  21393.2 |####################
  21720.5 |
  22047.7 |####################
  22375.0 |##########
  22702.2 |##########
  23029.5 |
  23356.8 |
  (1 below, 3 above range)

kernel_seq (n=18, range 5419.4-6452.1 ns)
   5419.4 |####################
   5471.1 |####################
   5522.7 |####################
   5574.3 |
   5626.0 |####################
   5677.6 |
   5729.2 |
   5780.9 |####################
   5832.5 |
   5884.1 |
   5935.8 |####################
   5987.4 |####################
   6039.0 |
   6090.6 |########################################
   6142.3 |########################################
   6193.9 |
   6245.5 |
   6297.2 |########################################
   6348.8 |####################
   6400.4 |####################
  (2 below, 1 above range)

nway (n=18, range 34277.8-45789.6 ns)
  34277.8 |########################################
  34853.4 |####################
  35428.9 |
  36004.5 |
  36580.1 |########################################
  37155.7 |
  37731.3 |
  38306.9 |####################
  38882.5 |########################################
  39458.1 |####################
  40033.7 |########################################
  40609.3 |####################
  41184.8 |
  41760.4 |########################################
  42336.0 |####################
  42911.6 |
  43487.2 |
  44062.8 |
  44638.4 |
  45214.0 |
  (1 below, 2 above range)

sequential (n=18, range 12523.6-13435.4 ns)
  12523.6 |
  12569.2 |
  12614.8 |####################
  12660.4 |
  12706.0 |####################
  12751.6 |
  12797.1 |########################################
  12842.7 |########################################
  12888.3 |
  12933.9 |####################
  12979.5 |########################################
  13025.1 |
  13070.7 |########################################
  13116.3 |
  13161.9 |
  13207.5 |####################
  13253.1 |####################
  13298.6 |####################
  13344.2 |
  13389.8 |
  (2 below, 2 above range)

std_parallel (n=18, range 53612.5-68609.7 ns)
  53612.5 |
  54362.4 |
  55112.2 |########################################
  55862.1 |####################
  56611.9 |####################
  57361.8 |
  58111.7 |####################
  58861.5 |
  59611.4 |
  60361.2 |####################
  61111.1 |####################
  61861.0 |########################################
  62610.8 |########################################
  63360.7 |####################
  64110.6 |####################
  64860.4 |####################
  65610.3 |
  66360.1 |
  67110.0 |
  67859.9 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=1272.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=1325.3% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=512.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=1499.8% of algo (FFI overhead may distort results)
- **nway**: bridge=278.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=702.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=203.6% of algo (FFI overhead may distort results)
