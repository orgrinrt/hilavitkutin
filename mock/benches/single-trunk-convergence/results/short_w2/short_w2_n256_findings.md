# fold_light, short fiber (1 to 4 morsels), W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 172% faster than the next best (kernel_seq)

sequential (7.71 us) leads kernel_seq (21.01 us) by 172%, a clear separation rather than a photo finish. CV 29.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 73% (significant)

sequential is -23.34 us (73%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 8.1x slower than the field

std_parallel (62.82 us) is 8.1x the fastest (7.71 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 29.4%)

sequential wins on median (7.71 us) yet has the highest variance (CV 29.4%), while head_tail is the steadiest (CV 7.1%, 31.96 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.66)

kernel_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential} vs {kernel_seq, head_tail_dyn, kernel_nway, head_tail, nway, std_parallel} (172% apart)

The field splits into a fast tier {sequential} and a slow tier {kernel_seq, head_tail_dyn, kernel_nway, head_tail, nway, std_parallel} with a 172% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 8.1x the fastest

Fastest sequential (7.71 us) to slowest std_parallel (62.82 us): 8.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 1.7x its best-20%

sequential's best 20% of batches run at 7.12 us but its worst 20% at 12.28 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 7711.1 ns median (-75.9% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 8.15x (fastest 7711.1 ns, slowest 62824.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 2 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 33116ns | 32488ns | 30188ns | 32277ns | 35838ns | base |
| head_tail_dyn | 31498ns | 30229ns | 26645ns | 29330ns | 37176ns | -4.89% |
| kernel_nway | 31765ns | 32357ns | 25716ns | 31408ns | 35326ns | -4.08% |
| kernel_seq | 23261ns | 21357ns | 19349ns | 20888ns | 28777ns | -29.76% |
| nway | 40802ns | 41068ns | 34648ns | 39465ns | 45886ns | +23.21% |
| sequential | 9419ns | 7977ns | 7343ns | 7968ns | 12633ns | -71.56% |
| std_parallel | 63266ns | 63529ns | 52729ns | 62212ns | 70116ns | +91.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 32625ns | 29706ns | 35326ns | base | 0.008 |
| head_tail_dyn | 31003ns | 26267ns | 36520ns | -4.97% | 0.008 |
| kernel_nway | 31262ns | 25239ns | 34797ns | -4.18% | 0.008 |
| kernel_seq | 22833ns | 18991ns | 28296ns | -30.01% | 0.011 |
| nway | 40402ns | 34305ns | 45438ns | +23.84% | 0.006 |
| sequential | 9125ns | 7122ns | 12278ns | -72.03% | 0.028 |
| std_parallel | 62667ns | 52236ns | 69360ns | +92.08% | 0.004 |

## Performance model

- Peak throughput: **0.036 Gops/s** (sequential; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.008 | 22.3% |
| head_tail_dyn | 0.009 | 23.9% |
| kernel_nway | 0.008 | 22.4% |
| kernel_seq | 0.012 | 33.9% |
| nway | 0.006 | 17.5% |
| sequential | 0.033 | 92.4% |
| std_parallel | 0.004 | 11.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 33116ns | 33116ns | base |
| head_tail_dyn | 31498ns | 31498ns | -4.89% |
| kernel_nway | 31765ns | 31765ns | -4.08% |
| kernel_seq | 23261ns | 23261ns | -29.76% |
| nway | 40802ns | 40802ns | +23.21% |
| sequential | 9419ns | 9419ns | -71.56% |
| std_parallel | 63266ns | 63266ns | +91.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 31965ns | base | --- | [30865, 34220] | --- | --- | --- | --- |
| head_tail_dyn | 29814ns | -3257.1ns (-10.2%) | [-4495, -1318]ns | [27443, 33666] | YES | 0.0371 | 0.0309 | 0 |
| kernel_nway | 31841ns | no significant difference | [-2051, +392]ns | [29166, 34425] | no | 0.4807 | 0.4807 | 0 |
| kernel_seq | 21007ns | -11303.1ns (-35.4%) | [-13138, -7600]ns | [19506, 26620] | YES | 0.0003 | 0.0001 | 0 |
| nway | 40664ns | +9245.4ns (+28.9%) | [+3502, +12438]ns | [37279, 43994] | YES | 0.0113 | 0.0075 | 0 |
| sequential | 7711ns | -23341.3ns (-73.0%) | [-26106, -20714]ns | [7557, 11443] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 62825ns | +29328.5ns (+91.8%) | [+25822, +34548]ns | [60458, 66053] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 31206ns | -11.8% | +2.0% | -34.3% | +44.8% | -60.8% | +123.3% |
| 1 | 2 | 31386ns | -7.9% | -15.3% | -9.9% | +29.6% | -75.6% | +76.4% |
| 1 | 3 | 34761ns | -3.1% | -0.3% | -44.5% | +0.3% | -78.3% | +80.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.141 | ok |
| head_tail_dyn | -0.061 | ok |
| kernel_nway | -0.588 | HIGH- (thermal bounce) |
| kernel_seq | -0.657 | HIGH- (thermal bounce) |
| nway | -0.003 | ok |
| sequential | -0.156 | ok |
| std_parallel | -0.514 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 32832.8ns | 32625.3ns | 100.6% | HIGH |
| head_tail_dyn | 32162.8ns | 31002.8ns | 103.7% | HIGH |
| kernel_nway | 31595.7ns | 31261.5ns | 101.1% | HIGH |
| kernel_seq | 23336.4ns | 22833.3ns | 102.2% | HIGH |
| nway | 41023.2ns | 40402.2ns | 101.5% | HIGH |
| sequential | 9617.2ns | 9124.8ns | 105.4% | HIGH |
| std_parallel | 63029.8ns | 62666.6ns | 100.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 29706.2-35326.4 ns)
  29706.2 |####################
  29987.2 |
  30268.2 |
  30549.3 |########################################
  30830.3 |########################################
  31111.3 |####################
  31392.3 |
  31673.3 |####################
  31954.3 |####################
  32235.3 |
  32516.3 |
  32797.3 |
  33078.3 |
  33359.3 |
  33640.3 |########################################
  33921.3 |####################
  34202.4 |####################
  34483.4 |####################
  34764.4 |
  35045.4 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 26266.7-36520.0 ns)
  26266.7 |####################
  26779.4 |####################
  27292.1 |########################################
  27804.7 |####################
  28317.4 |####################
  28830.0 |
  29342.7 |####################
  29855.4 |####################
  30368.0 |####################
  30880.7 |
  31393.4 |####################
  31906.0 |
  32418.7 |####################
  32931.3 |
  33444.0 |
  33956.7 |####################
  34469.3 |########################################
  34982.0 |
  35494.7 |
  36007.3 |
  (2 below, 2 above range)

