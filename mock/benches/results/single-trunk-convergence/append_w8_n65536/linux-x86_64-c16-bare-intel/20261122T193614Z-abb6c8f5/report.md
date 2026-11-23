# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 21% (significant)

kernel_nway is -5.97 us (21%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### nway is an outlier: 4.2x slower than the field

nway (115.69 us) is 4.2x the fastest (27.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.2x the fastest

Fastest kernel_nway (27.76 us) to slowest nway (115.69 us): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 9.2x its best-20%

nway's best 20% of batches run at 39.37 us but its worst 20% at 361.48 us (9.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 27757.4 ns median (-2.4% vs baseline)
- 1 variant significantly faster than baseline
- 5 variants significantly slower than baseline
- Spread: 4.17x (fastest 27757.4 ns, slowest 115693.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112862021 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 50072ns | 28703ns | 23767ns | 29773ns | 93673ns | base |
| head_tail_dyn | 57494ns | 37089ns | 28560ns | 33994ns | 107210ns | +14.82% |
| kernel_nway | 44889ns | 27998ns | 15528ns | 24712ns | 89837ns | -10.35% |
| kernel_seq | 60847ns | 31924ns | 28508ns | 31189ns | 121502ns | +21.52% |
| nway | 179036ns | 116398ns | 39562ns | 101664ns | 364830ns | +257.56% |
| sequential | 57300ns | 57248ns | 56839ns | 57177ns | 57717ns | +14.44% |
| std_parallel | 106407ns | 83803ns | 70054ns | 83017ns | 159668ns | +112.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 49456ns | 23598ns | 92307ns | base | 1.325 |
| head_tail_dyn | 56733ns | 28322ns | 105490ns | +14.71% | 1.155 |
| kernel_nway | 44331ns | 15256ns | 88730ns | -10.36% | 1.478 |
| kernel_seq | 60140ns | 28336ns | 119765ns | +21.60% | 1.090 |
| nway | 177530ns | 39367ns | 361477ns | +258.96% | 0.369 |
| sequential | 57112ns | 56670ns | 57516ns | +15.48% | 1.148 |
| std_parallel | 105771ns | 69654ns | 158668ns | +113.87% | 0.620 |

## Performance model

- Peak throughput: **4.296 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.305 | 53.7% |
| head_tail_dyn | 1.788 | 41.6% |
| kernel_nway | 2.361 | 55.0% |
| kernel_seq | 2.064 | 48.1% |
| nway | 0.566 | 13.2% |
| sequential | 1.149 | 26.7% |
| std_parallel | 0.785 | 18.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 50072ns | 50072ns | base |
| head_tail_dyn | 57494ns | 57494ns | +14.82% |
| kernel_nway | 44889ns | 44889ns | -10.35% |
| kernel_seq | 60847ns | 60847ns | +21.52% |
| nway | 179036ns | 179036ns | +257.56% |
| sequential | 57300ns | 57300ns | +14.44% |
| std_parallel | 106407ns | 106407ns | +112.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 28433ns | base | --- | [27444, 50530] | --- | --- | --- | --- |
| head_tail_dyn | 36649ns | no significant difference | [-183, +5317]ns | [28969, 59217] | no | 0.0963 | 0.0963 | 0 |
| kernel_nway | 27757ns | -5970.1ns (-21.0%) | [-12610, -263]ns | [18839, 44812] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| kernel_seq | 31745ns | +2029.0ns (+7.1%) | [+158, +6367]ns | [28518, 84684] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| nway | 115693ns | +49968.0ns (+175.7%) | [+19030, +219208]ns | [45798, 258758] | YES | 0.0226 | 0.0075 | 0 |
| sequential | 57055ns | +29058.2ns (+102.2%) | [+6702, +29321]ns | [56859, 57226] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| std_parallel | 83490ns | +45671.8ns (+160.6%) | [+29434, +66379]ns | [77271, 98733] | YES | 0.0079 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 26486ns | +19.5% | -17.8% | +19.9% | +72.9% | +116.2% | +215.2% |
| 1 | 2 | 53782ns | -21.5% | -21.5% | +53.6% | +108.5% | +5.9% | +48.1% |
| 1 | 3 | 27569ns | +6.9% | -31.0% | +3.4% | +823.9% | +106.3% | +461.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.666 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.650 | HIGH- (thermal bounce) |
| kernel_nway | -0.659 | HIGH- (thermal bounce) |
| kernel_seq | -0.665 | HIGH- (thermal bounce) |
| nway | -0.028 | ok |
| sequential | -0.091 | ok |
| std_parallel | -0.193 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 403000.6ns | 49456.4ns | 814.9% | HIGH |
| head_tail_dyn | 481170.2ns | 56732.6ns | 848.1% | HIGH |
| kernel_nway | 475809.7ns | 44330.6ns | 1073.3% | HIGH |
| kernel_seq | 440201.2ns | 60140.2ns | 732.0% | HIGH |
| nway | 1045833.6ns | 177530.4ns | 589.1% | HIGH |
| sequential | 297703.2ns | 57111.9ns | 521.3% | HIGH |
| std_parallel | 611464.2ns | 105771.5ns | 578.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 23598.3-92306.8 ns)
  23598.3 |#####
  27033.7 |########################################
  30469.1 |
  33904.6 |
  37340.0 |
  40775.4 |##########
  44210.8 |
  47646.3 |
  51081.7 |
  54517.1 |
  57952.6 |#####
  61388.0 |
  64823.4 |#####
  68258.8 |
  71694.3 |
  75129.7 |
  78565.1 |
  82000.5 |#####
  85436.0 |
  88871.4 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 28321.8-105489.5 ns)
  28321.8 |########################################
  32180.2 |######
  36038.6 |######
  39897.0 |######
  43755.4 |######
  47613.8 |######
  51472.1 |
  55330.5 |
  59188.9 |
  63047.3 |
  66905.7 |#############
  70764.1 |
  74622.4 |
  78480.8 |
  82339.2 |
  86197.6 |
  90056.0 |
  93914.4 |
  97772.7 |
  101631.1 |
  (2 below, 3 above range)

kernel_nway (n=18, range 15255.7-88729.9 ns)
  15255.7 |########################################
  18929.4 |####################
  22603.1 |##########
  26276.9 |####################
  29950.6 |####################
  33624.3 |
  37298.0 |##########
  40971.7 |
  44645.4 |
  48319.1 |
  51992.8 |##########
  55666.5 |
  59340.2 |
  63013.9 |
  66687.7 |
  70361.4 |
  74035.1 |
  77708.8 |
  81382.5 |
  85056.2 |##########
  (1 below, 3 above range)

kernel_seq (n=18, range 28336.1-119765.2 ns)
  28336.1 |########################################
  32907.6 |########
  37479.0 |
  42050.5 |####
  46621.9 |
  51193.4 |
  55764.8 |
  60336.3 |
  64907.7 |
  69479.2 |
  74050.6 |
  78622.1 |
  83193.5 |
  87765.0 |
  92336.4 |
  96907.9 |
  101479.3 |
  106050.8 |
  110622.2 |
  115193.7 |
  (1 below, 5 above range)

nway (n=18, range 39366.6-361476.7 ns)
  39366.6 |########################################
  55472.1 |
  71577.6 |########
  87683.1 |################
  103788.6 |
  119894.1 |########
  135999.6 |
  152105.1 |
  168210.6 |########
  184316.1 |########
  200421.7 |########
  216527.2 |
  232632.7 |
  248738.2 |
  264843.7 |
  280949.2 |
  297054.7 |
  313160.2 |########
  329265.7 |########
  345371.2 |
  (1 below, 3 above range)

sequential (n=18, range 56670.5-57515.9 ns)
  56670.5 |
  56712.8 |
  56755.0 |##########################
  56797.3 |#############
  56839.6 |#############
  56881.8 |
  56924.1 |
  56966.4 |##########################
  57008.7 |
  57050.9 |##########################
  57093.2 |#############
  57135.5 |
  57177.7 |#############
  57220.0 |########################################
  57262.3 |#############
  57304.6 |
  57346.8 |
  57389.1 |
  57431.4 |
  57473.6 |
  (2 below, 2 above range)

std_parallel (n=18, range 69653.6-158668.4 ns)
  69653.6 |##########################
  74104.3 |########################################
  78555.1 |#############
  83005.8 |########################################
  87456.5 |##########################
  91907.3 |
  96358.0 |#############
  100808.8 |#############
  105259.5 |
  109710.3 |
  114161.0 |
  118611.7 |
  123062.5 |
  127513.2 |
  131964.0 |#############
  136414.7 |
  140865.5 |
  145316.2 |
  149766.9 |
  154217.7 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=80.8% (high variance, measurements may be unstable)
- **head_tail**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **head_tail**: bridge=1133.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=76.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **head_tail_dyn**: bridge=1023.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=85.3% (high variance, measurements may be unstable)
- **kernel_nway**: worst_20/best_20 = 5.8x (possible bimodal distribution)
- **kernel_nway**: bridge=1535.7% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=77.9% (high variance, measurements may be unstable)
- **kernel_seq**: worst_20/best_20 = 4.2x (possible bimodal distribution)
- **kernel_seq**: bridge=1187.5% of algo (FFI overhead may distort results)
- **nway**: CV=82.4% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 9.2x (possible bimodal distribution)
- **nway**: bridge=602.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=519.3% of algo (FFI overhead may distort results)
- **std_parallel**: CV=47.1% (high variance, measurements may be unstable)
- **std_parallel**: bridge=585.5% of algo (FFI overhead may distort results)
