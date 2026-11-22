# fold_heavy, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 32% faster than the next best (head_tail_dyn)

kernel_nway (92.36 us) leads head_tail_dyn (122.23 us) by 32%, a clear separation rather than a photo finish. CV 65.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel is an outlier: 3.2x slower than the field

std_parallel (294.70 us) is 3.2x the fastest (92.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.2x the fastest

Fastest kernel_nway (92.36 us) to slowest std_parallel (294.70 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 5.6x its best-20%

nway's best 20% of batches run at 87.46 us but its worst 20% at 494.09 us (5.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: kernel_nway** at 92357.0 ns median (-25.1% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 3.19x (fastest 92357.0 ns, slowest 294704.0 ns)

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
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 177596ns | 123631ns | 115399ns | 129418ns | 280962ns | base |
| head_tail_dyn | 185545ns | 122564ns | 120396ns | 123040ns | 311878ns | +4.48% |
| kernel_nway | 126817ns | 92600ns | 72128ns | 92790ns | 205203ns | -28.59% |
| kernel_seq | 213036ns | 194924ns | 191509ns | 194618ns | 251426ns | +19.96% |
| nway | 253126ns | 172887ns | 87659ns | 146679ns | 495529ns | +42.53% |
| sequential | 211182ns | 195763ns | 195434ns | 195707ns | 242269ns | +18.91% |
| std_parallel | 336457ns | 295783ns | 162317ns | 262270ns | 534807ns | +89.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 176892ns | 115191ns | 279390ns | base | 0.370 |
| head_tail_dyn | 184762ns | 120171ns | 310042ns | +4.45% | 0.355 |
| kernel_nway | 125902ns | 71916ns | 203251ns | -28.83% | 0.521 |
| kernel_seq | 212055ns | 191049ns | 249766ns | +19.88% | 0.309 |
| nway | 252449ns | 87456ns | 494094ns | +42.71% | 0.260 |
| sequential | 210977ns | 195268ns | 241999ns | +19.27% | 0.311 |
| std_parallel | 334650ns | 161632ns | 531390ns | +89.18% | 0.196 |

## Performance model

- Peak throughput: **0.911 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.531 | 58.3% |
| head_tail_dyn | 0.536 | 58.8% |
| kernel_nway | 0.710 | 77.9% |
| kernel_seq | 0.338 | 37.0% |
| nway | 0.380 | 41.7% |
| sequential | 0.335 | 36.8% |
| std_parallel | 0.222 | 24.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 177596ns | 177596ns | base |
| head_tail_dyn | 185545ns | 185545ns | +4.48% |
| kernel_nway | 126817ns | 126817ns | -28.59% |
| kernel_seq | 213036ns | 213036ns | +19.96% |
| nway | 253126ns | 253126ns | +42.53% |
| sequential | 211182ns | 211182ns | +18.91% |
| std_parallel | 336457ns | 336457ns | +89.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 123373ns | base | --- | [115472, 251170] | --- | --- | --- | --- |
| head_tail_dyn | 122232ns | no significant difference | [-37471, +5880]ns | [121116, 174337] | no | 0.3568 | 0.2379 | 0 |
| kernel_nway | 92357ns | no significant difference | [-110750, +5684]ns | [73916, 162445] | no | 0.3568 | 0.2379 | 0 |
| kernel_seq | 194141ns | +69521.9ns (+56.4%) | [+680, +77044]ns | [192698, 206750] | YES (adj: no) | 0.2888 | 0.0963 | 0 |
| nway | 172492ns | no significant difference | [-33467, +34214]ns | [93861, 259203] | no | 1.0000 | 1.0000 | 0 |
| sequential | 195602ns | no significant difference | [-28527, +90698]ns | [195379, 238747] | no | 0.5768 | 0.4807 | 0 |
| std_parallel | 294704ns | +100679.0ns (+81.6%) | [+40454, +185913]ns | [201773, 389701] | YES | 0.0452 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 116170ns | +4.5% | +39.8% | +70.4% | +287.9% | +68.3% | +165.6% |
| 1 | 2 | 115455ns | +5.9% | -20.0% | +66.6% | -8.3% | +107.0% | +253.9% |
| 1 | 3 | 251170ns | -32.1% | -71.2% | -22.7% | -53.2% | -22.2% | -26.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.169 | ok |
| head_tail_dyn | -0.158 | ok |
| kernel_nway | -0.062 | ok |
| kernel_seq | -0.366 | moderate- |
| nway | -0.184 | ok |
| sequential | -0.667 | HIGH- (thermal bounce) |
| std_parallel | -0.463 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 177055.2ns | 176892.2ns | 100.1% | HIGH |
| head_tail_dyn | 187103.2ns | 184762.3ns | 101.3% | HIGH |
| kernel_nway | 127338.6ns | 125901.6ns | 101.1% | HIGH |
| kernel_seq | 215981.8ns | 212055.0ns | 101.9% | HIGH |
| nway | 259258.2ns | 252449.3ns | 102.7% | HIGH |
| sequential | 211748.2ns | 210977.2ns | 100.4% | HIGH |
| std_parallel | 342582.2ns | 334649.7ns | 102.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 115191.1-279390.2 ns)
  115191.1 |########################################
  123401.0 |#####
  131611.0 |#####
  139820.9 |
  148030.9 |
  156240.8 |
  164450.8 |
  172660.8 |
  180870.7 |
  189080.7 |
  197290.6 |#####
  205500.6 |
  213710.5 |
  221920.5 |
  230130.5 |
  238340.4 |
  246550.4 |###########
  254760.3 |#####
  262970.3 |#####
  271180.2 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 120171.2-310041.7 ns)
  120171.2 |########################################
  129664.7 |########
  139158.2 |
  148651.7 |
  158145.3 |
  167638.8 |
  177132.3 |
  186625.9 |
  196119.4 |
  205612.9 |
  215106.4 |####
  224600.0 |
  234093.5 |####
  243587.0 |
  253080.6 |
  262574.1 |####
  272067.6 |####
  281561.1 |
  291054.7 |
  300548.2 |
  (2 below, 1 above range)

