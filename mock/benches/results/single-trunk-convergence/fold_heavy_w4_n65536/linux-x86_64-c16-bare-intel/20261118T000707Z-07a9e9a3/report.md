# fold_heavy, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway is fastest but the noisiest (CV 129.8%)

kernel_nway wins on median (129.12 us) yet has the highest variance (CV 129.8%), while sequential is the steadiest (CV 0.6%, 195.42 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.66)

kernel_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (66.55 us) is smaller than the fastest variant's own run-to-run std-dev (167.59 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### nway is inconsistent: worst-20% is 4.5x its best-20%

nway's best 20% of batches run at 87.16 us but its worst 20% at 389.74 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 129119.1 ns median (-7.9% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.52x (fastest 129119.1 ns, slowest 195670.0 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 171361ns | 140680ns | 98108ns | 133617ns | 264604ns | base |
| head_tail_dyn | 149976ns | 138732ns | 121384ns | 131198ns | 192438ns | -12.48% |
| kernel_nway | 206152ns | 129390ns | 126715ns | 131574ns | 357739ns | +20.30% |
| kernel_seq | 207634ns | 196389ns | 192406ns | 195630ns | 233255ns | +21.17% |
| nway | 226922ns | 159460ns | 87398ns | 164052ns | 390988ns | +32.42% |
| sequential | 195026ns | 195591ns | 193135ns | 194946ns | 196091ns | +13.81% |
| std_parallel | 224288ns | 181625ns | 155233ns | 176729ns | 330154ns | +30.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 170257ns | 97825ns | 262037ns | base | 0.385 |
| head_tail_dyn | 149302ns | 121153ns | 190944ns | -12.31% | 0.439 |
| kernel_nway | 205382ns | 126512ns | 355899ns | +20.63% | 0.319 |
| kernel_seq | 206571ns | 192073ns | 231160ns | +21.33% | 0.317 |
| nway | 226298ns | 87156ns | 389742ns | +32.92% | 0.290 |
| sequential | 194852ns | 192971ns | 195920ns | +14.45% | 0.336 |
| std_parallel | 223567ns | 154999ns | 328782ns | +31.31% | 0.293 |

## Performance model

- Peak throughput: **0.752 Gops/s** (nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.467 | 62.1% |
| head_tail_dyn | 0.473 | 62.9% |
| kernel_nway | 0.508 | 67.5% |
| kernel_seq | 0.335 | 44.5% |
| nway | 0.412 | 54.7% |
| sequential | 0.335 | 44.6% |
| std_parallel | 0.362 | 48.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 171361ns | 171361ns | base |
| head_tail_dyn | 149976ns | 149976ns | -12.48% |
| kernel_nway | 206152ns | 206152ns | +20.30% |
| kernel_seq | 207634ns | 207634ns | +21.17% |
| nway | 226922ns | 226922ns | +32.42% |
| sequential | 195026ns | 195026ns | +13.81% |
| std_parallel | 224288ns | 224288ns | +30.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 140251ns | base | --- | [115420, 207271] | --- | --- | --- | --- |
| head_tail_dyn | 138471ns | no significant difference | [-73742, +25487]ns | [121714, 146721] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 129119ns | +14706.6ns (+10.5%) | [+3023, +28886]ns | [128572, 156335] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| kernel_seq | 195670ns | no significant difference | [-6840, +79398]ns | [194143, 197917] | no | 0.1444 | 0.0963 | 0 |
| nway | 159202ns | +41315.6ns (+29.5%) | [+11454, +142831]ns | [138135, 320112] | YES (adj: no) | 0.0927 | 0.0309 | 0 |
| sequential | 195421ns | no significant difference | [-10978, +80177]ns | [193612, 195627] | no | 0.2855 | 0.2379 | 0 |
| std_parallel | 181161ns | +37867.3ns (+27.0%) | [+17236, +52414]ns | [172104, 196233] | YES (adj: no) | 0.0927 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 242214ns | -49.8% | -43.4% | -19.4% | -43.2% | -20.3% | -24.5% |
| 1 | 2 | 103229ns | +18.7% | +25.0% | +91.3% | +35.0% | +89.3% | +81.0% |
| 1 | 3 | 117917ns | +23.7% | +13.7% | +65.4% | +151.2% | +65.9% | +34.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.225 | moderate- |
| head_tail_dyn | -0.148 | ok |
| kernel_nway | -0.570 | HIGH- (thermal bounce) |
| kernel_seq | -0.665 | HIGH- (thermal bounce) |
| nway | -0.161 | ok |
| sequential | -0.121 | ok |
| std_parallel | -0.246 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 172467.2ns | 170256.8ns | 101.3% | HIGH |
| head_tail_dyn | 152371.5ns | 149302.4ns | 102.1% | HIGH |
| kernel_nway | 203160.3ns | 205381.8ns | 98.9% | HIGH |
| kernel_seq | 208800.1ns | 206571.4ns | 101.1% | HIGH |
| nway | 225306.6ns | 226298.3ns | 99.6% | HIGH |
| sequential | 195078.8ns | 194852.0ns | 100.1% | HIGH |
| std_parallel | 224865.4ns | 223567.1ns | 100.6% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 97824.6-262036.7 ns)
  97824.6 |####################
  106035.2 |##########
  114245.8 |########################################
  122456.4 |
  130667.0 |##########
  138877.7 |##########
  147088.3 |##########
  155298.9 |
  163509.5 |
  171720.1 |
  179930.7 |
  188141.3 |##########
  196351.9 |##########
  204562.5 |
  212773.1 |##########
  220983.7 |
  229194.3 |
  237404.9 |
  245615.5 |
  253826.1 |
  (1 below, 4 above range)

