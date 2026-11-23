# fold_light, short fiber (1 to 4 morsels), W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 3.9x slower than the field

nway (47.39 us) is 3.9x the fastest (12.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 63.8%)

sequential wins on median (12.05 us) yet has the highest variance (CV 63.8%), while head_tail_dyn is the steadiest (CV 11.7%, 15.62 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail, kernel_nway, kernel_seq, head_tail_dyn} vs {std_parallel, nway} (182% apart)

The field splits into a fast tier {sequential, head_tail, kernel_nway, kernel_seq, head_tail_dyn} and a slow tier {std_parallel, nway} with a 182% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.9x the fastest

Fastest sequential (12.05 us) to slowest nway (47.39 us): 3.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 6.77 us but its worst 20% at 23.46 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 12045.7 ns median (-5.2% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 3.93x (fastest 12045.7 ns, slowest 47390.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 4 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112732635 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 14235ns | 13412ns | 12058ns | 13058ns | 17088ns | base |
| head_tail_dyn | 16326ns | 16443ns | 13348ns | 15763ns | 18660ns | +14.69% |
| kernel_nway | 14767ns | 13636ns | 12382ns | 13639ns | 17652ns | +3.74% |
| kernel_seq | 14425ns | 14432ns | 10052ns | 13710ns | 17684ns | +1.33% |
| nway | 48489ns | 48018ns | 41461ns | 46388ns | 55156ns | +240.64% |
| sequential | 14801ns | 12282ns | 6902ns | 11334ns | 23951ns | +3.98% |
| std_parallel | 57633ns | 44276ns | 35922ns | 44873ns | 87626ns | +304.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 13434ns | 11444ns | 16064ns | base | 0.076 |
| head_tail_dyn | 15469ns | 12783ns | 17581ns | +15.14% | 0.066 |
| kernel_nway | 13974ns | 11658ns | 16734ns | +4.02% | 0.073 |
| kernel_seq | 13579ns | 9521ns | 16512ns | +1.08% | 0.075 |
| nway | 47819ns | 40882ns | 54368ns | +255.95% | 0.021 |
| sequential | 14505ns | 6765ns | 23457ns | +7.97% | 0.071 |
| std_parallel | 57206ns | 35744ns | 86885ns | +325.82% | 0.018 |

## Performance model

- Peak throughput: **0.151 Gops/s** (sequential; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.081 | 53.2% |
| head_tail_dyn | 0.066 | 43.3% |
| kernel_nway | 0.079 | 52.3% |
| kernel_seq | 0.074 | 48.9% |
| nway | 0.022 | 14.3% |
| sequential | 0.085 | 56.2% |
| std_parallel | 0.023 | 15.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 14235ns | 14235ns | base |
| head_tail_dyn | 16326ns | 16326ns | +14.69% |
| kernel_nway | 14767ns | 14767ns | +3.74% |
| kernel_seq | 14425ns | 14425ns | +1.33% |
| nway | 48489ns | 48489ns | +240.64% |
| sequential | 14801ns | 14801ns | +3.98% |
| std_parallel | 57633ns | 57633ns | +304.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 12707ns | base | --- | [11703, 14378] | --- | --- | --- | --- |
| head_tail_dyn | 15624ns | +1737.9ns (+13.7%) | [+936, +3374]ns | [14194, 16909] | YES | 0.0151 | 0.0075 | 0 |
| kernel_nway | 12943ns | +570.5ns (+4.5%) | [+188, +1517]ns | [12159, 16252] | YES | 0.0463 | 0.0309 | 0 |
| kernel_seq | 13828ns | no significant difference | [-925, +1604]ns | [11598, 15495] | no | 0.9774 | 0.8145 | 0 |
| nway | 47390ns | +32414.0ns (+255.1%) | [+30325, +37414]ns | [43450, 50957] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12046ns | no significant difference | [-4929, +7448]ns | [7018, 21393] | no | 1.0000 | 1.0000 | 0 |
| std_parallel | 44013ns | +32096.4ns (+252.6%) | [+27814, +54241]ns | [38828, 69751] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 11504ns | +41.9% | +2.6% | -12.9% | +256.3% | +38.2% | +482.6% |
| 1 | 2 | 14194ns | +16.0% | +16.8% | +7.5% | +220.6% | -50.0% | +161.7% |
| 1 | 3 | 13255ns | +3.8% | -2.4% | +15.2% | +300.8% | +24.6% | +290.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.392 | moderate- |
| head_tail_dyn | -0.193 | ok |
| kernel_nway | -0.632 | HIGH- (thermal bounce) |
| kernel_seq | -0.166 | ok |
| nway | -0.014 | ok |
| sequential | -0.664 | HIGH- (thermal bounce) |
| std_parallel | -0.493 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 13862.9ns | 13434.3ns | 103.2% | HIGH |
| head_tail_dyn | 16026.7ns | 15468.6ns | 103.6% | HIGH |
| kernel_nway | 14390.2ns | 13974.4ns | 103.0% | HIGH |
| kernel_seq | 13951.5ns | 13578.8ns | 102.7% | HIGH |
| nway | 48465.2ns | 47819.4ns | 101.4% | HIGH |
| sequential | 15036.6ns | 14505.1ns | 103.7% | HIGH |
| std_parallel | 58673.9ns | 57206.4ns | 102.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 11443.7-16063.6 ns)
  11443.7 |########################################
  11674.7 |########################################
  11905.7 |#############
  12136.7 |
  12367.7 |#############
  12598.7 |
  12829.7 |#############
  13060.7 |#############
  13291.7 |
  13522.7 |#############
  13753.6 |
  13984.6 |#############
  14215.6 |
  14446.6 |
  14677.6 |#############
  14908.6 |
  15139.6 |#############
  15370.6 |
  15601.6 |
  15832.6 |#############
  (1 below, 2 above range)

head_tail_dyn (n=18, range 12782.9-17580.9 ns)
  12782.9 |
  13022.8 |
  13262.7 |
  13502.6 |####################
  13742.5 |
  13982.4 |########################################
  14222.3 |########################################
  14462.2 |####################
  14702.1 |
  14942.0 |
  15181.9 |
  15421.8 |####################
  15661.7 |####################
  15901.6 |########################################
  16141.5 |
  16381.4 |
  16621.3 |####################
  16861.2 |########################################
  17101.1 |
  17341.0 |
  (2 below, 3 above range)

kernel_nway (n=18, range 11657.5-16734.3 ns)
  11657.5 |########################################
  11911.3 |#############
  12165.2 |##########################
  12419.0 |#############
  12672.9 |#############
  12926.7 |##########################
  13180.5 |
  13434.4 |
  13688.2 |
  13942.1 |
  14195.9 |
  14449.7 |
  14703.6 |
  14957.4 |
  15211.3 |
  15465.1 |
  15719.0 |#############
  15972.8 |
  16226.6 |##########################
  16480.5 |##########################
  (1 below, 2 above range)

kernel_seq (n=18, range 9520.7-16512.0 ns)
   9520.7 |
   9870.3 |##########################
  10219.8 |
  10569.4 |
  10919.0 |
  11268.5 |#############
  11618.1 |#############
  11967.6 |
  12317.2 |
  12666.8 |
  13016.3 |
  13365.9 |##########################
  13715.5 |########################################
  14065.0 |
  14414.6 |
  14764.2 |#############
  15113.7 |
  15463.3 |########################################
  15812.9 |
  16162.4 |#############
  (2 below, 2 above range)

nway (n=18, range 40881.5-54367.7 ns)
  40881.5 |########################################
  41555.8 |
  42230.1 |#############
  42904.4 |
  43578.7 |#############
  44253.0 |
  44927.3 |########################################
  45601.7 |
  46276.0 |
  46950.3 |
  47624.6 |
  48298.9 |
  48973.2 |########################################
  49647.5 |#############
  50321.8 |
  50996.1 |
  51670.4 |#############
  52344.7 |
  53019.0 |
  53693.4 |########################################
  (1 below, 1 above range)

sequential (n=18, range 6765.4-23457.1 ns)
   6765.4 |########################################
   7600.0 |
   8434.6 |
   9269.2 |
  10103.8 |#####
  10938.3 |
  11772.9 |
  12607.5 |#####
  13442.1 |
  14276.7 |
  15111.3 |
  15945.8 |
  16780.4 |
  17615.0 |
  18449.6 |
  19284.2 |#####
  20118.8 |#####
  20953.3 |#####
  21787.9 |###########
  22622.5 |#####
  (1 below, 2 above range)

std_parallel (n=18, range 35743.8-86885.0 ns)
  35743.8 |########################################
  38300.9 |##########
  40858.0 |####################
  43415.0 |####################
  45972.1 |
  48529.1 |
  51086.2 |
  53643.2 |##########
  56200.3 |##########
  58757.4 |##########
  61314.4 |
  63871.5 |
  66428.5 |
  68985.6 |
  71542.7 |
  74099.7 |
  76656.8 |
  79213.8 |####################
  81770.9 |##########
  84328.0 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=105.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=103.3% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=20.1% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=102.7% of algo (FFI overhead may distort results)
- **nway**: bridge=102.2% of algo (FFI overhead may distort results)
- **sequential**: CV=52.9% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=102.1% of algo (FFI overhead may distort results)
- **std_parallel**: CV=43.7% (high variance, measurements may be unstable)
- **std_parallel**: bridge=106.4% of algo (FFI overhead may distort results)
