# fold_light, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn is an outlier: 2.2x slower than the field

head_tail_dyn (291.70 us) is 2.2x the fastest (130.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.66)

kernel_seq's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### kernel_seq is inconsistent: worst-20% is 1.7x its best-20%

kernel_seq's best 20% of batches run at 189.52 us but its worst 20% at 321.95 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 130008.2 ns median (-3.9% vs baseline)
- 5 variants significantly slower than baseline
- Spread: 2.24x (fastest 130008.2 ns, slowest 291697.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 150068ns | 136211ns | 126350ns | 135348ns | 184007ns | base |
| head_tail_dyn | 315987ns | 292490ns | 269393ns | 290547ns | 377445ns | +110.56% |
| kernel_nway | 139722ns | 130524ns | 117013ns | 127872ns | 168851ns | -6.89% |
| kernel_seq | 245042ns | 214178ns | 189896ns | 211490ns | 322943ns | +63.29% |
| nway | 185180ns | 191795ns | 136787ns | 178645ns | 219180ns | +23.40% |
| sequential | 174043ns | 175210ns | 161287ns | 171664ns | 183990ns | +15.98% |
| std_parallel | 186397ns | 169604ns | 146191ns | 166263ns | 236703ns | +24.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 149443ns | 125985ns | 183255ns | base | 7.017 |
| head_tail_dyn | 314957ns | 268400ns | 376554ns | +110.75% | 3.329 |
| kernel_nway | 139000ns | 116393ns | 167956ns | -6.99% | 7.544 |
| kernel_seq | 244397ns | 189521ns | 321954ns | +63.54% | 4.290 |
| nway | 184768ns | 136441ns | 218681ns | +23.64% | 5.675 |
| sequential | 173217ns | 160095ns | 183210ns | +15.91% | 6.054 |
| std_parallel | 185256ns | 145022ns | 235495ns | +23.96% | 5.660 |

## Performance model

- Peak throughput: **9.009 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 7.751 | 86.0% |
| head_tail_dyn | 3.595 | 39.9% |
| kernel_nway | 8.065 | 89.5% |
| kernel_seq | 4.905 | 54.4% |
| nway | 5.477 | 60.8% |
| sequential | 6.009 | 66.7% |
| std_parallel | 6.235 | 69.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 150068ns | 150068ns | base |
| head_tail_dyn | 315987ns | 315987ns | +110.56% |
| kernel_nway | 139722ns | 139722ns | -6.89% |
| kernel_seq | 245042ns | 245042ns | +63.29% |
| nway | 185180ns | 185180ns | +23.40% |
| sequential | 174043ns | 174043ns | +15.98% |
| std_parallel | 186397ns | 186397ns | +24.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 135289ns | base | --- | [131332, 143754] | --- | --- | --- | --- |
| head_tail_dyn | 291697ns | +154194.4ns (+114.0%) | [+152717, +167562]ns | [282506, 369335] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 130008ns | no significant difference | [-23796, +9303]ns | [120883, 143121] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 213770ns | +67012.4ns (+49.5%) | [+55258, +104796]ns | [196097, 254740] | YES | 0.0004 | 0.0001 | 0 |
| nway | 191463ns | +44251.7ns (+32.7%) | [+5455, +63376]ns | [162787, 206838] | YES | 0.0371 | 0.0309 | 0 |
| sequential | 174500ns | +34946.0ns (+25.8%) | [+26946, +37908]ns | [166957, 177520] | YES | 0.0151 | 0.0075 | 0 |
| std_parallel | 168176ns | +33045.4ns (+24.4%) | [+17922, +50462]ns | [158938, 187886] | YES | 0.0371 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 137812ns | +111.7% | +6.7% | +44.6% | +7.8% | +28.6% | +25.8% |
| 1 | 2 | 152838ns | +145.3% | -22.7% | +66.7% | +21.9% | +13.0% | +3.6% |
| 1 | 3 | 130695ns | +110.7% | +5.2% | +55.0% | +65.4% | +23.6% | +39.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.601 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.651 | HIGH- (thermal bounce) |
| kernel_nway | -0.596 | HIGH- (thermal bounce) |
| kernel_seq | -0.665 | HIGH- (thermal bounce) |
| nway | -0.003 | ok |
| sequential | -0.037 | ok |
| std_parallel | -0.576 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 147811.8ns | 149442.8ns | 98.9% | HIGH |
| head_tail_dyn | 316860.3ns | 314957.2ns | 100.6% | HIGH |
| kernel_nway | 137308.0ns | 138999.6ns | 98.8% | HIGH |
| kernel_seq | 228919.1ns | 244397.2ns | 93.7% | HIGH |
| nway | 175622.6ns | 184767.9ns | 95.1% | HIGH |
| sequential | 175995.9ns | 173217.4ns | 101.6% | HIGH |
| std_parallel | 186770.3ns | 185256.3ns | 100.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 125984.6-183255.4 ns)
  125984.6 |
  128848.1 |########################################
  131711.6 |####################
  134575.2 |####################
  137438.7 |##########
  140302.3 |####################
  143165.8 |
  146029.4 |##########
  148892.9 |
  151756.4 |
  154620.0 |
  157483.5 |##########
  160347.1 |
  163210.6 |
  166074.1 |
  168937.7 |
  171801.2 |
  174664.8 |
  177528.3 |
  180391.9 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 268400.3-376554.5 ns)
  268400.3 |####################
  273808.0 |
  279215.7 |########################################
  284623.4 |####################
  290031.1 |##########
  295438.8 |##########
  300846.5 |
  306254.2 |##########
  311661.9 |
  317069.7 |
  322477.4 |
  327885.1 |
  333292.8 |
  338700.5 |
  344108.2 |
  349515.9 |
  354923.6 |
  360331.3 |
  365739.0 |####################
  371146.8 |####################
  (1 below, 2 above range)