head_tail_dyn (n=18, range 121152.7-190943.6 ns)
  121152.7 |########################################
  124642.2 |
  128131.8 |#####
  131621.3 |
  135110.9 |
  138600.4 |
  142090.0 |
  145579.5 |############################
  149069.0 |
  152558.6 |
  156048.1 |
  159537.7 |
  163027.2 |#####
  166516.8 |
  170006.3 |
  173495.9 |
  176985.4 |
  180475.0 |
  183964.5 |
  187454.1 |
  (1 below, 3 above range)

kernel_nway (n=18, range 126512.3-355898.7 ns)
  126512.3 |########################################
  137981.7 |###############
  149451.0 |
  160920.3 |#####
  172389.6 |
  183858.9 |
  195328.2 |
  206797.6 |
  218266.9 |
  229736.2 |
  241205.5 |
  252674.8 |
  264144.2 |
  275613.5 |
  287082.8 |
  298552.1 |
  310021.4 |#####
  321490.7 |#####
  332960.1 |
  344429.4 |
  (2 below, 2 above range)

kernel_seq (n=18, range 192073.0-231160.0 ns)
  192073.0 |################
  194027.4 |########################################
  195981.7 |################################
  197936.1 |########
  199890.4 |
  201844.8 |
  203799.1 |
  205753.5 |
  207707.8 |
  209662.2 |
  211616.5 |
  213570.9 |
  215525.2 |########
  217479.6 |########
  219433.9 |
  221388.2 |
  223342.6 |
  225296.9 |
  227251.3 |
  229205.6 |
  (2 below, 2 above range)

nway (n=18, range 87156.4-389742.3 ns)
  87156.4 |########
  102285.7 |
  117415.0 |
  132544.3 |########################################
  147673.6 |########
  162802.8 |########
  177932.1 |########
  193061.4 |
  208190.7 |
  223320.0 |
  238449.3 |
  253578.6 |
  268707.9 |########
  283837.2 |
  298966.5 |
  314095.8 |########
  329225.1 |
  344354.4 |########
  359483.7 |################
  374613.0 |########
  (2 below, 1 above range)

sequential (n=18, range 192970.7-195919.6 ns)
  192970.7 |########
  193118.1 |################
  193265.6 |
  193413.0 |
  193560.5 |
  193707.9 |
  193855.4 |
  194002.8 |########
  194150.3 |
  194297.7 |
  194445.2 |
  194592.6 |
  194740.0 |
  194887.5 |
  195034.9 |
  195182.4 |########
  195329.8 |########################
  195477.3 |################
  195624.7 |########################################
  195772.2 |
  (2 below, 1 above range)

std_parallel (n=18, range 154999.3-328781.7 ns)
  154999.3 |####################
  163688.4 |##########
  172377.5 |########################################
  181066.6 |##############################
  189755.7 |##########
  198444.9 |##########
  207134.0 |
  215823.1 |
  224512.2 |
  233201.3 |
  241890.5 |##########
  250579.6 |
  259268.7 |
  267957.8 |
  276646.9 |
  285336.1 |
  294025.2 |##########
  302714.3 |
  311403.4 |
  320092.5 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=43.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=104.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=26.9% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=101.2% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=81.6% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=100.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.5% of algo (FFI overhead may distort results)
- **nway**: CV=57.2% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **nway**: bridge=101.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: CV=55.2% (high variance, measurements may be unstable)
- **std_parallel**: bridge=101.9% of algo (FFI overhead may distort results)
