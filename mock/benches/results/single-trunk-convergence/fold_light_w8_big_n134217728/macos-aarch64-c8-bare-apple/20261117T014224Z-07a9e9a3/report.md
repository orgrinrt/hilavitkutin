# fold_light, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail is fastest but the noisiest (CV 11.0%)

head_tail wins on median (8.62 ms) yet has the highest variance (CV 11.0%), while kernel_nway is the steadiest (CV 0.4%, 9.20 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (8.62 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader head_tail vs stability leader kernel_nway (+7% speed for 25.4x steadier)

head_tail is fastest (8.62 ms, CV 11.0%); kernel_nway gives up 6.7% median for 25.4x lower variance (CV 0.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (head_tail) is the fastest** at 8623140.6 ns median
- 3 variants significantly slower than baseline
- Spread: 1.82x (fastest 8623140.6 ns, slowest 15691109.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 8 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 9276590ns | 8626698ns | 8593375ns | 8619444ns | 10603917ns | base |
| head_tail_dyn | 15685106ns | 15694870ns | 15228948ns | 15612427ns | 16022203ns | +69.08% |
| kernel_nway | 9199441ns | 9199453ns | 9143375ns | 9188583ns | 9243760ns | -0.83% |
| kernel_seq | 11743439ns | 11562458ns | 11550073ns | 11558955ns | 12116849ns | +26.59% |
| nway | 9803241ns | 9549130ns | 9254938ns | 9452003ns | 10604250ns | +5.68% |
| sequential | 11956840ns | 11732812ns | 11555500ns | 11673830ns | 12582026ns | +28.89% |
| std_parallel | 9650731ns | 9442000ns | 9254271ns | 9390649ns | 10239083ns | +4.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9272545ns | 8590062ns | 10598776ns | base | 14.475 |
| head_tail_dyn | 15681630ns | 15225000ns | 16019219ns | +69.12% | 8.559 |
| kernel_nway | 9196500ns | 9140719ns | 9240344ns | -0.82% | 14.594 |
| kernel_seq | 11741601ns | 11548979ns | 12114755ns | +26.63% | 11.431 |
| nway | 9799773ns | 9251812ns | 10600719ns | +5.69% | 13.696 |
| sequential | 11954733ns | 11553062ns | 12579911ns | +28.93% | 11.227 |
| std_parallel | 9648420ns | 9251854ns | 10236411ns | +4.05% | 13.911 |

## Performance model

- Peak throughput: **15.625 Gops/s** (head_tail; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 15.565 | 99.6% |
| head_tail_dyn | 8.554 | 54.7% |
| kernel_nway | 14.594 | 93.4% |
| kernel_seq | 11.610 | 74.3% |
| nway | 14.061 | 90.0% |
| sequential | 11.442 | 73.2% |
| std_parallel | 14.218 | 91.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 9276590ns | 9276590ns | base |
| head_tail_dyn | 15685106ns | 15685106ns | +69.08% |
| kernel_nway | 9199441ns | 9199441ns | -0.83% |
| kernel_seq | 11743439ns | 11743439ns | +26.59% |
| nway | 9803241ns | 9803241ns | +5.68% |
| sequential | 11956840ns | 11956840ns | +28.89% |
| std_parallel | 9650731ns | 9650731ns | +4.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 8623141ns | base | --- | [8595719, 10598776] | --- | --- | --- | --- |
| head_tail_dyn | 15691109ns | +6907677.1ns (+80.1%) | [+4923500, +7396078]ns | [15334562, 16019219] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_nway | 9196661ns | no significant difference | [-1384781, +600943]ns | [9152495, 9240344] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 11560516ns | +2927109.4ns (+33.9%) | [+1515979, +2964078]ns | [11549531, 12114755] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 9545349ns | no significant difference | [-476880, +1350854]ns | [9253250, 10600719] | no | 0.3281 | 0.2188 | 0 |
| sequential | 11730688ns | +2932458.3ns (+34.0%) | [+1434781, +3679323]ns | [11553599, 12579911] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 9440052ns | no significant difference | [-1208026, +1619948]ns | [9268797, 10236411] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8608052ns | +84.4% | +6.6% | +34.3% | +15.7% | +34.2% | +17.1% |
| 1 | 2 | 10598776ns | +46.5% | -13.1% | +14.3% | -4.3% | +13.5% | -11.4% |
| 1 | 3 | 8610807ns | +81.8% | +6.8% | +34.1% | +8.0% | +42.6% | +10.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.667 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.410 | moderate- |
| kernel_nway | -0.427 | moderate- |
| kernel_seq | -0.666 | HIGH- (thermal bounce) |
| nway | -0.297 | moderate- |
| sequential | -0.023 | ok |
| std_parallel | -0.233 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 9302284.7ns | 9272545.2ns | 100.3% | HIGH |
| head_tail_dyn | 15721980.9ns | 15681630.2ns | 100.3% | HIGH |
| kernel_nway | 9210661.5ns | 9196500.0ns | 100.2% | HIGH |
| kernel_seq | 11737892.3ns | 11741600.7ns | 100.0% | HIGH |
| nway | 9887184.0ns | 9799772.6ns | 100.9% | HIGH |
| sequential | 11961194.4ns | 11954732.6ns | 100.1% | HIGH |
| std_parallel | 9675442.7ns | 9648420.1ns | 100.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 8590062.5-10598776.1 ns)
  8590062.5 |########################################
  8690498.2 |
  8790933.9 |
  8891369.5 |
  8991805.2 |
  9092240.9 |
  9192676.6 |
  9293112.2 |
  9393547.9 |
  9493983.6 |
  9594419.3 |
  9694855.0 |
  9795290.6 |
  9895726.3 |
  9996162.0 |
  10096597.7 |
  10197033.3 |
  10297469.0 |##########
  10397904.7 |
  10498340.4 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 15225000.0-16019218.8 ns)
  15225000.0 |########################################
  15264710.9 |
  15304421.9 |
  15344132.8 |
  15383843.8 |
  15423554.7 |########################################
  15463265.6 |
  15502976.6 |
  15542687.5 |
  15582398.4 |########################################
  15622109.4 |
  15661820.3 |
  15701531.2 |
  15741242.2 |
  15780953.1 |########################################
  15820664.1 |
  15860375.0 |
  15900085.9 |
  15939796.9 |########################################
  15979507.8 |
  (0 below, 1 above range)

kernel_nway (n=6, range 9140718.8-9240343.8 ns)
  9140718.8 |########################################
  9145700.1 |
  9150681.3 |
  9155662.6 |
  9160643.8 |########################################
  9165625.1 |
  9170606.3 |
  9175587.6 |
  9180568.8 |########################################
  9185550.1 |
  9190531.3 |
  9195512.6 |
  9200493.8 |
  9205475.1 |
  9210456.3 |########################################
  9215437.6 |########################################
  9220418.8 |
  9225400.1 |
  9230381.3 |
  9235362.6 |
  (0 below, 1 above range)

kernel_seq (n=6, range 11548979.2-12114755.2 ns)
  11548979.2 |########################################
  11577268.0 |
  11605556.8 |
  11633845.6 |
  11662134.4 |
  11690423.2 |
  11718712.0 |
  11747000.8 |
  11775289.6 |
  11803578.4 |
  11831867.2 |
  11860156.0 |
  11888444.8 |
  11916733.6 |
  11945022.4 |
  11973311.2 |
  12001600.0 |
  12029888.8 |##########
  12058177.6 |
  12086466.4 |
  (0 below, 1 above range)

nway (n=6, range 9251812.5-10600718.8 ns)
  9251812.5 |########################################
  9319257.8 |####################
  9386703.1 |
  9454148.4 |
  9521593.8 |
  9589039.1 |
  9656484.4 |
  9723929.7 |####################
  9791375.0 |
  9858820.3 |
  9926265.6 |
  9993710.9 |
  10061156.2 |
  10128601.6 |####################
  10196046.9 |
  10263492.2 |
  10330937.5 |
  10398382.8 |
  10465828.1 |
  10533273.4 |
  (0 below, 1 above range)

sequential (n=6, range 11553062.5-12579911.4 ns)
  11553062.5 |########################################
  11604404.9 |
  11655747.4 |
  11707089.8 |
  11758432.3 |
  11809774.7 |
  11861117.2 |#############
  11912459.6 |
  11963802.1 |
  12015144.5 |
  12066487.0 |
  12117829.4 |#############
  12169171.9 |
  12220514.3 |
  12271856.8 |
  12323199.2 |
  12374541.7 |
  12425884.1 |
  12477226.6 |
  12528569.0 |
  (0 below, 1 above range)

std_parallel (n=6, range 9251854.2-10236411.4 ns)
  9251854.2 |########################################
  9301082.1 |
  9350309.9 |####################
  9399537.8 |
  9448765.6 |
  9497993.5 |####################
  9547221.4 |
  9596449.2 |
  9645677.1 |####################
  9694905.0 |
  9744132.8 |
  9793360.7 |
  9842588.5 |
  9891816.4 |
  9941044.3 |
  9990272.1 |
  10039500.0 |
  10088727.9 |
  10137955.7 |
  10187183.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=102.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=99.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.2% of algo (FFI overhead may distort results)
- **nway**: bridge=99.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.8% of algo (FFI overhead may distort results)
