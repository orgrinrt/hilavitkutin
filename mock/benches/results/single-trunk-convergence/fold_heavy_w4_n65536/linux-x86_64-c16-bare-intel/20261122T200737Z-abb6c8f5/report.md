# fold_heavy, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.67)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (122.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_nway, head_tail_dyn} vs {std_parallel, kernel_seq, nway, sequential} (33% apart)

The field splits into a fast tier {head_tail, kernel_nway, head_tail_dyn} and a slow tier {std_parallel, kernel_seq, nway, sequential} with a 33% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### nway is inconsistent: worst-20% is 3.8x its best-20%

nway's best 20% of batches run at 135.36 us but its worst 20% at 514.44 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 122472.9 ns median
- 4 variants significantly slower than baseline
- Spread: 1.60x (fastest 122472.9 ns, slowest 195447.8 ns)

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
| Counter frequency | 2112792619 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 148158ns | 122775ns | 99391ns | 122004ns | 211771ns | base |
| head_tail_dyn | 146631ns | 132781ns | 121036ns | 128816ns | 186150ns | -1.03% |
| kernel_nway | 128065ns | 127373ns | 63255ns | 110076ns | 187454ns | -13.56% |
| kernel_seq | 217511ns | 194615ns | 191760ns | 193995ns | 265658ns | +46.81% |
| nway | 282335ns | 195373ns | 135722ns | 175603ns | 515738ns | +90.56% |
| sequential | 195045ns | 195612ns | 193400ns | 194969ns | 195981ns | +31.65% |
| std_parallel | 246465ns | 176401ns | 157700ns | 174405ns | 398936ns | +66.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 147457ns | 99028ns | 210332ns | base | 0.444 |
| head_tail_dyn | 146091ns | 120825ns | 185003ns | -0.93% | 0.449 |
| kernel_nway | 127475ns | 63035ns | 186317ns | -13.55% | 0.514 |
| kernel_seq | 216385ns | 191555ns | 263196ns | +46.74% | 0.303 |
| nway | 281638ns | 135361ns | 514440ns | +91.00% | 0.233 |
| sequential | 194874ns | 193229ns | 195810ns | +32.16% | 0.336 |
| std_parallel | 245627ns | 157427ns | 397227ns | +66.57% | 0.267 |

## Performance model

- Peak throughput: **1.040 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.535 | 51.5% |
| head_tail_dyn | 0.495 | 47.6% |
| kernel_nway | 0.515 | 49.6% |
| kernel_seq | 0.338 | 32.5% |
| nway | 0.336 | 32.3% |
| sequential | 0.335 | 32.3% |
| std_parallel | 0.372 | 35.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 148158ns | 148158ns | base |
| head_tail_dyn | 146631ns | 146631ns | -1.03% |
| kernel_nway | 128065ns | 128065ns | -13.56% |
| kernel_seq | 217511ns | 217511ns | +46.81% |
| nway | 282335ns | 282335ns | +90.56% |
| sequential | 195045ns | 195045ns | +31.65% |
| std_parallel | 246465ns | 246465ns | +66.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 122473ns | base | --- | [118115, 137961] | --- | --- | --- | --- |
| head_tail_dyn | 132462ns | no significant difference | [-3633, +22506]ns | [121418, 142839] | no | 0.8145 | 0.8145 | 0 |
| kernel_nway | 127171ns | no significant difference | [-52830, +15775]ns | [79191, 130160] | no | 0.5768 | 0.4807 | 0 |
| kernel_seq | 193949ns | +73488.0ns (+60.0%) | [+58074, +78677]ns | [192722, 195601] | YES | 0.0004 | 0.0001 | 0 |
| nway | 195001ns | +40211.2ns (+32.8%) | [+18043, +136514]ns | [152086, 297554] | YES | 0.0113 | 0.0075 | 0 |
| sequential | 195448ns | +73151.1ns (+59.7%) | [+58615, +77425]ns | [193668, 195561] | YES | 0.0113 | 0.0075 | 0 |
| std_parallel | 175941ns | +55631.7ns (+45.4%) | [+40138, +71658]ns | [170319, 245399] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 128066ns | -5.2% | -50.7% | +51.0% | +62.2% | +52.7% | +27.6% |
| 1 | 2 | 118985ns | +20.0% | +6.9% | +63.2% | +54.8% | +64.3% | +50.8% |
| 1 | 3 | 113188ns | +7.3% | +15.0% | +71.4% | +132.4% | +70.9% | +60.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.011 | ok |
| head_tail_dyn | -0.667 | HIGH- (thermal bounce) |
| kernel_nway | -0.144 | ok |
| kernel_seq | -0.350 | moderate- |
| nway | -0.356 | moderate- |
| sequential | -0.146 | ok |
| std_parallel | -0.111 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 147837.3ns | 147457.3ns | 100.3% | HIGH |
| head_tail_dyn | 148416.9ns | 146091.1ns | 101.6% | HIGH |
| kernel_nway | 129673.2ns | 127475.4ns | 101.7% | HIGH |
| kernel_seq | 219468.9ns | 216384.7ns | 101.4% | HIGH |
| nway | 280331.8ns | 281637.7ns | 99.5% | HIGH |
| sequential | 195100.0ns | 194874.4ns | 100.1% | HIGH |
| std_parallel | 246685.0ns | 245626.9ns | 100.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 99028.4-210331.7 ns)
  99028.4 |########
  104593.6 |
  110158.8 |
  115723.9 |########################################
  121289.1 |########################
  126854.2 |
  132419.4 |################
  137984.6 |########
  143549.7 |
  149114.9 |########
  154680.1 |
  160245.2 |
  165810.4 |
  171375.5 |
  176940.7 |
  182505.9 |
  188071.0 |
  193636.2 |
  199201.4 |
  204766.5 |
  (2 below, 3 above range)

