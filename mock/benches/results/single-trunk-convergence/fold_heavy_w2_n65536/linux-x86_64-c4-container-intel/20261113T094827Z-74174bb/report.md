# fold_heavy, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (head_tail, kernel_nway) are a dead heat (<1%)

head_tail (151.60 us) and kernel_nway (151.69 us) differ by 0.06%, inside the noise, even though the wider field spreads 93.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (151.60 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_nway, head_tail_dyn, nway, std_parallel} vs {sequential, kernel_seq} (48% apart)

The field splits into a fast tier {head_tail, kernel_nway, head_tail_dyn, nway, std_parallel} and a slow tier {sequential, kernel_seq} with a 48% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### nway is inconsistent: worst-20% is 2.1x its best-20%

nway's best 20% of batches run at 158.26 us but its worst 20% at 334.36 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (head_tail) is the fastest** at 151598.0 ns median
- 5 variants significantly slower than baseline
- Spread: 1.94x (fastest 151598.0 ns, slowest 293627.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 159982ns | 152586ns | 148059ns | 151921ns | 178035ns | base |
| head_tail_dyn | 183904ns | 167314ns | 154434ns | 168249ns | 222121ns | +14.95% |
| kernel_nway | 196187ns | 152417ns | 148509ns | 152995ns | 284815ns | +22.63% |
| kernel_seq | 321033ns | 294467ns | 282011ns | 292254ns | 383712ns | +100.67% |
| nway | 221447ns | 167260ns | 158619ns | 166538ns | 335226ns | +38.42% |
| sequential | 322764ns | 274443ns | 252162ns | 279015ns | 423688ns | +101.75% |
| std_parallel | 200392ns | 185541ns | 172994ns | 183829ns | 238935ns | +25.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 159151ns | 147663ns | 176874ns | base | 0.412 |
| head_tail_dyn | 183231ns | 153932ns | 221491ns | +15.13% | 0.358 |
| kernel_nway | 195349ns | 147968ns | 283623ns | +22.74% | 0.335 |
| kernel_seq | 320054ns | 281269ns | 382599ns | +101.10% | 0.205 |
| nway | 220921ns | 158264ns | 334363ns | +38.81% | 0.297 |
| sequential | 322070ns | 251818ns | 422429ns | +102.37% | 0.203 |
| std_parallel | 199836ns | 172600ns | 238334ns | +25.56% | 0.328 |

## Performance model

- Peak throughput: **0.444 Gops/s** (head_tail; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.432 | 97.4% |
| head_tail_dyn | 0.394 | 88.7% |
| kernel_nway | 0.432 | 97.3% |
| kernel_seq | 0.223 | 50.3% |
| nway | 0.393 | 88.5% |
| sequential | 0.239 | 53.9% |
| std_parallel | 0.355 | 79.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 159982ns | 159982ns | base |
| head_tail_dyn | 183904ns | 183904ns | +14.95% |
| kernel_nway | 196187ns | 196187ns | +22.63% |
| kernel_seq | 321033ns | 321033ns | +100.67% |
| nway | 221447ns | 221447ns | +38.42% |
| sequential | 322764ns | 322764ns | +101.75% |
| std_parallel | 200392ns | 200392ns | +25.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 151598ns | base | --- | [148959, 158274] | --- | --- | --- | --- |
| head_tail_dyn | 166422ns | +15210.6ns (+10.0%) | [+8550, +41937]ns | [160969, 191650] | YES | 0.0016 | 0.0013 | 0 |
| kernel_nway | 151687ns | no significant difference | [-1094, +112620]ns | [149009, 270314] | no | 0.4807 | 0.4807 | 0 |
| kernel_seq | 293627ns | +143968.2ns (+95.0%) | [+133280, +156338]ns | [285953, 325789] | YES | 0.0000 | 0.0000 | 0 |
| nway | 166910ns | +14695.9ns (+9.7%) | [+9614, +65353]ns | [159606, 225842] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 274098ns | +110292.2ns (+72.8%) | [+103656, +231316]ns | [257942, 387556] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 184859ns | +33499.0ns (+22.1%) | [+23207, +45342]ns | [179986, 207089] | YES | 0.0016 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 152433ns | +21.2% | +84.8% | +90.9% | +8.2% | +173.2% | +19.2% |
| 1 | 2 | 149376ns | +19.9% | -0.2% | +91.1% | +6.6% | +70.8% | +25.6% |
| 1 | 3 | 156691ns | +3.3% | -3.5% | +152.4% | +90.1% | +74.9% | +25.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.443 | moderate- |
| head_tail_dyn | -0.053 | ok |
| kernel_nway | -0.175 | ok |
| kernel_seq | -0.194 | ok |
| nway | -0.188 | ok |
| sequential | -0.232 | moderate- |
| std_parallel | -0.010 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 160921.7ns | 159150.7ns | 101.1% | HIGH |
| head_tail_dyn | 178418.7ns | 183231.2ns | 97.4% | HIGH |
| kernel_nway | 196013.3ns | 195349.3ns | 100.3% | HIGH |
| kernel_seq | 319194.1ns | 320053.9ns | 99.7% | HIGH |
| nway | 221121.3ns | 220920.9ns | 100.1% | HIGH |
| sequential | 316393.1ns | 322069.9ns | 98.2% | HIGH |
| std_parallel | 191487.8ns | 199835.8ns | 95.8% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 147663.4-176874.1 ns)
  147663.4 |########################################
  149123.9 |####################
  150584.4 |##########
  152045.0 |##########
  153505.5 |##########
  154966.1 |##########
  156426.6 |##########
  157887.1 |##########
  159347.7 |##########
  160808.2 |
  162268.7 |
  163729.3 |
  165189.8 |
  166650.4 |
  168110.9 |
  169571.4 |
  171032.0 |
  172492.5 |##########
  173953.0 |
  175413.6 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 153931.6-221491.1 ns)
  153931.6 |##########
  157309.5 |####################
  160687.5 |########################################
  164065.5 |##########
  167443.5 |##########
  170821.5 |
  174199.4 |##########
  177577.4 |
  180955.4 |
  184333.4 |
  187711.3 |####################
  191089.3 |##########
  194467.3 |
  197845.3 |
  201223.3 |##########
  204601.2 |
  207979.2 |
  211357.2 |
  214735.2 |
  218113.1 |
  (1 below, 3 above range)

