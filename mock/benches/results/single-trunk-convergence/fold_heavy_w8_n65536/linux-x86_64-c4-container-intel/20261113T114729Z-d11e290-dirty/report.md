# fold_heavy, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway shows alternating (throttle bounce) (autocorr -0.62)

nway's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### head_tail is inconsistent: worst-20% is 2.1x its best-20%

head_tail's best 20% of batches run at 152.27 us but its worst 20% at 326.11 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 192638.9 ns median (-27.6% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.82x (fastest 192638.9 ns, slowest 351181.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 248908ns | 267165ns | 152688ns | 228562ns | 327536ns | base |
| head_tail_dyn | 183950ns | 193242ns | 158638ns | 183852ns | 196753ns | -26.10% |
| kernel_nway | 200157ns | 196006ns | 166943ns | 191358ns | 229962ns | -19.59% |
| kernel_seq | 372768ns | 352828ns | 305759ns | 348952ns | 441997ns | +49.76% |
| nway | 229956ns | 233737ns | 193777ns | 224574ns | 256118ns | -7.61% |
| sequential | 276830ns | 276089ns | 255784ns | 273383ns | 292523ns | +11.22% |
| std_parallel | 225754ns | 210624ns | 175851ns | 205763ns | 280692ns | -9.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 247913ns | 152270ns | 326112ns | base | 0.264 |
| head_tail_dyn | 183258ns | 158218ns | 195969ns | -26.08% | 0.358 |
| kernel_nway | 199433ns | 165995ns | 229282ns | -19.56% | 0.329 |
| kernel_seq | 371565ns | 304955ns | 440587ns | +49.88% | 0.176 |
| nway | 229433ns | 193335ns | 255552ns | -7.45% | 0.286 |
| sequential | 276313ns | 255391ns | 292081ns | +11.46% | 0.237 |
| std_parallel | 225082ns | 175132ns | 279934ns | -9.21% | 0.291 |

## Performance model

- Peak throughput: **0.430 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.246 | 57.2% |
| head_tail_dyn | 0.340 | 79.0% |
| kernel_nway | 0.335 | 77.9% |
| kernel_seq | 0.187 | 43.4% |
| nway | 0.281 | 65.3% |
| sequential | 0.238 | 55.3% |
| std_parallel | 0.312 | 72.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 248908ns | 248908ns | base |
| head_tail_dyn | 183950ns | 183950ns | -26.10% |
| kernel_nway | 200157ns | 200157ns | -19.59% |
| kernel_seq | 372768ns | 372768ns | +49.76% |
| nway | 229956ns | 229956ns | -7.61% |
| sequential | 276830ns | 276830ns | +11.22% |
| std_parallel | 225754ns | 225754ns | -9.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 266114ns | base | --- | [159879, 299254] | --- | --- | --- | --- |
| head_tail_dyn | 192639ns | no significant difference | [-104283, +3051]ns | [166233, 194942] | no | 0.3568 | 0.2379 | 0 |
| kernel_nway | 195399ns | no significant difference | [-117412, +20903]ns | [182610, 212061] | no | 0.5768 | 0.4807 | 0 |
| kernel_seq | 351182ns | +98257.7ns (+36.9%) | [+82023, +155258]ns | [336786, 425117] | YES | 0.0000 | 0.0000 | 0 |
| nway | 233244ns | no significant difference | [-76206, +75412]ns | [212342, 244909] | no | 0.3568 | 0.2379 | 0 |
| sequential | 275592ns | no significant difference | [-17834, +96983]ns | [265801, 286213] | no | 0.8145 | 0.8145 | 0 |
| std_parallel | 209792ns | no significant difference | [-100782, +45429]ns | [198645, 218883] | no | 0.3568 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 156976ns | +1.9% | +23.9% | +108.3% | +56.0% | +71.1% | +44.4% |
| 1 | 2 | 266114ns | -27.5% | -14.8% | +28.8% | -21.1% | +6.3% | -30.1% |
| 1 | 3 | 326469ns | -40.3% | -44.5% | +31.7% | -27.9% | -15.6% | -35.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.018 | ok |
| head_tail_dyn | -0.139 | ok |
| kernel_nway | -0.613 | HIGH- (thermal bounce) |
| kernel_seq | -0.093 | ok |
| nway | -0.620 | HIGH- (thermal bounce) |
| sequential | -0.508 | HIGH- (thermal bounce) |
| std_parallel | -0.552 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 249863.1ns | 247913.3ns | 100.8% | HIGH |
| head_tail_dyn | 184248.5ns | 183257.7ns | 100.5% | HIGH |
| kernel_nway | 193856.9ns | 199433.4ns | 97.2% | HIGH |
| kernel_seq | 380975.5ns | 371564.7ns | 102.5% | HIGH |
| nway | 234124.9ns | 229433.3ns | 102.0% | HIGH |
| sequential | 284734.7ns | 276312.6ns | 103.0% | HIGH |
| std_parallel | 219811.8ns | 225082.4ns | 97.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 152269.8-326112.2 ns)
  152269.8 |########################################
  160962.0 |
  169654.1 |
  178346.2 |
  187038.3 |
  195730.4 |
  204422.5 |
  213114.7 |
  221806.8 |
  230498.9 |
  239191.0 |################
  247883.1 |
  256575.3 |########
  265267.4 |########
  273959.5 |################
  282651.6 |########
  291343.7 |
  300035.9 |
  308728.0 |########
  317420.1 |########
  (1 below, 3 above range)

