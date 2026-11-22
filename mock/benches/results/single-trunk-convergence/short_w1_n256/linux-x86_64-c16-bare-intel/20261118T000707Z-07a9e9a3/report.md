# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 34% faster than the next best (kernel_seq)

sequential (7.14 us) leads kernel_seq (9.56 us) by 34%, a clear separation rather than a photo finish. CV 112.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 4.8x slower than the field

std_parallel (34.03 us) is 4.8x the fastest (7.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 112.9%)

sequential wins on median (7.14 us) yet has the highest variance (CV 112.9%), while std_parallel is the steadiest (CV 8.5%, 34.03 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {sequential, kernel_seq, head_tail_dyn, head_tail, kernel_nway} vs {nway, std_parallel} (174% apart)

The field splits into a fast tier {sequential, kernel_seq, head_tail_dyn, head_tail, kernel_nway} and a slow tier {nway, std_parallel} with a 174% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.8x the fastest

Fastest sequential (7.14 us) to slowest std_parallel (34.03 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 6.64 us but its worst 20% at 23.79 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7137.5 ns median (-38.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 4.77x (fastest 7137.5 ns, slowest 34031.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 12935ns | 12652ns | 9882ns | 11979ns | 15893ns | base |
| head_tail_dyn | 11483ns | 11042ns | 9010ns | 10717ns | 13870ns | -11.22% |
| kernel_nway | 13558ns | 12748ns | 9674ns | 12321ns | 17355ns | +4.82% |
| kernel_seq | 13389ns | 10446ns | 8572ns | 10148ns | 20661ns | +3.52% |
| nway | 36331ns | 33543ns | 30730ns | 33155ns | 43897ns | +180.89% |
| sequential | 13116ns | 7276ns | 6778ns | 7766ns | 24312ns | +1.41% |
| std_parallel | 35056ns | 34656ns | 31387ns | 34136ns | 38270ns | +171.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 12032ns | 9288ns | 14728ns | base | 0.021 |
| head_tail_dyn | 10757ns | 8553ns | 12914ns | -10.60% | 0.024 |
| kernel_nway | 12646ns | 9108ns | 16094ns | +5.10% | 0.020 |
| kernel_seq | 12445ns | 8062ns | 19279ns | +3.42% | 0.021 |
| nway | 35793ns | 30306ns | 43206ns | +197.47% | 0.007 |
| sequential | 12843ns | 6641ns | 23791ns | +6.74% | 0.020 |
| std_parallel | 34377ns | 30882ns | 37562ns | +185.70% | 0.007 |

## Performance model

- Peak throughput: **0.039 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.022 | 57.4% |
| head_tail_dyn | 0.025 | 63.6% |
| kernel_nway | 0.021 | 55.0% |
| kernel_seq | 0.027 | 69.5% |
| nway | 0.008 | 20.1% |
| sequential | 0.036 | 93.1% |
| std_parallel | 0.008 | 19.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 12935ns | 12935ns | base |
| head_tail_dyn | 11483ns | 11483ns | -11.22% |
| kernel_nway | 13558ns | 13558ns | +4.82% |
| kernel_seq | 13389ns | 13389ns | +3.52% |
| nway | 36331ns | 36331ns | +180.89% |
| sequential | 13116ns | 13116ns | +1.41% |
| std_parallel | 35056ns | 35056ns | +171.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 11561ns | base | --- | [10048, 13859] | --- | --- | --- | --- |
| head_tail_dyn | 10449ns | no significant difference | [-2597, +860]ns | [9355, 11900] | no | 0.2855 | 0.2379 | 0 |
| kernel_nway | 12071ns | no significant difference | [-1580, +4081]ns | [10222, 14130] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 9557ns | no significant difference | [-4307, +6299]ns | [8820, 16642] | no | 0.2855 | 0.2379 | 0 |
| nway | 33060ns | +21709.9ns (+187.8%) | [+20847, +23475]ns | [31750, 35623] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 7137ns | no significant difference | [-3501, +5607]ns | [6846, 17967] | no | 0.2855 | 0.2379 | 0 |
| std_parallel | 34031ns | +22485.5ns (+194.5%) | [+20001, +24326]ns | [32482, 36053] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 10875ns | +20.0% | +41.6% | +80.2% | +191.9% | -37.1% | +213.1% |
| 1 | 2 | 10048ns | -7.3% | -2.1% | -11.0% | +232.8% | -30.1% | +249.1% |
| 1 | 3 | 14034ns | -25.5% | -14.0% | -34.7% | +145.6% | -29.3% | +135.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.291 | moderate- |
| head_tail_dyn | -0.359 | moderate- |
| kernel_nway | -0.431 | moderate- |
| kernel_seq | -0.177 | ok |
| nway | -0.013 | ok |
| sequential | -0.138 | ok |
| std_parallel | -0.495 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 12550.8ns | 12032.5ns | 104.3% | HIGH |
| head_tail_dyn | 11113.7ns | 10757.4ns | 103.3% | HIGH |
| kernel_nway | 13022.7ns | 12645.6ns | 103.0% | HIGH |
| kernel_seq | 13045.4ns | 12444.5ns | 104.8% | HIGH |
| nway | 36683.7ns | 35792.9ns | 102.5% | HIGH |
| sequential | 13392.5ns | 12843.4ns | 104.3% | HIGH |
| std_parallel | 35055.9ns | 34377.0ns | 102.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9288.1-14727.6 ns)
   9288.1 |########################################
   9560.0 |#############
   9832.0 |
  10104.0 |
  10376.0 |#############
  10647.9 |#############
  10919.9 |#############
  11191.9 |#############
  11463.9 |
  11735.8 |#############
  12007.8 |
  12279.8 |#############
  12551.8 |
  12823.7 |
  13095.7 |#############
  13367.7 |
  13639.7 |########################################
  13911.6 |
  14183.6 |#############
  14455.6 |#############
  (1 below, 1 above range)

head_tail_dyn (n=18, range 8553.3-12913.8 ns)
   8553.3 |##########
   8771.3 |
   8989.4 |
   9207.4 |########################################
   9425.4 |##########
   9643.4 |
   9861.5 |##########
  10079.5 |
  10297.5 |
  10515.5 |
  10733.6 |
  10951.6 |####################
  11169.6 |
  11387.6 |
  11605.7 |####################
  11823.7 |##########
  12041.7 |
  12259.7 |
  12477.8 |##########
  12695.8 |
  (2 below, 3 above range)

kernel_nway (n=18, range 9107.6-16093.6 ns)
   9107.6 |#############
   9456.9 |#############
   9806.2 |#############
  10155.5 |########################################
  10504.8 |
  10854.1 |
  11203.4 |
  11552.7 |#############
  11902.0 |##########################
  12251.3 |
  12600.6 |
  12949.9 |#############
  13299.2 |#############
  13648.5 |
  13997.8 |##########################
  14347.1 |
  14696.4 |
  15045.7 |#############
  15395.0 |#############
  15744.3 |
  (1 below, 2 above range)

kernel_seq (n=18, range 8061.8-19279.2 ns)
   8061.8 |##########
   8622.7 |##############################
   9183.5 |########################################
   9744.4 |####################
  10305.3 |
  10866.2 |
  11427.0 |
  11987.9 |
  12548.8 |
  13109.6 |
  13670.5 |
  14231.4 |
  14792.3 |
  15353.1 |##########
  15914.0 |
  16474.9 |
  17035.7 |##########
  17596.6 |
  18157.5 |
  18718.4 |##########
  (2 below, 3 above range)

nway (n=18, range 30306.1-43205.7 ns)
  30306.1 |##########################
  30951.1 |
  31596.1 |########################################
  32241.1 |##########################
  32886.1 |##########################
  33531.0 |##########################
  34176.0 |
  34821.0 |
  35466.0 |##########################
  36111.0 |
  36755.9 |
  37400.9 |#############
  38045.9 |
  38690.9 |#############
  39335.9 |
  39980.8 |
  40625.8 |
  41270.8 |
  41915.8 |#############
  42560.8 |
  (1 below, 1 above range)

sequential (n=18, range 6641.5-23791.1 ns)
   6641.5 |########################################
   7498.9 |
   8356.4 |
   9213.9 |
  10071.4 |
  10928.9 |
  11786.4 |
  12643.8 |####
  13501.3 |
  14358.8 |
  15216.3 |
  16073.8 |
  16931.2 |
  17788.7 |####
  18646.2 |
  19503.7 |
  20361.2 |
  21218.7 |
  22076.1 |####
  22933.6 |####
  (2 below, 3 above range)

std_parallel (n=18, range 30882.3-37562.2 ns)
  30882.3 |#############
  31216.3 |
  31550.3 |#############
  31884.3 |#############
  32218.3 |##########################
  32552.3 |
  32886.3 |
  33220.2 |##########################
  33554.2 |
  33888.2 |########################################
  34222.2 |
  34556.2 |#############
  34890.2 |
  35224.2 |
  35558.2 |
  35892.2 |########################################
  36226.2 |
  36560.2 |#############
  36894.2 |
  37228.2 |#############
  (1 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=104.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.3% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=23.5% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=104.3% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=40.3% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=104.1% of algo (FFI overhead may distort results)
- **nway**: CV=23.9% (high variance, measurements may be unstable)
- **nway**: bridge=102.5% of algo (FFI overhead may distort results)
- **sequential**: CV=62.7% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=101.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.8% of algo (FFI overhead may distort results)