kernel_nway (n=18, range 25238.5-34797.2 ns)
  25238.5 |
  25716.5 |#############
  26194.4 |
  26672.3 |
  27150.3 |#############
  27628.2 |
  28106.1 |
  28584.1 |#############
  29062.0 |#############
  29539.9 |#############
  30017.9 |
  30495.8 |#############
  30973.7 |#############
  31451.7 |
  31929.6 |
  32407.5 |#############
  32885.5 |
  33363.4 |
  33841.3 |########################################
  34319.3 |########################################
  (2 below, 2 above range)

kernel_seq (n=18, range 18990.7-28295.7 ns)
  18990.7 |####################
  19456.0 |##############################
  19921.2 |##########
  20386.5 |
  20851.7 |########################################
  21317.0 |##########
  21782.2 |
  22247.5 |
  22712.7 |
  23178.0 |
  23643.2 |
  24108.5 |
  24573.7 |
  25039.0 |
  25504.2 |
  25969.5 |##########
  26434.7 |
  26900.0 |##########
  27365.2 |
  27830.5 |##########
  (1 below, 3 above range)

nway (n=18, range 34304.6-45438.5 ns)
  34304.6 |####################
  34861.3 |####################
  35418.0 |
  35974.7 |
  36531.3 |####################
  37088.0 |####################
  37644.7 |####################
  38201.4 |####################
  38758.1 |
  39314.8 |
  39871.5 |
  40428.2 |########################################
  40984.9 |####################
  41541.6 |
  42098.3 |
  42655.0 |
  43211.7 |####################
  43768.4 |########################################
  44325.1 |####################
  44881.8 |
  (2 below, 3 above range)

sequential (n=18, range 7121.9-12278.3 ns)
   7121.9 |##########
   7379.7 |########################################
   7637.5 |########################################
   7895.4 |
   8153.2 |##########
   8411.0 |
   8668.8 |
   8926.6 |
   9184.5 |
   9442.3 |
   9700.1 |
   9957.9 |
  10215.7 |
  10473.6 |
  10731.4 |
  10989.2 |
  11247.0 |##########
  11504.8 |##########
  11762.7 |
  12020.5 |####################
  (2 below, 2 above range)

std_parallel (n=18, range 52236.1-69360.2 ns)
  52236.1 |
  53092.3 |
  53948.5 |
  54804.7 |#############
  55660.9 |#############
  56517.1 |
  57373.3 |
  58229.5 |
  59085.7 |
  59941.9 |##########################
  60798.1 |
  61654.3 |########################################
  62510.6 |#############
  63366.8 |#############
  64223.0 |#############
  65079.2 |#############
  65935.4 |#############
  66791.6 |
  67647.8 |
  68504.0 |#############
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=102.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=104.0% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.4% of algo (FFI overhead may distort results)
- **nway**: bridge=103.6% of algo (FFI overhead may distort results)
- **sequential**: CV=24.9% (high variance, measurements may be unstable)
- **sequential**: bridge=106.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.4% of algo (FFI overhead may distort results)