kernel_nway (n=18, range 116392.8-167956.4 ns)
  116392.8 |########################################
  118970.9 |########################################
  121549.1 |####################
  124127.3 |####################
  126705.5 |####################
  129283.7 |####################
  131861.8 |
  134440.0 |####################
  137018.2 |####################
  139596.4 |
  142174.6 |########################################
  144752.7 |####################
  147330.9 |
  149909.1 |
  152487.3 |
  155065.5 |####################
  157643.6 |
  160221.8 |
  162800.0 |####################
  165378.2 |
  (2 below, 1 above range)

kernel_seq (n=18, range 189520.6-321954.3 ns)
  189520.6 |########################################
  196142.3 |##########################
  202764.0 |
  209385.7 |########################################
  216007.4 |
  222629.0 |#############
  229250.7 |
  235872.4 |
  242494.1 |
  249115.8 |##########################
  255737.4 |#############
  262359.1 |#############
  268980.8 |
  275602.5 |
  282224.2 |
  288845.9 |
  295467.5 |#############
  302089.2 |
  308710.9 |
  315332.6 |
  (2 below, 2 above range)

nway (n=18, range 136441.0-218681.3 ns)
  136441.0 |########################################
  140553.0 |
  144665.0 |
  148777.0 |
  152889.0 |
  157001.0 |#############
  161113.1 |
  165225.1 |#############
  169337.1 |
  173449.1 |
  177561.1 |
  181673.1 |##########################
  185785.2 |#############
  189897.2 |#############
  194009.2 |##########################
  198121.2 |
  202233.2 |
  206345.2 |#############
  210457.3 |#############
  214569.3 |##########################
  (1 below, 2 above range)

sequential (n=18, range 160095.3-183210.4 ns)
  160095.3 |########################################
  161251.1 |####################
  162406.8 |
  163562.6 |
  164718.3 |
  165874.1 |########################################
  167029.8 |
  168185.6 |
  169341.3 |
  170497.1 |########################################
  171652.8 |
  172808.6 |
  173964.3 |########################################
  175120.1 |########################################
  176275.8 |####################
  177431.6 |####################
  178587.3 |####################
  179743.1 |
  180898.8 |####################
  182054.6 |####################
  (1 below, 1 above range)

std_parallel (n=18, range 145021.7-235494.9 ns)
  145021.7 |##########################
  149545.4 |#############
  154069.0 |#############
  158592.7 |#############
  163116.3 |########################################
  167640.0 |#############
  172163.6 |##########################
  176687.3 |
  181211.0 |
  185734.6 |##########################
  190258.3 |
  194781.9 |
  199305.6 |
  203829.2 |
  208352.9 |
  212876.6 |
  217400.2 |#############
  221923.9 |
  226447.5 |
  230971.2 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=21.3% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.6% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.7% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=22.7% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=101.0% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=29.6% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=104.3% of algo (FFI overhead may distort results)
- **nway**: bridge=96.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.6% of algo (FFI overhead may distort results)
- **std_parallel**: CV=24.7% (high variance, measurements may be unstable)
- **std_parallel**: bridge=99.5% of algo (FFI overhead may distort results)