kernel_nway (n=18, range 147968.1-283623.0 ns)
  147968.1 |########################################
  154750.8 |####
  161533.6 |####
  168316.3 |
  175099.1 |
  181881.8 |
  188664.6 |
  195447.3 |
  202230.0 |
  209012.8 |
  215795.5 |
  222578.3 |
  229361.0 |
  236143.8 |
  242926.5 |
  249709.2 |
  256492.0 |
  263274.7 |
  270057.5 |########
  276840.2 |####
  (1 below, 3 above range)

kernel_seq (n=18, range 281268.9-382599.4 ns)
  281268.9 |########################################
  286335.4 |########################################
  291401.9 |##########################
  296468.5 |##########################
  301535.0 |
  306601.5 |
  311668.0 |#############
  316734.6 |
  321801.1 |
  326867.6 |
  331934.1 |#############
  337000.6 |
  342067.2 |
  347133.7 |
  352200.2 |
  357266.7 |
  362333.3 |
  367399.8 |
  372466.3 |#############
  377532.8 |
  (2 below, 3 above range)

nway (n=18, range 158264.0-334363.1 ns)
  158264.0 |########################################
  167069.0 |###########
  175873.9 |#####
  184678.9 |
  193483.8 |
  202288.8 |
  211093.8 |
  219898.7 |#####
  228703.7 |
  237508.6 |
  246313.6 |#####
  255118.5 |
  263923.5 |#####
  272728.4 |
  281533.4 |
  290338.3 |
  299143.3 |
  307948.3 |
  316753.2 |
  325558.2 |#####
  (2 below, 2 above range)

sequential (n=18, range 251817.7-422429.4 ns)
  251817.7 |########################################
  260348.3 |######
  268878.9 |######
  277409.5 |######
  285940.1 |######
  294470.7 |
  303001.2 |
  311531.8 |
  320062.4 |
  328593.0 |
  337123.6 |
  345654.2 |
  354184.8 |
  362715.3 |######
  371245.9 |
  379776.5 |######
  388307.1 |######
  396837.7 |
  405368.3 |######
  413898.8 |
  (1 below, 3 above range)

std_parallel (n=18, range 172599.9-238333.8 ns)
  172599.9 |##########################
  175886.6 |##########################
  179173.3 |##########################
  182460.0 |########################################
  185746.7 |#############
  189033.4 |#############
  192320.1 |
  195606.7 |
  198893.4 |
  202180.1 |
  205466.8 |##########################
  208753.5 |#############
  212040.2 |
  215326.9 |
  218613.6 |
  221900.3 |
  225187.0 |
  228473.7 |
  231760.4 |
  235047.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: bridge=102.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.9% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=32.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=99.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=98.0% of algo (FFI overhead may distort results)
- **nway**: CV=45.5% (high variance, measurements may be unstable)
- **nway**: bridge=100.6% of algo (FFI overhead may distort results)
- **sequential**: CV=24.0% (high variance, measurements may be unstable)
- **sequential**: bridge=100.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.0% of algo (FFI overhead may distort results)