head_tail_dyn (n=18, range 120825.0-185002.6 ns)
  120825.0 |########################################
  124033.9 |
  127242.8 |
  130451.7 |###########
  133660.5 |
  136869.4 |
  140078.3 |######################
  143287.2 |###########
  146496.1 |
  149704.9 |
  152913.8 |
  156122.7 |
  159331.6 |
  162540.5 |
  165749.3 |
  168958.2 |
  172167.1 |
  175376.0 |
  178584.9 |
  181793.7 |
  (1 below, 2 above range)

kernel_nway (n=18, range 63035.3-186316.9 ns)
  63035.3 |###############
  69199.3 |
  75363.4 |
  81527.5 |
  87691.6 |
  93855.7 |#####
  100019.8 |
  106183.9 |
  112347.9 |
  118512.0 |
  124676.1 |########################################
  130840.2 |
  137004.3 |#####
  143168.4 |#####
  149332.4 |
  155496.5 |
  161660.6 |
  167824.7 |
  173988.8 |
  180152.9 |
  (2 below, 2 above range)

kernel_seq (n=18, range 191555.4-263196.4 ns)
  191555.4 |########################################
  195137.4 |########
  198719.5 |
  202301.5 |####
  205883.6 |
  209465.6 |
  213047.7 |
  216629.7 |
  220211.8 |
  223793.8 |
  227375.9 |
  230957.9 |
  234540.0 |
  238122.0 |
  241704.1 |
  245286.1 |
  248868.2 |
  252450.2 |
  256032.3 |
  259614.3 |
  (2 below, 3 above range)

nway (n=18, range 135360.6-514439.6 ns)
  135360.6 |########################################
  154314.5 |######
  173268.5 |
  192222.4 |##########################
  211176.4 |
  230130.3 |
  249084.3 |######
  268038.2 |
  286992.2 |
  305946.1 |
  324900.1 |######
  343854.0 |
  362808.0 |
  381761.9 |
  400715.9 |
  419669.8 |
  438623.8 |
  457577.7 |######
  476531.7 |
  495485.6 |
  (1 below, 3 above range)

sequential (n=18, range 193229.0-195809.9 ns)
  193229.0 |
  193358.0 |##########
  193487.1 |####################
  193616.1 |
  193745.1 |##########
  193874.2 |
  194003.2 |
  194132.3 |
  194261.3 |
  194390.4 |
  194519.4 |
  194648.5 |
  194777.5 |
  194906.6 |
  195035.6 |
  195164.7 |####################
  195293.7 |
  195422.7 |########################################
  195551.8 |########################################
  195680.8 |##########
  (2 below, 1 above range)

std_parallel (n=18, range 157427.3-397227.0 ns)
  157427.3 |###########
  169417.3 |########################################
  181407.3 |###########
  193397.3 |
  205387.2 |
  217377.2 |
  229367.2 |
  241357.2 |
  253347.2 |
  265337.2 |
  277327.2 |
  289317.2 |
  301307.1 |#####
  313297.1 |#####
  325287.1 |
  337277.1 |#####
  349267.1 |#####
  361257.1 |
  373247.1 |
  385237.0 |
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: CV=41.1% (high variance, measurements may be unstable)
- **head_tail**: bridge=99.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=30.6% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=101.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=49.8% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=99.9% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=27.3% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=99.8% of algo (FFI overhead may distort results)
- **nway**: CV=71.0% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **nway**: bridge=101.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: CV=67.8% (high variance, measurements may be unstable)
- **std_parallel**: bridge=100.7% of algo (FFI overhead may distort results)