kernel_nway (n=18, range 71916.0-203251.4 ns)
  71916.0 |########################################
  78482.8 |######
  85049.5 |######
  91616.3 |######
  98183.1 |
  104749.8 |
  111316.6 |######
  117883.4 |
  124450.1 |
  131016.9 |
  137583.7 |
  144150.4 |#############
  150717.2 |
  157284.0 |
  163850.7 |
  170417.5 |######
  176984.3 |
  183551.0 |
  190117.8 |
  196684.6 |######
  (1 below, 3 above range)

kernel_seq (n=18, range 191048.6-249766.4 ns)
  191048.6 |########################################
  193984.5 |##########
  196920.3 |##########
  199856.2 |
  202792.1 |
  205728.0 |
  208663.9 |
  211599.8 |#####
  214535.7 |#####
  217471.6 |
  220407.5 |
  223343.4 |
  226279.3 |
  229215.2 |
  232151.1 |
  235086.9 |#####
  238022.8 |
  240958.7 |
  243894.6 |
  246830.5 |
  (1 below, 2 above range)

nway (n=18, range 87455.8-494094.1 ns)
  87455.8 |########################################
  107787.7 |##########
  128119.7 |##########
  148451.6 |
  168783.5 |####################
  189115.4 |
  209447.3 |####################
  229779.2 |##########
  250111.1 |
  270443.1 |##########
  290775.0 |
  311106.9 |
  331438.8 |##########
  351770.7 |
  372102.6 |
  392434.5 |
  412766.5 |
  433098.4 |
  453430.3 |
  473762.2 |
  (2 below, 3 above range)

sequential (n=18, range 195267.8-241998.6 ns)
  195267.8 |########################################
  197604.3 |
  199940.9 |
  202277.4 |
  204614.0 |
  206950.5 |
  209287.0 |
  211623.6 |
  213960.1 |
  216296.6 |
  218633.2 |
  220969.7 |
  223306.3 |
  225642.8 |
  227979.3 |
  230315.9 |
  232652.4 |
  234989.0 |
  237325.5 |####################
  239662.0 |
  (2 below, 1 above range)

std_parallel (n=18, range 161632.0-531390.4 ns)
  161632.0 |####################
  180119.9 |####################
  198607.8 |########################################
  217095.8 |
  235583.7 |####################
  254071.6 |
  272559.5 |########################################
  291047.5 |####################
  309535.4 |####################
  328023.3 |####################
  346511.2 |
  364999.1 |
  383487.1 |########################################
  401975.0 |
  420462.9 |####################
  438950.8 |
  457438.8 |
  475926.7 |####################
  494414.6 |
  512902.5 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: CV=43.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=101.8% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=77.9% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=99.8% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=48.2% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=101.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.8% of algo (FFI overhead may distort results)
- **nway**: CV=84.8% (high variance, measurements may be unstable)
- **nway**: worst_20/best_20 = 5.6x (possible bimodal distribution)
- **nway**: bridge=102.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.1% of algo (FFI overhead may distort results)
- **std_parallel**: CV=52.1% (high variance, measurements may be unstable)
- **std_parallel**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **std_parallel**: bridge=97.9% of algo (FFI overhead may distort results)
