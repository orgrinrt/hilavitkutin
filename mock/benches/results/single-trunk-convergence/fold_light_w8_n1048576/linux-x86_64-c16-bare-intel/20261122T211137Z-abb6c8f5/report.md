# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 14% faster than the next best (sequential)

kernel_nway (89.67 us) leads sequential (102.01 us) by 14%, a clear separation rather than a photo finish. CV 82.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 33% (significant)

kernel_nway is -34.42 us (33%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### kernel_nway is fastest but the noisiest (CV 82.3%)

kernel_nway wins on median (89.67 us) yet has the highest variance (CV 82.3%), while sequential is the steadiest (CV 4.7%, 102.01 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### kernel_nway is inconsistent: worst-20% is 4.8x its best-20%

kernel_nway's best 20% of batches run at 39.25 us but its worst 20% at 188.82 us (4.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 89668.5 ns median (-13.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.94x (fastest 89668.5 ns, slowest 173820.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112793617 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 131138ns | 104081ns | 78309ns | 103935ns | 198357ns | base |
| head_tail_dyn | 192481ns | 174189ns | 149444ns | 166215ns | 253397ns | +46.78% |
| kernel_nway | 110836ns | 90731ns | 39547ns | 81110ns | 191068ns | -15.48% |
| kernel_seq | 138440ns | 106438ns | 101462ns | 106934ns | 204188ns | +5.57% |
| nway | 118015ns | 103845ns | 58664ns | 92070ns | 186607ns | -10.01% |
| sequential | 99522ns | 102299ns | 92606ns | 99166ns | 103515ns | -24.11% |
| std_parallel | 145727ns | 139079ns | 111226ns | 131397ns | 184474ns | +11.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 130539ns | 78010ns | 197134ns | base | 8.033 |
| head_tail_dyn | 191663ns | 148936ns | 251774ns | +46.82% | 5.471 |
| kernel_nway | 109625ns | 39249ns | 188820ns | -16.02% | 9.565 |
| kernel_seq | 137874ns | 101307ns | 203036ns | +5.62% | 7.605 |
| nway | 117471ns | 58431ns | 185677ns | -10.01% | 8.926 |
| sequential | 99251ns | 92355ns | 103240ns | -23.97% | 10.565 |
| std_parallel | 145066ns | 110844ns | 183341ns | +11.13% | 7.228 |

## Performance model

- Peak throughput: **26.716 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 10.103 | 37.8% |
| head_tail_dyn | 6.033 | 22.6% |
| kernel_nway | 11.694 | 43.8% |
| kernel_seq | 9.894 | 37.0% |
| nway | 10.142 | 38.0% |
| sequential | 10.279 | 38.5% |
| std_parallel | 7.566 | 28.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 131138ns | 131138ns | base |
| head_tail_dyn | 192481ns | 192481ns | +46.78% |
| kernel_nway | 110836ns | 110836ns | -15.48% |
| kernel_seq | 138440ns | 138440ns | +5.57% |
| nway | 118015ns | 118015ns | -10.01% |
| sequential | 99522ns | 99522ns | -24.11% |
| std_parallel | 145727ns | 145727ns | +11.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 103793ns | base | --- | [98348, 140200] | --- | --- | --- | --- |
| head_tail_dyn | 173821ns | +59896.3ns (+57.7%) | [+40593, +80524]ns | [158858, 221402] | YES | 0.0009 | 0.0001 | 0 |
| kernel_nway | 89668ns | -34423.4ns (-33.2%) | [-53378, -2726]ns | [61385, 110193] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| kernel_seq | 105978ns | no significant difference | [-7772, +29931]ns | [103803, 180866] | no | 0.1444 | 0.0963 | 0 |
| nway | 103394ns | no significant difference | [-58422, +12336]ns | [79349, 118626] | no | 0.2855 | 0.2379 | 0 |
| sequential | 102010ns | no significant difference | [-37995, +4882]ns | [92595, 102757] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 138598ns | +27342.5ns (+26.3%) | [+10592, +35921]ns | [114520, 162698] | YES (adj: no) | 0.0927 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 104116ns | +83.3% | -50.7% | +81.3% | +25.3% | -1.9% | +43.3% |
| 1 | 2 | 103586ns | +53.6% | +0.9% | +0.8% | +4.2% | -10.7% | +16.9% |
| 1 | 3 | 100446ns | +74.3% | -2.8% | +3.3% | -33.8% | +2.8% | +27.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.096 | ok |
| head_tail_dyn | -0.501 | HIGH- (thermal bounce) |
| kernel_nway | -0.239 | moderate- |
| kernel_seq | -0.163 | ok |
| nway | -0.019 | ok |
| sequential | -0.660 | HIGH- (thermal bounce) |
| std_parallel | -0.323 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 132692.9ns | 130538.9ns | 101.7% | HIGH |
| head_tail_dyn | 192919.3ns | 191663.0ns | 100.7% | HIGH |
| kernel_nway | 116023.2ns | 109625.1ns | 105.8% | HIGH |
| kernel_seq | 139209.1ns | 137873.8ns | 101.0% | HIGH |
| nway | 115955.5ns | 117470.5ns | 98.7% | HIGH |
| sequential | 99864.4ns | 99251.2ns | 100.6% | HIGH |
| std_parallel | 147207.7ns | 145066.1ns | 101.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 78010.3-197133.9 ns)
  78010.3 |
  83966.4 |########
  89922.6 |
  95878.8 |########################
  101835.0 |########################################
  107791.2 |
  113747.4 |
  119703.6 |########
  125659.7 |########
  131615.9 |
  137572.1 |
  143528.3 |
  149484.5 |########
  155440.7 |
  161396.8 |
  167353.0 |
  173309.2 |
  179265.4 |
  185221.6 |########
  191177.8 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 148935.5-251774.2 ns)
  148935.5 |##########################
  154077.4 |########################################
  159219.4 |##########################
  164361.3 |
  169503.2 |#############
  174645.2 |########################################
  179787.1 |
  184929.0 |
  190071.0 |
  195212.9 |
  200354.9 |
  205496.8 |#############
  210638.7 |
  215780.7 |
  220922.6 |
  226064.5 |
  231206.5 |#############
  236348.4 |#############
  241490.3 |
  246632.3 |
  (1 below, 3 above range)