head_tail_dyn (n=18, range 158217.6-195968.9 ns)
  158217.6 |####################
  160105.2 |##########
  161992.7 |
  163880.3 |##########
  165767.9 |##########
  167655.4 |
  169543.0 |
  171430.6 |
  173318.1 |
  175205.7 |
  177093.3 |
  178980.8 |
  180868.4 |
  182756.0 |
  184643.5 |
  186531.1 |
  188418.7 |
  190306.2 |####################
  192193.8 |########################################
  194081.4 |########################################
  (1 below, 2 above range)

kernel_nway (n=18, range 165995.4-229282.3 ns)
  165995.4 |
  169159.8 |
  172324.1 |
  175488.5 |####################
  178652.8 |########################################
  181817.2 |####################
  184981.5 |####################
  188145.8 |########################################
  191310.2 |
  194474.5 |
  197638.9 |####################
  200803.2 |########################################
  203967.6 |
  207131.9 |####################
  210296.3 |
  213460.6 |
  216625.0 |####################
  219789.3 |####################
  222953.6 |
  226118.0 |
  (2 below, 3 above range)

kernel_seq (n=18, range 304954.9-440587.0 ns)
  304954.9 |########################################
  311736.5 |
  318518.1 |
  325299.7 |
  332081.3 |#############
  338862.9 |########################################
  345644.5 |#############
  352426.1 |#############
  359207.7 |
  365989.3 |
  372770.9 |#############
  379552.5 |#############
  386334.1 |
  393115.7 |
  399897.3 |
  406678.9 |
  413460.6 |
  420242.2 |##########################
  427023.8 |##########################
  433805.4 |
  (1 below, 2 above range)

nway (n=18, range 193334.9-255551.5 ns)
  193334.9 |#############
  196445.7 |
  199556.5 |
  202667.4 |
  205778.2 |
  208889.0 |########################################
  211999.9 |##########################
  215110.7 |
  218221.5 |
  221332.4 |#############
  224443.2 |
  227554.0 |
  230664.9 |#############
  233775.7 |#############
  236886.5 |#############
  239997.4 |##########################
  243108.2 |
  246219.0 |#############
  249329.9 |#############
  252440.7 |
  (1 below, 3 above range)

sequential (n=18, range 255390.5-292081.0 ns)
  255390.5 |########################################
  257225.1 |
  259059.6 |
  260894.1 |
  262728.6 |####################
  264563.1 |########################################
  266397.7 |
  268232.2 |
  270066.7 |####################
  271901.2 |####################
  273735.8 |####################
  275570.3 |
  277404.8 |####################
  279239.3 |####################
  281073.8 |
  282908.4 |
  284742.9 |########################################
  286577.4 |####################
  288411.9 |########################################
  290246.4 |
  (1 below, 2 above range)

std_parallel (n=18, range 175131.5-279933.5 ns)
  175131.5 |
  180371.6 |#############
  185611.7 |#############
  190851.8 |
  196091.9 |##########################
  201332.0 |##########################
  206572.1 |##########################
  211812.2 |########################################
  217052.3 |
  222292.4 |#############
  227532.5 |##########################
  232772.6 |
  238012.7 |
  243252.8 |
  248492.9 |
  253733.0 |
  258973.1 |
  264213.2 |
  269453.3 |
  274693.4 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=29.2% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.2% of algo (FFI overhead may distort results)
- **nway**: bridge=98.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=98.6% of algo (FFI overhead may distort results)
- **std_parallel**: CV=31.2% (high variance, measurements may be unstable)
- **std_parallel**: bridge=100.3% of algo (FFI overhead may distort results)
