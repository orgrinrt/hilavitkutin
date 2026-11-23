# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel is an outlier: 2.6x slower than the field

std_parallel (68.26 us) is 2.6x the fastest (25.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (25.85 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### sequential is inconsistent: worst-20% is 3.6x its best-20%

sequential's best 20% of batches run at 12.07 us but its worst 20% at 43.73 us (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 25847.3 ns median
- 5 variants significantly slower than baseline
- Spread: 2.64x (fastest 25847.3 ns, slowest 68258.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112779539 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 27432ns | 26753ns | 24147ns | 26195ns | 30931ns | base |
| head_tail_dyn | 46101ns | 49810ns | 22439ns | 46667ns | 57082ns | +68.05% |
| kernel_nway | 33564ns | 29050ns | 24178ns | 28052ns | 46525ns | +22.35% |
| kernel_seq | 30478ns | 29025ns | 23548ns | 27990ns | 37677ns | +11.11% |
| nway | 55807ns | 53180ns | 44227ns | 50777ns | 69140ns | +103.44% |
| sequential | 28864ns | 34625ns | 12225ns | 24105ns | 44322ns | +5.22% |
| std_parallel | 72471ns | 69279ns | 61005ns | 68947ns | 83488ns | +164.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 26625ns | 23582ns | 29801ns | base | 2.461 |
| head_tail_dyn | 45310ns | 22162ns | 55865ns | +70.18% | 1.446 |
| kernel_nway | 32516ns | 23559ns | 44671ns | +22.13% | 2.016 |
| kernel_seq | 29627ns | 23076ns | 36484ns | +11.28% | 2.212 |
| nway | 55268ns | 43786ns | 68528ns | +107.58% | 1.186 |
| sequential | 28486ns | 12065ns | 43731ns | +6.99% | 2.301 |
| std_parallel | 71544ns | 60416ns | 82236ns | +168.71% | 0.916 |

## Performance model

- Peak throughput: **5.432 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.536 | 46.7% |
| head_tail_dyn | 1.338 | 24.6% |
| kernel_nway | 2.309 | 42.5% |
| kernel_seq | 2.380 | 43.8% |
| nway | 1.246 | 22.9% |
| sequential | 1.917 | 35.3% |
| std_parallel | 0.960 | 17.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 27432ns | 27432ns | base |
| head_tail_dyn | 46101ns | 46101ns | +68.05% |
| kernel_nway | 33564ns | 33564ns | +22.35% |
| kernel_seq | 30478ns | 30478ns | +11.11% |
| nway | 55807ns | 55807ns | +103.44% |
| sequential | 28864ns | 28864ns | +5.22% |
| std_parallel | 72471ns | 72471ns | +164.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 25847ns | base | --- | [24940, 27111] | --- | --- | --- | --- |
| head_tail_dyn | 48975ns | +21439.8ns (+82.9%) | [+17458, +23438]ns | [42836, 51730] | YES | 0.0003 | 0.0001 | 0 |
| kernel_nway | 28386ns | +1371.9ns (+5.3%) | [+394, +4161]ns | [26254, 31392] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| kernel_seq | 27539ns | +1509.2ns (+5.8%) | [+60, +4893]ns | [24706, 32816] | YES (adj: no) | 0.1155 | 0.0963 | 0 |
| nway | 52600ns | +26291.3ns (+101.7%) | [+21570, +31748]ns | [45663, 58935] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 34187ns | no significant difference | [-11357, +11283]ns | [12528, 39978] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 68259ns | +42971.6ns (+166.3%) | [+40068, +49011]ns | [66048, 76110] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 25445ns | +97.1% | +9.2% | -2.8% | +86.2% | +18.4% | +162.0% |
| 1 | 2 | 24792ns | +59.7% | -0.4% | +4.3% | +88.9% | -1.5% | +165.5% |
| 1 | 3 | 29244ns | +76.9% | +0.8% | +12.2% | +109.7% | +16.9% | +165.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.250 | moderate- |
| head_tail_dyn | -0.657 | HIGH- (thermal bounce) |
| kernel_nway | -0.586 | HIGH- (thermal bounce) |
| kernel_seq | -0.099 | ok |
| nway | -0.187 | ok |
| sequential | -0.553 | HIGH- (thermal bounce) |
| std_parallel | -0.205 | moderate- |

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
| head_tail | 27205.5ns | 26624.8ns | 102.2% | HIGH |
| head_tail_dyn | 46768.5ns | 45309.9ns | 103.2% | HIGH |
| kernel_nway | 31793.6ns | 32516.0ns | 97.8% | HIGH |
| kernel_seq | 31428.8ns | 29627.0ns | 106.1% | HIGH |
| nway | 55389.2ns | 55267.9ns | 100.2% | HIGH |
| sequential | 29236.4ns | 28485.9ns | 102.6% | HIGH |
| std_parallel | 71681.4ns | 71543.6ns | 100.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 23581.9-29801.4 ns)
  23581.9 |
  23892.8 |
  24203.8 |########################################
  24514.8 |####################
  24825.8 |
  25136.8 |########################################
  25447.7 |####################
  25758.7 |########################################
  26069.7 |####################
  26380.7 |####################
  26691.6 |####################
  27002.6 |
  27313.6 |####################
  27624.6 |
  27935.5 |
  28246.5 |####################
  28557.5 |
  28868.5 |
  29179.4 |
  29490.4 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 22161.7-55864.5 ns)
  22161.7 |
  23846.9 |
  25532.0 |####################
  27217.2 |
  28902.3 |
  30587.4 |
  32272.6 |
  33957.7 |
  35642.9 |
  37328.0 |##########
  39013.1 |
  40698.3 |##########
  42383.4 |
  44068.6 |####################
  45753.7 |##########
  47438.8 |####################
  49124.0 |##########
  50809.1 |########################################
  52494.3 |##########
  54179.4 |
  (1 below, 2 above range)

kernel_nway (n=18, range 23559.4-44671.0 ns)
  23559.4 |#############
  24615.0 |#############
  25670.6 |########################################
  26726.2 |#############
  27781.7 |##########################
  28837.3 |########################################
  29892.9 |
  30948.5 |
  32004.0 |#############
  33059.6 |
  34115.2 |
  35170.8 |
  36226.3 |
  37281.9 |
  38337.5 |##########################
  39393.1 |
  40448.6 |
  41504.2 |
  42559.8 |
  43615.4 |
  (2 below, 2 above range)

kernel_seq (n=18, range 23076.2-36484.2 ns)
  23076.2 |
  23746.6 |##########################
  24417.0 |########################################
  25087.4 |
  25757.8 |
  26428.2 |#############
  27098.6 |#############
  27769.0 |#############
  28439.4 |
  29109.8 |
  29780.2 |
  30450.6 |
  31121.0 |
  31791.4 |##########################
  32461.8 |########################################
  33132.2 |
  33802.6 |
  34473.0 |
  35143.4 |
  35813.8 |#############
  (2 below, 2 above range)

nway (n=18, range 43786.3-68527.8 ns)
  43786.3 |########################################
  45023.4 |
  46260.4 |####################
  47497.5 |
  48734.6 |
  49971.7 |####################
  51208.7 |
  52445.8 |
  53682.9 |##########
  54920.0 |##########
  56157.0 |
  57394.1 |##########
  58631.2 |####################
  59868.3 |
  61105.3 |
  62342.4 |##########
  63579.5 |##########
  64816.5 |
  66053.6 |
  67290.7 |
  (1 below, 2 above range)

sequential (n=18, range 12065.3-43730.7 ns)
  12065.3 |########################################
  13648.6 |##########
  15231.8 |
  16815.1 |
  18398.4 |##########
  19981.6 |
  21564.9 |
  23148.2 |
  24731.4 |
  26314.7 |
  27898.0 |
  29481.2 |
  31064.5 |
  32647.8 |##########
  34231.0 |##########
  35814.3 |##########
  37397.6 |##########
  38980.8 |##########
  40564.1 |##########
  42147.4 |##########
  (2 below, 3 above range)

std_parallel (n=18, range 60416.2-82236.0 ns)
  60416.2 |
  61507.2 |
  62598.2 |
  63689.1 |#############
  64780.1 |#############
  65871.1 |########################################
  66962.1 |##########################
  68053.1 |
  69144.1 |#############
  70235.1 |
  71326.1 |#############
  72417.1 |
  73508.1 |#############
  74599.1 |#############
  75690.1 |#############
  76781.1 |
  77872.0 |#############
  78963.0 |
  80054.0 |#############
  81145.0 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=102.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=27.0% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=102.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=37.0% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=100.3% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=20.2% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=109.5% of algo (FFI overhead may distort results)
- **nway**: CV=22.1% (high variance, measurements may be unstable)
- **nway**: bridge=100.9% of algo (FFI overhead may distort results)
- **sequential**: CV=49.1% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **sequential**: bridge=103.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=103.8% of algo (FFI overhead may distort results)