kernel_nway (n=18, range 39248.9-188819.9 ns)
  39248.9 |#############
  46727.4 |#############
  54206.0 |#############
  61684.5 |#############
  69163.1 |#############
  76641.6 |
  84120.2 |########################################
  91598.7 |
  99077.3 |#############
  106555.8 |########################################
  114034.4 |#############
  121512.9 |
  128991.5 |
  136470.0 |
  143948.6 |
  151427.2 |
  158905.7 |
  166384.3 |
  173862.8 |
  181341.4 |
  (2 below, 3 above range)

kernel_seq (n=18, range 101306.9-203036.1 ns)
  101306.9 |########################################
  106393.4 |####
  111479.8 |
  116566.3 |####
  121652.7 |
  126739.2 |
  131825.6 |
  136912.1 |
  141998.6 |
  147085.0 |
  152171.5 |
  157257.9 |
  162344.4 |
  167430.9 |
  172517.3 |####
  177603.8 |
  182690.2 |####
  187776.7 |#############
  192863.1 |
  197949.6 |
  (1 below, 1 above range)

nway (n=18, range 58431.0-185677.3 ns)
  58431.0 |########################################
  64793.3 |####################
  71155.6 |####################
  77517.9 |####################
  83880.3 |########################################
  90242.6 |
  96604.9 |####################
  102967.2 |########################################
  109329.5 |########################################
  115691.8 |
  122054.1 |####################
  128416.5 |
  134778.8 |####################
  141141.1 |
  147503.4 |####################
  153865.7 |
  160228.0 |
  166590.3 |
  172952.7 |
  179315.0 |
  (1 below, 2 above range)

sequential (n=18, range 92355.4-103240.5 ns)
  92355.4 |########################################
  92899.7 |
  93443.9 |
  93988.2 |
  94532.4 |
  95076.7 |
  95620.9 |
  96165.2 |
  96709.4 |
  97253.7 |
  97797.9 |
  98342.2 |
  98886.5 |
  99430.7 |
  99975.0 |
  100519.2 |
  101063.5 |
  101607.7 |################################
  102152.0 |########################
  102696.2 |########
  (1 below, 4 above range)

std_parallel (n=18, range 110843.9-183340.7 ns)
  110843.9 |########################################
  114468.8 |##########
  118093.6 |
  121718.4 |
  125343.3 |
  128968.1 |####################
  132593.0 |
  136217.8 |##########
  139842.6 |##########
  143467.5 |
  147092.3 |####################
  150717.1 |
  154342.0 |
  157966.8 |##########
  161591.6 |
  165216.5 |##########
  168841.3 |
  172466.2 |
  176091.0 |##########
  179715.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=42.4% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=24.3% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=67.4% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 4.8x (possible bimodal distribution)
- **kernel_nway**: bridge=100.3% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=37.2% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=101.9% of algo (FFI overhead may distort results)
- **nway**: CV=57.7% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **nway**: bridge=101.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.3% of algo (FFI overhead may distort results)
- **std_parallel**: CV=21.5% (high variance, measurements may be unstable)
- **std_parallel**: bridge=98.9% of algo (FFI overhead may distort results)
